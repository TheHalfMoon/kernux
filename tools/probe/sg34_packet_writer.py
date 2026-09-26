#!/usr/bin/env python3
"""Generate SG-000034 ContextSources and WorkPacket through the pinned SpecGrain CLI."""

from __future__ import annotations

import hashlib
import json
import os
import subprocess
from dataclasses import dataclass
from pathlib import Path

EXPECTED_BASE = os.environ.get("EXPECTED_BASE", "7e07035d8a29b234273fa84a496539cc685ad03e")
ROOT = Path.cwd()
CONTEXT_PATH = ROOT / "docs/evidence/SG-000034-context-sources.json"
PACKET_PATH = ROOT / "docs/evidence/SG-000034-work-packet.json"


@dataclass(frozen=True)
class Slice:
    path: str
    start: int | None = None
    end: int | None = None


@dataclass(frozen=True)
class Source:
    source_id: str
    priority: int
    provenance: str
    selection_reason: str
    slices: tuple[Slice, ...]


def run(*args: str) -> str:
    return subprocess.check_output(args, cwd=ROOT, text=True).strip()


def selected_bytes(item: Slice) -> bytes:
    path = ROOT / item.path
    raw = path.read_bytes()
    text = raw.decode("utf-8")
    if item.start is None and item.end is None:
        return raw
    lines = text.splitlines(keepends=True)
    start = 1 if item.start is None else item.start
    end = len(lines) if item.end is None else item.end
    if start < 1 or end < start or end > len(lines):
        raise RuntimeError(
            f"invalid line slice {item.path}#{start}-{end}; file has {len(lines)} lines"
        )
    return "".join(lines[start - 1 : end]).encode("utf-8")


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def main() -> None:
    head = run("git", "rev-parse", "HEAD")
    if head != EXPECTED_BASE:
        raise RuntimeError(f"target checkout must be exact base {EXPECTED_BASE}; observed {head}")
    if run("git", "status", "--porcelain", "--untracked-files=all"):
        raise RuntimeError("target checkout must start clean")

    sources = (
        Source(
            "live-governance",
            0,
            "repo:AGENTS.md",
            "Bind precedence, bounded delivery, exact-head qualification, mandatory Jev/OCR review, negative evidence, and protected-history rules.",
            (Slice("AGENTS.md"),),
        ),
        Source(
            "canonical-frontier",
            1,
            "repo:specs/CURRENT.md#L70-L135",
            "Bind the live P03 frontier: S02-T01 is PROVEN and only KX-P03-S03-T01 is authorized next.",
            (Slice("specs/CURRENT.md", 70, 135),),
        ),
        Source(
            "task-registry",
            2,
            "repo:specs/tasks.md#L105-L130",
            "Bind KX-P03-S03-T01 outcome, R1 floor, proven design-system dependency, later-task exclusions, and P03 gate context.",
            (Slice("specs/tasks.md", 105, 130),),
        ),
        Source(
            "sg34-grain",
            3,
            "repo:.specgrain/specs/SG-000034.json",
            "Bind the complete frozen SG-000034 outcome, acceptance, limits, evidence, recovery, safety, and authorized change surface.",
            (Slice(".specgrain/specs/SG-000034.json"),),
        ),
        Source(
            "execution-plan",
            4,
            "repo:docs/canonical/EXECUTION_MASTER_PLAN.md#L149-L190",
            "Bind S03.3 adaptive-pane scope and keep S03.4+ feature work outside this Grain.",
            (Slice("docs/canonical/EXECUTION_MASTER_PLAN.md", 149, 190),),
        ),
        Source(
            "ux-blueprint",
            5,
            "repo:docs/canonical/UX_BLUEPRINT.md#L1-L35",
            "Bind the task-driven adaptive workspace, dock/split/tab model, minimum-surface opening, and remembered user layout choices.",
            (Slice("docs/canonical/UX_BLUEPRINT.md", 1, 35),),
        ),
        Source(
            "architecture-authority",
            6,
            "repo:docs/canonical/ARCHITECTURE.md#L76-L146",
            "Bind renderer-never-authority and packages/ui-system boundaries while preserving desktop/daemon separation.",
            (Slice("docs/canonical/ARCHITECTURE.md", 76, 146),),
        ),
        Source(
            "renderer-baseline",
            7,
            "repo:apps/desktop/src/renderer.tsx",
            "Bind the existing renderer integration surface that the workspace projection may extend without gaining host authority.",
            (Slice("apps/desktop/src/renderer.tsx"),),
        ),
        Source(
            "ui-primitives",
            8,
            "repo:packages/ui-system/primitives.ts",
            "Bind the proven SG-000033 primitive contract so adaptive-pane semantics reuse instead of fork the design system.",
            (Slice("packages/ui-system/primitives.ts"),),
        ),
        Source(
            "ui-styles",
            9,
            "repo:packages/ui-system/styles.ts",
            "Bind the proven tokenized styling authority and prevent ad-hoc workspace styling drift.",
            (Slice("packages/ui-system/styles.ts"),),
        ),
        Source(
            "sg33-proof-precedent",
            10,
            "repo:docs/evidence/P03-S02-T01.md#L1-L120",
            "Bind the immediately preceding native qualification, Jev/OCR accounting, exact-head, and proof-bearing closeout precedent.",
            (Slice("docs/evidence/P03-S02-T01.md", 1, 120),),
        ),
    )

    records: list[dict[str, object]] = []
    for source in sources:
        material = b"".join(selected_bytes(part) for part in source.slices)
        size = len(material)
        records.append(
            {
                "source_id": source.source_id,
                "priority": source.priority,
                "requirement": "required",
                "provenance": source.provenance,
                "revision": f"git:{EXPECTED_BASE}",
                "size_bytes": size,
                "token_cost": (size + 3) // 4,
                "selection_reason": source.selection_reason,
            }
        )

    CONTEXT_PATH.parent.mkdir(parents=True, exist_ok=True)
    context_bytes = (json.dumps(records, indent=2, ensure_ascii=False) + "\n").encode("utf-8")
    CONTEXT_PATH.write_bytes(context_bytes)

    command = (
        "python3",
        "tools/specgrain/run.py",
        "packet",
        "SG-000034",
        "--context-sources",
        str(CONTEXT_PATH.relative_to(ROOT)),
        "--json",
    )
    first = subprocess.check_output(command, cwd=ROOT)
    second = subprocess.check_output(command, cwd=ROOT)
    if first != second:
        raise RuntimeError("pinned SpecGrain packet replay was not byte-identical")

    packet = json.loads(first.decode("utf-8"))
    if packet.get("spec_id") != "SG-000034":
        raise RuntimeError("exported packet is not SG-000034")
    if packet.get("context_sources") is None or len(packet["context_sources"]) != len(records):
        raise RuntimeError("exported packet did not preserve the selected context set")

    packet_bytes = first if first.endswith(b"\n") else first + b"\n"
    PACKET_PATH.write_bytes(packet_bytes)
    if PACKET_PATH.read_bytes() != packet_bytes:
        raise RuntimeError("persisted packet bytes differ from pinned CLI output")

    metadata = {
        "base_sha": EXPECTED_BASE,
        "context_file_sha256": sha256_bytes(context_bytes),
        "context_sources": len(records),
        "context_tokens": sum(int(record["token_cost"]) for record in records),
        "packet_file_sha256": sha256_bytes(packet_bytes),
        "packet_digest": packet["packet_digest"],
        "spec_revision": packet["spec_revision"],
        "context_plan_digest": packet["context_plan_digest"],
        "replay_byte_identical": True,
    }
    Path("/tmp/sg34-packet-meta.json").write_text(
        json.dumps(metadata, sort_keys=True, indent=2) + "\n", encoding="utf-8"
    )
    print(json.dumps(metadata, sort_keys=True))


if __name__ == "__main__":
    main()
