# P02-S02-T01 dependency-admission gate

## Scope

This evidence records the dependency-governance prerequisite for `KX-P02-S02-T01` / `SG-000020`.

It authorizes only the exact direct Rust dependencies required for the bounded local identity/key-lifecycle implementation. It does not prove installation identity semantics, key generation, rotation, recovery, session binding, persistence, credential-store integration, policy/Grant authority, remote enrollment, P02 phase completion, or release qualification.

The `kernux-identity` crate in this dependency slice is intentionally a compile-only shell with no identity semantics. SG-000020 implementation remains blocked until this dependency slice merges and its exact post-merge qualification succeeds.

## Pre-admission review

Before repository admission, the exact crates.io packages and APIs were inspected:

| Package | Exact version | Observed license | Rust version | Intended direct use |
| --- | --- | --- | --- | --- |

| `ed25519-dalek` | `3.0.0` | `BSD-3-Clause` | `1.85` | Ed25519 signing/verifying primitive for bounded local identity key lifecycle and identity-session signatures. |

| `getrandom` | `0.4.3` | `MIT OR Apache-2.0` | `1.85` | Operating-system randomness for installation IDs, instance IDs, and 32-byte Ed25519 seeds. |

| `zeroize` | `1.9.0` | `Apache-2.0 OR MIT` | `1.85` | Explicit zeroization for recovery-seed wrappers and secret-key handling. |

The selected versions are compatible with the repository Rust `1.98` toolchain.

Observed source repositories:

- `ed25519-dalek 3.0.0`: `https://github.com/dalek-cryptography/curve25519-dalek/tree/main/ed25519-dalek`;
- `getrandom 0.4.3`: `https://github.com/rust-random/getrandom`;
- `zeroize 1.9.0`: `https://github.com/RustCrypto/utils`.

## Exact direct dependency policy

`crates/kernux-identity/Cargo.toml` declares exactly:

```toml
ed25519-dalek = { version = "=3.0.0", default-features = false, features = ["zeroize"] }
getrandom = { version = "=0.4.3", default-features = false }
zeroize = { version = "=1.9.0", default-features = false }
```

This intentionally avoids:

- `ed25519-dalek` default `fast` precomputed-table feature;
- `rand_core` key-generation integration;
- PEM/PKCS#8 support;
- serde support for key material;
- batch/hazmat/legacy-compatibility features;
- a general-purpose RNG framework;
- storage, credential-provider, networking, or policy dependencies.

The production identity implementation will call the narrow `getrandom::fill` OS-random API directly, construct `SigningKey` from exact secret bytes, and use explicit zeroization where SG-000020 requires it. Those semantics are not implemented in this admission slice.

## Bounded license-expression review

The canonical license policy automatically admits only the exact single identifiers `MIT` and `Apache-2.0`. Every selected direct dependency therefore requires separately governed exception evidence:

- `ed25519-dalek 3.0.0` reports `BSD-3-Clause`;
- `getrandom 0.4.3` reports `MIT OR Apache-2.0`;
- `zeroize 1.9.0` reports `Apache-2.0 OR MIT`.

For this exact package/version set, the expressions are permissive or offer a permissive choice compatible with the Apache-2.0 Kernux project posture. The dependency registry records all three as `exception-reviewed` and binds this file as the policy-exception evidence path.

This is not a global license allowlist expansion. It does not approve another package, version, feature set, or SPDX expression merely because it is similar. Release-time notice/SBOM obligations remain governed by later release gates; this dependency gate proves only current direct-dependency admission.

## Fail-closed sequencing

The first repository-local validation attempt occurred after manifest/registry declaration but before this exception-evidence path existed. It failed closed exactly as required:

`Dependency admission: FAIL — cargo:ed25519-dalek exception path is not canonical evidence: docs/evidence/P02-S02-T01-DEPENDENCY-GATE.md`

That failure remains failure evidence and is not relabeled as PASS. No identity implementation code existed during that failed attempt.

## Locked graph review

Repository-local metadata resolves the exact direct packages and licenses above. `cargo tree --locked -d` reports no duplicate packages in the host-resolved graph. The exact `kernux-identity` feature tree enables only the requested direct features; transitive crates may enable their own required default features.

New locked packages introduced by this gate are:

| Package | Version | Observed license |
| --- | --- | --- |

| `block-buffer` | `0.12.1` | `MIT OR Apache-2.0` |

| `cfg-if` | `1.0.5` | `MIT OR Apache-2.0` |

| `cpufeatures` | `0.3.1` | `MIT OR Apache-2.0` |

| `crypto-common` | `0.2.2` | `MIT OR Apache-2.0` |

| `curve25519-dalek` | `5.0.0` | `BSD-3-Clause` |

| `curve25519-dalek-derive` | `0.1.1` | `MIT/Apache-2.0` |

| `digest` | `0.11.3` | `MIT OR Apache-2.0` |

| `ed25519` | `3.0.0` | `Apache-2.0 OR MIT` |

| `ed25519-dalek` | `3.0.0` | `BSD-3-Clause` |

| `fiat-crypto` | `0.3.0` | `MIT OR Apache-2.0 OR BSD-1-Clause` |

| `getrandom` | `0.4.3` | `MIT OR Apache-2.0` |

| `hybrid-array` | `0.4.15` | `MIT OR Apache-2.0` |

| `kernux-identity` | `0.0.0` | `Apache-2.0` |

| `r-efi` | `6.0.0` | `MIT OR Apache-2.0 OR LGPL-2.1-or-later` |

| `rustc_version` | `0.4.1` | `MIT OR Apache-2.0` |

| `semver` | `1.0.28` | `MIT OR Apache-2.0` |

| `sha2` | `0.11.0` | `MIT OR Apache-2.0` |

| `signature` | `3.0.0` | `Apache-2.0 OR MIT` |

| `subtle` | `2.6.1` | `BSD-3-Clause` |

| `syn` | `2.0.119` | `MIT OR Apache-2.0` |

| `typenum` | `1.20.1` | `MIT OR Apache-2.0` |

| `zeroize` | `1.9.0` | `Apache-2.0 OR MIT` |


New locked package count: `22`.

`Cargo.lock` SHA-256: `373b61386f33525e05f1c2c6cd4963edc8747b029e2569bf4b08f6e7fcb26d3c`.

This transitive table is evidence for the exact current lockfile only; it is not a permanent allowlist, vulnerability assessment, or release SBOM.

## CI boundary

The existing protected contexts are unchanged. The additive `Windows kernuxd` job keeps its exact job name and now also runs `kernux-identity` Clippy/tests natively on `windows-latest`. The normal Rust workspace job covers the new workspace crate on Ubuntu. Local macOS compilation/checks are additional evidence, not substitutes for Windows runtime qualification.

## Qualification boundary

This exact dependency candidate must pass:

- `python3 tools/dependencies/validate.py check`;
- `git diff --check`;
- full `pnpm check`;
- Diffcipline R3;
- Alibaba OpenCodeReview Delegation Mode plus complete manual accounting for exclusions;
- all five protected exact-head checks plus additive `Windows kernuxd`;
- exact-head review with zero unresolved threads;
- ordinary protected merge and exact six-job post-merge qualification.

No identity/key-lifecycle implementation is authorized until this gate merges and its post-merge qualification succeeds.
