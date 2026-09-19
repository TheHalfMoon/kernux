# Kernux Task Registry

## How to use this file

This is the canonical **macro-task registry**. It is intentionally more detailed than the master-plan phase list, but many entries will still need recursive SpecGrain refinement before implementation.

States:

- `PLANNED` — known work, not yet dependency-eligible/proven.
- `NEXT` — next macro-task after the canonical plan merges.
- `READY` — dependencies proven and bounded execution package ready.
- `IN_PROGRESS` — active execution.
- `IMPLEMENTED_UNPROVEN` — code exists but required evidence is incomplete.
- `PROVEN` — exact required evidence established.
- `BLOCKED_EXTERNAL` — only an external dependency blocks proof.
- `DEFERRED` — intentionally outside current frontier.

Risk is the minimum Diffcipline-style profile; refinement may raise it.

---

## P00 — Repository and governance foundation

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P00-S01-T01 | PROVEN | R1 | Establish pnpm + Cargo monorepo skeleton, canonical folders, root commands, formatting/editor baseline. | plan merge |
| KX-P00-S01-T02 | PROVEN | R1 | Add baseline CI for Rust/TS format, lint, static checks, unit placeholders and cache discipline. | T01 |
| KX-P00-S02-T01 | PROVEN | R1 | Initialize repository-local SpecGrain and document deterministic authoring/refinement workflow. | T01 |
| KX-P00-S03-T01 | PROVEN | R1 | Initialize Diffcipline policy with R0-R3 verification profiles and CI gate. | T02 |
| KX-P00-S04-T01 | PROVEN | R2 | Define machine-readable donor provenance/import manifest schema and validation command. | T01 |
| KX-P00-S04-T02 | PROVEN | R1 | Add third-party notices/license inventory structure and donor import checklist. | provenance schema |
| KX-P00-S01-T03 | PROVEN | R0 | Add CONTRIBUTING, SECURITY, SUPPORT, CODE_OF_CONDUCT/issue/PR templates and repository governance docs. | T01 |
| KX-P00-S01-T04 | PROVEN | R1 | Decide/apply project license and validate compatibility strategy for planned donor imports/dependencies. | notices structure |

**P00 gate: PROVEN.** Fresh clone reproduces checks; SpecGrain/Diffcipline are usable; protected `main` requires JavaScript / TypeScript, Rust, SpecGrain, Diffcipline R1, and Provenance; provenance is enforceable before donor imports.

---

## P01 — Core contracts and protocol spine

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P01-S01-T01 | PROVEN | R2 | Define canonical IDs/revisions for Project, Task, WorkUnit, Run, AgentSession, Runtime, Artifact, Evidence, Event. | P00 |
| KX-P01-S02-T01 | PROVEN | R3 | Define v1 capability taxonomy, resource URI model, consequence classes, constraints, Grant semantics. | identities |
| KX-P01-S03-T01 | PROVEN | R3 | Define runtime capability negotiation, contact/execution state, operation IDs, cancellation and structured errors. | identities + capabilities |
| KX-P01-S04-T01 | PROVEN | R2 | Define event envelope, lineage, artifact references, evidence binding and redaction metadata. | identities |
| KX-P01-S05-T01 | PROVEN | R2 | Choose one schema source and generate Rust + TS contracts with round-trip/conformance fixtures. | prior P01 contracts |
| KX-P01-S05-T02 | NEXT | R2 | Define compatibility/versioning rules and unknown/additive-field behavior. | generated contracts |
| KX-P01-S05-T03 | PLANNED | R2 | Build canonical protocol fixture corpus including disconnect, duplicate operation, malformed payload and version mismatch. | generated contracts |

**P01 gate:** cross-language contracts executable; provider names absent from core; disconnect cannot imply exit; grants structurally bounded.

---

## P02 — Privileged kernel (`kernuxd`)

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P02-S01-T01 | PLANNED | R3 | Create `kernuxd` lifecycle, private UDS/named-pipe transport, health/version and graceful shutdown. | P01 |
| KX-P02-S01-T02 | PLANNED | R3 | Bind local client sessions with launch nonce/peer identity and reject unauthorized local callers. | daemon transport |
| KX-P02-S02-T01 | PLANNED | R3 | Implement installation/device identity and key creation/rotation/recovery abstraction. | daemon transport |
| KX-P02-S03-T01 | PLANNED | R3 | Implement SQLite metadata store, schema versioning, WAL/migration and corruption/recovery fixtures. | P01 events/IDs |
| KX-P02-S04-T01 | PLANNED | R2 | Implement SHA-256 artifact CAS with bounded streaming, digest verification and retention metadata. | metadata store |
| KX-P02-S05-T01 | PLANNED | R3 | Implement policy evaluator and bounded grants/expiry/delegation/deny reasons. | capabilities + store |
| KX-P02-S05-T02 | PLANNED | R3 | Implement user-friendly Safe/Standard/Developer/Autonomous/Custom profiles compiling to granular grants. | policy engine |
| KX-P02-S06-T01 | PLANNED | R3 | Implement secret-provider abstraction and first OS credential-store adapters with handle-based use. | identity + policy |
| KX-P02-S06-T02 | PLANNED | R3 | Add redaction-safe security audit events and secret-leak tests. | secret broker + events |

**P02 gate:** privilege bypass/adversarial tests pass; secrets stay outside ordinary DB/logs; restart/migration recovery proven.

---

## P03 — Desktop workspace foundation

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P03-S01-T01 | PLANNED | R2 | Create Electron/React desktop shell with unprivileged renderer and typed preload bridge. | P02 IPC |
| KX-P03-S01-T02 | PLANNED | R2 | Implement daemon connection/reconnect projection and honest degraded states. | desktop shell |
| KX-P03-S02-T01 | PLANNED | R1 | Establish Kernux design tokens, primitives, accessibility and design-system lint rules. | desktop shell |
| KX-P03-S03-T01 | PLANNED | R1 | Implement tabs/splits/docking/layout persistence and keyboard navigation. | design system |
| KX-P03-S04-T01 | PLANNED | R1 | Implement project rail, command bar, onboarding/empty states and run strip. | pane framework |
| KX-P03-S05-T01 | PLANNED | R1 | Implement basic timeline projection and typed artifact preview shell. | event store + desktop |
| KX-P03-S05-T02 | PLANNED | R2 | Add hidden/headless desktop UI qualification that never steals user focus. | workspace |

**P03 gate:** renderer has no ambient host authority; layout/project state survives restart; accessibility and hidden E2E baseline pass.

---

## P04 — Local computer runtime

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P04-S01-T01 | PLANNED | R3 | Implement granted-root filesystem read/list/metadata primitives with canonical path resolution. | P02 policy |
| KX-P04-S01-T02 | PLANNED | R3 | Implement write/move/create/search with symlink/junction/reparse defenses and atomic-write patterns. | filesystem read |
| KX-P04-S02-T01 | PLANNED | R3 | Implement argv-first process start, cwd/env policy, lifecycle, bounded streaming and cancellation. | P02 policy |
| KX-P04-S03-T01 | PLANNED | R3 | Implement cross-platform PTY session lifecycle, streaming, writes, stable session IDs and scrollback artifacts. | process layer |
| KX-P04-S04-T01 | PLANNED | R2 | Implement process enumerate/inspect/terminate adapters on macOS/Windows/Linux without unsafe shell shortcuts. | process layer |
| KX-P04-S05-T01 | PLANNED | R2 | Import/adapt bounded Desktop Commander document/data primitives behind Kernux contracts with provenance. | provenance + files |
| KX-P04-S06-T01 | PLANNED | R3 | Define/implement structured host app/window/clipboard/notification/accessibility capability subset. | policy + platform adapters |
| KX-P04-S06-T02 | PLANNED | R3 | Build host-boundary adversarial suite: path escape, argv injection, destructive action, output exhaustion. | local runtime |

**P04 gate:** project-scoped host work proven on all desktop OS families; long-running sessions survive renderer restart; donor import characterized.

---

## P05 — Agent integration foundation

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P05-S01-T01 | PLANNED | R2 | Implement stable ACP v1 client adapter and capability/session mapping. | P01 + P03 |
| KX-P05-S02-T01 | PLANNED | R2 | Implement generic supervised CLI/PTTY agent adapter with raw transcript capture. | P04 PTY |
| KX-P05-S03-T01 | PLANNED | R2 | Qualify first external coding agent end-to-end through ACP or structured adapter. | ACP adapter |
| KX-P05-S03-T02 | PLANNED | R2 | Qualify second independent agent and generic CLI fallback. | ACP + PTY |
| KX-P05-S04-T01 | PLANNED | R2 | Implement provider-native auth/account/usage metadata boundary without credential capture. | agent adapters |
| KX-P05-S05-T01 | PLANNED | R1 | Implement Agent pane with structured status/tool/file/permission events and interrupt/resume controls. | desktop + adapters |
| KX-P05-S05-T02 | PLANNED | R2 | Add agent capability/version matrix and unsupported-feature tests. | two agents |

**P05 gate:** two agents can perform the same bounded task; provider differences are negotiated honestly; agent use of Kernux tools remains policy-bound.

---

## P06 — Browser and web runtime

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P06-S01-T01 | PLANNED | R3 | Implement isolated Chromium context/profile lifecycle, persistence policy, download/upload roots. | P02 + desktop |
| KX-P06-S02-T01 | PLANNED | R2 | Implement deterministic CDP/Playwright navigate/observe/act/wait/extract/trace contract. | browser contexts |
| KX-P06-S03-T01 | PLANNED | R3 | Implement negotiated WebMCP discovery/invocation with schema/provenance/policy validation. | browser contract |
| KX-P06-S04-T01 | PLANNED | R2 | Import/adapt AgentQL/TinyFish semantic targeting/extraction behind Kernux browser provider API. | provenance + deterministic browser |
| KX-P06-S05-T01 | PLANNED | R3 | Implement screenshot/vision + input fallback and human takeover state. | browser/computer abstractions |
| KX-P06-S06-T01 | PLANNED | R2 | Capture browser evidence: origin, action method, observation, screenshot/artifact and lineage. | all browser paths |
| KX-P06-S06-T02 | PLANNED | R3 | Build malicious page/WebMCP/tool-output injection and cross-project auth leakage suite. | browser runtime |
| KX-P06-S06-T03 | PLANNED | R2 | Prove research/extraction golden journey with inspectable source provenance. | browser evidence |

**P06 gate:** structured-to-visual hierarchy works; auth contexts isolated; page content cannot self-authorize elevated action.

---

## P07 — Task engine and agent fleet

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P07-S01-T01 | PLANNED | R2 | Implement Task/WorkUnit plan revisions, dependencies, acceptance/evidence/capability/runtime constraints. | P01/P02 |
| KX-P07-S02-T01 | PLANNED | R3 | Implement scheduler, dependency eligibility, cancellation and safe retry/idempotency enforcement. | task model + runtime |
| KX-P07-S03-T01 | PLANNED | R2 | Implement Fleet fan-out with isolated candidates and bounded concurrency. | scheduler + agents |
| KX-P07-S04-T01 | PLANNED | R2 | Implement verification/evidence comparison matrix with cost/latency metadata. | fleet + evidence |
| KX-P07-S04-T02 | PLANNED | R1 | Implement Fleet/Compare UX separating measured evidence from model-assisted analysis. | compare data |
| KX-P07-S05-T01 | PLANNED | R2 | Implement synthesis as new candidate lineage with mandatory fresh verification. | compare |
| KX-P07-S06-T01 | PLANNED | R2 | Implement provider/runtime budget and concurrency control with graceful unknown-usage behavior. | scheduler |

**P07 gate:** multi-agent work cannot corrupt shared state; retries do not duplicate side effects; synthesis is separately proven.

---

## P08 — Git, workspaces, and review

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P08-S01-T01 | PLANNED | R2 | Implement Project adapter for ordinary folders and git repositories. | P04 files |
| KX-P08-S02-T01 | PLANNED | R2 | Import/adapt worktree create/list/switch/remove/recovery primitives with provenance. | git project + provenance |
| KX-P08-S03-T01 | PLANNED | R2 | Implement runtime-scoped Git capability detection/fallback across native/WSL/SSH. | runtime contract |
| KX-P08-S03-T02 | PLANNED | R2 | Implement status/diff/stage/commit/branch/history with safe process invocation. | Git capability layer |
| KX-P08-S04-T01 | PLANNED | R1 | Implement diff/review UI, inline feedback-to-agent, staged state and candidate compare. | desktop + git |
| KX-P08-S04-T02 | PLANNED | R2 | Bind verification badges/results to exact candidate head. | quality/evidence + review |
| KX-P08-S05-T01 | PLANNED | R2 | Implement GitHub issue/PR/review adapter behind provider-neutral review contracts. | git review |

**P08 gate:** local code journey ends in proven diff + commit/PR; worktree candidates isolated; Git version/host fallbacks proven.

---

## P09 — Evidence, replay, and recovery

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P09-S01-T01 | PLANNED | R2 | Make task/run UI projections rebuild deterministically from durable events + indexed projections. | event model + product flows |
| KX-P09-S02-T01 | PLANNED | R2 | Define and implement supported checkpoint records across task/workspace/browser/runtime state. | projections |
| KX-P09-S03-T01 | PLANNED | R3 | Implement replay-semantics registry and enforcement for deterministic/idempotent/compensatable/non-replayable operations. | capability operations |
| KX-P09-S04-T01 | PLANNED | R3 | Implement fork-from-checkpoint with new lineage/operation identities. | checkpoints + replay semantics |
| KX-P09-S05-T01 | PLANNED | R2 | Implement independently verifiable evidence-bundle export with digests and redaction manifest. | evidence/artifact store |
| KX-P09-S06-T01 | PLANNED | R3 | Prove crash/restart/interruption recovery for daemon, renderer, browser and partial durable writes. | all P09 |
| KX-P09-S06-T02 | PLANNED | R3 | Prove duplicate-side-effect prevention/ambiguity surfacing after reconnect/retry. | replay + scheduler |

**P09 gate:** accepted local golden journeys recover honestly; evidence validates independently; replay never blindly repeats consequential actions.

---

## P10 — Runtime fabric

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P10-S01-T01 | PLANNED | R3 | Implement container runtime lifecycle, mounts, network/resource policy and artifact transfer. | P04 + KRP |
| KX-P10-S02-T01 | PLANNED | R3 | Define VM/microVM provider contract and qualify one stronger-isolation backend if feasible for Gen 1. | runtime fabric |
| KX-P10-S03-T01 | PLANNED | R3 | Implement WSL runtime identity/path/argv/env/files/process/PTY/Git handling. | KRP + local runtime |
| KX-P10-S04-T01 | PLANNED | R3 | Implement SSH runtime enrollment/probing/process/PTY/files/git/reconnect semantics. | KRP + identity |
| KX-P10-S05-T01 | PLANNED | R3 | Implement remote-device mutual identity, encrypted transport and revocation. | identity + KRP |
| KX-P10-S05-T02 | PLANNED | R3 | Implement resumable remote event streams and operation idempotency/duplicate-call handling. | remote transport |
| KX-P10-S05-T03 | PLANNED | R3 | Qualify mixed-version compatibility and controller/host policy intersection. | remote protocol |
| KX-P10-S06-T01 | PLANNED | R1 | Implement Machines/runtime chooser UX with trust/capability/contact visibility. | multiple runtime providers |

**P10 gate:** same bounded workload proven local + isolated + remote; disconnect remains unverifiable; replay attacks/duplicates fail safely.

---

## P11 — Tool, MCP, skills, and integration ecosystem

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P11-S01-T01 | PLANNED | R3 | Implement MCP 2026-07-28 client core, discovery/cache, auth and extension negotiation. | protocol spine + secrets |
| KX-P11-S01-T02 | PLANNED | R2 | Implement MCP Tasks mapping and compatibility fixtures for intentionally supported older MCP generation(s). | MCP core |
| KX-P11-S02-T01 | PLANNED | R3 | Implement policy-mediated Kernux MCP server surface for explicitly exposed capabilities. | MCP client/policy |
| KX-P11-S03-T01 | PLANNED | R2 | Implement Agent Skills discovery/install/project scope/provenance with no implicit privilege. | project + provenance |
| KX-P11-S04-T01 | PLANNED | R2 | Implement generic OpenAPI/HTTP and CLI tool adapters with output bounds and secret broker support. | tool runtime |
| KX-P11-S05-T01 | PLANNED | R3 | Define plugin manifest, capability/network/secret declarations and update permission-delta UX. | capabilities + provenance |
| KX-P11-S06-T01 | PLANNED | R2 | Publish internal extension SDK only for contracts proven stable by prior phases. | stable contracts |

**P11 gate:** external tools cannot self-expand permissions; MCP conformance and one real integration journey proven.

---

## P12 — Automation and long-running work

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P12-S01-T01 | PLANNED | R3 | Implement local scheduler with durable intent, missed-run policy and machine availability semantics. | P09 recovery |
| KX-P12-S02-T01 | PLANNED | R3 | Implement scoped event/webhook triggers through authorized connectors. | scheduler + integrations |
| KX-P12-S03-T01 | PLANNED | R3 | Implement background supervision, wake/restart/approval wait/budget notifications. | scheduler + runtimes |
| KX-P12-S04-T01 | PLANNED | R2 | Implement reusable task templates that compile to ordinary Task/WorkUnit contracts. | task engine |
| KX-P12-S04-T02 | PLANNED | R3 | Prove no duplicate scheduled side effect after crash/restart/clock or connectivity edge cases. | automation stack |

---

## P13 — Mobile companion

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P13-S01-T01 | PLANNED | R3 | Implement mobile pairing/auth/revocation against Kernux identity/relay model. | P10 remote |
| KX-P13-S02-T01 | PLANNED | R1 | Implement active tasks/status/results/notification experience. | mobile auth + task APIs |
| KX-P13-S03-T01 | PLANNED | R3 | Implement complete approval cards with action/target/data/origin/duration/risk context. | policy APIs |
| KX-P13-S04-T01 | PLANNED | R2 | Implement follow-up, pause/resume/stop and simple task launch. | task remote control |
| KX-P13-S05-T01 | PLANNED | R2 | Implement approved screenshot/browser/computer observation stream and artifact view. | runtime streams |
| KX-P13-S05-T02 | PLANNED | R3 | Prove stale/lost mobile clients cannot replay expired grants or duplicate commands. | mobile complete |

---

## P14 — Teams and optional Kernux cloud

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P14-S01-T01 | PLANNED | R3 | Define organization/user/team/project identity and membership model. | stable local identity |
| KX-P14-S02-T01 | PLANNED | R3 | Implement org policy intersection that can tighten but not bypass kernel/runtime hard constraints. | org identity + policy |
| KX-P14-S03-T01 | PLANNED | R3 | Implement shared runtime/skill/secret-reference ownership and revocation metadata. | teams + runtime/integrations |
| KX-P14-S04-T01 | PLANNED | R2 | Implement shared tasks, human/agent assignments, comments/notes and audit projection. | teams + task engine |
| KX-P14-S05-T01 | PLANNED | R3 | Implement authenticated encrypted relay/sync for NAT/mobile/remote scenarios with local-first fallback. | remote fabric |
| KX-P14-S06-T01 | DEFERRED | R3 | Design signed/provenanced marketplace/registry only after plugin ecosystem and governance prove mature. | P11 + team policy |

---

## P15 — Hardening, release, and launch

| ID | State | Risk | Macro outcome | Depends |
| --- | --- | --- | --- | --- |
| KX-P15-S01-T01 | PLANNED | R3 | Execute supported macOS/Windows/Linux qualification matrix plus applicable WSL/SSH journeys. | release feature set |
| KX-P15-S02-T01 | PLANNED | R3 | Close threat-model/adversarial corpus and perform independent/manual R3 security review. | feature complete |
| KX-P15-S03-T01 | PLANNED | R2 | Establish/ratchet startup, terminal, workspace, event, browser and multi-agent performance budgets from measurement. | feature complete |
| KX-P15-S03-T02 | PLANNED | R2 | Stress multi-agent, multi-terminal, browser and large-repository bounds; fix unbounded scans/output. | performance baseline |
| KX-P15-S04-T01 | PLANNED | R3 | Qualify installer, update, migration, rollback/recovery and uninstall on each supported desktop OS. | packaging |
| KX-P15-S05-T01 | PLANNED | R3 | Generate/verify SBOM, checksums, signatures/notarization, provenance attestations and notices. | release pipeline |
| KX-P15-S06-T01 | PLANNED | R1 | Complete user/admin/security/developer/onboarding/troubleshooting/privacy documentation. | stable product |
| KX-P15-S07-T01 | PLANNED | R2 | Preregister and run comparative evaluation before any superiority claims; preserve all negative evidence. | stable benchmark build |
| KX-P15-S08-T01 | PLANNED | R3 | Qualify exact release commit independently from ordinary CI and publish evidence packet. | all release gates |

---

# Cross-cutting permanent task classes

These are not one-off phase tasks. Every applicable Grain must account for them.

## X01 — Cross-platform

Any filesystem/process/PTY/git/browser/native behavior must state which of macOS, Windows, Linux, WSL, SSH are applicable and provide proof accordingly.

## X02 — Security

Any new privilege, secret, browser, plugin, remote, updater, installer, or destructive operation updates threat coverage and carries R3 unless explicitly justified lower.

## X03 — Provenance

Any copied/adapted donor code updates machine-readable provenance and notices in the same change or a mechanically linked import change.

## X04 — Compatibility

Any remote/wire/persisted schema change declares version/migration behavior and tests supported mixed versions.

## X05 — Accessibility

Any user-visible UI change preserves keyboard/focus/semantic status and non-color-only communication.

## X06 — Evidence

Any task completion updates exact-head/run evidence; failures and blocked checks remain visible.

## X07 — Recovery

Any durable or side-effecting workflow defines crash/cancel/retry/restart behavior before it is called complete.

## X08 — Data boundary/privacy

Any integration that sends content off-device declares what data, to whom, for what purpose, with which retention/configuration assumptions.

---

# Initial implementation sequence

After the canonical planning PR merges, the intended immediate sequence is:

```text
KX-P00-S01-T01
  -> KX-P00-S01-T02
  -> KX-P00-S02-T01 + KX-P00-S03-T01
  -> KX-P00-S04-T01/T02
  -> P00 gate
  -> P01 contracts
```

Do **not** start by wholesale-copying Orca, TinyFish, or Desktop Commander. Large imports begin only when the receiving Kernux contract and provenance gate exist.

# Registry maintenance

When implementation discovery creates new work:

- add it under the earliest dependency-correct phase;
- explain why existing tasks do not cover it;
- avoid catch-all tasks such as `finish integration`;
- refine over-large tasks into child SpecGrain specs rather than inflating prompts;
- never mark later tasks PROVEN to make the plan look closer to completion.