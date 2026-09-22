# Platform Fabrics Architecture

> Normative module of the Kernux Platform Fabrics and Source Integration Plan.

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

### 2.1 Local privacy is the default trust boundary

All fabrics, providers, Capability Packs and user surfaces inherit `docs/canonical/LOCAL_PRIVACY_IMPLEMENTATION_PLAN.md`.

Hard rules:

- local task truth, memory, search, artifacts and evidence stay local by default;
- external AI/model/embedding/search processing is opt-in and separately classified from ordinary direct web access;
- local provider failure never silently activates cloud fallback;
- provider/tool manifests declare egress, data classes, secrets, retention and telemetry;
- every external transfer is attributable to an operation, destination, Grant and privacy mode;
- no Capability Pack may introduce a hidden hosted dependency or alternate privacy policy.

## 3. Correct source identities

Several similarly named sources must remain distinct in provenance and architecture.

### 3.1 Laya application

- source: `aayushch/laya` / `https://laya.aay.sh`;
- pinned planning revision: `5970a114241ee09cec09d571acf9ba52d27ae612`;
- founder states Kernux has permission to copy, modify, and use Laya source code;
- role: local-first event ingestion, cross-application action staging, action-card UX, workspaces, rules, briefings, hybrid search, budget visibility, dead-event recovery, and approval-oriented egress;
- public license observed during planning: Apache-2.0, with an upstream `NOTICE` file;
- architecture: local n8n event normalization -> Python engine -> Tauri/Svelte action-card UI -> approved egress;
- implementation-ready mapping: `docs/canonical/platform-fabrics/LAYA_INTEGRATION.md`.

Laya's dependencies and hosted/provider boundaries remain independently governed. n8n is not admitted merely because Laya uses it and must not become a mandatory Kernux runtime.

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

