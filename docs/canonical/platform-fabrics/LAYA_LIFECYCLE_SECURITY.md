# Laya Lifecycle, Product, and Security Plan

> Normative planning module under the Kernux Platform Fabrics and Source Integration Plan.
>
> Depends on `LAYA_SOURCE_ARCHITECTURE.md`. This module defines event/action/rule lifecycles, connector behavior, user surfaces, relationship/briefing behavior, security, privacy, economics, analytics, recovery, storage ownership, and cross-platform requirements.

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

