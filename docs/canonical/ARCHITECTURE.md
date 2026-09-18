# Canonical Architecture

## 1. Architectural objective

Kernux must unify three traditionally separate systems without turning into a coupled bundle of donor applications:

1. agent workspace/orchestration;
2. browser/web execution;
3. computer/host execution.

The architectural answer is a **capability kernel** with provider-neutral contracts, an explicit privileged boundary, a durable task/evidence model, and adapters around all external agents/runtimes/tools.

## 2. System shape

```text
+-------------------------------------------------------------------+
|                           User surfaces                           |
|                                                                   |
| Desktop Workspace | CLI | Mobile Companion | Future Web Console   |
+------------------------------+------------------------------------+
                               |
                               v
+-------------------------------------------------------------------+
|                       Kernux Orchestrator                         |
|                                                                   |
| Task Graph | Agent Fleet | Context | Compare | Synthesis | Budget |
| Browser Strategy | Tool Routing | Verification Coordination       |
+------------------------------+------------------------------------+
                               |
                    typed capability requests
                               |
                               v
+-------------------------------------------------------------------+
|                         kernuxd (Rust)                            |
|                 privileged local authority                        |
|                                                                   |
| Capability Policy | Runtime Registry | Host Process Supervisor    |
| Secret Broker      | Event Ledger     | Artifact CAS               |
| Enrollment/Keys    | Storage          | Approval Enforcement       |
+----------+-------------------+----------------------+--------------+
           |                   |                      |
           v                   v                      v
+----------------+   +--------------------+   +---------------------+
| Browser Fabric |   | Computer Fabric    |   | Tool/Agent Fabric   |
|                |   |                    |   |                     |
| Local Chromium |   | Filesystem         |   | CLI agents          |
| CDP/Playwright |   | PTY/process        |   | API agents          |
| WebMCP         |   | Documents/data     |   | A2A agents          |
| AgentQL        |   | Native UI adapters |   | MCP/Skills/OpenAPI  |
| TinyFish       |   | Clipboard/apps     |   | Native integrations |
+--------+-------+   +---------+----------+   +----------+----------+
         |                     |                         |
         +---------------------+-------------------------+
                               |
                               v
+-------------------------------------------------------------------+
|                         Runtime Fabric                            |
|                                                                   |
| Local host | WSL | Container | VM | SSH | Remote device | Cloud  |
+-------------------------------------------------------------------+
```

## 3. Privilege model

### 3.1 `kernuxd` owns authority

`kernuxd` is a small Rust daemon that owns privileged decisions and durable local truth:

- capability grants and policy evaluation;
- filesystem/process/PTY host operations;
- sandbox lifecycle authority;
- secret references and brokered use;
- machine/runtime enrollment and device identity;
- event and evidence persistence;
- artifact content-addressed storage;
- privileged operation audit.

The renderer, model, agent adapter, browser page, plugin, and remote peer are never authorization authorities.

### 3.2 Renderer remains unprivileged

The desktop renderer can render state and request actions, but it must not receive ambient Node or arbitrary host authority. Electron preload exposes a narrow typed bridge. Privileged actions still pass through the kernel policy path.

### 3.3 Orchestrator is powerful but not privileged

The TypeScript orchestrator may decide *what to request* and coordinate agents/tools, but it cannot silently bypass kernel policy. This is the main defense against a compromised plugin, agent prompt, browser page, or renderer component acquiring host power.

## 4. Recommended repository shape

The initial monorepo should converge toward:

```text
apps/
  desktop/                  Electron + React workspace
  mobile/                   companion application (later phase)
  web/                      optional remote/team console (later)

crates/
  kernuxd/                  privileged daemon
  kernux-policy/            capability policy primitives
  kernux-store/             SQLite/event/artifact metadata
  kernux-runtime-host/      host process/filesystem/PTY primitives
  kernux-identity/          device identity and signed grants

packages/
  protocol/                 schemas + generated TS types
  orchestrator/             task graph and fleet coordination
  agent-adapters/           CLI/API/A2A agent adapters
  browser-runtime/          browser contract and providers
  computer-runtime/         computer capability facade
  tool-runtime/             MCP/Skills/OpenAPI/native tools
  git-runtime/              folders/worktrees/git/review
  evidence/                 evidence builders/readers
  ui-system/                design tokens and shared UI primitives
  sdk/                      public TypeScript SDK when stable

specs/
  CURRENT.md
  tasks.md

docs/
  canonical/
  research/
  adr/                      individual ADRs once implementation begins

third_party/
  notices/
  provenance/               machine-readable donor/import records
```

The exact folder names may be refined before implementation, but the boundaries must survive.

## 5. Technology choices

### Desktop

- Electron for the first desktop generation because it maximizes reuse of proven Orca terminal/workspace/runtime code and cross-platform behavior.
- React 19 + TypeScript for UI.
- Vite/electron-vite class tooling for build speed.
- A tokenized design system rather than ad hoc styling.

Electron is a shell choice, not a privilege model. The Rust daemon remains the privileged authority.

### Core daemon

- Rust stable.
- Async runtime chosen only when required by process/IPC/network needs.
- SQLite for local metadata and event indexing, using WAL and explicit migrations.
- Content-addressed artifact store on disk using SHA-256 digests.

### Workspace/package tooling

- `pnpm` workspace for JavaScript/TypeScript packages.
- Cargo workspace for Rust crates.
- One root task runner only if it measurably simplifies cross-language gates; avoid adding build orchestration for fashion.

### Browser

- Chromium/CDP foundation.
- Playwright-class deterministic control where appropriate.
- WebMCP adapter when exposed and supported.
- semantic/natural-language layer through AgentQL/TinyFish-derived capability.
- vision/computer-use adapter as fallback.

### IPC

Local daemon IPC should use a private local transport, not an unauthenticated TCP port.

Preferred shape:

- Unix domain socket on macOS/Linux;
- named pipe on Windows;
- framed request/response + event streaming;
- schema-versioned JSON/CBOR payloads or another inspectable encoding;
- launch nonce/session binding;
- generated contract types on both Rust and TypeScript sides.

The encoding is an implementation detail; stable method/event semantics are the contract.

## 6. Core data model

Canonical identity and revision semantics for Project, Task, WorkUnit, Run, AgentSession, Runtime, Artifact, Evidence, and Event are frozen in [`IDENTITY_AND_REVISION_MODEL.md`](IDENTITY_AND_REVISION_MODEL.md). The entity descriptions below define meaning; they do not override that identity/revision contract.


### Workspace

A user-level container for layout, open projects, enrolled runtimes, and preferences.

### Project

A durable work context: repository/folder, policies, integrations, skills, task history, and artifacts.

### Task

A user outcome with one or more plan revisions and work units.

### WorkUnit

A bounded unit of execution. Kernux WorkUnits are designed to map cleanly to SpecGrain Grains when repository work is being performed.

Required properties include:

- id and revision;
- outcome;
- scope in/out;
- dependencies;
- requested capabilities;
- target runtime constraints;
- acceptance/evidence requirements;
- risk/recovery metadata;
- budget and deadline hints.

### Run

One execution occurrence of a Task or WorkUnit. Re-running identical content creates a distinct occurrence identity.

### AgentSession

One logical adapter-backed agent conversation/execution continuity. Provider identity remains foreign metadata; mutable usage/status are projected state around the immutable AgentSession identity.

### CapabilityRequest

A typed proposed operation with canonical request identity, bounded subject/run scope, exact action, canonical resource URI, exact runtime, requested constraints, and provenance. The v1 semantic contract is [`CAPABILITY_AND_GRANT_MODEL.md`](CAPABILITY_AND_GRANT_MODEL.md).

### Grant

An immutable bounded authorization record binding exact subject scope, action, canonical resource scope, runtime, constraints, consequence ceiling, finite lifetime/use budget, issuer/policy revision, and delegation depth. Persistent convenience policy issues bounded Grants rather than creating an immortal wildcard Grant.

### Runtime

An execution host implementing a negotiated subset of Kernux runtime capabilities.

### Artifact

A logical immutable-content output record with its own Artifact ID plus a separate content digest, media type, size, producer, source event, optional semantic metadata, and retention class.

### Evidence

An assertion backed by observed checks and bound to exact task/work-unit/run/implementation/artifact revisions.

### Event

Append-oriented fact in the run timeline. Events describe what happened; mutable projections derive current UI state.

## 7. Event and replay model

Kernux should be event-oriented from the beginning.

Examples:

```text
task.created
plan.revised
workunit.ready
agent.session.started
capability.requested
capability.approved
browser.navigated
browser.observed
process.started
file.changed
git.diff.observed
verification.started
verification.passed
artifact.created
run.suspended
runtime.contact_lost
runtime.contact_restored
run.completed
```

Rules:

- durable Event IDs are unique; later event-envelope contracts define explicit per-stream ordering coordinates and MUST NOT use UUIDv7 lexical order as event-order authority;
- events have producer identity and timestamp;
- large payloads live in the artifact store and are referenced by digest;
- current state is a projection and can be rebuilt;
- replay never means blindly repeating side effects; operations declare replay semantics: deterministic, idempotent, compensatable, or non-replayable;
- fork-from-checkpoint creates a new run lineage rather than mutating history.

## 8. Runtime contract

Every runtime advertises capabilities and versions. A runtime may be local, WSL, SSH, sandbox, remote device, or cloud.

Representative contract surfaces:

```text
runtime.describe
runtime.health
runtime.files.*
runtime.process.*
runtime.pty.*
runtime.git.*
runtime.browser.*
runtime.computer.*
runtime.artifacts.*
```

A caller must not infer a capability from runtime type. Negotiate it.

### Status truth

The execution host owns process and agent lifecycle truth.

Contact state and execution state are distinct:

```text
contact: connected | disconnected | degraded
execution: live | exited | unverifiable
```

A disconnect can make execution `unverifiable`; it must never be rewritten as `exited` merely because the controller lost contact.

## 9. Agent adapter contract

Agent providers vary radically, so Kernux normalizes only what is real.

Adapters declare supported features rather than pretending all agents are identical:

```text
createSession
send
interrupt
resume
status
usage
snapshot
close
```

Negotiated features may include:

- streaming text/events;
- structured tool calls;
- image/file input;
- native plan mode;
- resume/session id;
- provider-managed computer use;
- native MCP;
- context/usage reporting;
- auth/account switching.

CLI agents run through supervised PTYs and captured transcripts. API agents use provider SDK/API adapters. Remote opaque agents may use A2A.

## 10. Browser action hierarchy

For each intended browser action, use the highest-structure reliable path:

1. **WebMCP/first-party typed tool** — preferred when trustworthy and available.
2. **Deterministic browser primitive** — DOM/accessibility/CDP/known selector.
3. **Semantic primitive** — AgentQL-style natural-language extraction/element targeting.
4. **Vision/computer use** — screenshot/coordinate interaction when structure is unavailable.

Every level records the chosen method and observation/evidence used to justify the action.

This hierarchy is a core Kernux differentiator: the platform can move between reliable automation and general computer use without forcing one mechanism onto every page.

## 11. Computer action hierarchy

Likewise prefer:

1. typed application/tool API;
2. OS accessibility/UI automation tree;
3. terminal/CLI or file-level operation where semantically equivalent;
4. vision + mouse/keyboard fallback.

The goal is not to imitate a human when a safer structured operation exists.

## 12. Storage

### Metadata store

SQLite owns local durable metadata:

- projects/tasks/runs;
- event indexes;
- runtime enrollment metadata;
- grants/policies;
- artifact metadata;
- adapter configuration references;
- migrations/schema version.

### Artifact store

Content-addressed files are stored separately from SQLite. Deduplication is by digest, not filename.

Sensitive artifacts receive retention/encryption classification.

### Secrets

Secret plaintext is not ordinary application data. See the security model. Store references and broker policy in the project database; secret material uses OS keychain/credential facilities or an approved secret provider.

## 13. Remote architecture

Remote runtimes are paired devices with explicit identity and capability negotiation.

Requirements:

- enrolled device identity;
- mutual authentication;
- version/capability negotiation;
- reconnect and resumable streams;
- operation ids/idempotency boundaries;
- no duplicate side effects after ambiguous retry;
- durable host-side execution status;
- explicit revocation;
- auditable remote grants.

Mobile is a remote control/client surface, not a trusted shortcut around the same rules.

## 14. Git and project architecture

Kernux supports both git projects and ordinary folders.

Git worktrees are an isolation mechanism, not the definition of a workspace.

Git adapters must support:

- local native Git;
- WSL-hosted Git;
- SSH-hosted Git;
- capability detection/fallback across Git versions;
- provider-neutral review concepts with GitHub/GitLab adapters later.

## 15. Observability

Every run produces structured telemetry independent of UI logs:

- task/run/work-unit ids;
- agent/provider/model identity where available;
- tool/capability operation;
- runtime id/type;
- latency;
- token/cost usage where available;
- retries/failures;
- approval latency;
- verification outcomes.

Adopt OpenTelemetry semantic conventions where stable/applicable and keep content-bearing prompt/tool payload export opt-in.

## 16. Compatibility policy

Public and remote contracts use explicit versions.

Rules:

- additive optional fields are preferred;
- unknown optional fields must be ignored safely;
- new behavior with side effects requires capability negotiation;
- old clients and new hosts are a normal deployment state;
- migration paths are tested, not documented only;
- runtime/provider adapters expose capability versions independently from app version.

## 17. Architecture constraints that must not regress

1. Models never authorize themselves.
2. Renderer never becomes an ambient privileged host process.
3. Browser content is never trusted instruction authority.
4. Remote disconnect never proves process death.
5. Agent self-report never proves task completion.
6. Provider-specific behavior never defines the core public contract.
7. Work must remain inspectable after restart.
8. Side-effect retries require idempotency or explicit ambiguity handling.
9. Secrets must not be casually copied into agent context or logs.
10. Donor code must remain provenance-traceable.
