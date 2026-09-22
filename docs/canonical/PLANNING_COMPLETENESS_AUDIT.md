# Planning Completeness Audit

## 1. Purpose

This audit challenges the canonical Kernux plan before implementation starts. It asks whether the plan actually covers the product thesis — a single operating environment for agents, web, computers, tools, local/remote execution, and verifiable outcomes — rather than merely combining three donor codebases.

Audit date: **2026-09-17**.

The audit is intentionally conservative. A capability is considered covered only when there is a clear architectural owner, security boundary, delivery phase, and evidence path.

## 2. Inputs reviewed

### Founding donors

- `stablyai/orca` at planning revision recorded in `DONOR_PROVENANCE.md`;
- TinyFish / public `tinyfish-io/agentql` reference plus founder-authorized source;
- `wonderwhy-er/DesktopCommanderMCP` at planning revision recorded in `DONOR_PROVENANCE.md`.

### Kernux canonical plans

- product thesis;
- architecture;
- architecture decisions;
- security model;
- protocol strategy;
- UX blueprint;
- quality/evidence;
- execution master plan;
- macro task registry;
- donor provenance.

### Current protocol truth reverified during planning

- ACP stable v1 remains the stable API while protocol v2 is draft/experimental;
- MCP `2026-07-28` is final and introduces a stateless core plus extension framework/Tasks/auth changes;
- A2A 1.0 is the current stable documentation generation for agent-to-agent interoperability;
- WebMCP remains experimental/origin-trial territory and must be feature-detected/negotiated.

Protocol-dependent implementation must reverify this truth again at execution time.

## 3. Founding-donor feature coverage

### Orca-derived capability families

| Capability family | Kernux coverage | Notes |
| --- | --- | --- |
| Multi-agent workspace | COVERED | P03 + P05 + P07 |
| Parallel isolated coding work | COVERED | P07 + P08 |
| Terminal splits/persistence | COVERED | P03/P04 |
| Browser workspace | COVERED | P06 |
| Design Mode | ADDED/CLARIFIED | Native agent/headless contract makes structured capture explicit |
| Git/worktree review | COVERED | P08 |
| SSH/remote runtime | COVERED | P10 |
| Mobile steering | COVERED | P13 |
| CLI/scriptability | ADDED/CLARIFIED | Headless/CLI is now a product contract rather than only a tool-adapter detail |
| GitHub/review integration | COVERED | P08 |
| Agent account/usage metadata | COVERED | P05 |
| Notifications/unread | COVERED | UX + P12/P13 |
| Rich file/artifact preview | COVERED | P03/P04 |
| Cross-platform hardening | COVERED | permanent X01 + P15 |

### TinyFish / AgentQL-derived capability families

| Capability family | Kernux coverage | Notes |
| --- | --- | --- |
| Natural-language web targeting | COVERED | P06 semantic provider |
| Structured extraction | COVERED | P06 |
| Playwright integration | COVERED | P06 deterministic layer |
| Authenticated browser sessions | COVERED | P06 security/lifecycle |
| UI-change resilience | COVERED AS PROVIDER FEATURE | not promoted to core invariant |
| Local/remote browser providers | COVERED | browser contract + runtime fabric |
| Browser evidence/provenance | STRONGER THAN DONOR BASELINE | P06 + P09 |
| Prompt-injection boundary | STRONGER THAN DONOR BASELINE | security model + P06 adversarial gate |

### Desktop Commander-derived capability families

| Capability family | Kernux coverage | Notes |
| --- | --- | --- |
| Filesystem read/write/search | COVERED | P04 |
| Long-running process/session control | COVERED | P04 |
| PTY/terminal | COVERED | P04 |
| Process inspection/termination | COVERED | P04 |
| Python/Node/R execution patterns | COVERED THROUGH PROCESS/SANDBOX | no special unsafe bypass |
| Excel/PDF/DOCX/data operations | COVERED | P04 document/data capability |
| MCP exposure | COVERED | P11 |
| Remote device | COVERED | P10 |
| Audit history | STRONGER KERNUX MODEL | P02/P09 event/evidence model |
| Docker isolation | COVERED | P10, explicitly not equivalent to universal sandboxing |
| Safety guardrails | REPLACED BY STRONGER MODEL | kernel-enforced capability policy |

## 4. Product-level gaps found and closed

The first canonical pass had strong runtime/security architecture but six product-level surfaces were not explicit enough. They are now binding refinements of the existing phases.

### G01 — Native Kernux agent/model runtime

**Why it mattered:** relying only on external coding agents would make Kernux primarily a developer shell. The product thesis is broader.

**Closure:** `NATIVE_AGENT_AND_HEADLESS.md` separates ModelProvider, external Agent Adapter, and Kernux Native Agent. A native model-backed loop uses typed capabilities through kernel policy.

**Required before alpha:** one native model-backed task proven end-to-end in addition to two external agent integrations.

### G02 — Context, search, indexing, and memory

**Why it mattered:** agents without reliable project/task context require huge prompts, repeated user explanation, or opaque provider memory.

**Closure:** `CONTEXT_MEMORY_AND_SEARCH.md` defines provenance-bound ContextBundles, bounded local search, optional semantic retrieval, scoped user-governed memory, freshness/invalidation, and memory-poisoning defenses.

**Required before alpha:** quick-open/search, inspectable context bundle, source invalidation, local-only operation, and deletable memory.

### G03 — First-class CLI/headless operation

**Why it mattered:** a universal runtime must be scriptable from CI, terminals, servers, and remote systems without driving Electron UI.

**Closure:** `NATIVE_AGENT_AND_HEADLESS.md` defines headless use of the same task/policy/evidence model, machine-readable CLI output, stable error/exit classes, and safe non-interactive approvals.

**Required before alpha:** launch/inspect/stop a bounded task from CLI and desktop against the same contract.

### G04 — Design Mode / structured visual context capture

**Why it mattered:** this is one of the strongest interaction ideas in Orca and generalizes beyond coding to UI debugging/design work.

**Closure:** browser/native Design Mode captures DOM/accessibility/style/screenshot context as typed provenance-bearing artifacts. It observes; it does not self-authorize subsequent actions.

### G05 — Data lifecycle, portability, backup, and deletion

**Why it mattered:** local-first trust is incomplete if users cannot move or remove their own history.

**Closure:** `DATA_LIFECYCLE_AND_PORTABILITY.md` defines retention classes, CAS GC, project export/import, backup/restore, delete semantics, browser-profile cleanup, telemetry defaults, and support bundles.

### G06 — Privacy-safe diagnostics

**Why it mattered:** privileged software needs supportability without normalizing prompt/file/secret collection.

**Closure:** support bundles are manifest-driven, reviewable, redacted, and separable from optional product telemetry. Telemetry-disabled mode remains functional.

## 5. Additional architecture questions checked

### Does Kernux require cloud to be useful?

No. Local daemon, desktop/CLI, local browser, local files/processes, agents/models, evidence, and project history are designed to work without Kernux cloud.

### Can a renderer/model/plugin grant itself power?

No. `kernuxd` owns privileged enforcement; UI/model/tool text can request but not authorize capabilities.

### Can donor internals become public architecture accidentally?

The plan explicitly requires donor code behind Kernux-owned contracts and provenance records.

### Is remote disconnect treated as process death?

No. Contact and execution are independent; `unverifiable` is a first-class state.

### Are retries safe for consequential operations?

The operation/request identity distinction plus replay/idempotency registry is mandatory before remote/background automation is considered proven.

### Can web content or retrieved memory escalate privilege?

No. Untrusted data can influence understanding, not authorization. Memory and browser provenance remain part of policy input.

### Can Kernux be used by non-developers?

Yes architecturally after the native agent runtime, browser/computer capabilities, document/data primitives, general workspaces, and artifact delivery are implemented. Developer workflows remain the first wedge because they offer stronger deterministic verification.

### Is every subsystem required to be its own permanent service?

No. Contracts define ownership boundaries; implementation can remain a modular monolith where isolation does not require a process boundary. `kernuxd` is the deliberate privileged boundary.

## 6. Explicitly deferred — not forgotten

These are deferred because building them earlier would harm focus or depend on unproven contracts:

- broad enterprise administration;
- public marketplace before plugin permission/provenance maturity;
- dozens of cloud/GPU providers;
- full IDE parity with JetBrains/VS Code;
- visual no-code workflow canvas;
- proprietary foundation models;
- universal autonomous purchasing/financial defaults;
- broad native-app coordinate automation where structured APIs/accessibility suffice;
- cloud-required memory or sync;
- unsupported claims of fully autonomous general computer use.

Deferred items must not be marketed as existing capabilities.

## 7. Mandatory phase refinements

The macro task registry stays intentionally coarse. During SpecGrain refinement, the following child work is mandatory before the associated phase gate may close.

### P01

- model/provider/agent identity and capability envelopes;
- ContextSource/ContextItem/ContextBundle/MemoryRecord schema;
- CLI/API structured result/error/event envelopes;
- data retention/export schema version identifiers.

### P02

- context-scope enforcement;
- index metadata ownership;
- authenticated headless local-client boundary;
- retention/GC metadata;
- diagnostic logging/redaction foundation.

### P03

- universal quick-open/search;
- Files/Editor baseline;
- context inspector;
- memory inspector/management;
- CLI bootstrap and `doctor` flow;
- Design Mode UI entry/capture inspector.

### P05

- ModelProvider interface;
- at least one native model adapter suitable for development/testing;
- Kernux Native Agent execution loop;
- native-agent context assembly integration;
- capability/tool observation mapping;
- two external agent integrations remain independently qualified.

### P06

- browser Design Mode capture;
- semantic-provider provenance;
- authenticated profile lifecycle and cleanup;
- WebMCP feature detection because the platform remains experimental.

### P07

- native-agent planner validation;
- context bound to exact Task/WorkUnit revision;
- model strategy/budget selection;
- fleet candidates retain independent ContextBundle/evidence lineage.

### P09

- export/import format;
- CAS reference-aware GC;
- backup/restore;
- delete semantics;
- context/memory replay lineage;
- portable evidence verification outside UI.

### P11

- published headless/SDK surface only after contracts stabilize;
- external integrations declare data boundaries and requested capabilities.

### P15

- telemetry-disabled qualification;
- support-bundle secret fixtures;
- backup/restore migration qualification;
- deletion/uninstall qualification;
- context/search performance bounds;
- privacy/data-lifecycle documentation.

## 8. Expanded golden journeys

In addition to the existing quality/UX journeys, release planning must eventually prove:

1. **Native agent knowledge work** — open a non-code workspace, ask for a bounded research/analysis outcome, use browser/files/tools, inspect sources, export artifact.
2. **Headless parity** — launch the same bounded task through CLI and desktop and observe the same task/evidence model.
3. **Context provenance** — inspect exactly which file/artifact/web/memory items an agent received and invalidate stale context after a source change.
4. **Design Mode** — select a browser UI element, attach the structured capture to an agent task, make a verified change, and trace capture provenance.
5. **Portable project history** — export, validate, import on a clean installation, and recover selected tasks/evidence/artifacts without secrets.
6. **Deletion** — remove a Kernux project/history/browser profile and prove policy-required data is gone while the source folder remains untouched unless explicitly selected.
7. **Offline local core** — complete a useful local task with Kernux cloud unavailable and telemetry disabled.
8. **Memory poisoning defense** — malicious retrieved web/document text cannot become trusted durable memory or authorize a privileged action.

## 9. Alpha readiness after this audit

The planned early alpha remains after P09 plus a bounded P10 sandbox subset, but the alpha definition is strengthened. It now requires:

- desktop workspace plus usable CLI;
- privileged daemon/capability policy;
- local files/process/PTY;
- Kernux Native Agent with one usable model path;
- at least two external agents;
- local search/context inspection;
- browser deterministic + semantic paths;
- Design Mode capture;
- task/fleet basics;
- git/worktree review for developer workflows;
- evidence/replay/restart recovery;
- exportable evidence/project history subset;
- one isolated runtime;
- telemetry-disabled local operation.

This is intentionally stricter than simply proving the three donor feature sets individually.

## 10. Completion verdict

**Planning architecture:** COHERENT

**Founding donor coverage:** ACCOUNTED FOR

**Initial-pass critical product gaps identified:** 6

**Universal-platform second-pass gap families identified:** 10

**Critical architectural gap families left without a canonical owner/refinement path:** 0 known at this planning depth

**Implementation started:** NO

**Release ready:** NO

This is not a claim that future implementation will reveal no new work. It means the current plan has explicit ownership and a refinement path for all major capability, security, lifecycle, interoperability, UX, and evidence surfaces identified in this audit.

## 11. Universal-platform second-pass audit — 2026-09-21

The initial audit focused on whether Kernux could coherently combine agents, browser/web execution, computer execution, local/remote runtimes, context, evidence and security.

A second pass challenged a harder objective: whether the architecture can eventually become a trusted default layer for broad digital work without collapsing into an unfocused application clone or a founder-subsidized cloud service.

Current external product/reference observations reinforced several expectations:

- agent systems increasingly work across files, apps, browser and desktop surfaces;
- finished editable documents/spreadsheets/presentations are becoming first-class outputs;
- long-running/background work needs durable state, approvals and recovery;
- broad integration catalogs require scalable auth/discovery/routing rather than static tool injection;
- computer-use products increasingly support hosted and bring-your-own-machine paths;
- multi-agent fan-out and durable orchestration require explicit budget/retry/evidence semantics.

The second pass identified the following additional gap families.

### G07 — Finished artifact production and round-trip editing

**Risk if absent:** Kernux could perform complex reasoning yet still force users into another product for every final deliverable.

**Closure:** `UNIVERSAL_PLATFORM_PLAN.md` U05 plus P16 makes editable documents, spreadsheets, presentations, analysis packages, reports and lightweight sites/apps first-class Artifacts with validation, lineage, templates and explicit round-trip limits.

### G08 — Scalable integration/account/event fabric

**Risk if absent:** broad app support would degrade into hundreds of bespoke integrations, giant tool lists, duplicated OAuth handling and inconsistent revocation.

**Closure:** U04 plus P17 defines connector discovery, account/auth lifecycle, secret brokering, intent-based tool routing, event subscriptions, data-boundary inspection, capability drift and generated/external adapter boundaries. `platform-fabrics/LAYA_INTEGRATION.md` additionally freezes the donor source boundary, IntegrationEvent contract, connector/event lifecycle, dependency disposition and bounded Laya source-admission path.

### G09 — Full web intelligence beyond browser clicking

**Risk if absent:** research/monitoring work would depend on browser navigation even when search/fetch/crawl/change-detection paths are cheaper, more reliable or easier to cite.

**Closure:** U02 distinguishes search, fetch, crawl, extraction, browser interaction, authenticated sessions, monitoring and source lineage behind provider-neutral contracts.

### G10 — Durable proactive/background work

**Risk if absent:** schedules, triggers and monitors could become a second unsafe workflow engine with different authority/retry semantics.

**Closure:** U06, P19 and ADR-0046 require all proactive/background work to compile to ordinary Task/WorkUnit/Run/Grant/Operation/Evidence semantics. `platform-fabrics/LAYA_INTEGRATION.md` adds ActionProposal/rule state machines, dedupe/replay, stale-approval handling, ambiguous-side-effect reconciliation and dead-event recovery.

### G11 — Multimodal and voice interaction

**Risk if absent:** Kernux would remain keyboard/text-centric while real computer work includes screenshots, screens, audio, voice and visual context.

**Closure:** U01, P18 and ADR-0048 define multimodal observations as provenance-bearing context, never authority.

### G12 — General visual computer-use fallback

**Risk if absent:** structured APIs/accessibility/CLI are preferable but cannot cover every legacy or closed desktop application.

**Closure:** U03 and P18 require a bounded vision + mouse/keyboard fallback behind the existing capability kernel, approval and evidence model.

### G13 — Cross-device continuity beyond mobile steering

**Risk if absent:** desktop, CLI, mobile and remote machines could show different task truth or duplicate side effects during handoff.

**Closure:** U10 and P18 require one task/run identity, resumable observation, device trust/revocation, offline/degraded state and timezone-consistent scheduling.

### G14 — Proactive personal/work context

**Risk if absent:** Kernux could remain a task launcher rather than becoming more useful over time across the user's approved files, apps, memory and routines.

**Closure:** U07/U19 and P19 define user-governed relationship context, universal capture, routines, condition watches and proactive-but-policy-bound suggestions. `platform-fabrics/LAYA_INTEGRATION.md` adds Action Inbox, Action Workspace, Coherence, Omni/Attention/Briefing projections, correction learning and source-inspection requirements. Graph state remains retrieval structure, never authority.

### G15 — Accessibility and internationalization as architecture

**Risk if absent:** a supposedly universal product would embed English/Latin/keyboard/mouse assumptions that are expensive to remove later.

**Closure:** U15, P18, X05 and X12 cover semantic accessibility, keyboard operation, RTL, Unicode, IME, locale and timezone behavior.

### G16 — Universal reliability/evaluation/ecosystem maturity

**Risk if absent:** feature breadth could grow faster than evidence, SLOs, extension governance and provider-failure resilience.

**Closure:** U12-U14, P20, X13 and existing P15/P11 governance establish signed extensions, benchmark/regression coverage, SLO/chaos qualification, reproducible comparison rules and provider-loss journeys.

### Second-pass conclusion

The universal objective is now separated into:

- **Generation 1:** P00-P15, preserving the focused developer/power-user wedge and proving the trusted Agent Operating Environment;
- **Generation 2:** P16-P20, expanding the proven kernel into work-product creation, broad integrations, multimodal/cross-device use, proactive personal/work assistance and ecosystem-scale maturity.

This prevents universal ambition from authorizing premature breadth during the current P02 implementation.

The canonical anti-gap ownership matrix in `UNIVERSAL_PLATFORM_PLAN.md` requires every future feature to identify outcome, capability, authorization, runtime, context, secret/data/cost ownership, persistence, recovery, side effects, artifact, evidence, accessibility, compatibility, deletion, cross-device behavior and test path.

## 12. Rule

**A complete plan does not predict every bug; it makes every major responsibility, boundary, failure mode, and proof obligation have an owner before code makes the decision accidentally.**
