# CURRENT

## Canonical project state

**PROJECT:** Kernux

**CATEGORY:** Agent Operating Environment

**STATUS:** P00_PROVEN_P01_READY

**CANONICAL_PLAN_MERGED_TO_MAIN:** TRUE

**PRODUCT_IMPLEMENTATION_STARTED:** FALSE

**RELEASE_READY:** FALSE

## Current repository truth

The canonical pre-implementation plan was merged to `main` through PR #1 at merge commit:

`ad3277ff7f3be00577d646098b401f409865d995`

The first repository-foundation task, `KX-P00-S01-T01`, was merged through PR #3 at merge commit:

`a921ad864077b6465de62427524136e11a576245`

The baseline-CI task, `KX-P00-S01-T02`, was merged through PR #5 at merge commit:

`646fda9f784ce2368f534a56d8a454ae27b87a35`

The SpecGrain-adoption task, `KX-P00-S02-T01`, was merged through PR #7 at merge commit:

`752b5f9b977a31c54743d92ff6f9063428eb3976`

The Diffcipline-adoption task, `KX-P00-S03-T01`, was merged through PR #9 at merge commit:

`da096259e1bb7d669206f2d097e56731004d7c41`

The donor-provenance contract task, `KX-P00-S04-T01`, was completed through two bounded implementation slices:

- PR #13 core contract merge: `6851f8f59d236a33e3f94c4e77e17756ba16dba2`;
- PR #14 conformance/enforcement merge: `c68b22c74aa402511c1a8e6ee2d0a298cc28d189`.

The third-party notice/checklist task, `KX-P00-S04-T02`, was refined through PR #16 and completed through two bounded implementation slices:

- PR #16 SpecGrain refinement merge: `dfa4cedde60a77a924d4d60e21c2843d8afb46f1`;
- PR #17 notice inventory core merge: `e7dcfda6a425a761cc709e1f24b5254de50f702c`;
- PR #18 conformance/enforcement + donor import checklist merge: `e14debcf36ddf64f75b60dc3bbc4bb9c291443fb`.

The project-license/compatibility task, `KX-P00-S01-T04`, was refined through PR #21 and completed through two bounded implementation slices:

- PR #21 SpecGrain refinement merge: `4709ad3c68d82dc54ae3de5ab9c338dd3aa013a3`;
- PR #22 Apache-2.0 project-license core merge: `e7c864e82507a7c7e168226190ee4ec26278a9c9`;
- PR #23 conformance/enforcement merge: `d19122c9217e2ad1d9c133de3e64f34ff984f7d5`.

The contributor/repository-governance task, `KX-P00-S01-T03`, was refined and completed with an explicit scope-correction chain:

- PR #25 primary SpecGrain refinement merge: `c6f67895e62a1f5a5d5dbca43a3dd01cc6504279`;
- PR #26 governance core merge: `e159848ee6dbb820d98d0590d2c200e6d31d1a49`;
- PR #27 workspace-enforcement merge: `7c87797421d698e1e08de34d623a0ff4158fbc8c`;
- PR #28 explicit-path corrective refinement merge: `e4f1ed03b18543efb71b26bb18627aa4e67fb7d3`;
- PR #29 explicit-path corrective implementation merge: `f483762e1afdff04686801c6ac097d555e749259`.

SG-000005 remains natively unverified because its `.github/ISSUE_TEMPLATE/**` change-surface entry is not glob-expanded by the pinned verifier. That negative evidence is preserved. SG-000006 explicitly re-adopted the four affected paths and is natively VERIFIED with evidence record `sha256:5096e6059a53f6c1db6c2d00317b61137f03fc2c8ca9945795838b9d8f3fc6bb`.

T02 established pinned, reproducible Rust and JavaScript/TypeScript checks. T02.1 established repository-local SpecGrain state and the first bounded successor Grain. T03 established a repository-owned Diffcipline v1.0.0 policy, explicit R0-R3 verification profiles, and a pinned CI proof gate. T04.1 established the strict donor import manifest v1 schema, fail-closed validator, 21-test conformance corpus, and dedicated Provenance CI job. T04.2 added the versioned notice inventory, byte-preserved founding-donor public license snapshots, fail-closed notice/provenance cross-validation, an 18-test notice corpus, a canonical donor import checklist, and expanded Provenance CI. KX-P00-S01-T04 established the Apache-2.0 project-license core, explicit third-party license boundaries, conservative current compatibility policy, fail-closed project-license validation, and 12 additional conformance tests for 51 combined provenance/notice/project-license tests.

No product code or founding-donor product code has been imported yet.

## Active frontier

**PHASE:** `P01 — Core contracts and protocol spine`

**LATEST PROVEN PHASE:** `P00 — Repository and governance foundation`

**P00 EXIT STATUS:** `PROVEN`

**P00 MACRO TASKS:** `8 / 8 PROVEN`

**NEXT AUTHORIZED MACRO TASK:** `KX-P01-S01-T01 — Define canonical IDs/revisions for Project, Task, WorkUnit, Run, AgentSession, Runtime, Artifact, Evidence, Event`

P00 exit proof is recorded in `docs/evidence/P00-EXIT.md`.

The protected exit PR merged at `25d172c65cdc4723256889d2c624ffc19a581d0b`, and post-merge main CI run `35331802827` passed all five required jobs.

`main` is protected with strict required status checks for:

- JavaScript / TypeScript;
- Rust;
- SpecGrain;
- Diffcipline R1;
- Provenance.

Pull requests are required, administrator enforcement is enabled, conversation resolution is required, required approving review count is zero for the current single-maintainer bootstrap, and force pushes/deletion are disabled.

No donor product source has been imported.

P01 may begin only through bounded SpecGrain refinement of KX-P01-S01-T01. No later P01 task may bypass its declared dependencies.

Proof for P00 is recorded in:

- `docs/evidence/P00-S01-T01.md`;
- `docs/evidence/P00-S01-T02.md`;
- `docs/evidence/P00-S01-T03.md`;
- `docs/evidence/P00-S01-T04.md`;
- `docs/evidence/P00-S02-T01.md`;
- `docs/evidence/P00-S03-T01.md`;
- `docs/evidence/P00-S04-T01.md`;
- `docs/evidence/P00-S04-T02.md`;
- `docs/evidence/P00-EXIT.md`.

## Dependency rule

Implementation proceeds in dependency order from `docs/canonical/EXECUTION_MASTER_PLAN.md` and `specs/tasks.md`.

A later task may begin only when:

- its declared dependencies are PROVEN;
- the task has been refined to a bounded Grain/WorkPacket where SpecGrain is active;
- required context/provenance is current;
- no live repository truth contradicts the planned frontier.

## Delivery method

Kernux uses:

- **SpecGrain** for bounded, independently verifiable work preparation;
- **Diffcipline** for exact-change proof and risk-profile verification.

Canonical task completion state is `PROVEN`, not an agent's claim of `done`.

SpecGrain and Diffcipline are both repository-active and pinned to exact verified source revisions. Diffcipline enforces the always-on exact-diff R1 CI floor; higher-risk tasks still require their explicit profile plus task-specific evidence. The dedicated Provenance job independently runs provenance, notice, and project-license conformance corpora, validates the project license and third-party notice inventory, and validates every tracked donor record from full Git history.

## Founding donor authority

Founder authorization has been stated for full source reuse from:

- `stablyai/orca`;
- TinyFish source / `tinyfish-io/agentql` public reference;
- `wonderwhy-er/DesktopCommanderMCP`.

No donor code has been imported into Kernux yet. The machine-readable provenance contract, notice/license inventory, donor import checklist, Apache-2.0 project-license policy, and CI enforcement are now proven. Before any import, the active import task must still authorize the bounded source change and provide exact source revision, permission/provenance, source/destination mapping, transformation class, preserved notices, dependency/asset review, characterization evidence, and all applicable security evidence.

## Architecture invariants

Implementation must preserve these from the first product commit:

1. `kernuxd` is the privileged local authority.
2. Renderer/agent/browser/plugin code cannot grant itself host privilege.
3. Core architecture is capability-based, not donor-provider-based.
4. Local-first core does not require Kernux cloud.
5. Execution host owns process/session lifecycle truth.
6. Disconnect is not process death; `unverifiable` is a valid state.
7. Side-effect retries require idempotency/ambiguity handling.
8. Untrusted web/document/tool content cannot authorize elevated actions.
9. Secrets are brokered/referenced rather than casually exposed.
10. Evidence is bound to exact revisions/runs; self-report is not proof.
11. macOS, Windows, Linux, WSL, SSH, and folder workspaces are architectural inputs, not later patches.
12. Donor code remains provenance-traceable.
13. Native model-backed agents and external coding agents use the same task/capability/evidence model.
14. Context, search, indexing, and durable memory remain provenance-aware and user-governed.
15. Headless CLI/API and desktop UI are peer surfaces over the same core contracts.
16. Kernux-owned data has explicit retention, export, backup, restore, and deletion semantics.

## First-generation protocol posture

- ACP stable v1: preferred structured coding-agent client path.
- A2A 1.0: independent/opaque agent collaboration adapter.
- MCP 2026-07-28 generation: new native tool/context integration baseline.
- WebMCP: negotiated experimental structured web-action path.
- CDP/Playwright: deterministic browser control.
- KRP: Kernux-owned privileged runtime protocol.

Reverify protocol truth before implementing each protocol-dependent phase.

## First alpha definition

The earliest coherent external alpha is expected after P09 plus a bounded sandbox part of P10, and must include:

- desktop workspace;
- privileged daemon + capability policy;
- local files/process/PTY;
- at least two real agent integrations;
- native model-backed agent path;
- browser deterministic + semantic paths;
- task/fleet basics;
- git/worktree review;
- context/search with inspectable provenance;
- evidence/replay/restart recovery;
- headless CLI/API path;
- at least one isolated runtime.

An alpha is not authorized merely because individual donor features work.

## Completion reporting

When reporting progress, provide:

- PROVEN canonical tasks / total currently planned;
- phase exit gates proven / total;
- applicable golden journeys proven / total;
- platform matrix status;
- R3/security blocker status;
- release-qualification status.

Do not infer project completion from LOC, feature count, donor coverage, or green unit tests alone.
