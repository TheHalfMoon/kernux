#!/usr/bin/env python3
"""Conformance tests for Kernux project-license enforcement."""

from __future__ import annotations

import json
import pathlib
import shutil
import tempfile
import unittest

from tools.provenance import validate_project_license as license_policy


class ProjectLicenseTests(unittest.TestCase):
    FILES = (
        "LICENSE",
        "docs/canonical/LICENSE_POLICY.md",
        "package.json",
        "Cargo.toml",
        "README.md",
        "third_party/README.md",
        "third_party/notices/README.md",
    )

    def materialize(self) -> tuple[tempfile.TemporaryDirectory[str], pathlib.Path]:
        temporary = tempfile.TemporaryDirectory()
        root = pathlib.Path(temporary.name)
        for relative in self.FILES:
            source = pathlib.Path(relative)
            destination = root / relative
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(source, destination)
        return temporary, root

    def assert_invalid(
        self,
        root: pathlib.Path,
        fragment: str,
    ) -> None:
        errors = license_policy.validate_repository(root)
        self.assertTrue(errors, "repository unexpectedly passed")
        self.assertTrue(
            any(fragment in error for error in errors),
            f"expected {fragment!r} in {errors!r}",
        )

    def test_canonical_repository_state_passes(self) -> None:
        self.assertEqual(license_policy.validate_repository(), [])

    def test_canonical_identities_are_pinned(self) -> None:
        self.assertEqual(license_policy.EXPECTED_SPDX, "Apache-2.0")
        self.assertEqual(
            license_policy.EXPECTED_LICENSE_SHA256,
            "cfc7749b96f63bd31c3c42b5c471bf756814053e847c10f3eb003417bc523d30",
        )
        self.assertEqual(
            license_policy.EXPECTED_POLICY_SHA256,
            "52036ff4149918166c6feb843798e2cdd023ddd6a7d6c0d3ef98d342709ceaf3",
        )

    def test_license_missing_tampered_and_symlinked_fail_closed(self) -> None:
        temporary, root = self.materialize()
        try:
            (root / "LICENSE").unlink()
            self.assert_invalid(root, "does not exist at validation head")
        finally:
            temporary.cleanup()

        temporary, root = self.materialize()
        try:
            (root / "LICENSE").write_text("tampered", encoding="utf-8")
            self.assert_invalid(root, "SHA-256 mismatch")
        finally:
            temporary.cleanup()

        temporary, root = self.materialize()
        try:
            path = root / "LICENSE"
            path.unlink()
            target = root / "real-license"
            target.write_bytes(pathlib.Path("LICENSE").read_bytes())
            try:
                path.symlink_to(target)
            except OSError as exc:
                self.skipTest(f"symlink creation unavailable: {exc}")
            self.assert_invalid(root, "must not be a symlink")
        finally:
            temporary.cleanup()

    def test_parent_directory_symlink_is_rejected(self) -> None:
        temporary, root = self.materialize()
        try:
            docs = root / "docs"
            moved = root / "docs-real"
            docs.rename(moved)
            try:
                docs.symlink_to(moved, target_is_directory=True)
            except OSError as exc:
                self.skipTest(f"directory symlink creation unavailable: {exc}")
            self.assert_invalid(root, "must not traverse a symlink")
        finally:
            temporary.cleanup()

    def test_policy_missing_tampered_and_symlinked_fail_closed(self) -> None:
        relative = pathlib.Path("docs/canonical/LICENSE_POLICY.md")

        temporary, root = self.materialize()
        try:
            (root / relative).unlink()
            self.assert_invalid(root, "does not exist at validation head")
        finally:
            temporary.cleanup()

        temporary, root = self.materialize()
        try:
            (root / relative).write_text("tampered", encoding="utf-8")
            self.assert_invalid(root, "SHA-256 mismatch")
        finally:
            temporary.cleanup()

        temporary, root = self.materialize()
        try:
            path = root / relative
            path.unlink()
            target = root / "real-policy"
            target.write_bytes(pathlib.Path(relative).read_bytes())
            try:
                path.symlink_to(target)
            except OSError as exc:
                self.skipTest(f"symlink creation unavailable: {exc}")
            self.assert_invalid(root, "must not be a symlink")
        finally:
            temporary.cleanup()

    def test_root_npm_manifest_must_remain_private(self) -> None:
        temporary, root = self.materialize()
        try:
            path = root / "package.json"
            data = json.loads(path.read_text())
            data["private"] = False
            path.write_text(json.dumps(data), encoding="utf-8")
            self.assert_invalid(root, "must remain private")
        finally:
            temporary.cleanup()

    def test_private_root_npm_manifest_rejects_publish_license_metadata(self) -> None:
        temporary, root = self.materialize()
        try:
            path = root / "package.json"
            data = json.loads(path.read_text())
            data["license"] = "Apache-2.0"
            path.write_text(json.dumps(data), encoding="utf-8")
            self.assert_invalid(root, "must not carry publish-license metadata")
        finally:
            temporary.cleanup()

    def test_root_cargo_manifest_must_remain_virtual(self) -> None:
        temporary, root = self.materialize()
        try:
            path = root / "Cargo.toml"
            path.write_text(
                path.read_text()
                + '\n[package]\nname = "unexpected-root-package"\nversion = "0.0.0"\n',
                encoding="utf-8",
            )
            self.assert_invalid(root, "must remain a virtual workspace")
        finally:
            temporary.cleanup()

    def test_virtual_cargo_root_rejects_inherited_publish_license_metadata(self) -> None:
        temporary, root = self.materialize()
        try:
            path = root / "Cargo.toml"
            path.write_text(
                path.read_text().replace(
                    'rust-version = "1.98"',
                    'rust-version = "1.98"\nlicense = "Apache-2.0"',
                ),
                encoding="utf-8",
            )
            self.assert_invalid(root, "must not carry inherited")
        finally:
            temporary.cleanup()

    def test_malformed_and_non_utf8_manifests_fail_closed(self) -> None:
        temporary, root = self.materialize()
        try:
            (root / "package.json").write_text('{"private":true,"private":true}', encoding="utf-8")
            self.assert_invalid(root, "duplicate object key")
        finally:
            temporary.cleanup()

        temporary, root = self.materialize()
        try:
            (root / "Cargo.toml").write_bytes(b"[workspace\xff")
            self.assert_invalid(root, "not valid UTF-8")
        finally:
            temporary.cleanup()

    def test_required_repository_boundary_markers_are_enforced(self) -> None:
        cases = (
            (
                pathlib.Path("README.md"),
                license_policy.README_MARKER,
                "readme",
            ),
            (
                pathlib.Path("third_party/README.md"),
                license_policy.THIRD_PARTY_MARKER,
                "third-party",
            ),
            (
                pathlib.Path("third_party/notices/README.md"),
                license_policy.NOTICE_MARKER,
                "notices",
            ),
        )
        for relative, marker, label in cases:
            with self.subTest(path=relative.as_posix()):
                temporary, root = self.materialize()
                try:
                    path = root / relative
                    path.write_text(path.read_text().replace(marker, ""), encoding="utf-8")
                    self.assert_invalid(root, label)
                finally:
                    temporary.cleanup()

    def test_diagnostics_are_sorted_and_deterministic(self) -> None:
        temporary, root = self.materialize()
        try:
            package = root / "package.json"
            data = json.loads(package.read_text())
            data["private"] = False
            data["license"] = "Apache-2.0"
            package.write_text(json.dumps(data), encoding="utf-8")

            cargo = root / "Cargo.toml"
            cargo.write_text(
                cargo.read_text().replace(
                    'rust-version = "1.98"',
                    'rust-version = "1.98"\nlicense = "Apache-2.0"',
                ),
                encoding="utf-8",
            )

            first = license_policy.validate_repository(root)
            second = license_policy.validate_repository(root)
            self.assertEqual(first, second)
            self.assertEqual(first, sorted(set(first)))
        finally:
            temporary.cleanup()


if __name__ == "__main__":
    unittest.main()
