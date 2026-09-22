# Local Privacy Foundations

> Normative module of the Kernux Local Privacy Implementation Plan.

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

## 1.1 Economic boundary

Local privacy and zero runtime COGS reinforce each other. `LOCAL_SUBSCRIPTION_ZERO_RUNTIME_COGS.md` is normative for cost ownership: local execution uses the user's hardware/accounts, and external providers use user/organization-owned credentials unless they are a separately priced managed service. Privacy controls must never be weakened to reduce cost, and founder-funded cloud infrastructure must never be introduced merely to simplify the local path.

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

