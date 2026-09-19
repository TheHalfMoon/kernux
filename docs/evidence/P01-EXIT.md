# P01 Exit Gate Evidence

## Status

**PHASE:** `P01 — Core contracts and protocol spine`

**EXIT QUALIFICATION:** `QUALIFIED_PENDING_NATIVE_CLOSEOUT`

**P01 EXIT PROVEN:** `FALSE`

**P02 IMPLEMENTATION AUTHORIZED:** `FALSE`

**ACTIVE GRAIN:** `SG-000017 — KX-P01-EXIT — Prove the P01 core-contract and protocol-spine exit gate`

All seven P01 macro tasks are PROVEN and the four frozen P01 exit criteria independently pass on the qualified assessment base.

This packet deliberately does not mark the phase exit PROVEN and does not authorize P02. Canonical completion still requires native SG-000017 verification, a protected final closeout PR, ordinary merge without bypass, and successful five-job post-merge main qualification.

## SG-000017 identity

Refinement PR: `#80 — docs: refine P01 exit gate`

Refinement head: `652642e22c9cfeeb93892274eebbae0f4e5d4225`

Refinement merge: `da7edf02e0f347836ac8468bd1570dc57d49bd42`

Spec revision: `sha256:7d90f1f773d99ea982bd94a0a6ec4f4f0ecdddd7405816b8e0ccf2465ed581c6`

WorkPacket: `sha256:e31a693fc93da43d344337bab8d93b2a41334fb6608687001ab1b6df775a4df6`

Context-plan digest: `sha256:3252450974a19f8c8177b752cb0c3658054ae67d2eb5a09c405205d7b6cfe7e9`

Context accounting:

- 16 required revision-bound sources;
- 10,477 / 14,000 context tokens;
- 8 acceptance criteria;
- 11 required evidence identifiers;
- four authorized implementation paths.

Refinement exact-head CI: `35423303436` — five required jobs SUCCESS.

Refinement post-merge CI on `main@da7edf02e0f347836ac8468bd1570dc57d49bd42`: `35423350781` — five required jobs SUCCESS.

Observed jobs in both runs:

- JavaScript / TypeScript — SUCCESS;
- Rust — SUCCESS;
- SpecGrain — SUCCESS;
- Diffcipline R1 — SUCCESS;
- Provenance — SUCCESS.

## P01 macro-task registry

All seven P01 macro tasks remain PROVEN, from `KX-P01-S01-T01` through `KX-P01-S05-T03` as listed in `specs/tasks.md`.

Canonical evidence packets are present for all seven macro tasks.

Native SpecGrain evidence directories `SG-000008` through `SG-000016` each contain at least one immutable record with `verified=true` and `issues=[]`.

**Status:** PASS.

## Exit criterion 1 — cross-language contracts executable

Authoritative KRP schema: `protocol/schema/krp.v1.schema.json`.

Current authoritative schema SHA-256: `7c8ec2e44777a35c73414da488394ef482a7db9510ba1bda7419929c3bbf03a9`.

Focused exact-base execution on `main@da7edf02e0f347836ac8468bd1570dc57d49bd42`:

- `node tools/protocol/generate.mjs --check` — PASS;
- `node tools/protocol/conformance.mjs` — PASS, 8 shared core cases;
- `node tools/protocol/compatibility.mjs` — PASS, 10 payload + 1 truth + 2 event-projection + 3 capability-admission + 3 feature-gate cases;
- `node tools/protocol/adversarial.mjs` — PASS, 20 policy + 13 wire cases;
- `cargo test -p kernux-contracts --test fixtures` — PASS, 1/1;
- `cargo test -p kernux-contracts --test compatibility` — PASS, 3/3;
- `cargo test -p kernux-contracts --test adversarial` — PASS, 3/3;
- full `pnpm check` — PASS.

The normal baseline also runs TypeScript no-emit typechecking, generated-output drift checks, Rust formatting, Clippy with `-D warnings`, all Rust workspace tests, and SpecGrain validation.

The shared core and adversarial fixture bytes remain consumed across Node/schema and Rust paths rather than being re-authored per language.

**Exit criterion:** PASS.

## Exit criterion 2 — provider names absent from core

The assessment scanned these authoritative/public/core surfaces:

- `protocol/schema/krp.v1.schema.json`;
- `packages/contracts/src/generated.ts`;
- `crates/kernux-contracts/src/generated.rs`;
- `protocol/fixtures/v1/core.json`;
- `protocol/fixtures/v1/adversarial.json`.

The scan checked the provider/model markers already enforced by the adversarial checker: `openai`, `anthropic`, `claude`, `gemini`, `qwen`, `ollama`, `mistral`, and `groq`.

Observed matches on the core/public surfaces: `0`.

`tools/protocol/adversarial.mjs` intentionally contains those strings only as a denylist that fails if a provider name leaks into the adversarial fixture. Those checker-enforcement strings are not KRP wire/public semantics.

**Exit criterion:** PASS.

## Exit criterion 3 — disconnect cannot imply exit

The canonical runtime model keeps contact state and execution state separate.

Executable shared adversarial cases prove the required truth-preservation boundary:

- `ADV-01`: disconnected contact + last-known live execution remains `live`;
- `ADV-02`: heartbeat timeout does not synthesize exit; execution remains `live`;
- `ADV-17`: request timeout without authoritative execution observation becomes `unverifiable`, not failed/exited;
- `ADV-18`: controller cache loss without authoritative exit observation becomes `unverifiable`, not exited.

The Node adversarial checker and Rust same-byte mirror both pass over the canonical corpus. The Rust evidence-boundary test also rejects turning fixture conformance into claims about unimplemented live runtime behavior.

**Exit criterion:** PASS.

## Exit criterion 4 — Grants structurally bounded

The authoritative `Grant` definition is a closed object with `additionalProperties=false`.

Required Grant fields are exactly:

- `grant_id`, `subject_scope`, `action`, `resource_uri`, `resource_match`, `runtime`, `constraints`, `consequence_ceiling`;
- `issuer_authority`, `policy_revision`, `issued_at`, `not_before`, `expires_at`, `max_uses`, `delegation_depth`.

Structural bounds include:

- one exact action grammar rather than an issued-action wildcard;
- canonical `kernux://` resource identity and explicit resource-match semantics;
- explicit subject and runtime references;
- typed `ConstraintSet` and explicit consequence ceiling;
- policy revision >= 1;
- not-before / expiry fields;
- finite `max_uses >= 1`;
- `delegation_depth >= 0`;
- optional parent Grant identity for delegation lineage.

The canonical `bounded_grant` shared fixture binds action `files.write`, one project filesystem resource, subtree matching, explicit runtime revision, allowed-root/byte/use constraints, consequence ceiling `C2`, a finite time window, one allowed use, and delegation depth zero.

The canonical authority model still requires current subject, action, resource, runtime, constraints, consequence, capability negotiation, Grant state, and policy checks. Runtime advertisement or possession of a Grant identifier alone does not create or widen authority.

**Exit criterion:** PASS.

## No-runtime-overclaim boundary

This exit assessment proves the P01 contract/protocol spine only.

It does not prove:

- `kernuxd` daemon behavior or local transport;
- process control, SQLite persistence, or recovery;
- live cancellation delivery or reconnect persistence;
- exactly-once execution or durable replay protection;
- browser/computer runtime behavior;
- P02 implementation;
- release readiness.

**Boundary status:** PASS.

## Exact-base local qualification

Assessment base: `main@da7edf02e0f347836ac8468bd1570dc57d49bd42`.

Observed before this evidence packet was authored:

- P01 registry/native/evidence integrity — PASS;
- generator/shared conformance/compatibility/adversarial gates — PASS;
- focused Rust fixture/compatibility/adversarial tests — PASS;
- provider-neutral core scan — PASS, zero matches;
- provider-denylist/core-semantics distinction — PASS;
- disconnect truth assertions — PASS;
- Grant closed-structure and bounded-fixture assertions — PASS;
- full `pnpm check` — PASS.

No failed check is relabeled as PASS.

## Current protected-main evidence

Live branch metadata reports `main` as protected and requires these five status contexts:

- JavaScript / TypeScript;
- Rust;
- SpecGrain;
- Diffcipline R1;
- Provenance.

This packet does not claim a fresh read of branch-protection administration fields that the connected GitHub App cannot access.

The final closeout must still pass these five exact-head checks and merge through the ordinary protected path.

## Qualification state

The four frozen P01 exit criteria are independently qualified.

`P01_EXIT_PROVEN = FALSE`

`P02_IMPLEMENTATION_AUTHORIZED = FALSE`

The canonical transition is intentionally withheld until:

1. this qualification change passes local checks, Diffcipline R2, OpenCodeReview accounting, exact-head protected CI, ordinary merge, and post-merge main qualification;
2. SG-000017 receives native verification against that qualified implementation revision;
3. a final protected closeout PR appends the immutable native proof, marks P01 exit PROVEN, and advances only `KX-P02-S01-T01` to NEXT;
4. the final closeout merge itself receives successful five-job post-merge main qualification.

## Next frontier

Native SG-000017 verification after this qualification merge and post-merge main qualification.

No P02 implementation is authorized before the final protected closeout completes.
