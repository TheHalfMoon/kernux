# Product Strategy and Distribution

## 1. Purpose

A technically excellent Agent Operating Environment can still fail if installation is painful, trust is unclear, first value takes hours, or users must abandon the agents/tools they already pay for.

Kernux should optimize for **adoption through compatibility, trust, immediate usefulness, and compounding ecosystem value**.

This document is a product/distribution contract. It does not promise market leadership. It defines the product choices required to maximize the chance of broad durable adoption while preserving evidence-based claims.

## 2. Founding wedge

The first users are power users who already combine several of:

- Codex/Claude/OpenCode/Gemini/other agent CLIs;
- terminals;
- git worktrees;
- browsers;
- SSH/remote machines;
- MCP/skills;
- local scripts and development tools.

Their problem is not lack of another agent. It is **coordination fragmentation**.

Kernux's initial wedge is:

> Bring the agents and machines you already use into one task/evidence workspace without forcing you to change providers.

This wedge provides deterministic developer workflows that can prove the capability kernel before expanding to less deterministic knowledge work.

## 3. First-value target

The installation-to-value path should be measurable and aggressively simplified.

Ideal first-run journey:

```text
Install Kernux
    -> daemon starts safely
    -> detected agents/tools/browser/git shown
    -> choose/open a folder or general workspace
    -> type one bounded task
    -> watch agent + terminal/browser work
    -> inspect verified result
```

The user should not need to:

- create a Kernux cloud account for local work;
- manually edit JSON configuration for ordinary detected tools;
- understand MCP/A2A/ACP terminology;
- build a workflow graph;
- copy API keys into chat;
- choose ten provider knobs before the first task.

Advanced configuration is progressively disclosed.

## 4. Compatibility is distribution

Kernux grows faster when it works with tools users already have.

Distribution priorities:

1. detected local coding agents and provider-native subscriptions;
2. standard MCP/ACP/A2A/Agent Skills paths;
3. local folders/git repositories without import/conversion;
4. ordinary shells, WSL, SSH, containers;
5. existing browser profiles only through explicit safe linking/reuse;
6. generic CLI/OpenAPI integrations before bespoke one-off connectors.

A user should be able to adopt Kernux incrementally rather than migrate their entire workflow on day one.

## 5. Bring-your-own-agent and bring-your-own-runtime

Kernux should avoid making its own hosted inference or cloud runtime mandatory.

Users can choose:

- existing agent subscription/auth;
- API model provider;
- local model endpoint;
- local machine;
- SSH machine;
- container/sandbox;
- later managed/cloud runtime.

This reduces switching cost and prevents Kernux's business model from becoming architectural lock-in.

## 6. The activation event

Define activation as a **verified useful outcome**, not account creation or app launch.

An activated user has completed at least one meaningful task where Kernux coordinated two or more capability classes, for example:

- agent + files + terminal;
- agent + browser + artifact;
- agent + remote runtime + verification.

Track:

- install -> first project/workspace;
- project -> first task;
- task -> first verified result;
- time to verified result;
- setup blockers;
- permission-abandonment points;
- provider/runtime failures.

## 7. Retention loops

Kernux should become more useful as users work, without creating hidden lock-in.

Retention comes from:

### Persistent project truth

Tasks, evidence, artifacts, project decisions, context, and approved memory reduce repeated setup.

### Runtime continuity

Users can leave long-running work, reconnect, and recover honest state.

### Multi-agent leverage

Users can compare or delegate without rebuilding tool setup in every provider UI.

### Reusable capabilities

Connected agents, runtimes, tools, skills, policies, and project conventions become reusable under scoped permissions.

### Proof and replay

Users trust results because they can inspect how work happened and reproduce the important parts.

## 8. Trust is a growth feature

The strongest adoption blocker for computer-using agents is fear of uncontrolled action.

Trust-building product requirements:

- visible runtime/trust boundary;
- legible permission requests;
- emergency stop;
- exact changed-file/action summaries;
- evidence attached to results;
- clear local/cloud data boundary;
- no hidden global autonomy switch;
- export/delete controls;
- transparent failure/ambiguity states;
- preserved negative evidence.

Kernux should make safe autonomy feel more powerful, not more bureaucratic.

## 9. Shareable objects

Growth should come from useful artifacts, not spammy referral mechanics.

Potential shareable objects:

- task templates/recipes;
- skills;
- integration manifests;
- runtime adapters;
- evidence bundles;
- reproducible benchmark/journey definitions;
- sanitized run reports;
- project starter policies;
- Design Mode captures/issues where appropriate.

Sharing never implies sharing secrets, authenticated browser state, private source, or unrestricted grants.

## 10. Ecosystem flywheel

```text
More users
   -> more integrations/skills/adapters
   -> more tasks possible in Kernux
   -> more verified workflows/templates
   -> easier adoption for new users
   -> more users
```

The ecosystem should be protocol-first so contributors can build value without forking Kernux core.

Before a public marketplace, establish:

- versioned extension contracts;
- capability declarations;
- source/provenance metadata;
- update permission deltas;
- signatures/immutable versions where appropriate;
- security review/reporting path;
- uninstall/revocation semantics.

## 11. Open-source strategy

The public repository should be useful, buildable, documented, and trustworthy rather than a marketing shell around a closed product.

Open-source value should include the local core needed to understand and contribute to:

- capability contracts;
- local daemon/runtime;
- desktop/CLI fundamentals;
- protocol adapters;
- evidence/provenance machinery;
- extension SDK/contracts when stable.

Optional hosted collaboration/managed runtime services can remain separable. The architecture must not intentionally cripple local mode to force hosted adoption.

## 12. Developer experience as distribution

Contributors and integration authors need:

- one-command development bootstrap where practical;
- clear architecture map;
- generated protocol types;
- fixture/conformance harnesses;
- deterministic local tests;
- sample agent/runtime/tool adapters;
- extension templates after APIs stabilize;
- good error diagnostics;
- first-class Windows/macOS/Linux instructions;
- documented compatibility/version policy.

A contributor should not need tribal knowledge from chat to add a safe integration.

## 13. Installation and update channels

Public launch should target trusted native installation paths where practical, for example platform-signed installers and common package managers. Exact channels are an implementation/release decision.

Requirements independent of channel:

- signatures/notarization where applicable;
- checksums/provenance;
- predictable update channel;
- rollback/recovery;
- no privilege escalation hidden inside auto-update;
- clean uninstall and explicit local-data choice.

## 14. Product packaging

Avoid exposing architecture as a confusing product matrix. The commercial model is a **local-software subscription**, not bundled compute.

The base subscription pays for Kernux product entitlement while execution remains on the user's device, existing provider subscriptions, BYOK accounts, organization infrastructure, or user-owned remote runtimes. The base subscription must not include hidden founder-funded inference, search, browser, storage, relay, or automation usage.

Commercial entitlement and runtime authority are separate: subscription state may enable paid product modules, but it never grants filesystem, network, secret, browser, tool, or external-side-effect permission.

The normative economics and entitlement architecture are defined in [`LOCAL_SUBSCRIPTION_ZERO_RUNTIME_COGS.md`](LOCAL_SUBSCRIPTION_ZERO_RUNTIME_COGS.md).

### Open-source and paid product boundary

Kernux Core remains governed by the existing Apache-2.0 policy. A paid official product layer may contain separately licensed commercial modules or Capability Packs, official signed distribution/update entitlements, and support/organization conveniences, but any such boundary must be explicit and must not silently relicense Apache-covered source.

If every paid feature is released under Apache-2.0, subscription revenue should be treated as a convenience/distribution/support model because users retain the right to self-build the Apache-covered code.

### Subscription economics

The product must optimize for:

```text
monthly subscription revenue
- payment / merchant fees
- taxes and unavoidable commerce costs
- small fixed distribution/business costs
= gross margin

founder-funded variable runtime COGS
= 0
```

Do not meter local tokens, local tasks, local browser minutes, local storage, local models, local memory or local automations. These run on user-owned resources.



The default product is simply **Kernux**.

Possible named surfaces can include:

- Kernux Desktop;
- Kernux CLI;
- Kernux Mobile;
- Kernux Runtime;
- Kernux Cloud later.

Internals such as `kernuxd`, KRP, ACP, MCP, A2A, CAS, or policy engines are implementation concepts, not the primary onboarding vocabulary.

## 15. Expansion strategy

### Stage 1 — Developer/power-user wedge

Prove coding, browser, terminal, git, remote runtime, fleet, evidence, recovery.

### Stage 2 — Research and analysis

Strengthen web research, local documents/data, notebooks/scripts, source provenance, artifact/report export, native agent/context engine.

### Stage 3 — Operations and cross-application work

Add scheduled/triggered workflows, structured computer actions, service integrations, remote/mobile steering.

### Stage 4 — Teams

Shared policy, projects, agents/runtimes, collaboration and organization audit remain self-hostable or organization-owned by default. Optional managed relay/cloud is a separate priced add-on with positive unit economics, never a hidden cost of the base subscription.

Do not build separate incompatible products for these stages; expand the same task/capability/runtime model.

## 16. Adoption metrics

Measure cohorts by task class and platform.

### Acquisition/activation

- successful install rate;
- daemon healthy after install;
- detected agent/tool connection rate;
- time to first verified task;
- first-task failure reason.

### Engagement

- verified tasks per active user;
- active projects;
- capability classes used per user;
- agent/runtime diversity;
- reusable template/skill use;
- remote/mobile steering use when available.

### Retention

- weekly retained active users;
- project return rate;
- repeat verified-task rate;
- recovery/reconnect success;
- percentage returning without onboarding/help.

### Trust/safety

- permission prompts per task;
- deny/narrow rate;
- abandoned tasks due to permission confusion;
- consequential-action incident rate;
- recovery from ambiguous/failed side effects;
- secret/privacy incidents.

### Ecosystem

- active third-party integrations;
- adapter/skill installs and successful task completions;
- extension update failure/security rate;
- contributor time to first accepted change.

Do not optimize raw task count by encouraging meaningless autonomous loops.

## 17. Launch gates

### Internal/dev preview

Architecture and core contracts can change quickly. Focus on proving local kernel/runtime and donor integration strategy.

### External alpha

The product must already feel unified and safe. The alpha definition in the execution/audit docs applies.

### Public beta

Require at minimum:

- reliable installers/updates on declared beta platforms;
- clear onboarding and diagnostics;
- real-world crash/recovery telemetry available on an opt-in/privacy-safe basis;
- documented supported agents/runtimes/integrations;
- no known unresolved critical privilege bypass;
- migration compatibility policy.

### v1

Requires the P15 release qualification gates and evidence-based public claims.

## 18. What not to optimize for

- maximum number of provider logos;
- largest prompt/context window;
- highest autonomous step count;
- flashy workflow graphs;
- copying every donor feature before core coherence;
- forcing cloud sign-up to inflate accounts;
- bundling unlimited founder-funded inference/browser/search/storage into the local-software subscription;
- per-task/token metering for resources supplied by the user's own device/accounts;
- benchmark wins without representative reproducible tasks;
- marketplace size before extension safety.

## 19. Durable moat

Donor code alone is not a durable moat because source can evolve and competitors can implement similar features.

Kernux's durable product advantage should come from the combination of:

- a coherent capability/permission kernel;
- execution across local + remote runtime fabric;
- provider-neutral agent/model/tool interoperability;
- trustworthy context and memory with provenance;
- durable event/evidence/replay history;
- cross-platform reliability;
- a broad safe extension ecosystem;
- low switching cost into Kernux and low lock-in cost out of Kernux.

That combination compounds with user trust and ecosystem depth.

## 20. Rule

**Kernux should be easier to adopt than the fragmented stack it replaces, safer to trust than opaque autonomy, and more valuable with every compatible agent/tool/runtime added.**
