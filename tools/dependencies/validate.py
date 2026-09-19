#!/usr/bin/env python3
"""Fail-closed direct dependency admission validation for Kernux."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import tomllib
from dataclasses import dataclass
from pathlib import Path
from typing import Any, Iterable

APPROVAL_SCHEMA = "kernux.dependency-approvals/v1"
APPROVAL_PATH = Path("third_party/dependencies/approved.json")
NPM_SECTIONS = {
    "dependencies": "runtime",
    "devDependencies": "dev",
    "optionalDependencies": "optional",
}
CARGO_SECTIONS = {
    "dependencies": "runtime",
    "dev-dependencies": "dev",
    "build-dependencies": "build",
}
EXACT_SEMVER = re.compile(
    r"^[0-9]+\.[0-9]+\.[0-9]+(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?$"
)
FULL_GIT_REV = re.compile(r"^[0-9a-fA-F]{40}$")
PERMITTED_LICENSE_IDS = {"MIT", "Apache-2.0"}
SPDX_TOKEN = re.compile(r"MIT|Apache-2\.0|AND|OR|\(|\)|\s+")


class ValidationError(RuntimeError):
    pass


@dataclass(frozen=True)
class DirectDependency:
    ecosystem: str
    name: str
    source: str
    version: str
    scopes: tuple[str, ...]
    manifest: str

    @property
    def scope(self) -> str:
        return self.scopes[0] if len(self.scopes) == 1 else "+".join(self.scopes)


@dataclass(frozen=True)
class Approval:
    ecosystem: str
    name: str
    source: str
    version: str
    scope: str
    purpose: str
    license_expression: str
    license_posture: str
    license_policy_exception: str | None = None


def _load_json(path: Path) -> Any:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as exc:
        raise ValidationError(f"cannot read valid JSON {path}: {exc}") from exc


def _load_toml(path: Path) -> dict[str, Any]:
    try:
        return tomllib.loads(path.read_text(encoding="utf-8"))
    except (OSError, tomllib.TOMLDecodeError) as exc:
        raise ValidationError(f"cannot read valid TOML {path}: {exc}") from exc


def _nonempty_string(value: Any, field: str) -> str:
    if not isinstance(value, str) or not value.strip():
        raise ValidationError(f"{field} must be a non-empty string")
    return value.strip()


def _license_is_permitted(expression: str) -> bool:
    compact = re.sub(r"\s+", "", expression)
    if expression in PERMITTED_LICENSE_IDS:
        return True
    # Permit compound expressions only when every license identifier is already
    # permitted by canonical LICENSE_POLICY. Operators do not expand the set.
    stripped = SPDX_TOKEN.sub("", expression)
    return not stripped and "MIT" in expression or not stripped and "Apache-2.0" in expression


def _validate_license(approval: Approval, root: Path) -> None:
    if _license_is_permitted(approval.license_expression):
        if approval.license_posture != "permitted-by-LICENSE_POLICY":
            raise ValidationError(
                f"{approval.ecosystem}:{approval.name} permitted license must use "
                "license_posture=permitted-by-LICENSE_POLICY"
            )
        return

    if approval.license_posture != "exception-reviewed":
        raise ValidationError(
            f"{approval.ecosystem}:{approval.name} license {approval.license_expression!r} "
            "requires separately governed policy exception"
        )
    exception = approval.license_policy_exception
    if not exception:
        raise ValidationError(
            f"{approval.ecosystem}:{approval.name} exception-reviewed approval lacks exception path"
        )
    path = root / exception
    if not path.is_file() or not (
        exception.startswith("docs/canonical/") or exception.startswith("docs/evidence/")
    ):
        raise ValidationError(
            f"{approval.ecosystem}:{approval.name} exception path is not canonical evidence: "
            f"{exception}"
        )


def load_approvals(root: Path) -> dict[tuple[str, str], Approval]:
    raw = _load_json(root / APPROVAL_PATH)
    if not isinstance(raw, dict) or raw.get("schema") != APPROVAL_SCHEMA:
        raise ValidationError(f"{APPROVAL_PATH} must declare {APPROVAL_SCHEMA}")
    entries = raw.get("approvals")
    if not isinstance(entries, list):
        raise ValidationError("approvals must be an array")

    result: dict[tuple[str, str], Approval] = {}
    required = {
        "ecosystem",
        "name",
        "source",
        "version",
        "scope",
        "purpose",
        "license_expression",
        "license_posture",
    }
    allowed = required | {"license_policy_exception"}
    for index, entry in enumerate(entries):
        if not isinstance(entry, dict):
            raise ValidationError(f"approval[{index}] must be an object")
        unknown = set(entry) - allowed
        missing = required - set(entry)
        if unknown:
            raise ValidationError(f"approval[{index}] unknown fields: {sorted(unknown)}")
        if missing:
            raise ValidationError(f"approval[{index}] missing fields: {sorted(missing)}")

        ecosystem = _nonempty_string(entry["ecosystem"], f"approval[{index}].ecosystem")
        if ecosystem not in {"npm", "cargo"}:
            raise ValidationError(f"approval[{index}] unsupported ecosystem {ecosystem!r}")
        source = _nonempty_string(entry["source"], f"approval[{index}].source")
        if source not in {"registry", "git"}:
            raise ValidationError(f"approval[{index}] unsupported source {source!r}")

        approval = Approval(
            ecosystem=ecosystem,
            name=_nonempty_string(entry["name"], f"approval[{index}].name"),
            source=source,
            version=_nonempty_string(entry["version"], f"approval[{index}].version"),
            scope=_nonempty_string(entry["scope"], f"approval[{index}].scope"),
            purpose=_nonempty_string(entry["purpose"], f"approval[{index}].purpose"),
            license_expression=_nonempty_string(
                entry["license_expression"], f"approval[{index}].license_expression"
            ),
            license_posture=_nonempty_string(
                entry["license_posture"], f"approval[{index}].license_posture"
            ),
            license_policy_exception=entry.get("license_policy_exception"),
        )
        key = (approval.ecosystem, approval.name)
        if key in result:
            raise ValidationError(f"duplicate approval for {approval.ecosystem}:{approval.name}")
        if approval.source == "registry" and not EXACT_SEMVER.fullmatch(approval.version):
            raise ValidationError(
                f"{approval.ecosystem}:{approval.name} approval version must be exact semver"
            )
        if approval.source == "git" and not FULL_GIT_REV.fullmatch(approval.version):
            raise ValidationError(
                f"{approval.ecosystem}:{approval.name} git approval version must be full commit"
            )
        _validate_license(approval, root)
        result[key] = approval
    return result


def _exact_npm_version(name: str, value: Any) -> str:
    if not isinstance(value, str) or not EXACT_SEMVER.fullmatch(value):
        raise ValidationError(f"npm:{name} must use exact semver, observed {value!r}")
    return value


def discover_npm(root: Path) -> list[DirectDependency]:
    manifest = _load_json(root / "package.json")
    found: dict[str, DirectDependency] = {}
    for section, scope in NPM_SECTIONS.items():
        values = manifest.get(section, {})
        if not isinstance(values, dict):
            raise ValidationError(f"package.json {section} must be an object")
        for name, value in values.items():
            if name in found:
                raise ValidationError(f"npm:{name} is declared in multiple dependency sections")
            found[name] = DirectDependency(
                ecosystem="npm",
                name=name,
                source="registry",
                version=_exact_npm_version(name, value),
                scopes=(scope,),
                manifest="package.json",
            )
    return sorted(found.values(), key=lambda dep: dep.name)


def _workspace_member_manifests(root: Path, cargo: dict[str, Any]) -> list[Path]:
    workspace = cargo.get("workspace")
    if not isinstance(workspace, dict):
        raise ValidationError("Cargo.toml must define [workspace]")
    members = workspace.get("members", [])
    if not isinstance(members, list):
        raise ValidationError("Cargo workspace members must be an array")
    paths: set[Path] = set()
    for pattern in members:
        if not isinstance(pattern, str):
            raise ValidationError("Cargo workspace member pattern must be a string")
        for match in root.glob(pattern):
            manifest = match / "Cargo.toml" if match.is_dir() else match
            if manifest.is_file():
                paths.add(manifest)
    return sorted(paths)


def _cargo_external_spec(
    name: str,
    spec: Any,
    workspace_dependencies: dict[str, Any],
) -> tuple[str, str, str] | None:
    if isinstance(spec, str):
        requirement = spec
        source = "registry"
        package = name
    elif isinstance(spec, dict):
        if spec.get("path") is not None:
            return None
        if spec.get("workspace") is True:
            if name not in workspace_dependencies:
                raise ValidationError(f"cargo:{name} workspace dependency has no root definition")
            return _cargo_external_spec(name, workspace_dependencies[name], workspace_dependencies)
        package = spec.get("package", name)
        if not isinstance(package, str) or not package:
            raise ValidationError(f"cargo:{name} package alias must be a string")
        if "git" in spec:
            rev = spec.get("rev")
            if not isinstance(rev, str) or not FULL_GIT_REV.fullmatch(rev):
                raise ValidationError(f"cargo:{package} git dependency must pin a full rev")
            return package, "git", rev.lower()
        requirement = spec.get("version")
        source = "registry"
    else:
        raise ValidationError(f"cargo:{name} dependency spec has unsupported type")

    if not isinstance(requirement, str) or not requirement.startswith("="):
        raise ValidationError(
            f"cargo:{package} registry dependency must use exact =x.y.z requirement"
        )
    version = requirement[1:]
    if not EXACT_SEMVER.fullmatch(version):
        raise ValidationError(f"cargo:{package} exact registry version is invalid: {requirement}")
    return package, source, version


def _iter_cargo_tables(doc: dict[str, Any]) -> Iterable[tuple[str, dict[str, Any]]]:
    for section, scope in CARGO_SECTIONS.items():
        table = doc.get(section, {})
        if table:
            if not isinstance(table, dict):
                raise ValidationError(f"Cargo {section} must be a table")
            yield scope, table
    targets = doc.get("target", {})
    if targets:
        if not isinstance(targets, dict):
            raise ValidationError("Cargo target must be a table")
        for target_name, target in targets.items():
            if not isinstance(target, dict):
                raise ValidationError(f"Cargo target {target_name} must be a table")
            for section, scope in CARGO_SECTIONS.items():
                table = target.get(section, {})
                if table:
                    if not isinstance(table, dict):
                        raise ValidationError(
                            f"Cargo target {target_name}.{section} must be a table"
                        )
                    yield scope, table


def discover_cargo(root: Path) -> list[DirectDependency]:
    root_doc = _load_toml(root / "Cargo.toml")
    workspace = root_doc.get("workspace", {})
    workspace_dependencies = workspace.get("dependencies", {})
    if workspace_dependencies and not isinstance(workspace_dependencies, dict):
        raise ValidationError("[workspace.dependencies] must be a table")

    aggregate: dict[tuple[str, str, str], set[str]] = {}
    manifests = _workspace_member_manifests(root, root_doc)
    for manifest in manifests:
        doc = _load_toml(manifest)
        for scope, table in _iter_cargo_tables(doc):
            for key, spec in table.items():
                external = _cargo_external_spec(key, spec, workspace_dependencies)
                if external is None:
                    continue
                name, source, version = external
                aggregate.setdefault((name, source, version), set()).add(scope)

    by_name: dict[str, tuple[str, str, set[str]]] = {}
    for (name, source, version), scopes in aggregate.items():
        previous = by_name.get(name)
        if previous and (previous[0], previous[1]) != (source, version):
            raise ValidationError(f"cargo:{name} resolves to multiple direct sources/versions")
        if previous:
            previous[2].update(scopes)
        else:
            by_name[name] = (source, version, set(scopes))

    return [
        DirectDependency(
            ecosystem="cargo",
            name=name,
            source=source,
            version=version,
            scopes=tuple(sorted(scopes)),
            manifest="Cargo workspace members",
        )
        for name, (source, version, scopes) in sorted(by_name.items())
    ]


def _cargo_metadata(root: Path) -> dict[str, Any]:
    command = ["cargo", "metadata", "--locked", "--format-version", "1"]
    completed = subprocess.run(
        command,
        cwd=root,
        text=True,
        capture_output=True,
        check=False,
    )
    if completed.returncode != 0:
        detail = completed.stderr.strip() or completed.stdout.strip()
        raise ValidationError(f"cargo metadata --locked failed: {detail}")
    try:
        return json.loads(completed.stdout)
    except json.JSONDecodeError as exc:
        raise ValidationError(f"cargo metadata returned invalid JSON: {exc}") from exc


def _validate_cargo_metadata(
    dependencies: list[DirectDependency],
    approvals: dict[tuple[str, str], Approval],
    metadata: dict[str, Any],
) -> None:
    packages = metadata.get("packages")
    if not isinstance(packages, list):
        raise ValidationError("cargo metadata packages must be an array")
    for dep in dependencies:
        approval = approvals[("cargo", dep.name)]
        matches = [
            package
            for package in packages
            if package.get("name") == dep.name and package.get("version") == dep.version
        ]
        if not matches:
            raise ValidationError(
                f"cargo:{dep.name}@{dep.version} is not resolved by cargo metadata --locked"
            )
        package = matches[0]
        observed_license = package.get("license")
        if observed_license != approval.license_expression:
            raise ValidationError(
                f"cargo:{dep.name}@{dep.version} license mismatch: "
                f"approval={approval.license_expression!r} metadata={observed_license!r}"
            )
        source = package.get("source")
        if dep.source == "registry" and not (
            isinstance(source, str) and source.startswith("registry+")
        ):
            raise ValidationError(f"cargo:{dep.name} expected registry source, observed {source!r}")
        if dep.source == "git" and not (
            isinstance(source, str) and source.startswith("git+") and dep.version in source
        ):
            raise ValidationError(f"cargo:{dep.name} expected pinned git source, observed {source!r}")


def validate_repository(
    root: Path,
    *,
    cargo_metadata_override: dict[str, Any] | None = None,
) -> list[DirectDependency]:
    root = root.resolve()
    approvals = load_approvals(root)
    dependencies = discover_npm(root) + discover_cargo(root)
    actual = {(dep.ecosystem, dep.name): dep for dep in dependencies}

    for key, dep in actual.items():
        approval = approvals.get(key)
        if approval is None:
            raise ValidationError(f"unapproved direct dependency {dep.ecosystem}:{dep.name}")
        if approval.source != dep.source:
            raise ValidationError(
                f"{dep.ecosystem}:{dep.name} source mismatch: "
                f"approval={approval.source} manifest={dep.source}"
            )
        if approval.version != dep.version:
            raise ValidationError(
                f"{dep.ecosystem}:{dep.name} version mismatch: "
                f"approval={approval.version} manifest={dep.version}"
            )
        if approval.scope != dep.scope:
            raise ValidationError(
                f"{dep.ecosystem}:{dep.name} scope mismatch: "
                f"approval={approval.scope} manifest={dep.scope}"
            )

    stale = sorted(set(approvals) - set(actual))
    if stale:
        joined = ", ".join(f"{ecosystem}:{name}" for ecosystem, name in stale)
        raise ValidationError(f"stale dependency approvals: {joined}")

    cargo_dependencies = [dep for dep in dependencies if dep.ecosystem == "cargo"]
    if cargo_dependencies:
        metadata = cargo_metadata_override or _cargo_metadata(root)
        _validate_cargo_metadata(cargo_dependencies, approvals, metadata)

    return dependencies


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=["check"])
    parser.add_argument("--root", default=".")
    args = parser.parse_args()
    try:
        dependencies = validate_repository(Path(args.root))
    except ValidationError as exc:
        print(f"Dependency admission: FAIL — {exc}")
        return 1
    print(f"Dependency admission: PASS ({len(dependencies)} direct external dependencies)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
