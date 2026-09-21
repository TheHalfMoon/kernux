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

**PHASE:** `P02 — Privileged kernel (kernuxd)`

**LATEST PROVEN PHASE:** `P01 — Core contracts and protocol spine`

**LATEST PROVEN P01 MACRO TASK:** `KX-P01-S05-T03 — Build canonical protocol fixture corpus including disconnect, duplicate operation, malformed payload and version mismatch`

**P01 PROGRESS:** `7 / 7 macro tasks PROVEN`

**P01 EXIT STATUS:** `PROVEN`

**P02 ENTRY AUTHORIZED:** `KX-P02-S05-T02 ONLY`

**NEXT CANONICAL FRONTIER:** `KX-P02-S05-T02 — Implement user-friendly Safe/Standard/Developer/Autonomous/Custom profiles compiling to granular grants`

**LATEST PROVEN P02 MACRO TASK:** `KX-P02-S05-T01 — Implement policy evaluator and bounded grants/expiry/delegation/deny reasons`

**P02 PROGRESS:** `6 / 9 macro tasks PROVEN`

**KX-P02-S01-T01 QUALIFICATION:** `PROVEN`

**KX-P02-S01-T02 QUALIFICATION:** `PROVEN`

**KX-P02-S02-T01 QUALIFICATION:** `PROVEN`

**KX-P02-S03-T01 QUALIFICATION:** `PROVEN`

**KX-P02-S04-T01 QUALIFICATION:** `PROVEN`

**KX-P02-S05-T01 QUALIFICATION:** `PROVEN`

The identity/revision, capability/Grant, runtime-operation, Event/Evidence, generated-contract, compatibility/versioning, and adversarial-corpus tasks are PROVEN.

Canonical generated-contract proof:

- authoritative JSON Schema: `protocol/schema/krp.v1.schema.json`;
- schema SHA-256: `0ec273c387cf8f344b178c66ae046f75af88945dc428f4a16601156f4e538500`;
- shared fixture source: `protocol/fixtures/v1/core.json`;
- evidence packet: `docs/evidence/P01-S05-T01.md`;
- native SG-000012 evidence record: `sha256:6d3caf7c146995e76228ec4d10ede1b2b1e440a8873b40079313d7d427d475e4`;
- native SG-000013 prerequisite evidence record: `sha256:41d657f718ec18d70bca33d5c73e5decff5a51e7f6ad46066f8509b2883db800`.

Final S05-T01 implementation merged at `a1ee7a16339302152324155357c6f96d1314fd6c`. Native proof records merged at `f4ce48bf5258c2106764d8cfbe6b769b879f5ce3`, and post-merge CI run `35414497010` passed all five protected required jobs.

Generated KRP truth is provider-neutral and schema-bound: one JSON Schema 2020-12 source, deterministic Rust/TypeScript generation, exact schema-digest constants, one shared fixture source consumed across languages, semantic JSON round trips, generated-output drift rejection, and governed dependency admission.

KRP v1 compatibility is now canonically frozen and natively proven:

- compatibility policy: `docs/canonical/KRP_COMPATIBILITY.md`;
- schema SHA-256: `7c8ec2e44777a35c73414da488394ef482a7db9510ba1bda7419929c3bbf03a9`;
- evidence packet: `docs/evidence/P01-S05-T02.md`;
- primary native record: `sha256:a022dc5a6c2c52bf1dfcd988a29384d6eadd2d0c23553162ed4fe2f7ee0613b4`;
- corrective digest-propagation native record: `sha256:82b6cc51af7b85bcd416bfcf44d3c3f2e4e0b930b8612e89ef162192f8c1203a`;
- final implementation merge: `5f805fc014da24a17c1fe330c45c284384deb400`;
- final implementation post-merge CI: `35418918294`, five protected jobs PASS.

The focused T02 matrix covers exact-v1 success, closed decoding, required/optional behavior, protocol/runtime truth separation, Event nonprojection, capability-admission exact matching, and feature-negotiated additive sending.

KRP adversarial conformance is now natively proven:

- shared adversarial corpus: `protocol/fixtures/v1/adversarial.json`;
- Node adversarial gate: `tools/protocol/adversarial.mjs`;
- Rust same-byte mirror: `crates/kernux-contracts/tests/adversarial.rs`;
- evidence packet: `docs/evidence/P01-S05-T03.md`;
- native SG-000016 evidence record: `sha256:d0f0200aec1f62a725add036579ce70808ac54cfb83a31df998f69f75a6c0e74`;
- final implementation merge: `986b0fd6754411885f88f544c63718a9467b3382`;
- final implementation post-merge CI: `35422072516`, five protected jobs PASS.

The adversarial corpus covers all twenty canonical runtime-operation invariants plus thirteen malformed/version wire cases while preserving the explicit boundary between fixture conformance and unimplemented live runtime behavior.

No donor product source has been imported.

KX-P01-S05-T03 is PROVEN after native verification and its protected closeout. P01 has 7 / 7 macro tasks PROVEN. The four frozen P01 exit criteria and native SG-000017 verification are PROVEN with immutable record `sha256:1a56e457ab012d82b6dbe7cd362c2834670a04600fb8e4380cc4c53f0e66931d`. Proof-bearing closeout PR #82 merged at `ab9ef10d15a2250b1e82eccaf9b91218bfd57a98`, and post-merge CI run `35424368459` passed all five protected jobs. P01 exit is PROVEN. KX-P02-S01-T01 is now PROVEN: implementation qualified on `main@e7645d5e950c9f959f5bbe1c3c5e3ad36e339f42`; native SG-000018 verification produced immutable record `sha256:c4a424bc65aed750668e3d58d7339a7c56af6a449799b9d296511a23e53520e6`; proof-bearing PR #91 exact head `5138e8df65de27ab621a033c1105cbe8e66d6d61` passed run `35429680844` 6/6 and exact-head review `PRR_kwDOUevqks8AAAABOTkDww` with zero threads; PR #91 merged at `f2ea966b398e445e57af94ee7b25bfbe1af21f2b`; post-merge run `35429947987` passed all six jobs. Only KX-P02-S01-T02 is authorized as NEXT; later P02 tasks remain PLANNED and P02 phase exit remains unproven. KX-P02-S01-T02 is now PROVEN: implementation qualified on `main@74363207a63b532a237ff505e80c47b49ceca07f`; native SG-000019 verification produced immutable record `sha256:fb9c164cecf8a9c72b2fd47557161d0d17046656fa3ce08bfb2939e172066aa3`; proof-bearing PR #97 exact head `f1b744f97b7a7e84d2ae40dfb223199e9dece588` passed run `35435714668` 6/6 and exact-head review `PRR_kwDOUevqks8AAAABOTz3Bg` with zero threads; PR #97 merged at `78631b3095ce83ec39985f54c0ebc8e769ca4203`; post-merge run `35435786206` passed all six jobs. Only KX-P02-S02-T01 is authorized as NEXT; all other later P02 tasks remain PLANNED and P02 phase exit remains unproven. KX-P02-S02-T01 is now PROVEN: implementation qualified on `main@8f7a43361a0a0d44202e336358e64a47d7686cfc`; native SG-000020 verification produced immutable record `sha256:c4dc5233ff46ef728c3338182f48c1ff26cefb955b1048cf33097032b3327cb2`; proof-bearing PR #105 exact head `2fae11cd955cc3343a576753509f21df80c1b433` passed run `35442301123` 6/6 and exact-head review `PRR_kwDOUevqks8AAAABOUOGWg` with zero threads; PR #105 merged at `f0e457d55021e59799f898a51e634a1d8d284cdc`; post-merge run `35443098538` passed all six jobs. Only KX-P02-S03-T01 is authorized as NEXT; all later P02 tasks remain PLANNED and P02 phase exit remains unproven. KX-P02-S03-T01 is now PROVEN: implementation qualified on `main@3bf286402b70790beb997bd555cdf1b064f0492d`; native SG-000021 verification produced immutable record `sha256:e7136d88dd6ec7444054fb0a21103630c296975f0a68ad8f35c996802f9229ef`; proof-bearing retry PR #119 exact head `77f9b3395f38693f9d635acffbcf0338e9fff6f3` passed run `35453792735` 6/6 and exact-head review `PRR_kwDOUevqks8AAAABOU39BQ` with zero threads; PR #119 merged at `ec252919e243c2e815dd878eb2d9bd17077f11d8`; post-merge run `35454187226` passed all six jobs. Only KX-P02-S04-T01 is authorized as NEXT; all later P02 tasks remain PLANNED and P02 phase exit remains unproven. KX-P02-S04-T01 is now PROVEN: implementation qualified on `main@114ac49cf6d8bb154e3cb1c52627ab9fcf6d9a7e`; native SG-000022 verification produced immutable record `sha256:6c358678f4b2225543637212a36cda613d9a914f4e4e93fc2cebf7813ec2e9cb`; proof-bearing PR #125 exact head `779d036c22d571cf284809d8469b059d8a32f301` passed run `35458690404` 6/6 and exact-head review `5256775962` with zero threads; PR #125 merged at `415f7fe41b4eff85d61137132256ef97195a4fbf`; post-merge run `35458837422` passed all six jobs. Only KX-P02-S05-T01 is authorized as NEXT; KX-P02-S05-T02 and all S06 tasks remain PLANNED and P02 phase exit remains unproven. KX-P02-S05-T01 is now PROVEN through the corrective SG-000024 path: immutable native record `sha256:f6340663290f6a609777bc1c2d10269f85e5107aa5aa4f385a06ab67b41b452e`; proof-bearing PR #152 exact head `a8efc29352979ea6fd4651c554adda746c9e9cbb` passed native macOS qualification `35577145490`, exact-head CI `35577135982` 6/6, and exact-head review `5264407121` with zero threads; PR #152 merged normally at `d93b54de0013e47809336db60f857d5435d8c6a4`; post-merge CI `35579619127` passed all six jobs. Only KX-P02-S05-T02 is authorized as NEXT; all S06 tasks remain PLANNED and P02 phase exit remains unproven.

Proof for the current frontier includes:

- `docs/evidence/P00-EXIT.md`;
- `docs/evidence/P01-S01-T01.md`;
- `docs/evidence/P01-S02-T01.md`;
- `docs/evidence/P01-S03-T01.md`;
- `docs/evidence/P01-S04-T01.md`;
- `docs/evidence/P01-S05-T01.md`;
- `docs/evidence/P01-S05-T02.md`;
- `docs/evidence/P01-S05-T03.md`;
- `docs/evidence/P01-EXIT.md` — P01 phase-exit proof, native SG-000017 record, protected closeout, and post-merge qualification.
- `docs/evidence/P02-S01-T01.md` — PROVEN T01 implementation, native SG-000018 proof, protected closeout, exact-head review, merge, and post-merge qualification.
- `docs/evidence/P02-S01-T02.md` — PROVEN T02 implementation, native SG-000019 proof, protected closeout, exact-head review, merge, and post-merge qualification.
- `docs/evidence/P02-S02-T01.md` — PROVEN local-identity implementation, native SG-000020 proof, protected closeout, exact-head review, merge, and post-merge qualification.
- `docs/evidence/P02-S03-T01.md` — PROVEN metadata-store implementation, native SG-000021 proof, protected closeout, exact-head review, merge, and post-merge qualification.
- `docs/evidence/P02-S04-T01.md` — PROVEN Artifact CAS implementation, native SG-000022 proof, protected closeout, exact-head review, merge, and post-merge qualification.
- `docs/evidence/P02-S05-T01.md` — PROVEN policy/Grant implementation outcome through preserved SG-000023 negative evidence plus corrective native SG-000024 proof, protected closeout, exact-head review, merge, and post-merge qualification.

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

The founder further states that the TinyFish authorization is a personal, full-source permission to copy and use the source as needed for Kernux, and that the source repositories recorded by Kernux as authorized reuse sources are covered by founder-supplied reuse permission. This statement is the founder-supplied authorization basis; exact source identity/revision, provenance, preserved notices, embedded third-party obligations, assets, models, trademarks, and hosted-service terms remain independently governed.

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
17. Owner-sustainable execution is mandatory: core usefulness cannot require owner-subsidized variable compute; local/self-hosted, BYOK, BYOC, organization-funded, or explicitly paid managed paths preserve capability without making Kernux cloud a hidden prerequisite.

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
