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

