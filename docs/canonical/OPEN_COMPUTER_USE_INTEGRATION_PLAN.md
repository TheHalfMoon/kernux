# Open Computer Use Integration Plan

## Status

- Scope: planning only; no donor source import is authorized by this document alone.
- Current Kernux frontier: P02 remains authoritative. This plan is non-blocking to the active SG-000029 cycle.
- Donor/reference: `opensymph/open-computer-use`.
- Planning revision inspected: `5b433b98019c18201a15d11e8c3cb0010879a3d8`.
- Upstream license observed at that revision: MIT.
- Upstream third-party notices must be reviewed per import wave.
- Primary Kernux owners: P04 local computer runtime; P06 browser visual/input fallback; supporting P03 Design Mode/headless qualification and P05 native-agent tool exposure.

## 1. Purpose

Kernux needs a cross-platform native computer-use layer that can observe applications and windows, inspect structured accessibility state, capture screenshots, and perform bounded user-approved interaction without making raw desktop automation an authority boundary.

Open Computer Use is a strong donor/reference because it already demonstrates:

- one conceptual tool contract across macOS, Windows, and Linux;
- accessibility-first interaction before global synthetic input;
- structured accessibility-tree observation plus screenshots;
- app/window discovery and targeting;
- background-first interaction paths;
- explicit gates around focus stealing and global input;
- fixture-driven smoke testing;
- native platform implementations instead of a browser-only abstraction.

Kernux must reuse these strengths behind Kernux-owned contracts. The donor MCP server, environment-variable gates, process-local state, and raw local permission model are not the Kernux security boundary.

## 2. Non-negotiable architecture

The privileged path is:

```text
Agent / UI / CLI
    |
    v
typed Kernux capability request
    |
    v
kernuxd capability + Grant + egress policy
    |
    v
ComputerUse contract
    |
    v
platform adapter / helper
    |
    v
OS accessibility / window / capture / input APIs
```

The following paths are forbidden:

```text
Agent -> raw donor MCP -> desktop authority
Renderer -> platform automation APIs
Model text -> implicit permission
Environment variable -> authorization
Screenshot/AX content -> self-authorizing action
```

Kernux owns authorization, approval, audit, identity, evidence, lifecycle, and recovery.

## 3. Source disposition

| Donor area | Kernux disposition | Reason |
| --- | --- | --- |
| Accessibility-tree observation | port/adapt | High-value structured observation primitive |
| App/window enumeration | port/adapt | Needed for native computer contract |
| Semantic accessibility actions | port/adapt | Preferred non-intrusive actuation path |
| Window-targeted input | reference/port selectively | Useful before global input fallback |
| Screenshot capture | port/adapt | Required for visual fallback and evidence |
| Element-index snapshot mapping | reference/port | Useful, but must become revision-bound Kernux refs |
| Fixture/smoke-test strategy | adapt | Strong deterministic qualification pattern |
| Permission onboarding patterns | reference/adapt | Useful for macOS TCC and truthful degraded state |
| Global input fallbacks | reference-only until separately authorized | Highest-risk interaction mode |
| `sky_click` / private SkyLight behavior | reference-only | Private API fragility and supportability risk |
| Recording/polish compositor | out of initial import | Not required for core computer-use contract |
| Donor MCP transport | reference-only | Kernux transport/authority is not donor MCP |
| Donor software cursor asset | exclude by default | Upstream notice says the image came from an official Codex bundle; do not assume the repository MIT license grants redistribution |
| Donor branding/assets | exclude | Kernux must not impersonate donor products |

Default transformation strategy: prefer `ported` or `adapted` behavior behind Kernux contracts over wholesale source import.

## 4. Kernux ComputerUse contract

The contract must be provider- and platform-neutral.

### 4.1 Observation

A computer observation contains:

- runtime/device identity;
- app identity;
- process identity where available;
- window identity;
- observation revision;
- accessibility tree or bounded structured projection;
- optional screenshot artifact reference;
- coordinate-space metadata;
- capture timestamp;
- truncation/budget metadata;
- protected-content/redaction metadata;
- provenance describing the observation method.

Observation is not authority.

### 4.2 Stable target references

Do not expose donor-style element indices as durable authority.

Define a revision-bound `ElementRef` that includes enough information to detect stale observations, for example:

- observation revision;
- app/process identity;
- window identity;
- element ordinal or structural identity;
- optional role/name/frame fingerprint.

Any action against a stale or mismatched reference fails closed and requires re-observation.

PID reuse, window replacement, app relaunch, or observation revision change must not silently retarget an action.

### 4.3 Actions

The initial Kernux-owned action vocabulary should cover only what is required by canonical tasks:

- observe app/window;
- activate/raise window when explicitly authorized;
- semantic click/press;
- secondary action;
- scroll;
- drag;
- type text;
- press key/chord;
- set value;
- launch app;
- screenshot/capture.

Clipboard and notification capabilities remain separately governed even if exposed near the same UX.

### 4.4 Action method classification

Every action result records the method actually used:

- `ACCESSIBILITY_SEMANTIC`;
- `WINDOW_TARGETED_INPUT`;
- `COORDINATE_FALLBACK`;
- `GLOBAL_INPUT`.

A caller may request an allowed method ceiling. The adapter must not silently escalate to a stronger method.

Example:

```text
requested ceiling = ACCESSIBILITY_SEMANTIC
semantic action unavailable
=> FAIL
=> never silently use GLOBAL_INPUT
```

### 4.5 Capability scopes

Plan explicit capability scopes instead of one `computer.use` super-capability:

- observe apps;
- observe windows;
- inspect accessibility state;
- capture screen/window;
- interact with element;
- type text;
- send keys;
- drag/scroll;
- launch app;
- activate/focus window;
- use coordinate fallback;
- use global input.

Exact names are implementation-time schema decisions, but the authority separation is binding.

## 5. Safety and approval model

### 5.1 Background-first

The default method order is:

```text
structured OS accessibility
→ app/window-targeted interaction
→ coordinate fallback
→ global input only with explicit capability
```

The user's physical pointer, keyboard focus, and foreground application should remain untouched whenever the OS provides a reliable targeted action.

### 5.2 No silent escalation

Failures never escalate interaction strength automatically.

A semantic action failing does not authorize coordinate input.
A coordinate action failing does not authorize global input.
A hidden-window action failing does not authorize foreground focus stealing.

### 5.3 Sensitive targets

A static password-manager denylist is insufficient.

Kernux needs a policy classifier capable of treating at least these as sensitive:

- password managers and secret stores;
- secure/password text fields;
- OS credential prompts;
- permission/consent dialogs;
- payment/financial authorization surfaces;
- security-key/MFA enrollment;
- account recovery flows;
- destructive system controls.

Protected targets either deny automation or require an explicit stronger approval class defined by the future Grain.

No screenshot, tree projection, log, Debug representation, or evidence artifact may expose protected plaintext.

### 5.4 Destructive actions

Computer-use actions that can produce irreversible or high-impact effects must bind to the existing approval/capability model rather than heuristics in the adapter.

Examples include:

- confirming purchases;
- sending external messages;
- deleting data;
- security/privacy settings;
- destructive shell/app UI operations;
- granting OS permissions;
- changing account credentials.

## 6. Privacy and egress

Computer use is local host authority, not permission to send observations elsewhere.

Screenshot pixels and accessibility text are local data.

Any later transfer to:

- an external model;
- an external tool;
- a connected account;
- a remote runtime;
- telemetry;

must pass P02 egress policy separately.

A local observation grant does not imply an external-processing grant.

Browser `DIRECT_DESTINATION` network access also remains distinct from `EXTERNAL_MODEL` processing.

## 7. Secret handling

The computer-use layer must remain compatible with the proven secret broker:

- never log typed secret plaintext;
- never serialize secret-bearing keystrokes into ordinary evidence;
- never copy secure-field values into accessibility snapshots;
- never retain secret plaintext to support retries;
- use handles/approved secret injection paths when future tasks authorize them;
- record that a protected action occurred without recording the secret value.

If an OS accessibility API exposes a secure value unexpectedly, the adapter must redact it before it can cross the platform boundary.

## 8. Observation budgets and denial of service

Accessibility trees and screenshots are attacker-influenced inputs.

The contract therefore requires configurable hard limits for:

- node count;
- tree depth;
- text bytes per node;
- total structured bytes;
- screenshot dimensions;
- encoded screenshot bytes;
- capture timeout;
- action timeout;
- retry count;
- snapshot cache count;
- event/output volume.

Truncation must be explicit in metadata and must not silently remove the target while preserving a misleading success state.

## 9. Coordinate correctness

Coordinate fallback must account for:

- multiple monitors;
- negative desktop coordinates;
- mixed DPI/scaling;
- Retina/backing scale;
- window-relative vs screen-relative coordinates;
- display rotation;
- virtual desktops/spaces where applicable;
- window movement between observation and action.

Every coordinate action must bind to the observation coordinate space and verify that the target window identity still matches before actuation.

## 10. Platform strategy

### macOS

Primary APIs:

- Accessibility;
- ScreenCaptureKit or supported system capture path;
- public event APIs for explicitly authorized fallbacks.

Requirements:

- stable signed application/helper identity so TCC grants survive upgrades;
- Accessibility and Screen Recording onboarding;
- truthful permission state;
- no private SkyLight API in the baseline;
- no focus stealing unless explicitly authorized;
- native macOS qualification on supported versions.

### Windows

Primary APIs:

- UI Automation;
- Win32 window/process identity;
- supported capture/input APIs.

Requirements:

- validate process/window ownership before targeted actions;
- isolate risky/hanging UIA operations where necessary;
- DPI-aware coordinates;
- no silent SendInput/global fallback;
- Windows qualification across the supported architecture matrix.

### Linux

Primary structured path:

- AT-SPI2 over D-Bus.

Linux support must distinguish display stacks honestly.

Do not claim full Linux global-input/screenshot parity merely because an X11 path works.

The execution Grain must explicitly detect and report:

- X11;
- Wayland;
- desktop accessibility availability;
- portal/capture availability;
- unsupported global input paths.

X11-specific tools such as `xdotool` may be reference behavior but cannot define the universal Kernux contract.

## 11. Process placement

Platform automation runs either:

1. inside a narrowly bounded `kernuxd` platform adapter when safe; or
2. inside a dedicated local helper with authenticated private IPC and least privilege.

The final choice is made by the implementation Grain per platform.

The renderer, browser page, MCP plugin, or model must never directly own OS automation privileges.

Helper crashes/timeouts must fail the operation closed and preserve daemon integrity.

## 12. Browser hierarchy

Open Computer Use does not replace the structured browser stack.

Browser action order remains:

```text
WebMCP / typed site capability
→ deterministic CDP/Playwright
→ semantic targeting provider
→ native accessibility/computer observation
→ vision/coordinate fallback
→ human takeover
```

P06 consumes the P04 ComputerUse abstraction. It must not import a second unrelated desktop-control stack.

## 13. Design Mode

P03 Design Mode may reuse the observation model for native UI capture.

Design Mode is read/inspect by default:

- app/window identity;
- accessibility structure;
- screenshot;
- geometry;
- provenance.

Captured context cannot authorize a later action.

Any transition from inspect to act requires a capability request through `kernuxd`.

## 14. Deterministic fixtures

Before importing donor behavior, build Kernux-owned fixtures.

Required fixture families:

- native control gallery;
- text field and secure-field fixture;
- scroll containers;
- drag targets;
- menus/dialogs;
- multi-window app;
- dynamic/stale element fixture;
- high-node-count/deep-tree fixture;
- mixed-DPI/multi-display fixture where CI infrastructure permits;
- destructive-action fake confirmation surface;
- protected-content fixture.

Fixtures must never require interaction with real password managers, user accounts, or personal applications.

## 15. Cross-platform conformance

The same contract fixtures must test all supported adapters.

A platform capability matrix records:

- operation supported;
- observation method;
- action methods;
- required OS permissions;
- foreground/focus effects;
- coordinate support;
- limitations;
- failure semantics.

Unsupported capability is an explicit result, not a fallback to a stronger method.

## 16. Evidence model

Every privileged computer action should be able to produce redaction-safe evidence containing:

- operation ID;
- Grant/policy decision reference;
- app/window identity;
- observation revision;
- target reference;
- requested action;
- actual action method;
- result/denial reason;
- before/after observation digests where appropriate;
- screenshot artifact digests where allowed;
- timing/timeout metadata;
- platform adapter/version.

Do not store secret plaintext, protected field values, full screenshots by default, or ambient unrelated UI state.

Retention follows Kernux data lifecycle policy.

## 17. Recovery and race handling

The adapter must handle:

- target app exit;
- process restart;
- PID reuse;
- window replacement;
- stale element;
- window movement;
- permission revocation;
- desktop session lock;
- display change;
- helper crash;
- action timeout;
- partial drag/type sequence;
- user takeover during agent action.

Retry is never assumed safe.

The action contract must classify retryability and idempotency explicitly.

## 18. Human takeover

Human takeover is a first-class state, not an error workaround.

When takeover begins:

- automated input stops;
- in-flight input sequences cancel when safe;
- the agent cannot immediately resume on stale observation state;
- the next automated action requires fresh observation and policy eligibility.

Kernux should surface whether the physical pointer/focus was touched.

## 19. Packaging and updates

Platform permission state depends on stable binary/app identity.

Future packaging work must prove:

- stable macOS signing identity/bundle ID across updates;
- permission preservation or truthful re-onboarding;
- Windows binary provenance/signing as required by release policy;
- Linux package/runtime dependency detection;
- rollback compatibility;
- adapter protocol compatibility across upgrades.

Do not import donor updater or telemetry behavior into the core computer-use layer.

## 20. Dependency and asset policy

Before each source import:

- pin exact upstream revision;
- capture Open Computer Use MIT license;
- review its current `THIRD_PARTY_NOTICES.md`;
- identify copied Cua/yabai-derived code and preserve applicable notices;
- exclude the donor cursor image unless independent redistribution authority is established;
- do not inherit donor names/logos/update endpoints;
- record source-to-destination mapping and transformation class;
- characterize behavior before adaptation.

A donor MIT license does not automatically resolve separately sourced assets.

## 21. Import waves

### Wave 0 — provenance only

Deliver:

- donor entry;
- exact revision;
- license snapshot;
- third-party notice review;
- source map;
- explicit excluded assets/private APIs.

No runtime code.

### Wave 1 — characterization

Deliver Kernux-owned fixtures and behavior matrix for:

- observation;
- semantic actions;
- stale-target behavior;
- focus/pointer effects;
- permission denial;
- screenshot behavior;
- timeouts.

No donor architecture becomes canonical from characterization alone.

### Wave 2 — ComputerUse contract

Freeze the first bounded Kernux contract:

- observations;
- target refs;
- actions;
- action methods;
- capability scopes;
- denial/failure classes;
- evidence hooks.

### Wave 3 — structured platform adapters

Implement accessibility-first app/window observation and semantic action on macOS, Windows, and Linux behind the Kernux contract.

Prefer public supported OS APIs.

### Wave 4 — targeted and coordinate fallbacks

Add only the bounded fallbacks justified by evidence.

Global input remains separately capability-gated and may remain unsupported on platforms where safe semantics cannot be proven.

### Wave 5 — security/adversarial qualification

Prove:

- stale refs fail;
- identity substitution fails;
- protected content redacts;
- stronger-method escalation never happens silently;
- permission revocation fails closed;
- agent/page content cannot self-authorize;
- local observation does not imply egress.

### Wave 6 — browser/agent integration

Expose the qualified computer abstraction to P05/P06 through capability-generated tool schemas.

Raw donor MCP is not exposed as authority.

### Wave 7 — release hardening

Prove packaging, signing, OS permissions, upgrade/rollback, crash recovery, and the supported platform matrix.

## 22. Canonical task ownership

Primary ownership is:

- P03-S05-T02: hidden/headless desktop qualification patterns;
- P03-S08-T01: native Design Mode observation use;
- P04-S06-T01: Kernux ComputerUse contract and structured host capability baseline;
- P04-S06-T03: bounded Open Computer Use donor characterization/import and platform adapters;
- P04-S06-T04: cross-platform safety/permission/focus/global-input qualification;
- P04-S06-T02: host-boundary adversarial suite consumes the qualified computer-use surface;
- P05-S06-T02/T03: native-agent exposure only through kernel-generated capability schemas;
- P06-S05-T01: visual/input fallback and human takeover consume P04 ComputerUse;
- P06-S08-T01: browser privacy/egress re-evaluation remains authoritative;
- P15: release packaging/signing/update qualification.

No Open Computer Use work changes the current P02 frontier.

## 23. Implementation-ready acceptance matrix

Before P04 ComputerUse can be called proven, evidence must show:

1. donor revision/license/notices are pinned;
2. excluded assets/private APIs are documented;
3. a Kernux-owned ComputerUse contract exists;
4. renderer/model/raw MCP cannot bypass `kernuxd`;
5. capability scopes separate observe, type, focus, coordinate, and global input authority;
6. stale observations fail closed;
7. process/window identity substitution fails closed;
8. protected fields/content are redacted;
9. semantic/background action is preferred;
10. method escalation is never silent;
11. global input requires explicit capability;
12. coordinate transforms are proven on supported display configurations;
13. snapshot/output budgets and timeouts are bounded;
14. app exit/restart/permission revocation are handled truthfully;
15. user takeover invalidates stale automation state;
16. screenshots/accessibility data remain local unless separately egress-authorized;
17. secret compatibility passes;
18. redaction-safe audit/evidence passes;
19. macOS native qualification passes;
20. Windows native qualification passes;
21. Linux supported-display-stack qualification passes;
22. unsupported Linux/Wayland cases report explicit limitations;
23. deterministic fixtures pass;
24. browser uses structured paths before computer fallback;
25. donor MCP is not the security boundary;
26. packaging/update identity and permission behavior are proven before release;
27. provenance validators and notice inventory pass;
28. no founder-funded metered service is required for ordinary local computer use.

## 24. Anti-gap review

Every implementation Grain derived from this plan must identify:

- product outcome;
- exact capability;
- authorization owner;
- runtime/process owner;
- app/window/target identity model;
- secret boundary;
- data/egress boundary;
- persistence/retention;
- recovery;
- side effects;
- action method/fallback ceiling;
- evidence;
- accessibility;
- compatibility;
- deletion;
- cross-platform behavior;
- test fixture;
- negative/adversarial cases;
- rollback.

If one has no explicit owner, the Grain is not ready.

## 25. Explicit non-goals

This plan does not authorize:

- replacing P02 policy;
- exposing raw donor MCP as privileged authority;
- browser-first automation through desktop coordinates;
- arbitrary global keyboard/mouse injection;
- automating password managers;
- importing donor branding or cursor assets;
- private macOS API dependence in the baseline;
- recording/polishing desktop video as a core requirement;
- telemetry collection;
- cloud fallback;
- source import before provenance and characterization.

## 26. Completion rule

The Open Computer Use integration plan is implementation-ready only when every imported behavior maps to:

```text
Kernux contract
+ capability/Grant
+ platform owner
+ egress/data boundary
+ failure semantics
+ evidence
+ deterministic test
+ provenance/license record
```

No donor behavior is accepted merely because it works upstream.
