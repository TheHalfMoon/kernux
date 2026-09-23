# Open Computer Use Integration Plan

## Status

- Planning only; this document does not authorize source import by itself.
- Current P02/SG-000029 frontier remains authoritative and unchanged.
- Donor/reference: `opensymph/open-computer-use`.
- Planning revision: `5b433b98019c18201a15d11e8c3cb0010879a3d8`.
- Observed upstream license: MIT.
- Primary owners: P04 native computer runtime, P06 visual/input fallback; P03 Design Mode/headless qualification and P05 native-agent exposure consume the same abstraction.

## 1. Goal and architectural rule

Open Computer Use is valuable because it demonstrates one cross-platform computer-use shape over macOS Accessibility/ScreenCapture, Windows UI Automation/Win32, and Linux AT-SPI2/D-Bus, with accessibility-first actions, screenshots, app/window targeting, bounded snapshots, permission handling, and fixtures.

Kernux must reuse those strengths without inheriting donor authority.

```text
Agent / UI / CLI
  -> typed capability request
  -> kernuxd Grant + policy + egress
  -> Kernux ComputerUse contract
  -> platform adapter/helper
  -> OS accessibility/window/capture/input APIs
```

Forbidden authority paths:

```text
Agent -> raw donor MCP -> desktop authority
Renderer -> OS automation APIs
Model/UI text -> implicit permission
Environment variable -> authorization
Screenshot/accessibility content -> self-authorizing action
```

Kernux owns authorization, approval, identity, audit/evidence, lifecycle, recovery, and fallback policy.

## 2. Source disposition

| Donor area | Disposition | Constraint |
| --- | --- | --- |
| Accessibility-tree observation | port/adapt | Behind Kernux observation contract |
| App/window discovery | port/adapt | Bind stable app/process/window identity |
| Semantic accessibility actions | port/adapt | Preferred interaction path |
| Window-targeted input | reference/port selectively | Never implicit escalation |
| Screenshot capture | port/adapt | Local artifact + privacy/egress policy |
| Snapshot element mapping | reference/port | Replace durable raw indices with revision-bound refs |
| Fixture/smoke strategy | adapt | Kernux-owned deterministic fixtures |
| Permission onboarding | reference/adapt | Truthful degraded states |
| Global input | reference-only until separately authorized | Explicit high-risk capability |
| Private macOS/SkyLight behavior | reference-only | Not baseline support |
| Recording/polish compositor | out initially | Not required for core contract |
| Raw donor MCP transport | reference-only | Not a security boundary |
| Donor cursor image | exclude by default | Upstream says it came from an official Codex bundle; separate redistribution authority is not established by the donor MIT license |
| Donor branding/assets | exclude | No product impersonation |

Default transformation: `ported` or `adapted` behavior behind Kernux contracts rather than wholesale runtime embedding.

## 3. Kernux ComputerUse contract

### Observation

A `ComputerObservation` must carry:

- runtime/device identity;
- app/process/window identity;
- observation revision;
- bounded accessibility projection;
- optional screenshot artifact reference;
- coordinate-space metadata;
- capture timestamp;
- truncation/budget metadata;
- protected-content/redaction metadata;
- observation-method provenance.

Observation is never authority.

### Target identity

Use a revision-bound `ElementRef`, not a durable donor-style integer index. It must bind enough state to detect stale or substituted targets:

- observation revision;
- app/process identity;
- window identity;
- element identity/ordinal;
- optional role/name/frame fingerprint.

PID reuse, app relaunch, window replacement, stale observation, or mismatched identity must fail closed and require re-observation.

### Initial action vocabulary

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

Clipboard and notifications remain separately governed capabilities.

### Action methods

Every result records the actual method:

- `ACCESSIBILITY_SEMANTIC`;
- `WINDOW_TARGETED_INPUT`;
- `COORDINATE_FALLBACK`;
- `GLOBAL_INPUT`.

The request carries an allowed method ceiling. Failure at one level does not authorize a stronger level.

```text
ceiling = ACCESSIBILITY_SEMANTIC
semantic action unavailable
=> fail
=> never silently use coordinate/global input
```

### Capability separation

Do not create one `computer.use` super-capability. Refined Grains must keep authority separable for observation, screen capture, element interaction, typing, key input, app launch, focus/activation, coordinate fallback, and global input.

## 4. Safety model

### Background first

Preferred order:

```text
structured accessibility
-> app/window-targeted action
-> coordinate fallback
-> explicit global input
```

Physical pointer, keyboard focus, and foreground app should remain untouched when reliable targeted interaction exists.

### Sensitive targets

A password-manager denylist is insufficient. Policy must cover at least:

- password managers/secret stores;
- secure/password fields;
- OS credential prompts;
- OS permission/consent dialogs;
- payment/financial authorization;
- MFA/security-key enrollment;
- account recovery;
- destructive system controls.

Protected targets deny automation or require a separately defined stronger approval. Protected plaintext must not appear in trees, screenshots exposed to unapproved consumers, logs, Debug output, or ordinary evidence.

### Consequential actions

Purchases, external-message sends, deletion, security/privacy changes, credential changes, permission grants, and other irreversible/high-impact actions bind to the existing Kernux approval/capability model. The platform adapter does not decide consequence policy itself.

## 5. Privacy, egress, and secrets

Local computer observation does not authorize external processing.

Screenshot pixels/accessibility text transferred to `EXTERNAL_MODEL`, `EXTERNAL_TOOL`, `CONNECTED_ACCOUNT`, `REMOTE_RUNTIME`, or `TELEMETRY` require separate P02 egress authorization. `DIRECT_DESTINATION` also remains distinct from model processing.

Secret compatibility requirements:

- never log typed secret plaintext;
- never serialize secret-bearing keystrokes into ordinary evidence;
- never expose secure-field values;
- never retain secret plaintext for retries;
- preserve future handle-based secret injection;
- record protected operations without recording protected values.

Unexpected secure values from OS APIs must be redacted before crossing the platform-adapter boundary.

## 6. Resource and input bounds

Accessibility/screenshot content is attacker-influenced. Every adapter needs hard limits for:

- node count and depth;
- text bytes per node and total structured bytes;
- screenshot dimensions/encoded bytes;
- observation/action timeouts;
- retry count;
- cached observations;
- event/output volume.

Truncation is explicit metadata and cannot produce a misleading success.

## 7. Coordinate and race correctness

Coordinate fallback must account for:

- multi-monitor and negative coordinates;
- mixed DPI/Retina/backing scale;
- window-relative versus screen-relative spaces;
- display rotation where supported;
- virtual desktop/space behavior;
- window movement between observation and action.

Before actuation, revalidate target window identity and coordinate space.

Race/recovery handling must explicitly cover:

- app exit/restart and PID reuse;
- window replacement/movement;
- stale element;
- permission revocation;
- locked desktop session;
- display change;
- adapter/helper crash;
- timeout;
- partial drag/type sequence;
- human takeover.

Retryability/idempotency is explicit; retries are never assumed safe.

## 8. Platform strategy

### macOS

Use public Accessibility and supported capture/input APIs by default. Requirements:

- stable signed app/helper identity so TCC permissions survive upgrades;
- Accessibility + Screen Recording onboarding;
- truthful permission state and re-onboarding;
- no private SkyLight API in baseline;
- no focus stealing without explicit authority;
- native supported-version qualification.

### Windows

Use UI Automation + Win32 identity and supported capture/input APIs. Requirements:

- process/window ownership revalidation;
- isolation/timeouts for hanging UIA operations where needed;
- DPI-aware coordinates;
- no silent `SendInput`/global fallback;
- native architecture/version qualification.

### Linux

Use AT-SPI2/D-Bus as the structured path. Detect/report the actual desktop stack:

- X11;
- Wayland;
- accessibility availability;
- portal/capture availability;
- supported input methods.

Do not claim Wayland/global-input/screenshot parity from an X11-only path. `xdotool` behavior may inform an X11 adapter but does not define the universal contract.

## 9. Process placement

Automation executes either in a narrow `kernuxd` platform adapter or a dedicated least-privileged helper authenticated over private IPC. The implementation Grain chooses per platform based on isolation needs.

Renderer, browser page, raw MCP plugin, and model never directly own host-automation privileges. Helper crash/timeout fails closed and must not compromise daemon integrity.

## 10. Browser, Design Mode, and agent hierarchy

Browser order remains:

```text
WebMCP / typed capability
-> CDP/Playwright
-> semantic provider
-> qualified native ComputerUse
-> vision/coordinate fallback
-> human takeover
```

P06 consumes P04 ComputerUse; it must not import a second desktop-control stack.

P03 Design Mode may reuse read-only observation (identity, accessibility structure, screenshot, geometry, provenance). Captured context cannot authorize later action.

P05 native agents see ComputerUse only through capability-generated Kernux tool schemas, never raw donor MCP.

## 11. Human takeover

Human takeover is a first-class state:

- automated input stops;
- in-flight sequences cancel where safe;
- stale observations are invalidated;
- resume requires fresh observation and policy eligibility;
- evidence records whether physical pointer/focus was touched.

## 12. Deterministic fixtures

Build Kernux-owned fixtures before donor adaptation:

- native control gallery;
- normal + secure text fields;
- scroll containers;
- drag targets;
- menus/dialogs;
- multi-window app;
- dynamic/stale element fixture;
- deep/high-node tree;
- destructive-action fake confirmation;
- protected-content fixture;
- mixed-DPI/multi-display fixture where CI permits.

Never test against real password managers, personal accounts, or personal applications.

## 13. Cross-platform conformance

One contract matrix records per platform:

- supported operations;
- observation/action methods;
- required permissions;
- focus/pointer effects;
- coordinate support;
- failure/unsupported semantics;
- display-stack limitations.

Unsupported capability returns an explicit unsupported/degraded result, never a stronger fallback.

## 14. Evidence contract

A privileged action must be able to produce redaction-safe evidence containing:

- operation ID;
- Grant/policy decision reference;
- app/window identity;
- observation revision and target ref;
- requested action;
- actual action method;
- result/denial;
- before/after observation digests where appropriate;
- screenshot artifact digest where approved;
- timeout/timing metadata;
- platform adapter/version.

Do not retain secret plaintext, protected field values, full screenshots by default, or unrelated ambient UI. Retention follows the Kernux data lifecycle policy.

## 15. Packaging and updates

Before release prove:

- stable macOS signing identity/bundle ID and permission behavior across upgrades;
- truthful macOS re-onboarding when identity/permissions change;
- Windows binary provenance/signing required by release policy;
- Linux runtime/dependency detection;
- adapter protocol compatibility;
- rollback compatibility.

Do not inherit donor updater or telemetry behavior.

## 16. Dependency, license, and asset policy

Before each import wave:

1. reverify/pin exact upstream revision;
2. preserve Open Computer Use MIT license where required;
3. review current upstream `THIRD_PARTY_NOTICES.md`;
4. identify any Cua/yabai-derived code and preserve applicable notices;
5. exclude the donor cursor image unless separate redistribution authority is established;
6. exclude donor branding/update endpoints;
7. record source-to-destination mapping and transformation class;
8. characterize behavior before adaptation.

The donor MIT license does not automatically resolve separately sourced assets.

## 17. Import waves

### W0 — provenance

Pin revision, license/notices, source map, exclusions/private-API policy. No runtime code.

### W1 — characterization

Build fixtures and capture behavior for observation, semantic actions, stale targets, focus/pointer effects, permission denial, screenshots, timeouts, and platform limitations.

### W2 — Kernux contract

Freeze bounded observation, target-ref, action, method-ceiling, capability, denial/failure, and evidence contracts.

### W3 — structured adapters

Implement accessibility-first observation/semantic action on macOS, Windows, and Linux behind Kernux contracts using supported OS APIs.

### W4 — bounded fallbacks

Add only evidence-justified targeted/coordinate fallbacks. Global input remains separately gated and may remain unsupported where safe semantics cannot be proven.

### W5 — adversarial qualification

Prove stale refs, identity substitution, protected-content redaction, no silent escalation, permission revocation, non-self-authorizing UI/page content, and observation/egress separation.

### W6 — browser/agent integration

Expose only the qualified Kernux abstraction to P05/P06.

### W7 — release hardening

Prove signing/packaging, permissions, upgrades/rollback, crash recovery, and supported platform matrix.

## 18. Canonical task ownership

- P03-S05-T02 — hidden/headless UI qualification patterns.
- P03-S08-T01 — read-only native Design Mode observation.
- P04-S06-T01 — Kernux ComputerUse contract + structured host capability baseline.
- P04-S06-T03 — bounded Open Computer Use characterization/import/platform adapters.
- P04-S06-T04 — native cross-platform permission/focus/identity/coordinate/global-input qualification.
- P04-S06-T02 — adversarial host suite consumes the qualified surface.
- P05-S06-T02/T03 — native-agent exposure through capability-generated tools.
- P06-S05-T01 — visual/input fallback and human takeover consume P04 ComputerUse.
- P06-S08-T01 — browser privacy/egress remains authoritative.
- P15 — packaging/signing/update qualification.

No Open Computer Use work changes the current P02 frontier.

## 19. Implementation-readiness gate

Before the P04 ComputerUse surface can be called proven, evidence must show:

1. donor revision/license/notices pinned;
2. excluded assets/private APIs documented;
3. Kernux-owned ComputerUse contract exists;
4. renderer/model/raw MCP cannot bypass `kernuxd`;
5. observe/type/focus/coordinate/global-input authority is separable;
6. stale observations fail closed;
7. process/window identity substitution fails closed;
8. protected content redacts;
9. semantic/background interaction is preferred;
10. stronger-method escalation is never silent;
11. global input requires explicit capability;
12. supported coordinate transforms are proven;
13. snapshot/output budgets and timeouts are bounded;
14. exit/restart/revocation/recovery is truthful;
15. human takeover invalidates stale automation state;
16. local observations require separate egress authority before external use;
17. secret compatibility passes;
18. audit/evidence is redaction-safe;
19. native macOS qualification passes;
20. native Windows qualification passes;
21. supported Linux display-stack qualification passes;
22. X11/Wayland limitations are explicit;
23. deterministic fixtures pass;
24. browser structured paths precede ComputerUse fallback;
25. donor MCP is not authority;
26. packaging/update identity and permission behavior are proven before release;
27. provenance/notice validators pass;
28. ordinary local computer use needs no founder-funded metered service.

## 20. Anti-gap rule

Every implementation Grain derived from this plan must explicitly own:

- outcome/capability;
- authorization and approval;
- runtime/process;
- app/window/target identity;
- secret and data/egress boundary;
- persistence/retention/deletion;
- recovery/retry/idempotency;
- side effects;
- action method/fallback ceiling;
- evidence;
- accessibility;
- compatibility/update/rollback;
- cross-platform behavior;
- deterministic fixtures;
- negative/adversarial cases.

A Grain with any unowned item is not ready.

## 21. Non-goals

This plan does not authorize:

- replacing P02 policy;
- raw donor MCP as privileged authority;
- browser-first coordinate automation;
- arbitrary global keyboard/mouse injection;
- password-manager automation;
- donor branding/cursor assets;
- private macOS API dependence in baseline;
- desktop recording/polish as a core requirement;
- telemetry collection;
- cloud fallback;
- source import before provenance/characterization.

## 22. Completion rule

An imported behavior is implementation-ready only when it maps to:

```text
Kernux contract
+ capability/Grant
+ platform owner
+ secret/data/egress boundary
+ failure/recovery semantics
+ evidence
+ deterministic test
+ provenance/license record
```

Working upstream behavior alone is never completion authority.
