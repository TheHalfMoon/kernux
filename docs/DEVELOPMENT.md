# Development baseline

This document describes the repository baseline established by `KX-P00-S01-T01`. It is intentionally narrower than the future CI and verification system.

## Toolchain

- Node.js: `>=24.0.0 <27`
- pnpm: `12.4.2` (pinned in `package.json`)
- Rust: `1.98.1` (pinned in `rust-toolchain.toml`)
- Rust edition: `2024`

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

At T01 this validates the workspace contract only. It does **not** claim that formatting, linting, static analysis, unit tests, security tests, or CI are already configured. Those gates begin in subsequent P00 tasks and must not be represented as passing before they exist.

## Formatting baseline

- `.editorconfig` is the editor-neutral whitespace baseline.
- `.gitattributes` normalizes repository text to LF while preserving Windows batch-file CRLF requirements.
- `rustfmt.toml` defines the initial Rust formatter behavior.
- JavaScript/TypeScript formatter/linter dependencies are intentionally deferred to the baseline-CI task so their executable versions and CI gates are introduced together.

## Dependency discipline

- Root dependency versions are exact by default through `.npmrc`.
- Do not add a dependency merely to implement repository scaffolding.
- New dependencies must have a bounded purpose and follow the later supply-chain/provenance gates as they become active.

## Current boundary

This baseline contains no product application, no `kernuxd` implementation, and no founding-donor code. The canonical next-state authority remains `specs/CURRENT.md`.
