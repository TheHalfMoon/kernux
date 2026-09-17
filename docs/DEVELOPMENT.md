# Development baseline

This document describes the repository baseline established by `KX-P00-S01-T01`. It is intentionally narrower than the future CI and verification system.

## Toolchain

- Node.js: `>=24.0.0 <27`
- pnpm: `12.4.2` (pinned in `package.json`)
- Rust: `1.98.1` (pinned in `rust-toolchain.toml`)
- Rust edition: `2024`
- Python: `>=3.11` for repository governance tooling only

The Rust pin uses 1.98.1 rather than 1.98.0 because 1.98.1 is the current stable point release at baseline creation and fixes a compiler miscompilation present in 1.98.0.

## Workspace map

```text
apps/                   user-facing applications
packages/               shared TypeScript/JavaScript packages
crates/                 Rust crates
tools/                  repository-local deterministic tooling
third_party/            third-party boundaries/notices/provenance
docs/canonical/         product and architecture authority
specs/                  execution frontier and task registry
```

The directories are intentionally skeletal. Creating a directory does not authorize work from a later phase.

## Root command

Run the deterministic repository-baseline check:

```bash
pnpm check
```

The root command now runs the baseline workspace, JavaScript/TypeScript, Rust, and pinned SpecGrain checks. Security-specific and higher-risk verification profiles are introduced by later P00 tasks and must not be represented as passing before they exist.

Baseline CI currently proves:

- the repository workspace contract;
- Oxfmt formatting;
- Oxlint linting;
- strict TypeScript compilation;
- Node smoke tests;
- Rustfmt formatting;
- Clippy with warnings denied;
- Rust smoke tests;
- pinned SpecGrain store validation.

GitHub Actions installs pnpm dependencies from the committed lockfile with --frozen-lockfile and keys the pnpm cache from that lockfile. Rust currently has no external crates, so a separate dependency cache would add complexity without measurable value; revisit this when the Rust dependency graph becomes non-empty. Python is not a Kernux product runtime dependency; it is required only for pinned repository-governance tooling such as SpecGrain.

## Formatting baseline

- `.editorconfig` is the editor-neutral whitespace baseline.
- `.gitattributes` normalizes repository text to LF while preserving Windows batch-file CRLF requirements.
- `rustfmt.toml` defines the initial Rust formatter behavior.
- JavaScript/TypeScript formatting and linting are pinned through the root lockfile and run in CI.

## Dependency discipline

- Root dependency versions are exact by default through `.npmrc`.
- Do not add a dependency merely to implement repository scaffolding.
- New dependencies must have a bounded purpose and follow the later supply-chain/provenance gates as they become active.

## Current boundary

This baseline contains no product application, no `kernuxd` implementation, and no founding-donor code. The canonical next-state authority remains `specs/CURRENT.md`.
