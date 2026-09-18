# Contributing to Kernux

Kernux is an evidence-first, local-first agent operating environment. Contributions are welcome when they preserve the repository's canonical architecture, security boundaries, and proof requirements.

## Before you change code

- Read `AGENTS.md`, `specs/CURRENT.md`, `specs/tasks.md`, and the relevant canonical documents.
- Confirm that the work is inside the currently authorized frontier and that declared dependencies are proven.
- Read `docs/DEVELOPMENT.md` for the supported local toolchain and baseline checks.
- For security-sensitive findings, follow `SECURITY.md`; do not disclose unpublished vulnerability details in a public issue.
- For copied, adapted, generated-from, or otherwise externally derived material, follow the donor provenance and notice rules before importing source.

## Local setup

Use the repository-pinned toolchain and lock state:

```bash
pnpm install --frozen-lockfile
pnpm check
```

A green local check is evidence, not permission to bypass task-specific gates.
## Change discipline

- Branch from current `main`; keep the change bounded to one authorized outcome.
- Do not force-push or rewrite shared history used for review/evidence.
- Keep unrelated cleanup out of the same pull request.
- Preserve cross-platform behavior explicitly when filesystem, process, PTY, Git, browser, or runtime code changes.
- Never weaken tests, policy, provenance, or security gates merely to make a change pass.
- Never invent CI, test, review, runtime, provider, or release evidence.

## Pull requests

Use the repository pull-request template. A reviewable PR explains scope, authority, risk, exact verification, platform impact, data/security impact, and provenance/license impact. Completion is established by repository-owned evidence bound to the exact candidate revision; an author or agent saying "done" is not proof.

External source reuse must satisfy `docs/canonical/DONOR_IMPORT_CHECKLIST.md`, the machine-readable provenance contract, and third-party notice requirements in the same governed change or a mechanically linked import change.

Contribution licensing is governed by the repository `LICENSE` and `docs/canonical/LICENSE_POLICY.md`. Do not introduce additional contribution terms, a CLA/DCO requirement, or a conflicting license without an explicit governed policy change.

By participating, follow `CODE_OF_CONDUCT.md`. For help choosing the right channel, see `SUPPORT.md`.
