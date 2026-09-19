# Protocol Strategy

## Goal

Kernux should be protocol-native rather than protocol-captive. Different standards solve different interoperability layers; Kernux should adopt each where it provides durable leverage and keep an internal capability model above them.

The public architecture must not collapse into `MCP everywhere` or `one proprietary protocol everywhere`.

## Protocol map

| Layer | Preferred standard / interface | Kernux role |
| --- | --- | --- |
| Editor/client ↔ coding agent | ACP v1 where supported | Native agent-client interoperability |
| Agent ↔ agent | A2A 1.0 where supported | Opaque/remote agent collaboration |
| Agent/app ↔ tools/context | MCP 2026-07-28+ | Tools/resources/integrations |
| Web page ↔ browser agent | WebMCP when available | First-party structured page actions |
| Browser automation | CDP + Playwright-class APIs | Deterministic fallback/control plane |
| Local privileged execution | Kernux Runtime Protocol | Policy-enforced host/runtime operations |
| Remote Kernux runtime | Kernux Runtime Protocol over authenticated transport | Device/runtime execution fabric |
| General HTTP services | OpenAPI/HTTP adapters | Tool integration |
| CLI/TUI programs | supervised PTY/process adapter | Universal compatibility fallback |
| Observability | OpenTelemetry-compatible telemetry | Traces/metrics/log correlation |
| Containers/artifacts | OCI-compatible formats where useful | Portable sandbox/image distribution |

## 1. ACP — Agent Client Protocol

ACP is the preferred structured path for coding agents that support it.

Why:

- it separates agent implementation from client/editor UX;
- it already targets multi-agent/editor interoperability;
- common coding agents are available through ACP implementations/registry paths;
- it provides a more structured interface than scraping TUI output.

Kernux responsibilities:

- implement an ACP client adapter;
- preserve agent-native auth/billing ownership unless explicitly proxied;
- expose ACP capabilities without forcing every agent into the same feature set;
- support stable ACP v1 first;
- treat draft/experimental protocol revisions as opt-in behind version gates;
- keep PTY fallback for agents without ACP or when a user intentionally wants native TUI behavior.

ACP does **not** replace Kernux permissions. An ACP agent may request actions, but host/browser/tool authority still flows through the capability kernel when Kernux is the execution owner.

## 2. A2A — Agent2Agent Protocol

A2A 1.0 is intended for communication between independent, potentially opaque agent systems.

Kernux uses A2A for:

- discovering external agent capabilities;
- delegating bounded tasks to remote/organizational agents;
- exchanging files/structured data/messages;
- tracking collaborative task status where the remote agent owns execution.

Do not use A2A as the local privileged host-control protocol. External agent authority must remain bounded by the Kernux task/capability contract.

A2A tasks map to Kernux external-agent work units through an adapter; they do not become the canonical local data model.

## 3. MCP — Model Context Protocol

Target the current final `2026-07-28` protocol generation for new native support while maintaining compatibility adapters where ecosystem value requires older MCP versions.

Planning implications of the 2026-07-28 generation:

- protocol core is stateless;
- requests are self-describing;
- discovery is explicit/optional rather than session initialization authority;
- method/tool names can be routed/authorized through headers;
- list results can carry cache metadata;
- long-running work moves through the Tasks extension;
- authorization hardening matters for desktop/remote clients;
- extension negotiation is first-class;
- older stateful assumptions must not leak into Kernux architecture.

Kernux MCP roles:

### MCP client

Connect third-party servers and expose their declared tools/resources to agents under Kernux policy.

### MCP server

Expose selected Kernux capabilities to authorized external clients without giving those clients ambient host authority.

### MCP gateway

Optionally route/mediate organization-managed MCP endpoints with:

- policy intersection;
- secret brokering;
- allowlists;
- telemetry;
- version negotiation;
- permission manifest review.

### MCP Tasks

Use MCP Tasks for long-running MCP-side work when a server supports the extension. Map task identity/status into a Kernux external-operation record rather than assuming MCP task state is the entire Kernux run.

## 4. WebMCP

WebMCP is a proposed/experimental web standard and therefore must be capability-negotiated, not assumed.

When a page exposes WebMCP tools, Kernux should prefer them over visual clicking when:

- the tool is semantically appropriate;
- origin/tool identity is clear;
- policy allows the operation;
- tool schema/output passes validation;
- security/provenance requirements are satisfied.

Important security posture:

- page-provided tool names/descriptions are untrusted content;
- tool outputs may contain prompt injection;
- a WebMCP tool cannot authorize itself;
- authenticated browser context does not imply permission to perform every available action;
- purchase/publish/credential/consequential actions still require Kernux policy.

Fallback order remains WebMCP -> deterministic DOM/CDP -> semantic AgentQL/TinyFish -> visual computer use.

## 5. Kernux Runtime Protocol (KRP)

KRP is the minimal Kernux-owned protocol required because no external standard currently captures all of Kernux's privileged runtime invariants.

KRP connects orchestrator/clients with `kernuxd` and enrolled Kernux runtimes.

The provider-neutral runtime-operation semantics for negotiation, contact/execution truth, RequestId/OperationId, deduplication, cancellation, structured errors, retry guidance, and reconnect reconciliation are frozen in [`RUNTIME_OPERATION_MODEL.md`](RUNTIME_OPERATION_MODEL.md). The generated wire representation remains owned by later P01 schema work.

### Required properties

- explicit protocol and capability versions;
- authenticated peer identity;
- request id + operation id distinction;
- idempotency metadata for side effects;
- typed resource identifiers;
- streaming/event support;
- resumable observation after reconnect;
- bounded cancellation semantics;
- runtime-owned process status;
- clear `live | exited | unverifiable` execution vocabulary;
- structured errors;
- forward-compatible optional fields;
- capability negotiation rather than runtime-type inference.

### Core namespaces

```text
runtime.*
policy.*
grant.*
files.*
process.*
pty.*
git.*
browser.*
computer.*
artifact.*
event.*
secret.*
sandbox.*
identity.*
```

KRP uses `protocol/schema/krp.v1.schema.json` (JSON Schema Draft 2020-12) as its single v1 generated-contract source. Rust and TypeScript outputs are deterministic derivatives; protocol conformance tests are mandatory before remote use.

KRP v1 compatibility is frozen in [`KRP_COMPATIBILITY.md`](KRP_COMPATIBILITY.md):

- `krp/1` is the current semantic compatibility family;
- schema SHA-256 is provenance/drift identity, not a negotiated wire version;
- core objects are closed and unknown fields fail closed;
- the current generated capability version token is exact `1`;
- semantics-neutral additive optional fields require explicit feature negotiation and sender suppression for unsupported peers;
- protocol mismatch never rewrites contact/execution or side-effect truth;
- structurally valid but unnegotiated Event types are non-projecting.

## 6. Agent Skills

Agent Skills are packaging/instruction assets, not an authorization system.

Kernux should:

- discover/install portable skills;
- show source/version/content before trust elevation;
- treat skill text as instructions with provenance, not code-level permission;
- optionally expose Kernux capabilities through skill guidance;
- support repository/project-local skills without forcing cloud distribution.

## 7. CLI and PTY compatibility

If software runs in a terminal, Kernux should be able to supervise it even without a formal protocol.

PTY adapters require:

- captured raw transcript when parsing terminal UI state;
- provider/version-specific readiness parsers kept separate;
- robust interrupt/exit semantics;
- resumability honestly declared;
- no claim that terminal text is a stable API when it is not.

Structured ACP/API adapters are preferred when available.

## 8. Browser protocols

Kernux's browser layer can use:

- Chrome DevTools Protocol for low-level browser control/inspection;
- Playwright-compatible deterministic workflows;
- WebMCP for page-exposed tools;
- semantic extraction/targeting providers;
- vision/computer input fallback.

Browser provider choice is internal. The task asks for capabilities such as `browser.navigate`, `browser.observe`, `browser.act`, and `browser.extract`.

## 9. Observability protocols

Use OpenTelemetry-compatible trace/span concepts where possible:

- task/run/work-unit correlation;
- runtime/provider spans;
- tool/capability calls;
- approvals and waits;
- verification;
- cost/usage metadata.

Prompt/content payload export must be opt-in because traces can otherwise become a data-exfiltration path.

## 10. Version policy

The rules in this section govern external ecosystem adapters. KRP's owned compatibility contract is defined separately in [`KRP_COMPATIBILITY.md`](KRP_COMPATIBILITY.md).

Each external protocol adapter records:

- protocol name;
- negotiated version;
- supported extensions/capabilities;
- adapter version;
- compatibility status.

Rules:

1. Do not infer protocol behavior from application version alone.
2. Do not silently upgrade breaking protocol behavior.
3. Experimental features require explicit opt-in or bounded compatibility mode.
4. New side-effect semantics require negotiation.
5. Conformance fixtures must include older supported versions.
6. Protocol deprecations become tracked migration tasks before upstream removal dates.

## 11. Protocol boundary tests

Every adapter must cover:

- happy path;
- missing capability;
- version mismatch;
- unknown optional fields;
- malformed/untrusted payload;
- timeout/disconnect;
- cancellation;
- duplicate/replayed request;
- auth expiration/rotation;
- side-effect ambiguity;
- recovery after restart when the protocol permits it.

## 12. Strategic rule

**Standards connect Kernux to ecosystems; Kernux contracts preserve product invariants.**

Adopt open standards early where they remove bespoke integration work, but never outsource permissions, evidence, runtime truth, or task semantics to a protocol that does not guarantee them.