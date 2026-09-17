# Native Agent Runtime and Headless Interfaces

## 1. Purpose

Kernux must work for more than users who already have a coding-agent CLI installed. External coding agents remain first-class, but the product also needs a Kernux-native agent runtime that can coordinate models, capabilities, tools, browser/computer actions, verification, and humans under the same kernel policy.

This contract also makes Kernux usable without the desktop renderer through a stable CLI/headless API.

## 2. Separate three concepts

Do not collapse these into one provider abstraction:

1. **Model Provider** — inference endpoint/model with messages, multimodal inputs, structured outputs/tool calls, usage and limits.
2. **Agent Adapter** — an independent agent implementation such as an ACP agent, CLI agent, A2A agent, or hosted agent service.
3. **Kernux Native Agent** — Kernux-owned orchestration loop that uses one or more Model Providers and Kernux capabilities directly.

This separation lets Kernux serve developers, researchers, analysts, operators, and general knowledge workers without pretending every model is a coding agent.

## 3. ModelProvider contract

A model adapter advertises actual capabilities rather than a lowest-common-denominator fiction.

Representative metadata:

- provider/model identity and revision where available;
- context/input/output limits;
- text/image/audio/document support;
- structured output/tool-call support;
- streaming;
- prompt caching if exposed;
- reasoning-summary availability if explicitly provided;
- usage/cost telemetry availability;
- rate-limit metadata;
- local vs external data boundary;
- cancellation semantics;
- supported auth mode.

Unsupported features remain unsupported.

## 4. First provider classes

The architecture should support, without requiring all for alpha:

- provider API models;
- local OpenAI-compatible or equivalent endpoints;
- local model servers;
- organization-managed model gateways;
- future specialized models.

Provider credentials remain owned by the secret broker or provider-native auth flow. Model adapters do not receive unrestricted secret plaintext.

## 5. Kernux Native Agent

The native agent is an orchestrator over typed capabilities, not a hidden root shell.

Core loop:

```text
Task/WorkUnit
    |
    v
ContextBundle
    |
    v
Model turn
    |
    +--> proposed capability/tool calls
    |           |
    |           v
    |      policy / approval
    |           |
    |           v
    |       execution
    |           |
    +<----- observations
    |
    +--> verifier / acceptance checks
    |
    v
Result + evidence
```

The model can propose; `kernuxd` authorizes privileged execution.

## 6. Agent state

Persist observable state required to resume or inspect work:

- task/work-unit revision;
- agent/session id;
- provider/model/adapter version;
- ContextBundle ids/digests;
- user/agent messages appropriate for persistence;
- capability requests and results;
- approvals;
- artifacts;
- verification/evidence;
- compacted summaries with lineage;
- cancellation/restart state.

Do not make private chain-of-thought persistence a product dependency. Store user-visible reasoning summaries only when a provider exposes them and policy allows it.

## 7. Tool routing

The native agent sees Kernux tool/capability schemas produced from the Capability Kernel and approved integrations.

Routing rules:

- capability availability is run/runtime scoped;
- tools unavailable under policy are omitted or marked unavailable;
- tool descriptions do not grant permission;
- schema/result size is bounded;
- untrusted output carries trust/provenance metadata;
- large outputs become artifacts with summaries/references rather than giant prompt payloads.

## 8. Planning behavior

Kernux may ask a model to propose a task plan, but the canonical plan is a Kernux Task/WorkUnit structure with explicit dependencies, evidence, capabilities, and runtime constraints.

The model-generated plan is input to a deterministic planner/validator, not authority by itself.

High-consequence or large tasks may require user approval of the execution proposal before work begins.

## 9. Native agent verification

A model saying `done` does not end a task.

The task engine checks declared acceptance/evidence. Verification may be:

- deterministic commands;
- artifact validation;
- browser observation;
- protocol conformance;
- human review;
- separately configured verifier/agent analysis.

Model-assisted evaluation is labeled analysis, never substituted for a deterministic check that exists.

## 10. Multi-model strategy

The native agent runtime can support:

- one-model execution;
- planner/executor split;
- verifier model;
- fleet fan-out;
- synthesis;
- model escalation when a cheaper/smaller model cannot complete a bounded unit.

These strategies remain policy/budget controlled. More agents/models are not automatically better.

## 11. Headless architecture

The desktop app is one client of Kernux, not the only client.

`kernuxd` plus orchestration services must support a headless mode suitable for:

- terminal users;
- CI;
- remote machines;
- automated tests;
- servers/workstations without a visible desktop;
- mobile/remote control through an authenticated relay later.

Headless operation uses the same policy, task, event, evidence, and capability contracts as desktop operation.

## 12. Kernux CLI

The first-class CLI should eventually expose at least:

```text
kernux doctor
kernux project open|list|inspect
kernux task create|plan|run|status|follow-up|cancel
kernux run list|inspect|tail|stop
kernux agent list|inspect
kernux runtime list|inspect|enroll|revoke
kernux tool list|inspect
kernux permission inspect|approve|deny
kernux artifact list|show|export
kernux evidence verify|export
kernux replay inspect|fork
kernux config get|set
```

Command names may evolve, but the product contract is stable: every important workflow must be scriptable without simulating UI clicks.

## 13. Machine-readable CLI

Automation requires deterministic output modes.

Requirements:

- human-readable default;
- `--json` for finite structured results;
- NDJSON/event streaming mode for long-running status/output where appropriate;
- stable documented exit-code classes;
- no ANSI/color in machine mode;
- bounded output and artifact references for large payloads;
- explicit version/schema field in machine output;
- stderr/stdout separation that remains automation friendly.

## 14. Non-interactive approvals

There must be no unsafe universal `--yes` that silently converts approval-requiring work into unlimited autonomy.

Headless execution can proceed without a human prompt only when:

- policy already grants the exact capability scope;
- an organization policy grants it;
- the task runs in an intentionally preauthorized bounded sandbox/runtime;
- or a separately authenticated approver resolves the request.

Otherwise the run enters `waiting_for_approval` and can be resumed later.

## 15. Local API

A narrow local API may support desktop/CLI/SDK clients through the same authenticated IPC boundary.

The local API must expose product contracts rather than internal database tables. Candidate domains:

- projects;
- tasks/runs;
- agents;
- runtimes;
- capabilities/approvals;
- artifacts/evidence;
- events/subscriptions;
- search/context.

Remote APIs require explicit enrollment/auth and must not be enabled by merely binding the local daemon to a network interface.

## 16. SDK strategy

Do not publish a broad stable SDK before internal contracts prove durable.

When released, the SDK should prefer generated/versioned contracts and high-level task/capability interfaces. Internal storage/layout APIs remain private.

## 17. Design Mode

Kernux should preserve and extend the high-value interaction lesson from Orca's Design Mode.

For a browser-rendered interface, the user can select an element and create a structured context capture containing, where available:

- origin/URL;
- element DOM/accessibility identity;
- relevant HTML/attributes;
- computed-style subset;
- layout box;
- cropped screenshot;
- surrounding semantic context;
- page/runtime revision metadata;
- user annotation.

That capture becomes a typed artifact/context item that can be attached to an agent task.

For native desktop applications, an analogous mode can use accessibility/UI-automation trees plus screenshot regions when platform APIs permit it.

Design Mode is observation/context capture. It does not bypass capability policy for subsequent actions.

## 18. Implementation mapping

This contract refines existing phases:

- **P01** — model/agent/session capability schemas and CLI/API error/result envelopes;
- **P02** — authenticated local API and headless daemon authority;
- **P03** — CLI foundation, agent/context inspector, Design Mode UI entry points;
- **P05** — ModelProvider contract, native agent loop, external agent adapters;
- **P06** — browser/native Design Mode capture implementation;
- **P07** — planner/scheduler/fleet strategies used by native agent;
- **P09** — resumable agent state and evidence;
- **P10** — headless/remote runtime clients;
- **P11** — published SDK and external protocol/tool exposure only after contract maturity;
- **P12** — non-interactive/background execution under existing policy.

SpecGrain refinement must create explicit child units for these requirements before the relevant phase gates are closed.

## 19. Alpha acceptance

Before the first external alpha, Kernux should prove:

1. at least one Kernux-native model-backed task that uses typed capabilities end-to-end;
2. at least two independent external agent integrations remain usable alongside the native agent;
3. the same bounded task can be launched from desktop and CLI against the same task contract;
4. CLI machine mode is deterministic and bounded;
5. approval-required work pauses safely in headless mode;
6. a Design Mode capture can be attached to a task and traced to its source;
7. provider/model limitations are visible rather than normalized away.

## 20. Rule

**Kernux owns orchestration and policy; models provide intelligence; external agents remain interchangeable collaborators.**
