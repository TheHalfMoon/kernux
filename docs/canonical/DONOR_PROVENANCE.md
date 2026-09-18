# Donor Provenance

## Purpose

Kernux is intentionally built with permission to reuse substantial source from founding donor systems. That is an advantage only if provenance stays explicit. This document records the founding sources, the revisions inspected during planning, the intended reuse boundaries, and the import rules that prevent donor internals from accidentally becoming Kernux architecture.

This file remains narrative authority for donor intent; machine-readable import records are governed by the tracked v1 contract under `third_party/provenance/`.

## Machine-readable import gate

The v1 contract is `third_party/provenance/schema/import-manifest.schema.json`; donor records live under `third_party/provenance/manifests/` and must pass `python3 tools/provenance/validate.py check` before import. The validator is dependency-free, rejects ambiguous/unsafe records, and never executes manifest content. An empty manifest set is valid only while no donor source has entered Kernux.

## Founding donor 1 — Orca

- Repository: `stablyai/orca`
- Planning revision inspected: `0d23ea6e688410c878096dab8b1779857354b7d4`
- License observed at that revision: MIT
- Upstream license holder text observed: Lovecast Inc.
- Role in Kernux: **workspace/runtime donor**

High-value areas to characterize and potentially reuse:

- Electron desktop shell patterns;
- pane/dock/split workspace behavior;
- terminal/PTTY integration and persistence;
- agent session lifecycle and CLI-agent adapters;
- worktree isolation and git workflows;
- remote runtime and SSH patterns;
- mobile companion patterns;
- agent status projection;
- account/usage UX;
- browser integration and design-mode interaction patterns;
- cross-platform hardening, especially Windows/WSL/SSH behavior;
- test infrastructure and runtime compatibility lessons.

Do **not** copy the architectural assumption that Orca is the Kernux public contract. Kernux must place reused implementation behind its own runtime, capability, event, policy, and adapter boundaries.

Planning observations worth preserving:

- execution-host truth should own process/agent status;
- contact loss and execution death are different states;
- remote wire compatibility must tolerate mixed client/host versions;
- git capability differences across local, WSL, and SSH hosts need explicit detection/fallback;
- cross-platform process spawning is security/reliability sensitive;
- UI validation should avoid stealing user focus;
- workspaces must not assume every project is a git worktree.

## Founding donor 2 — TinyFish / AgentQL

### Public reference inspected

- Repository: `tinyfish-io/agentql`
- Planning revision inspected: `418ba8ad1c69dfac134a6833369a01dfba5a24a7`
- License observed in repository: MIT
- Upstream license holder text observed: Tiny Fish, Inc.
- Role in Kernux: **semantic web/runtime donor and reference**

High-value areas:

- natural-language element/data selection;
- structured extraction;
- resilient targeting across changing page structure;
- Playwright integration;
- authenticated-session workflows;
- browser automation examples and semantic query patterns;
- debugger/developer ergonomics around web-agent queries.

### Founder-authorized TinyFish source

The founder states that Kernux has permission to use the full TinyFish source code as desired. The exact source repository/artifact and revision of any non-public component must be entered into the provenance ledger **before import**. Do not invent a public revision for source that is not independently visible here.

Required import record fields:

- source owner/repository or artifact identity;
- exact commit/release/content digest;
- permission basis/reference supplied by founder;
- source paths;
- destination paths;
- transformation class;
- inherited dependency/license notices;
- characterization tests;
- import commit SHA.

## Founding donor 3 — Desktop Commander MCP

- Repository: `wonderwhy-er/DesktopCommanderMCP`
- Planning revision inspected: `08ff76192919a6e8fe2557b39c3f99e7b54c3b92`
- License observed at repository: MIT
- Role in Kernux: **computer/host capability donor**

High-value areas:

- filesystem read/write/search primitives;
- terminal command execution and streaming;
- long-running process/session management;
- process listing/termination;
- Python/Node/R in-memory execution patterns;
- document/data support for Excel, PDF, DOCX;
- MCP exposure patterns;
- remote-device patterns;
- local audit history;
- configuration and bounded-output patterns;
- path/symlink guardrails;
- Docker isolation option.

Important constraint: upstream explicitly describes guardrails as **not a sandbox**. Kernux must not inherit an MCP server's local permission model as the system security boundary. Privileged operations go through `kernuxd` and the Kernux capability policy.

## Methodology sources — not product donors by default

### SpecGrain

- Repository: `TheHalfMoon/SpecGrain`
- Role: delivery control methodology/tooling
- Key concepts adopted: recursive refinement, Grains, WorkPackets, exact context/provenance, independent evidence.

### Diffcipline

- Repository: `TheHalfMoon/Diffcipline`
- Role: proof-before-done methodology/tooling
- Key concepts adopted: exact diff, explicit risk class, executable verification, PASS/REVIEW/FAIL semantics, immutable evidence expectations.

Code from these projects should only enter Kernux product code if a future spec explicitly requires it. Their immediate role is repository governance and execution discipline.

## Transformation classes

Every donor-derived file or subsystem receives one of these labels:

- `verbatim` — copied without semantic modification except formatting/import path adjustments;
- `adapted` — recognizable source retained but modified for Kernux contracts;
- `ported` — behavior reimplemented across language/runtime while intentionally preserving semantics;
- `reference-only` — studied but no source copied;
- `generated` — machine-generated from a source schema/contract with generator provenance.

## Import procedure

No donor import is considered complete until all of the following exist:

1. exact source revision pinned;
2. relevant source license/notice captured;
3. dependency obligations reviewed;
4. source-to-destination mapping recorded;
5. transformation class recorded;
6. characterization tests added or an explicit reason recorded when impossible;
7. imported code placed behind a Kernux-owned contract;
8. security review performed when the import touches host, browser, auth, remote, secrets, update, or plugin boundaries;
9. exact import commit recorded;
10. follow-up adaptation commit separated where practical.

## Mechanical import strategy

Prefer mechanically reviewable import history:

```text
commit A: provenance + notices + characterization fixture
commit B: donor import with minimal mechanical path/name changes
commit C+: Kernux adaptation/refactor behind canonical contracts
```

This makes future upstream comparison and license auditing possible.

For very large subsystem imports, a dedicated branch or subtree-like staging workflow is acceptable, but the final history still needs a deterministic source mapping.

## Third-party dependencies inside donors

Founder permission over donor source does not automatically change licenses or terms of embedded dependencies, fonts, icons, media, SDKs, model weights, trademarks, hosted APIs, or proprietary services.

Before copying an asset or vendored component, verify its own terms independently.

## Branding and trademarks

Do not ship donor names, logos, icons, screenshots, store assets, telemetry endpoints, update channels, service domains, or account backends as Kernux branding unless explicitly authorized and intentionally retained.

Kernux should reuse engineering, not impersonate donor products.

## Update policy

The pinned planning revisions are not permanent forks. Before each import wave:

- re-check upstream head and relevant changes;
- decide whether to import the pinned revision, a newer revision, or a bounded subset;
- document the choice;
- avoid silently mixing code from multiple upstream revisions in one import record.

## Initial donor map

| Kernux surface | Primary donor/reference | Kernux ownership requirement |
| --- | --- | --- |
| Desktop shell/workspace | Orca | Kernux UX + protocol boundary |
| Terminal/PTTY | Orca + Desktop Commander | Kernux runtime contract |
| CLI agent sessions | Orca | ACP/adapter abstraction |
| Git/worktrees | Orca | folder + runtime-neutral contract |
| Browser shell | Orca | browser capability contract |
| Semantic web extraction | TinyFish/AgentQL | provider adapter |
| Host filesystem/process | Desktop Commander | `kernuxd` policy enforcement |
| Documents/data | Desktop Commander | capability + artifact model |
| Remote machine execution | Orca + Desktop Commander | Kernux identity/protocol |
| Mobile steering | Orca | Kernux approval/task model |
| Security authorization | none adopted wholesale | Kernux-native capability kernel |
| Evidence/replay | none adopted wholesale | Kernux-native event/evidence model |
| Delivery decomposition | SpecGrain | repository methodology |
| Exact-diff proof | Diffcipline | repository methodology |

## Canonical rule

**Reuse aggressively where permission and provenance allow; couple conservatively.**

Kernux wins by combining proven implementations under stronger shared contracts, not by maintaining three applications inside one monorepo.