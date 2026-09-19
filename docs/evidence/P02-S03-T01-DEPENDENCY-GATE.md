# P02-S03-T01 dependency-admission gate

## Scope

This packet records only the direct dependency prerequisite for `KX-P02-S03-T01` / `SG-000021`.

It authorizes an empty `kernux-store` crate shell plus the exact SQLite/UUID dependencies required by the governing WorkPacket. It does **not** prove schema, migration, WAL, metadata CRUD, Event append, corruption/recovery, CAS, policy, secrets, daemon wiring, or P02 completion.

No store semantics may land until this dependency candidate merges and its exact post-merge six-job qualification succeeds.

## Exact direct dependencies

| Package | Version | Direct features | Defaults | Observed license | Purpose |
| --- | --- | --- | --- | --- | --- |
| `rusqlite` | `0.40.2` | `bundled` | disabled | `MIT` | Synchronous SQLite wrapper with repository-resolved bundled SQLite. |
| `uuid` | `1.26.1` | `std` | disabled | `Apache-2.0 OR MIT` | Strict canonical RFC 9562 UUIDv7 parsing/version/variant validation. |

The exact manifest requirements are:

```toml
rusqlite = { version = "=0.40.2", default-features = false, features = ["bundled"] }
uuid = { version = "=1.26.1", default-features = false, features = ["std"] }
```

`rusqlite` is automatically permitted by the canonical license policy because its exact observed expression is `MIT`.

`uuid` uses a compound SPDX expression, so the dependency registry records it as `exception-reviewed` and binds this evidence path. Both alternatives are already-compatible permissive licenses; this review does not expand the repository-wide automatic allowlist or approve another version/feature set.

## Bundled SQLite review

`rusqlite 0.40.2` resolves `libsqlite3-sys 0.38.2` (`MIT`). The selected `bundled` feature compiles the crate's bundled SQLite amalgamation instead of relying on an independently installed host SQLite.

Exact bundled source observed in the resolved crate:

- SQLite version: `3.53.2`;
- amalgamation SHA-256: `0a409f1633283fa31a9126b11fbfd64a1991c5d30defad07e5745d4667f5e23d`;
- the amalgamation source states that the author disclaims copyright to the SQLite source code.

The `libsqlite3-sys` wrapper remains MIT-licensed. SQLCipher, OpenSSL, loadable extensions, build-time bindgen, session/preupdate hooks, and unrelated rusqlite optional features are not enabled by this direct dependency selection.

## Feature minimality

The exact host feature tree proves:

- direct `rusqlite` feature: `bundled`;
- no direct `rusqlite` `default`, `cache`, `ffi-sqlite-wasm-rs`, `sqlcipher`, `load_extension`, `serde_json`, `uuid`, or ORM feature;
- direct `uuid` feature: `std` only;
- no direct UUID generation (`v4`/`v7` RNG), serde, JavaScript, or fast-RNG feature;
- `cargo tree --locked -d` reports no duplicate packages on the host-resolved graph.

`libsqlite3-sys` internally exposes its own default minimum-version support while the selected `bundled` feature also activates bundled bindings/compilation. This transitive behavior is recorded rather than misrepresented as a direct rusqlite default feature.

Cargo.lock contains target-specific `uuid` WebAssembly support packages because Cargo records conditional package metadata for supported targets. They are not active in the current host feature tree and do not mean the forbidden rusqlite wasm default feature is enabled.

## Fail-closed sequencing

The repository-local dependency validator was run after declaring the manifest/approval entries but before this exception evidence existed. It failed exactly as required:

`Dependency admission: FAIL — cargo:uuid exception path is not canonical evidence: docs/evidence/P02-S03-T01-DEPENDENCY-GATE.md`

That result remains FAIL. It is not reused or relabeled as successful evidence. The `kernux-store` crate contained only a compile-only shell at that point.

## Exact lock delta

No package present on the qualified base lockfile was removed. The candidate introduces the following external locked packages:

| Package | Version | Observed license |
| --- | --- | --- |
| `bitflags` | `2.13.2` | `MIT OR Apache-2.0` |
| `bumpalo` | `3.20.3` | `MIT OR Apache-2.0` |
| `cc` | `1.4.7` | `MIT OR Apache-2.0` |
| `fallible-iterator` | `0.3.0` | `MIT/Apache-2.0` |
| `fallible-streaming-iterator` | `0.1.9` | `MIT/Apache-2.0` |
| `find-msvc-tools` | `0.1.13` | `MIT OR Apache-2.0` |
| `futures-core` | `0.3.34` | `MIT OR Apache-2.0` |
| `futures-task` | `0.3.34` | `MIT OR Apache-2.0` |
| `futures-util` | `0.3.34` | `MIT OR Apache-2.0` |
| `js-sys` | `0.3.105` | `MIT OR Apache-2.0` |
| `libsqlite3-sys` | `0.38.2` | `MIT` |
| `once_cell` | `1.21.4` | `MIT OR Apache-2.0` |
| `pin-project-lite` | `0.2.17` | `Apache-2.0 OR MIT` |
| `pkg-config` | `0.3.34` | `MIT OR Apache-2.0` |
| `rusqlite` | `0.40.2` | `MIT` |
| `rustversion` | `1.0.23` | `MIT OR Apache-2.0` |
| `shlex` | `2.0.1` | `MIT OR Apache-2.0` |
| `slab` | `0.4.12` | `MIT` |
| `smallvec` | `1.16.1` | `MIT OR Apache-2.0` |
| `uuid` | `1.26.1` | `Apache-2.0 OR MIT` |
| `vcpkg` | `0.2.15` | `MIT/Apache-2.0` |
| `wasm-bindgen` | `0.2.128` | `MIT OR Apache-2.0` |
| `wasm-bindgen-macro` | `0.2.128` | `MIT OR Apache-2.0` |
| `wasm-bindgen-macro-support` | `0.2.128` | `MIT OR Apache-2.0` |
| `wasm-bindgen-shared` | `0.2.128` | `MIT OR Apache-2.0` |


External new locked package count: `25`.

Candidate `Cargo.lock` SHA-256:

`3a6c0ced60c39d1bb7bda2c3aab79e1fb24164eb1cbb80ace40af73eadb5a50d`

This exact transitive inventory is dependency evidence for the current lockfile only. It is not a permanent allowlist, vulnerability assessment, release SBOM, or signature qualification.

## CI boundary

The existing protected context names remain unchanged.

The additive `Windows kernuxd` job keeps its exact job name and now runs `kernux-store` Clippy/tests alongside `kernuxd` and `kernux-identity` on native `windows-latest`. The normal Rust workspace job covers the same crate on Ubuntu. Local macOS compilation/tests are additive evidence, never a substitute for native Windows runtime evidence.

## Implementation boundary

At dependency-gate time `crates/kernux-store/src/lib.rs` contains no Store type, SQL, migration, path handling, UUID validation, persistence API, CAS logic, policy logic, or secret storage. It is intentionally only a crate shell with `#![forbid(unsafe_code)]`.

The following remain blocked until exact protected merge plus post-merge qualification succeeds:

- SQLite open/pragmas;
- schema/migration implementation;
- UUIDv7 persistence validation;
- revision compare-and-swap;
- immutable metadata APIs;
- Artifact metadata rules;
- Grant/reference persistence;
- Event stream append;
- corruption/recovery behavior.

## Qualification requirements

This candidate must pass:

- `python3 tools/dependencies/validate.py check`;
- exact feature-minimality assertions;
- `cargo tree --locked -d` review;
- `git diff --check`;
- full `pnpm check`;
- Diffcipline R3;
- Alibaba OpenCodeReview Delegation Mode plus complete manual review of excluded files;
- all five protected exact-head jobs plus additive `Windows kernuxd`;
- exact-head review with zero unresolved threads;
- ordinary protected merge;
- exact post-merge six-job qualification.

Only then may the SG-000021 schema/migration/WAL implementation slice begin.
