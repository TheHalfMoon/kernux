# P02-S01-T01 dependency-admission gate

## Scope

This evidence records the dependency-governance prerequisite for `KX-P02-S01-T01` / `SG-000018`.

It authorizes only the exact direct Rust dependencies required for the first `kernuxd` slice. It does not prove daemon lifecycle, IPC runtime behavior, caller authentication, privileged operations, persistence, P02 phase completion, or release qualification.

## Pre-admission review

Before the repository manifest used `interprocess`, an isolated temporary Cargo project resolved the exact registry package:

- package: `interprocess`;
- version: `2.4.4`;
- observed license: `0BSD OR Apache-2.0`;
- upstream repository: `https://github.com/kotauskas/interprocess`;
- default feature set: empty;
- optional async features: `async` and `tokio`;
- Kernux admission: synchronous use only, with `default-features = false`.

This review was performed outside the Kernux repository so dependency metadata could be inspected before repository admission.

## Exact direct dependencies

| Package | Exact version | Aggregate scope | Observed license | Purpose |
| --- | --- | --- | --- | --- |
| `interprocess` | `2.4.4` | runtime | `0BSD OR Apache-2.0` | Synchronous cross-platform local IPC primitives for Unix-domain sockets and Windows local named pipes, with default features disabled. |
| `serde_json` | `1.0.151` | dev+runtime | `MIT OR Apache-2.0` | Existing shared JSON conformance fixtures plus bounded `kernuxd` local control-frame encoding/decoding. |

`serde_json` remains at the already qualified exact version. Only its aggregate approved scope and purpose expand from dev-only to dev+runtime.

## Bounded license-expression review

The canonical license policy automatically admits exact MIT and exact Apache-2.0 identifiers. Compound or otherwise unlisted expressions require separately governed exception evidence.

For this exact dependency gate:

- `interprocess 2.4.4` reports `0BSD OR Apache-2.0`;
- `serde_json 1.0.151` reports `MIT OR Apache-2.0`.

Both direct dependencies are therefore recorded as `exception-reviewed` and bind this evidence file as their policy-exception path.

This is not a global allowlist expansion. It does not approve another package or version merely because it reports the same SPDX expression.

## Minimality

The first daemon slice deliberately does not add Tokio, an HTTP stack, a TCP/UDP transport, a database, an authentication framework, or another serialization framework.

The repository uses:

- one synchronous local-IPC dependency: `interprocess = { version = "=2.4.4", default-features = false }`;
- the existing exact `serde_json = "=1.0.151"` package for bounded JSON control framing.

The `kernuxd` crate shell in this gate contains no daemon lifecycle or IPC implementation. Runtime implementation remains blocked until this dependency gate qualifies and merges.

## Fail-closed sequencing

The first repository-local dependency validation attempt intentionally failed because the newly declared exception path did not yet exist:

`Dependency admission: FAIL — cargo:interprocess exception path is not canonical evidence: docs/evidence/P02-S01-T01-DEPENDENCY-GATE.md`

That failure is preserved as evidence that the validator did not silently admit the dependency before review evidence existed.

## Locked graph review

Repository-local validation now observes:

- `python3 tools/dependencies/validate.py check` — PASS, 6 direct external dependencies across npm and Cargo;
- `interprocess 2.4.4` — exact registry resolution, license `0BSD OR Apache-2.0`;
- `serde_json 1.0.151` — exact registry resolution, license `MIT OR Apache-2.0`;
- `cargo tree --locked -d` — no duplicate packages;
- `interprocess` features `async` and `tokio` — disabled;
- `Cargo.lock` SHA-256 — `d7e50d246cc3091cd2833f7f685fa8277e5f894216d255eb54d2276f88d0dffb`.

New locked packages introduced by this gate are:

| Package | Version | Observed license |
| --- | --- | --- |
| `doctest-file` | `1.1.1` | `0BSD` |
| `interprocess` | `2.4.4` | `0BSD OR Apache-2.0` |
| `libc` | `0.2.189` | `MIT OR Apache-2.0` |
| `recvmsg` | `1.0.0` | `0BSD` |
| `widestring` | `1.2.1` | `MIT OR Apache-2.0` |
| `windows-link` | `0.2.1` | `MIT OR Apache-2.0` |
| `windows-sys` | `0.61.2` | `MIT OR Apache-2.0` |

The project-owned `kernuxd 0.0.0` workspace crate is Apache-2.0 and non-publishable. This transitive table is evidence for the exact current lockfile only; it is not a permanent allowlist or release SBOM.

## Qualification boundary

The dependency-admission semantics are now locally proven. This exact candidate must still pass:

- full `pnpm check`;
- Diffcipline R3 on the committed range;
- OpenCodeReview-assisted exact-range review with manual accounting for excluded files;
- the five protected exact-head checks;
- protected merge and five-job post-merge qualification.

No `kernuxd` lifecycle/IPC implementation is authorized until this gate merges and its post-merge qualification succeeds.
