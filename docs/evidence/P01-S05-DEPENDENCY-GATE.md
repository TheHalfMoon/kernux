# P01-S05 dependency-admission gate and first Rust dependency review

## Scope

This evidence records the bounded governance prerequisite discovered during `KX-P01-S05-T01` and the license/dependency review for the first external Rust packages used by the generated contract crate.

It does not change the Kernux project license, create a general dependency allowlist, establish vulnerability/SBOM/release qualification, or approve unrelated future packages.

## Governance deadlock that required repair

The original Rust-generation candidate was PR #54 at exact head:

`8049415463e1626ad8e6b5a6c8f7ea393ffa50f1`

Workflow run:

`35410413890`

JavaScript / TypeScript, Rust, SpecGrain, and Provenance succeeded. Required `Diffcipline R1` failed because the then-canonical policy classified every dependency-manifest and lockfile change as `REVIEW`; Diffcipline v1 preserves `REVIEW` as exit 1 and has no approval hook. Protected `main` had `enforce_admins=true`, so the legitimate change could not merge without bypassing governance.

The failure was preserved. PR #54 was never merged or relabeled as PASS.

## SG-000013 refinement identity

Spec:

`SG-000013 — KX-P01-S05-T01 prerequisite — Replace unmergeable dependency REVIEW with enforced dependency admission`

Spec revision:

`sha256:ad439f9bf15b318df7dbab23fab29d5e3f979345b76b15904c0a4b862572e53f`

WorkPacket:

`sha256:24e46f3454f199cde77e33f131a083f7cfef8e5bc6ecd0dabcc581b85f4bf510`

Context plan:

`sha256:606e9a4e0bb02720f944e7bde2f90634dc7797a4b0b6b1d79bfd2d4343d1dde7`

## Proven gate construction

| Slice | PR | Exact head | Merge | PR CI | Post-merge CI |
| --- | --- | --- | --- | --- | --- |
| Refinement | #55 | `8a693a84c63a810f05ab3ab7c6cb60ebb66c58b9` | `3f0d3aa01ac58669e6dfeade56392f6ce9996421` | `35410717575` SUCCESS | `35410773350` SUCCESS |
| Validator core | #56 | `8fc70faa39ae4dff7f584a29ce5156c3a911ed5b` | `0d3b48fb443d98b7dd42bb037769e46f5fe7bca2` | `35410944987` SUCCESS | `35410994555` SUCCESS |
| Required Provenance integration | #57 | `4705bea536267c21698b64f94a01db21c3923096` | `695e7f5b03f0fa2dc54db806ada155674fe0778f` | `35411096147` SUCCESS | `35411135338` SUCCESS |
| Diffcipline policy transfer | #58 | `1af0a7f20cfad87ee011a74f1a2271ae1dd3440f` | `944755f7c4a36bed25a2b5708c0970e5054aad0e` | `35411950111` SUCCESS | `35412372896` SUCCESS |
| Admission hardening | #59 | `8c3b5ac9e3740ae3d3b2e14b71490d3cf7dab086` | `d309f7f4328507415d80037645f6d595ed78d369` | `35412467936` SUCCESS | `35412537556` SUCCESS |

At the policy-transfer boundary, branch protection remained:

- JavaScript / TypeScript — required;
- Rust — required;
- SpecGrain — required;
- Diffcipline R1 — required;
- Provenance — required;
- `enforce_admins=true`.

Dependency judgment moved into the already-required Provenance job before Diffcipline dependency/lockfile observations changed from REVIEW to ALLOW. Size, scope, untracked-file, risk-profile, and verification enforcement remained in Diffcipline.

## Dependency validator posture

The required dependency gate now fails closed on at least:

- unapproved direct npm/Cargo dependency;
- floating/range direct versions;
- approval/manifest source, exact-version, scope, or source-URL mismatch;
- stale, duplicate, malformed, or unsupported approvals;
- Cargo git dependencies without exact URL plus full commit identity;
- Cargo locked-resolution mismatch;
- direct-package license mismatch;
- license expressions outside the exact canonical automatic posture unless separately governed exception evidence is named.

The conformance suite contains 19 focused dependency tests after the admission-hardening slice.

## First Rust direct dependencies

The `kernux-contracts` crate is non-publishable and introduces only these direct external packages:

| Package | Version | Scope | Observed license | Admission |
| --- | --- | --- | --- | --- |
| `serde` | `1.0.229` | runtime | `MIT OR Apache-2.0` | bounded exception review in this evidence |
| `serde_json` | `1.0.151` | dev/test only | `MIT OR Apache-2.0` | bounded exception review in this evidence |

The versions are exact in the manifest and registry resolution is locked.

## Bounded license-expression review

Canonical `LICENSE_POLICY.md` automatically admits exact MIT and exact Apache-2.0 classes. It requires review for other SPDX expressions/classes rather than silently inferring compatibility.

The direct packages above report `MIT OR Apache-2.0`. For this exact dependency graph, Kernux records a bounded repository-policy exception allowing those two direct packages at the exact versions above. This does not add `MIT OR Apache-2.0` to the global automatic allowlist and does not authorize a future package or version merely because it reports the same expression.

The exact locked graph also reports:

| Locked package | Version | Observed license expression |
| --- | --- | --- |
| `itoa` | `1.0.18` | `MIT OR Apache-2.0` |
| `memchr` | `2.8.3` | `Unlicense OR MIT` |
| `proc-macro2` | `1.0.107` | `MIT OR Apache-2.0` |
| `quote` | `1.0.47` | `MIT OR Apache-2.0` |
| `serde` | `1.0.229` | `MIT OR Apache-2.0` |
| `serde_core` | `1.0.229` | `MIT OR Apache-2.0` |
| `serde_derive` | `1.0.229` | `MIT OR Apache-2.0` |
| `serde_json` | `1.0.151` | `MIT OR Apache-2.0` |
| `syn` | `3.0.6` | `MIT OR Apache-2.0` |
| `unicode-ident` | `1.0.26` | `(MIT OR Apache-2.0) AND Unicode-3.0` |
| `zmij` | `1.0.23` | `MIT` |

This table is evidence for the exact lockfile, not a global policy expansion. Release-time SBOM/notices and final transitive obligation qualification remain owned by later release governance.

Exact `Cargo.lock` SHA-256 for this reviewed graph:

`c537ddbf17e2aa9d1188926d87686e75505c164df6d23d9b741ec106946524e8`

`cargo tree --locked -d` reports no duplicate packages.

## Dependency purpose and minimality

`serde` is required to derive serialization/deserialization for generated Rust wire contracts from the single authoritative JSON Schema source.

`serde_json` is dev-only and required for the SG-000012 acceptance criterion that Rust consumes the same tracked JSON fixture bytes as TypeScript/Node and proves semantic JSON round-trip equality.

No larger runtime framework, schema generator crate, or validation framework is introduced.

## Completion boundary

This evidence authorizes only the exact dependency/crate-shell candidate described above once its own protected exact-head and post-merge evidence succeed.

`SG-000013` is not called natively PROVEN by this document alone. Its final verification record must bind the exact dependency-admission implementation/result and the successful first real dependency change.
