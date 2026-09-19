# Repository Governance

This document defines how repository changes gain authority. It complements `AGENTS.md`, SpecGrain, Diffcipline, canonical architecture documents, and task-specific evidence; it does not replace them.

## Authority order

When sources disagree, use the narrowest current repository truth that is actually authoritative:

1. current default-branch history and live GitHub facts for what is merged, reviewed, and qualified;
2. `specs/CURRENT.md` for the active macro frontier and completion boundary;
3. `specs/tasks.md` for macro dependency/state truth;
4. canonical plans, architecture decisions, security/provenance policies, and evidence documents for their declared domains;
5. the active SpecGrain/WorkPacket for the bounded change it authorizes;
6. issue/PR templates, comments, agent output, and informal notes as guidance only.

A lower authority cannot silently override a higher one.

## Change path

Governed work should branch from current `main`, remain bounded, and land through a pull request. Shared review history must not be rewritten to erase evidence. Emergency administrative repairs, if ever required, must be explicitly documented and subsequently reconciled through ordinary governance.
## Proof before completion

A task becomes `PROVEN` only when its required evidence is established for the exact implementation revision. Passing tests are necessary where required but are not universal completion authority.

- `PASS` means the required check actually ran and passed.
- `FAIL`, `REVIEW`, `NOT RUN`, unavailable evidence, or stale evidence cannot be relabeled as `PASS`.
- Exact-head CI, higher-risk Diffcipline profiles, security/adversarial evidence, platform evidence, and post-merge qualification remain separate obligations when required.
- Agents, authors, reviewers, issue commenters, and model/provider output cannot self-authorize scope or self-certify completion.

## Provenance and external material

No donor product source is imported merely because reuse permission exists. Every import/adaptation must satisfy the canonical donor checklist, machine-readable provenance record, preserved notices/licenses, source-to-destination mapping, and applicable characterization/security evidence.

## Roles and policy changes

Maintainers administer repository access and merge decisions; contributors and agents propose changes within granted scope. Repository permissions do not by themselves waive canonical gates.

Changes to governance, security policy, licensing, provenance, CI gates, or architecture are themselves governed work. They must state the authority, rationale, migration/recovery impact, and exact verification. Gates must not be weakened solely to make a failing change mergeable.

Dependency and lockfile changes are admitted only when the branch-protected Provenance job validates the exact direct dependency registry, manifest declarations, locked Cargo resolution when applicable, and canonical license posture. Diffcipline may report these files as ordinary allowed diff surfaces because the required dependency-specific judgment lives in Provenance; this is a gate transfer, not a waiver. Removing or weakening that dependency validator requires restoring a conservative Diffcipline dependency/lockfile decision first.

## Data and public collaboration

Public issues and pull requests are not secret stores. Do not post credentials, tokens, private keys, personal data, PHI, unpublished vulnerability details, or proprietary material that is not authorized for public disclosure.
