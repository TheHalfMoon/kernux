# Local Privacy Implementation Plan

## 1. Purpose

Local privacy is a **product invariant**, not a deployment preference.

Kernux must remain meaningfully useful when:

- no Kernux cloud account exists;
- no external model provider is configured;
- telemetry is disabled;
- the machine is offline except for user-authorized direct destinations;
- local indexes, memory, artifacts, task history and evidence never leave the user's device;
- external providers disappear.

The privacy objective is:

> **User data stays local by default. External data movement is explicit, destination-bound, inspectable, revocable and never required for core correctness.**

This plan makes that objective implementation-ready without changing the active P02 implementation frontier.

## 2. Privacy threat model

Kernux handles highly sensitive material:

- source code and repositories;
- files and documents;
- terminal/process output;
- browser history and authenticated sessions;
- emails, messages, calendars and connected-account data;
- durable memory;
- screenshots, audio and future multimodal input;
- credentials and secret-derived actions;
- task history, artifacts and evidence.

Primary privacy threats include:

1. silent content upload to model/search/telemetry providers;
2. hidden cloud dependency for an apparently local feature;
3. prompt/tool output causing data exfiltration;
4. secret exposure to agents, logs, prompts, screenshots or crash reports;
5. cross-project/account leakage;
6. stale local caches surviving deletion;
7. vector/index databases becoming shadow canonical stores;
8. remote runtime or connector receiving broader context than required;
9. cloud fallback activating automatically when local inference fails;
10. analytics collecting content-bearing identifiers or payloads;
11. provider SDK telemetry outside Kernux control;
12. model/download/update paths unexpectedly contacting the network;
13. local unencrypted sensitive stores being copied from disk;
14. backups/exports containing secrets or deleted data;
15. browser cookies/session state leaking between tasks or projects.

## 3. Privacy modes

Kernux must expose explicit privacy modes compiled into policy. These are not UI-only labels.

### 3.1 LOCAL_ONLY

Default for a new local project unless the user chooses otherwise.

Allowed:

- loopback IPC;
- local files/processes/PTY;
- local models;
- local decision providers;
- local memory/search/indexing;
- local browser automation against already-local content;
- user-owned local containers/VMs;
- local artifacts and evidence.

Denied by default:

- model-provider API calls;
- hosted search/fetch APIs;
- telemetry;
- crash upload;
- remote runtimes;
- connected SaaS accounts;
- arbitrary internet egress.

### 3.2 DIRECT_NETWORK

Allows explicit network destinations needed for the task without granting general cloud-AI processing.

Examples:

- browse `docs.rs`;
- fetch a public GitHub repository;
- access a user-selected API endpoint.

The destination set is explicit and auditable.

### 3.3 CONNECTED_SERVICES

Allows selected connected accounts such as Gmail, Slack, GitHub or Jira.

Each connection is separately scoped by:

- account;
- capability;
- OAuth/API scope;
- data classes;
- allowed operations;
- project/task scope;
- expiry/revocation.

### 3.4 EXTERNAL_AI

Allows selected external AI/model/embedding/search providers.

This mode requires explicit provider configuration and an inspectable declaration of:

- data sent;
- destination/provider;
- purpose;
- retention/training assumptions when known/configurable;
- credential used;
- estimated/actual cost;
- fallback behavior.

Failure of an external AI provider must never silently widen another provider's access.

### 3.5 REMOTE_USER_OWNED

Allows an explicitly enrolled user-owned remote machine/server.

Remote execution remains a separate trust domain and receives only bounded task context.

## 4. Egress classification

Every network-capable operation must carry one egress class:

- `NONE` — no non-loopback network;
- `DIRECT_DESTINATION` — direct user/task-selected origin;
- `CONNECTED_ACCOUNT` — explicit third-party account;
- `EXTERNAL_MODEL` — model/embedding/reranking/inference provider;
- `EXTERNAL_TOOL` — hosted tool/API provider;
- `REMOTE_RUNTIME` — enrolled remote device/server;
- `UPDATE` — software/model/package update;
- `TELEMETRY` — analytics/diagnostics/crash upload.

Policy evaluates egress before execution.

No provider may relabel `EXTERNAL_MODEL` traffic as ordinary `DIRECT_DESTINATION` to bypass privacy controls.

## 5. Data-boundary contract

Every provider/capability manifest must declare:

- whether it can operate fully locally;
- network destinations;
- egress class;
- input data classes;
- output data classes;
- secret usage;
- retention assumptions;
- content logging behavior;
- provider telemetry behavior;
- account identity;
- encryption in transit;
- deletion/revocation behavior;
- offline behavior;
- fallback behavior.

Unknown privacy metadata fails closed for sensitive operations.

## 6. Local-first capability baseline

The first public alpha must provide a complete useful local path for the following.

### 6.1 Local task and project truth

Local only:

- Task;
- WorkUnit;
- Run;
- Event;
- Evidence;
- approval state;
- runtime ledger;
- artifact metadata.

No hosted control plane is required.

### 6.2 Local files and computer execution

Local provider path for:

- file read/write/list/search;
- process execution;
- PTY/terminal;
- Git;
- artifact production;
- deterministic validation.

### 6.3 Local agent/model path

At least one qualified local generative-model path must exist before the product claims a local AI workflow.

Provider-neutral contract first; supported local runtimes may include qualified adapters for:

- llama.cpp;
- Ollama;
- MLX on supported Apple hardware;
- other explicitly qualified local runtimes.

No specific runtime becomes architecture authority.

A deterministic fake provider remains available for CI.

### 6.4 Local Decision Fabric

At least one local DecisionProvider must be qualified before Decision Fabric is a required product dependency.

Candidates include SemIf, Decider, Laya-model or another qualified local provider.

Kernux correctness must not depend on a decision model being installed.

### 6.5 Local search and memory

Required without cloud embeddings:

- exact lookup;
- SQLite FTS/BM25 or equivalent local lexical search;
- local file/symbol search;
- temporal/provenance filtering;
- local memory inspection/edit/delete/export.

Optional vectors must have a local embedding path or remain disabled.

External embeddings are opt-in `EXTERNAL_AI`.

### 6.6 Local artifacts and evidence

Artifacts, evidence and replay remain local unless explicitly exported/shared.

### 6.7 Local browser/computer use

Browser automation itself runs locally by default.

A direct web visit is `DIRECT_DESTINATION`; sending page content to an external model is a separate `EXTERNAL_MODEL` operation requiring separate authority.

This separation is mandatory.

## 7. No silent cloud fallback

A local provider failure may result in:

- retry locally;
- select another already-authorized local provider;
- pause and ask the user;
- fail/degrade explicitly.

It must not automatically send content to an external provider.

This rule applies to:

- generative models;
- decision models;
- embeddings;
- OCR;
- speech/transcription;
- search/reranking;
- document processing;
- browser interpretation;
- verification.

## 8. Network enforcement

Privacy cannot rely only on prompts or provider configuration.

Implementation must add an enforceable network policy path with:

- destination allow/deny evaluation in `kernuxd`;
- loopback distinction;
- DNS resolution/rebinding defenses where applicable;
- redirect re-evaluation;
- private/link-local/metadata-address rules;
- per-operation destination evidence;
- connector/provider destination declarations;
- fail-closed behavior for undeclared destinations.

Sandbox/container/VM profiles should support stronger network isolation where the runtime allows it.

Host-mode limitations must be displayed honestly: Kernux can mediate Kernux-owned network operations but cannot claim to firewall arbitrary unrelated host processes unless an OS-level enforcement mechanism is actually active.

## 9. Secret privacy

Secrets are never ordinary context.

Required:

- OS credential store or explicitly qualified local secret provider;
- `SecretHandle` references in task/tool/model context;
- plaintext retrieval only at the narrowest execution boundary;
- destination-bound injection where possible;
- no secret plaintext in Event/Evidence/ContextBundle/MemoryRecord;
- redaction in terminal/log/tool output where technically possible;
- secret reveal/copy/export as a separate privileged capability;
- secret use audit without logging the value.

The current P02 secret-provider work is a prerequisite, not duplicated by this plan.

## 10. Local data protection at rest

Kernux must classify local stores by sensitivity.

### 10.1 Secret material

Must use an OS credential store or separately qualified encrypted secret store.

### 10.2 Sensitive user content

Before external alpha, Kernux must define and qualify encryption-at-rest behavior for:

- durable memory;
- connected-account cached content;
- browser profile/session material;
- sensitive artifacts when stored in a Kernux-managed private vault;
- sensitive task/context payloads.

The implementation may use OS/platform encryption, an encrypted database/vault, or another qualified mechanism. The architecture must not hard-code an unqualified encryption library.

Keys must not be stored alongside encrypted content in plaintext.

### 10.3 Ordinary project files

Kernux does not claim to transparently encrypt the user's entire existing filesystem. It respects existing OS/filesystem protections and clearly distinguishes project files from Kernux-managed private-vault content.

## 11. Privacy firewall

All content entering durable memory, external providers or high-consequence tools passes a privacy/taint decision.

Data classes must include at least:

- PUBLIC;
- PROJECT_PRIVATE;
- PERSONAL;
- CREDENTIAL_OR_SECRET;
- CONNECTED_ACCOUNT_PRIVATE;
- ORGANIZATION_RESTRICTED;
- USER_DECLARED_SENSITIVE.

Policy determines whether each class may cross a given boundary.

A model may recommend classification but cannot lower a stricter deterministic/user classification.

## 12. Context minimization

Remote/external providers receive the minimum sufficient context.

The Context Compiler must:

1. determine required sources;
2. remove unrelated project/user data;
3. redact secret material;
4. preserve provenance;
5. disclose truncation;
6. bind the resulting ContextBundle digest to the provider call.

"Send whole workspace" is never the default.

## 13. Browser and connected-account privacy

Required behavior:

- browser contexts isolated by project/task unless the user explicitly reuses one;
- authenticated profiles named and visible;
- cookie/storage persistence explicit;
- downloads/uploads scoped;
- cross-project profile reuse denied by default;
- connected-account content does not become global memory automatically;
- outbound actions preview destination/account/content before approval when policy requires;
- read access does not imply write access.

## 14. Telemetry and diagnostics

Core operation must work with telemetry disabled.

Default privacy posture:

- no prompt/file/message/document content in product analytics;
- no secret plaintext;
- no hidden session replay;
- no background content upload;
- diagnostics export is user initiated;
- crash/support bundles are locally inspectable and redacted before upload;
- provider SDK telemetry is disabled where supported or disclosed where it cannot be disabled.

Telemetry is a separate `TELEMETRY` egress class.

## 15. Update and model acquisition privacy

Software/model updates are network operations with explicit provenance.

Required:

- pinned source/release identity;
- digest verification;
- download destination visibility;
- no model auto-download merely because a task was opened;
- offline operation after required local assets are installed;
- model caches are user-inspectable and deletable;
- update checks can be disabled;
- update traffic carries no project content.

## 16. Deletion, export and backup

Privacy requires lifecycle closure.

For every Kernux-owned durable data class define:

- location;
- retention;
- export;
- backup;
- restore;
- deletion;
- derived-index invalidation;
- tombstone/compaction behavior;
- secret exclusion;
- external-system deletion limitations.

Deletion must cover rebuildable search/vector/graph projections, not only the primary row.

## 17. Local Privacy Inspector

Before first external alpha, users must be able to inspect at least:

- current privacy mode;
- connected accounts;
- configured external AI providers;
- active remote runtimes;
- secrets by reference/owner, never value;
- recent egress by destination/class;
- which providers received task content;
- local data locations;
- memory/index state;
- browser profiles;
- telemetry/update settings.

The inspector is a view over real policy/event state, not a decorative settings page.

## 18. Privacy evidence

Every external data transfer should emit evidence sufficient to answer:

- what operation caused it;
- which destination/provider;
- egress class;
- which data classes crossed;
- which credential handle was used;
- which task/run authorized it;
- which policy/grant allowed it;
- whether content was redacted/minimized;
- timestamp and result.

Evidence must not reproduce the sensitive payload itself unless explicitly required and safely stored.

## 19. Privacy adversarial corpus

Add deterministic fixtures for:

- prompt injection requesting secret upload;
- tool output requesting wider network access;
- browser redirect to undeclared/private destination;
- DNS rebinding/private-network target;
- model provider fallback attempted from local-only mode;
- external embeddings attempted from local-only mode;
- cross-project memory retrieval;
- cross-account connector leakage;
- browser-profile leakage;
- deleted memory remaining in derived indexes;
- secret echoed in process output;
- telemetry with content payload;
- crash bundle containing private data;
- provider SDK attempting undeclared telemetry;
- remote runtime requesting extra context;
- stale grant after mode downgrade;
- task changed from EXTERNAL_AI to LOCAL_ONLY mid-run.

Negative evidence is preserved.

## 20. Privacy SLOs and release metrics

Track at least:

- undeclared external egress: target zero;
- secret plaintext in ordinary logs/evidence: target zero;
- silent cloud fallback: target zero;
- cross-project/account leakage in qualification corpus: target zero;
- external-provider calls without provider/data-boundary identity: target zero;
- deletion residuals in owned derived indexes after completed deletion: target zero;
- local-only golden-journey completion rate;
- context-minimization ratio where measurable;
- privacy-mode policy evaluation failures;
- redaction failures;
- unsupported provider telemetry disclosures.

These are engineering metrics, not marketing claims.

## 21. Implementation mapping

This plan refines the existing roadmap rather than creating a new parallel roadmap.

### P02 — privileged kernel

Must establish:

- secret-provider abstraction;
- handle-based secret use;
- capability/policy foundation;
- metadata/artifact/evidence storage already planned.

Add during dependency-correct refinement, not out of order:

- egress-class policy primitives if not already representable;
- destination metadata required by later network mediation.

### P03 — desktop/headless shell

Must add:

- Privacy Inspector shell;
- privacy mode selection/status;
- provider/account/runtime visibility;
- local data-location and deletion/export entry points.

### P04 — local computer/data runtime

Must prove:

- local files/process/PTY golden path;
- local data-store boundaries;
- private-vault/encryption-at-rest strategy for Kernux-managed sensitive content;
- no-network local execution fixture.

### P05 — agents/models/Decision Fabric

Must add:

- external-vs-local provider metadata;
- at least one qualified local generative-model adapter before local AI claims;
- deterministic fake provider for CI;
- local DecisionProvider qualification;
- no-silent-cloud-fallback tests;
- provider-call ContextBundle digest/evidence.

### P06 — web/browser

Must prove:

- local browser execution;
- DIRECT_DESTINATION separate from EXTERNAL_MODEL;
- redirect/DNS/private-network controls;
- authenticated profile isolation;
- TinyFish/AgentQL hosted paths optional;
- deterministic offline fixtures.

### P07-P09 — orchestration/evidence/replay

Must prove:

- privacy mode inherited by child WorkUnits;
- child agents cannot widen data boundary;
- egress evidence/replay;
- provider fallback respects privacy mode;
- restart does not restore expired/wider grants.

### P10 — isolation/runtime

Must qualify stronger network isolation profiles for sandbox/container/VM where supported.

### P11 — tools/integrations

ToolDescriptor must include egress/data-boundary/secret/cost metadata.

Unknown sensitive-tool data boundary fails closed.

### P12 — automation

Background/scheduled work inherits the same privacy mode and cannot activate external providers merely because the user is absent.

### P13-P14 — remote/mobile/team

Remote devices/teams are explicit trust domains.

No automatic upload of local memory/task history to a Kernux cloud is permitted as a prerequisite for continuity.

### P16-P20

Artifacts, integration fabrics, proactive work and ecosystem packages inherit the same local-privacy contracts.

No Capability Pack may introduce an unmediated cloud dependency.

## 22. Implementation-ready work packages

Final IDs remain owned by SpecGrain refinement, but the work is dependency-bounded as follows.

### LP-01 — Egress taxonomy and policy contract

Outcome:
Kernux can represent and deny/allow every network operation by explicit egress class and destination metadata.

Acceptance:

- typed egress enum;
- destination identity;
- local-only denial fixture;
- redirect destination re-evaluation contract;
- exact event/evidence shape;
- cross-language protocol fixtures if wire-visible.

Risk: R3.

### LP-02 — Provider privacy manifest

Outcome:
Every model/tool/web/connector provider declares local/external behavior and data boundary.

Acceptance:

- schema;
- fail-closed validation;
- unknown sensitive metadata rejected;
- provider revision binding;
- provenance integration.

Risk: R2.

### LP-03 — No-silent-cloud-fallback enforcement

Outcome:
Local provider failure never causes external data movement without existing authority.

Acceptance:

- local failure fixture;
- external provider configured but unauthorized fixture;
- user-approved escalation fixture;
- cancellation/race/restart tests.

Risk: R3.

### LP-04 — Context minimization and redaction contract

Outcome:
Provider calls receive a digest-bound minimum ContextBundle with secret exclusion.

Acceptance:

- selected-source accounting;
- omitted-source disclosure;
- secret-handle exclusion;
- provider call bound to ContextBundle digest;
- adversarial tests.

Risk: R3.

### LP-05 — Privacy Inspector

Outcome:
User can inspect actual privacy state and recent egress.

Acceptance:

- privacy mode;
- external providers;
- accounts;
- remote runtimes;
- recent egress;
- local storage locations;
- telemetry/update settings;
- keyboard/semantic accessibility.

Risk: R2.

### LP-06 — Local generative-model qualification

Outcome:
One bounded AI task completes without an external model API.

Acceptance:

- exact local runtime/model revision;
- offline-after-install proof;
- bounded resource behavior;
- cancellation/OOM behavior;
- local ContextBundle;
- evidence;
- no network beyond loopback during the qualified task.

Risk: R2/R3 depending on runtime.

### LP-07 — Local DecisionProvider qualification

Outcome:
Decision Fabric has one qualified local path and remains optional.

Acceptance:

- typed decision fixtures;
- calibration;
- OOD/abstain;
- multilingual sample;
- offline proof;
- fallback disabled/enabled parity for authorization semantics.

Risk: R2.

### LP-08 — Local memory/search qualification

Outcome:
Useful retrieval works with no external embedding/model service.

Acceptance:

- lexical/exact/temporal path;
- provenance;
- delete/export;
- derived-index deletion;
- cross-project denial.

Risk: R2.

### LP-09 — Local browser privacy boundary

Outcome:
Browser can visit explicit sites locally without page content being silently sent to external AI.

Acceptance:

- DIRECT_DESTINATION fixture;
- EXTERNAL_MODEL separate approval;
- profile isolation;
- redirects/private-network adversarial corpus;
- download/upload scope.

Risk: R3.

### LP-10 — Sensitive local-vault protection

Outcome:
Kernux-managed sensitive durable content has a qualified at-rest protection strategy.

Acceptance:

- threat model;
- key ownership/storage;
- migration/backup/restore;
- loss/recovery behavior;
- platform qualification;
- no plaintext key beside ciphertext.

Risk: R3.

### LP-11 — Telemetry/update privacy qualification

Outcome:
Core works with telemetry off; updates never send project content.

Acceptance:

- telemetry-disabled golden path;
- support bundle redaction;
- model/software update fixtures;
- no content-bearing update request.

Risk: R2.

### LP-12 — Local-only end-to-end golden journey

Outcome:
A meaningful Kernux workflow completes with no external AI/cloud service.

Minimum journey:

1. open local repository/project;
2. local search/context assembly;
3. local model or deterministic local agent path;
4. local file/process execution;
5. local verification;
6. local artifact/evidence;
7. restart/replay;
8. inspect privacy evidence.

Network oracle must confirm no non-loopback traffic for the fully offline fixture.

Risk: R3.

## 23. Alpha privacy gate

Kernux must not claim a privacy-first/local-first external alpha until all are proven:

1. secret handles and OS secret provider;
2. privacy mode contract;
3. egress classification;
4. no-silent-cloud-fallback;
5. local file/process/PTY path;
6. local search/memory path;
7. one local model-backed or explicitly model-free meaningful AI/agent path;
8. local browser separation between direct network and external AI;
9. Privacy Inspector;
10. deletion/export for Kernux-owned user data;
11. telemetry-off operation;
12. privacy adversarial corpus;
13. local-only golden journey with network oracle;
14. exact-head review and required R3 security evidence.

## 24. Definition of implementation-ready

The local-privacy plan is implementation-ready when:

- this contract is adopted into canonical planning;
- each LP package is mapped to a dependency-correct SpecGrain unit before its phase becomes eligible;
- active P02 authority remains unchanged;
- no privacy requirement exists only in prose without a future owner;
- providers introduced by PLATFORM_FABRICS_AND_SOURCE_INTEGRATION.md inherit this contract;
- the first applicable implementation Grain names its privacy acceptance and evidence explicitly.

## 25. Final rule

> **Local is the default trust boundary. Network access is a capability. Cloud processing is an explicit data-boundary decision. Failure never silently weakens privacy.**
