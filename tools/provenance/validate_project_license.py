#!/usr/bin/env python3
"""Validate the Kernux project-license and third-party boundary contract."""

from __future__ import annotations

import argparse
import hashlib
import sys
import tomllib
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[2]
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))

from tools.provenance import validate as provenance  # noqa: E402

EXPECTED_SPDX = "Apache-2.0"
EXPECTED_LICENSE_SHA256 = "cfc7749b96f63bd31c3c42b5c471bf756814053e847c10f3eb003417bc523d30"
EXPECTED_POLICY_SHA256 = "52036ff4149918166c6feb843798e2cdd023ddd6a7d6c0d3ef98d342709ceaf3"
MAX_TEXT_BYTES = 256 * 1024

LICENSE_PATH = Path("LICENSE")
POLICY_PATH = Path("docs/canonical/LICENSE_POLICY.md")
PACKAGE_PATH = Path("package.json")
CARGO_PATH = Path("Cargo.toml")
README_PATH = Path("README.md")
THIRD_PARTY_README_PATH = Path("third_party/README.md")
NOTICE_README_PATH = Path("third_party/notices/README.md")
README_MARKER = "Kernux-owned code is licensed under the Apache License 2.0."
THIRD_PARTY_MARKER = "The root Apache-2.0 license applies to Kernux-owned code and does not relicense third-party material under this directory."
NOTICE_MARKER = "The root Apache-2.0 license applies to Kernux-owned code and does not relicense the third-party evidence or material represented here."


def _read_regular_bytes(
    root: Path,
    relative: Path,
    *,
    label: str,
) -> tuple[bytes | None, list[str]]:
    path = root / relative
    if path.is_symlink():
        return None, [f"{label}: path must not be a symlink: {relative.as_posix()}"]
    path_errors = provenance._repository_path_errors(
        root,
        relative.as_posix(),
        label,
        require_file=True,
    )
    if path_errors:
        return None, path_errors
    try:
        size = path.stat().st_size
        if size > MAX_TEXT_BYTES:
            return None, [
                f"{label}: file exceeds {MAX_TEXT_BYTES} bytes: {relative.as_posix()}"
            ]
        return path.read_bytes(), []
    except OSError as exc:
        return None, [f"{label}: cannot read {relative.as_posix()}: {exc}"]


def _utf8_text(
    root: Path,
    relative: Path,
    *,
    label: str,
) -> tuple[str | None, list[str]]:
    raw, errors = _read_regular_bytes(root, relative, label=label)
    if raw is None:
        return None, errors
    try:
        return raw.decode("utf-8"), errors
    except UnicodeDecodeError:
        errors.append(f"{label}: file is not valid UTF-8: {relative.as_posix()}")
        return None, errors


def _digest_errors(
    root: Path,
    relative: Path,
    *,
    label: str,
    expected: str,
) -> list[str]:
    raw, errors = _read_regular_bytes(root, relative, label=label)
    if raw is None:
        return errors
    observed = hashlib.sha256(raw).hexdigest()
    if observed != expected:
        errors.append(
            f"{label}: SHA-256 mismatch for {relative.as_posix()}: "
            f"expected {expected}, observed {observed}"
        )
    return errors


def _package_errors(root: Path) -> list[str]:
    _, guard_errors = _read_regular_bytes(root, PACKAGE_PATH, label="package")
    if guard_errors:
        return guard_errors
    path = root / PACKAGE_PATH
    try:
        data = provenance.read_json(path)
    except (OSError, provenance.ProvenanceError) as exc:
        return [f"package: cannot load package.json safely: {exc}"]
    if not isinstance(data, dict):
        return ["package: package.json top-level value must be an object"]
    errors: list[str] = []
    if data.get("private") is not True:
        errors.append(
            "package: root package.json must remain private under license policy v1"
        )
    if "license" in data:
        errors.append(
            "package: private root package.json must not carry publish-license metadata "
            "under license policy v1"
        )
    return errors


def _cargo_errors(root: Path) -> list[str]:
    text, errors = _utf8_text(root, CARGO_PATH, label="cargo")
    if text is None:
        return errors
    try:
        data: Any = tomllib.loads(text)
    except tomllib.TOMLDecodeError as exc:
        errors.append(f"cargo: Cargo.toml is malformed TOML: {exc}")
        return errors
    if not isinstance(data, dict):
        errors.append("cargo: Cargo.toml top-level value must be a table")
        return errors
    if "package" in data:
        errors.append(
            "cargo: root Cargo.toml must remain a virtual workspace under license policy v1"
        )
    workspace = data.get("workspace")
    if not isinstance(workspace, dict):
        errors.append("cargo: root Cargo.toml must contain a workspace table")
        return errors
    workspace_package = workspace.get("package")
    if isinstance(workspace_package, dict) and "license" in workspace_package:
        errors.append(
            "cargo: virtual root workspace.package must not carry inherited "
            "publish-license metadata under license policy v1"
        )
    return errors


def _marker_errors(
    root: Path,
    relative: Path,
    marker: str,
    *,
    label: str,
) -> list[str]:
    text, errors = _utf8_text(root, relative, label=label)
    if text is None:
        return errors
    if marker not in text:
        errors.append(
            f"{label}: required project/third-party boundary marker is missing "
            f"from {relative.as_posix()}"
        )
    return errors


def validate_repository(root: Path = ROOT) -> list[str]:
    errors: list[str] = []
    errors.extend(
        _digest_errors(
            root,
            LICENSE_PATH,
            label="license",
            expected=EXPECTED_LICENSE_SHA256,
        )
    )
    errors.extend(
        _digest_errors(
            root,
            POLICY_PATH,
            label="policy",
            expected=EXPECTED_POLICY_SHA256,
        )
    )
    errors.extend(_package_errors(root))
    errors.extend(_cargo_errors(root))
    errors.extend(
        _marker_errors(
            root,
            README_PATH,
            README_MARKER,
            label="readme",
        )
    )
    errors.extend(
        _marker_errors(
            root,
            THIRD_PARTY_README_PATH,
            THIRD_PARTY_MARKER,
            label="third-party",
        )
    )
    errors.extend(
        _marker_errors(
            root,
            NOTICE_README_PATH,
            NOTICE_MARKER,
            label="notices",
        )
    )
    return sorted(set(errors))


def _check() -> int:
    errors = validate_repository()
    if errors:
        print("Project-license validation: FAIL", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1
    print("Project-license validation: PASS")
    print(f"SPDX: {EXPECTED_SPDX}")
    print(f"LICENSE sha256:{EXPECTED_LICENSE_SHA256}")
    print(f"Policy sha256:{EXPECTED_POLICY_SHA256}")
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        description="Validate Kernux project-license and third-party boundaries."
    )
    subparsers = parser.add_subparsers(dest="command", required=True)
    subparsers.add_parser("check", help="validate canonical project-license state")
    args = parser.parse_args(argv)
    if args.command == "check":
        return _check()
    parser.error(f"unsupported command: {args.command}")
    return 2


if __name__ == "__main__":
    raise SystemExit(main())
