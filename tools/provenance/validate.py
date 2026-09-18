#!/usr/bin/env python3
"""Strict dependency-free validator for Kernux donor provenance manifests."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
from pathlib import Path
from urllib.parse import urlsplit
from typing import Any

ROOT = Path(__file__).resolve().parents[2]
SCHEMA = ROOT / "third_party/provenance/schema/import-manifest.schema.json"
MANIFESTS = ROOT / "third_party/provenance/manifests"
MAX_BYTES = 512 * 1024


class ProvenanceError(ValueError):
    pass


def _pairs(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
    result: dict[str, Any] = {}
    for key, value in pairs:
        if key in result:
            raise ProvenanceError(f"duplicate object key: {key}")
        result[key] = value
    return result


def _constant(token: str) -> Any:
    raise ProvenanceError(f"non-finite JSON number: {token}")


def read_json(path: Path) -> Any:
    if path.is_symlink():
        raise ProvenanceError(f"{path}: symlink inputs are forbidden")
    if not path.is_file():
        raise ProvenanceError(f"{path}: expected a regular file")
    size = path.stat().st_size
    if size > MAX_BYTES:
        raise ProvenanceError(f"{path}: exceeds {MAX_BYTES}-byte limit")
    try:
        text = path.read_bytes().decode("utf-8")
    except UnicodeDecodeError as exc:
        raise ProvenanceError(f"{path}: input is not valid UTF-8") from exc
    try:
        return json.loads(text, object_pairs_hook=_pairs, parse_constant=_constant)
    except json.JSONDecodeError as exc:
        raise ProvenanceError(f"{path}: malformed JSON: {exc.msg}") from exc


def _resolve(root: dict[str, Any], ref: str) -> dict[str, Any]:
    if not ref.startswith("#/"):
        raise ProvenanceError(f"unsupported schema reference: {ref}")
    value: Any = root
    for part in ref[2:].split("/"):
        value = value[part.replace("~1", "/").replace("~0", "~")]
    if not isinstance(value, dict):
        raise ProvenanceError(f"schema reference is not an object: {ref}")
    return value


def _type_ok(value: Any, name: str) -> bool:
    return {
        "object": isinstance(value, dict),
        "array": isinstance(value, list),
        "string": isinstance(value, str),
        "integer": isinstance(value, int) and not isinstance(value, bool),
        "boolean": isinstance(value, bool),
        "null": value is None,
    }.get(name, False)


def _schema_errors(value: Any, rule: dict[str, Any], root: dict[str, Any], at: str) -> list[str]:
    if "$ref" in rule:
        return _schema_errors(value, _resolve(root, rule["$ref"]), root, at)
    errors: list[str] = []
    expected = rule.get("type")
    if expected is not None:
        choices = expected if isinstance(expected, list) else [expected]
        if not any(_type_ok(value, choice) for choice in choices):
            return [f"{at}: expected type {'|'.join(choices)}"]
    if "const" in rule and value != rule["const"]:
        errors.append(f"{at}: expected constant {rule['const']!r}")
    if "enum" in rule and value not in rule["enum"]:
        errors.append(f"{at}: value is not in the allowed enum")
    if isinstance(value, str):
        if len(value) < rule.get("minLength", 0):
            errors.append(f"{at}: string is too short")
        if "maxLength" in rule and len(value) > rule["maxLength"]:
            errors.append(f"{at}: string is too long")
        if "pattern" in rule and re.search(rule["pattern"], value) is None:
            errors.append(f"{at}: string does not match required pattern")
    if isinstance(value, dict):
        required = rule.get("required", [])
        for key in required:
            if key not in value:
                errors.append(f"{at}: missing required property {key}")
        properties = rule.get("properties", {})
        if rule.get("additionalProperties") is False:
            for key in value:
                if key not in properties:
                    errors.append(f"{at}: unknown property {key}")
        for key, nested in value.items():
            child = properties.get(key)
            if child is not None:
                errors.extend(_schema_errors(nested, child, root, f"{at}.{key}"))
    if isinstance(value, list):
        if len(value) < rule.get("minItems", 0):
            errors.append(f"{at}: array has too few items")
        if "maxItems" in rule and len(value) > rule["maxItems"]:
            errors.append(f"{at}: array has too many items")
        if rule.get("uniqueItems"):
            encoded = [json.dumps(item, sort_keys=True, separators=(",", ":")) for item in value]
            if len(set(encoded)) != len(encoded):
                errors.append(f"{at}: array items must be unique")
        if "items" in rule:
            for index, item in enumerate(value):
                errors.extend(_schema_errors(item, rule["items"], root, f"{at}[{index}]"))
    return errors


def _schema_contract_errors(schema: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if schema.get("$schema") != "https://json-schema.org/draft/2020-12/schema":
        errors.append("schema: expected JSON Schema 2020-12")
    if schema.get("properties", {}).get("schema_version", {}).get("const") != 1:
        errors.append("schema: schema_version must be const 1")
    status = schema.get("properties", {}).get("status", {}).get("enum")
    if status != ["prepared", "imported"]:
        errors.append("schema: status enum drift")
    transformation = schema.get("$defs", {}).get("mapping", {}).get("properties", {}).get("transformation", {}).get("enum")
    if transformation != ["verbatim", "adapted", "ported", "reference-only", "generated"]:
        errors.append("schema: transformation enum drift")

    def walk(node: Any, at: str) -> None:
        if isinstance(node, dict):
            if node.get("type") == "object" and node.get("additionalProperties") is not False:
                errors.append(f"{at}: object schema must set additionalProperties=false")
            for key, child in node.items():
                walk(child, f"{at}.{key}")
        elif isinstance(node, list):
            for index, child in enumerate(node):
                walk(child, f"{at}[{index}]")

    walk(schema, "schema")
    return errors


def _semantic_errors(data: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    text_fields = [
        (data["donor"]["name"], "$.donor.name"),
        (data["donor"]["role"], "$.donor.role"),
        (data["license"]["spdx_expression"], "$.license.spdx_expression"),
        (data["dependency_review"]["notes"], "$.dependency_review.notes"),
        (data["security_review"]["reason"], "$.security_review.reason"),
    ]
    text_fields.extend(
        (item["reference"], f"$.authorization[{index}].reference")
        for index, item in enumerate(data["authorization"])
    )
    reason = data["characterization"]["reason"]
    if isinstance(reason, str):
        text_fields.append((reason, "$.characterization.reason"))
    for value, at in text_fields:
        if not value.strip() or value != value.strip():
            errors.append(f"{at}: must contain trimmed non-whitespace text")

    source = data["source"]
    if source["kind"] == "git":
        if "repository" not in source or "revision" not in source:
            errors.append("$.source: git source requires repository and revision")
        if "artifact_id" in source or "content_digest" in source:
            errors.append("$.source: git source must not declare artifact identity")
        repository = source.get("repository")
        if isinstance(repository, str):
            try:
                parsed = urlsplit(repository)
                _ = parsed.port
                path_parts = parsed.path.split("/")
                canonical = (
                    parsed.scheme == "https"
                    and bool(parsed.hostname)
                    and parsed.username is None
                    and parsed.password is None
                    and not parsed.query
                    and not parsed.fragment
                    and repository == repository.strip()
                    and not any(
                        character.isspace()
                        or ord(character) < 32
                        or ord(character) == 127
                        for character in repository
                    )
                    and "\\" not in repository
                    and "%" not in parsed.path
                    and parsed.path.startswith("/")
                    and not parsed.path.endswith("/")
                    and len(path_parts) > 1
                    and all(part not in ("", ".", "..") for part in path_parts[1:])
                )
            except ValueError:
                canonical = False
            if not canonical:
                errors.append(
                    "$.source.repository: git repository must be a credential-free canonical HTTPS repository URL"
                )
    else:
        if "artifact_id" not in source or "content_digest" not in source:
            errors.append("$.source: artifact source requires artifact_id and content_digest")
        if "repository" in source or "revision" in source:
            errors.append("$.source: artifact source must not declare git identity")
        if isinstance(source.get("artifact_id"), str) and source["artifact_id"] != source["artifact_id"].strip():
            errors.append("$.source.artifact_id: must contain trimmed non-whitespace text")

    for index, mapping in enumerate(data["mappings"]):
        destinations = mapping["destination_paths"]
        transformation = mapping["transformation"]
        generation = mapping.get("generation")
        if transformation == "reference-only" and destinations:
            errors.append(f"$.mappings[{index}]: reference-only mapping must have no destinations")
        if transformation != "reference-only" and not destinations:
            errors.append(f"$.mappings[{index}]: imported mapping requires a destination")
        if transformation == "generated" and generation is None:
            errors.append(f"$.mappings[{index}]: generated mapping requires generator provenance")
        if transformation != "generated" and generation is not None:
            errors.append(f"$.mappings[{index}]: generator provenance is only valid for generated mappings")
        if generation is not None:
            for field in ("tool", "revision"):
                if not generation[field].strip() or generation[field] != generation[field].strip():
                    errors.append(f"$.mappings[{index}].generation.{field}: must contain trimmed non-whitespace text")
    characterization = data["characterization"]
    if characterization["status"] == "complete" and not characterization["test_paths"]:
        errors.append("$.characterization: complete status requires test_paths")
    if characterization["status"] == "not-applicable" and not characterization["reason"]:
        errors.append("$.characterization: not-applicable status requires reason")

    imported = data["status"] == "imported"
    commit = data["import"]["commit"]
    if imported and commit is None:
        errors.append("$.import.commit: imported status requires an exact import commit")
    if not imported and commit is not None:
        errors.append("$.import.commit: prepared status must not declare an import commit")
    if not imported and data["import"]["adaptation_commits"]:
        errors.append("$.import.adaptation_commits: prepared status must not declare adaptation commits")
    if imported:
        for index, mapping in enumerate(data["mappings"]):
            generation = mapping.get("generation")
            if mapping["transformation"] == "generated" and generation is not None and not generation["evidence_paths"]:
                errors.append(f"$.mappings[{index}].generation.evidence_paths: imported generated mapping requires evidence")
    if imported and characterization["status"] == "planned":
        errors.append("$.characterization: imported status cannot remain planned")
    if imported and not data["license"]["evidence_paths"]:
        errors.append("$.license.evidence_paths: imported status requires local license/notice evidence")
    if imported and not data["dependency_review"]["evidence_paths"]:
        errors.append("$.dependency_review.evidence_paths: imported status requires review evidence")
    security = data["security_review"]
    if security["status"] == "complete" and not security["evidence_paths"]:
        errors.append("$.security_review: complete status requires evidence_paths")
    if not data["dependency_review"]["evidence_paths"] and not data["dependency_review"]["notes"].strip():
        errors.append("$.dependency_review: evidence_paths or notes are required")
    return errors



def _repository_path_errors(
    root: Path, relative: str, at: str, *, require_file: bool = False
) -> list[str]:
    candidate = root
    for part in relative.split("/"):
        candidate /= part
        if candidate.is_symlink():
            return [f"{at}: repository path must not traverse a symlink"]
    try:
        root_resolved = root.resolve(strict=True)
        resolved = candidate.resolve(strict=True)
    except FileNotFoundError:
        return [f"{at}: repository path does not exist at validation head"]
    except OSError as exc:
        return [f"{at}: repository path could not be resolved: {exc}"]
    try:
        resolved.relative_to(root_resolved)
    except ValueError:
        return [f"{at}: repository path resolves outside the repository"]
    if require_file and not resolved.is_file():
        return [f"{at}: evidence path must resolve to a regular file"]
    return []


def _git_is_ancestor(root: Path, ancestor: str, descendant: str) -> bool:
    result = subprocess.run(
        ["git", "merge-base", "--is-ancestor", ancestor, descendant],
        cwd=root,
        stdout=subprocess.DEVNULL,
        stderr=subprocess.DEVNULL,
        timeout=5,
        check=False,
    )
    return result.returncode == 0


def _commit_destination_errors(root: Path, oid: str, relative: str, at: str) -> list[str]:
    try:
        result = subprocess.run(
            ["git", "diff-tree", "--root", "-m", "--no-commit-id", "--name-only", "-r", oid, "--", relative],
            cwd=root, text=True, capture_output=True, timeout=5, check=False,
        )
    except (OSError, subprocess.TimeoutExpired) as exc:
        return [f"{at}: import-commit destination verification failed: {exc}"]
    if result.returncode != 0:
        return [f"{at}: import-commit destination verification failed"]
    if not result.stdout.strip():
        return [f"{at}: import commit does not touch claimed destination {relative}"]
    return []


def _commit_reference_errors(root: Path, oid: str, at: str) -> list[str]:
    try:
        exists = subprocess.run(
            ["git", "cat-file", "-e", f"{oid}^{{commit}}"],
            cwd=root, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
            timeout=5, check=False,
        )
        if exists.returncode != 0:
            return [f"{at}: referenced commit does not exist in this repository"]
        ancestor_ok = _git_is_ancestor(root, oid, "HEAD")
    except (OSError, subprocess.TimeoutExpired) as exc:
        return [f"{at}: git commit verification failed: {exc}"]
    if not ancestor_ok:
        return [f"{at}: referenced commit is not an ancestor of validation HEAD"]
    return []


def _repository_errors(data: dict[str, Any], root: Path = ROOT) -> list[str]:
    errors: list[str] = []

    def local_paths(
        paths: list[str], at: str, *, require_file: bool = True
    ) -> None:
        for index, relative in enumerate(paths):
            errors.extend(
                _repository_path_errors(
                    root, relative, f"{at}[{index}]", require_file=require_file
                )
            )

    local_paths(data["license"]["evidence_paths"], "$.license.evidence_paths")
    local_paths(data["dependency_review"]["evidence_paths"], "$.dependency_review.evidence_paths")
    local_paths(data["characterization"]["test_paths"], "$.characterization.test_paths")
    local_paths(data["security_review"]["evidence_paths"], "$.security_review.evidence_paths")

    for index, mapping in enumerate(data["mappings"]):
        generation = mapping.get("generation")
        if generation is not None:
            local_paths(generation["evidence_paths"], f"$.mappings[{index}].generation.evidence_paths")
        if data["status"] == "imported" and mapping["transformation"] != "reference-only":
            local_paths(
                mapping["destination_paths"],
                f"$.mappings[{index}].destination_paths",
                require_file=False,
            )

    if data["status"] == "imported":
        import_commit = data["import"]["commit"]
        assert isinstance(import_commit, str)
        import_errors = _commit_reference_errors(root, import_commit, "$.import.commit")
        errors.extend(import_errors)
        if not import_errors:
            for index, mapping in enumerate(data["mappings"]):
                if mapping["transformation"] == "reference-only":
                    continue
                for destination in mapping["destination_paths"]:
                    errors.extend(
                        _commit_destination_errors(
                            root, import_commit, destination, f"$.mappings[{index}].destination_paths"
                        )
                    )
        previous = import_commit
        for index, oid in enumerate(data["import"]["adaptation_commits"]):
            at = f"$.import.adaptation_commits[{index}]"
            errors.extend(_commit_reference_errors(root, oid, at))
            try:
                if oid == previous or not _git_is_ancestor(root, previous, oid):
                    errors.append(
                        f"{at}: adaptation commit must strictly descend from the import commit and prior adaptations"
                    )
            except (OSError, subprocess.TimeoutExpired) as exc:
                errors.append(f"{at}: adaptation ancestry verification failed: {exc}")
            previous = oid

    return errors

def validate_data(data: Any, schema: dict[str, Any]) -> list[str]:
    errors = _schema_errors(data, schema, schema, "$")
    if not errors and isinstance(data, dict):
        errors.extend(_semantic_errors(data))
    return sorted(set(errors))


def validate_file(path: Path, schema: dict[str, Any]) -> list[str]:
    try:
        data = read_json(path)
    except ProvenanceError as exc:
        return [str(exc)]
    return validate_data(data, schema)



def _manifest_paths(directory: Path = MANIFESTS) -> list[Path]:
    if not directory.exists():
        return []
    if directory.is_symlink():
        raise ProvenanceError(f"{directory}: manifest directory must not be a symlink")
    if not directory.is_dir():
        raise ProvenanceError(f"{directory}: manifest location must be a directory")

    manifests: list[Path] = []
    for entry in sorted(directory.iterdir(), key=lambda item: item.name):
        if entry.is_symlink():
            raise ProvenanceError(f"{entry}: symlink entries are forbidden")
        if not entry.is_file():
            raise ProvenanceError(f"{entry}: nested manifest directories are forbidden")
        if entry.suffix != ".json":
            raise ProvenanceError(f"{entry}: unexpected non-JSON entry in manifest ledger")
        manifests.append(entry)
    return manifests

def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("command", choices=["check"])
    parser.add_argument("paths", nargs="*")
    args = parser.parse_args(argv)
    try:
        schema_data = read_json(SCHEMA)
        if not isinstance(schema_data, dict):
            raise ProvenanceError("schema: top-level value must be an object")
        errors = _schema_contract_errors(schema_data)
        paths = [Path(item) for item in args.paths] if args.paths else _manifest_paths()
        record_ids: dict[str, Path] = {}
        destinations: dict[str, Path] = {}
        for path in paths:
            try:
                data = read_json(path)
            except ProvenanceError as exc:
                errors.append(str(exc))
                continue
            data_errors = validate_data(data, schema_data)
            errors.extend(f"{path}: {error}" for error in data_errors)
            if not data_errors and isinstance(data, dict):
                errors.extend(f"{path}: {error}" for error in _repository_errors(data))
                for mapping in data["mappings"]:
                    if mapping["transformation"] == "reference-only":
                        continue
                    for destination in mapping["destination_paths"]:
                        conflict = next(
                            ((claimed, owner) for claimed, owner in destinations.items()
                             if destination == claimed or destination.startswith(claimed + "/") or claimed.startswith(destination + "/")),
                            None,
                        )
                        if conflict is not None:
                            errors.append(
                                f"{path}: destination {destination} overlaps claim {conflict[0]} by {conflict[1]}"
                            )
                        destinations[destination] = path
            if isinstance(data, dict) and isinstance(data.get("record_id"), str):
                prior = record_ids.setdefault(data["record_id"], path)
                if prior != path:
                    errors.append(f"{path}: duplicate record_id {data['record_id']} already declared by {prior}")
        errors = sorted(set(errors))
    except (OSError, KeyError, TypeError, ProvenanceError) as exc:
        print(f"Provenance validation: ERROR\n- {exc}", file=sys.stderr)
        return 2
    if errors:
        print("Provenance validation: FAIL", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1
    print("Provenance validation: PASS")
    print(f"Schema: {SCHEMA.relative_to(ROOT)}")
    print(f"Manifests: {len(paths)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
