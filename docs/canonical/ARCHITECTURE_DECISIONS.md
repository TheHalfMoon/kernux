# Architecture Decisions

This file records first-generation decisions that constrain implementation. These are planning decisions, not immutable forever. Changes require an explicit ADR with migration and compatibility impact.

## ADR-0001 — Product category is Agent Operating Environment

**Status:** Accepted

Kernux is defined around tasks, capabilities, runtimes, agents, evidence, and collaboration. It is not constrained to a coding IDE category even though developers are the first wedge.

## ADR-0002 — Capability contracts are the architectural center

**Status:** Accepted

Core callers request capabilities rather than donor/provider-specific operations. Provider implementations remain replaceable behind versioned contracts.

Why: combining donor internals directly would create permanent coupling and make future providers expensive.

## ADR-0003 — Privileged authority lives in `kernuxd`

**Status:** Accepted

A separate Rust daemon owns host privilege, policy enforcement, runtime enrollment, secret brokering, durable event truth, and artifact authority.

The Electron renderer is never the ambient privileged authority.

## ADR-0004 — Electron + React for first desktop generation

**Status:** Accepted for Gen 1

Use Electron/React/TypeScript to maximize reuse of proven Orca workspace/terminal/browser behavior and accelerate cross-platform delivery.

This does not authorize renderer-level host privilege. A future shell migration is possible because core privileged/runtime contracts live outside Electron.

## ADR-0005 — Rust for privileged core

**Status:** Accepted

Use Rust for `kernuxd`, capability policy primitives, runtime host operations, identity/crypto-sensitive code, and durable-store boundaries where appropriate.

Goals: memory safety, portable native binaries, explicit concurrency, strong typed contracts.

## ADR-0006 — Local-first core, cloud optional

**Status:** Accepted

A developer/user can perform meaningful Kernux work with desktop + local daemon without a Kernux account or hosted control plane.

Cloud services may later add team sync, relay, managed runtimes, marketplace, and organization policy, but core execution cannot depend on them.

## ADR-0007 — Event-oriented durable run history

**Status:** Accepted

Runs emit append-oriented events; current UI/task state is a projection.

Why: restart recovery, replay inspection, audit, remote reconciliation, and evidence all require stronger state than chat transcripts.

Kernux is not committing to pure event sourcing for every table. Mutable projections/indexes are allowed; durable events remain the historical facts.

## ADR-0008 — SQLite metadata + content-addressed artifact store

**Status:** Accepted

Use SQLite for local structured metadata/event indexes and a filesystem CAS keyed by SHA-256 for potentially large artifacts.

Do not put large terminal transcripts, screenshots, archives, or binary outputs directly into ordinary relational rows unless a bounded special case requires it.

## ADR-0009 — Secrets are brokered references

**Status:** Accepted

Secret plaintext belongs in OS credential storage or an approved secret provider. Kernux stores references, policies, and audit metadata.

Prefer handle/broker flows over inserting secret values into prompts, logs, argv, or sandbox files.

## ADR-0010 — Permission is kernel-enforced and provenance-aware

**Status:** Accepted

Capability grants bind subject + action + resource + runtime + constraints. High-consequence actions can consider causal provenance from untrusted content.

Prompt text cannot grant privilege.

## ADR-0011 — Isolation modes are explicit

**Status:** Accepted

Kernux exposes host, container, VM/microVM, SSH/remote, and cloud execution modes. Generated/untrusted code does not automatically execute on the host.

Do not call simple command blocklists a sandbox.

## ADR-0012 — Execution host owns lifecycle truth

**Status:** Accepted

The runtime actually running the process/agent owns process lifecycle status. Controller connectivity is a separate dimension.

Canonical execution states include `live`, `exited`, and `unverifiable`. Contact loss alone never becomes `exited`.

## ADR-0013 — Side effects require operation identity and retry semantics

**Status:** Accepted

Requests and side-effecting operations have distinguishable identities. A retry after ambiguous network failure cannot repeat an effect unless idempotency or compensation is established.

## ADR-0014 — Protocol-native integration

**Status:** Accepted

Prefer open protocols where they match the layer:

- ACP for editor/client ↔ coding agent;
- A2A for independent/opaque agent ↔ agent;
- MCP for tools/context/integrations;
- WebMCP for page-exposed browser tools;
- CDP/Playwright for browser control;
- Kernux Runtime Protocol for privileged runtime semantics not supplied by those standards.

## ADR-0015 — Stable ACP v1 before experimental v2

**Status:** Accepted

Build ACP v1 compatibility first. Experimental ACP v2 may be researched behind an explicit feature/version gate and must not define stable Kernux interfaces prematurely.

## ADR-0016 — MCP targets 2026-07-28 generation

**Status:** Accepted

New native MCP work targets the final 2026-07-28 protocol model including stateless core, extension negotiation, Tasks extension, and authorization hardening.

Compatibility with earlier widely used MCP versions may be provided through adapters where justified.

## ADR-0017 — Structured browser actions before vision

**Status:** Accepted

Browser strategy order:

1. WebMCP/first-party typed tools;
2. DOM/accessibility/CDP deterministic action;
3. semantic/natural-language provider such as AgentQL/TinyFish;
4. vision + coordinate computer use.

Vision remains essential fallback, not the default when structure exists.

## ADR-0018 — Structured computer control before coordinates

**Status:** Accepted

Prefer native tool/app APIs, accessibility trees, file/CLI operations, and semantic controls before raw mouse/keyboard coordinates where outcomes are equivalent.

## ADR-0019 — Folder workspaces are first-class

**Status:** Accepted

A Project may be a git repository or ordinary folder. Worktrees are one isolation mechanism, not the definition of a Kernux project.

## ADR-0020 — Git isolation uses worktrees where useful, not universally

**Status:** Accepted

Parallel coding candidates should default to separate git worktrees when the source project supports them. Sandboxes/branches/folder snapshots remain valid alternatives depending on runtime and project type.

## ADR-0021 — Provider features are negotiated, not normalized by fiction

**Status:** Accepted

Agent, runtime, browser, and tool adapters advertise actual capabilities. Kernux does not invent fake support for resume, plan mode, computer use, cost telemetry, structured tools, or file input when a provider lacks them.

## ADR-0022 — One command bar is primary UX

**Status:** Accepted

Ordinary users express outcomes in one command bar. Workflow graphs, provider knobs, and low-level logs are progressively disclosed.

## ADR-0023 — Multi-agent fleet is evidence-driven

**Status:** Accepted

Parallel agents can propose candidates. Comparison distinguishes measured evidence from model judgment. Synthesis creates a new lineage/candidate and must re-run relevant proof.

## ADR-0024 — Replay is not blind side-effect repetition

**Status:** Accepted

Every operation declares replay semantics: deterministic, idempotent, compensatable, inspect-only, or non-replayable. Fork-from-checkpoint starts a new run lineage.

## ADR-0025 — Donor code is imported behind Kernux contracts

**Status:** Accepted

Orca, TinyFish/AgentQL, and Desktop Commander source may be reused under founder authorization and observed licensing, but imports require pinned provenance, notices, characterization, and source-to-destination mapping.

## ADR-0026 — Donor imports and adaptations should be separable

**Status:** Accepted

When practical, preserve a mechanical import commit before Kernux-specific refactoring. This improves auditability and upstream comparison.

## ADR-0027 — Design system is tokenized and Kernux-owned

**Status:** Accepted

Kernux may reuse proven UI primitives/interactions, but establishes its own brand, tokens, components, accessibility rules, and visual identity. Raw ad hoc palette/style proliferation should be lintable.

## ADR-0028 — Runtime/UI contracts generated from schemas

**Status:** Accepted

Define cross-language wire schemas once and generate/validate Rust + TypeScript types. Hand-maintained duplicate interface definitions are not acceptable for critical daemon/remote contracts.

## ADR-0029 — Remote peers update independently

**Status:** Accepted

Mixed desktop/runtime/mobile versions are normal. Remote protocol changes must be additive or capability-negotiated, with conformance fixtures for supported combinations.

## ADR-0030 — Local IPC is private and authenticated/bound

**Status:** Accepted

Prefer Unix sockets/named pipes with launch/session binding over a world-accessible unauthenticated TCP listener. Renderer/preload access remains narrow and typed.

## ADR-0031 — OpenTelemetry-compatible observability

**Status:** Accepted

Use OpenTelemetry concepts for task/run/provider/runtime tracing where practical. Content-bearing prompts, files, and tool payloads are opt-in for export.

## ADR-0032 — SpecGrain + Diffcipline govern implementation

**Status:** Accepted

Large phases are refined into independently verifiable Grains. Exact-diff proof is required before `PROVEN` completion. Agents cannot self-certify.

## ADR-0033 — Security is a product surface

**Status:** Accepted

Permissions, runtime boundaries, secret use, provenance, approvals, audit, and recovery are visible user concepts. Security must not be hidden only in settings or developer docs.

## ADR-0034 — Mobile is a steering surface

**Status:** Accepted

Mobile prioritizes status, approvals, follow-up, pause/stop, result/diff inspection, screenshots, and remote launch. It is not required to duplicate the full desktop editing environment.

## ADR-0035 — Team/cloud features come after local core reliability

**Status:** Accepted

Do not let organization billing, cloud sync, or marketplace architecture block proving the local product. Team services integrate through the same identities/policies/events later.

## ADR-0036 — No hidden global autonomy switch

**Status:** Accepted

`Autonomous` is a compiled permission profile for a bounded task/runtime/scope. Kernel hard constraints and organization policy remain active.

## ADR-0037 — Release claims require reproducible evidence

**Status:** Accepted

No `best`, `most reliable`, or superiority claim without a predeclared comparative protocol and preserved results, including negative evidence.

## ADR-0038 — Canonical entity IDs are UUIDv7; revision and ordering are separate contracts

**Status:** Accepted

Project, Task, WorkUnit, Run, AgentSession, Runtime, Artifact, Evidence, and Event use RFC 9562 UUIDv7 canonical Kernux IDs in lowercase hyphenated textual form.

Canonical IDs are opaque, non-secret identifiers and never authorization tokens. Provider/native IDs, content digests, repository revisions, paths, and other foreign identities remain separate metadata.

Revisioned entities use positive uint32 revisions beginning at 1 with exact expected-revision compare-and-swap and fail-closed overflow. Run, AgentSession, Evidence, and Event are immutable identity records at semantic revision 1; their live or current views are projections over events and related state rather than in-place identity mutation.

UUIDv7 timestamp or lexical order is not causal, event, freshness, replay, or security authority. Contracts that need ordering must define explicit ordering coordinates.

Artifact byte identity is SHA-256 and is invariant for one Artifact ID; changed bytes create a new Artifact identity. Schema-source and generated Rust or TypeScript representation remain deferred to the planned P01 schema-generation task.

## ADR-0039 — Authorization uses exact actions, canonical resources, bounded Grants, and fail-closed intersection

**Status:** Accepted

Every privileged operation is proposed as a CapabilityRequest and evaluated through one provider-neutral authorization model.

Core v1 actions are exact lower-case dot-separated identifiers; issued Grants do not contain action wildcards. Resources use the canonical `kernux://` URI model and are matched through parsed exact/subtree semantics, never raw string prefix.

The trusted kernel owns consequence classification and may only raise, never lower, contextual consequence. Untrusted browser/document/tool/remote content can influence a request but cannot mint or widen authority.

Issued Grants are immutable bounded authorization metadata with exact subject/action/runtime, canonical resource scope, typed conjunctive constraints, consequence ceiling, finite lifetime/use budget, policy/issuer binding, and default-zero delegation depth. Persistent permission choices are policy rules that issue bounded Grants rather than immortal wildcard Grants.

Delegation requires independent `grant.delegate` authority and can only narrow. Child operations consume applicable ancestor budgets so delegation cannot multiply authority.

Remote execution requires the intersection of controller authorization, Kernux/kernel policy, negotiated runtime capability, and remote-host policy. Unknown or missing mandatory authority fails closed.

Wire representation, policy-engine implementation, runtime cancellation/errors, Event envelope, persistence, and generated Rust/TypeScript contracts remain owned by later dependency-ordered work.

## ADR-0040 — Runtime truth uses negotiated capability, stable OperationId, host-owned observation, and reconcile-before-retry

**Status:** Accepted

Runtime/provider type never implies capability. Kernux explicitly negotiates the provider-neutral capability/action contract, then separately applies the existing authorization and remote-host-policy intersection.

Contact and execution state are independent. The closed conceptual contact vocabulary is `connected | degraded | disconnected`; execution is `live | exited | unverifiable`. Disconnect, timeout, controller restart, missing heartbeat, request failure, or cancellation acceptance cannot establish `exited`.

`RequestId` identifies one protocol attempt. `OperationId` identifies one logical side-effecting operation across transport retries and reconnects. One OperationId is bound to one canonical operation fingerprint. Same-ID/different-fingerprint reuse fails closed; same-ID/same-fingerprint replay resolves the existing logical operation rather than creating a duplicate side effect.

After a `may_have_started` outcome, callers reconcile the existing OperationId before creating any fresh operation identity. Unknown retry safety defaults to reconciliation, not optimistic re-execution.

Cancellation is a request against an OperationId. Acceptance is not termination proof. Only authoritative execution-owner observation establishes terminal `exited` state and cause.

Structured errors separate protocol-request failure from operation truth and carry provider-neutral retry guidance plus side-effect certainty. Provider-native status/error data remains foreign diagnostic metadata.

Reconnect authenticates Runtime identity, renegotiates capability/protocol support, rechecks current authority for new admissions, and reconciles outstanding operations. It cannot resurrect expired/revoked Grants or silently transfer operation authority to a changed trust identity.

Event-envelope representation, generated schema encoding, global compatibility rules, transport implementation, operation-ledger persistence, and concrete runtime adapters remain dependency-ordered downstream work.

## ADR-0041 — Durable truth is append-only Event streams with independently bound Evidence

**Status:** Accepted

Canonical Event order is explicit per typed owner stream through contiguous `stream_seq` plus `previous_event_id`; UUIDv7 and timestamps are never ordering authority. Events and Evidence are immutable revision-1 records. Corrections, supersessions, and forks append new lineage rather than editing history.

Artifact bytes relied upon by Events/Evidence bind SHA-256. Evidence records its actual source class and observation basis; agent/model self-report cannot satisfy a stronger independent gate.

Redaction/exclusion is explicit view/export metadata. Transformed bytes receive a new Artifact identity/digest, and canonical source history is not mutated. Ordinary Event/Evidence records do not store secret plaintext.

Wire schema, canonical serialization, whole-Event digest/signature, persistent event-store layout, compatibility policy, and executable fixture corpus remain downstream work.

## ADR-0042 — JSON Schema 2020-12 is the single source for generated KRP core contracts

**Status:** Accepted

The authoritative v1 cross-language wire source is `protocol/schema/krp.v1.schema.json`, using JSON Schema Draft 2020-12 and a deliberately restricted repository-owned subset. Rust and TypeScript contract files are generated deterministically from those bytes and carry the schema SHA-256; generated files are never a competing source of truth.

The generator must fail on unsupported schema constructs rather than approximating them. Schema validation proves structural shape only and does not grant authority.

S05-T02 remains authoritative for version/unknown/additive compatibility behavior, and S05-T03 remains authoritative for the full adversarial protocol corpus.

## ADR-0043 — KRP v1 uses closed decoding and feature-negotiated additive evolution

**Status:** Accepted

`krp/1` is the current semantic compatibility family. The exact schema SHA-256 binds provenance and generated bytes but is not a negotiated wire version.

KRP v1 core objects are closed: unknown fields and unrecognized required enum/version values fail closed in both schema/Node validation and generated Rust decoding. Missing optional fields mean absence and cannot synthesize authority, feature support, terminal execution, side-effect certainty, or retry safety.

A semantics-neutral additive optional field may remain in `krp/1` only behind an explicit provider-neutral feature token, and a sender emits it only when both sides advertise support. Changes that alter required structure, closed enum meaning, authority, or side-effect semantics require an explicit version/contract path rather than permissive fallback.

Protocol mismatch blocks affected admission/interpretation but cannot establish runtime exit, cancellation, definitely-not-started state, or safe retry. Structurally valid but unnegotiated Event types are non-projecting until a supported interpreter is explicitly available.


## ADR-0044 — Owner-sustainable execution; no mandatory owner-subsidized compute

**Status:** Accepted

Kernux must preserve a complete useful execution path whose recurring variable compute cost is not involuntarily subsidized by the project owner.

For every core capability that can create recurring per-use infrastructure or provider cost, at least one viable path must be available through one or more of:

- local execution on the user's machine;
- self-hosted execution;
- BYOK (bring your own provider/API credentials);
- BYOC (bring your own compute/runtime);
- organization-owned infrastructure or provider accounts;
- an optional paid Kernux-managed service with an explicit sustainable cost/revenue boundary.

This is a **cost-allocation and architectural-independence invariant**, not a ban on cloud services, paid providers, GPUs, hosted browsers, managed search, relays, or premium convenience. Kernux may use those capabilities when they materially improve product quality, provided the user or organization explicitly chooses the funded path or the managed service is designed so that Kernux is not required to indefinitely subsidize variable user consumption.

The local/community core must not depend on promotional credits, founder-funded API keys, hidden hosted inference, hidden browser farms, or another metered service whose exhaustion would make the core product unusable.

Implementation implications:

- local/self-hosted providers are first-class where technically viable, not degraded compatibility fallbacks;
- provider-native authentication, subscriptions, quotas, and billing remain owned by the user/organization unless a Kernux-managed service explicitly assumes that boundary;
- provider contracts remain neutral so a paid provider can be replaced by a local, BYOK, BYOC, or managed implementation;
- browser/web execution must retain a local deterministic path even when semantic or hosted providers add convenience;
- model-backed agents must support user-owned/provider-owned credentials and local model paths where qualified;
- optional Kernux cloud, relay, managed runtime, inference, browser, search, storage, or team services must not become a hidden prerequisite for the local core;
- cost and usage telemetry, where available, is surfaced for budgeting but never treated as authorization or completion evidence;
- tests and release qualification must not rely on temporary free credits as the only proof of a core capability.

Product quality, security, privacy, evidence integrity, and provider independence remain first-class requirements. This decision must not be interpreted as permission to ship an inferior local path merely to claim zero owner cost.

## Change process

Any implementation discovery that invalidates one of these decisions should create an ADR rather than silently violating the plan. A replacement ADR must describe:

- old decision;
- observed evidence/problem;
- alternatives;
- new decision;
- migration/compatibility impact;
- required task-plan changes.