# UX Blueprint

## 1. Experience goal

Kernux should feel simpler than the systems it coordinates. The default mental model is not a graph editor, terminal multiplexer, browser automation console, or agent dashboard. It is:

> **Tell Kernux the outcome. Watch the work. Approve what matters. Inspect the proof.**

The product must serve expert developers without requiring every user to think like one.

## 2. Primary shell

The desktop shell has four persistent regions:

1. **Project rail** — projects, recent tasks, machines/runtimes, notifications.
2. **Command bar** — the primary entry point: `What do you want done?`.
3. **Adaptive workspace** — panes opened by task needs: Agent, Browser, Computer, Terminal, Files, Diff, Artifact, Data, Timeline.
4. **Run strip** — active task state, runtime, budget, approvals, blockers, verification.

The workspace uses dock/split/tab primitives, but the default layout is task-driven. Kernux opens the minimum surfaces needed for the current task and remembers user layout choices.

## 3. Core user journey

### Start

A user opens a project or creates a general workspace, then writes an outcome in the command bar.

Kernux responds with a compact execution proposal when planning is useful:

- intended result;
- major work units;
- selected agent/runtime strategy;
- requested elevated capabilities;
- estimated cost/time class when known;
- important uncertainty.

Low-risk obvious work can start immediately under existing policy.

### Execute

The task view shows live cards for work units and agents. Each card answers:

- status;
- owner/agent;
- runtime;
- current action;
- elapsed time;
- blocker/approval;
- evidence produced so far.

### Intervene

Users can:

- send follow-up instructions;
- pause/stop one agent or the whole task;
- take over browser/computer control;
- move a work unit to another runtime;
- add/remove an agent candidate;
- narrow scope;
- approve/deny a consequential action.

Intervention creates a timeline event and new plan/task revision where relevant.

### Review

The review surface prioritizes outcomes and evidence:

- what changed;
- candidate comparison;
- tests/checks;
- screenshots/artifacts;
- unresolved uncertainty;
- approvals performed;
- side effects performed;
- exact source/runtime revisions.

### Deliver

Delivery actions are explicit and typed: commit, open PR, export report, save artifact, send message, publish, deploy, etc. Consequential delivery never hides inside a generic `Done` button.

## 4. Workspace panes

### Agent pane

Shows conversation plus structured activity, not only transcript text.

- plan/reasoning summaries when provider exposes them;
- tool/capability calls;
- files/artifacts referenced;
- permissions requested;
- provider/model/session metadata;
- interrupt/resume/restart controls.

Multiple agents can be tiled or grouped into a Fleet view.

### Fleet / Compare pane

For parallel candidates:

- same task contract shown at top;
- candidate status and cost;
- changed-file/artifact summaries;
- verification matrix;
- human notes;
- model-assisted comparison clearly separated from measured evidence;
- synthesis action that creates a new candidate lineage rather than mutating winners.

### Browser pane

A real browser surface with:

- current origin and profile/context;
- automation method badge: `WebMCP`, `DOM/CDP`, `Semantic`, `Vision`;
- agent cursor/action indicator;
- human takeover mode;
- page/tool provenance inspector;
- downloads/uploads drawer;
- screenshot/evidence capture;
- auth/session persistence controls.

### Computer pane

Shows the enrolled machine or sandbox desktop when visual interaction is required.

- runtime identity and trust boundary always visible;
- app/window list where supported;
- human takeover;
- input activity indicator;
- screenshot/recording policy;
- emergency stop.

### Terminal pane

A high-quality PTY terminal with persistent scrollback and splits. Terminal output survives renderer restart through runtime-backed session state where feasible.

### Files / Editor pane

Kernux needs strong file inspection and light/medium editing, but it does not need to beat every full IDE before launch.

Must support:

- file tree;
- text/code editing;
- syntax/LSP integration where practical;
- image/PDF/Markdown/data previews;
- drag/drop into agent context;
- file provenance and agent-change markers.

### Diff / Review pane

- side-by-side/unified diff;
- line comments sent back to an agent;
- staged/unstaged distinction;
- candidate comparison;
- verification attached to exact head;
- source-control provider actions when configured.

### Artifact pane

Render generated outputs by type: Markdown, PDF, image, CSV/table, notebook/report, JSON, logs, archives. Every artifact exposes digest, producer, source run, and export/share actions.

### Timeline / Replay pane

Chronological event stream with filters for agents, capabilities, approvals, runtime, files, browser, verification, and errors.

Actions:

- inspect event;
- open referenced artifact/state;
- fork from supported checkpoint;
- compare two checkpoints/runs;
- export evidence bundle.

## 5. Permission experience

Permission prompts must be rare, legible, and consequential.

Each approval shows:

- **Action** — what will happen;
- **Target** — file/domain/machine/service;
- **Data/secret** — what sensitive context is involved;
- **Reason** — why the current task needs it;
- **Origin** — which agent/tool/content caused the proposal;
- **Duration** — once, this run, this project, or policy-managed;
- **Risk** — why Kernux escalated it.

Choices can include `Allow once`, `Allow for this run`, `Always allow within this bounded scope`, `Deny`, and `Edit scope` when safe.

Never present broad permission wording when the kernel can express a narrow grant.

## 6. Onboarding

First launch should reach useful work quickly.

### Step 1 — local capability

Kernux checks:

- desktop/daemon health;
- shell/git availability;
- browser availability;
- optional container runtime;
- known coding-agent CLIs.

### Step 2 — connect agents

Offer detected agents and ACP/CLI/provider setup without requiring a Kernux cloud account.

### Step 3 — open project

Folder or git repository.

### Step 4 — first task

A guided low-risk task demonstrates agent + terminal/browser + verification with clear permissions.

Advanced connections (remote machines, cloud browsers, MCP servers, organizations) stay optional.

## 7. Runtime and machine UX

A Machines view lists:

- local host;
- WSL distros;
- containers/sandboxes;
- SSH hosts;
- paired remote devices;
- cloud/GPU runtimes.

Each runtime shows:

- contact state;
- execution capability set;
- operating system/architecture;
- trust mode;
- policy owner;
- active runs;
- health/version;
- revocation/disconnect controls.

Tasks target capabilities, not hard-coded machine types. Users may pin a runtime when location matters.

## 8. Project UX

A project contains:

- tasks/runs;
- files/repository;
- agents/default strategy;
- skills/tools/MCPs;
- runtimes;
- secrets references;
- policies;
- artifacts/evidence;
- source-control integration;
- team membership later.

Project settings explain data boundaries in plain language.

## 9. Notifications

Notify only for meaningful state transitions:

- approval required;
- task blocked;
- agent needs user input;
- verification failed;
- run completed;
- runtime disconnected during active work;
- budget threshold reached.

Desktop/mobile unread state must be synchronized when remote services are configured, while local-only mode remains functional.

## 10. Mobile companion

The mobile app is for steering, not reproducing the entire desktop IDE.

Priority mobile flows:

- active task list;
- approval cards;
- agent follow-up;
- pause/resume/stop;
- diff/result summary;
- browser/computer screenshot stream when allowed;
- notifications;
- launch a predefined or simple task on an enrolled machine.

## 11. Accessibility

Accessibility is architectural:

- keyboard-first navigation;
- semantic roles and focus management;
- screen-reader-readable task/approval state;
- no color-only status encoding;
- reduced motion support;
- scalable typography;
- accessible terminal/browser controls;
- WebMCP/DOM semantics preferred over purely visual automation when possible.

## 12. Design language

Target feel: precise, calm, dense when needed, never noisy.

- one coherent token system;
- restrained elevation/shadows;
- strong information hierarchy;
- status colors with text/icon redundancy;
- monospace used for identifiers/code, not entire product chrome;
- motion communicates state transitions rather than decoration;
- destructive/irreversible operations look materially different from ordinary actions.

Do not clone donor styling. Reuse interaction lessons and proven primitives while establishing a distinct Kernux identity.

## 13. UX acceptance journeys

A release is not UX-ready until these journeys are demonstrated end-to-end:

1. Open a local repo -> delegate a bounded coding task -> review diff -> verify -> commit.
2. Ask a web research task -> browse/extract -> inspect provenance -> export artifact.
3. Run the same task with two agents -> compare evidence -> synthesize candidate.
4. Start work locally -> move or delegate a unit to SSH/sandbox -> resume after disconnect.
5. Encounter a permission escalation caused by web content -> deny/narrow safely.
6. Restart Kernux during a long-running task -> recover honest state without duplicate side effects.
7. Approve/steer an active task from mobile.

These journeys are product contracts, not demo scripts.