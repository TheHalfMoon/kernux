# Laya Integration Plan

> Normative planning module under the Kernux Platform Fabrics and Source Integration Plan.

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

## 7. State machines

### 7.1 Integration event pipeline

```text
RECEIVED
  -> NORMALIZED
  -> DEDUPED
  -> CLASSIFIED
  -> ASSOCIATED
  -> ENRICHED
  -> PROJECTED
```

Failure states:

- INVALID;
- DUPLICATE;
- QUARANTINED;
- BLOCKED;
- RETRY_PENDING;
- DEAD;
- STALE.

Every retry must preserve source identity and avoid duplicate side effects.

### 7.2 Action proposal lifecycle

```text
DRAFT
  -> STAGED
  -> WAITING_APPROVAL
  -> AUTHORIZED
  -> EXECUTING
  -> RECONCILING
  -> VERIFIED
```

Alternative terminal states:

- DISMISSED;
- EXPIRED;
- STALE;
- DENIED;
- CANCELED;
- FAILED;
- AMBIGUOUS;
- COMPENSATED.

A proposal whose source or destination changed after approval must be revalidated or expire; approval cannot silently bind to materially changed content.

### 7.3 Rule firing lifecycle

```text
MATCHED
  -> SIMULATED
  -> ELIGIBLE
  -> PROPOSED
  -> POLICY_EVALUATED
  -> EXECUTED/NOOP/BLOCKED
  -> RECORDED
```

Model-evaluated predicates may help determine `MATCHED`; they cannot produce `AUTHORIZED` by themselves.

## 8. Event dedupe, idempotency, and replay

Every connector must define:

- source-native event identity;
- deterministic dedupe key;
- source replay semantics;
- ingestion idempotency;
- operation idempotency where the destination supports it;
- compensation/reconciliation when it does not;
- duplicate-event suppression window;
- ordering assumptions;
- out-of-order handling;
- stale-event behavior;
- restart/crash recovery;
- dead-letter state;
- manual retry semantics.

A retry cannot infer that an external side effect failed merely because Kernux lost contact.

## 9. Connector architecture

The connector path is:

```text
Source system
    |
    v
Connector/Adapter
    |
    v
IntegrationEvent
    |
    v
Event/Automation Fabric
    |
    +--> Context association / routing / research
    |
    v
ActionProposal
    |
    v
Kernux capability + policy + Grant
    |
    v
Approved connector operation
    |
    v
Reconciliation + Evidence
```

Connectors must declare:

- source/provider identity;
- account identity model;
- supported read/write operations;
- auth scheme;
- SecretHandle bindings;
- webhook/poll/subscription behavior;
- scopes/permissions;
- data sent/received;
- rate limits;
- cost owner;
- retention/telemetry assumptions;
- idempotency support;
- cancellation semantics;
- evidence returned;
- schema version;
- capability drift detection;
- connector health.

## 10. Initial connector qualification set

P17 qualification should include at least:

- GitHub;
- one email provider;
- one calendar provider;
- one messaging provider;
- one project-tracking provider;
- one document/storage provider;
- one business-system provider.

The exact providers must be selected by the future Grain based on local/BYOK/BYOC availability, stable APIs, testability, licensing, and zero-founder-COGS constraints.

Laya's existing GitHub, Gmail, Google Calendar, Slack, Jira, Linear, Notion, Outlook, and Bitbucket implementations are high-value characterization/reference sources.

## 11. Action Inbox UX

The Action Inbox should support:

- priority and freshness;
- source app/account;
- explanation of why the item matters;
- source evidence/context;
- proposed action preview;
- destination/account preview;
- data/secret usage preview;
- consequence level;
- cost/payer preview when observable;
- expected evidence;
- approve;
- edit;
- dismiss;
- snooze;
- open workspace;
- mark stale;
- bookmark/pin;
- batch triage only when each underlying operation remains separately authorized.

Keyboard, screen-reader, non-color-only state, RTL, localization, and scalable text are required architecture properties.

## 12. Action Workspace UX

A workspace for complex proposals must expose:

- task objective;
- source event timeline;
- context/evidence panel;
- active agent/run;
- live progress;
- staged artifacts/patches/drafts;
- tool/capability requests;
- approvals;
- proposed external writes;
- verification results;
- failures/retries;
- final reconciliation.

Agent requests surfaced in the workspace remain ordinary capability requests; the UI cannot bypass policy.

## 13. Coherence and relationship trace

Kernux should preserve Laya's strongest cross-platform idea while strengthening truth semantics.

Required behavior:

- exact-key lookup;
- lexical retrieval;
- semantic retrieval where locally or explicitly externally qualified;
- relationship traversal;
- temporal filtering;
- source-system links;
- contradiction/conflict display;
- correction/unlink;
- explain-why-this-result;
- derived narrative with source citations;
- no graph edge as authorization.

Morize may later implement the durable MemoryProvider/relationship-memory role, but Kernux task/evidence/external-source truth remains separate.

## 14. Omni and briefings

Omni-style output becomes a derived view with temporal layers:

- Attention;
- Recent;
- Period;
- Milestone.

Required controls:

- exact time window;
- source selection;
- project/space selection;
- local vs external processing mode;
- pinned facts/items;
- user corrections;
- regenerate;
- show sources;
- export.

Morning/daily briefings use the same projection machinery and must not silently promote generated text into memory.

## 15. Spaces

A Space is a scoped user-facing context lens, not a new authority domain.

A Space may bind:

- projects;
- connector accounts;
- approved models/agents;
- default privacy mode;
- retrieval scope;
- default cost owner;
- routines;
- UI filters.

Actual authority remains the intersection of Kernux identity, policy, Grants, runtime capabilities, connector scopes, organization constraints, and destination permissions.

## 16. Classification and learning

Laya-style correction learning is allowed only under explicit boundaries.

Learnable inputs may include:

- priority correction;
- category/persona correction;
- association/link correction;
- rule suggestions;
- preferred presentation.

Requirements:

- learned rule provenance;
- editable rules;
- version history;
- rollback;
- simulation before activation for side-effecting automation;
- no learned rule may create broader authority;
- sensitive personal inferences require stricter policy;
- poisoning/adversarial tests are mandatory.

## 17. Security threat model additions

Every applicable Grain must test:

1. prompt injection inside email/message/ticket/document content;
2. malicious connector payloads;
3. spoofed actor/source metadata;
4. duplicate/replayed webhook events;
5. cross-account confused-deputy actions;
6. stale approvals;
7. proposal payload mutation after approval;
8. connector scope expansion;
9. secret leakage into prompts/logs/UI;
10. rule loops and notification storms;
11. model-evaluated condition self-authorization;
12. external destination substitution;
13. link/entity poisoning;
14. memory poisoning;
15. forged completion evidence;
16. provider outage and partial execution;
17. ambiguous external side effects;
18. silent cloud fallback;
19. hidden telemetry/retention;
20. unsafe bulk/batch approval.

High-consequence and external-write surfaces are R3 by default unless a future Grain proves a lower class is justified.

## 18. Privacy and data sovereignty

Local-first means more than local UI.

For every event, research step, embedding, model call, connector action, and derived summary record:

- what data leaves device;
- exact destination class;
- purpose;
- credential used;
- retention/telemetry assumptions;
- provider revision;
- user/org policy basis;
- local fallback behavior;
- deletion/invalidation semantics.

Local provider failure must not silently activate a cloud provider.

The fully local supported journey must remain useful without Laya cloud, Kernux cloud, hosted embeddings, hosted inference, or founder-owned credentials.

## 19. Cost and budget model

Adopt Laya's budget visibility but align it with Kernux economics.

Every chargeable operation should identify, where observable:

- provider;
- feature/task/run;
- quantity unit;
- estimated/actual cost;
- payer;
- budget;
- remaining quota/window;
- pause/block reason.

Valid core cost owners:

- user local hardware;
- user provider-native subscription;
- user BYOK;
- organization-owned account/infrastructure;
- separately priced managed service with positive unit economics.

Founder-funded shared runtime credentials are not a valid normal-path dependency.

## 20. Analytics

Analytics are derived from local events/evidence where practical.

Useful measures:

- events ingested;
- proposals generated;
- approvals/edits/dismissals;
- verified actions;
- failed/ambiguous actions;
- dedupe/replay counts;
- connector failures;
- rule firing outcomes;
- agent/provider latency;
- cost by provider/feature;
- time-to-attention;
- time-to-verified-result;
- recovery rate;
- stale proposal rate.

"Time saved" or productivity claims require an explicit measurement method and must not be fabricated.

## 21. Failure and recovery

Required recoverable states:

- connector offline;
- auth expired;
- rate limited;
- provider unavailable;
- local model unavailable;
- agent process crashed;
- app restarted;
- source event duplicated;
- source event arrived out of order;
- proposal expired;
- approval arrived after source changed;
- external write timed out;
- external system succeeded but response was lost;
- external system rejected the write;
- indexing failed after canonical persistence;
- derived summary generation failed;
- memory provider unavailable.

Derived-view/index failure must not corrupt canonical task/event/evidence state.

## 22. Storage-of-record rules

| Data class | Canonical owner |
| --- | --- |
| Task/WorkUnit/Run | Kernux task store |
| authority/Grants/approval | kernuxd policy state |
| IntegrationEvent | Integration/Event Fabric durable store |
| ActionProposal | Kernux task/integration state |
| external SaaS record | source system unless explicitly imported |
| external execution truth | source system + reconciled Kernux evidence |
| artifact | Artifact CAS or explicit external source |
| evidence | Kernux evidence ledger |
| secrets | approved secret provider |
| durable memory | MemoryProvider |
| search/vector/graph index | derived rebuildable projection |
| Attention/Briefing/Omni | derived projection |
| analytics | derived projection from canonical events/evidence |

No Laya SQLite or ChromaDB table may silently become a second canonical owner.

## 23. Cross-platform contract

Applicable work must qualify:

- macOS;
- Windows;
- Linux;
- WSL where relevant;
- local loopback/network restrictions;
- process lifecycle;
- filesystem path handling;
- keychain/secret-provider behavior;
- notifications;
- system tray;
- deep links;
- app restart/recovery;
- timezone behavior.

A donor implementation passing on one platform is not evidence for all Kernux-supported platforms.

## 24. Import strategy

### Wave L0 — source admission

Before copying source:

- reverify upstream revision;
- capture Laya LICENSE and NOTICE;
- inventory selected source files;
- inventory direct/transitive dependencies for selected code;
- classify assets/trademarks/services;
- prepare provenance manifest;
- prepare source-to-destination map;
- write characterization tests;
- choose transformation class per file.

### Wave L1 — contracts and fixtures

Implement Kernux-owned:

- IntegrationEvent;
- ActionProposal;
- association/attention projection contracts;
- connector descriptors;
- lifecycle fixtures;
- adversarial fixture corpus.

No donor UI or engine code is required yet.

### Wave L2 — event pipeline

Implement:

- normalize;
- dedupe;
- durable ingestion;
- classification hooks;
- association hooks;
- enrichment;
- recovery/dead-letter state;
- exact provenance.

### Wave L3 — connector/account lifecycle

Implement:

- connector registry;
- auth/account selection;
- SecretHandle binding;
- scope/permission inspection;
- revocation;
- health;
- capability drift;
- rate/cost metadata.

### Wave L4 — Action Inbox

Port/adapt Laya Action Card UX onto ActionProposal and Kernux policy.

### Wave L5 — Action Workspace

Port/adapt workspace interaction onto Task/WorkUnit/Run/Context/Evidence.

### Wave L6 — rules and proactive staging

Implement:

- deterministic rules;
- model-evaluated predicates;
- simulation;
- firing log;
- rate/suppression;
- loop prevention;
- proposal generation only.

### Wave L7 — Coherence / Briefings / Omni

Implement derived:

- association;
- entity trace;
- hybrid retrieval;
- cross-app narrative;
- briefing;
- temporal Omni;
- bookmarks/pins;
- correction learning.

### Wave L8 — audit, recovery, budget, analytics

Implement:

- dead-event recovery;
- connector failure views;
- execution reconciliation;
- budget windows/caps;
- local analytics;
- support/export.

### Wave L9 — bounded donor source import

Only after receiving contracts exist:

- mechanical source import where reuse is justified;
- minimal namespace/path adaptation;
- characterization proof;
- subsequent semantic adaptation in separate commits where practical.

### Wave L10 — qualification

Run the full evidence matrix and phase gates before calling the integration complete.

## 25. Evidence matrix

Every applicable implementation slice must select evidence from this matrix:

| Area | Required evidence examples |
| --- | --- |
| contracts | schema/fixture conformance, unknown-field/version behavior |
| provenance | exact upstream revision, LICENSE/NOTICE, manifest, source map |
| dependency | license/admission validator, lockfile proof |
| event ingestion | duplicate/replay/out-of-order/restart corpus |
| security | R3 review, prompt-injection/confused-deputy/secret fixtures |
| policy | proposal cannot self-authorize; destination/account bindings |
| connectors | auth expiry/revocation/scope drift/provider failure |
| recovery | crash/restart/ambiguous external side effect |
| privacy | local-only journey, no silent cloud fallback, network oracle where applicable |
| cost | explicit payer and no founder-funded normal path |
| accessibility | keyboard/screen-reader/non-color/zoom/RTL where applicable |
| cross-platform | same core journey on each applicable supported OS |
| UI | proposal mutation/staleness/approval previews |
| memory/search | poisoning/correction/delete/index rebuild |
| evidence | reconciliation confirms or refuses completion |
| performance | bounded event backlog, search, inbox, workspace and connector latency |
| provider loss | task truth survives model/connector/index provider loss |

## 26. Golden journeys

The integration is not complete until representative golden journeys are proven.

### GJ-L01 — GitHub review event

GitHub PR event -> dedupe -> research/context -> ActionProposal -> workspace -> user approval -> comment/review action -> reconciliation -> evidence.

### GJ-L02 — Email reply

Email -> association with project/person/history -> draft -> destination/data preview -> approval -> send -> source-system confirmation -> evidence.

### GJ-L03 — Calendar follow-up

Meeting/calendar event -> related notes/tasks -> follow-up proposal -> approval -> calendar/message action -> evidence.

### GJ-L04 — Cross-app coherence

One entity appears in messaging + issue tracker + source control + documents -> association -> trace -> source-linked narrative -> correction/unlink proof.

### GJ-L05 — Morning briefing

Local event/task/calendar window -> derived briefing -> source inspection -> no memory promotion without policy.

### GJ-L06 — Duplicate webhook

Same source event delivered repeatedly -> one durable IntegrationEvent projection -> no duplicate proposal/action.

### GJ-L07 — Ambiguous write

External system commits the write but response is lost -> Kernux records ambiguity -> reconciles from destination -> no blind retry duplicate.

### GJ-L08 — Credential revocation

Connector token revoked while queued proposal exists -> proposal blocks/revalidates -> no secret fallback -> explicit recovery path.

### GJ-L09 — Local-only mode

Supported local connectors/local model/local retrieval -> no hidden cloud processing -> useful Action Inbox/briefing remains available.

### GJ-L10 — Prompt injection

Malicious email/ticket tells agent to exfiltrate secrets or bypass approval -> content remains untrusted -> no authority escalation -> evidence shows denial.

## 27. Performance and reliability targets

Do not freeze arbitrary marketing SLOs before measurement.

The first implementation Grain for each surface must establish baselines for:

- event ingestion throughput;
- duplicate suppression;
- inbox projection latency;
- association/search latency;
- workspace resume time;
- connector recovery;
- rule evaluation;
- briefing generation;
- memory/index rebuild;
- UI startup with representative backlog.

P20 may ratchet proven SLOs after measurement.

## 28. Implementation task mapping

Existing canonical ownership remains:

### P17 — Integration and event fabric

Owns:

- connector contracts;
- auth/account lifecycle;
- intent-based tool discovery;
- health/drift/data boundaries;
- webhooks/subscriptions;
- IntegrationEvent;
- event normalization/dedupe/association/enrichment;
- LiveDataObject where applicable.

Add:

- Laya donor admission/characterization;
- porting of selected normalized connector/event logic behind Kernux contracts.

### P19 — Proactive personal/work agent

Owns:

- universal inbox;
- proactive suggestions;
- routines;
- ActionProposal/Action Inbox;
- processing rules;
- briefings/coherence.

Add:

- Action Workspace;
- Omni/attention/bookmark temporal views;
- dead-event/action recovery UX;
- local analytics/budget visibility.

P02-P16 dependencies remain unchanged.

## 29. Additional macro tasks

These tasks close gaps not fully represented by the existing P17/P19 macro registry.

### P17 additions

- `KX-P17-S10-T01` — admit and characterize pinned Laya donor surfaces with exact provenance, LICENSE/NOTICE, dependency/asset disposition, source-to-destination mapping, and pre-adaptation fixtures.
- `KX-P17-S10-T02` — port/adapt selected Laya normalization and connector workflow logic behind Kernux-owned IntegrationEvent/connector contracts, with n8n optional rather than mandatory.

### P19 additions

- `KX-P19-S09-T01` — implement Action Workspace over Task/WorkUnit/Run/Context/Artifact/Evidence/ActionProposal with live agent progress and policy-bound approvals.
- `KX-P19-S09-T02` — implement Omni/Attention/Bookmarks/temporal current-state views as rebuildable projections with source inspection, correction, and no truth duplication.
- `KX-P19-S10-T01` — implement user-visible dead-event/action recovery plus local budget/usage/operations analytics without telemetry or cloud dependency.

## 30. SpecGrain decomposition rule

Each macro task above must be refined before implementation.

A donor-import Grain must never combine all of Laya.

Refine by bounded receiving contract/surface, for example:

- one contract family;
- one connector;
- one event-normalization slice;
- one Action Inbox slice;
- one workspace slice;
- one association/search slice;
- one briefing/Omni slice.

Each Grain freezes:

- source revision;
- exact source paths;
- exact destination paths;
- transformation class;
- dependencies;
- privacy boundary;
- risk class;
- recovery;
- acceptance;
- evidence.

## 31. Anti-gap checklist

Before any Laya-derived implementation Grain can become GRAIN, answer all applicable questions:

### Source/provenance
- exact upstream revision pinned?
- LICENSE/NOTICE captured?
- founder authorization recorded?
- source paths exact?
- destination paths exact?
- transformation class exact?
- dependency and asset rights independently reviewed?

### Architecture
- receiving Kernux contract already exists or is in-scope?
- canonical owner clear?
- no duplicate truth store?
- no duplicate authority?
- no mandatory donor runtime?

### Security
- untrusted event content classified?
- exact account/destination bound?
- secret handling uses SecretHandle?
- consequence classification correct?
- stale approval/mutation handled?
- prompt injection fixtures present?

### Privacy/economics
- local/external boundary explicit?
- no silent cloud fallback?
- cost owner explicit?
- deletion/retention behavior defined?
- founder-funded runtime dependency absent?

### Reliability
- dedupe/replay behavior defined?
- retry identity defined?
- crash/restart defined?
- ambiguous external side effect defined?
- connector revocation/drift defined?
- provider loss defined?

### UX/accessibility
- source/evidence visible?
- action/destination/data/credential preview visible?
- edit/dismiss/snooze/review behavior defined?
- keyboard/screen-reader/non-color behavior defined?
- RTL/locale/timezone implications defined?

### Evidence
- characterization test exists?
- exact-head CI required?
- R3 required?
- cross-platform proof required?
- native/platform proof required?
- post-merge qualification required?
- negative evidence preserved?

If any applicable answer is missing, the Grain is not implementation-ready.

## 32. Completion gate

The Laya integration program is complete only when:

1. every imported source path has exact provenance and notice coverage;
2. no unreviewed third-party dependency is required;
3. Kernux works without mandatory n8n/LiteLLM/ChromaDB/cloud dependency;
4. IntegrationEvent and ActionProposal contracts are stable and versioned;
5. Action Inbox and Action Workspace operate only through Kernux authority;
6. connector auth/revocation/drift/recovery is proven;
7. duplicate/replayed events cannot duplicate side effects;
8. stale/mutated proposals cannot reuse old approval;
9. Coherence/Omni/briefings are derived and source-inspectable;
10. memory/index providers can disappear without corrupting task/evidence truth;
11. dead-event and ambiguous-action recovery is user-visible and proven;
12. local-only mode has no silent cloud fallback;
13. recurring runtime cost has an explicit non-founder payer;
14. security/adversarial evidence passes;
15. applicable macOS/Windows/Linux evidence passes;
16. accessibility and internationalization acceptance passes;
17. representative golden journeys pass;
18. provider-loss/recovery journeys pass;
19. exact-head and post-merge qualification passes for every governed slice;
20. no project document claims completion beyond observed evidence.

## 33. Final architecture

```text
External apps / services
        |
        v
Kernux connectors / optional integration providers
        |
        v
IntegrationEvent
        |
        v
Automation + Event Fabric
        |
        +--> Decision / Agent Fabric
        +--> Context / Memory Fabric
        +--> Search / Association
        |
        v
ActionProposal
        |
        v
Action Inbox / Action Workspace
        |
        v
Capability Request -> Policy -> Grant -> kernuxd
        |
        v
Connector / Tool / Browser / Computer operation
        |
        v
Reconciliation -> Evidence -> Derived views
                         |
                         +--> Coherence
                         +--> Briefing
                         +--> Omni
                         +--> Analytics
```

This preserves Laya's strongest product idea:

> **The useful answer and proposed action should be ready before the user opens the notification.**

Kernux strengthens it with:

> **The proposed action is never authority; only the capability kernel can authorize execution, and completion requires evidence.**
