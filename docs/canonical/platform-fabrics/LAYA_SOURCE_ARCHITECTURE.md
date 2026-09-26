# Laya Source and Architecture Plan

> Normative planning module under the Kernux Platform Fabrics and Source Integration Plan.
>
> This module freezes source authority, donor boundaries, dependency disposition, canonical receiving contracts, and storage ownership. Lifecycle/security/product behavior and delivery/qualification are defined in dependent planning modules.

## 1. Status and authority

This document makes the Laya application integration path implementation-ready without changing the current Kernux implementation frontier.

It does **not** authorize immediate source import, dependency admission, phase skipping, task-state mutation, or merge of unrelated work. Live authority remains:

1. current `main`;
2. `specs/CURRENT.md`;
3. `specs/tasks.md`;
4. applicable canonical architecture/security/privacy/economics/provenance policy;
5. the active SpecGrain/WorkPacket;
6. exact-head evidence and review.

At plan preparation time:

- Kernux canonical main: `937fd04fa900639ede99641a772354b3643c271f`;
- active implementation frontier: `KX-P02-S06-T01`;
- Laya application source: `aayushch/laya`;
- pinned planning revision: `5970a114241ee09cec09d571acf9ba52d27ae612`;
- public Laya license observed at that revision: Apache-2.0;
- Laya includes a repository `NOTICE` file that must be preserved when applicable;
- founder states that Kernux has permission to copy, modify, and use Laya source code.

The founder authorization and the public Apache-2.0 license do not relicense Laya's dependencies, bundled runtimes, third-party assets, trademarks, hosted services, model providers, or external integration systems.

## 2. Product decision

Laya is a **donor and reference subsystem**, not a replacement architecture.

Kernux adopts the strongest Laya product patterns while keeping Kernux-owned authority, task truth, evidence, secrets, runtimes, provider contracts, memory boundaries, privacy controls, and cost ownership.

Permanent rule:

> **Reuse Laya implementation aggressively where provenance and dependency admission allow; preserve Kernux contracts as the only product authority.**

Laya must not introduce:

- a second permission model;
- a second secret authority;
- a second task/run truth store;
- a second durable evidence authority;
- a mandatory external AI provider;
- a mandatory integration vendor;
- hidden cloud fallback;
- founder-funded per-use runtime cost;
- an alternate background executor outside Kernux task/capability policy.

## 3. What Kernux adopts from Laya

Kernux should adopt or adapt these Laya concepts:

- normalized cross-application event ingestion;
- Action Card / Action Inbox experience;
- card workspaces for complex tasks;
- staged actions before external side effects;
- cross-application association and entity trace;
- Coherence-style search and narrative synthesis;
- Omni-style rolling work-state summaries;
- daily briefings;
- classification/routing feedback loops;
- processing rules;
- connector health and account UX;
- audit and dead-event recovery;
- bookmarks/pins and deferred attention;
- cost/usage visibility;
- multi-persona specialization as a UX/routing pattern;
- local model and installed-agent inference options;
- preview-before-send egress;
- derived analytics over events/tasks/actions.

Kernux should **not** adopt these assumptions as canonical architecture:

- n8n as the sole integration gateway;
- Python/FastAPI as the canonical orchestration authority;
- LiteLLM as the public model contract;
- ChromaDB as durable truth;
- Laya SQLite tables as Kernux canonical task/evidence truth;
- OS keyring access that bypasses the Kernux secret broker;
- an LLM classification result as permission;
- a UI approval button as the actual authorization mechanism;
- single-user assumptions as a permanent platform constraint.

## 4. Source disposition map

| Laya source area | Kernux disposition | Receiving Kernux owner |
| --- | --- | --- |
| `ui/src/routes/feed` | adapt/port | Action Inbox desktop surface |
| `ui/src/routes/workspace` | adapt/port | Task/WorkUnit workspace surface |
| `ui/src/routes/coherence` | adapt/port | Context/relationship trace surface |
| `ui/src/routes/omni` | adapt/port | Derived current-state/attention surface |
| `ui/src/routes/dashboard` | reference/adapt | local analytics/operations surface |
| `ui/src/routes/settings` | reference/adapt | connector/provider/privacy settings |
| Action Card components | adapt | `ActionProposal` rendering |
| Engine event ingestion | port semantics | Integration and Event Fabric |
| Router/classifier pipeline | adapt | Decision/Agent Fabric routing |
| Persona workers | reference/adapt | agent-role templates and routing |
| Context association | port/adapt | Context and Memory Fabric |
| Coherence search | port/adapt | hybrid local retrieval + relationship trace |
| Omni synthesis | port/adapt | derived summary projection |
| Processing rules | port/adapt | Automation and Event Fabric |
| Egress module | port behavior only | capability calls through kernuxd policy |
| Audit/dead-event logic | port/adapt | Event/Evidence + recovery projections |
| Budget tracking | adapt | provider/runtime cost-owner telemetry |
| Coding-agent adapters | compare/reuse selectively | Agent Fabric adapters |
| Tauri shell/process management | reference/selective reuse | desktop/runtime fabric; Orca remains another donor |
| SQLite schema | reference only | map to Kernux-owned stores |
| ChromaDB integration | optional derived index | rebuildable search projection only |
| LiteLLM integration | reference/optional provider adapter | Agent/Model Fabric |
| n8n workflows | reference/port normalization logic | native/generated connector adapters |
| n8n runtime | optional external provider only | never a mandatory Kernux dependency |

No path is copied until a future import Grain freezes exact source files, destination files, transformation class, license/notice evidence, characterization tests, and dependency disposition.

## 5. Third-party and dependency boundary

Laya's Apache-2.0 project license does not automatically apply to all dependencies.

Every import wave must classify each dependency as one of:

- already admitted Kernux dependency;
- new direct dependency requiring provenance/license admission;
- optional external provider;
- build/dev-only dependency;
- source logic to port without retaining the dependency;
- rejected dependency.

Specific mandatory decisions:

### 5.1 n8n

n8n is **not** a mandatory Kernux runtime.

Kernux may:

- study Laya's n8n workflow structure;
- port normalization/execution logic into Kernux-owned connectors;
- optionally support user-owned n8n through an adapter if separately licensed and admitted.

Kernux must not:

- bundle or embed n8n merely because Laya uses it;
- depend on n8n for core product correctness;
- make Kernux connector breadth depend on one integration vendor.

### 5.2 LiteLLM

LiteLLM may be used only as an optional provider implementation after ordinary dependency admission.

The Kernux public model/agent contract remains provider-neutral.

### 5.3 ChromaDB / embeddings

Vector indexes remain derived and rebuildable.

No vector database or embedding provider becomes canonical memory, task truth, evidence truth, or authorization state.

### 5.4 OS keyring

Laya keyring behavior must map to Kernux SecretHandle/secret-provider contracts.

Agents, UI code, and connectors must not receive reusable plaintext secrets merely because donor code did so.

## 6. Canonical contracts

### 6.1 IntegrationEvent

A normalized external event distinct from the kernel audit `Event`.

Required fields:

- `integration_event_id`;
- source platform/provider;
- connector identity and revision;
- account/tenant identity;
- source-native event id;
- event type;
- observed timestamp;
- source timestamp when available;
- actor references;
- subject/object references;
- payload artifact/reference;
- trust class;
- sensitivity/data classification;
- original source reference;
- dedupe key;
- causal/correlation references;
- freshness/staleness state;
- ingestion run identity.

Unknown event types remain non-authorizing data.

### 6.2 ActionProposal

The canonical replacement for a Laya Action Card's executable meaning.

Required fields:

- proposal identity;
- source Task/WorkUnit and IntegrationEvents;
- proposed capability/action;
- canonical resource/destination;
- account identity;
- proposed payload/artifact;
- reason/explanation;
- side-effect preview;
- data classes to leave the local boundary;
- SecretHandle requirements;
- consequence class;
- reversibility/compensation metadata;
- expected evidence;
- estimated provider/runtime cost and payer;
- expiry/staleness boundary;
- proposal revision;
- approval state;
- execution operation identity when admitted.

An ActionProposal is **not authority**.

Approval/policy evaluation must compile to ordinary Kernux capability/Grant semantics.

### 6.3 ContextAssociation

A derived relationship between events, tasks, artifacts, entities, people, tickets, messages, meetings, repositories, files, and other context.

It must record:

- association identity;
- endpoints;
- source/provenance;
- method;
- confidence/score semantics if probabilistic;
- confirmed/rejected/corrected state;
- created/updated timestamps;
- rule/model revision;
- user correction lineage.

Association never grants authority and never replaces source-system identity.

### 6.4 AttentionItem

A derived inbox item pointing to canonical task/event/proposal state.

It may contain:

- urgency;
- priority;
- rationale;
- deadline/freshness;
- unread/pinned/snoozed state;
- suggested next action.

Attention state is presentation metadata, not task truth.

### 6.5 BriefingView

A reproducible derived projection over selected events/tasks/proposals/calendar/context.

It records:

- query/window;
- source revisions;
- generation method/provider;
- omissions/warnings;
- output artifact;
- created time.

A briefing does not become durable memory merely because it exists.

### 6.6 WorkspaceView

A Card Workspace becomes a projection over ordinary Kernux objects:

- Task;
- WorkUnits;
- Runs;
- ContextBundle;
- Artifacts;
- Evidence;
- ActionProposals;
- approvals;
- events;
- agent sessions.

No workspace-specific side-channel may mutate external state.

