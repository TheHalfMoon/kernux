# Kernux

**Agents. Web. Computer. One runtime.**

Kernux is an open **Agent Operating Environment**: one workspace where humans and AI agents can plan, browse, use computers, run tools, work across local and remote runtimes, and deliver outcomes with explicit permissions and verifiable evidence.

> **Repository state:** canonical plan established on `main`. Product implementation has not started.

## The product in one sentence

**One prompt -> any agent -> any approved capability -> any enrolled runtime -> a verifiable result.**

Kernux is deliberately broader than a coding IDE, browser agent, desktop automation tool, or chat client. It combines those surfaces behind one capability kernel and one task/evidence model.

```text
                         KERNUX

 Human intent
     |
     v
 Task Graph -----> Agent Fleet -------------------+
     |              |                             |
     |              +-- Native / CLI / API / A2A agents
     |                                            |
     +------> Capability Kernel <-----------------+
                  |       |       |
                  |       |       +--> Tools / MCP / Skills
                  |       +----------> Browser / WebMCP / CDP
                  +------------------> Computer / Files / PTY
                                      |
                         Runtime Fabric
                 local | sandbox | SSH | cloud
                                      |
                    Context + Evidence
                         + Replay
```

## Founding design principles

- **Local-first, cloud-capable.** A useful core requires no Kernux cloud account.
- **Provider-neutral.** Agents, models, browsers, sandboxes, git hosts, and tools live behind versioned contracts.
- **Permission before power.** Privileged actions are authorized by a capability kernel, not by a model or UI convention.
- **Structured before visual.** Prefer typed tools, WebMCP, accessibility/DOM, and deterministic automation before vision/coordinates.
- **Execution-host truth.** The machine executing work owns process/session truth; network loss is not proof of process death.
- **Evidence before done.** Agent self-report is never completion authority.
- **Context with provenance.** Agents receive bounded, inspectable context bundles rather than ambient access to everything.
- **User-governed memory.** Durable memory is scoped, reviewable, exportable, and deletable.
- **Replayable work.** Tasks preserve state, events, artifacts, approvals, and evidence rather than only chat history.
- **Portable local data.** Kernux-owned history has explicit export, backup, retention, and deletion semantics.
- **Cross-platform by contract.** macOS, Windows, Linux, WSL, SSH, folders, and git worktrees are designed in rather than patched in later.
- **Open ecosystem.** MCP, A2A, Agent Skills, WebMCP, CLI, OpenAPI, and native adapters are first-class integration paths.
- **Provenance by default.** Donor code and external references remain traceable to exact source revisions and obligations.

## Canonical authority set

Start here:

- [`docs/canonical/PRODUCT_THESIS.md`](docs/canonical/PRODUCT_THESIS.md)
- [`docs/canonical/PRODUCT_STRATEGY_AND_DISTRIBUTION.md`](docs/canonical/PRODUCT_STRATEGY_AND_DISTRIBUTION.md)
- [`docs/canonical/ARCHITECTURE.md`](docs/canonical/ARCHITECTURE.md)
- [`docs/canonical/SECURITY_MODEL.md`](docs/canonical/SECURITY_MODEL.md)
- [`docs/canonical/UX_BLUEPRINT.md`](docs/canonical/UX_BLUEPRINT.md)
- [`docs/canonical/CONTEXT_MEMORY_AND_SEARCH.md`](docs/canonical/CONTEXT_MEMORY_AND_SEARCH.md)
- [`docs/canonical/NATIVE_AGENT_AND_HEADLESS.md`](docs/canonical/NATIVE_AGENT_AND_HEADLESS.md)
- [`docs/canonical/DATA_LIFECYCLE_AND_PORTABILITY.md`](docs/canonical/DATA_LIFECYCLE_AND_PORTABILITY.md)
- [`docs/canonical/DONOR_PROVENANCE.md`](docs/canonical/DONOR_PROVENANCE.md)
- [`docs/canonical/LICENSE_POLICY.md`](docs/canonical/LICENSE_POLICY.md)
- [`docs/canonical/PROTOCOL_STRATEGY.md`](docs/canonical/PROTOCOL_STRATEGY.md)
- [`docs/canonical/QUALITY_AND_EVIDENCE.md`](docs/canonical/QUALITY_AND_EVIDENCE.md)
- [`docs/canonical/ARCHITECTURE_DECISIONS.md`](docs/canonical/ARCHITECTURE_DECISIONS.md)
- [`docs/canonical/EXECUTION_MASTER_PLAN.md`](docs/canonical/EXECUTION_MASTER_PLAN.md)
- [`docs/canonical/PLANNING_COMPLETENESS_AUDIT.md`](docs/canonical/PLANNING_COMPLETENESS_AUDIT.md)
- [`docs/research/EXTERNAL_REFERENCES.md`](docs/research/EXTERNAL_REFERENCES.md)
- [`specs/CURRENT.md`](specs/CURRENT.md)
- [`specs/tasks.md`](specs/tasks.md)

## Founding donor sources

The founder has stated that Kernux has permission to use the full source of the three founding donor systems. Code import is still governed by exact-revision provenance and third-party notice preservation.

- `stablyai/orca` — agent workspace, parallel worktrees, terminal/runtime UX, remote/mobile patterns.
- `tinyfish-io/agentql` and TinyFish source — resilient web interaction and browser-agent primitives.
- `wonderwhy-er/DesktopCommanderMCP` — local filesystem, process, terminal, document/data, MCP, and remote-device primitives.

SpecGrain and Diffcipline are used as delivery/proof methodology rather than product donors:

- `TheHalfMoon/SpecGrain` — bounded work, explicit readiness, WorkPackets, independent evidence.
- `TheHalfMoon/Diffcipline` — proof-before-done checks against the exact change.

See [`DONOR_PROVENANCE.md`](docs/canonical/DONOR_PROVENANCE.md) for pinned revisions and import rules.

## Current frontier

The canonical plan is established on `main`; no product implementation is canonically complete yet. The first implementation frontier is **P00 — Repository and governance foundation**, followed by the capability kernel and its contracts. See [`specs/CURRENT.md`](specs/CURRENT.md).

The pre-implementation plan has also been challenged through [`PLANNING_COMPLETENESS_AUDIT.md`](docs/canonical/PLANNING_COMPLETENESS_AUDIT.md). That audit adds binding refinement requirements for native model-backed agents, context/search/memory, headless CLI/API operation, Design Mode, data portability/lifecycle, and privacy-safe diagnostics before the relevant phase gates may be called proven.

## License

Kernux-owned code is licensed under the Apache License 2.0. Third-party material retains its own applicable terms and notices. See [`LICENSE`](LICENSE) and [`docs/canonical/LICENSE_POLICY.md`](docs/canonical/LICENSE_POLICY.md).
