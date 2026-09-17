# Security Model

## 1. Security objective

Kernux coordinates systems that can read private data, execute code, browse adversarial pages, control applications, use credentials, and create external side effects. The security model therefore assumes that **useful agent autonomy is a privileged distributed-systems problem**, not a prompt-engineering problem.

No model, plugin, browser page, renderer component, or remote agent is trusted to enforce its own permissions.

## 2. Trust zones

Kernux distinguishes at least these zones:

1. **Human user / organization policy** — authority source.
2. **Kernel authority (`kernuxd`)** — trusted policy and privileged-operation enforcement.
3. **Desktop/mobile UI** — trusted to present, not trusted with ambient host authority.
4. **Orchestrator** — trusted to coordinate requests, not to grant itself privilege.
5. **Agent/model** — probabilistic, potentially compromised by input.
6. **Plugin/tool/MCP server** — third-party executable/integration boundary.
7. **Browser/web content** — hostile by default.
8. **Downloaded/document content** — untrusted data that may contain instructions or active payloads.
9. **Sandbox runtime** — intentionally less trusted than host.
10. **Remote runtime/device** — separately enrolled trust domain.
11. **Cloud provider** — external data-processing boundary.

Crossing a zone requires an explicit typed interface and audit event.

## 3. Capability model

Every privileged operation becomes a capability request.

Representative request:

```json
{
  "subject": "agent-session:as_123",
  "run": "run_456",
  "action": "computer.files.write",
  "resource": "project://repo/src/auth.ts",
  "runtime": "runtime:local",
  "provenance": ["event:web-observation-789"],
  "constraints": {
    "max_bytes": 32768
  }
}
```

Policy evaluates the request against:

- subject/agent identity;
- task/work-unit scope;
- runtime and resource;
- project/org policy;
- current user grant;
- input provenance/trust;
- requested side effect;
- secret use;
- network destination;
- risk classification;
- time/delegation limits.

A grant is bounded. Avoid global `allow all tools` as the default mental model.

## 4. Permission profiles

User-friendly profiles compile to granular rules. Profiles are convenience, not separate authorization logic.

### Safe

- automatic low-risk reads inside explicitly opened project/workspace;
- no host writes, shell execution, secret use, or external side effects without approval;
- isolated execution preferred.

### Standard

- project-scoped read/write may be pre-authorized;
- low-risk declared verification commands may run automatically;
- network and browser access follow project/domain policy;
- secrets and consequential external actions remain gated.

### Developer

- project-scoped shell/process/git authority can be persistent for the run/project;
- host-system areas remain separately gated;
- destructive/system/credential/publish actions remain explicit unless an organization policy intentionally scopes them.

### Autonomous

- broad preauthorization may be granted for a bounded task/runtime/sandbox;
- autonomy never converts untrusted web/document instructions into new privilege;
- kernel hard constraints, organization policy, runtime isolation, secret scoping, and audit remain active.

### Custom

Granular policy editor for resources, actions, domains, runtimes, tools, time, budget, and approval rules.

## 5. Consequential action classes

Actions should be classified by consequence, not by UI surface.

Examples requiring stronger policy treatment:

- deleting or overwriting data without a recoverable path;
- changing OS/security settings;
- reading credential stores;
- using a secret with an external destination;
- sending messages or publishing content externally;
- creating purchases/financial commitments;
- changing production infrastructure;
- modifying repository/release governance;
- installing privileged software;
- executing untrusted/generated code on the host;
- granting new permissions to another agent/tool/runtime.

The approval UI must show **what will happen, where, with what data/credential, and why Kernux believes it is needed**.

## 6. Prompt-injection defense

Browser and document prompt injection is a core threat, not an edge case.

### Rule: data cannot authorize itself

Instructions encountered in web pages, emails, documents, tool output, source files, images, or remote-agent messages are treated as **untrusted content** unless the human/org explicitly designated that source as an instruction authority for the task.

Untrusted content may influence task understanding but cannot by itself expand capability scope.

### Provenance-aware actions

High-consequence requests carry source provenance. Policy can reject or escalate an action when the causal chain includes untrusted content.

### Structured separation

Keep system/task instructions, tool schemas, web/document content, and secrets as distinct typed channels internally. Avoid concatenating all context into one undifferentiated prompt where possible.

### Browser defenses

- domain allow/block policy;
- navigation provenance;
- popup/download policy;
- content-to-action taint metadata;
- sanitization/normalization of extracted text;
- optional injection classifier/adversarial detector;
- visual indication when a page caused a consequential proposal;
- human confirmation for cross-origin credential or external side-effect transitions when policy requires it.

No classifier result is a proof that content is safe.

## 7. Secret architecture

Secrets are referenced, not casually revealed.

### Requirements

- OS keychain/credential store or approved secret provider for plaintext-at-rest.
- Database stores secret references and policy metadata, not plaintext.
- Agent prompts receive secret handles when possible, not values.
- Sandbox code should use brokered/host-bound credentials where possible.
- Domain/service allowlists constrain secret use.
- Redaction applies to logs, transcripts, screenshots where technically possible, and artifact export.
- Secret access is auditable.
- Copying plaintext into clipboard/file/prompt is a distinct high-risk capability.

### Preferred remote/sandbox pattern

For HTTP credentials, prefer an egress broker that substitutes the actual credential only for an approved destination. The agent/sandbox holds an opaque placeholder rather than the secret value.

## 8. Execution isolation

Kernux exposes explicit execution modes:

### Host mode

Runs against the user's actual machine with declared host permissions. Appropriate for trusted project work that requires local applications or environment state.

### Container mode

Filesystem/network mounts are explicit. Good default for generated code and routine build/test tasks where full VM isolation is unnecessary.

### VM/microVM mode

Stronger isolation for untrusted code, security-sensitive tasks, or provider-backed environments.

### Remote mode

Execution occurs on an enrolled SSH/device/cloud host under that runtime's capability set and local policy.

The UI must make the active execution boundary visible.

## 9. Filesystem safety

- Normalize and resolve paths in the trusted kernel.
- Prevent path traversal outside a granted root.
- Resolve symlink/junction/reparse-point behavior before authorization, not only before I/O.
- Treat network shares, removable media, and special device paths as distinct policy surfaces.
- Atomic write/rename patterns where correctness requires them.
- Durable recovery for multi-step mutations that cannot be atomic.
- Destructive actions should offer preview and recovery when feasible.

## 10. Process and terminal safety

- Avoid shell interpolation when argv execution is available.
- Keep shell choice explicit and runtime-specific.
- Capture process identity, parentage, runtime, cwd, environment-policy summary, and lifecycle.
- Long-running sessions have stable ids and resumable/loggable output.
- Cancellation differentiates request cancellation from verified process termination.
- Process execution on remote hosts is owned by the execution host.
- Ambiguous retries of side-effecting commands fail closed unless idempotency is known.

## 11. Browser safety

Each browser task uses an isolated context unless the user explicitly reuses an authenticated profile.

Sensitive browser state requires:

- named profile/context;
- origin-aware cookie/storage handling;
- controlled download/upload paths;
- explicit persistence/retention;
- visible takeover state when a human controls the page;
- session recording policy;
- cleanup/revocation behavior.

Authenticated profiles must not silently leak across projects/tasks.

## 12. Plugin and integration security

Every extension/integration manifest declares:

- identity/version/source;
- requested Kernux capabilities;
- external network hosts;
- executable entrypoints;
- secret requirements;
- runtime requirements;
- data retention behavior where known.

Install/update flows show permission deltas.

Future marketplace packages should support signatures/provenance and immutable version resolution. Dependency vulnerabilities and malicious updates are part of the threat model.

## 13. Remote runtime security

Enrollment creates an explicit device identity and trust relationship.

Requirements:

- mutual authentication;
- short-lived/session-scoped authorization where practical;
- key rotation/revocation;
- replay protection;
- monotonic operation ids or idempotency keys;
- encrypted transport;
- capability/version negotiation;
- remote policy intersection: controller grants cannot exceed host policy;
- reconnect does not reauthorize expired privilege;
- durable host-side state for in-flight operations.

## 14. Audit and evidence

Security-relevant events are append-oriented:

- capability requested/allowed/denied;
- approval shown/resolved;
- policy source and revision;
- secret handle used and destination class;
- plugin installed/updated;
- runtime enrolled/revoked;
- browser profile persisted/reused;
- high-risk file/process/network action;
- sandbox created/destroyed;
- policy bypass attempt or denied escape.

Audit logs must avoid storing secret plaintext and should support export with integrity metadata.

## 15. Update and supply-chain security

Before public binary release:

- reproducible or independently verifiable build strategy where practical;
- locked dependencies and automated vulnerability review;
- SBOM generation;
- signed/notarized platform artifacts;
- release checksums and provenance attestations;
- protected release workflow;
- update metadata signing;
- rollback/recovery path;
- donor provenance verification.

## 16. Security test classes

Every relevant release gate should include tests for:

- path traversal/symlink escape;
- shell/argv injection;
- renderer-to-host privilege bypass;
- plugin capability escalation;
- prompt injection -> privileged action attempts;
- secret exfiltration attempts;
- cross-project browser/session leakage;
- sandbox escape assumptions/config mistakes;
- remote replay/duplicate side effect;
- lost connection ambiguity;
- stale/forged grants;
- malicious artifact names/content;
- update/provenance tampering;
- permission UI mismatch with actual request.

## 17. Security invariants

1. **Authority is code-enforced, not prompt-enforced.**
2. **Untrusted content cannot grant privilege.**
3. **A grant is scoped to subject + action + resource + runtime + constraints.**
4. **Secret use is distinct from secret disclosure.**
5. **Generated code does not default to host execution.**
6. **Remote/controller authority is the intersection of both sides' policy.**
7. **Unknown execution state is not success and not death.**
8. **Side-effect retries require an idempotency story.**
9. **Audit/evidence cannot depend on the same agent's self-report.**
10. **Security controls remain active in autonomous mode.**
