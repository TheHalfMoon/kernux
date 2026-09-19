from __future__ import annotations

import json
import tempfile
import unittest
from pathlib import Path

from tools.dependencies.validate import ValidationError, validate_repository


class DependencyValidationTests(unittest.TestCase):
    def make_repo(
        self,
        *,
        npm: dict[str, str] | None = None,
        approvals: list[dict[str, str]] | None = None,
        cargo_member: str = "",
    ) -> Path:
        temp = tempfile.TemporaryDirectory()
        self.addCleanup(temp.cleanup)
        root = Path(temp.name)
        (root / "third_party/dependencies").mkdir(parents=True)
        (root / "crates/example").mkdir(parents=True)

        package = {
            "name": "fixture",
            "private": True,
            "devDependencies": npm or {},
        }
        (root / "package.json").write_text(json.dumps(package), encoding="utf-8")
        registry = {
            "schema": "kernux.dependency-approvals/v1",
            "approvals": approvals or [],
        }
        (root / "third_party/dependencies/approved.json").write_text(
            json.dumps(registry), encoding="utf-8"
        )
        (root / "Cargo.toml").write_text(
            '[workspace]\nmembers = ["crates/example"]\nresolver = "3"\n',
            encoding="utf-8",
        )
        (root / "crates/example/Cargo.toml").write_text(
            "[package]\nname = \"example\"\nversion = \"0.0.0\"\nedition = \"2024\"\n"
            + cargo_member,
            encoding="utf-8",
        )
        return root

    @staticmethod
    def approval(
        ecosystem: str,
        name: str,
        version: str,
        *,
        scope: str = "dev",
        license_expression: str = "MIT",
        source: str = "registry",
    ) -> dict[str, str]:
        return {
            "ecosystem": ecosystem,
            "name": name,
            "source": source,
            "version": version,
            "scope": scope,
            "purpose": "fixture dependency",
            "license_expression": license_expression,
            "license_posture": "permitted-by-LICENSE_POLICY",
        }

    def assert_invalid(self, root: Path, needle: str, **kwargs: object) -> None:
        with self.assertRaisesRegex(ValidationError, needle):
            validate_repository(root, **kwargs)

    def test_valid_exact_npm_dependency(self) -> None:
        approval = self.approval("npm", "typescript", "7.0.2")
        root = self.make_repo(npm={"typescript": "7.0.2"}, approvals=[approval])
        deps = validate_repository(root)
        self.assertEqual([(dep.name, dep.version) for dep in deps], [("typescript", "7.0.2")])

    def test_unapproved_npm_dependency_fails(self) -> None:
        root = self.make_repo(npm={"typescript": "7.0.2"})
        self.assert_invalid(root, "unapproved direct dependency npm:typescript")

    def test_npm_range_fails(self) -> None:
        approval = self.approval("npm", "typescript", "7.0.2")
        root = self.make_repo(npm={"typescript": "^7.0.2"}, approvals=[approval])
        self.assert_invalid(root, "must use exact semver")

    def test_version_mismatch_fails(self) -> None:
        approval = self.approval("npm", "typescript", "7.0.1")
        root = self.make_repo(npm={"typescript": "7.0.2"}, approvals=[approval])
        self.assert_invalid(root, "version mismatch")

    def test_duplicate_approval_fails(self) -> None:
        approval = self.approval("npm", "typescript", "7.0.2")
        root = self.make_repo(
            npm={"typescript": "7.0.2"},
            approvals=[approval, dict(approval)],
        )
        self.assert_invalid(root, "duplicate approval")

    def test_stale_approval_fails(self) -> None:
        approval = self.approval("npm", "typescript", "7.0.2")
        root = self.make_repo(approvals=[approval])
        self.assert_invalid(root, "stale dependency approvals")

    def test_malformed_approval_fails(self) -> None:
        approval = self.approval("npm", "typescript", "7.0.2")
        del approval["purpose"]
        root = self.make_repo(npm={"typescript": "7.0.2"}, approvals=[approval])
        self.assert_invalid(root, "missing fields")

    def test_unsupported_ecosystem_fails(self) -> None:
        approval = self.approval("pip", "example", "1.0.0")
        root = self.make_repo(approvals=[approval])
        self.assert_invalid(root, "unsupported ecosystem")

    def test_internal_cargo_path_dependency_needs_no_approval(self) -> None:
        root = self.make_repo(
            cargo_member='\n[dependencies]\nlocal = { path = "../local" }\n'
        )
        (root / "crates/local").mkdir()
        (root / "crates/local/Cargo.toml").write_text(
            '[package]\nname = "local"\nversion = "0.0.0"\nedition = "2024"\n',
            encoding="utf-8",
        )
        self.assertEqual(validate_repository(root), [])

    def test_valid_locked_cargo_registry_dependency(self) -> None:
        approval = self.approval(
            "cargo",
            "serde",
            "1.0.229",
            scope="runtime",
            license_expression="MIT OR Apache-2.0",
        )
        root = self.make_repo(
            approvals=[approval],
            cargo_member='\n[dependencies]\nserde = "=1.0.229"\n',
        )
        metadata = {
            "packages": [
                {
                    "name": "serde",
                    "version": "1.0.229",
                    "license": "MIT OR Apache-2.0",
                    "source": "registry+https://github.com/rust-lang/crates.io-index",
                }
            ]
        }
        deps = validate_repository(root, cargo_metadata_override=metadata)
        self.assertEqual(deps[0].name, "serde")

    def test_cargo_range_fails(self) -> None:
        approval = self.approval("cargo", "serde", "1.0.229", scope="runtime")
        root = self.make_repo(
            approvals=[approval],
            cargo_member='\n[dependencies]\nserde = "1.0.229"\n',
        )
        self.assert_invalid(root, "must use exact =x.y.z")

    def test_cargo_locked_resolution_mismatch_fails(self) -> None:
        approval = self.approval(
            "cargo",
            "serde",
            "1.0.229",
            scope="runtime",
            license_expression="MIT OR Apache-2.0",
        )
        root = self.make_repo(
            approvals=[approval],
            cargo_member='\n[dependencies]\nserde = "=1.0.229"\n',
        )
        metadata = {
            "packages": [
                {
                    "name": "serde",
                    "version": "1.0.228",
                    "license": "MIT OR Apache-2.0",
                    "source": "registry+https://github.com/rust-lang/crates.io-index",
                }
            ]
        }
        self.assert_invalid(
            root,
            "not resolved by cargo metadata --locked",
            cargo_metadata_override=metadata,
        )

    def test_cargo_license_mismatch_fails(self) -> None:
        approval = self.approval(
            "cargo",
            "serde",
            "1.0.229",
            scope="runtime",
            license_expression="MIT OR Apache-2.0",
        )
        root = self.make_repo(
            approvals=[approval],
            cargo_member='\n[dependencies]\nserde = "=1.0.229"\n',
        )
        metadata = {
            "packages": [
                {
                    "name": "serde",
                    "version": "1.0.229",
                    "license": "GPL-3.0-only",
                    "source": "registry+https://github.com/rust-lang/crates.io-index",
                }
            ]
        }
        self.assert_invalid(
            root,
            "license mismatch",
            cargo_metadata_override=metadata,
        )

    def test_unpermitted_license_requires_exception(self) -> None:
        approval = self.approval(
            "npm",
            "example",
            "1.0.0",
            license_expression="BSD-3-Clause",
        )
        root = self.make_repo(npm={"example": "1.0.0"}, approvals=[approval])
        self.assert_invalid(root, "requires separately governed policy exception")


if __name__ == "__main__":
    unittest.main()
