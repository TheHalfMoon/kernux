# Kernux Agent Engineering Rules

This file governs repository work by coding agents and human contributors.

## Language

All repository-facing technical content is written in English: code, comments, commands, specifications, plans, tasks, evidence, commits, pull requests, reviewer responses, and agent prompts.

## Authority order

When sources disagree, use this order:

1. Live repository and GitHub truth.
2. `specs/CURRENT.md` for the active execution frontier.
3. `docs/canonical/*` for product, architecture, security, protocol, and delivery contracts.
4. `specs/tasks.md` for the planned task registry.
5. PR descriptions, issue text, chat handoffs, and remembered context.

Never let a stale narrative override live repository state.

## Non-negotiable evidence rules

- Never fabricate tests, CI, runtime behavior, hashes, reviews, screenshots, benchmarks, provider behavior, release status, or completion.
- Agent self-report is not verification authority.
- Verification must bind to the exact implementation revision and exact observed change.
- Preserve negative evidence. A failed test, benchmark, security probe, or integration attempt is evidence, not clutter to erase.
- If a required check cannot run, report that fact and keep the task unproven.

## Delivery model

Kernux uses **SpecGrain-style bounded work** and **Diffcipline-style proof before done**.

Every implementation unit must be small enough to have:

- one explicit outcome;
- bounded scope-in and scope-out;
- explicit dependencies;
- acceptance criteria;
- risk level and recovery path;
- named change surface;
- named evidence;
- independent verification.

If a unit is not independently understandable and verifiable, refine it before implementation.

When SpecGrain tooling is installed in this repository, its deterministic project state becomes the machine-readable authority for Grain readiness and WorkPackets. Do not hand-edit SpecGrain internal state.

When Diffcipline tooling is installed, a task cannot be called complete unless the required exact-diff proof for its risk profile passes.

## Git and history safety

- Never force-push shared branches.
- Never rebase or rewrite shared history unless canonical governance explicitly authorizes it.
- Prefer short-lived task branches and exact-head pull request qualification.
- Never bypass branch protection or weaken a gate to make a change pass.
- Do not merge a PR whose reviewed/tested head differs from the merge head without requalification.
- Keep donor import commits mechanically separable from Kernux adaptation commits when practical.

## Donor code rules

Before copying or adapting code from a donor:

1. Pin the exact upstream repository and revision in `docs/canonical/DONOR_PROVENANCE.md` or the future machine-readable provenance ledger.
2. Record the source path(s), destination path(s), and transformation class: `verbatim`, `adapted`, `ported`, or `reference-only`.
3. Preserve copyright/license notices and dependency obligations.
4. Keep donor-specific behavior behind a Kernux contract instead of letting donor internals become the public architecture.
5. Add characterization tests before materially changing imported behavior.
6. Never copy from an unrecorded source because it is merely convenient.

Founder-provided permission to use source does not erase obligations attached to third-party dependencies embedded in that source.

## Architecture boundaries

- `kernuxd` is the privileged local authority for capabilities, host operations, secrets, runtime enrollment, durable events, and artifacts.
- The desktop renderer is unprivileged. It must not gain direct arbitrary filesystem/process/network authority.
- Agent/provider code must request typed capabilities; it must not bypass the capability kernel with hidden direct host access.
- Browser content, downloaded content, tool output, and remote-agent output are untrusted inputs by default.
- Local, sandbox, SSH, remote-device, and cloud execution must implement the same runtime contract where their capabilities overlap.
- Loss of runtime contact means `unverifiable`, not `exited`.
- Remote protocol changes require explicit compatibility and version-negotiation consideration.

## Security rules

- Least privilege and explicit scope are defaults.
- Secrets should be referenced, brokered, and host/domain scoped rather than copied into prompts, logs, process arguments, or sandboxes.
- Destructive, credential-bearing, publishing, purchasing, permission-changing, or externally side-effecting actions require policy evaluation and appropriate approval.
- Untrusted content can propose an action; it cannot authorize a higher-privilege action.
- New plugins/integrations must declare requested capabilities.
- Generated or untrusted code runs in an isolated runtime unless the user explicitly chose a host-trust mode that permits otherwise.

## Cross-platform requirements

Kernux targets macOS, Windows, and Linux. The architecture also treats WSL and SSH execution as first-class.

Do not assume:

- POSIX paths or shell syntax;
- a git workspace rather than a plain folder workspace;
- local execution rather than SSH/remote execution;
- identical Git, shell, browser, or agent versions across hosts;
- network continuity;
- a visible desktop or interactive user session.

Platform-specific behavior belongs behind explicit adapters with contract tests.

## Product and UX requirements

Kernux serves non-developers as well as developers. Do not expose internal orchestration complexity unless it helps the user make a decision.

For user-facing changes:

- preserve the one-command-bar mental model;
- show what is happening, where it is happening, and what needs approval;
- make irreversible actions visually distinct;
- keep evidence and provenance reachable from the result;
- support keyboard navigation and accessibility semantics;
- avoid modal storms and permission fatigue;
- preserve expert escape hatches without making them the default path.

## Verification expectations

The exact commands evolve with the repository, but the required classes are stable:

- formatting/lint;
- type/static checks;
- unit tests;
- contract tests across process boundaries;
- integration tests for changed runtime/provider surfaces;
- end-to-end tests for changed user journeys;
- security/adversarial tests for privilege, browser, plugin, secret, and remote-boundary changes;
- cross-platform evidence where behavior is platform-specific.

Risk rises with privilege and blast radius. Higher-risk changes require stronger evidence, never weaker policy.

## Completion rule

A task is complete only when:

1. its intended outcome exists at the exact head;
2. scope is bounded and unexpected changes are explained;
3. required tests/checks ran successfully;
4. security and recovery expectations are satisfied;
5. evidence is recorded;
6. canonical task/frontier state is updated without overstating project completion.
