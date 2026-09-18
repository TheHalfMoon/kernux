#!/usr/bin/env python3
"""Validate Kernux third-party notice inventory and provenance linkage."""

from __future__ import annotations

import argparse
import hashlib
import re
import sys
from pathlib import Path
from typing import Any
from urllib.parse import urlsplit

ROOT = Path(__file__).resolve().parents[2]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from tools.provenance import validate as provenance  # noqa: E402

INVENTORY = ROOT / "third_party/notices/inventory.json"
LICENSE_ROOT = ROOT / "third_party/notices/licenses"
MAX_ENTRIES = 1024
_GIT_OID = re.compile(r"^(?:[0-9a-f]{40}|[0-9a-f]{64})$")
_SHA256 = re.compile(r"^sha256:[0-9a-f]{64}$")
_ENTRY_ID = re.compile(r"^[a-z0-9][a-z0-9._-]{2,63}$")
_CATEGORIES = frozenset({"donor", "dependency", "asset", "tooling", "other"})


class NoticeError(ValueError):
    """Raised when canonical notice state cannot be inspected safely."""


def _exact_fields(value: Any, expected: set[str], at: str) -> list[str]:
    if not isinstance(value, dict):
        return [f"{at}: expected object"]
    errors: list[str] = []
    unknown = sorted(set(value) - expected)
    missing = sorted(expected - set(value))
    if unknown:
        errors.append(f"{at}: unknown properties: {', '.join(unknown)}")
    if missing:
        errors.append(f"{at}: missing properties: {', '.join(missing)}")
    return errors


def _trimmed_text(
    value: Any,
    at: str,
    *,
    maximum: int,
) -> list[str]:
    if not isinstance(value, str):
        return [f"{at}: expected string"]
    if not value or value != value.strip():
        return [f"{at}: must contain trimmed non-whitespace text"]
    if len(value) > maximum:
        return [f"{at}: string exceeds {maximum} characters"]
    if any(ord(character) < 32 or ord(character) == 127 for character in value):
        return [f"{at}: control characters are forbidden"]
    return []


def _repo_path_errors(value: Any, at: str) -> list[str]:
    if not isinstance(value, str):
        return [f"{at}: expected repository-relative path"]
    errors = _trimmed_text(value, at, maximum=512)
    if errors:
        return errors
    parts = value.split("/")
    if (
        value.startswith("/")
        or value.endswith("/")
        or "\\" in value
        or ":" in value
        or "//" in value
        or any(part in ("", ".", "..") for part in parts)
    ):
        return [f"{at}: path is not canonical repository-relative form"]
    return []


def _canonical_repository(value: Any, at: str) -> list[str]:
    if not isinstance(value, str):
        return [f"{at}: expected canonical HTTPS repository URL"]
    try:
        parsed = urlsplit(value)
        _ = parsed.port
        path_parts = parsed.path.split("/")
        canonical = (
            parsed.scheme == "https"
            and bool(parsed.hostname)
            and parsed.username is None
            and parsed.password is None
            and not parsed.query
            and not parsed.fragment
            and value == value.strip()
            and not any(
                character.isspace()
                or ord(character) < 32
                or ord(character) == 127
                for character in value
            )
            and "\\" not in value
            and "%" not in parsed.path
            and parsed.path.startswith("/")
            and not parsed.path.endswith("/")
            and len(path_parts) > 1
            and all(part not in ("", ".", "..") for part in path_parts[1:])
        )
    except ValueError:
        canonical = False
    return [] if canonical else [f"{at}: expected canonical credential-free HTTPS repository URL"]


def _source_errors(source: Any, at: str) -> list[str]:
    if not isinstance(source, dict):
        return [f"{at}: expected object"]
    kind = source.get("kind")
    if kind == "git":
        expected = {"kind", "repository", "revision"}
    elif kind == "artifact":
        expected = {"kind", "artifact_id", "content_digest"}
    else:
        expected = {"kind"}
    errors = _exact_fields(source, expected, at)
    if kind not in ("git", "artifact"):
        errors.append(f"{at}.kind: expected git or artifact")
        return errors
    if errors:
        return errors
    if kind == "git":
        errors.extend(_canonical_repository(source["repository"], f"{at}.repository"))
        revision = source["revision"]
        if not isinstance(revision, str) or _GIT_OID.fullmatch(revision) is None:
            errors.append(f"{at}.revision: expected full lowercase Git object ID")
    else:
        errors.extend(_trimmed_text(source["artifact_id"], f"{at}.artifact_id", maximum=256))
        digest = source["content_digest"]
        if not isinstance(digest, str) or _SHA256.fullmatch(digest) is None:
            errors.append(f"{at}.content_digest: expected sha256:<64 lowercase hex>")
    return errors


def _source_identity(source: dict[str, Any]) -> tuple[str, ...]:
    if source["kind"] == "git":
        return ("git", source["repository"], source["revision"])
    return ("artifact", source["artifact_id"], source["content_digest"])


def _license_errors(license_data: Any, at: str) -> list[str]:
    expected = {
        "spdx_expression",
        "source_license_path",
        "snapshot_path",
        "snapshot_sha256",
    }
    errors = _exact_fields(license_data, expected, at)
    if errors or not isinstance(license_data, dict):
        return errors
    errors.extend(
        _trimmed_text(
            license_data["spdx_expression"],
            f"{at}.spdx_expression",
            maximum=256,
        )
    )
    errors.extend(
        _repo_path_errors(license_data["source_license_path"], f"{at}.source_license_path")
    )
    errors.extend(_repo_path_errors(license_data["snapshot_path"], f"{at}.snapshot_path"))
    snapshot_path = license_data["snapshot_path"]
    if isinstance(snapshot_path, str) and not snapshot_path.startswith(
        "third_party/notices/licenses/"
    ):
        errors.append(
            f"{at}.snapshot_path: snapshot must live under third_party/notices/licenses/"
        )
    digest = license_data["snapshot_sha256"]
    if not isinstance(digest, str) or _SHA256.fullmatch(digest) is None:
        errors.append(f"{at}.snapshot_sha256: expected sha256:<64 lowercase hex>")
    return errors


def validate_inventory(data: Any) -> tuple[list[str], list[dict[str, Any]]]:
    errors = _exact_fields(data, {"schema_version", "entries"}, "$")
    if errors or not isinstance(data, dict):
        return sorted(set(errors)), []
    version = data["schema_version"]
    if isinstance(version, bool) or version != 1:
        errors.append("$.schema_version: expected integer 1")
    entries = data["entries"]
    if not isinstance(entries, list):
        errors.append("$.entries: expected array")
        return sorted(set(errors)), []
    if not entries:
        errors.append("$.entries: at least one notice entry is required")
    if len(entries) > MAX_ENTRIES:
        errors.append(f"$.entries: exceeds {MAX_ENTRIES}-entry limit")

    valid_entries: list[dict[str, Any]] = []
    entry_ids: dict[str, int] = {}
    source_ids: dict[tuple[str, ...], int] = {}
    snapshots: dict[str, int] = {}

    for index, entry in enumerate(entries):
        at = f"$.entries[{index}]"
        entry_errors = _exact_fields(
            entry,
            {"entry_id", "category", "source", "license"},
            at,
        )
        if entry_errors or not isinstance(entry, dict):
            errors.extend(entry_errors)
            continue
        entry_id = entry["entry_id"]
        if not isinstance(entry_id, str) or _ENTRY_ID.fullmatch(entry_id) is None:
            errors.append(f"{at}.entry_id: expected canonical notice entry ID")
        category = entry["category"]
        if category not in _CATEGORIES:
            errors.append(f"{at}.category: value is not in the allowed enum")

        source_errors = _source_errors(entry["source"], f"{at}.source")
        license_errors = _license_errors(entry["license"], f"{at}.license")
        errors.extend(source_errors)
        errors.extend(license_errors)
        if source_errors or license_errors:
            continue

        assert isinstance(entry_id, str)
        prior_entry = entry_ids.setdefault(entry_id, index)
        if prior_entry != index:
            errors.append(
                f"{at}.entry_id: duplicate entry_id {entry_id} already declared at $.entries[{prior_entry}]"
            )

        identity = _source_identity(entry["source"])
        prior_source = source_ids.setdefault(identity, index)
        if prior_source != index:
            errors.append(
                f"{at}.source: duplicate source identity already declared at $.entries[{prior_source}]"
            )

        snapshot = entry["license"]["snapshot_path"]
        prior_snapshot = snapshots.setdefault(snapshot, index)
        if prior_snapshot != index:
            errors.append(
                f"{at}.license.snapshot_path: duplicate snapshot path already declared at $.entries[{prior_snapshot}]"
            )
        valid_entries.append(entry)

    return sorted(set(errors)), valid_entries


def _sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        while chunk := handle.read(1024 * 1024):
            digest.update(chunk)
    return digest.hexdigest()


def _snapshot_errors(entry: dict[str, Any], index: int, root: Path = ROOT) -> list[str]:
    at = f"$.entries[{index}].license"
    relative = entry["license"]["snapshot_path"]
    errors = provenance._repository_path_errors(
        root,
        relative,
        f"{at}.snapshot_path",
        require_file=True,
    )
    if errors:
        return errors
    path = root / relative
    try:
        actual = _sha256(path)
    except OSError as exc:
        return [f"{at}.snapshot_path: cannot hash snapshot: {exc}"]
    expected = entry["license"]["snapshot_sha256"].removeprefix("sha256:")
    if actual != expected:
        return [
            f"{at}.snapshot_sha256: digest mismatch; expected {expected}, observed {actual}"
        ]
    return []


def _load_manifest_data(
    paths: list[Path],
) -> tuple[list[str], list[tuple[Path, dict[str, Any]]]]:
    errors: list[str] = []
    records: list[tuple[Path, dict[str, Any]]] = []
    try:
        schema = provenance.read_json(provenance.SCHEMA)
    except (OSError, provenance.ProvenanceError) as exc:
        raise NoticeError(f"cannot load provenance schema: {exc}") from exc
    if not isinstance(schema, dict):
        raise NoticeError("provenance schema must be a JSON object")
    contract_errors = provenance._schema_contract_errors(schema)
    if contract_errors:
        raise NoticeError("provenance schema contract is invalid: " + "; ".join(contract_errors))

    for path in paths:
        try:
            data = provenance.read_json(path)
        except provenance.ProvenanceError as exc:
            errors.append(str(exc))
            continue
        data_errors = provenance.validate_data(data, schema)
        if data_errors:
            errors.extend(f"{path}: {error}" for error in data_errors)
            continue
        if not isinstance(data, dict):
            errors.append(f"{path}: expected provenance object")
            continue
        records.append((path, data))
    return errors, records


def crosscheck_provenance(
    entries: list[dict[str, Any]],
    manifest_paths: list[Path],
) -> list[str]:
    errors, records = _load_manifest_data(manifest_paths)
    by_source = {_source_identity(entry["source"]): entry for entry in entries}

    for path, record in records:
        identity = _source_identity(record["source"])
        entry = by_source.get(identity)
        if entry is None:
            errors.append(f"{path}: source identity has no matching notice inventory entry")
            continue
        inventory_license = entry["license"]
        manifest_license = record["license"]
        if manifest_license["spdx_expression"] != inventory_license["spdx_expression"]:
            errors.append(f"{path}: license SPDX expression does not match notice inventory")
        if (
            manifest_license["source_license_path"]
            != inventory_license["source_license_path"]
        ):
            errors.append(f"{path}: source license path does not match notice inventory")
        snapshot = inventory_license["snapshot_path"]
        if snapshot not in manifest_license["evidence_paths"]:
            errors.append(
                f"{path}: license evidence_paths must include notice snapshot {snapshot}"
            )
    return sorted(set(errors))


def validate_repository(
    data: Any,
    *,
    root: Path = ROOT,
    manifest_paths: list[Path] | None = None,
) -> tuple[list[str], list[dict[str, Any]], int]:
    errors, entries = validate_inventory(data)
    if errors:
        return errors, entries, 0

    for index, entry in enumerate(entries):
        errors.extend(_snapshot_errors(entry, index, root))

    if manifest_paths is None:
        try:
            manifest_paths = provenance._manifest_paths(
                root / "third_party/provenance/manifests"
            )
        except provenance.ProvenanceError as exc:
            errors.append(str(exc))
            manifest_paths = []

    errors.extend(crosscheck_provenance(entries, manifest_paths))
    return sorted(set(errors)), entries, len(manifest_paths)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("command", choices=["check"])
    args = parser.parse_args(argv)

    try:
        location_errors = provenance._repository_path_errors(
            ROOT,
            "third_party/notices/inventory.json",
            "inventory",
            require_file=True,
        )
        if location_errors:
            raise NoticeError("; ".join(location_errors))
        data = provenance.read_json(INVENTORY)
        errors, entries, manifest_count = validate_repository(data)
    except (KeyError, OSError, TypeError, NoticeError, provenance.ProvenanceError) as exc:
        print(f"Notice validation: ERROR\n- {exc}", file=sys.stderr)
        return 2

    if errors:
        print("Notice validation: FAIL", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1

    print("Notice validation: PASS")
    print(f"Inventory: {INVENTORY.relative_to(ROOT)}")
    print(f"Entries: {len(entries)}")
    print(f"Provenance manifests: {manifest_count}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
