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

## Change process

Any implementation discovery that invalidates one of these decisions should create an ADR rather than silently violating the plan. A replacement ADR must describe:

- old decision;
- observed evidence/problem;
- alternatives;
- new decision;
- migration/compatibility impact;
- required task-plan changes.