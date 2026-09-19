# P02-S04-T01 dependency-admission gate

## Scope

This evidence records only the direct hashing dependency prerequisite for KX-P02-S04-T01 / SG-000022.

It does not implement or prove Artifact CAS layout, ingest, verified reads, publication, deduplication, metadata binding, retention behavior, deletion/GC, policy, secrets, daemon wiring, or P02 completion.

No CAS semantics may land until this dependency candidate merges through the ordinary protected path and its exact post-merge six-job qualification succeeds.

## Exact direct dependency

| Package | Version | Direct features | Defaults | Observed license | Purpose |
| --- | --- | --- | --- | --- | --- |
| sha2 | 0.11.0 | none | disabled | MIT OR Apache-2.0 | Incremental SHA-256 computation/verification for bounded local Artifact CAS bytes. |

Exact manifest requirement: sha2 = { version = "=0.11.0", default-features = false }.

The compound SPDX expression is recorded as exception-reviewed and binds this evidence path. Both alternatives are compatible permissive license families already accepted by project policy; this exception approves only this exact package/version/purpose/feature set and does not expand the automatic allowlist.

## Minimality

sha2 0.11.0 was already present in the qualified base Cargo.lock through ed25519-dalek 3.0.0. S04 makes that exact package a direct kernux-store dependency rather than adding a second hashing implementation, a hex package, an async runtime, or another storage framework.

Direct defaults are disabled. No optional sha2 feature is requested.

## Implementation boundary

At this dependency gate:

- crates/kernux-store/src/cas.rs does not exist;
- no existing Rust source file changes;
- no Artifact bytes are written or read by new code;
- SQLite schema/version remains unchanged;
- no CAS path/layout/publication/dedup/read/bind/delete/GC API is introduced.

The exact lock delta, feature observations, validation result, and qualification evidence below are completed from the candidate itself.

## Exact lock and feature observations

Qualified candidate observations:

- repository dependency validation: PASS with 12 direct external dependencies;
- qualified-base external locked package identities: 64;
- candidate external locked package identities: 64;
- added external package/version/source identities: none;
- removed external package/version/source identities: none;
- Cargo.lock changes only the existing kernux-store package dependency list by adding sha2;
- candidate Cargo.lock SHA-256: da75f3c4bd0eedfcdaa554839fa48b3768dea011e2b1011ffd5ea1ecf6ea35d6;
- sha2 registry checksum remains 446ba717509524cb3f22f17ecc096f10f4822d76ab5c0b9822c5f9c284e825f4;
- cargo metadata observes sha2 0.11.0 license MIT OR Apache-2.0 and rust-version 1.85;
- kernux-store direct requirement is exactly =0.11.0;
- kernux-store requests zero sha2 optional features;
- kernux-store sets uses_default_features=false for sha2;
- the already-resolved sha2 transitive support graph remains cfg-if, cpufeatures, and digest; no second sha2 version is introduced.

The first post-manifest compile attempt intentionally used cargo check --locked before the lockfile edge was refreshed. Cargo rejected that attempt because the lockfile required an update. That attempt remains FAIL and is not qualification evidence. The corrected candidate updated only the internal kernux-store dependency edge with cargo check --offline, then reran locked validation successfully.

No source file under crates/kernux-store/src changes in this dependency candidate, and crates/kernux-store/src/cas.rs remains absent.

## Qualification requirements

The exact candidate must pass repository dependency validation, exact feature-minimality and lock-delta assertions, git diff --check, full pnpm check, Diffcipline R3, Alibaba OpenCodeReview Delegation Mode plus complete manual accounting for exclusions, all five protected exact-head jobs plus native Windows kernuxd, exact-head review with zero unresolved threads, ordinary merge, and exact post-merge six-job qualification.

Only then may SG-000022 CAS implementation begin.
