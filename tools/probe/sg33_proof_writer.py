from __future__ import annotations

import hashlib
import json
import os
import sys
from pathlib import Path

ROOT = Path.cwd()
sys.path.insert(0, str(ROOT))

from tools.specgrain.run import _load_pin, _source_root

source = _source_root(_load_pin())
sys.path.insert(0, str(source / "src"))

from specgrain import (
    CheckEvidence,
    ExecutionResult,
    WorkPacket,
    append_verification_report,
    load_project,
    verify_execution,
)

CLOSEOUT_SHA = os.environ["CLOSEOUT_SHA"]
CLOSEOUT_PR = os.environ["CLOSEOUT_PR"]
CLOSEOUT_EXACT_CI = os.environ["CLOSEOUT_EXACT_CI"]
CLOSEOUT_POST_CI = os.environ["CLOSEOUT_POST_CI"]
NATIVE_RUN = os.environ["NATIVE_RUN"]
DRY_RUN = os.environ["DRY_RUN"]
PACKET_CI = os.environ["PACKET_CI"]
PACKET_POST_CI = os.environ["PACKET_POST_CI"]
EXPECTED_RESULT_DIGEST = os.environ["EXPECTED_RESULT_DIGEST"]

project = load_project(ROOT)
node = next(item for item in project.specs if item.id == "SG-000033")
packet = WorkPacket.from_dict(
    json.loads(Path("docs/evidence/SG-000033-work-packet.json").read_text(encoding="utf-8-sig"))
)

changed_paths = (
    ".specgrain/specs/SG-000033.json",
    "apps/desktop/src/renderer.tsx",
    "apps/desktop/test/design-system.test.ts",
    "apps/desktop/test/tsconfig.tests.json",
    "docs/evidence/SG-000033-context-sources.json",
    "docs/evidence/SG-000033-work-packet.json",
    "package.json",
    "packages/ui-system/primitives.ts",
    "packages/ui-system/styles.ts",
    "packages/ui-system/tokens.ts",
    "tools/ui-system/authority.mjs",
    "tools/ui-system/check.mjs",
    "tools/ui-system/check.test.mjs",
    "tools/ui-system/foundation.mjs",
    "tools/ui-system/primitive-style.mjs",
)

result = ExecutionResult(
    packet_digest=packet.packet_digest,
    status="succeeded",
    summary="Implemented and natively qualified the bounded SG-000033 design-system foundation.",
    changed_paths=changed_paths,
    reported_evidence=packet.required_evidence,
)
assert result.result_digest == EXPECTED_RESULT_DIGEST, result.result_digest

acceptance_detail = {
    "Canonical closeout advances only KX-P03-S02-T01 scope": (
        "repo:specs/CURRENT.md;repo:specs/tasks.md",
        "Protected closeout PR #240 and this proof transition only KX-P03-S02-T01; later P03 tasks remain PLANNED.",
    ),
    "Existing connection projection and frozen unprivileged shell semantics remain unchanged and tests cover hostile token and primitive inputs": (
        "github-pr:237;github-actions:36239099789",
        "Renderer authority remained additive and the native corpus passed hostile token/primitive inputs on Linux, macOS, and Windows.",
    ),
    "Focus treatment is always visible for keyboard use, bypasses mouse styling, respects reduced motion, and uses scalable typography": (
        "github-pr:237;github-actions:36239099789",
        "The renderer accessibility style contract and native UI-system corpus establish focus-visible, reduced-motion, and scalable typography behavior.",
    ),
    "Native SpecGrain verification binds the WorkPacket to observed evidence with zero issues": (
        "specgrain:SG-000033",
        "This verification itself; all nine acceptance checks and all twenty-two required evidence checks pass with zero issues before append.",
    ),
    "One canonical renderer-safe token source defines the closed foundation and exposes no host capability or secret-bearing data": (
        "github-pr:232;github-actions:36239099789",
        "The canonical token authority is renderer-safe, closed, deterministic, and contains no host capability or secret-bearing data.",
    ),
    "Primitives preserve native semantic HTML and accessible names; interactive status and buttons support keyboard, disabled, loading, and error semantics without redundant roles": (
        "github-pr:233;github-actions:36239099789",
        "Semantic primitive tests establish native elements, accessible names, keyboard use, and deterministic disabled/loading/error/status semantics.",
    ),
    "Raw color, spacing, radius, typography, and motion literals are rejected outside the canonical token source except documented structural data": (
        "github-pr:235;github-pr:236;github-actions:36239099789",
        "Fail-closed conformance rejects raw design literals outside the canonical authority while retaining only documented structural exceptions.",
    ),
    "Semantic color pairs meet frozen machine-checked WCAG AA contrast thresholds and never encode status by color alone": (
        "github-pr:235;github-actions:36239099789",
        "Machine-checked semantic pairs meet frozen WCAG AA thresholds and status primitives retain non-color text semantics.",
    ),
    "The conformance checker fails closed on token bypass, unknown tokens, invalid contrast, invalid primitive semantics, and renderer authority drift": (
        "github-pr:234;github-pr:235;github-pr:236;github-actions:36239099789",
        "The repository-owned checker and hostile corpus fail closed across token, contrast, primitive, and renderer-authority violations.",
    ),
}

evidence_detail = {
    "accessibility-state-pass": ("github-pr:233;github-actions:36239099789", "Semantic disabled, loading, error and status state coverage passed natively."),
    "closeout-post-merge-ci-pass": (f"github-actions:{CLOSEOUT_POST_CI}", "Closeout merge post-merge CI completed 6/6 SUCCESS."),
    "design-conformance-pass": ("github-pr:234;github-pr:235;github-pr:236;github-actions:36239099789", "Design-system fail-closed conformance and hostile corpus passed."),
    "diffcipline-r1-proof-pass": (f"github-actions:{CLOSEOUT_EXACT_CI};github-actions:{CLOSEOUT_POST_CI}", "Diffcipline R1 passed at the exact closeout head and canonical merge."),
    "focus-reduced-motion-pass": ("github-pr:237;github-actions:36239099789", "Focus-visible, reduced-motion and scalable-type checks passed natively."),
    "full-baseline-pass": (f"github-actions:{DRY_RUN}", "Fresh Linux/macOS/Windows qualification passed UI-system, workspace, protocol, SpecGrain, typecheck, format and lint checks."),
    "implementation-post-merge-ci-pass": ("repo:docs/evidence/P03-S02-T01.md", "All bounded implementation slice post-merge CI runs are recorded SUCCESS in the canonical evidence document."),
    "implementation-pr-pass": ("github-pr:232;github-pr:233;github-pr:234;github-pr:235;github-pr:236;github-pr:237", "All bounded implementation slices merged normally after exact-head qualification."),
    "jev-review-pass": ("jev:0.3.2/jev-1.13.0", "Genuine TypeSafe Jev review on exact implementation slice diffs returned no blocking defect finding."),
    "linux-desktop-qualification-pass": (f"github-actions:{NATIVE_RUN}", "Linux native desktop qualification passed the complete SG-000033 suite."),
    "macos-desktop-qualification-pass": (f"github-actions:{NATIVE_RUN}", "macOS-14 ARM64 native desktop qualification passed the complete SG-000033 suite."),
    "native-specgrain-verification-pass": ("specgrain:SG-000033", "This final native verification is zero-issue and is appended immutably by this proof candidate."),
    "open-code-review-accounting-pass": ("alibaba-open-code-review:1.12.9;repo:docs/evidence/P03-S02-T01.md", "Alibaba OpenCodeReview 1.12.9 Delegation Mode accounting is preserved for the implementation slices; no semantic model-review claim is fabricated."),
    "packet-persistence-post-merge-ci-pass": (f"github-actions:{PACKET_POST_CI}", "Packet-persistence merge post-merge CI completed SUCCESS."),
    "packet-persistence-pr-pass": ("github-pr:231", "ContextSources and the pinned WorkPacket were persisted before implementation."),
    "protected-closeout-pr-pass": (f"github-pr:{CLOSEOUT_PR};github-actions:{CLOSEOUT_EXACT_CI}", "Protected closeout PR #240 passed exact-head 6/6 CI, semantic review with zero blocking threads, and merged normally."),
    "renderer-authority-preservation-pass": ("github-pr:235;github-pr:237;github-actions:36239099789", "Renderer remains unprivileged and the design system creates no second authority."),
    "semantic-primitive-pass": ("github-pr:233;github-actions:36239099789", "Semantic primitive and accessibility corpus passed natively."),
    "token-authority-pass": ("github-pr:232;github-actions:36239099789", "Single canonical token authority and hostile token lookup checks passed."),
    "token-contrast-pass": ("github-pr:235;github-actions:36239099789", "Frozen semantic WCAG AA contrast checks passed on all native legs."),
    "windows-desktop-qualification-pass": (f"github-actions:{NATIVE_RUN}", "Windows Server 2025 native desktop qualification passed the complete SG-000033 suite."),
    "workpacket-durable-recovery-pass": (f"github-pr:231;github-actions:{PACKET_CI}", "Pinned WorkPacket replay was byte-identical and the durable packet digest remained frozen."),
}

assert set(acceptance_detail) == set(packet.acceptance)
assert set(evidence_detail) == set(packet.required_evidence)

acceptance = tuple(
    CheckEvidence(check_id, True, acceptance_detail[check_id][0], acceptance_detail[check_id][1])
    for check_id in packet.acceptance
)
evidence = tuple(
    CheckEvidence(check_id, True, evidence_detail[check_id][0], evidence_detail[check_id][1])
    for check_id in packet.required_evidence
)

report = verify_execution(
    current_node=node,
    packet=packet,
    result=result,
    implementation_revision=f"git:{CLOSEOUT_SHA}",
    observed_changed_paths=changed_paths,
    acceptance_checks=acceptance,
    evidence_checks=evidence,
)
assert report.verified, report.to_dict()
assert not report.issues, report.to_dict()
assert len(report.acceptance_checks) == 9
assert len(report.evidence_checks) == 22

record = append_verification_report(ROOT, report)
record_path = Path(".specgrain/evidence/SG-000033") / f"{record.record_digest[7:]}.json"
raw = record_path.read_bytes()
file_sha = hashlib.sha256(raw).hexdigest()
print("SG33_VERIFIED", report.verified)
print("SG33_RECORD", record.record_digest)
print("SG33_RECORD_PATH", record_path.as_posix())
print("SG33_RECORD_BYTES", len(raw))
print("SG33_RECORD_FILE_SHA256", file_sha)
print("SG33_RESULT_DIGEST", result.result_digest)

tasks_path = Path("specs/tasks.md")
tasks = tasks_path.read_text(encoding="utf-8")
old_row = "| KX-P03-S02-T01 | PLANNED | R1 | Establish Kernux design tokens, primitives, accessibility and design-system lint rules. | desktop shell |"
new_row = "| KX-P03-S02-T01 | PROVEN | R1 | Establish Kernux design tokens, primitives, accessibility and design-system lint rules. | desktop shell |"
assert tasks.count(old_row) == 1
tasks = tasks.replace(old_row, new_row, 1)
closeout_note = (
    "\n**KX-P03-S02-T01 closeout: PROVEN.** SG-000033 native verification record `"
    + record.record_digest
    + "` binds the frozen WorkPacket to the 15-path implementation/spec/packet range with acceptance 9/9, required evidence 22/22 and zero issues. "
    + "The bounded implementation chain is PRs #232-#237; implementation evidence PR #238 and native-evidence PR #239 are canonical. Native desktop qualification `36239099789` passed Linux, macOS-14 and Windows; fresh dry verification `36243087264` failed closed on exactly five closeout-dependent checks before any record append. "
    + "Genuine Jev 0.3.2 / jev-1.13.0 review returned no blocking implementation finding, and Alibaba OpenCodeReview 1.12.9 Delegation Mode accounting is preserved without a semantic-review claim. Protected closeout PR #240 exact head passed CI `36243245499` 6/6, zero blocking review threads, ordinary merge `"
    + CLOSEOUT_SHA
    + "`, and post-merge CI `"
    + CLOSEOUT_POST_CI
    + "` 6/6. KX-P03-S02-T01 is PROVEN; only KX-P03-S03-T01 is NEXT and later P03 tasks remain PLANNED.\n"
)
tasks = tasks.replace(new_row, new_row + closeout_note, 1)
tasks_path.write_text(tasks, encoding="utf-8", newline="\n")

current_path = Path("specs/CURRENT.md")
current = current_path.read_text(encoding="utf-8")
replacements = {
    "**NEXT CANONICAL FRONTIER:** `KX-P03-S02-T01 - Establish Kernux design tokens, primitives, accessibility and design-system lint rules.`": "**NEXT CANONICAL FRONTIER:** `KX-P03-S03-T01 - Implement tabs/splits/docking/layout persistence and keyboard navigation.`",
    "**P03 ENTRY AUTHORIZED:** `KX-P03-S02-T01 ONLY`": "**P03 ENTRY AUTHORIZED:** `KX-P03-S03-T01 ONLY`",
    "**LATEST PROVEN P03 MACRO TASK:** `KX-P03-S01-T02 - Implement daemon connection/reconnect projection and honest degraded states.`": "**LATEST PROVEN P03 MACRO TASK:** `KX-P03-S02-T01 - Establish Kernux design tokens, primitives, accessibility and design-system lint rules.`",
    "Only KX-P03-S02-T01 is authorized as NEXT; later P03 tasks remain PLANNED.": "That closeout authorized KX-P03-S02-T01 as NEXT; later P03 tasks remained PLANNED.",
}
for old, new in replacements.items():
    assert current.count(old) == 1, (old, current.count(old))
    current = current.replace(old, new, 1)
qual_anchor = "**KX-P03-S01-T02 QUALIFICATION:** `PROVEN`"
assert current.count(qual_anchor) == 1
current = current.replace(qual_anchor, qual_anchor + "\n\n**KX-P03-S02-T01 QUALIFICATION:** `PROVEN`", 1)
proof_anchor = "\n\nProof for the current frontier includes:"
assert current.count(proof_anchor) == 1
current_note = (
    "\n\nKX-P03-S02-T01 is now PROVEN through the SG-000033 packet-first path: immutable native record `"
    + record.record_digest
    + "`, native Linux/macOS/Windows qualification `36239099789`, protected closeout PR #240 with exact-head CI `36243245499`, ordinary merge `"
    + CLOSEOUT_SHA
    + "`, and post-merge CI `"
    + CLOSEOUT_POST_CI
    + "` all passed. Genuine Jev review and Alibaba OpenCodeReview accounting remain bound to the implementation chain. Only KX-P03-S03-T01 is authorized as NEXT; later P03 tasks remain PLANNED."
)
current = current.replace(proof_anchor, current_note + proof_anchor, 1)
current_path.write_text(current, encoding="utf-8", newline="\n")

evidence_path = Path("docs/evidence/P03-S02-T01.md")
doc = evidence_path.read_text(encoding="utf-8")
doc = doc.replace("# Evidence — KX-P03-S02-T01 (implementation, UNPROVEN)", "# Evidence — KX-P03-S02-T01 (PROVEN)", 1)
doc = doc.replace("Grain: `SG-000033`, state `GRAIN`, risk `R1`. Status: `IMPLEMENTED_UNPROVEN`.", "Grain: `SG-000033`, state `GRAIN`, risk `R1`. Status: `PROVEN`.", 1)
doc = doc.replace("No task-state, phase-exit, release, or `PROJECT_COMPLETE` advancement is claimed by this record.", "Canonical task-state advancement is limited to `KX-P03-S02-T01`; no phase-exit, release, or `PROJECT_COMPLETE` claim is made.", 1)
doc = doc.replace("## Unproven gates", "## Pre-proof gates (historical negative evidence)", 1)
doc = doc.replace("The following required SG-000033 evidence remains honestly unproven and prevents PROVEN status:", "At the implementation-evidence point, the following required SG-000033 evidence remained honestly unproven and prevented PROVEN status:", 1)
doc = doc.replace("No ledger transition, native evidence record, protected closeout, or `PROJECT_COMPLETE` claim is made here. The next lawful action is protected closeout preparation and SpecGrain dry verification, not later P03 feature implementation.", "At that point no ledger transition, native evidence record, protected closeout, or `PROJECT_COMPLETE` claim was made; the lawful next action was protected closeout preparation and SpecGrain dry verification.", 1)
doc = doc.replace("Status remains `IMPLEMENTED_UNPROVEN` until that proof chain completes.", "That pre-proof status remained `IMPLEMENTED_UNPROVEN` until the proof chain below completed.", 1)
final_section = f"""

## Final native proof and canonical advancement

Protected closeout PR #240 exact head `58206d5e5ec1cb97ae34ddb8449bb4ebd6f87a96` passed CI `36243245499` 6/6 and semantic review `5325959839` with zero blocking review threads, then merged normally at `{CLOSEOUT_SHA}` (tree `1cc3bc5aa415ae28bbfc42be271a86c81c08d62c`). Post-merge CI `{CLOSEOUT_POST_CI}` passed 6/6.

Only after those gates existed, the pinned SpecGrain verifier reran the same execution result used by dry run `36243087264`. Final verification returned `verified=true`, acceptance `9/9`, required evidence `22/22`, and zero issues. The deterministic result digest remained `{result.result_digest}`. The immutable native record is `{record.record_digest}` at `{record_path.as_posix()}` ({len(raw)} bytes; file SHA-256 `{file_sha}`).

This proof candidate advances only `KX-P03-S02-T01` to `PROVEN`, records its qualification in `specs/CURRENT.md`, and advances the sole canonical frontier to `KX-P03-S03-T01`. Later P03 tasks remain `PLANNED`; no P03 phase-exit, release-ready, or `PROJECT_COMPLETE` claim is made.
"""
doc += final_section
evidence_path.write_text(doc, encoding="utf-8", newline="\n")

meta = {
    "record_digest": record.record_digest,
    "record_path": record_path.as_posix(),
    "record_bytes": len(raw),
    "record_file_sha256": file_sha,
    "result_digest": result.result_digest,
}
Path("/tmp/sg33-proof-meta.json").write_text(json.dumps(meta, sort_keys=True, indent=2), encoding="utf-8")
