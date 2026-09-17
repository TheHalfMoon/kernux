#!/usr/bin/env python3
"""Run the exact SpecGrain source revision pinned by Kernux."""

from __future__ import annotations

import hashlib
import json
import os
import shutil
import stat
import subprocess
import sys
import tempfile
import urllib.request
import zipfile
from pathlib import Path, PurePosixPath

MAX_ARCHIVE_BYTES = 16 * 1024 * 1024
MAX_EXTRACTED_BYTES = 64 * 1024 * 1024
CHUNK_SIZE = 1024 * 1024


def _repo_root() -> Path:
    return Path(__file__).resolve().parents[2]


def _load_pin() -> dict[str, object]:
    path = Path(__file__).with_name("pin.json")
    data = json.loads(path.read_text(encoding="utf-8"))
    required = {
        "schema_version",
        "repository",
        "commit",
        "archive_url",
        "archive_sha256",
        "license",
        "minimum_python",
    }
    if set(data) != required or data["schema_version"] != 1:
        raise RuntimeError("invalid SpecGrain pin contract")
    commit = data["commit"]
    digest = data["archive_sha256"]
    if not isinstance(commit, str) or len(commit) != 40:
        raise RuntimeError("SpecGrain commit pin must be a full SHA-1")
    if not isinstance(digest, str) or len(digest) != 64:
        raise RuntimeError("SpecGrain archive pin must be a SHA-256 digest")
    return data


def _require_python() -> None:
    if sys.version_info < (3, 11):
        raise RuntimeError(
            f"SpecGrain requires Python >=3.11; found {sys.version.split()[0]}"
        )


def _sha256(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        while chunk := handle.read(CHUNK_SIZE):
            digest.update(chunk)
    return digest.hexdigest()


def _download(url: str, destination: Path) -> None:
    destination.parent.mkdir(parents=True, exist_ok=True)
    descriptor, temporary_name = tempfile.mkstemp(
        prefix="specgrain-", suffix=".zip", dir=destination.parent
    )
    os.close(descriptor)
    temporary = Path(temporary_name)
    total = 0
    try:
        request = urllib.request.Request(
            url, headers={"User-Agent": "Kernux-SpecGrain-Bootstrap/1"}
        )
        with urllib.request.urlopen(request, timeout=30) as response, temporary.open(
            "wb"
        ) as output:
            while chunk := response.read(CHUNK_SIZE):
                total += len(chunk)
                if total > MAX_ARCHIVE_BYTES:
                    raise RuntimeError("SpecGrain archive exceeds download bound")
                output.write(chunk)
        os.replace(temporary, destination)
    finally:
        temporary.unlink(missing_ok=True)


def _verified_archive(pin: dict[str, object], cache: Path) -> Path:
    archive = cache / "source.zip"
    expected = str(pin["archive_sha256"])
    if not archive.exists():
        _download(str(pin["archive_url"]), archive)
    actual = _sha256(archive)
    if actual != expected:
        archive.unlink(missing_ok=True)
        raise RuntimeError(
            f"SpecGrain archive digest mismatch: expected {expected}, observed {actual}"
        )
    return archive


def _safe_member_parts(name: str, expected_root: str) -> tuple[str, ...]:
    path = PurePosixPath(name)
    if path.is_absolute() or ".." in path.parts or not path.parts:
        raise RuntimeError(f"unsafe SpecGrain archive path: {name!r}")
    if path.parts[0] != expected_root:
        raise RuntimeError(f"unexpected SpecGrain archive root: {path.parts[0]!r}")
    return tuple(path.parts[1:])


def _extract_verified(archive: Path, source: Path, commit: str) -> None:
    if source.is_dir() and (source / "src/specgrain/__main__.py").is_file():
        return
    if source.exists():
        shutil.rmtree(source)
    source.parent.mkdir(parents=True, exist_ok=True)
    staging = Path(tempfile.mkdtemp(prefix="source-", dir=source.parent))
    expected_root = f"SpecGrain-{commit}"
    extracted = 0
    try:
        with zipfile.ZipFile(archive) as bundle:
            for member in bundle.infolist():
                parts = _safe_member_parts(member.filename, expected_root)
                if not parts:
                    continue
                file_type = (member.external_attr >> 16) & 0o170000
                if file_type == stat.S_IFLNK:
                    raise RuntimeError(
                        f"SpecGrain archive contains symlink: {member.filename!r}"
                    )
                extracted += member.file_size
                if extracted > MAX_EXTRACTED_BYTES:
                    raise RuntimeError("SpecGrain archive exceeds extraction bound")
                destination = staging.joinpath(*parts)
                if member.is_dir():
                    destination.mkdir(parents=True, exist_ok=True)
                    continue
                destination.parent.mkdir(parents=True, exist_ok=True)
                with bundle.open(member) as src, destination.open("wb") as dst:
                    shutil.copyfileobj(src, dst, CHUNK_SIZE)
        required = staging / "src/specgrain/__main__.py"
        if not required.is_file():
            raise RuntimeError("SpecGrain archive is missing src/specgrain/__main__.py")
        os.replace(staging, source)
    finally:
        if staging.exists():
            shutil.rmtree(staging, ignore_errors=True)


def _source_root(pin: dict[str, object]) -> Path:
    root = _repo_root()
    commit = str(pin["commit"])
    cache = root / ".cache/specgrain" / commit
    archive = _verified_archive(pin, cache)
    source = cache / "source"
    _extract_verified(archive, source, commit)
    return source


def main(argv: list[str] | None = None) -> int:
    _require_python()
    pin = _load_pin()
    args = list(sys.argv[1:] if argv is None else argv)
    if args == ["--show-pin"]:
        print(json.dumps(pin, sort_keys=True))
        return 0

    source = _source_root(pin)
    env = os.environ.copy()
    python_path = str(source / "src")
    if env.get("PYTHONPATH"):
        python_path += os.pathsep + env["PYTHONPATH"]
    env["PYTHONPATH"] = python_path
    command = [sys.executable, "-m", "specgrain", *args]
    return subprocess.call(command, cwd=_repo_root(), env=env)


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, RuntimeError, ValueError, zipfile.BadZipFile) as exc:
        print(f"SpecGrain bootstrap error: {exc}", file=sys.stderr)
        raise SystemExit(2)
