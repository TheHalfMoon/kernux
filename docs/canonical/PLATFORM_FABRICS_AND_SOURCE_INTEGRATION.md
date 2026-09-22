# Platform Fabrics and Source Integration Plan

## 1. Status and authority

This document is a **planning amendment proposal** for Kernux. It does not authorize implementation, source import, model acquisition, dependency admission, phase skipping, or task-state mutation.

The live implementation frontier remains governed by `specs/CURRENT.md`, SpecGrain, Diffcipline, repository policy, exact-head evidence, and the dependency order in `docs/canonical/EXECUTION_MASTER_PLAN.md`.

At the time this amendment was prepared, canonical `main` was:

- commit: `2520554051273f28793e649c5aefc54271c78a70`;
- tree: `085bc4cd3da4a166630605383f48a13b96d68fcc`;
- active frontier: P02 secret-provider work through SG-000026;
- later platform-fabric work remains non-authorizing planning input.

This amendment exists to prevent future source integration from turning Kernux into a coupled collection of donor applications.

## 2. Product invariant

Kernux remains one product:

> **One intent -> one governed task graph -> any qualified agent or decision provider -> any approved capability -> any eligible runtime -> a verifiable outcome.**

Kernux is not a fork aggregation project. Donor projects may supply code, algorithms, UX patterns, tests, schemas, fixtures, or research, but Kernux-owned contracts define the product.

The permanent integration rule is:

> **Adopt capabilities behind Kernux contracts; never adopt donor architecture as product authority.**

## 3. Correct source identities

Several similarly named sources must remain distinct in provenance and architecture.

### 3.1 Laya application

- source: `aayushch/laya` / `https://laya.aay.sh`;
- role: local-first event ingestion, cross-application action staging, action-card UX, workspaces, rules, briefings, hybrid search, budget visibility, dead-event recovery, and approval-oriented egress;
- public license observed during planning: Apache-2.0;
- architecture: local n8n event normalization -> Python engine -> Tauri/Svelte action-card UI -> approved egress.

This source is **not** the same project as the `convaiinnovations/laya` model family.

### 3.2 Laya decision model family

- source: `convaiinnovations/laya` on Hugging Face;
- role: candidate future typed-decision/scoring provider;
- architecture authority: none;
- output authority: advisory only.

### 3.3 Other current decision-provider candidates

- `TheoLeeCJ/SemIf` — open semantic-if research implementation with direct option scoring, shared-state reuse, multiple local backends, calibration work, and auditable fixtures;
- `Mapika/decider` — typed choice/score/boolean decision models, large option sets, long contexts, schema caching, and a TypeSafe-compatible wire surface;
- `bespokelabs/Bespoke-Nimble-9B` — Qwen3.5-9B LoRA for evidence-grounded booleans, choices, and rubric scores; current public model card documents bounded option/context limits and CUDA/BF16 execution.

These are providers or research references. None becomes policy, authorization, or verification authority.

### 3.4 Tool and integration sources

- `superdesigndev/treg` — capability/tool catalog, credential injection, endpoint/CLI/skill registry, team-scoped sharing, pricing metadata, and intent-oriented discovery;
- `tinyfish-io/*` — web acquisition, AgentQL semantic page interaction, browser automation integrations, and BigSet-style live structured datasets;
- `wonderwhy-er/DesktopCommanderMCP` and Desktop Commander remote patterns — filesystem, process, terminal, search, document/data, and remote-device execution primitives.

### 3.5 Kernux-adjacent repositories owned by the founder

The founder's GitHub portfolio is a source pool, not an automatic dependency pool.

High-value candidate roles include:

- Morize -> governed durable memory provider and memory-evaluation concepts;
- SpecGrain -> bounded decomposition and WorkPacket preparation;
- Diffcipline -> exact-change proof-before-done;
- Ascout -> changed-code verification receipts and coverage/execution evidence;
- Winds -> independently observed candidate/workspace execution and verification;
- Ineractive -> software-building capability pack;
- Himsat -> conversation/media intelligence capability pack.

Domain products such as healthcare, HR, real-estate, games, and other vertical applications must not leak their domain architecture into Kernux Core merely because the code is available. Only separately qualified general-purpose primitives may move inward.

## 4. Permanent architecture tiers

All future functionality must have exactly one primary tier.

### Tier 0 — Authority Kernel

Owned by `kernuxd`.

Responsibilities:

- capability and Grant enforcement;
- local identity and runtime enrollment;
- secret references and brokered secret use;
- privileged filesystem/process/runtime operations;
- approval enforcement;
- durable authority-relevant event/evidence persistence;
- artifact CAS;
- operation identity, retry ambiguity, and recovery truth.

No model, agent, plugin, browser page, connector, decision provider, capability pack, or UI may bypass Tier 0.

### Tier 1 — Core semantic contracts

Provider-neutral Kernux-owned types and state machines:

- Task / WorkUnit / Run;
- Capability / Grant / Approval;
- Runtime / Operation;
- ContextItem / ContextBundle;
- Artifact / Evidence / Event;
- Provider identity and revision;
- cost/budget ownership;
- data-boundary metadata;
- cancellation, retry, replay, and recovery semantics.

Tier 1 must remain small enough to reason about and version.

### Tier 2 — Platform Fabrics

Reusable subsystems over Tier 1:

1. Agent and Model Fabric
2. Decision Fabric
3. Tool and Integration Fabric
4. Web and Live-Data Fabric
5. Computer Fabric
6. Context and Memory Fabric
7. Runtime Fabric
8. Automation and Event Fabric
9. Evidence and Verification Fabric
10. Artifact Fabric

A fabric owns contracts, routing, lifecycle, and evidence expectations for one capability family. It does not own kernel authority.

### Tier 3 — Providers and adapters

Examples:

- OpenAI / Anthropic / Gemini / local model adapter;
- SemIf / Decider / Nimble / Laya-model adapter;
- MCP / OpenAPI / CLI / Skill / A2A adapter;
- AgentQL / TinyFish / Playwright provider;
- Desktop Commander-derived computer provider;
- Morize memory provider;
- Ascout / Winds / Diffcipline verification providers.

Providers are replaceable and versioned. Provider disappearance must not invalidate durable Kernux task truth.

### Tier 4 — Capability Packs

Composable product-level bundles that reuse the same fabrics:

- Software Builder pack;
- Research pack;
- Conversation Intelligence pack;
- Data pack;
- Security pack;
- Healthcare pack;
- future domain packs.

A pack may bundle agents, prompts, tools, templates, policies, UI panels, verifiers, and data schemas. It may not introduce an alternate permission system, alternate secret store, alternate task truth, or hidden background executor.

### Tier 5 — User surfaces

- desktop workspace;
- CLI/headless;
- mobile;
- optional web/team console;
- future focused surfaces such as Inbox or Data Workspace.

All surfaces operate the same Task/WorkUnit/Capability/Approval/Evidence contracts.

## 5. Decision Fabric

### 5.1 Purpose

Use small or specialized decision systems for frequent bounded decisions without forcing a generative LLM to produce prose that software immediately reparses.

Representative workloads:

- route a WorkUnit;
- classify priority/category/persona;
- choose a tool/provider candidate;
- decide whether evidence supports a bounded proposition;
- rank or score candidates;
- recommend retry/escalation;
- detect likely out-of-domain or near-tie cases;
- provide advisory moderation/guardrail signals.

### 5.2 Canonical request contract

A future `DecisionRequest` must carry at least:

- request and task/work-unit identity;
- state/context reference or bounded inline state;
- question schema;
- allowed answer types;
- option descriptions/rubrics;
- trust/provenance metadata for state inputs;
- latency/cost/device/data-boundary constraints;
- required calibration profile;
- cancellation/deadline;
- provider allow/deny policy.

Minimum answer types:

- BOOLEAN;
- CHOICE;
- ORDINAL_SCORE;
- RANK;
- ABSTAIN / NO_FIT where supported.

Do not force every provider to support every type.

### 5.3 Canonical response contract

A `DecisionResult` must record:

- provider, model, revision and artifact digest;
- exact request/schema digest;
- selected value;
- probability distribution or provider-native score semantics;
- calibration profile/revision;
- OOD/unsupported/abstain signal when available;
- latency, device and resource metadata;
- local/external data boundary;
- warnings such as truncation, option-limit pressure, quantization, fallback, or degraded mode.

### 5.4 Authority rule

A DecisionResult is an observation.

It cannot:

- grant permission;
- lower consequence;
- approve its own side effect;
- promote memory trust;
- mark verification complete;
- bypass a human/org policy;
- silently convert uncertainty into allow.

### 5.5 Router

The Decision Router chooses among eligible providers using deterministic constraints first:

1. capability/type support;
2. language;
3. context length;
4. option count;
5. hardware/runtime availability;
6. privacy/data boundary;
7. latency budget;
8. cost owner and budget;
9. qualified workload-specific calibration;
10. health and recent error state.

Probabilistic routing may rank already-eligible providers, but cannot make an ineligible provider eligible.

### 5.6 Disagreement and escalation

Support explicit disagreement as a signal.

Examples:

- low-risk + strong agreement -> continue;
- near tie -> abstain or escalate;
- provider disagreement -> route to a qualified stronger provider or generative verifier;
- high-consequence task -> policy may require human approval regardless of confidence.

Thresholds are workload-specific and versioned. There is no global confidence cutoff.

### 5.7 Evaluation

Every decision provider must be qualified on Kernux-owned fixtures covering:

- accuracy/task utility where meaningful;
- proper calibration metrics;
- near ties and abstention;
- multilingual/non-Latin inputs;
- option-count boundaries;
- long/short context boundaries;
- adversarial/prompt-injected text;
- OOD and distribution shift;
- repeatability;
- quantization/runtime changes;
- CPU/GPU latency and memory;
- disagreement/escalation behavior.

Upstream benchmark claims are research input, not Kernux evidence.

## 6. Tool and Integration Fabric

### 6.1 Treg-derived lesson

Kernux should adopt the useful pattern, not the donor product boundary:

- discover tools by what they can do;
- keep credentials out of agent prompts where possible;
- support HTTP endpoints, CLIs, skills, MCP, OpenAPI and native adapters;
- attach cost and health metadata;
- allow team/private catalogs.

### 6.2 Canonical ToolDescriptor

Every tool/provider entry must declare:

- stable tool/provider identity and version;
- capability names;
- input/output schema;
- effect class: READ, WRITE, EXTERNAL_SIDE_EFFECT, DESTRUCTIVE, PRIVILEGED;
- auth requirements and SecretHandle bindings;
- allowed network destinations;
- data sent externally;
- retention assumptions;
- cost model and payer;
- rate/usage limits;
- idempotency semantics;
- reversibility/compensation semantics;
- runtime requirements;
- timeout/cancellation behavior;
- evidence returned;
- provenance/source.

### 6.3 Discovery

Agents must not receive thousands of raw tools.

Routing sequence:

Intent -> capability retrieval -> policy filter -> account/runtime/data-boundary filter -> cost/health filter -> bounded candidate set -> provider selection.

The tool catalog is context, not authority.

### 6.4 Credential ladder

Preferred order:

1. explicit user/org connection;
2. user/org BYOK credential;
3. local/self-hosted unauthenticated capability;
4. explicitly selected paid managed provider with an identified payer.

Founder-funded or hidden shared credentials must not be required for core usefulness.

### 6.5 Drift and revocation

The registry must detect and surface:

- auth expiry/revocation;
- schema drift;
- capability removal;
- permission/scope changes;
- pricing changes;
- provider outage;
- incompatible version changes.

Cached capability metadata must have freshness and revision semantics.

## 7. Automation and Event Fabric

The Laya application contributes strong product patterns here.

### 7.1 External IntegrationEvent

Do not overload the kernel's audit Event type.

Define a separate normalized external `IntegrationEvent` with:

- source/platform/account;
- source event id;
- observed/source timestamps;
- actor/entity references;
- subject/object;
- payload references;
- sensitivity and trust;
- dedupe key;
- original source reference;
- connector revision.

### 7.2 Event pipeline

Receive -> normalize -> dedupe -> classify -> associate -> enrich -> propose -> stage -> approve/policy -> execute -> reconcile -> evidence.

Each transition is durable where needed and restart-safe.

### 7.3 ActionProposal

Laya-style action cards become a general Kernux `ActionProposal`, not a separate permission system.

An ActionProposal records:

- proposed capability call;
- why it is proposed;
- source events/context;
- side-effect preview;
- destination/account;
- data/credential use;
- consequence class;
- estimated cost;
- evidence expected;
- approval state;
- expiry/staleness.

Approval compiles to ordinary Kernux grants/operation authority.

### 7.4 Rules

Rules may include deterministic and model-evaluated conditions.

Requirements:

- versioned rule definitions;
- dry-run/simulation;
- firing log;
- rate limits and suppression;
- loop prevention;
- action budget;
- explicit side-effect policy;
- no AI-evaluated condition can self-authorize a consequential action.

### 7.5 Briefings and inbox

Daily briefings, cross-platform summaries, and attention inboxes are derived views over ordinary events/tasks/context. They are not canonical truth stores.

## 8. Web and Live-Data Fabric

### 8.1 Web acquisition hierarchy

Prefer:

1. native structured API/WebMCP;
2. read-only search/fetch;
3. deterministic DOM/accessibility/CDP;
4. semantic extraction/AgentQL-style provider;
5. stateful browser automation;
6. visual coordinate fallback.

Read-only acquisition must remain separate from stateful authenticated browser authority.

### 8.2 LiveDataObject

Adopt the strongest BigSet concept as a Kernux-owned object.

A LiveDataObject contains:

- schema and schema revision;
- entity/row identity;
- field-level source claims;
- source URL/object reference and retrieval time;
- freshness/TTL;
- confidence/verification state;
- contradiction/conflict state;
- deduplication lineage;
- refresh policy;
- incremental update history;
- creator task/run;
- data-boundary and retention metadata;
- export/query projections.

### 8.3 Truth ownership

Live datasets are not Morize memories.

- LiveDataObject owns structured changing datasets.
- Morize/MemoryProvider may index or summarize them.
- ContextBundle may reference selected rows/claims.
- source systems remain authoritative where applicable.
- derived indexes remain rebuildable.

### 8.4 Refresh semantics

Scheduled refresh must define:

- full vs incremental refresh;
- conditional fetch/no-change;
- deleted/disappeared entities;
- source disagreement;
- stale rows;
- partial failure;
- schema migration;
- cancellation/retry;
- cost budget;
- provenance completeness.

## 9. Computer Fabric

Desktop Commander-derived primitives fit behind Kernux policy.

Provider-neutral capabilities include:

- file read/write/list/search;
- process start/input/output/terminate;
- PTY/terminal sessions;
- clipboard;
- app launch;
- document/data helpers;
- accessibility/native UI observation;
- screenshot/visual fallback;
- remote-device relay.

The provider never receives ambient authority merely because it can technically reach the machine.

Remote execution must preserve:

- enrolled device identity;
- execution-host process truth;
- reconnect without implicit restart;
- bounded delegated grants;
- operation identity;
- redaction and audit;
- no stale PID/connection assumptions.

## 10. Context and Memory Fabric

### 10.1 Storage-of-record matrix

To prevent dual truth:

| Information | Canonical owner |
| --- | --- |
| Task/WorkUnit/Run state | Kernux task store |
| Grants/policy/approvals | kernuxd policy store |
| Runtime operation truth | execution host + Kernux runtime ledger |
| Secrets | approved secret provider / OS credential store |
| Artifacts | Kernux artifact CAS / explicit external artifact source |
| Evidence | Kernux evidence ledger |
| Durable user/project memory | MemoryProvider, with Morize as a candidate provider |
| Live structured datasets | LiveDataObject store |
| Search/vector/graph indexes | derived, rebuildable projections |
| External SaaS records | source system; Kernux caches are derived unless explicitly imported |

### 10.2 Morize boundary

If Morize is adopted, it should implement the MemoryProvider role rather than replacing Kernux task/evidence state.

Useful Morize concepts:

- temporal truth;
- contradiction/supersession;
- evidence-linked memory;
- memory firewall;
- explainable recall;
- branch/diff/merge of memory state;
- memory evaluation.

No memory record authorizes a capability.

## 11. Evidence and Verification Fabric

Generalize beyond coding.

### 11.1 Provider roles

- SpecGrain -> work preparation/decomposition evidence;
- Diffcipline -> exact-change completion discipline;
- Ascout -> changed-code verification receipt;
- Winds -> independently observed candidate/runtime verification;
- artifact validators -> DOCX/XLSX/PPTX/PDF/data validation;
- browser verifiers -> DOM/state/source confirmation;
- integration verifiers -> external object/message/record confirmation;
- human decisions -> explicit HUMAN_DECIDED evidence.

### 11.2 Evidence claim model

Every completion claim should identify:

- claim;
- subject revision/run/object;
- authority class;
- observer/provider;
- method/check;
- timestamp;
- inputs;
- result;
- omissions;
- negative evidence;
- reproducibility metadata.

A provider's PASS means only what that provider actually observed.

### 11.3 Completion policy

`VERIFIED_COMPLETE` is computed from task-specific required evidence. It is never inferred from an agent saying "done".

## 12. Capability Packs

Capability Packs prevent domain products from bloating Kernux Core.

Each pack manifest declares:

- identity/version/source;
- required Kernux capabilities;
- agents/models/decision providers;
- tools/connectors;
- runtime requirements;
- secrets;
- network destinations;
- data classes;
- background behavior;
- UI extensions;
- evidence requirements;
- policy pack;
- provenance and license metadata.

Examples:

- Ineractive-derived Software Builder pack;
- Himsat-derived Conversation Intelligence pack.

A pack may be disabled/uninstalled without corrupting Kernux Core state.

## 13. Reliability and failure semantics

Every fabric/provider must define the following before production qualification:

- timeout;
- cancellation;
- retry class;
- idempotency key behavior;
- ambiguous side effect;
- reconciliation;
- provider unavailable;
- provider returns malformed/partial data;
- provider revision changed;
- credential revoked;
- device disconnected;
- local storage full;
- OOM/resource exhaustion;
- clock/timezone error for scheduled work;
- crash between external effect and local commit.

Exactly-once external execution must not be claimed unless the external system actually provides the required semantics. Default to durable operation identity, at-least-once-safe design, reconciliation, and explicit ambiguity.

Use circuit breakers, bounded retries, health cooldowns, and provider fallback without duplicating side effects.

## 14. Security and privacy closure

Required permanent invariants:

- data cannot authorize itself;
- tool output cannot expand tool authority;
- connector events cannot self-authorize action;
- decision confidence is never permission;
- secrets remain handles where possible;
- destination-bound secret use is preferred;
- tool manifests declare network and data egress;
- cross-project/space/account boundaries fail closed;
- external content carries trust/taint provenance;
- provider telemetry is opt-in/declared and redacted;
- plugin/provider updates expose permission deltas;
- remote-device compromise is a separate trust domain;
- rules cannot create unbounded loops or uncontrolled external publishing.

## 15. Cost and sustainability

Every metered capability records:

- payer;
- estimated cost;
- actual observable cost;
- budget scope;
- hard/soft cap;
- fallback behavior;
- no-payer behavior.

Core golden journeys require at least one path that is local, self-hosted, BYOK, BYOC, organization-funded, subscription-backed, or explicitly user-funded.

Do not copy Treg's provider-funded catalog economics as a hidden Kernux obligation. A future marketplace may exist only with explicit positive unit economics and payer visibility.

## 16. Provenance and license admission

Founder-supplied permission is important but does not replace exact source admission.

Every import must capture:

- source repository/model/dataset;
- immutable revision;
- source and destination path map;
- imported blob/model/artifact digests;
- public license;
- private permission evidence/reference where applicable;
- contributor/NOTICE obligations;
- embedded third-party dependencies/assets;
- model base-weight license;
- tokenizer/config licenses;
- dataset/training-data obligations if imported;
- trademark/branding separation;
- hosted-service terms separated from source-code rights;
- transformation class;
- characterization tests.

Planning observations requiring special care:

- Treg's public LICENSE contains Apache-2.0 plus additional hosted/commercial service restrictions; code import for a commercial hosted/embedded Kernux path requires the founder's claimed private permission to explicitly cover the intended use, and third-party obligations still require review.
- BigSet's public repository is AGPL-3.0; do not copy AGPL-covered implementation into Apache-2.0 Kernux absent separately recorded permission/rights that actually cover the imported material and intended distribution.
- Desktop Commander's public local MCP repository is MIT, while current public Remote Desktop Commander documentation states that its hosted service implementation is not open source; any claimed private authorization for remote implementation must be recorded separately before import.
- Model adapters require base-model and adapter licenses to be checked independently.

## 17. Observability

Required fabric-level metrics include:

- task admission and verified-completion rate;
- provider selection/fallback rate;
- decision calibration/drift and abstention;
- tool discovery precision and failed-route rate;
- browser acquisition success and stale-source rate;
- LiveData freshness, duplicate rate and provenance completeness;
- external action ambiguity/duplicate-side-effect rate;
- approval abandonment;
- runtime reconnect/recovery;
- memory retrieval freshness/conflict rate;
- verification false-green investigations;
- latency and resource use;
- cost by task/fabric/provider.

Support bundles must be redacted, reviewable, and not depend on always-on cloud telemetry.

## 18. UX convergence

Kernux should not become a chat clone.

Primary workspace concepts:

- universal task/intention input;
- Work Canvas that can render browser/app/document/data/terminal views;
- Task Graph and agent/fabric status;
- Action Inbox for event-driven proposals;
- Data Workspace for LiveDataObjects;
- Approval panel showing exact effect, destination, data and credential use;
- Evidence panel;
- Context/provenance inspector;
- cost/budget panel;
- runtime/device panel;
- durable event timeline and replay.

Laya-style Action Cards are a valuable UX pattern for the Action Inbox, but cards are a projection of Task/Event/ActionProposal state, not a new state authority.

## 19. Evaluation program

Add fabric-specific benchmark families to the existing Kernux evaluation strategy.

### Decision

- calibration;
- OOD;
- disagreement;
- multilingual;
- option counts;
- long context;
- latency/memory;
- hardware/quantization drift.

### Tool routing

- capability retrieval precision/recall;
- wrong-tool rate;
- permission filtering;
- provider outage;
- schema drift;
- prompt/tool-description injection;
- giant-catalog behavior.

### Event/action

- dedupe;
- critical-event miss rate;
- false priority;
- bad association;
- unwanted action-proposal rate;
- rule-loop resistance;
- approval/reconciliation correctness.

### Web/live data

- source recall;
- field-level provenance;
- duplicate entities;
- stale rows;
- contradictory sources;
- refresh correctness;
- partial-failure recovery.

### Computer/runtime

- cross-platform file/process/PTY behavior;
- reconnect;
- cancellation;
- destructive-action containment;
- remote-device identity.

### Memory

- recall utility;
- stale-memory resistance;
- contradiction handling;
- poisoning;
- deletion/redaction;
- provenance.

### Evidence

- false green;
- omitted-check visibility;
- exact-revision binding;
- replay;
- provider disagreement.

Public "best" or market-leadership claims require preregistered reproducible comparative evaluation.

## 20. Dependency-correct delivery mapping

Do not create a parallel roadmap. Refine the existing phases.

### P02

Continue the current secret-provider and privileged-kernel work unchanged.

No new donor integration from this document may interrupt P02.

### P05 — Agent/model layer

Refinement should add:

- DecisionProvider contract;
- deterministic fake DecisionProvider;
- provider adapters only after qualification;
- workload-specific calibration registry;
- disagreement/escalation policy;
- native-agent use of DecisionResult as advisory observation.

Candidates: SemIf, Decider, Bespoke Nimble, convaiinnovations Laya.

### P06 — Web

Refinement should add/retain:

- read-only WebAcquisitionProvider;
- TinyFish/AgentQL-derived semantic provider;
- separation of acquisition from stateful action;
- source/freshness/content-digest lineage;
- deterministic offline fixtures.

### P11 — Tools/MCP/skills

Refinement should define the baseline ToolDescriptor, local registry, bounded discovery, auth bindings, side-effect metadata and provider execution contracts.

Treg-derived code/patterns may be considered only behind these contracts.

### P12 — Durable automation

Refinement should ensure durable event-triggered execution, scheduler semantics, dedupe, missed-run policy, operation identity and reconciliation are sufficient for later Action Inbox work.

### P16 — Artifacts

Data/artifact views must support LiveData exports and reproducible analysis packages, without making LiveData canonical ownership an artifact concern.

### P17 — Integration and event fabric

Expand the current P17 plan to cover:

- scalable capability/tool catalog;
- connected account registry;
- normalized IntegrationEvent;
- provider drift/health/cost;
- event subscriptions/webhooks;
- LiveDataObject contract/store;
- web/connector population and refresh;
- private/org registries;
- generated adapters.

### P19 — Proactive work layer

Refine to include:

- Action Inbox / ActionProposal;
- daily briefing and temporal summaries;
- rules with simulation/firing log;
- cross-app association/coherence;
- correction learning with explicit user control;
- proactive suggestions that remain non-authoritative.

### P20 — Ecosystem

Use Capability Pack manifests and signed/provenanced provider packages rather than allowing arbitrary plugins to invent new authority surfaces.

## 21. Candidate future planning units

These are candidate refinements only. They are not canonical task-state mutations.

- P05: DecisionProvider contract and fake provider.
- P05: calibration/evaluation registry and escalation policy.
- P05: first two heterogeneous local decision-provider qualifications.
- P11: ToolDescriptor and local capability registry.
- P11: credential-bound HTTP/CLI/MCP adapters.
- P17: IntegrationEvent and connector lifecycle contract.
- P17: LiveDataObject storage/provenance/refresh contract.
- P17: Treg-style catalog/provider selection behind Kernux policy.
- P19: ActionProposal and Action Inbox.
- P19: rules, briefings, association and correction-learning qualification.
- P20: Capability Pack manifest/signing/revocation.

SpecGrain must assign final IDs, dependencies, scopes, acceptance conditions and evidence.

## 22. Anti-gap admission checklist

No new capability is planning-complete until all applicable rows have an owner:

| Question | Required answer |
| --- | --- |
| What user outcome does this enable? | Golden journey / acceptance |
| Which tier owns it? | Kernel / contract / fabric / provider / pack / surface |
| What is the provider-neutral contract? | Schema/API/state machine |
| Who authorizes it? | kernuxd policy path |
| What data leaves the machine? | Explicit data boundary |
| Which secret is used and where? | SecretHandle + destination |
| Who pays? | Cost owner and budget |
| What is canonical vs derived state? | Storage-of-record declaration |
| How is it cancelled? | Cancellation semantics |
| How is it retried? | Idempotency/ambiguity/reconciliation |
| What happens when provider disappears? | Fallback/degraded state |
| What happens on crash/restart? | Recovery |
| How is drift detected? | Version/health/freshness |
| How is it observed? | Logs/metrics/traces with redaction |
| How is it proven? | Evidence provider and completion policy |
| How is it benchmarked? | Fixtures/metrics/negative evidence |
| Is it accessible and internationalized? | Keyboard/semantic/RTL/locale where applicable |
| What source was used? | Immutable provenance |
| What licenses/permissions apply? | Source + dependencies + private permission |
| Can it be removed/replaced? | Migration/uninstall/provider-loss path |

If any applicable row is unanswered, the capability remains planning debt.

## 23. Explicit non-goals

This plan does not authorize:

- wholesale copying of any donor repository;
- combining decision-model weights;
- treating model probability as permission or proof;
- creating a second task engine inside a Capability Pack;
- duplicating Morize memory truth inside Kernux task storage;
- duplicating Kernux task/evidence truth inside Morize;
- exposing every tool to every agent;
- storing raw long-lived credentials in prompts or ordinary SQLite rows;
- making TinyFish, Treg, Desktop Commander remote, or any hosted provider a mandatory Kernux dependency;
- making AGPL or source-available code silently part of Apache-2.0 Kernux;
- replacing current P02 work with long-horizon integration work;
- claiming universal superiority without reproducible evidence.

## 24. Planning exit criteria

This amendment is ready for canonical adoption only when:

1. the existing Universal Platform Plan and task registry reference or absorb these ownership boundaries;
2. Issues tracking Laya-model/TinyFish planning distinguish the Laya application from the Laya model family;
3. P05 refinement owns Decision Fabric obligations;
4. P11/P17 refinement owns Tool/Integration Fabric obligations;
5. P17 owns LiveData and IntegrationEvent semantics;
6. P19 owns Action Inbox/rules/briefing semantics;
7. storage-of-record ownership is represented in canonical architecture;
8. donor/provenance policy can represent private permission separately from public license;
9. no current P02 implementation authority is widened;
10. a planning completeness audit finds no orphan requirement present only in this document.

## 25. Final architecture rule

> **Kernux owns authority, task truth, contracts, routing policy and evidence semantics. Fabrics own reusable capability families. Providers implement replaceable mechanisms. Capability Packs compose product experiences. Donors never become architecture authority merely because their source is available.**
