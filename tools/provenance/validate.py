#!/usr/bin/env python3
"""Strict dependency-free validator for Kernux donor provenance manifests."""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path
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
    source = data["source"]
    if source["kind"] == "git":
        if "repository" not in source or "revision" not in source:
            errors.append("$.source: git source requires repository and revision")
        if "artifact_id" in source or "content_digest" in source:
            errors.append("$.source: git source must not declare artifact identity")
    else:
        if "artifact_id" not in source or "content_digest" not in source:
            errors.append("$.source: artifact source requires artifact_id and content_digest")
        if "repository" in source or "revision" in source:
            errors.append("$.source: artifact source must not declare git identity")

    for index, mapping in enumerate(data["mappings"]):
        destinations = mapping["destination_paths"]
        if mapping["transformation"] == "reference-only" and destinations:
            errors.append(f"$.mappings[{index}]: reference-only mapping must have no destinations")
        if mapping["transformation"] != "reference-only" and not destinations:
            errors.append(f"$.mappings[{index}]: imported mapping requires a destination")

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
        paths = [Path(item) for item in args.paths] if args.paths else sorted(MANIFESTS.glob("*.json"))
        record_ids: dict[str, Path] = {}
        for path in paths:
            try:
                data = read_json(path)
            except ProvenanceError as exc:
                errors.append(str(exc))
                continue
            errors.extend(validate_data(data, schema_data))
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
