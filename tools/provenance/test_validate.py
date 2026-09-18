#!/usr/bin/env python3
"""Conformance and adversarial tests for the Kernux provenance contract."""

from __future__ import annotations

import copy
import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from typing import Any

from tools.provenance import validate


def _run_git(root: Path, *args: str) -> str:
    result = subprocess.run(
        ["git", *args],
        cwd=root,
        text=True,
        capture_output=True,
        check=True,
    )
    return result.stdout.strip()


def _write_json(path: Path, value: Any) -> None:
    path.write_text(json.dumps(value, sort_keys=True), encoding="utf-8")


class ProvenanceConformanceTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        cls.schema = validate.read_json(validate.SCHEMA)
        if not isinstance(cls.schema, dict):
            raise AssertionError("provenance schema must be an object")

    def prepared_manifest(self) -> dict[str, Any]:
        return {
            "schema_version": 1,
            "record_id": "donor-example",
            "status": "prepared",
            "donor": {"name": "Example Donor", "role": "workspace-runtime"},
            "source": {
                "kind": "git",
                "repository": "https://github.com/example/donor",
                "revision": "1" * 40,
            },
            "authorization": [
                {"basis": "public-license", "reference": "upstream LICENSE"}
            ],
            "license": {
                "spdx_expression": "MIT",
                "source_license_path": "LICENSE",
                "evidence_paths": ["README.md"],
            },
            "mappings": [
                {
                    "source_paths": ["src/example.py"],
                    "destination_paths": ["tools/workspace/check.mjs"],
                    "transformation": "adapted",
                }
            ],
            "dependency_review": {
                "status": "complete",
                "evidence_paths": ["README.md"],
                "notes": "Dependency obligations reviewed.",
            },
            "characterization": {
                "status": "planned",
                "test_paths": [],
                "reason": None,
            },
            "security_review": {
                "status": "not-required",
                "evidence_paths": [],
                "reason": "No privileged or secret-bearing surface.",
            },
            "import": {"commit": None, "adaptation_commits": []},
        }

    def assert_invalid(self, manifest: dict[str, Any], fragment: str) -> None:
        errors = validate.validate_data(manifest, self.schema)
        self.assertTrue(errors, "manifest unexpectedly passed")
        self.assertTrue(
            any(fragment in error for error in errors),
            f"expected {fragment!r} in {errors!r}",
        )

    def test_valid_prepared_record(self) -> None:
        self.assertEqual(validate.validate_data(self.prepared_manifest(), self.schema), [])

    def test_unknown_property_fails_closed(self) -> None:
        manifest = self.prepared_manifest()
        manifest["unexpected"] = True
        self.assert_invalid(manifest, "unknown property unexpected")

    def test_schema_contract_is_strict_and_current(self) -> None:
        self.assertEqual(validate._schema_contract_errors(self.schema), [])

        drifted = copy.deepcopy(self.schema)
        drifted["$defs"]["donor"].pop("additionalProperties")
        errors = validate._schema_contract_errors(drifted)
        self.assertTrue(
            any("object schema must set additionalProperties=false" in error for error in errors),
            errors,
        )


    def test_whitespace_and_padded_identity_fields_are_rejected(self) -> None:
        cases = [
            (("donor", "name"), "   "),
            (("donor", "role"), " role"),
            (("authorization", 0, "reference"), "license "),
            (("license", "spdx_expression"), " MIT"),
            (("dependency_review", "notes"), "   "),
            (("security_review", "reason"), " reason "),
        ]
        for path, bad_value in cases:
            with self.subTest(path=path):
                manifest = self.prepared_manifest()
                target: Any = manifest
                for component in path[:-1]:
                    target = target[component]
                target[path[-1]] = bad_value
                self.assert_invalid(manifest, "trimmed non-whitespace text")

    def test_unsafe_repository_paths_are_rejected(self) -> None:
        bad_paths = [
            "../escape",
            "/absolute",
            "C:/drive",
            "dir\\file",
            "dir//file",
            "./relative",
            "dir/../file",
            "trailing/",
            " leading/file",
            "trailing/file ",
        ]
        for bad_path in bad_paths:
            with self.subTest(path=bad_path):
                manifest = self.prepared_manifest()
                manifest["mappings"][0]["source_paths"] = [bad_path]
                self.assert_invalid(manifest, "required pattern")

    def test_valid_unicode_repository_path_is_accepted(self) -> None:
        manifest = self.prepared_manifest()
        manifest["mappings"][0]["source_paths"] = ["src/بحث صحي/example file.py"]
        self.assertEqual(validate.validate_data(manifest, self.schema), [])

    def test_git_source_identity_is_unambiguous(self) -> None:
        manifest = self.prepared_manifest()
        manifest["source"]["artifact_id"] = "also-an-artifact"
        self.assert_invalid(manifest, "must not declare artifact identity")

        manifest = self.prepared_manifest()
        del manifest["source"]["revision"]
        self.assert_invalid(manifest, "requires repository and revision")

    def test_artifact_source_identity_is_unambiguous(self) -> None:
        manifest = self.prepared_manifest()
        manifest["source"] = {
            "kind": "artifact",
            "artifact_id": "archive-1",
            "content_digest": "sha256:" + "a" * 64,
        }
        self.assertEqual(validate.validate_data(manifest, self.schema), [])

        manifest["source"]["repository"] = "https://github.com/example/donor"
        self.assert_invalid(manifest, "must not declare git identity")

    def test_git_repository_url_must_be_canonical(self) -> None:
        bad_urls = [
            "http://github.com/example/donor",
            "https://user:secret@github.com/example/donor",
            "https://github.com/example/donor/",
            "https://github.com/example//donor",
            "https://github.com/example/../donor",
            "https://github.com/example/%64onor",
            "https://github.com/example/donor?ref=main",
            "https://github.com/example/donor#fragment",
        ]
        for url in bad_urls:
            with self.subTest(url=url):
                manifest = self.prepared_manifest()
                manifest["source"]["repository"] = url
                self.assertTrue(
                    validate.validate_data(manifest, self.schema),
                    f"non-canonical URL unexpectedly passed: {url}",
                )

    def test_mapping_semantics_are_enforced(self) -> None:
        manifest = self.prepared_manifest()
        manifest["mappings"][0]["transformation"] = "reference-only"
        self.assert_invalid(manifest, "reference-only mapping must have no destinations")

        manifest = self.prepared_manifest()
        manifest["mappings"][0]["destination_paths"] = []
        self.assert_invalid(manifest, "imported mapping requires a destination")

        manifest = self.prepared_manifest()
        manifest["mappings"][0]["transformation"] = "generated"
        self.assert_invalid(manifest, "requires generator provenance")

    def test_prepared_state_cannot_claim_import_commits(self) -> None:
        manifest = self.prepared_manifest()
        manifest["import"]["commit"] = "a" * 40
        self.assert_invalid(manifest, "prepared status must not declare an import commit")

        manifest = self.prepared_manifest()
        manifest["import"]["adaptation_commits"] = ["b" * 40]
        self.assert_invalid(manifest, "prepared status must not declare adaptation commits")

    def test_read_json_rejects_duplicate_keys_non_utf8_and_oversize(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)

            duplicate = root / "duplicate.json"
            duplicate.write_text('{"a":1,"a":2}', encoding="utf-8")
            with self.assertRaisesRegex(validate.ProvenanceError, "duplicate object key"):
                validate.read_json(duplicate)

            non_utf8 = root / "non-utf8.json"
            non_utf8.write_bytes(b'{"x":"\xff"}')
            with self.assertRaisesRegex(validate.ProvenanceError, "not valid UTF-8"):
                validate.read_json(non_utf8)

            oversized = root / "oversized.json"
            oversized.write_bytes(b" " * (validate.MAX_BYTES + 1))
            with self.assertRaisesRegex(validate.ProvenanceError, "exceeds"):
                validate.read_json(oversized)

    def test_read_json_rejects_symlink_input(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            target = root / "target.json"
            target.write_text("{}", encoding="utf-8")
            link = root / "link.json"
            try:
                link.symlink_to(target)
            except OSError as exc:
                self.skipTest(f"symlink creation unavailable: {exc}")
            with self.assertRaisesRegex(validate.ProvenanceError, "symlink inputs are forbidden"):
                validate.read_json(link)

    def _init_git_repo(self, root: Path) -> None:
        _run_git(root, "init", "-q")
        _run_git(root, "config", "user.name", "Kernux Test")
        _run_git(root, "config", "user.email", "kernux-test@example.invalid")

    def _imported_manifest(
        self,
        *,
        import_commit: str,
        destination: str = "imported/example.py",
        adaptation_commits: list[str] | None = None,
    ) -> dict[str, Any]:
        manifest = self.prepared_manifest()
        manifest["status"] = "imported"
        manifest["license"]["evidence_paths"] = ["evidence/license.txt"]
        manifest["dependency_review"]["evidence_paths"] = ["evidence/dependencies.txt"]
        manifest["characterization"] = {
            "status": "complete",
            "test_paths": ["tests/characterization.txt"],
            "reason": None,
        }
        manifest["security_review"] = {
            "status": "complete",
            "evidence_paths": ["evidence/security.txt"],
            "reason": "Security review completed.",
        }
        manifest["mappings"][0]["destination_paths"] = [destination]
        manifest["import"] = {
            "commit": import_commit,
            "adaptation_commits": adaptation_commits or [],
        }
        return manifest

    def test_valid_imported_record_and_adaptation_chain(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            self._init_git_repo(root)
            for relative in (
                "evidence/license.txt",
                "evidence/dependencies.txt",
                "evidence/security.txt",
                "tests/characterization.txt",
                "imported/example.py",
            ):
                path = root / relative
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text(f"{relative}\n", encoding="utf-8")
            _run_git(root, "add", ".")
            _run_git(root, "commit", "-qm", "import donor")
            import_commit = _run_git(root, "rev-parse", "HEAD")

            (root / "imported/example.py").write_text("adapted = True\n", encoding="utf-8")
            _run_git(root, "add", "imported/example.py")
            _run_git(root, "commit", "-qm", "adapt donor")
            adaptation_commit = _run_git(root, "rev-parse", "HEAD")

            manifest = self._imported_manifest(
                import_commit=import_commit,
                adaptation_commits=[adaptation_commit],
            )
            self.assertEqual(validate.validate_data(manifest, self.schema), [])
            self.assertEqual(validate._repository_errors(manifest, root), [])

    def test_import_commit_must_touch_claimed_destination(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            self._init_git_repo(root)

            (root / "evidence").mkdir()
            (root / "evidence/base.txt").write_text("base\n", encoding="utf-8")
            _run_git(root, "add", ".")
            _run_git(root, "commit", "-qm", "unrelated ancestor")
            unrelated = _run_git(root, "rev-parse", "HEAD")

            for relative in (
                "evidence/license.txt",
                "evidence/dependencies.txt",
                "evidence/security.txt",
                "tests/characterization.txt",
                "imported/example.py",
            ):
                path = root / relative
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text(f"{relative}\n", encoding="utf-8")
            _run_git(root, "add", ".")
            _run_git(root, "commit", "-qm", "actual import")

            manifest = self._imported_manifest(import_commit=unrelated)
            errors = validate._repository_errors(manifest, root)
            self.assertTrue(
                any("does not touch claimed destination" in error for error in errors),
                errors,
            )

    def test_non_descending_adaptation_commit_is_rejected(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            self._init_git_repo(root)
            default_branch = _run_git(root, "branch", "--show-current")
            (root / "base.txt").write_text("base\n", encoding="utf-8")
            _run_git(root, "add", "base.txt")
            _run_git(root, "commit", "-qm", "base")
            base_commit = _run_git(root, "rev-parse", "HEAD")

            for relative in (
                "evidence/license.txt",
                "evidence/dependencies.txt",
                "evidence/security.txt",
                "tests/characterization.txt",
                "imported/example.py",
            ):
                path = root / relative
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text(f"{relative}\n", encoding="utf-8")
            _run_git(root, "add", ".")
            _run_git(root, "commit", "-qm", "import")
            import_commit = _run_git(root, "rev-parse", "HEAD")

            _run_git(root, "checkout", "-qb", "side", base_commit)
            (root / "side.txt").write_text("side\n", encoding="utf-8")
            _run_git(root, "add", "side.txt")
            _run_git(root, "commit", "-qm", "side")
            side_commit = _run_git(root, "rev-parse", "HEAD")
            _run_git(root, "checkout", "-q", default_branch)

            manifest = self._imported_manifest(
                import_commit=import_commit,
                adaptation_commits=[side_commit],
            )
            errors = validate._repository_errors(manifest, root)
            self.assertTrue(
                any("must strictly descend" in error for error in errors),
                errors,
            )

    def _run_cli(self, manifests: list[dict[str, Any]]) -> subprocess.CompletedProcess[str]:
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        paths: list[str] = []
        for index, manifest in enumerate(manifests):
            path = Path(temporary.name) / f"manifest-{index}.json"
            _write_json(path, manifest)
            paths.append(str(path))
        return subprocess.run(
            [sys.executable, str(validate.ROOT / "tools/provenance/validate.py"), "check", *paths],
            cwd=validate.ROOT,
            text=True,
            capture_output=True,
            check=False,
        )

    def test_destination_claims_are_exclusive_across_records_and_hierarchy(self) -> None:
        first = self.prepared_manifest()
        first["record_id"] = "claim-one"
        first["mappings"][0]["destination_paths"] = ["tools"]

        second = self.prepared_manifest()
        second["record_id"] = "claim-two"
        second["source"]["revision"] = "2" * 40
        second["mappings"][0]["destination_paths"] = ["tools/workspace/check.mjs"]

        result = self._run_cli([first, second])
        self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
        self.assertIn("overlaps claim tools", result.stderr)

    def test_duplicate_record_ids_are_rejected(self) -> None:
        first = self.prepared_manifest()
        second = copy.deepcopy(first)
        second["source"]["revision"] = "2" * 40
        result = self._run_cli([first, second])
        self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
        self.assertIn("duplicate record_id", result.stderr)

    def test_manifest_ledger_rejects_hidden_state(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / "unexpected.txt").write_text("hidden\n", encoding="utf-8")
            with self.assertRaisesRegex(validate.ProvenanceError, "unexpected non-JSON"):
                validate._manifest_paths(root)

        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / "nested").mkdir()
            with self.assertRaisesRegex(validate.ProvenanceError, "nested manifest directories"):
                validate._manifest_paths(root)

    def test_manifest_ledger_rejects_symlink_directory(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            target = root / "target"
            target.mkdir()
            link = root / "ledger"
            try:
                link.symlink_to(target, target_is_directory=True)
            except OSError as exc:
                self.skipTest(f"directory symlink creation unavailable: {exc}")
            with self.assertRaisesRegex(validate.ProvenanceError, "manifest directory must not be a symlink"):
                validate._manifest_paths(link)

    def test_diagnostics_are_sorted_and_deterministic(self) -> None:
        manifest = self.prepared_manifest()
        manifest["donor"]["name"] = " "
        manifest["donor"]["role"] = " "
        first = validate.validate_data(copy.deepcopy(manifest), self.schema)
        second = validate.validate_data(copy.deepcopy(manifest), self.schema)
        self.assertEqual(first, second)
        self.assertEqual(first, sorted(set(first)))


if __name__ == "__main__":
    unittest.main()
