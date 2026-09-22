# Local Privacy Qualification and Delivery

> Normative module of the Kernux Local Privacy Implementation Plan.

## 17. Local Privacy Inspector

Before first external alpha, users must be able to inspect at least:

- current privacy mode;
- connected accounts;
- configured external AI providers;
- active remote runtimes;
- secrets by reference/owner, never value;
- recent egress by destination/class;
- which providers received task content;
- local data locations;
- memory/index state;
- browser profiles;
- telemetry/update settings.

The inspector is a view over real policy/event state, not a decorative settings page.

## 18. Privacy evidence

Every external data transfer should emit evidence sufficient to answer:

- what operation caused it;
- which destination/provider;
- egress class;
- which data classes crossed;
- which credential handle was used;
- which task/run authorized it;
- which policy/grant allowed it;
- whether content was redacted/minimized;
- timestamp and result.

Evidence must not reproduce the sensitive payload itself unless explicitly required and safely stored.

## 19. Privacy adversarial corpus

Add deterministic fixtures for:

- prompt injection requesting secret upload;
- tool output requesting wider network access;
- browser redirect to undeclared/private destination;
- DNS rebinding/private-network target;
- model provider fallback attempted from local-only mode;
- external embeddings attempted from local-only mode;
- cross-project memory retrieval;
- cross-account connector leakage;
- browser-profile leakage;
- deleted memory remaining in derived indexes;
- secret echoed in process output;
- telemetry with content payload;
- crash bundle containing private data;
- provider SDK attempting undeclared telemetry;
- remote runtime requesting extra context;
- stale grant after mode downgrade;
- task changed from EXTERNAL_AI to LOCAL_ONLY mid-run.

Negative evidence is preserved.

## 20. Privacy SLOs and release metrics

Track at least:

- undeclared external egress: target zero;
- secret plaintext in ordinary logs/evidence: target zero;
- silent cloud fallback: target zero;
- cross-project/account leakage in qualification corpus: target zero;
- external-provider calls without provider/data-boundary identity: target zero;
- deletion residuals in owned derived indexes after completed deletion: target zero;
- local-only golden-journey completion rate;
- context-minimization ratio where measurable;
- privacy-mode policy evaluation failures;
- redaction failures;
- unsupported provider telemetry disclosures.

These are engineering metrics, not marketing claims.

## 21. Implementation mapping

This plan refines the existing roadmap rather than creating a new parallel roadmap.

### P02 — privileged kernel

Must establish:

- secret-provider abstraction;
- handle-based secret use;
- capability/policy foundation;
- metadata/artifact/evidence storage already planned.

Add during dependency-correct refinement, not out of order:

- egress-class policy primitives if not already representable;
- destination metadata required by later network mediation.

### P03 — desktop/headless shell

Must add:

- Privacy Inspector shell;
- privacy mode selection/status;
- provider/account/runtime visibility;
- local data-location and deletion/export entry points.

### P04 — local computer/data runtime

Must prove:

- local files/process/PTY golden path;
- local data-store boundaries;
- private-vault/encryption-at-rest strategy for Kernux-managed sensitive content;
- no-network local execution fixture.

### P05 — agents/models/Decision Fabric

Must add:

- external-vs-local provider metadata;
- at least one qualified local generative-model adapter before local AI claims;
- deterministic fake provider for CI;
- local DecisionProvider qualification;
- no-silent-cloud-fallback tests;
- provider-call ContextBundle digest/evidence.

### P06 — web/browser

Must prove:

- local browser execution;
- DIRECT_DESTINATION separate from EXTERNAL_MODEL;
- redirect/DNS/private-network controls;
- authenticated profile isolation;
- TinyFish/AgentQL hosted paths optional;
- deterministic offline fixtures.

### P07-P09 — orchestration/evidence/replay

Must prove:

- privacy mode inherited by child WorkUnits;
- child agents cannot widen data boundary;
- egress evidence/replay;
- provider fallback respects privacy mode;
- restart does not restore expired/wider grants.

### P10 — isolation/runtime

Must qualify stronger network isolation profiles for sandbox/container/VM where supported.

### P11 — tools/integrations

ToolDescriptor must include egress/data-boundary/secret/cost metadata.

Unknown sensitive-tool data boundary fails closed.

### P12 — automation

Background/scheduled work inherits the same privacy mode and cannot activate external providers merely because the user is absent.

### P13-P14 — remote/mobile/team

Remote devices/teams are explicit trust domains.

No automatic upload of local memory/task history to a Kernux cloud is permitted as a prerequisite for continuity.

### P16-P20

Artifacts, integration fabrics, proactive work and ecosystem packages inherit the same local-privacy contracts.

No Capability Pack may introduce an unmediated cloud dependency.

## 22. Implementation-ready work packages

Final IDs remain owned by SpecGrain refinement, but the work is dependency-bounded as follows.

### LP-01 — Egress taxonomy and policy contract

Outcome:
Kernux can represent and deny/allow every network operation by explicit egress class and destination metadata.

Acceptance:

- typed egress enum;
- destination identity;
- local-only denial fixture;
- redirect destination re-evaluation contract;
- exact event/evidence shape;
- cross-language protocol fixtures if wire-visible.

Risk: R3.

### LP-02 — Provider privacy manifest

Outcome:
Every model/tool/web/connector provider declares local/external behavior and data boundary.

Acceptance:

- schema;
- fail-closed validation;
- unknown sensitive metadata rejected;
- provider revision binding;
- provenance integration.

Risk: R2.

### LP-03 — No-silent-cloud-fallback enforcement

Outcome:
Local provider failure never causes external data movement without existing authority.

Acceptance:

- local failure fixture;
- external provider configured but unauthorized fixture;
- user-approved escalation fixture;
- cancellation/race/restart tests.

Risk: R3.

### LP-04 — Context minimization and redaction contract

Outcome:
Provider calls receive a digest-bound minimum ContextBundle with secret exclusion.

Acceptance:

- selected-source accounting;
- omitted-source disclosure;
- secret-handle exclusion;
- provider call bound to ContextBundle digest;
- adversarial tests.

Risk: R3.

### LP-05 — Privacy Inspector

Outcome:
User can inspect actual privacy state and recent egress.

Acceptance:

- privacy mode;
- external providers;
- accounts;
- remote runtimes;
- recent egress;
- local storage locations;
- telemetry/update settings;
- keyboard/semantic accessibility.

Risk: R2.

### LP-06 — Local generative-model qualification

Outcome:
One bounded AI task completes without an external model API.

Acceptance:

- exact local runtime/model revision;
- offline-after-install proof;
- bounded resource behavior;
- cancellation/OOM behavior;
- local ContextBundle;
- evidence;
- no network beyond loopback during the qualified task.

Risk: R2/R3 depending on runtime.

### LP-07 — Local DecisionProvider qualification

Outcome:
Decision Fabric has one qualified local path and remains optional.

Acceptance:

- typed decision fixtures;
- calibration;
- OOD/abstain;
- multilingual sample;
- offline proof;
- fallback disabled/enabled parity for authorization semantics.

Risk: R2.

### LP-08 — Local memory/search qualification

Outcome:
Useful retrieval works with no external embedding/model service.

Acceptance:

- lexical/exact/temporal path;
- provenance;
- delete/export;
- derived-index deletion;
- cross-project denial.

Risk: R2.

### LP-09 — Local browser privacy boundary

Outcome:
Browser can visit explicit sites locally without page content being silently sent to external AI.

Acceptance:

- DIRECT_DESTINATION fixture;
- EXTERNAL_MODEL separate approval;
- profile isolation;
- redirects/private-network adversarial corpus;
- download/upload scope.

Risk: R3.

### LP-10 — Sensitive local-vault protection

Outcome:
Kernux-managed sensitive durable content has a qualified at-rest protection strategy.

Acceptance:

- threat model;
- key ownership/storage;
- migration/backup/restore;
- loss/recovery behavior;
- platform qualification;
- no plaintext key beside ciphertext.

Risk: R3.

### LP-11 — Telemetry/update privacy qualification

Outcome:
Core works with telemetry off; updates never send project content.

Acceptance:

- telemetry-disabled golden path;
- support bundle redaction;
- model/software update fixtures;
- no content-bearing update request.

Risk: R2.

### LP-12 — Local-only end-to-end golden journey

Outcome:
A meaningful Kernux workflow completes with no external AI/cloud service.

Minimum journey:

1. open local repository/project;
2. local search/context assembly;
3. local model or deterministic local agent path;
4. local file/process execution;
5. local verification;
6. local artifact/evidence;
7. restart/replay;
8. inspect privacy evidence.

Network oracle must confirm no non-loopback traffic for the fully offline fixture.

Risk: R3.

## 23. Alpha privacy gate

Kernux must not claim a privacy-first/local-first external alpha until all are proven:

1. secret handles and OS secret provider;
2. privacy mode contract;
3. egress classification;
4. no-silent-cloud-fallback;
5. local file/process/PTY path;
6. local search/memory path;
7. one local model-backed or explicitly model-free meaningful AI/agent path;
8. local browser separation between direct network and external AI;
9. Privacy Inspector;
10. deletion/export for Kernux-owned user data;
11. telemetry-off operation;
12. privacy adversarial corpus;
13. local-only golden journey with network oracle;
14. exact-head review and required R3 security evidence.

## 24. Definition of implementation-ready

The local-privacy plan is implementation-ready when:

- this contract is adopted into canonical planning;
- each LP package is mapped to a dependency-correct SpecGrain unit before its phase becomes eligible;
- active P02 authority remains unchanged;
- no privacy requirement exists only in prose without a future owner;
- providers introduced by PLATFORM_FABRICS_AND_SOURCE_INTEGRATION.md inherit this contract;
- the first applicable implementation Grain names its privacy acceptance and evidence explicitly.

## 25. Final rule

> **Local is the default trust boundary. Network access is a capability. Cloud processing is an explicit data-boundary decision. Failure never silently weakens privacy.**

