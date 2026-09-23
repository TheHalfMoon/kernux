# Laya Delivery and Qualification Plan

> Normative planning module under the Kernux Platform Fabrics and Source Integration Plan.
>
> Depends on `LAYA_SOURCE_ARCHITECTURE.md` and `LAYA_LIFECYCLE_SECURITY.md`. This module defines bounded import waves, evidence, golden journeys, task ownership, SpecGrain decomposition, anti-gap readiness, completion gates, and final composition.

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



## 34. Implementation-readiness hardening

The Laya integration inherits, and must explicitly bind each future Grain to, the existing Kernux canonical contracts for security, privacy, lifecycle, portability, diagnostics, updates, cross-platform behavior, and release qualification. This section closes integration-specific gaps that are easy to miss when porting a working donor application.

### 34.1 Donor differential characterization

Before semantic adaptation of any copied donor path:

- preserve the exact upstream revision and source bytes;
- capture selected dependency versions or an equivalent reproducible dependency snapshot;
- generate dependency/license/SBOM evidence for the selected surface;
- write characterization fixtures against the donor behavior before adaptation;
- run differential tests where practical between donor behavior and the Kernux-adapted implementation;
- document every intentional behavior difference;
- preserve negative evidence when the donor behavior is unsafe, ambiguous, platform-specific, or incompatible with Kernux authority.

Floating upstream dependency ranges are not sufficient reproducibility evidence for a Kernux import Grain.

### 34.2 Connector conformance harness

Every connector admitted through the Laya program must pass a provider-independent conformance harness with deterministic fake/provider fixtures. The harness must cover, where applicable:

- OAuth authorization-code + PKCE or provider-equivalent secure flow;
- refresh-token rotation and refresh failure;
- API-key/token replacement and revocation;
- account and tenant selection;
- cross-account isolation and confused-deputy resistance;
- scope grant, reduction, expansion and capability drift;
- webhook signature/authentication verification;
- webhook timestamp/replay-window checks;
- webhook secret rotation;
- polling/subscription cursor persistence;
- rate-limit and retry metadata;
- deterministic idempotency identity;
- timeout/cancellation;
- provider malformed payloads;
- provider schema evolution;
- provider outage and partial response;
- account deletion/disconnect cleanup;
- SecretHandle-only credential access;
- explicit external data-boundary declaration.

Live provider tests may supplement but never replace deterministic CI fixtures.

### 34.3 Backpressure, resource and event-storm safety

Event ingestion and proactive processing must define bounded behavior for:

- queue depth;
- event payload size;
- attachment/artifact size;
- concurrent connector work;
- concurrent model/agent work;
- retry count and retry horizon;
- disk growth;
- derived-index growth;
- dead-letter growth;
- per-source fairness;
- per-space fairness;
- user-visible overload state;
- notification/action-proposal generation rate.

Overload must degrade by delaying, coalescing, suppressing, quarantining or refusing bounded work. It must not silently widen resource use, drop canonical evidence, duplicate side effects, or create unlimited founder-funded runtime cost.

### 34.4 Suspend, resume, clock and timezone correctness

Desktop/background behavior must be tested across:

- OS suspend and resume;
- application restart;
- system clock adjustment;
- timezone change;
- daylight-saving transitions where applicable;
- long offline intervals;
- webhook/poll cursor catch-up;
- expired approvals after resume;
- expired connector credentials after resume;
- budget-window rollover.

Wall-clock time is not a safe substitute for durable operation identity or monotonic retry ordering.

### 34.5 Persisted-state evolution and rollback

Laya-derived persisted connector/event/projection state must follow `DATA_LIFECYCLE_AND_PORTABILITY.md` and Kernux migration discipline:

- version every persisted schema;
- deterministic forward migrations;
- crash-safe migration checkpoints;
- restore/rollback plan before destructive migration;
- compatibility tests from every supported source version;
- backup/restore qualification for canonical local state;
- rebuild derived indexes instead of treating them as migration authority;
- preserve unknown future data during supported downgrade/rollback where possible;
- never make a model call part of migration correctness.

### 34.6 Update and supply-chain integrity

Any imported or adapted Laya code must remain inside Kernux update/supply-chain policy:

- exact donor source revision;
- exact import commit;
- LICENSE/NOTICE coverage;
- dependency lock evidence;
- vulnerability review;
- SBOM inclusion;
- signed/reproducible release expectations where applicable;
- updater metadata/signature verification;
- explicit rollback/recovery path;
- no unreviewed runtime code download by a connector, workflow, model adapter or UI component.

Laya's bundled/runtime assumptions are references, not automatic Kernux admissions.

### 34.7 Content rendering, attachments and deep links

External content is hostile by default. User-visible event/action surfaces must define:

- HTML/Markdown sanitization;
- safe URL rendering;
- deep-link allow/deny behavior;
- external-navigation confirmation where policy requires it;
- attachment MIME/type/size validation;
- active-content quarantine;
- no implicit execution of downloaded/opened content;
- content provenance preserved into any later ActionProposal;
- Unicode/bidirectional text handling that cannot visually spoof destination/account/action meaning.

Sanitization is presentation defense, not an authorization boundary.

### 34.8 Notification and attention governance

The proactive layer must prevent notification fatigue from becoming a reliability failure. The product contract must support:

- per-source and per-space notification budgets;
- dedupe/coalescing;
- quiet hours;
- digest/briefing substitution;
- snooze with explicit wake condition;
- stale-item retirement;
- repeated-failure suppression;
- escalation only under explicit policy;
- user-visible reason for interruption;
- no dark-pattern reactivation of dismissed items.

Notification suppression must never suppress durable evidence of a consequential action or failure.

### 34.9 Multi-account and identity isolation

All integration entities must bind to explicit connector + provider + account/tenant identity. Tests must prove that:

- same native IDs in two accounts never collide;
- one account's association graph does not silently leak into another;
- one account's secret cannot authorize another account;
- one account's approval cannot be replayed against another;
- account unlink/revocation invalidates dependent proposals safely;
- cross-account actions require explicit user-visible destination identity.

### 34.10 Observability and supportability

Every Laya-derived pipeline stage must emit structured, redaction-safe local diagnostics sufficient to reconstruct:

- source event identity;
- connector/account identity;
- pipeline state transition;
- rule/model/agent revision where applicable;
- proposal revision;
- capability request/Grant;
- external operation identity;
- reconciliation outcome;
- retry/dead-letter reason.

Support bundles follow the existing reviewable/redacted Kernux support-bundle contract and must not upload automatically.

## 35. Additional qualification journeys

The following journeys extend the minimum golden set and are mandatory when the relevant capability exists.

### GJ-L11 — Cross-account isolation

Two connected accounts contain colliding native identifiers -> events remain account-bound -> association remains scoped -> proposal targets the intended account -> wrong-account replay is denied.

### GJ-L12 — Authenticated webhook and secret rotation

Valid signed webhook accepted -> replay outside allowed window rejected -> signing secret rotates -> old/new overlap follows provider policy -> no duplicate proposal or event loss.

### GJ-L13 — Event storm and bounded degradation

Large duplicate/burst event load -> bounded queue/resource use -> fair processing -> dedupe/coalescing -> visible overload state -> canonical evidence preserved -> no notification storm or duplicate external action.

### GJ-L14 — Offline/suspend catch-up

Machine sleeps or goes offline -> provider events accumulate -> resume/catch-up preserves cursor/order semantics -> expired approvals/credentials are revalidated -> no blind stale execution.

### GJ-L15 — Upgrade, migration and restore

Representative persisted integration state -> application/schema upgrade -> deterministic migration -> crash interruption recovery -> backup restore -> derived indexes rebuild -> canonical task/event/evidence truth remains intact.

### GJ-L16 — Hostile rendered content

Email/ticket/document contains malicious HTML, deceptive Unicode, deep links and active attachment instructions -> rendering is safe -> content stays untrusted -> no hidden navigation/execution/authority escalation.

### GJ-L17 — Irreversible or ambiguous action

Proposal targets a non-idempotent or irreversible external write -> UI shows consequence/reversibility -> approval binds exact revision/account/destination -> timeout becomes AMBIGUOUS -> reconciliation prevents blind duplicate retry.

### GJ-L18 — Notification fatigue controls

Repeated low-value events -> dedupe/suppression/quiet-hours/digest behavior activates -> consequential failures remain inspectable -> dismissed/snoozed items do not dark-pattern reappear.

## 36. Integration-specific readiness gate

A Laya-derived implementation Grain is not ready until it can point to all applicable owners below rather than leaving the requirement implicit:

| Concern | Canonical owner / proof source |
| --- | --- |
| secrets and credential use | P02 secret provider + SecretHandle contracts |
| OAuth/account lifecycle | KX-P17-S02-T01 |
| webhook/subscription authenticity and replay | KX-P17-S05-T01 |
| connector health/scope/capability drift | KX-P17-S04-T01 |
| normalized event identity | KX-P17-S08-T01 |
| restart-safe normalization pipeline | KX-P17-S08-T02 |
| lifecycle/export/backup/delete/migrations | `DATA_LIFECYCLE_AND_PORTABILITY.md` + P15 qualification |
| privacy/no-silent-cloud fallback | `LOCAL_PRIVACY_IMPLEMENTATION_PLAN.md` |
| update/SBOM/supply-chain integrity | `SECURITY_MODEL.md` + release gates |
| locale/timezone/RTL/Unicode | KX-P18-S05-T01 |
| accessibility | KX-P18-S06-T01 |
| proactive notification suppression | KX-P19-S03-T01 + KX-P19-S04-T01 |
| provider-loss maturity | KX-P20-S08-T01 |
| load/SLO/chaos maturity | KX-P20-S06-T01 |

If a future Grain cannot identify its owning contract, dependency, recovery model and evidence class, it must remain below GRAIN.

## 37. Revised completion gate

The 20-condition gate in section 32 remains binding. In addition, the Laya integration program cannot be called implementation-complete until:

21. selected donor behavior has pre-adaptation characterization and intentional-difference accounting;
22. imported dependencies have reproducible lock/SBOM/license evidence;
23. connector conformance covers credential rotation, revocation, account isolation, webhook authenticity and schema drift;
24. event storms and backlog have bounded resource behavior and no unbounded notification generation;
25. suspend/resume/offline catch-up cannot execute stale approvals or credentials;
26. persisted integration state has tested migration, backup, restore and recovery behavior;
27. external content rendering, links and attachments preserve the hostile-content trust boundary;
28. account/tenant identities cannot collide or cross-authorize;
29. notification/attention controls prevent repeated low-value events from becoming user-facing storms;
30. structured diagnostics can reconstruct a failed/ambiguous pipeline without exposing secret plaintext;
31. update/supply-chain evidence includes selected donor code and dependencies;
32. every Laya-specific requirement has a canonical phase/task owner and no orphan implementation obligation remains only in narrative planning.
