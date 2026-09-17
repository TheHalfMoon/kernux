# Product Thesis

## Category

Kernux defines an **Agent Operating Environment (AOE)**: a user-facing operating layer for coordinating AI agents with browsers, computers, tools, runtimes, and people.

It is intentionally not positioned as only:

- a coding IDE;
- an AI chat client;
- a browser automation product;
- a desktop-control MCP server;
- a model router;
- a workflow builder;
- a hosted agent control plane.

Kernux contains those capabilities where useful, but the durable product boundary is the task, capability, runtime, evidence, and collaboration model that unifies them.

## North star

**One prompt -> any agent -> any approved capability -> any enrolled runtime -> a verifiable result.**

The user should think in outcomes, not tool routing. Kernux decides how to coordinate available agents and capabilities while keeping execution visible and policy-bound.

## User promise

A useful Kernux installation should let a person say something like:

> Investigate why checkout fails, reproduce it in the browser, inspect the repository, let two agents propose fixes, test both, show me the differences, and prepare the change for review.

The user should not have to manually decide which window, agent CLI, browser, terminal, remote machine, MCP server, or script owns every intermediate step.

## Primary jobs to be done

### Developers

- delegate bounded code tasks to one or many agents;
- isolate parallel changes;
- run and inspect tests;
- browse a live application while agents work;
- review, annotate, compare, verify, commit, and publish changes;
- continue work across local, WSL, SSH, sandbox, and remote machines.

### Researchers and analysts

- search and browse live sources;
- collect structured evidence;
- run local or remote analysis tools;
- preserve source lineage, files, notebooks, figures, and reports;
- compare model/agent outputs without losing provenance.

### Founders and operators

- delegate cross-application work;
- automate repeatable web/computer workflows;
- review approvals and results rather than micromanage tools;
- schedule or trigger recurring tasks;
- inspect costs, failures, and audit history.

### Teams

- share projects, policies, machines, skills, and evidence;
- assign work to humans or agents;
- preserve organizational approval and secret boundaries;
- reproduce a run rather than sharing screenshots and chat fragments.

## Product pillars

### 1. Unified Workspace

One task-centric surface with dynamic panes for agents, browser, computer, terminal, files, source control, diff, artifacts, approvals, and timeline.

The workspace is not a fixed IDE layout. It opens the surfaces a task needs and lets users pin or rearrange them.

### 2. Capability Kernel

Agents do not receive ambient host power. They request typed capabilities such as:

```text
browser.navigate
browser.observe
browser.act
browser.extract
computer.files.read
computer.files.write
computer.process.start
computer.terminal.write
git.diff
git.commit
tool.invoke
secret.use
external.publish
```

Policy evaluates the subject, target, runtime, scope, provenance, trust level, requested side effect, and user/org rules before execution.

### 3. Agent Fleet

Kernux can run one agent or a coordinated fleet. Fleet behavior includes:

- fan-out of one bounded task to independent agents;
- role-based delegation;
- deterministic verification of candidates;
- side-by-side comparison;
- synthesis from proven components;
- interruption, follow-up, resume, and replacement;
- explicit cost, time, and evidence accounting.

A model judgment may inform comparison, but it never replaces repository/runtime evidence.

### 4. Web Runtime

Kernux treats the web as a first-class execution environment with an ordered strategy:

1. first-party structured tools such as WebMCP when available;
2. deterministic DOM/accessibility/CDP operations;
3. semantic/natural-language web primitives such as AgentQL;
4. visual computer-use fallback when structure is insufficient.

This order targets reliability, token efficiency, accessibility, and auditability.

### 5. Computer Runtime

The computer runtime includes files, directories, terminal/PTY, processes, documents/data, applications, clipboard, notifications, and native UI control. Structured OS/application APIs are preferred before coordinate-only automation.

### 6. Runtime Fabric

The same task can span:

- the local host;
- WSL;
- Docker/container sandboxes;
- VM/microVM providers;
- SSH hosts;
- remote enrolled devices;
- cloud browsers;
- GPU/compute runtimes.

Location is an execution property, not a separate product mode.

### 7. Evidence and Replay

A Kernux run preserves:

- task/plan revisions;
- agent sessions;
- tool calls;
- approvals;
- browser and computer observations;
- terminal/process state transitions;
- changed files and git revisions;
- artifacts and hashes;
- verification results;
- failures and retries.

Users can inspect the timeline, replay supported operations, fork from a checkpoint, or reproduce a verified result.

### 8. Open Capability Ecosystem

Kernux supports native adapters and open protocols rather than a single proprietary plugin system. The target integration surface includes MCP, A2A, Agent Skills, WebMCP, CLI tools, OpenAPI/HTTP, and language SDKs.

## Experience principles

### Outcome first

The default entry point is a command bar: **What do you want done?**

Power-user configuration exists, but the user should not need to assemble a workflow graph before doing ordinary work.

### Progressive disclosure

Everyday users see task progress, decisions, approvals, and results. Experts can drill into prompts, tool calls, logs, traces, environment details, and policy evaluation.

### Visible autonomy

Autonomy must be observable. Users should always be able to answer:

- What is Kernux doing?
- Which agent/runtime is doing it?
- What changed?
- What is blocked?
- What needs my approval?
- What evidence supports the result?

### Local trust first

The core product works locally without uploading project contents to a Kernux service. Remote/cloud providers are explicit capabilities with visible data boundaries.

### Recovery over illusion

Kernux must tolerate process crashes, app restarts, remote disconnects, stale agents, browser failures, and partial operations. Ambiguous state is surfaced and recovered explicitly; it is never silently guessed into success.

## Non-goals for the first product generation

- Building a proprietary foundation model.
- Replacing operating systems or hypervisors.
- Reimplementing every editor feature before Kernux can complete useful tasks.
- Building a no-code workflow designer as the primary interface.
- Requiring Kernux Cloud for local use.
- Claiming that unrestricted autonomous computer use is safe.
- Calling probabilistic model confidence proof.

## Product success dimensions

Metrics must be segmented by task class and runtime rather than collapsed into one vanity score.

Track at minimum:

- time to first successful task;
- verified task completion rate;
- human interventions per task and why they occurred;
- recovery success after interruption/restart;
- permission prompts per task and approval-fatigue rate;
- agent/runtime/tool failure rate;
- task latency and active compute time;
- model/tool/runtime cost;
- replay/reproduction success;
- weekly retained active users;
- percentage of users using more than one capability class;
- ecosystem adoption: skills, MCP servers, agents, runtimes, extensions.

No superiority claim should be made without a reproducible comparative evaluation.

## Founding wedge and expansion

Kernux should not launch as an unfocused everything-app. The first compelling wedge is **power users who already combine coding agents, terminals, browsers, and remote machines**. This group immediately benefits from the founding donors and can validate the capability kernel.

After that foundation is reliable, the same contracts expand naturally to research, analysis, operations, and broader knowledge work without forking the architecture into separate products.
