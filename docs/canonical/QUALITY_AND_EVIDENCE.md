# Quality and Evidence

## Principle

Kernux coordinates privileged, stateful, cross-platform work. Plausible output is not enough. Completion requires evidence bound to the exact change or runtime occurrence.

Kernux therefore uses two complementary disciplines:

- **SpecGrain-style preparation:** refine work until one unit is bounded and independently verifiable.
- **Diffcipline-style finish line:** inspect the exact diff, run the declared checks, and fail closed when proof is missing.

## 1. Evidence hierarchy

From strongest to weakest for a concrete claim:

1. deterministic machine-observed check on the exact revision/runtime state;
2. independently reproduced integration/E2E observation;
3. signed/hashed artifact or protocol transcript tied to a run;
4. structured runtime telemetry;
5. human review/observation;
6. agent/provider self-report.

Agent self-report may explain intent; it cannot close a gate that requires machine evidence.

## 2. Risk profiles

Every implementation Grain receives a minimum risk profile.

### R0 — documentation / inert metadata

Examples: typo, non-behavioral docs, comments.

Minimum proof:

- relevant formatting/link/schema checks;
- exact diff review.

### R1 — ordinary bounded product behavior

Examples: isolated UI behavior, pure logic, non-privileged adapter change.

Minimum proof:

- format/lint/static checks;
- focused unit tests;
- relevant contract tests;
- exact diff proof.

### R2 — cross-boundary or stateful behavior

Examples: persistence, IPC, browser/provider adapters, process lifecycle, git mutation, remote protocol, updater.

Minimum proof:

- R1 gates;
- integration tests;
- recovery/failure-path tests;
- compatibility tests where relevant;
- end-to-end journey evidence for user-facing behavior.

### R3 — security/privilege/release critical

Examples: capability policy, secret broker, sandbox boundary, auth/enrollment, renderer privilege bridge, update signing, release process, destructive action policy.

Minimum proof:

- R2 gates;
- adversarial/security tests;
- explicit threat-model review;
- cross-platform evidence where surface differs;
- negative/failure-path evidence;
- independent review or reproduction where feasible;
- release/provenance evidence when applicable.

A task may require a stronger profile than its category suggests. A task must not silently downgrade itself to make CI pass.

## 3. Spec readiness

Before implementation, a Grain must name:

- outcome;
- scope-in;
- scope-out;
- dependencies;
- expected change surface;
- acceptance conditions;
- risk profile;
- recovery/rollback path;
- context sources and provenance;
- evidence required;
- minimality choice/rationale;
- safety assumptions.

If these cannot be stated without hand-waving, refine the task.

## 4. Exact-head rule

Evidence belongs to an exact revision.

For repository work:

- tests/review must identify the head SHA they qualify;
- a moved head invalidates previous exact-head qualification unless the gate can prove the new delta is irrelevant;
- merge qualification occurs against the actual merge candidate;
- no `tests passed earlier` narrative substitutes for current evidence.

For runtime work:

- evidence identifies run id, runtime id, operation id, provider/adapter versions, and relevant artifact digests.

## 5. Required gate classes

The command names may evolve, but the classes are mandatory.

### Static hygiene

- format;
- lint;
- TypeScript/Rust static checks;
- dependency/schema generation freshness;
- dead-code/forbidden-import ratchets where valuable.

### Unit

Pure logic, parsers, policy evaluation, reducers/projections, serializers, migrations, adapter mapping.

### Contract

- TS ↔ Rust generated protocol compatibility;
- daemon IPC schemas;
- runtime capability negotiation;
- agent adapter contracts;
- browser adapter contracts;
- MCP/ACP/A2A/WebMCP fixtures;
- persisted schema compatibility.

### Integration

Real process/PTY, filesystem, browser, git, database, remote transport, sandbox, secret-provider integration as applicable.

### End-to-end

User journeys through the packaged application for changed critical paths.

### Security/adversarial

Privilege escalation, prompt injection, symlink/path escape, command injection, secret exfiltration, plugin escalation, remote replay, duplicate side effects, stale grants, cross-project session leakage.

### Compatibility

Representative macOS, Windows, Linux; WSL and SSH for surfaces that use them; mixed client/runtime protocol versions.

### Packaging/release

Install, update, uninstall, migration, signing/notarization, SBOM, checksums, provenance, rollback.

## 6. Test pyramid for privileged software

Do not rely on huge E2E suites to prove core safety.

Preferred layering:

```text
many deterministic unit/policy tests
        ↓
contract and protocol fixtures
        ↓
focused real-runtime integration
        ↓
small set of high-value E2E journeys
        ↓
release/platform qualification
```

Privilege decisions must be testable without launching the full desktop UI.

## 7. Characterization before donor adaptation

For imported donor code:

1. preserve or port upstream tests when useful;
2. add focused characterization for behavior Kernux relies upon;
3. prove the imported baseline;
4. then adapt behind Kernux contracts;
5. keep failures that reveal upstream assumptions instead of deleting inconvenient tests.

Large donor imports without characterization are not implementation progress.

## 8. Golden journeys

The project maintains executable golden journeys covering at minimum:

1. local project coding task;
2. parallel-agent candidate comparison;
3. browser research/extraction with provenance;
4. local host terminal/process work;
5. sandboxed generated-code execution;
6. SSH/remote task with disconnect/reconnect;
7. permission escalation caused by untrusted content;
8. task recovery after desktop/daemon restart;
9. secret-brokered external tool call;
10. mobile approval/steering once mobile exists.

Each golden journey defines expected state transitions, not only screenshots.

## 9. Reliability invariants to test continuously

- no duplicate side effect from retry/reconnect;
- unknown remote execution state remains `unverifiable`;
- renderer compromise cannot directly spawn arbitrary host processes through an undocumented path;
- project grants cannot escape path/domain/runtime scope;
- interrupted durable writes recover or fail closed;
- artifact digest mismatch is detected;
- event projection can rebuild from durable events;
- schema migration is forward-only with tested rollback/recovery strategy;
- old supported clients/runtimes negotiate safely with newer peers;
- app restart does not silently lose active run identity.

## 10. Security test corpus

Maintain adversarial fixtures for:

- malicious web pages and WebMCP manifests;
- prompt-injected documents;
- hostile filenames/archives;
- symlink/junction/reparse-point escapes;
- shell metacharacter/argv edge cases across platforms;
- malicious MCP/A2A/ACP payloads;
- revoked/expired credentials;
- stale remote tokens;
- compromised plugin manifests;
- permission UI/request mismatches;
- large-output/context exhaustion attempts.

Security fixtures should be safe and synthetic.

## 11. Performance budgets

Performance claims require measurement. Track at least:

- cold/warm desktop launch;
- daemon startup/reconnect;
- command-bar-to-first-agent-output latency;
- terminal input/output latency;
- pane switching/rendering;
- event projection throughput;
- large repository scan bounds;
- artifact import/export;
- browser action latency;
- remote round-trip overhead;
- memory with multiple concurrent agents/terminals.

Budgets are established empirically during implementation and ratcheted rather than invented in planning.

## 12. Product metrics and experiments

Do not collapse success into `agent success rate`.

For each task class measure:

- verified completion;
- user intervention count/reason;
- time to verified result;
- retries/failures;
- permission prompts;
- cost/usage;
- recovery after interruption;
- reproducibility/replay outcome.

Comparisons between agent strategies must preregister task sets, metrics, scoring, and exclusions before results are used for superiority claims.

## 13. Evidence bundle

Canonical Event/Evidence, Artifact binding, evidence-source, and redaction semantics are defined in [`EVENT_AND_EVIDENCE_MODEL.md`](EVENT_AND_EVIDENCE_MODEL.md).

A completed high-value run can export an evidence bundle containing:

- task/work-unit revisions;
- run identity;
- agent/provider/runtime versions;
- event manifest;
- approval decisions;
- changed file/git metadata;
- verification commands/results;
- artifact manifest with digests;
- browser/source provenance where applicable;
- unresolved warnings/limitations.

Sensitive content can be excluded/redacted while the manifest preserves that redaction occurred.

## 14. CI shape

Initial CI should evolve toward:

- `static` — format/lint/type/clippy/schema checks;
- `unit` — Rust + TS unit suites;
- `contract` — generated protocol and adapter fixtures;
- `integration-linux` — fast real-runtime integration;
- `integration-windows` — process/path/PTY specifics;
- `integration-macos` — platform specifics;
- `security` — deterministic adversarial corpus;
- `desktop-e2e` — selected hidden/headless journeys;
- `diffcipline` — exact diff + required risk verification;
- `supply-chain` — dependency/license/provenance checks;
- release-only packaging/signing qualification.

Avoid CI theater: a green job that did not execute its declared proof is not a pass.

## 15. Flake policy

A flaky test is a defect in the evidence system.

- do not blindly rerun until green and discard the first failure;
- classify infrastructure vs product vs test nondeterminism;
- preserve first failure evidence;
- quarantine only with a tracked owner, reason, and removal criterion;
- critical security/release gates may not be waived by a generic flaky label.

## 16. Universal capability benchmark

Generation 2 breadth must not turn Kernux quality into an unscored collection of demos.

Maintain a versioned benchmark/regression suite across the six product modes defined by the universal platform plan:

- **Build** — repository, terminal, browser, test, review and remote-runtime work;
- **Work** — email/calendar/files/apps, planning and cross-application execution;
- **Research** — search/fetch/browse, source collection, analysis, citation and report generation;
- **Automate** — schedules, triggers, condition watches, background execution and recovery;
- **Operate** — business systems, admin workflows, databases, support/CRM and remote systems;
- **Create** — documents, spreadsheets, presentations, reports, charts, sites/apps and media artifacts.

For each task fixture record:

- fixture/task version;
- exact Kernux revision;
- OS/runtime/browser version;
- provider/model/adapter version;
- connected integration versions where observable;
- required permissions and approvals;
- expected artifacts/evidence;
- success/failure/partial criteria;
- latency and active-compute measurements;
- cost owner and observed cost when available;
- retry/recovery behavior;
- negative evidence and unsupported behavior.

A benchmark result from one model/provider/runtime combination is not automatically representative of another.

### Universal reliability scorecard

Track distributions and task-class segmentation for at least:

- verified completion rate;
- false-completion rate;
- duplicate-side-effect incidents;
- crash/reconnect recovery success;
- scheduled-run lateness/miss rate;
- integration auth/refresh failure rate;
- browser/computer action recovery rate;
- artifact validation and round-trip failure rate;
- stale-context retrieval rate;
- user intervention/approval rate;
- notification false-positive/noise rate;
- latency to first useful progress and final verified result;
- variable cost per successful task where observable;
- storage/index growth and GC effectiveness;
- accessibility journey pass rate;
- provider-loss substitution success.

No single composite score replaces the underlying measurements.

### Universal golden journeys

The Generation 1 journeys remain mandatory. Generation 2 adds the canonical journeys from `UNIVERSAL_PLATFORM_PLAN.md`, including:

- morning work review;
- cross-app meeting follow-up;
- research-to-editable-deliverable;
- business operation across multiple systems;
- authenticated browser fallback;
- desktop application visual fallback;
- condition-watch notification;
- cross-device handoff;
- provider failure/substitution;
- memory correction/deletion;
- project portability;
- accessible operation;
- owner-sustainable execution.

Every universal journey defines expected state transitions and strongest-available evidence, not only a final screenshot or natural-language answer.

## 16. Completion vocabulary

Use explicit states:

- `PLANNED`
- `READY`
- `IN_PROGRESS`
- `IMPLEMENTED_UNPROVEN`
- `PROVEN`
- `BLOCKED_EXTERNAL`
- `DEFERRED`

Do not use `done` in canonical task state when the required evidence has not established `PROVEN`.

## 17. Project completion

Kernux as a whole is not complete merely because all planned tasks are checked off. Project/release completion requires:

- canonical acceptance journeys proven;
- supported-platform matrix qualified;
- security invariants tested;
- installer/update/recovery qualified;
- license/provenance closure;
- no unresolved release-blocking R3 findings;
- docs/onboarding/support surfaces complete;
- actual release artifacts independently verifiable.

## 18. Rule

**The faster agents become, the stronger the evidence boundary must become.**