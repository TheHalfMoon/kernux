#!/usr/bin/env python3
"""Conformance tests for Kernux third-party notice inventory enforcement."""

from __future__ import annotations

import copy
import json
import pathlib
import shutil
import tempfile
import unittest

from tools.provenance import validate as provenance
from tools.provenance import validate_notices as notices


class NoticeInventoryTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.inventory = provenance.read_json(notices.INVENTORY)
        if not isinstance(cls.inventory, dict):
            raise AssertionError("canonical notice inventory must be an object")

    def inventory_copy(self) -> dict[str, object]:
        return copy.deepcopy(self.inventory)

    def assert_invalid(self, data: object, fragment: str) -> None:
        errors, _ = notices.validate_inventory(data)
        self.assertTrue(errors, "inventory unexpectedly passed")
        self.assertTrue(
            any(fragment in error for error in errors),
            f"expected {fragment!r} in {errors!r}",
        )

    def materialize_snapshots(self, root: pathlib.Path) -> None:
        for entry in self.inventory["entries"]:
            relative = entry["license"]["snapshot_path"]
            target = root / relative
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(notices.ROOT / relative, target)

    def prepared_manifest(
        self,
        entry: dict[str, object],
        *,
        evidence: bool = True,
    ) -> dict[str, object]:
        license_data = entry["license"]
        return {
            "schema_version": 1,
            "record_id": f"{entry['entry_id']}-prepared",
            "status": "prepared",
            "donor": {"name": entry["entry_id"], "role": "test-donor"},
            "source": copy.deepcopy(entry["source"]),
            "authorization": [
                {"basis": "public-license", "reference": "upstream LICENSE"}
            ],
            "license": {
                "spdx_expression": license_data["spdx_expression"],
                "source_license_path": license_data["source_license_path"],
                "evidence_paths": (
                    [license_data["snapshot_path"]] if evidence else []
                ),
            },
            "mappings": [
                {
                    "source_paths": ["src"],
                    "destination_paths": [],
                    "transformation": "reference-only",
                }
            ],
            "dependency_review": {
                "status": "complete",
                "evidence_paths": [],
                "notes": "Reference-only preparation has no copied dependencies.",
            },
            "characterization": {
                "status": "planned",
                "test_paths": [],
                "reason": None,
            },
            "security_review": {
                "status": "not-required",
                "evidence_paths": [],
                "reason": "Reference-only preparation.",
            },
            "import": {"commit": None, "adaptation_commits": []},
        }

    def manifest_file(
        self,
        root: pathlib.Path,
        manifest: dict[str, object],
        name: str = "manifest.json",
    ) -> pathlib.Path:
        path = root / name
        path.write_text(
            json.dumps(manifest, sort_keys=True),
            encoding="utf-8",
        )
        return path

    def test_canonical_inventory_and_repository_state_pass(self) -> None:
        errors, entries, manifest_count = notices.validate_repository(self.inventory)
        self.assertEqual(errors, [])
        self.assertGreaterEqual(len(entries), 3)
        self.assertEqual(manifest_count, len(provenance._manifest_paths()))

    def test_founding_donor_pins_and_license_digests_are_exact(self) -> None:
        expected = {
            "agentql": (
                "https://github.com/tinyfish-io/agentql",
                "418ba8ad1c69dfac134a6833369a01dfba5a24a7",
                "MIT",
                "sha256:167782868139652b3db1527f8a66a41a550cff0379373d0cb3beff44db28f883",
            ),
            "desktop-commander": (
                "https://github.com/wonderwhy-er/DesktopCommanderMCP",
                "08ff76192919a6e8fe2557b39c3f99e7b54c3b92",
                "MIT",
                "sha256:e77cff545e0506903bf6a4bf29169fa40911eb76d70d239267df10c5c59370dd",
            ),
            "orca": (
                "https://github.com/stablyai/orca",
                "0d23ea6e688410c878096dab8b1779857354b7d4",
                "MIT",
                "sha256:ff1b611f80580d49f4b97e93a97b24eb050b0671b26b8afe16341fab699112f3",
            ),
        }
        observed = {
            entry["entry_id"]: (
                entry["source"]["repository"],
                entry["source"]["revision"],
                entry["license"]["spdx_expression"],
                entry["license"]["snapshot_sha256"],
            )
            for entry in self.inventory["entries"]
        }
        for entry_id, values in expected.items():
            self.assertIn(entry_id, observed)
            self.assertEqual(observed[entry_id], values)

    def test_schema_version_requires_integer_one(self) -> None:
        for value in (True, 1.0, 0, 2, "1"):
            with self.subTest(value=value):
                data = self.inventory_copy()
                data["schema_version"] = value
                self.assert_invalid(data, "expected integer 1")

    def test_unknown_and_missing_fields_fail_closed(self) -> None:
        data = self.inventory_copy()
        data["unexpected"] = True
        self.assert_invalid(data, "unknown properties")

        data = self.inventory_copy()
        del data["entries"][0]["license"]["snapshot_sha256"]
        self.assert_invalid(data, "missing properties")

    def test_entry_id_and_category_are_strict(self) -> None:
        data = self.inventory_copy()
        data["entries"][0]["entry_id"] = "Bad Entry"
        self.assert_invalid(data, "canonical notice entry ID")

        data = self.inventory_copy()
        data["entries"][0]["category"] = []
        self.assert_invalid(data, "allowed enum")

    def test_git_source_identity_is_canonical(self) -> None:
        bad_urls = (
            "http://github.com/stablyai/orca",
            "https://user:secret@github.com/stablyai/orca",
            "https://github.com/stablyai/orca/",
            "https://github.com/stablyai/../orca",
            "https://github.com/stablyai/%6frca",
            "https://github.com/stablyai/orca?ref=main",
        )
        for value in bad_urls:
            with self.subTest(value=value):
                data = self.inventory_copy()
                data["entries"][2]["source"]["repository"] = value
                self.assert_invalid(data, "canonical credential-free HTTPS")

        data = self.inventory_copy()
        data["entries"][2]["source"]["revision"] = "main"
        self.assert_invalid(data, "full lowercase Git object ID")

    def test_artifact_source_identity_is_supported_and_strict(self) -> None:
        data = self.inventory_copy()
        source = {
            "kind": "artifact",
            "artifact_id": "private-source-bundle",
            "content_digest": "sha256:" + "a" * 64,
        }
        data["entries"][0]["source"] = source
        errors, _ = notices.validate_inventory(data)
        self.assertEqual(errors, [])

        data["entries"][0]["source"]["repository"] = "https://example.com/source"
        self.assert_invalid(data, "unknown properties")

    def test_duplicate_ids_sources_and_snapshot_paths_are_rejected(self) -> None:
        data = self.inventory_copy()
        data["entries"][1]["entry_id"] = data["entries"][0]["entry_id"]
        self.assert_invalid(data, "duplicate entry_id")

        data = self.inventory_copy()
        data["entries"][1]["source"] = copy.deepcopy(data["entries"][0]["source"])
        self.assert_invalid(data, "duplicate source identity")

        data = self.inventory_copy()
        data["entries"][1]["license"]["snapshot_path"] = (
            data["entries"][0]["license"]["snapshot_path"]
        )
        self.assert_invalid(data, "duplicate snapshot path")

    def test_unsafe_license_paths_and_digest_shapes_are_rejected(self) -> None:
        bad_paths = (
            "../LICENSE",
            "/LICENSE",
            "C:/LICENSE",
            "dir\\LICENSE",
            "dir//LICENSE",
            "./LICENSE",
            "dir/../LICENSE",
            "trailing/",
            " leading/LICENSE",
            "trailing/LICENSE ",
        )
        for value in bad_paths:
            with self.subTest(value=value):
                data = self.inventory_copy()
                data["entries"][0]["license"]["snapshot_path"] = value
                self.assert_invalid(data, "snapshot")

        data = self.inventory_copy()
        data["entries"][0]["license"]["snapshot_sha256"] = "sha256:xyz"
        self.assert_invalid(data, "expected sha256")

    def test_digest_mismatch_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            self.materialize_snapshots(root)
            data = self.inventory_copy()
            data["entries"][0]["license"]["snapshot_sha256"] = "sha256:" + "0" * 64
            errors, _, _ = notices.validate_repository(
                data,
                root=root,
                manifest_paths=[],
            )
            self.assertTrue(any("digest mismatch" in error for error in errors), errors)

    def test_symlink_snapshot_and_license_root_are_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            self.materialize_snapshots(root)
            relative = self.inventory["entries"][0]["license"]["snapshot_path"]
            snapshot = root / relative
            snapshot.unlink()
            target = root / "real-license"
            target.write_text("replacement", encoding="utf-8")
            try:
                snapshot.symlink_to(target)
            except OSError as exc:
                self.skipTest(f"symlink creation unavailable: {exc}")
            errors, _, _ = notices.validate_repository(
                self.inventory,
                root=root,
                manifest_paths=[],
            )
            self.assertTrue(any("symlink" in error for error in errors), errors)

        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            self.materialize_snapshots(root)
            license_root = root / "third_party/notices/licenses"
            moved = root / "licenses-real"
            license_root.rename(moved)
            try:
                license_root.symlink_to(moved, target_is_directory=True)
            except OSError as exc:
                self.skipTest(f"directory symlink creation unavailable: {exc}")
            errors, _, _ = notices.validate_repository(
                self.inventory,
                root=root,
                manifest_paths=[],
            )
            self.assertTrue(
                any("license snapshot directory must not be a symlink" in error for error in errors),
                errors,
            )

    def test_untracked_license_snapshot_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            self.materialize_snapshots(root)
            extra = root / "third_party/notices/licenses/untracked/LICENSE"
            extra.parent.mkdir(parents=True)
            extra.write_text("untracked", encoding="utf-8")
            errors, _, _ = notices.validate_repository(
                self.inventory,
                root=root,
                manifest_paths=[],
            )
            self.assertTrue(
                any("untracked license snapshot" in error for error in errors),
                errors,
            )

    def test_reused_json_reader_rejects_adversarial_inventory_bytes(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)

            duplicate = root / "duplicate.json"
            duplicate.write_text('{"schema_version":1,"schema_version":1}', encoding="utf-8")
            with self.assertRaisesRegex(provenance.ProvenanceError, "duplicate object key"):
                provenance.read_json(duplicate)

            non_utf8 = root / "non-utf8.json"
            non_utf8.write_bytes(b'{"x":"\xff"}')
            with self.assertRaisesRegex(provenance.ProvenanceError, "not valid UTF-8"):
                provenance.read_json(non_utf8)

            oversized = root / "oversized.json"
            oversized.write_bytes(b" " * (provenance.MAX_BYTES + 1))
            with self.assertRaisesRegex(provenance.ProvenanceError, "exceeds"):
                provenance.read_json(oversized)

    def test_provenance_source_requires_matching_notice_entry(self) -> None:
        entry = self.inventory["entries"][2]
        manifest = self.prepared_manifest(entry)
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            path = self.manifest_file(root, manifest)
            entries = [
                item
                for item in self.inventory["entries"]
                if item["entry_id"] != entry["entry_id"]
            ]
            errors = notices.crosscheck_provenance(entries, [path])
        self.assertTrue(
            any("no matching notice inventory entry" in error for error in errors),
            errors,
        )

    def test_provenance_license_fields_must_match_inventory(self) -> None:
        entry = self.inventory["entries"][2]
        cases = (
            ("spdx_expression", "Apache-2.0", "SPDX expression"),
            ("source_license_path", "COPYING", "source license path"),
        )
        for field, value, fragment in cases:
            with self.subTest(field=field):
                manifest = self.prepared_manifest(entry)
                manifest["license"][field] = value
                with tempfile.TemporaryDirectory() as temporary:
                    root = pathlib.Path(temporary)
                    path = self.manifest_file(root, manifest)
                    errors = notices.crosscheck_provenance(
                        self.inventory["entries"],
                        [path],
                    )
                self.assertTrue(any(fragment in error for error in errors), errors)

    def test_provenance_must_reference_preserved_snapshot(self) -> None:
        entry = self.inventory["entries"][2]
        manifest = self.prepared_manifest(entry, evidence=False)
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            path = self.manifest_file(root, manifest)
            errors = notices.crosscheck_provenance(self.inventory["entries"], [path])
        self.assertTrue(any("must include notice snapshot" in error for error in errors), errors)

    def test_matching_prepared_provenance_record_passes_crosscheck(self) -> None:
        entry = self.inventory["entries"][2]
        manifest = self.prepared_manifest(entry)
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            path = self.manifest_file(root, manifest)
            errors = notices.crosscheck_provenance(self.inventory["entries"], [path])
        self.assertEqual(errors, [])

    def test_inventory_diagnostics_are_deterministic(self) -> None:
        data = self.inventory_copy()
        data["schema_version"] = 1.0
        data["entries"][0]["category"] = []
        first, _ = notices.validate_inventory(copy.deepcopy(data))
        second, _ = notices.validate_inventory(copy.deepcopy(data))
        self.assertEqual(first, second)
        self.assertEqual(first, sorted(set(first)))


if __name__ == "__main__":
    unittest.main()
