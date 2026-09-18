#!/usr/bin/env python3
from __future__ import annotations

import contextlib
import io
import os
import sys
import tempfile
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import validate as pv


def prepared() -> dict:
    return {
        "schema_version": 1,
        "record_id": "orca.workspace-v1",
        "status": "prepared",
        "donor": {"name": "Orca", "role": "workspace/runtime donor"},
        "source": {"kind": "git", "repository": "stablyai/orca", "revision": "a" * 40},
        "authorization": [{"basis": "public-license", "reference": "MIT license at pinned source revision"}],
        "license": {"spdx_expression": "MIT", "source_license_path": "LICENSE", "evidence_paths": []},
        "mappings": [{"source_paths": ["src/main"], "destination_paths": ["adapters/orca"], "transformation": "adapted"}],
        "dependency_review": {"status": "complete", "evidence_paths": [], "notes": "Embedded dependency obligations reviewed before import."},
        "characterization": {"status": "planned", "test_paths": [], "reason": "Characterization executes before import."},
        "import": {"commit": None, "adaptation_commits": []}
    }


def imported() -> dict:
    value = prepared()
    value["record_id"] = "desktop-commander.documents-v1"
    value["status"] = "imported"
    value["source"] = {"kind": "artifact", "artifact_id": "desktop-commander-docs-v1", "content_digest": "sha256:" + "b" * 64}
    value["license"]["evidence_paths"] = ["third_party/notices/desktop-commander.txt"]
    value["characterization"] = {"status": "complete", "test_paths": ["tests/donor/desktop_commander.py"], "reason": None}
    value["import"]["commit"] = "c" * 40
    return value


class ProvenanceValidationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.schema = pv.read_json(pv.SCHEMA)

    def assert_invalid(self, value: dict, needle: str) -> None:
        errors = pv.validate_data(value, self.schema)
        self.assertTrue(errors)
        self.assertEqual(errors, sorted(errors))
        self.assertTrue(any(needle in error for error in errors), errors)

    def test_schema_contract_and_valid_states(self) -> None:
        self.assertEqual(pv._schema_contract_errors(self.schema), [])
        self.assertEqual(pv.validate_data(prepared(), self.schema), [])
        self.assertEqual(pv.validate_data(imported(), self.schema), [])

    def test_unknown_and_unsafe_paths_fail(self) -> None:
        value = prepared()
        value["surprise"] = True
        self.assert_invalid(value, "unknown property surprise")
        value = prepared()
        value["mappings"][0]["source_paths"] = ["../escape"]
        self.assert_invalid(value, "required pattern")

    def test_source_identity_must_be_unambiguous(self) -> None:
        value = prepared()
        value["source"]["artifact_id"] = "also-an-artifact"
        value["source"]["content_digest"] = "sha256:" + "d" * 64
        self.assert_invalid(value, "must not declare artifact identity")
        value = imported()
        del value["source"]["content_digest"]
        self.assert_invalid(value, "requires artifact_id and content_digest")

    def test_mapping_and_lifecycle_rules_fail_closed(self) -> None:
        value = prepared()
        value["mappings"][0] = {
            "source_paths": ["README.md"],
            "destination_paths": ["docs/copied.md"],
            "transformation": "reference-only"
        }
        self.assert_invalid(value, "reference-only mapping")
        value = prepared()
        value["import"]["commit"] = "e" * 40
        self.assert_invalid(value, "prepared status must not declare")
        value = imported()
        value["characterization"] = {"status": "planned", "test_paths": [], "reason": "later"}
        self.assert_invalid(value, "cannot remain planned")

    def test_duplicate_keys_non_utf8_and_oversize_are_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            duplicate = root / "duplicate.json"
            duplicate.write_text('{"schema_version":1,"schema_version":1}', encoding="utf-8")
            with self.assertRaisesRegex(pv.ProvenanceError, "duplicate object key"):
                pv.read_json(duplicate)
            binary = root / "binary.json"
            binary.write_bytes(b"\xff\xfe")
            with self.assertRaisesRegex(pv.ProvenanceError, "not valid UTF-8"):
                pv.read_json(binary)
            huge = root / "huge.json"
            huge.write_bytes(b"x" * (pv.MAX_BYTES + 1))
            with self.assertRaisesRegex(pv.ProvenanceError, "exceeds"):
                pv.read_json(huge)

    def test_symlink_input_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            target = root / "target.json"
            target.write_text("{}", encoding="utf-8")
            link = root / "link.json"
            try:
                os.symlink(target, link)
            except OSError as exc:
                self.skipTest(f"symlink unavailable: {exc}")
            with self.assertRaisesRegex(pv.ProvenanceError, "symlink inputs are forbidden"):
                pv.read_json(link)

    def test_duplicate_record_ids_across_files_fail(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            for name in ("one.json", "two.json"):
                (root / name).write_text(__import__("json").dumps(prepared()), encoding="utf-8")
            paths = (root / "one.json", root / "two.json")
            error = io.StringIO()
            with contextlib.redirect_stderr(error):
                self.assertEqual(pv.main(["check", *(str(path) for path in paths)]), 1)
            self.assertIn("duplicate record_id orca.workspace-v1", error.getvalue())

    def test_invalid_revision_digest_and_characterization_fail(self) -> None:
        value = prepared()
        value["source"]["revision"] = "main"
        self.assert_invalid(value, "required pattern")
        value = imported()
        value["source"]["content_digest"] = "sha256:bad"
        self.assert_invalid(value, "required pattern")
        value = imported()
        value["characterization"] = {"status": "not-applicable", "test_paths": [], "reason": None}
        self.assert_invalid(value, "requires reason")


if __name__ == "__main__":
    unittest.main()
