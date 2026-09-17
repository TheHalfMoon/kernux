# Context, Search, and Memory

## 1. Purpose

Kernux cannot be an Agent Operating Environment if every agent starts from a blank chat window or if useful context is assembled by ad hoc prompt concatenation. Context is a first-class product and security surface.

This contract defines how Kernux discovers, indexes, selects, scopes, renders, remembers, invalidates, and deletes context without turning local-first work into an opaque hosted memory system.

The design goal is:

> Give every agent the smallest trustworthy context needed for the current outcome, with provenance and user control.

## 2. Architectural boundary

Context services sit between projects/artifacts/events and agent/model adapters. They do not grant capabilities.

```text
Project / Task / Files / Events / Artifacts / Integrations
                         |
                         v
                  Context Engine
       +-----------------+------------------+
       |                 |                  |
   Search/index      Context assembly     Memory
       |                 |                  |
       +-----------------+------------------+
                         |
                typed ContextBundle
                         |
                         v
             Agent / model adapter
```

The capability kernel remains the authority for reading any source. A context index is not a permission bypass: retrieval results are filtered by the current task/project/runtime policy before they can enter an agent bundle.

## 3. Core records

The v1 schema should represent at least:

### ContextSource

- stable source id;
- source type (`file`, `artifact`, `event`, `message`, `web`, `tool`, `memory`, `integration`, `runtime`);
- project/task scope;
- provenance URI;
- source revision or digest;
- trust classification;
- sensitivity classification;
- freshness metadata;
- owning runtime/provider when relevant.

### ContextItem

- source id + exact span/chunk/symbol/range;
- extracted text or artifact reference;
- revision/digest;
- retrieval method;
- relevance metadata;
- token/byte cost;
- redaction metadata;
- reason selected;
- citation/display handle.

### ContextBundle

- bundle id and digest;
- target agent/model/session;
- task/work-unit revision;
- ordered selected items;
- omitted/truncated item summary;
- total token/byte estimate;
- assembly policy/version;
- generated summary references when summaries are used.

### MemoryRecord

- memory id;
- scope (`run`, `task`, `project`, `user`, `organization`);
- statement/data reference;
- provenance;
- author (`human`, `verified-system`, `agent-proposed`);
- confidence/status;
- sensitivity;
- created/updated/expiry timestamps;
- revision history;
- deletion/tombstone state.

Memory is not a raw transcript dump.

## 4. Search hierarchy

Kernux should use the cheapest deterministic method that can answer the query before escalating to more expensive or probabilistic retrieval.

Recommended hierarchy:

1. exact path/name lookup and quick-open index;
2. bounded text search (ripgrep-class behavior);
3. symbol/structure search from parser/LSP/tree-sitter-style indexes;
4. SQLite FTS or equivalent local full-text index for tasks/artifacts/docs;
5. optional semantic embedding retrieval;
6. optional graph/code-relationship retrieval where it measurably improves outcomes.

Semantic search is an enhancement, not the only path to local knowledge. A missing embedding service must not make local projects unusable.

All search must be bounded. Repository history, remote refs, archives, generated directories, and very large files require explicit budgets and exclusions rather than unbounded scans.

## 5. Index ownership and freshness

Indexes are derived state, never primary truth.

Requirements:

- source revisions/digests bind indexed entries;
- file watcher events invalidate or update local indexes;
- git checkout/worktree changes invalidate affected source entries;
- artifact/event indexes update transactionally with durable metadata where practical;
- stale results remain marked stale or are omitted;
- corrupt indexes can be deleted and rebuilt without losing primary project/run data;
- indexes are project-scoped by default to avoid accidental cross-project disclosure.

## 6. Context assembly

Context assembly is a deterministic, inspectable pipeline around probabilistic retrieval components.

A typical assembly pass:

1. resolve task/work-unit and current capability scope;
2. gather explicit user attachments and pinned sources;
3. gather mandatory system/project instructions;
4. retrieve bounded task-relevant project/artifact/history context;
5. apply trust/sensitivity filters;
6. deduplicate by source revision/span;
7. prioritize exact evidence over summaries;
8. fit the target model/provider budget;
9. record omissions/truncation;
10. emit a digestible ContextBundle with provenance.

The user and verifier should be able to answer: **what context did this agent receive, from where, and at which revision?**

## 7. Context budgeting

Each model/agent adapter declares practical input limits and supported content types.

The context engine tracks:

- token estimate;
- raw byte size;
- image/document attachment limits;
- provider-specific message/tool overhead;
- reserved output/tool budget;
- summary cost and lineage.

Do not silently discard the most important context because a provider window is smaller than expected. If mandatory context cannot fit, the task must be refined, summarized with traceable lineage, or routed to a suitable provider.

## 8. Summaries and compaction

Summaries are derived artifacts, not replacements for source truth.

Every durable summary used as context records:

- source item ids/digests;
- summarizer/provider/version when model-generated;
- prompt/policy version or deterministic algorithm id;
- creation time;
- sensitivity/redaction state;
- invalidation rules.

A summary derived from changed source material is stale until regenerated or explicitly accepted as historical.

Kernux does not require storage of private chain-of-thought. Durable records should preserve observable actions, decisions, user-facing reasoning summaries when available/appropriate, evidence, and source lineage rather than hidden model reasoning.

## 9. Memory model

### Run memory

Ephemeral execution facts needed to continue one run.

### Task memory

Decisions, constraints, accepted outputs, and unresolved questions for one task lineage.

### Project memory

Stable project conventions, architecture decisions, user-approved defaults, and known facts scoped to one project.

### User memory

Explicitly user-owned preferences that genuinely improve work across projects.

### Organization memory

Policy-managed shared conventions and references. Organization memory can constrain but must not silently overwrite project/user truth.

## 10. Memory write policy

Agents may **propose** durable memory; they do not gain unlimited authority to permanently store arbitrary statements.

Rules:

- low-risk run/task facts may be recorded automatically when directly machine-observed;
- inferred preferences, identity claims, secrets, legal/medical/financial facts, or broad cross-project memories require stricter policy and often explicit user action;
- source provenance is mandatory for machine-derived facts;
- secret plaintext is never a normal memory payload;
- untrusted web/document content cannot become durable trusted memory merely because an agent repeated it;
- conflicting memories remain explicit rather than silently choosing one;
- users can inspect, edit, pin, expire, export, and delete durable memory.

## 11. Memory poisoning defenses

Memory retrieval is part of the prompt-injection threat model.

Every MemoryRecord carries provenance/trust. Retrieval policy can exclude or label:

- unverified agent proposals;
- untrusted web-derived statements;
- stale facts;
- cross-project data;
- records whose source was deleted or revoked;
- records created under a compromised/revoked integration.

A retrieved memory cannot authorize a new capability. Capability authority still comes from user/org/kernel policy.

## 12. Search and quick-open UX

Kernux should expose one universal search/quick-open surface across:

- files and symbols;
- projects/workspaces;
- tasks/runs;
- agents/sessions;
- artifacts/evidence;
- commands;
- tools/skills/MCP integrations;
- machines/runtimes;
- approved memory.

Results show type, scope, freshness, and source. Search must remain useful offline for local sources.

## 13. Agent-visible citations

When an agent answer/result depends on retrieved context, adapters should preserve source handles so the UI can render inspectable provenance.

For generated reports/research, references should survive export where the artifact format supports them.

## 14. Privacy and local-first rules

- local indexes stay local by default;
- embeddings are computed locally when configured, or sent to an explicit external provider under declared data-boundary policy;
- no prompt/file content is exported merely to improve product analytics;
- indexes and memory follow project/user deletion and export rules;
- remote runtimes only receive the ContextItems required for their work unit;
- cross-project retrieval is opt-in and policy-scoped.

## 15. Performance requirements

Measure and ratchet:

- initial project scan time;
- incremental update latency;
- quick-open latency;
- text/symbol search latency;
- memory retrieval latency;
- context-assembly latency;
- index disk footprint;
- large-repository behavior;
- cold rebuild behavior.

Search should degrade by returning bounded partial results, not by retaining unbounded output in memory.

## 16. Implementation mapping

This contract refines existing macro phases rather than creating a parallel architecture:

- **P02** stores source/index metadata and enforces context read scope;
- **P03** implements universal search/quick-open, context inspector, and memory management UX;
- **P04** supplies bounded filesystem/document sources;
- **P05** consumes ContextBundles in agent/model sessions;
- **P06** contributes web observations with trust/provenance;
- **P07** binds context to Task/WorkUnit revisions and fleet candidates;
- **P09** preserves context bundle/event lineage for replay/evidence;
- **P11** adds integration-provided context under the same policy;
- **P14** adds organization-scoped memory/policy only after local semantics are proven.

SpecGrain refinement must create explicit child work for these obligations before the relevant phase exit gate is considered PROVEN.

## 17. Acceptance gates

Before the first external alpha:

1. a user can quick-open/search a large local project without cloud dependency;
2. an agent run exposes an inspectable ContextBundle/provenance view;
3. context is invalidated correctly after source revision changes;
4. cross-project access is denied by default;
5. durable memory is inspectable/exportable/deletable;
6. a prompt-injected web page cannot promote itself into trusted durable memory or privilege;
7. model/provider context limits produce explicit truncation/refinement behavior rather than silent loss;
8. index deletion/rebuild does not destroy primary task/project evidence.

## 18. Rule

**Context is selected evidence, not ambient access. Memory is user-governed state, not an agent-owned diary.**
