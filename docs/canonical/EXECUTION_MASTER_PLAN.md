# Execution Master Plan

## 0. Purpose

This is the canonical dependency-ordered build plan for Kernux. It converts the product thesis into implementation phases, slices, acceptance gates, and release criteria.

Implementation work must be refined into SpecGrain-sized units before execution. `specs/tasks.md` provides the initial task registry; `specs/CURRENT.md` identifies the active frontier.

The plan optimizes for **correct architecture and proof first, donor reuse second, feature count third**. Large donor imports are intentionally delayed until the Kernux contracts they must obey exist.

## 1. Delivery rules

1. Never implement a later slice by bypassing an unfinished invariant in an earlier dependency.
2. Each slice ends with evidence against its exact head.
3. Donor code is characterized and provenance-recorded before adaptation.
4. Privileged authority never migrates into the renderer for convenience.
5. Local-first end-to-end usefulness must be proven before cloud/team expansion.
6. Cross-platform assumptions are tested while surfaces are small.
7. Protocol/version compatibility is designed before remote production use.
8. A phase may run parallel slices only when their contracts are already frozen enough to avoid duplicate architecture.

---

# P00 — Repository and governance foundation

**Objective:** make the repository safe for high-velocity multi-agent implementation.

### S00.1 Repository baseline

Create/qualify:

- monorepo skeleton;
- Rust and pnpm workspace roots;
- canonical docs/spec layout;
- editor/config formatting baseline;
- CI skeleton;
- issue/PR templates;
- contributor/security/support docs;
- license decision and notices structure.

### S00.2 SpecGrain adoption

Initialize repository-local SpecGrain state and represent active implementation work as bounded Grains. Do not hand-edit generated/internal state.

### S00.3 Diffcipline adoption

Initialize proof policy with explicit R0-R3 profiles and CI integration. Initial policy should be conservative while repository size is small.

### S00.4 Provenance system

Create machine-readable donor ledger and notice inventory. Define import manifest schema and verification command.

**Exit gate P00**

- fresh clone can run documented checks;
- CI proves both Rust and JS workspace baselines;
- SpecGrain/Diffcipline workflows are documented and reproducible;
- donor import cannot be merged without provenance record;
- no product code yet depends on ungoverned donor internals.

---

# P01 — Core contracts and protocol spine

**Objective:** define the vocabulary every subsystem must share before importing large implementations.

### S01.1 Identity and resource model

Freeze v1 identifiers and revisions for Project, Task, WorkUnit, Run, AgentSession, Runtime, CapabilityRequest, Grant, Event, Artifact, Evidence.

### S01.2 Capability taxonomy

Define v1 capability namespaces, resource URI rules, constraints, consequence classes, and grant semantics.

### S01.3 Runtime contract

Define runtime discovery, capability negotiation, contact/execution status, operation identity, cancellation, and error semantics.

### S01.4 Event/evidence schema

Define append-oriented event envelope, artifact references, evidence binding, lineage/fork relationships, and redaction metadata.

### S01.5 Generated protocol types

Select schema source and generate Rust/TypeScript types with round-trip fixtures and versioning rules.

**Exit gate P01**

- cross-language schema conformance passes;
- invariants are represented in executable tests;
- runtime disconnect cannot be represented as implicit process exit;
- grants are structurally bounded;
- provider-specific names are absent from core contracts.

---

# P02 — Privileged kernel (`kernuxd`)

**Objective:** establish the security and durability foundation before the UI gains power.

### S02.1 Daemon lifecycle and private IPC

Implement launch, health, authenticated/bound local IPC, version handshake, structured errors, and graceful shutdown.

### S02.2 Local identity

Create installation/device identity, key material handling, instance/session binding, rotation/recovery primitives.

### S02.3 Metadata store and migrations

SQLite schema/migrations for projects, tasks, runs, events, runtimes, grants, artifacts, adapter config references.

### S02.4 Artifact CAS

Content-addressed artifact ingest/read, digest verification, retention metadata, bounded streaming.

### S02.5 Policy engine v1

Implement capability evaluation, scoped grants, profiles, expiry, delegation bounds, deny reasons, audit event creation.

### S02.6 Secret broker v1

OS credential integration abstraction, secret references, domain/tool scope, redaction-safe audit.

**Exit gate P02 (R3)**

- renderer-equivalent unprivileged client cannot bypass policy;
- restart preserves durable identity/events/grants correctly;
- migration/corruption/recovery paths tested;
- path/IPC/auth adversarial corpus passes;
- secrets are not persisted in ordinary SQLite records/logs.

---

# P03 — Desktop workspace foundation

**Objective:** produce the first usable Kernux shell without bypassing the daemon.

### S03.1 Electron shell + typed preload

Desktop process structure, narrow IPC bridge, startup/reconnect state, crash-safe renderer lifecycle.

### S03.2 Design system

Kernux tokens, primitives, typography, accessibility baseline, lintable design constraints.

### S03.3 Adaptive pane framework

Tabs/splits/docking, layout persistence, task-driven pane opening, keyboard navigation.

### S03.4 Project and command-bar shell

Project open/create, command bar, empty/onboarding states, run strip, notifications shell.

### S03.5 Timeline/artifact viewers

Basic durable event projection and artifact preview surfaces.

**Exit gate P03**

- desktop can connect/reconnect to daemon without ambient renderer Node privilege;
- layout/project state survives restart;
- basic accessibility keyboard journey passes;
- hidden/headless desktop E2E can validate UI without focus theft.

---

# P04 — Local computer runtime

**Objective:** give Kernux safe local hands through `kernuxd`.

### S04.1 Filesystem capabilities

Read/write/list/search/metadata with granted-root enforcement, symlink/junction defenses, atomic writes.

### S04.2 Process execution

argv-first process start, cwd/env policy, lifecycle, bounded output, cancellation/termination semantics.

### S04.3 PTY and terminal sessions

Cross-platform PTY, streaming, persistent session identity, scrollback/artifacts, interactive writes.

### S04.4 Process inspection

List/inspect/terminate with platform adapters and honest ownership/status.

### S04.5 Documents/data primitives

Bounded PDF/DOCX/Excel/CSV/JSON read/write/preview capability adapters derived where useful from Desktop Commander.

### S04.6 Host computer structured UI primitives

App/window/clipboard/notification/accessibility-tree operations where platform support allows; raw visual input remains later fallback.

**Exit gate P04 (R3 for host boundary)**

- no path escape outside grants;
- argv/shell behavior proven on macOS/Windows/Linux;
- long-running sessions survive renderer restart;
- destructive actions show appropriate policy escalation;
- donor-derived primitives have provenance + characterization tests.

---

# P05 — Agent integration foundation

**Objective:** make Kernux useful with real agents without binding to one provider.

### S05.1 ACP client v1

Implement stable ACP v1 client, registry/config discovery where appropriate, session lifecycle, permission/tool event mapping.

### S05.2 CLI/PTTY agent adapter

Generic supervised CLI agent adapter with explicit launch configs and transcript capture.

### S05.3 Initial agent profiles

Qualify representative Codex, Claude, OpenCode, Gemini/other available agents through ACP or CLI as supported by actual versions.

### S05.4 Agent auth/account ownership

Preserve provider-native auth/subscription ownership; support account/usage metadata only when observable.

### S05.5 Agent pane UX

Structured session events, messages, status, interrupt/resume, file/tool references, usage.

**Exit gate P05**

- at least two independent agents can perform the same bounded local task;
- unsupported provider features remain explicitly unsupported, not guessed;
- TUI parsing is transcript-backed;
- agent cannot bypass Kernux capability policy when using Kernux-owned tools.

---

# P06 — Browser and web runtime

**Objective:** integrate reliable web observation/action under one browser capability contract.

### S06.1 Browser context lifecycle

Local Chromium contexts, profile isolation, downloads/uploads, auth persistence policy, screenshots/artifacts.

### S06.2 Deterministic CDP/Playwright layer

Navigate, observe DOM/accessibility, act, wait, extract, trace.

### S06.3 WebMCP adapter

Discover/invoke page tools with schema validation, provenance, policy escalation, and experimental capability negotiation.

### S06.4 Semantic web provider

Import/adapt AgentQL/TinyFish semantics for natural-language targeting and structured extraction behind Kernux browser contracts.

### S06.5 Visual fallback

Screenshot + computer-use action provider for pages/apps where structured methods fail, with explicit confidence/observation and takeover UX.

### S06.6 Browser evidence

Action method, page origin, relevant observation/screenshot, downloads, and source lineage captured in timeline.

**Exit gate P06 (R3 for injection/credential boundaries)**

- authenticated contexts do not leak across projects by default;
- malicious page/WebMCP/tool-output injection cannot self-authorize privilege;
- hierarchy WebMCP -> deterministic -> semantic -> vision is observable/tested;
- research journey can produce structured evidence bundle.

---

# P07 — Task engine and agent fleet

**Objective:** turn individual agent sessions into outcome-oriented orchestration.

### S07.1 Task/WorkUnit planner

Plan revisions, dependencies, readiness, capability/runtime requirements, budgets, acceptance/evidence requirements.

### S07.2 Execution scheduler

Dependency-aware work units, concurrency limits, cancellation, retries only with safe semantics, runtime selection.

### S07.3 Fleet fan-out

Run bounded task across multiple agents with isolated workspaces/runtimes.

### S07.4 Candidate evaluation

Verification matrix, measured evidence, cost/latency, human annotations, model-assisted analysis clearly marked as opinion.

### S07.5 Synthesis lineage

Create a new candidate from selected components/feedback and require fresh proof.

### S07.6 Budget/resource control

Token/cost/runtime quotas when providers expose reliable usage; hard concurrency/time limits regardless.

**Exit gate P07**

- one command can coordinate multiple agents without shared-write corruption;
- retries do not duplicate consequential operations;
- comparison never presents model judgment as test evidence;
- task restart/recovery is honest and deterministic.

---

# P08 — Git, workspaces, and review

**Objective:** provide a world-class developer wedge on top of the general runtime.

### S08.1 Folder/git project adapter

Support plain folders and git repositories.

### S08.2 Worktree isolation

Create/list/switch/remove bounded worktrees, cleanup/recovery, collision handling.

### S08.3 Source control

Status, diff, stage, commit, branch, history, conflict-safe operations across native/WSL/SSH Git versions.

### S08.4 Review UX

Diff comments, agent feedback loop, candidate compare, exact-head verification badges.

### S08.5 GitHub provider

Issues/PRs/review workflow behind provider-neutral review contracts. GitLab/other providers remain future adapters, not architecture blockers.

**Exit gate P08**

- parallel coding candidates are isolated;
- exact-head proof shown in review;
- Git baseline/fallback behavior proven on representative hosts;
- user can complete local task -> diff -> verify -> commit/PR journey.

---

# P09 — Evidence, replay, and recovery

**Objective:** make Kernux work durable, inspectable, and reproducible.

### S09.1 Event projections

Deterministic rebuild of run/task UI from durable event stream + projections.

### S09.2 Checkpoints

Record supported task/runtime/browser/workspace checkpoints without pretending every external system can be snapshotted.

### S09.3 Replay semantics registry

Mark operations deterministic/idempotent/compensatable/non-replayable and enforce safe fork behavior.

### S09.4 Fork from checkpoint

Create new run lineage with inherited artifacts/context and fresh operation ids.

### S09.5 Evidence bundle export

Manifest + digests + verification + provenance + redaction metadata.

### S09.6 Crash/interruption recovery

Desktop crash, daemon restart, remote interruption, browser death, partial durable write.

**Exit gate P09**

- accepted golden journeys survive restart where contract says they should;
- duplicate side effects are prevented or surfaced as unresolved ambiguity;
- event replay reproduces projections;
- evidence bundle validates independently.

---

# P10 — Runtime fabric: sandbox, WSL, SSH, remote device

**Objective:** make execution location interchangeable without losing truth or policy.

### S10.1 Container runtime

Create/destroy/mount/network policy/resource limits/artifact transfer.

### S10.2 VM/microVM provider interface

Define stronger-isolation contract and qualify at least one implementation if practical for Gen 1.

### S10.3 WSL runtime

Explicit distro identity, argv/path/environment handling, file mapping, Git/process/PTY capability.

### S10.4 SSH runtime

Enrollment/config, capability probing, reconnect, remote process ownership, port forwarding where needed.

### S10.5 Kernux remote device

Mutual identity, version/capability negotiation, resumable event streams, operation/idempotency protocol, revocation.

### S10.6 Runtime chooser UX

Automatic/default selection plus visible user override/trust boundary.

**Exit gate P10 (R3)**

- same bounded workload can run local + one isolated + one remote runtime;
- disconnect never fabricates exit;
- replay/duplicate call tests pass;
- controller grants cannot exceed remote host policy;
- mixed supported versions negotiate safely.

---

# P11 — Tool, MCP, skills, and integration ecosystem

**Objective:** make Kernux extensible without turning every integration into core code.

### S11.1 MCP 2026-07-28 client

Stateless requests, discovery/cache semantics, auth, extensions, Tasks mapping, compatibility strategy.

### S11.2 MCP server/gateway

Expose selected Kernux tools/capabilities under policy; optional gateway/organization mediation later.

### S11.3 Agent Skills

Discover/install/project-scope skills with provenance and declared capability requirements.

### S11.4 OpenAPI/HTTP/CLI adapters

General integration paths with secret brokering and output bounds.

### S11.5 Plugin manifest

Identity/version/source/capabilities/network/secrets/entrypoint/data-boundary declarations; permission delta on updates.

### S11.6 Extension SDK

Stable subset of task/capability/event/artifact interfaces only after internal contracts prove durable.

**Exit gate P11**

- third-party integration cannot silently expand permissions;
- protocol conformance fixtures pass;
- plugin update permission deltas are visible;
- one external MCP and one skill/tool integration complete golden workflow.

---

# P12 — Automation, triggers, and long-running work

**Objective:** support useful unattended operation without weakening the permission model.

### S12.1 Scheduled tasks

Local scheduler with explicit machine availability semantics.

### S12.2 Event/webhook triggers

Scoped triggers for repositories/services where connectors support them.

### S12.3 Background run supervision

Wake/sleep/restart semantics, notifications, budgets, approval waits.

### S12.4 Templates/recipes

Reusable task intentions that compile to ordinary task/work-unit contracts, not a separate workflow engine.

**Exit gate P12**

- unattended task cannot bypass expiry/approval rules;
- restart preserves scheduler intent without duplicate execution;
- missed/late execution semantics are explicit.

---

# P13 — Mobile companion

**Objective:** safely steer remote work from phone/tablet.

### S13.1 Pairing/auth

Pair mobile client to user/runtime/service with revocation.

### S13.2 Task/status/notification UX

Active runs, blockers, results.

### S13.3 Approval cards

Full consequential-action context rendered on mobile.

### S13.4 Steering

Follow-up, pause/resume/stop, simple task launch.

### S13.5 Visual observation

Approved screenshot/browser/computer stream and result artifacts.

**Exit gate P13**

- mobile cannot exceed desktop/kernel policy;
- approval state synchronizes safely;
- lost/stale mobile client cannot replay expired grants.

---

# P14 — Teams and optional Kernux cloud

**Objective:** add collaboration without making cloud mandatory.

### S14.1 Organization identity and projects

Users, teams, roles, project membership.

### S14.2 Policy intersection

Organization baselines can tighten, not silently weaken, local/runtime hard constraints.

### S14.3 Shared runtimes/skills/secrets references

Explicit ownership, audit, revocation.

### S14.4 Collaboration

Shared tasks, human/agent assignments, comments/notes, presence where useful.

### S14.5 Relay/sync

Encrypted authenticated relay for NAT/mobile/remote scenarios with transparent data boundary and local-first fallback.

### S14.6 Marketplace/registry (post-core)

Signed/provenanced skills/integrations/runtimes when ecosystem maturity justifies it.

**Exit gate P14**

- local-only projects remain functional;
- organization policy and audit are deterministic;
- shared secret values are not exposed by synchronization metadata;
- revocation propagates and fails closed.

---

# P15 — Hardening, release, and public launch

**Objective:** prove Kernux as a reliable product, not only a working repository.

### S15.1 Cross-platform qualification

macOS, Windows, Linux plus relevant WSL/SSH journeys.

### S15.2 Security closure

Threat model review, adversarial corpus, dependency/vulnerability review, penetration-style manual checks for R3 surfaces.

### S15.3 Performance and scale

Multi-agent/terminal/browser stress, large repository bounds, event/artifact storage scale, startup/latency budgets.

### S15.4 Installer/update/uninstall/recovery

Signed installers, update channel, migrations, rollback/recovery, clean uninstall behavior.

### S15.5 Supply chain

SBOM, checksums, provenance attestations, license/notice closure, protected release pipeline.

### S15.6 Documentation/onboarding

User docs, admin/security docs, developer SDK docs, troubleshooting, privacy/data-boundary docs.

### S15.7 Comparative evaluation

Preregister reproducible benchmark/journey set if making competitive claims. Preserve negative evidence.

### S15.8 Release qualification

Qualify exact release commit and artifacts independently from ordinary branch CI.

**Exit gate P15 / RELEASE_READY**

- all release-blocking acceptance journeys are PROVEN;
- supported platform matrix is qualified;
- R3 findings are closed or explicitly release-accepted with owner/rationale;
- release artifacts are signed/checksummed/provenanced;
- donor/third-party notices are complete;
- installer/update/recovery are proven;
- public claims do not exceed evidence.

---

# 2. Parallelization strategy

Safe early parallelism after P01 contracts:

- daemon storage/artifacts can progress beside desktop design system;
- agent ACP adapter research can progress beside local runtime primitives;
- browser deterministic layer can progress beside git adapters;
- documentation/golden journey fixtures can progress continuously.

Unsafe parallelism to avoid:

- importing donor host code before capability policy/runtime contract exists;
- building remote protocol before operation/status semantics are frozen;
- implementing fleet synthesis before candidate isolation/evidence is reliable;
- building cloud/team policy before local grant semantics stabilize;
- building a marketplace before plugin permission/provenance model is proven.

# 3. First valuable product milestone

The first external alpha should not wait for P15 features. The earliest coherent **Kernux Alpha** is after P09 with a bounded P10 subset:

- desktop workspace;
- local daemon and permission kernel;
- local files/process/PTY;
- two real agent integrations;
- browser deterministic + semantic capability;
- task/fleet basics;
- git/worktree review;
- evidence/replay/restart recovery;
- one sandbox runtime.

That alpha must already feel like one product, not a donor demo.

# 4. Scope discipline

Features intentionally deferred until their dependencies exist:

- team marketplace;
- broad enterprise administration;
- proprietary foundation models;
- full IDE language feature parity;
- arbitrary no-code workflow canvas;
- dozens of cloud runtime providers;
- autonomous financial/purchase execution defaults;
- unsupported claims of universal computer autonomy.

# 5. Definition of project progress

Do not report a single completion percentage without explaining denominator. Track separately:

- canonical tasks PROVEN / planned;
- phases with exit gates PROVEN / total;
- golden journeys PROVEN / total applicable;
- platform qualification matrix;
- security R3 closure;
- release qualification.

A feature count is not project completion.

# 6. Implementation handoff rule

Any build agent starting from this plan must:

1. reverify live repository/GitHub truth;
2. read `AGENTS.md`, `specs/CURRENT.md`, this master plan, and relevant canonical contracts;
3. operate only on the active dependency-eligible Grain/task;
4. preserve donor provenance;
5. run required exact-risk proof;
6. update canonical frontier truth without overstating completion;
7. continue automatically to the next genuinely authorized dependency-eligible unit when instructed to build the project continuously.