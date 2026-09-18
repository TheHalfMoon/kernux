# P00 Exit Gate Evidence

## Status

**PHASE:** `P00 — Repository and governance foundation`

**EXIT STATUS:** `PROVEN`

**P00 EXIT PROVEN:** `TRUE`

**P01 IMPLEMENTATION AUTHORIZED:** `TRUE AFTER CANONICAL CLOSEOUT MERGES`

**ACTIVE GRAIN:** `SG-000007 — KX-P00-EXIT — Protect main and qualify the P00 phase exit gate`

The eight P00 macro tasks and the P00 phase exit are now PROVEN. Required `main` protection is configured, the protected P00 exit PR merged without bypass, and the resulting `main` revision passed all five required jobs.

This document is an evidence packet and blocker record, not a completion claim.

## SG-000007 identity

Refinement PR:

`#31 — docs: refine P00 exit gate`

Refinement head:

`bd34125d7a3b9c130bd3b69b3e9a63ff29b1a78f`

Refinement merge:

`d52b5e7e6db04f4655b66beff5d62720f9d5bd77`

Spec revision:

`sha256:fd43b7065dcc13ef6e2427e8a7d0688449660612d16366c90fd61fba50b23628`

WorkPacket:

`sha256:044edbc4f501adbf7069c55b4df6f7ea2a018246f6adbabac7bfae89a952f06e`

Context-plan digest:

`sha256:b748d028a0a200018d7421a45f5a7f9658692f2ae2d159bfd7efe07c7eac4379`

Context accounting:

- 8 required sources;
- 20,071 bytes;
- 5,021 / 6,500 estimated tokens.

Refinement PR CI:

`35326522853` — SUCCESS.

Observed required jobs:

- JavaScript / TypeScript — SUCCESS;
- Rust — SUCCESS;
- SpecGrain — SUCCESS;
- Diffcipline R1 — SUCCESS;
- Provenance — SUCCESS.

Post-merge refinement CI on `main@d52b5e7e6db04f4655b66beff5d62720f9d5bd77`:

`35327049103` — SUCCESS.

Observed required jobs:

- JavaScript / TypeScript — SUCCESS;
- Rust — SUCCESS;
- SpecGrain — SUCCESS;
- Diffcipline R1 — SUCCESS;
- Provenance — SUCCESS.

## P00 macro-task registry

All eight P00 macro tasks remain PROVEN:

- `KX-P00-S01-T01`;
- `KX-P00-S01-T02`;
- `KX-P00-S02-T01`;
- `KX-P00-S03-T01`;
- `KX-P00-S04-T01`;
- `KX-P00-S04-T02`;
- `KX-P00-S01-T03`;
- `KX-P00-S01-T04`.

No P01 macro task is authorized while the phase exit is unproven.

## Fresh-clone and baseline evidence

A real shallow fresh clone of exact post-refinement `main@d52b5e7e6db04f4655b66beff5d62720f9d5bd77` was created after branch protection was enabled.

Observed from that clone:

- `pnpm install --frozen-lockfile` — PASS;
- workspace baseline — PASS;
- JavaScript/TypeScript format, lint, typecheck, and tests — PASS;
- Rust format, clippy, and tests — PASS;
- SpecGrain check — PASS;
- full `pnpm check` — PASS.

The live CI run on the same main revision, `35327049103`, independently covers the supported Node 24 execution baseline.

**Status:** PROVEN.

## SpecGrain / Diffcipline reproducibility

Repository-owned pinned SpecGrain and Diffcipline remain active.

SG-000007 refinement:

- SpecGrain check — PASS;
- Grain promotion — PASS;
- WorkPacket export — PASS;
- local `pnpm check` — PASS;
- local Diffcipline R1 — PASS;
- PR-head CI — PASS;
- post-merge main CI — PASS.

The current main CI independently runs the SpecGrain and Diffcipline R1 jobs.

**Status:** PROVEN for current P00 tooling reproducibility, subject to the separate merge-enforcement blocker below.

## Provenance and donor-import state

The exact post-refinement main Provenance job is SUCCESS.

At `main@d52b5e7e6db04f4655b66beff5d62720f9d5bd77`:

- `third_party/provenance/` contains its README and schema directory only;
- no donor import manifest record exists;
- canonical CURRENT states that no product code or founding-donor product code has been imported;
- project-license, notice-inventory, and provenance validators are exercised by the Provenance CI job.

**Current donor-import state:** no governed donor product import exists, and no ungoverned donor product import has been established by repository evidence.

## Draft closeout PR qualification

Draft PR:

`#32 — docs: qualify P00 exit gate`

Current exact head before protection:

`851e9a794a50442bd776c491ad1c209ad2d51cb5`

Pull-request CI:

`35328524376` — SUCCESS.

Observed jobs:

- JavaScript / TypeScript — SUCCESS;
- Rust — SUCCESS;
- SpecGrain — SUCCESS;
- Diffcipline R1 — SUCCESS;
- Provenance — SUCCESS.

Review threads observed: none.

This proves the repository-side closeout candidate is green before branch protection is enabled. It does **not** satisfy the protected-closeout criterion because the required `main` protection has not been established.

## Main branch protection proof

The required classic branch protection was applied through the repository owner's authenticated GitHub CLI and immediately read back from the GitHub API.

Observed protection:

- branch: `main`;
- `protected: true`;
- `protection.enabled: true`;
- required-status-check enforcement: `everyone`;
- strict/up-to-date required checks: `true`;
- required contexts, exactly:
  - `JavaScript / TypeScript`;
  - `Rust`;
  - `SpecGrain`;
  - `Diffcipline R1`;
  - `Provenance`;
- required pull request reviews policy present;
- required approving review count: `0`;
- code-owner reviews: `false`;
- last-push approval: `false`;
- administrator enforcement: `true`;
- conversation resolution required: `true`;
- force pushes: `false`;
- branch deletion: `false`;
- required linear history: `false`;
- required signatures: `false`;
- branch lock: `false`.

This establishes that `Provenance` is now a merge-time required status check rather than only an evidence-producing workflow job.

**Status:** PROVEN.

## Protected closeout PR evidence

PR #32 remained draft while protection was installed.

A branch update made after protection produced exact head:

`2df3cf78d2ff61b28f89ec42e575fe08c5bab0a5`

Protected pull-request CI:

`35331377363` — SUCCESS.

Observed jobs:

- JavaScript / TypeScript — SUCCESS;
- Rust — SUCCESS;
- SpecGrain — SUCCESS;
- Diffcipline R1 — SUCCESS;
- Provenance — SUCCESS.

Review threads observed: none.

The final closeout head will also be required by GitHub to satisfy these five checks before merge. No administrator bypass is authorized.

**Protected closeout policy status:** PROVEN.

## Protected P00 exit merge and post-merge qualification

PR #32 final exact head:

`9fcba2f9f5e8f3ad80a62a36b1d8d7fe68dde6d7`

Protected pull-request CI:

`35331700649` — SUCCESS.

Observed required jobs:

- JavaScript / TypeScript — SUCCESS;
- Rust — SUCCESS;
- SpecGrain — SUCCESS;
- Diffcipline R1 — SUCCESS;
- Provenance — SUCCESS.

GitHub reported the ready PR as:

- `mergeStateStatus: CLEAN`;
- `mergeable: MERGEABLE`;
- no review threads;
- protection read-back still required exactly the five canonical contexts;
- administrator enforcement remained enabled;
- required approving review count remained zero;
- force pushes and deletion remained disabled.

PR #32 merged through the ordinary merge endpoint with exact expected head and no bypass.

Merge revision:

`25d172c65cdc4723256889d2c624ffc19a581d0b`

Post-merge main CI:

`35331802827` — SUCCESS.

All five canonical jobs passed on the resulting `main` revision.

**Status:** PROVEN.

## P00 acceptance conclusion

**PROVEN** for the `P00 — Repository and governance foundation` exit gate.

Established:

- fresh-clone reproducibility: PROVEN;
- JavaScript/TypeScript baseline: PROVEN;
- Rust baseline: PROVEN;
- SpecGrain reproducibility: PROVEN;
- Diffcipline R1 reproducibility: PROVEN;
- donor provenance/notice/license enforcement: PROVEN;
- no current donor product import: PROVEN;
- protected `main` with required pull requests/status checks: PROVEN;
- `Provenance` as required merge-time status check: PROVEN;
- protected closeout PR without bypass: PROVEN;
- post-merge main qualification: PROVEN;
- P00 macro-task registry: 8 / 8 PROVEN.

No P01 product/contract implementation was performed before this proof existed.

## Completion boundary

`P00_COMPLETE = TRUE`

`P01_AUTHORIZED = TRUE AFTER THIS CANONICAL CLOSEOUT MERGES`

`PRODUCT_IMPLEMENTATION_STARTED = FALSE`

`RELEASE_READY = FALSE`

No later phase may bypass this blocker.
