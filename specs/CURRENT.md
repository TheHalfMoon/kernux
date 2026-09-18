# CURRENT

## Canonical project state

**PROJECT:** Kernux

**CATEGORY:** Agent Operating Environment

**STATUS:** P00_IN_PROGRESS

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

T02 established pinned, reproducible Rust and JavaScript/TypeScript checks. T02.1 established repository-local SpecGrain state and the first bounded successor Grain. T03 established a repository-owned Diffcipline v1.0.0 policy, explicit R0-R3 verification profiles, and a pinned CI proof gate. T04.1 established the strict donor import manifest v1 schema, fail-closed validator, 21-test conformance corpus, and dedicated Provenance CI job proven on both pull-request and main-push event paths.

No product code or founding-donor product code has been imported yet.

## Active frontier

**PHASE:** `P00 — Repository and governance foundation`

**SLICE:** `S00.4 — Provenance system`

**LATEST PROVEN MACRO TASK:** `KX-P00-S04-T01 — Define machine-readable donor provenance/import manifest schema and validation command`

**NEXT AUTHORIZED MACRO TASK:** `KX-P00-S04-T02 — Add third-party notices/license inventory structure and donor import checklist`

T04.1 is proven and the machine-readable provenance validator is now an enforced CI surface. T04.2 is dependency-eligible and must be refined through repository-local SpecGrain before implementation. It completes the human- and machine-reviewable notices/license/checklist layer required before founding-donor import work can begin.

No later task may bypass the dependency order or donor-provenance gates.

Proof is recorded in:

- `docs/evidence/P00-S01-T01.md`;
- `docs/evidence/P00-S01-T02.md`;
- `docs/evidence/P00-S02-T01.md`;
- `docs/evidence/P00-S03-T01.md`;
- `docs/evidence/P00-S04-T01.md`.

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

SpecGrain and Diffcipline are both repository-active and pinned to exact verified source revisions. Diffcipline enforces the always-on exact-diff R1 CI floor; higher-risk tasks still require their explicit profile plus task-specific evidence. The dedicated Provenance job independently runs the donor-manifest conformance corpus and validates every tracked donor record from full Git history.

## Founding donor authority

Founder authorization has been stated for full source reuse from:

- `stablyai/orca`;
- TinyFish source / `tinyfish-io/agentql` public reference;
- `wonderwhy-er/DesktopCommanderMCP`.

No donor code has been imported into Kernux yet. The machine-readable provenance contract and CI validator are now proven. Before import, exact source revision, permission/provenance, source/destination mapping, transformation class, notices, characterization evidence, and all task-specific security evidence remain required.

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
