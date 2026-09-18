# P00 Exit Gate Evidence

## Status

**PHASE:** `P00 — Repository and governance foundation`

**EXIT STATUS:** `BLOCKED_EXTERNAL`

**P00 EXIT PROVEN:** `FALSE`

**P01 IMPLEMENTATION AUTHORIZED:** `FALSE`

**ACTIVE GRAIN:** `SG-000007 — KX-P00-EXIT — Protect main and qualify the P00 phase exit gate`

The eight P00 macro tasks are canonically PROVEN. The phase exit is not yet proven because merge-time protection of `main` has not been established with repository-admin evidence.

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

A real shallow fresh clone of `main@296e123b63ce9666f86b9b0a0765e80cce51c407` was created before SG-000007 refinement.

Observed from that clone:

- `pnpm install --frozen-lockfile` — PASS;
- workspace baseline — PASS;
- formatting — PASS;
- JavaScript/TypeScript lint/typecheck/tests — PASS;
- Rust format/clippy/tests — PASS;
- SpecGrain check — PASS;
- full `pnpm check` — PASS.

The only repository change between that fresh-clone revision and `d52b5e7e6db04f4655b66beff5d62720f9d5bd77` is the SG-000007 refinement spec.

Exact SG-000007 main CI is green, but an exact post-refinement local fresh-clone run is not independently recorded in this packet yet. This criterion therefore remains **PARTIAL**, not silently upgraded to PASS.

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

## Merge-enforcement blocker

Before SG-000007 refinement, live repository-administration truth was queried through the repository owner's authenticated GitHub CLI:

- branch protection endpoint for `main`: `404 Branch not protected`;
- repository rulesets: `[]`.

Therefore the five CI jobs were evidence-producing checks but were not proven merge requirements.

SG-000007 requires `main` protection with:

1. pull requests required;
2. strict required-status-check mode / branch up to date before merge;
3. exactly these required checks:
   - `JavaScript / TypeScript`;
   - `Rust`;
   - `SpecGrain`;
   - `Diffcipline R1`;
   - `Provenance`;
4. required approving review count effectively zero for the single-maintainer bootstrap;
5. conversation resolution required;
6. administrator enforcement / no bypass;
7. force pushes disabled;
8. branch deletion disabled;
9. no extra signed-commit, linear-history, deployment, CODEOWNERS, or unrelated gate introduced solely for P00 exit.

### Current execution-environment limitation

No available tool in the current execution environment can presently apply or verify the administration setting:

- the connected GitHub App can read/write repository content and PRs but its installation token does not expose GitHub administration / branch-protection access;
- the available authenticated-computer connector reports its remote-call allowance exhausted for the current period;
- the available browser-automation environment has no saved GitHub credentials.

These are tool-access limitations, not evidence that protection exists.

**Branch protection status for P00 proof:** NOT PROVEN.

## Required remaining proof

P00 remains blocked until all of the following are observed with real evidence:

- [ ] enable the exact SG-000007 `main` protection policy;
- [ ] read back the live protection configuration with repository-admin authority;
- [ ] prove `Provenance` is a required `main` status check, not only a workflow job;
- [ ] record an exact post-refinement fresh-clone `pnpm install --frozen-lockfile && pnpm check` pass, or an equivalently explicit fresh-clone proof accepted by the Grain;
- [ ] update this packet from `BLOCKED_EXTERNAL` to `PROVEN` only after the above evidence exists;
- [ ] update `specs/CURRENT.md` to P00-complete truth and advance `KX-P01-S01-T01` only then;
- [ ] run the P00 closeout PR under the protected rule without admin bypass;
- [ ] obtain final post-merge main CI success.

## Completion boundary

`P00_COMPLETE = FALSE`

`P01_AUTHORIZED = FALSE`

`PRODUCT_IMPLEMENTATION_STARTED = FALSE`

`RELEASE_READY = FALSE`

No later phase may bypass this blocker.
