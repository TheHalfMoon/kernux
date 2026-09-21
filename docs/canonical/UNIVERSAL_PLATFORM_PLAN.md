# Universal Platform Plan

## 1. Purpose

Kernux's first wedge remains power users who already combine agents, terminals, browsers, repositories, and remote machines. That wedge is intentionally narrow enough to prove the capability kernel with deterministic evidence.

The long-term platform objective is broader:

> **Kernux should become the default trusted operating environment for digital work: one place where a person can express an outcome, let qualified agents use approved apps, web, computers, files, tools, and runtimes, and receive a verifiable finished result without being locked to one model, cloud, or application ecosystem.**

This objective is a direction and architecture contract, not a claim that Kernux already replaces every application or is universally superior. Any comparative or superiority claim remains evidence-gated.

The strategy is **integration over imitation**. Kernux should not rebuild every email client, office suite, browser, CRM, design tool, or operating system. It should provide the trusted orchestration, context, capability, execution, evidence, and continuity layer that can use those systems through the best available interface.

## 2. Non-negotiable platform principles

1. **Outcome first.** Users describe desired outcomes; Kernux handles routing and decomposition.
2. **Capability before provider.** Public contracts describe what can be done, not which vendor performs it.
3. **Structured before visual.** Prefer typed APIs, MCP/WebMCP, accessibility, DOM, CLI, and file contracts before coordinate automation.
4. **Local-first, cloud-capable.** Core usefulness must not require Kernux cloud.
5. **Owner-sustainable execution.** Core usefulness must not depend on project-owner-subsidized variable compute.
6. **Human agency before autonomy.** High-consequence actions require explicit policy, bounded grants, evidence, and appropriate approval.
7. **Evidence before done.** Model self-report never proves completion when stronger evidence exists.
8. **Portable user truth.** Projects, memory, artifacts, policies, and history remain inspectable, exportable, and deletable.
9. **Provider and ecosystem portability.** Users can bring agents, models, runtimes, tools, data, and accounts they already use.
10. **Progressive disclosure.** Simple work stays simple; advanced policy, traces, and routing are available when needed.
11. **Accessibility and internationalization are architecture.** Keyboard, screen-reader, RTL, locale, timezone, and multilingual behavior cannot be release-afterthoughts.
12. **Failure is a first-class state.** Ambiguity, partial work, unavailable providers, stale data, and blocked approvals remain explicit and recoverable.

## 3. Universal capability families

A long-term universal Kernux must cover every major digital-work responsibility through a stable owner and evidence path.

### U01 — Interaction and multimodal input/output

Kernux must support, behind negotiated capability contracts:

- text and structured command input;
- files, folders, URLs, clipboard, and drag/drop;
- image and screenshot context;
- audio/voice input and output;
- screen/window capture;
- camera/video context where platform policy permits;
- selection/annotation/Design Mode context;
- machine-readable CLI/API input/output;
- mobile and remote steering.

Voice, image, and screen input are context channels. They do not bypass capability policy.

### U02 — Web intelligence

The web layer must distinguish:

- search;
- fetch/read;
- crawl/bounded site traversal;
- structured extraction;
- browser navigation and interaction;
- authenticated browser sessions;
- downloads/uploads;
- change detection and monitoring;
- citations/source lineage;
- site-provided structured tools such as WebMCP;
- visual fallback when structured access is unavailable.

A provider-neutral Search/Fetch/Web Intelligence contract must allow local/self-hosted, BYOK, BYOC, and optional managed providers. TinyFish/AgentQL-derived capability may implement parts of this layer but must not define the public contract.

### U03 — Computer and application control

Kernux should be able to work with applications through this ordered hierarchy:

1. application-native API or plugin;
2. MCP/OpenAPI/CLI or other structured tool;
3. filesystem/document format;
4. OS accessibility/UI-automation tree;
5. deterministic window/application controls;
6. vision + mouse/keyboard fallback.

The platform must support files, processes, PTY/terminal, clipboard, notifications, application launch/control, and bounded visual computer use without granting ambient host privilege.

### U04 — Integration and account fabric

Kernux must support broad app coverage without hard-coding hundreds of services into the core.

Required platform surfaces:

- connector/adapter registry;
- MCP, OpenAPI/HTTP, CLI, A2A, Agent Skills, and native adapter paths;
- OAuth and API-key lifecycle through the secret broker;
- provider-native subscription/account ownership;
- connection health and capability discovery;
- scoped account selection when multiple identities are connected;
- revocation and permission-delta handling;
- event/webhook/subscription adapters;
- intent-based tool discovery so agents do not receive thousands of tools at once;
- data-boundary metadata for every integration;
- optional external integration providers behind the same contracts.

Breadth should come from protocols, generated adapters, and ecosystem packages rather than coupling core code to every vendor.

### U05 — Finished artifact and work-product studio

Kernux must produce and revise real user deliverables, not only chat responses.

Target artifact families include:

- Markdown/plain text;
- PDF;
- DOCX and compatible word-processing documents;
- XLSX/CSV and spreadsheet models;
- PPTX and presentation decks;
- notebooks and reproducible analysis packages;
- charts/figures;
- HTML/reports;
- lightweight sites and apps;
- structured JSON/data exports;
- image/media artifacts through approved providers/tools.

Required quality properties:

- round-trip editing when format support exists;
- preservation of formulas, layouts, styles, notes, and metadata when requested;
- template and brand-system support;
- source/provenance links;
- deterministic validation where feasible;
- editable output rather than flattened screenshots when the target format supports editing.

Kernux should prefer existing mature editors and file formats over rebuilding an office suite.

### U06 — Durable automation and proactive work

Long-running and unattended work must use the same task/capability/evidence model as interactive work.

Required behavior:

- one-time delayed tasks;
- recurring schedules;
- event/webhook triggers;
- condition watches;
- missed-run policy;
- machine-offline behavior;
- retries with operation identity;
- suspension for approval;
- wake/reconnect/resume;
- deduplication;
- deadlines and budgets;
- cancellation;
- notification and escalation;
- observable state across restarts.

No scheduled or background mode may introduce a hidden autonomy bypass.

### U07 — Context, search, memory, and relationship graph

The existing ContextBundle and MemoryRecord model remains the foundation.

Universal operation additionally requires:

- project/workspace context;
- personal user-owned context;
- organization context under explicit policy;
- connected-app context;
- relationship/entity indexing where measurable;
- temporal/freshness semantics;
- conflict handling;
- source revision binding;
- explainable retrieval;
- memory promotion/rejection/edit/delete;
- cross-project retrieval only by policy;
- local search remaining useful without external embedding services.

A knowledge graph is a retrieval structure, not an authorization system or unquestioned source of truth.

### U08 — Agent fleet and specialization

Kernux must support:

- one-agent execution;
- planner/executor;
- researcher/analyst/writer/reviewer roles;
- independent parallel candidates;
- wide research/fan-out;
- verifier agents;
- synthesis with new lineage;
- escalation to stronger/different models;
- replacement when an agent fails;
- human participants as explicit task actors;
- remote A2A collaborators.

Fleet expansion must remain budget-, permission-, context-, and evidence-aware.

### U09 — Runtime and isolation fabric

The same task model should work across:

- host;
- WSL;
- container;
- stronger VM/microVM isolation;
- SSH;
- enrolled remote device;
- user-owned server;
- GPU runtime;
- optional managed cloud runtime;
- local and remote browser providers.

Every runtime negotiates capabilities. Runtime type never grants authority.

### U10 — Cross-device continuity

Kernux must preserve one task truth across:

- desktop;
- CLI/headless;
- web/control surface where offered;
- mobile companion;
- remote machines.

Required continuity includes:

- task/run state;
- approvals;
- notifications;
- artifacts/results;
- device trust and revocation;
- handoff without duplicating side effects;
- offline/degraded state;
- resumable observation streams;
- locale/timezone-consistent scheduling.

No device becomes a shortcut around kernel or organization policy.

### U11 — Team and enterprise governance

The platform must support:

- users, teams, projects, roles;
- policy intersection;
- runtime and connection ownership;
- shared tasks and artifacts;
- audit;
- SSO/identity-provider adapters when enterprise scope requires them;
- retention/legal-hold/compliance policy hooks without weakening user-visible boundaries;
- organization-managed connectors/secrets;
- administrative reporting;
- controlled external sharing;
- tenancy and data-boundary isolation.

Enterprise controls may tighten local policy but cannot silently weaken kernel hard constraints.

### U12 — Ecosystem and extension platform

Kernux should eventually support:

- stable SDKs only after contracts prove durable;
- signed/provenanced extension packages;
- capability declarations;
- secret/network/data declarations;
- compatibility metadata;
- permissions-delta review during update;
- extension health and revocation;
- curated registry/marketplace after governance is mature;
- organization-private registries;
- portable task templates, skills, policy packs, adapters, and benchmark journeys.

Extensions cannot self-grant authority.

### U13 — Reliability, observability, and supportability

Universal dependence requires production reliability, not demo reliability.

The platform must define and measure:

- startup/reconnect SLOs;
- task admission latency;
- action latency by capability;
- verified completion rate by task class;
- recovery rate after crash/disconnect;
- duplicate-side-effect rate;
- provider/runtime failure rate;
- context freshness failures;
- permission-abandonment rate;
- cost/usage where observable;
- artifact validation failure rate;
- background-task lateness/miss rate;
- storage/index growth and GC behavior.

Observability follows OpenTelemetry-compatible concepts where practical. Content export remains privacy-controlled.

Support bundles must be reviewable, redacted, and independent of always-on telemetry.

### U14 — Evaluation and anti-regression

Kernux must maintain a versioned capability benchmark suite.

It should cover:

- coding;
- browser research;
- authenticated web work;
- local computer work;
- document/spreadsheet/presentation production;
- data analysis;
- integration workflows;
- long-running/scheduled work;
- recovery;
- prompt injection;
- memory poisoning;
- permission boundaries;
- multi-agent comparison;
- cross-platform behavior;
- accessibility;
- cost/latency envelopes.

Every benchmark records environment, provider/model/runtime versions, exact candidate revision, and negative evidence.

No `best`, `most reliable`, or similar claim is allowed without a reproducible comparison protocol.

### U15 — Accessibility, internationalization, and user diversity

A universal platform must account for users outside one language, region, device, or ability profile.

Required design properties include:

- keyboard-complete desktop operation;
- semantic screen-reader support;
- non-color-only state;
- scalable text/zoom;
- RTL layout capability;
- locale-aware dates/numbers;
- timezone-safe scheduling;
- Unicode-safe file/task/context handling;
- multilingual model/input/output paths;
- IME support where platform stack requires it;
- accessibility metadata in Design Mode and computer-use surfaces.

Localization breadth can expand gradually; the architecture cannot assume English-only layout or Latin-only input.

## 4. Product modes without architecture forks

Kernux should expose capability through simple product modes, not separate incompatible products.

### Work

General knowledge and operational work across apps, files, web, and artifacts.

### Build

Software development, repositories, terminals, browsers, tests, and review.

### Research

Search, browse, collect, analyze, cite, compare, and publish evidence-backed outputs.

### Automate

Schedules, triggers, monitors, recurring routines, and background tasks.

### Operate

Cross-application business operations, dashboards, customer/admin workflows, and remote systems.

### Create

Documents, spreadsheets, presentations, reports, sites/apps, and media using shared artifact contracts.

These are UX lenses over the same Task, WorkUnit, Capability, Runtime, Context, Artifact, Evidence, and Event models.

## 5. Delivery generations

### Generation 1 — Trusted Agent Operating Environment

Existing P00-P15 remains the dependency-ordered path to the first public product.

Generation 1 proves:

- privileged capability kernel;
- desktop + CLI;
- local computer runtime;
- agent/model integration;
- browser/web execution;
- fleet orchestration;
- git/review;
- evidence/replay/recovery;
- runtime fabric;
- MCP/skills/tools;
- automation;
- mobile steering;
- teams/optional cloud;
- release hardening.

The initial market wedge remains developer/power-user workflows.

### Generation 2 — Universal work layer

Post-launch P16-P20 extends the proven kernel rather than rewriting it.

#### P16 — Work-product and artifact studio

Deliver first-class creation/editing/validation for documents, spreadsheets, presentations, reports, analysis artifacts, and lightweight sites/apps.

#### P17 — Integration and event fabric

Deliver scalable connector discovery, auth/account lifecycle, intent-based tool routing, event subscriptions, and broad app-category coverage.

#### P18 — Multimodal and cross-device interaction

Deliver voice/audio/image/screen input, richer mobile continuity, cross-device handoff, accessibility/i18n hardening, and visual computer-use fallback.

#### P19 — Proactive personal/work agent

Deliver user-governed routines, condition monitoring, personal/work relationship context, universal task capture, proactive-but-policy-bound suggestions, and cross-app execution.

#### P20 — Ecosystem and universal-scale maturity

Deliver signed extension distribution, private/public registries, mature enterprise policy packs, managed-service options, broad benchmark coverage, and large-scale reliability qualification.

Generation 2 work is never allowed to invalidate Generation 1 local-first, evidence, security, portability, or owner-sustainable invariants.

## 6. App-category coverage target

Kernux does not need a bespoke native connector for every vendor. It needs proven integration paths across every major category:

- email;
- calendar;
- contacts;
- messaging/chat;
- meetings;
- cloud storage;
- documents;
- spreadsheets;
- presentations;
- notes/knowledge bases;
- project/task management;
- source control/CI;
- design;
- CRM;
- support/helpdesk;
- marketing;
- analytics/BI;
- databases/data warehouses;
- finance/accounting;
- commerce;
- forms/surveys;
- e-signature;
- HR/people systems;
- developer/cloud platforms;
- social/content publishing;
- custom internal APIs;
- arbitrary web/desktop apps through browser/computer use when structured integration is unavailable.

Category coverage is proven with representative real integrations and protocol fixtures; marketing counts of connectors are not a substitute for reliability evidence.

## 7. Universal golden journeys

In addition to existing journeys, universal maturity requires evidence for at least:

1. **Morning work review** — inspect email/calendar/tasks, identify priorities, prepare a plan, and require approval before consequential outbound actions.
2. **Cross-app meeting follow-up** — ingest notes/transcript, update project records, draft/send approved follow-ups, schedule actions, and preserve provenance.
3. **Research to deliverable** — search/fetch/browse sources, analyze data, create a cited report and editable presentation/spreadsheet.
4. **Build and verify** — inspect a repository and live app, fan out candidate fixes, test, compare evidence, and prepare review.
5. **Business operation** — combine CRM, spreadsheet/database, email/chat, and web/admin systems under scoped credentials.
6. **Authenticated browser fallback** — complete a task when no API exists, with screenshots/trace and explicit approval for consequential steps.
7. **Desktop application fallback** — use accessibility/vision interaction when no structured interface exists without escaping granted scope.
8. **Scheduled monitor** — watch a condition over time, survive restart/offline windows, and notify only when the condition is satisfied.
9. **Cross-device handoff** — start on desktop, approve from mobile, inspect result elsewhere, with one run identity and no duplicate side effect.
10. **Offline/local core** — complete meaningful local work with Kernux cloud and telemetry unavailable.
11. **Provider failure** — replace a failed model/browser/tool/runtime provider without losing task truth or silently repeating side effects.
12. **Memory correction** — inspect a remembered fact, trace its source, correct/delete it, and prove future retrieval reflects the change.
13. **Project portability** — export/import on a clean installation and retain evidence/artifact integrity without secret leakage.
14. **Accessible operation** — complete a representative task keyboard-only and with semantic assistive-technology output.
15. **Owner-sustainable execution** — prove a complete useful path without founder-funded metered credentials.

## 8. Anti-gap ownership matrix

Every new feature or discovered requirement must answer all of these before it can be considered planned:

| Responsibility | Required owner/evidence |
| --- | --- |
| User outcome | Product job and acceptance journey |
| Capability | Provider-neutral capability contract |
| Authorization | Capability/Grant policy |
| Runtime | Execution owner and lifecycle truth |
| Context | Exact ContextBundle/source provenance |
| Secrets | Broker/reference ownership and scope |
| Data boundary | Local/external destination and retention |
| Cost | Explicit cost owner and budget behavior |
| Persistence | Durable vs derived state |
| Recovery | Crash/cancel/retry/reconnect semantics |
| Side effects | Operation identity/idempotency/ambiguity |
| Artifact | Format, validation, retention, export |
| Evidence | Strongest available completion observation |
| Accessibility | Keyboard/semantic/locale implications |
| Compatibility | Version/capability negotiation |
| Telemetry | Privacy-safe observability behavior |
| Extension impact | Provenance, permissions, update delta |
| Deletion | What is deleted, retained, or provider-owned |
| Cross-device | Authority and synchronization semantics |
| Test path | Deterministic fixture/golden journey/adversarial proof |

A requirement without an owner in this matrix is a planning gap.

## 9. Adoption strategy

Kernux should not demand that users migrate everything on day one.

Adoption sequence:

1. detect existing agents, tools, runtimes, browser, and projects;
2. complete one useful verified task;
3. connect additional apps only when a task needs them;
4. accumulate user-governed project/context/memory value;
5. introduce automation after interactive trust is established;
6. introduce team/shared policy after local semantics are proven;
7. offer managed convenience without making it mandatory.

The product wins when it becomes the trusted coordination layer around the user's existing digital life, not when it forces replacement of every native application.

## 10. Competitive design lessons

Current systems demonstrate several durable expectations for modern agent platforms:

- users expect agents to work across files, apps, and browsers, not only chat;
- finished editable work products matter as much as reasoning output;
- long-running/background work requires durable state and human steering;
- broad integrations require scalable auth/tool discovery rather than giant static tool lists;
- computer use needs both hosted and bring-your-own-machine paths;
- self-hosted/local execution remains strategically valuable;
- multi-agent work needs explicit orchestration and evidence;
- cost/credit models make owner-sustainable and BYOK/BYOC paths a product advantage.

These are design lessons only. External products remain references, not implicit code donors.

## 11. Completion rule

Kernux is not universally mature because it has many features.

Universal maturity requires:

- every capability family has an architectural owner;
- every consequential action has a bounded authorization path;
- every durable workflow has recovery semantics;
- every integration has auth/data/cost ownership;
- every output has an artifact/evidence model;
- every public claim has reproducible support;
- every core path has a sustainable execution model;
- every user can inspect, export, and delete Kernux-owned truth;
- the platform remains useful when any one external provider disappears.

**The goal is not to contain every application. The goal is to become the trusted operating layer that can use any application, runtime, agent, model, or tool on the user's behalf under one coherent policy, context, evidence, and recovery model.**
