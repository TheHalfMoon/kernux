# Kernux Project License Policy

**Policy version:** 1
**Project-owned code license:** Apache-2.0
**Canonical license file:** `/LICENSE`

## Purpose

This policy defines the repository boundary between Kernux-owned material and third-party material. It is an engineering governance control for source admission, provenance, attribution, and release preparation. It is not a substitute for legal review when terms are ambiguous or materially different from the classes already proven here.

## Kernux-owned code

Unless a file or subtree carries a different explicit notice, code authored for and owned by the Kernux project is licensed under the Apache License, Version 2.0.

The canonical license text is the repository-root `LICENSE` file.

The current root npm manifest is private and the Cargo root is a virtual workspace. Neither root is a distributable package surface, so this P00 task intentionally does not add publish-license metadata to those roots.

Every future distributable npm package or Rust crate must declare an explicit SPDX license before release. Kernux-owned distributable code defaults to `Apache-2.0` unless a separately governed exception proves another license is required.

The selected project license is permissive and includes an explicit patent grant and patent-termination terms. The repository validator binds the exact license bytes, canonical policy identity, and private/virtual root boundary so accidental drift is visible.

## Third-party boundary

The root Apache-2.0 license does **not** relicense third-party material.

Third-party source, dependencies, license snapshots, assets, model/data artifacts, fonts, icons, binaries, trademarks, hosted services, and other externally governed material retain their own applicable terms. Their provenance and obligations must be preserved independently.

In particular:

- material under `third_party/` remains governed by its recorded upstream evidence;
- a donor source import must preserve the donor license and required notices;
- founder authorization to reuse source does not erase embedded third-party obligations;
- an SPDX label alone does not settle asset, trademark, model/data, service, or custom-term rights;
- generated or adapted code must still retain source provenance when the donor terms require attribution or notice preservation.

## Current proven source/license posture

The following is the bounded evidence posture used for the P00 project-license decision. It is not a permanent allowlist of every possible future dependency.

| Source / package | Pinned or observed version | Observed license | Current posture |
| --- | --- | --- | --- |
| `stablyai/orca` | `0d23ea6e688410c878096dab8b1779857354b7d4` | MIT | Permissive; preserve upstream notice on reuse |
| `tinyfish-io/agentql` | `418ba8ad1c69dfac134a6833369a01dfba5a24a7` | MIT | Permissive; preserve upstream notice on reuse |
| `wonderwhy-er/DesktopCommanderMCP` | `08ff76192919a6e8fe2557b39c3f99e7b54c3b92` | MIT | Permissive; preserve upstream notice on reuse |
| `oxfmt` | `0.68.0` | MIT | Development tooling; permissive |
| `oxlint` | `1.83.0` | MIT | Development tooling; permissive |
| `tinypool` | `2.1.2` | MIT | Installed support dependency; permissive |
| `typescript` | `7.0.2` | Apache-2.0 | Development tooling; same SPDX family as project license |
| installed Oxc platform bindings | current lock/install resolution | MIT | Platform support packages; permissive |
| installed TypeScript platform package | current lock/install resolution | Apache-2.0 | Platform support package |
| Rust workspace | current `Cargo.lock` | no external package dependency | No third-party Rust package obligation yet |

The public founding-donor identities and preserved upstream license snapshots remain authoritative in `third_party/notices/inventory.json`.

The JavaScript rows above are a bounded observation from the current lock/install state. Platform-specific optional packages may differ by operating system. Release qualification must regenerate a complete distribution-facing inventory rather than treating this planning snapshot as a permanent SBOM.

## Admission policy

For repository-owned code and currently planned donor reuse:

- **MIT:** permitted subject to preserving copyright/license notices and provenance.
- **Apache-2.0:** permitted subject to preserving applicable license/NOTICE obligations and provenance.
- **Any other SPDX expression or license class:** review required before admission to a distributed Kernux artifact.

The review-required rule includes, without limitation:

- BSD, ISC, or other permissive licenses not yet recorded by policy;
- MPL, EPL, LGPL, or other weak-copyleft terms;
- GPL or other copyleft terms;
- AGPL or other network-copyleft terms;
- proprietary, source-available, custom, dual-license, or non-commercial terms;
- missing, unknown, ambiguous, or non-SPDX declarations.

This conservative rule is intentional. A future policy change may add a class only with explicit evidence and exact repository review.

## Non-code material

Do not infer rights for non-code material from the project code license.

Separate review is required for:

- model weights and model licenses;
- datasets and data-use restrictions;
- fonts, images, icons, audio, video, and design assets;
- trademarks, logos, names, and trade dress;
- application-store assets and platform SDK terms;
- hosted APIs, accounts, service-specific terms, and remote backends;
- binary redistributables and native SDKs.

## Donor import rule

Before donor code enters Kernux, the active canonical task must satisfy `docs/canonical/DONOR_IMPORT_CHECKLIST.md` and the machine-readable provenance/notice gates.

A donor import must record:

1. exact source identity and immutable revision;
2. authorization basis;
3. upstream license evidence;
4. dependency/asset/service review;
5. source-to-destination mappings and transformation class;
6. characterization evidence before semantic adaptation;
7. applicable security review;
8. mechanical import and adaptation history;
9. exact-head and post-merge qualification.

## Validation contract

Run:

```bash
python3 tools/provenance/validate_project_license.py check
```

Validation is repository-local and network-independent. It fails closed if:

- the root `LICENSE` is missing, symlinked, or differs from the canonical Apache-2.0 bytes;
- the root npm manifest stops being private or gains publish-license metadata without a governed policy update;
- the Cargo root stops being virtual or gains inherited publish-license metadata without a governed policy update;
- this canonical policy document drifts without updating its pinned policy identity;
- required third-party boundary language disappears.

Any intentional project-license or policy change is governed work and must update its evidence, validator contract, and compatibility review together.

## Release boundary

This P00 policy establishes the project-license decision and source-admission posture only.

It does not itself generate or prove:

- a release SBOM;
- a release NOTICE bundle;
- installer/package attribution completeness;
- every transitive production dependency obligation;
- app-store or platform distribution compliance;
- trademark clearance;
- compatibility with future licenses not present in the proven current inventory.

Those remain release/supply-chain qualification work.
