# Capability and Grant Model

## 1. Purpose

Kernux can read private data, execute code, control applications, use credentials, change repositories, operate browsers, and create external side effects. Those operations require one authorization vocabulary that is narrower than a model prompt, provider permission string, tool description, UI profile, or remote-runtime claim.

This document freezes the conceptual v1 contract for:

- CapabilityRequest identity and meaning;
- core capability actions;
- canonical resource URIs;
- resource containment;
- consequence classification;
- typed narrowing constraints;
- Grant issuance and matching;
- Grant lifetime, use accounting, revocation, and delegation;
- provenance-aware escalation;
- remote-policy intersection.

This document does not implement the policy engine, runtime protocol, Event envelope, wire-schema source, persistence, approval UI, secret broker, or organization policy service.

## 2. Security invariants

The capability model is governed by these rules:

1. A request proposes authority; it never creates authority.
2. A Grant records bounded authority; the Grant ID is not a bearer secret and possession alone grants nothing.
3. No model, agent, tool, browser page, document, remote peer, or provider-native identifier can mint or widen Kernux authority.
4. Unknown action, resource authority, constraint, encoding, capability version, policy result, or host-policy result fails closed.
5. Resource containment uses parsed canonical segments, never raw string-prefix comparison.
6. Constraints combine conjunctively. Normalization and intersection can only preserve or narrow authority.
7. Consequence classification is kernel-owned. A caller can supply facts but cannot lower the authoritative class.
8. Persistent convenience choices compile to policy rules that issue bounded Grants; there is no immortal global allow-all Grant.
9. Remote execution authority is the intersection of controller authorization, Kernux policy, runtime-advertised capability, and remote-host policy.
10. Untrusted content can influence task understanding but cannot authorize itself.
11. A Grant cannot make an operation possible when the runtime or trusted adapter does not implement the required capability.
12. Capability checks occur at operation admission; later runtime contracts own cancellation and ambiguous in-flight outcomes.

## 3. Authorization-record identity

CapabilityRequest and Grant are authorization records rather than additions to the nine core entity types frozen by the identity model.

Each record nevertheless uses the same canonical RFC 9562 UUIDv7 textual and validation rules defined in `IDENTITY_AND_REVISION_MODEL.md`.

Conceptual identities:

- `request_id` — unique identity of one authorization proposal;
- `grant_id` — unique identity of one issued bounded authorization.

Neither value is:

- a secret;
- a capability bearer token;
- proof that a policy decision remains active;
- proof that the referenced runtime or resource still exists.

A caller presenting a Grant ID must still pass current Grant-state, subject, action, resource, runtime, constraint, consequence, capability-negotiation, and policy checks.

## 4. CapabilityRequest

One privileged proposed operation is represented conceptually as:

~~~text
CapabilityRequest {
  request_id
  subject_scope
  action
  resource
  runtime_id
  requested_constraints
  provenance
  reason
}
~~~

### 4.1 Subject scope

For v1 agent/tool execution, subject scope binds:

- exact Run ID; and
- optional exact AgentSession ID when one session is the requesting principal.

Absence of an AgentSession ID means the request is run-scoped; it does not mean any session or any user.

Future subject kinds require an explicit versioned contract. A provider-native session/user/tool identifier never substitutes for a canonical Kernux subject reference.

### 4.2 Runtime binding

Every privileged request binds one exact canonical Runtime ID.

A local host is represented by its enrolled local Runtime identity rather than by an implicit wildcard such as `local`, `host`, or `*`.

The runtime binding does not prove capability availability. Runtime negotiation is separately owned by KX-P01-S03-T01.

### 4.3 Provenance

A request carries exact provenance references sufficient for policy to determine whether untrusted browser, document, tool, model, remote-agent, or downloaded content materially caused the proposal.

Provenance is policy input. It is never a substitute for human or organization authority.

### 4.4 Reason

A reason is display/audit context only.

Reason text can explain why an action is requested, but it cannot widen action, resource, constraints, consequence, lifetime, subject, runtime, or delegation.

## 5. Action grammar

### 5.1 Syntax

A v1 action is a lowercase dot-separated ASCII identifier.

Each token matches:

~~~text
[a-z][a-z0-9]*
~~~

A core action contains at least two tokens.

Examples:

~~~text
files.read
process.spawn
browser.navigate
grant.delegate
~~~

Issued Grants MUST bind one exact action.

The following are invalid in an issued Grant:

- `*`;
- `files.*`;
- `browser.**`;
- empty tokens;
- uppercase forms;
- whitespace;
- provider-native permission names used as if they were Kernux core actions.

Profiles and policy rules that conceptually cover multiple actions compile to multiple exact action decisions.

### 5.2 Core v1 action taxonomy

The core v1 set is closed and provider-neutral.

| Family | Core actions |
| --- | --- |
| files | `files.read`, `files.metadata`, `files.create`, `files.write`, `files.move`, `files.delete` |
| process | `process.inspect`, `process.spawn`, `process.signal` |
| pty | `pty.open`, `pty.write`, `pty.resize` |
| git | `git.read`, `git.modify`, `git.publish`, `git.admin` |
| browser | `browser.observe`, `browser.navigate`, `browser.interact`, `browser.download`, `browser.upload`, `browser.commit` |
| computer | `computer.observe`, `computer.input` |
| clipboard | `clipboard.read`, `clipboard.write` |
| network | `network.connect`, `network.send` |
| secret | `secret.use`, `secret.reveal` |
| artifact | `artifact.read`, `artifact.create`, `artifact.export`, `artifact.delete` |
| runtime | `runtime.inspect`, `runtime.manage` |
| sandbox | `sandbox.create`, `sandbox.manage`, `sandbox.destroy` |
| tool | `tool.invoke` |
| policy | `policy.inspect`, `policy.modify` |
| grant | `grant.issue`, `grant.revoke`, `grant.delegate` |
| identity | `identity.inspect`, `identity.manage` |

New core actions are additive versioned contract changes and are unknown to older policy engines until negotiated.

### 5.3 Extension actions

An extension action uses:

~~~text
ext.<registered-namespace>.<action...>
~~~

The registered namespace and exact extension action/version must be known to both the trusted adapter/runtime registry and policy engine.

Unknown extension namespaces or actions are denied.

Tool or provider self-description is untrusted metadata. It cannot register itself into the trusted action namespace or lower its consequence floor.

Where an external tool maps cleanly to a core action, adapters SHOULD use the core action rather than inventing an extension action.

## 6. Canonical resource URI

### 6.1 Generic form

Every capability request identifies one primary resource using a canonical URI:

~~~text
kernux://<authority>/<segment>/<segment>/...
~~~

The scheme is Kernux-owned. RFC 3986 supplies generic URI syntax and encoding rules; this contract intentionally applies stricter scheme-specific rules for authorization.

### 6.2 Canonical generic rules

At an authorization boundary, a resource URI is canonical only when all of these hold:

- scheme is exactly lowercase `kernux`;
- authority is exactly lowercase ASCII and registered by this capability version;
- userinfo is absent;
- port component is absent;
- query is absent;
- fragment is absent;
- path starts with `/`;
- required path segments are present;
- repeated slash / empty segment is absent;
- `.` and `..` segments are forbidden after canonical decoding;
- percent encoding uses uppercase hexadecimal digits;
- characters in the RFC 3986 unreserved set are not percent-encoded;
- malformed or incomplete percent escapes are rejected;
- UTF-8 data encoded into a URI segment uses canonical UTF-8 bytes followed by uppercase percent encoding;
- encoded slash, NUL, or another delimiter that would change segment structure is rejected;
- a canonical UUID segment obeys the lower-case UUIDv7 rules of `IDENTITY_AND_REVISION_MODEL.md`;
- no wildcard segment exists.

A noncanonical URI is rejected at the canonical authorization boundary rather than silently normalized and authorized.

An adapter/import boundary may normalize foreign input only before creating a new canonical resource value, and that normalization is itself trusted adapter logic.

### 6.3 No generic Unicode or filesystem safety assumption

The generic URI layer does not claim that Unicode normalization, case folding, symlink resolution, filesystem mount behavior, network name resolution, browser origin equivalence, or operating-system path semantics are safe merely because the URI parsed.

Each resource authority has a trusted resolver.

That resolver must convert the canonical logical resource into the platform/provider-specific target, prove any required containment, and reject ambiguity.

A URI string alone is never proof of:

- project filesystem containment;
- symlink safety;
- Windows or POSIX path equivalence;
- DNS/IP equivalence;
- browser same-origin equivalence;
- secret identity;
- runtime trust identity.

## 7. Core resource authorities

### 7.1 project

Examples:

~~~text
kernux://project/<ProjectId>
kernux://project/<ProjectId>/fs/src/auth.ts
~~~

Used for project-scoped metadata and logical filesystem resources.

Project filesystem paths are relative logical segments. They are not raw host absolute paths. The trusted filesystem resolver owns platform-specific resolution and project-root containment.

### 7.2 runtime

Examples:

~~~text
kernux://runtime/<RuntimeId>
kernux://runtime/<RuntimeId>/host/fs/etc/hosts
~~~

Used for a runtime and host-level resources outside a project abstraction.

Host filesystem resources require platform-specific trusted canonicalization before authorization.

### 7.3 artifact

~~~text
kernux://artifact/<ArtifactId>
~~~

Artifact content identity remains separately bound by digest when byte identity matters.

### 7.4 network

Canonical origin resource:

~~~text
kernux://network/origin/https/example.com/443
~~~

The origin scheme and host are lower-case canonical values supplied by a trusted network/browser resolver.

For v1 canonical origin resources:

- port is explicit decimal without leading zeroes;
- DNS international names use a trusted IDNA-to-ASCII canonical form before resource construction;
- IP literals use one trusted canonical textual representation;
- URL path/query is not smuggled into the origin resource.

A narrower URL/page constraint, when needed, is action-specific and still subject to trusted URL parsing.

### 7.5 secret

~~~text
kernux://secret/ref/<opaque-kernux-secret-ref>
~~~

This URI identifies the secret reference, not secret plaintext.

### 7.6 browser

Examples:

~~~text
kernux://browser/profile/<opaque-profile-ref>
kernux://browser/profile/<opaque-profile-ref>/origin/https/example.com/443
~~~

Provider/browser-native target IDs remain foreign metadata.

### 7.7 git

~~~text
kernux://git/project/<ProjectId>
~~~

Git refs/objects remain repository identities and do not replace the Kernux Project ID.

### 7.8 system

~~~text
kernux://system/os/settings
kernux://system/software/install
~~~

System resources are intentionally explicit because they normally carry elevated consequence floors.

### 7.9 policy

~~~text
kernux://policy/project/<ProjectId>
~~~

Future organization policy resources require an explicit later organization identity contract rather than inventing one here.

### 7.10 grant

~~~text
kernux://grant/<GrantId>
~~~

Used when revoking/delegating/inspecting a particular Grant.

### 7.11 tool

~~~text
kernux://tool/<registered-tool-ref>
~~~

The reference is a Kernux trusted registry identity, not a provider's display name.

### 7.12 sandbox

~~~text
kernux://sandbox/<registered-sandbox-ref>
~~~

Sandbox lifecycle and runtime semantics are defined later.

### 7.13 identity

Examples:

~~~text
kernux://identity/runtime/<RuntimeId>
kernux://identity/project/<ProjectId>
~~~

Used for identity inspection/management operations where the identity itself is the policy resource.

### 7.14 ext

~~~text
kernux://ext/<registered-namespace>/<segments...>
~~~

Extension resource namespaces are denied until explicitly negotiated and understood by the trusted resolver and policy engine.

## 8. Action/resource compatibility

An action is valid only against an allowed primary resource authority.

| Action family | Allowed primary authorities |
| --- | --- |
| files | project, runtime |
| process | runtime, sandbox |
| pty | runtime, sandbox |
| git | git |
| browser | browser |
| computer | runtime |
| clipboard | runtime |
| network | network |
| secret | secret |
| artifact | artifact |
| runtime | runtime |
| sandbox | sandbox |
| tool | tool, ext |
| policy | policy |
| grant | grant, policy |
| identity | identity |

A family/authority mismatch is rejected.

Operations involving multiple resources must expose each consequential secondary resource through a typed action-specific constraint or separate capability check.

Examples:

- browser upload requires browser authority plus explicit read authority for the uploaded Artifact/project file;
- a secret-backed network request requires `secret.use` plus network/browser authority;
- artifact export requires `artifact.export` plus authorization for the destination;
- process spawn does not implicitly grant arbitrary file, network, secret, or child-process authority.

No operation receives hidden secondary authority merely because its primary action was granted.

## 9. Resource scope matching

### 9.1 Scope modes

A Grant resource scope is one of:

- `exact`;
- `subtree`.

There is no implicit wildcard.

`subtree` is permitted only for a resource authority/path shape declared hierarchical by the trusted resource resolver.

### 9.2 Segment-aware containment

Resource matching is performed over parsed canonical resource components.

For subtree matching, candidate resource segments must have the entire base segment vector as a prefix.

Therefore:

~~~text
base:      kernux://project/P/fs/src
candidate: kernux://project/P/fs/src/auth.ts
result:    match
~~~

but:

~~~text
base:      kernux://project/P/fs/src
candidate: kernux://project/P/fs/src-old/auth.ts
result:    no match
~~~

Raw string prefix matching is forbidden.

Changing authority, Project ID, Runtime ID, percent-encoding representation, or a canonical segment prevents accidental containment.

### 9.3 Exact resources

Nonhierarchical identities such as one Artifact ID, one Grant ID, one secret reference, or one Runtime identity ordinarily require `exact`.

Attempting a subtree Grant over a resource kind that does not declare hierarchy is invalid rather than interpreted broadly.

## 10. Consequence classes

Consequence is an ordered policy input.

| Class | Meaning | Typical examples |
| --- | --- | --- |
| C0 | observation with no sensitive disclosure or external side effect | inspect public/non-sensitive state, bounded metadata observation |
| C1 | bounded reversible local mutation | edit project content with recovery, sandbox-local mutation |
| C2 | sensitive access/use or meaningful external side effect | network send, browser commit, git publish, artifact export, bounded secret use |
| C3 | destructive, privileged, or high-impact operation | unrecoverable delete, secret reveal, privileged host change, untrusted/generated host execution |
| C4 | authority expansion or irreversible external commitment | Grant delegation/issuance, policy authority expansion, trust-identity change, financial commitment, protected production/release governance |

The ordering is:

~~~text
C0 < C1 < C2 < C3 < C4
~~~

### 10.1 Kernel-owned classification

A request cannot select its authoritative consequence class.

Trusted policy logic computes at least a minimum class from:

- action;
- resolved resource;
- runtime/isolation boundary;
- data/secret classification;
- reversibility/recovery path;
- external side effect;
- production/release impact;
- requested delegation/authority change;
- causal provenance/trust.

Adapters may provide facts or a conservative suggested floor. A caller/tool/model suggestion can raise review attention but cannot lower the kernel result.

### 10.2 Explicit floors and escalators

At minimum:

- `secret.use` is C2;
- `secret.reveal` is C3;
- `grant.issue` is C4;
- `grant.delegate` is C4;
- `grant.revoke` is at least C2;
- `policy.modify` is C4;
- `identity.manage` is C4;
- privileged system/security changes are at least C3 and become C4 when they alter trust/authorization boundaries;
- unrecoverable deletion is at least C3;
- generated or untrusted code executed on the host is at least C3;
- network send, browser external commit, git publish, and artifact export are at least C2;
- a financial commitment is C4;
- production/release governance mutation is C4.

A normally low-risk read escalates when the target contains secrets or sensitive data.

A reversible mutation escalates if the recovery path is absent or unverifiable.

Untrusted causal provenance may require stronger approval or denial. It can never lower consequence or create authority.

## 11. Constraints

### 11.1 Constraint principles

Constraints are typed policy semantics, not free-form prompt text.

Rules:

- all active constraints are conjunctive;
- a Grant constraint remains enforced even if the request omits the corresponding request hint;
- an unknown mandatory constraint denies;
- an incomparable constraint denies;
- intersection/narrowing can only reduce authority;
- normalization cannot silently discard a constraint;
- an empty constraint set does not convert action/resource/runtime/subject into wildcards.

### 11.2 Common constraint families

Conceptual v1 families include:

#### Time

- not-before;
- expires-at.

The trusted kernel clock evaluates the window.

Expiry blocks new operation admission. It does not claim that an already admitted operation has been cancelled; S03 owns in-flight cancellation semantics.

#### Use count

- positive `max_uses`.

Use admission must be atomic.

One use is consumed when the operation is successfully admitted for execution, not when it completes successfully.

A failed operation after admission still consumes the use. A request rejected before admission does not.

This prevents concurrent/retry races from exceeding the authority budget.

#### Byte limits

- maximum bytes readable;
- maximum bytes writable;
- maximum upload/export bytes where applicable.

Requested/effective limits cannot exceed the Grant maximum.

#### Resource scope

- exact;
- subtree when the resource kind permits it.

#### Destination/origin

A set of canonical network/origin resources.

The effective destination set is the intersection of policy/request/Grant/runtime-host limits.

#### Process

Action-specific process constraints may bind:

- exact executable resource;
- exact or explicitly defined argv vector/prefix semantics;
- cwd resource;
- allowed environment keys;
- resource limits.

Arbitrary shell text is not treated as equivalent to a structured argv constraint.

#### Secret

- exact secret reference set;
- allowed destination/service set;
- use-versus-reveal semantics.

A Grant for `secret.use` never implies `secret.reveal`.

#### Secondary resources

Actions such as upload/export may bind explicit Artifact/project/network destination resources.

Each secondary resource remains independently validated.

### 11.3 Subset semantics

Typical narrowing rules:

- numeric maximum: child/effective maximum <= parent maximum;
- allowed set: child/effective set is a subset;
- time window: child begins no earlier and expires no later;
- exact value: values are equal;
- resource subtree: child scope is contained within parent scope;
- boolean permission where defined: false is narrower than true;
- delegation depth: child depth <= parent depth - 1.

Action-specific schema defines any additional comparison. If safe subset comparison is not defined, delegation/narrowing fails closed.

## 12. Grant record

An issued Grant is immutable authority metadata.

Conceptually:

~~~text
Grant {
  grant_id
  subject_scope
  action
  resource
  resource_match
  runtime_id
  constraints
  consequence_ceiling
  issuer_authority
  policy_revision
  issued_at
  not_before
  expires_at
  max_uses
  delegation_depth
  parent_grant_id?
}
~~~

### 12.1 One exact action

One Grant authorizes one exact action.

A profile or policy decision covering several actions issues/evaluates separate exact Grants or separate policy rules.

### 12.2 Exact subject/runtime

The Grant binds exact Run scope and, when applicable, exact AgentSession scope.

The Grant binds one exact Runtime ID.

There is no `any runtime`, `all agents`, or `*` value in an issued v1 Grant.

### 12.3 Finite envelope

Every issued Grant has:

- finite time window;
- positive finite max-use budget;
- explicit resource scope;
- exact action;
- exact runtime;
- bounded subject scope;
- bounded consequence ceiling;
- finite delegation depth.

Persistent convenience is represented by policy that can issue new bounded Grants, not by an immortal Grant.

### 12.4 Immutable issuance

After issuance, widening any authority field requires a new authorization decision and a new Grant ID.

The original record is not edited to become broader.

Grant state such as revocation, expiry, and use consumption is derived from trusted policy/event/consumption records around the immutable issuance record.

## 13. Grant matching and admission

A Grant can participate in an allow decision only when all applicable checks pass:

1. Grant record is valid and known.
2. Grant is not revoked.
3. Trusted time is within the active window.
4. Atomic remaining-use budget is positive.
5. request subject is within exact Grant subject scope.
6. request action equals Grant action.
7. request runtime equals Grant runtime.
8. request resource matches exact/subtree parsed-resource semantics.
9. requested/effective constraints are within Grant constraints.
10. authoritative consequence <= Grant consequence ceiling.
11. contextual escalation/policy still permits the operation.
12. runtime advertises the required action/version.
13. local kernel hard constraints permit it.
14. applicable project/user/organization policy permits it.
15. remote-host policy permits it for a remote runtime.
16. required secondary-resource authorizations are present.

Any mandatory unknown result denies or moves to an explicit approval/policy-resolution state; it never converts into allow.

Admission atomically consumes the use budget before side-effect execution begins.

Grant match does not guarantee operation success.

## 14. Revocation, expiry, and exhaustion

Derived Grant state may be:

- not-yet-active;
- active;
- revoked;
- expired;
- exhausted.

Revocation, expiry, or exhaustion prevents new operation admission.

This contract intentionally does not claim that revocation synchronously stops an operation already admitted. Cancellation/termination and ambiguous remote outcomes belong to KX-P01-S03-T01.

A policy engine must preserve enough audit state to explain why a Grant was active or inactive at an admission decision.

## 15. Delegation

### 15.1 Default deny

Default `delegation_depth` is 0.

A Grant with depth 0 cannot be delegated.

Possessing a delegable Grant is not, by itself, authority to delegate. The delegating subject must also be authorized for the C4 `grant.delegate` operation targeting the parent Grant/recipient context.

### 15.2 Child subset

A delegated child Grant must not exceed the parent in:

- action;
- resource scope;
- runtime;
- effective constraints;
- consequence ceiling;
- time window;
- use budget;
- authority lineage.

The child may target an explicitly authorized narrower/different subject only when the delegation decision authorizes that recipient.

Child delegation depth must be less than the parent's remaining depth.

### 15.3 Shared ancestor budgets

Delegation must not multiply parent authority.

A child max-use budget is bounded by the parent's remaining budget, and an admitted child operation consumes from the child and all applicable ancestor budgets atomically or through an equivalent shared-ledger mechanism.

Therefore a parent with 10 remaining uses cannot delegate two children that each independently create 10 additional effective uses.

### 15.4 Parent lifetime/revocation

A child cannot:

- begin before its parent;
- outlive its parent;
- survive parent revocation as usable authority;
- regain authority after an ancestor is exhausted.

Parent/child/issuer lineage is durable audit/evidence input.

## 16. Persistent policy and permission profiles

User-facing `Safe`, `Standard`, `Developer`, `Autonomous`, and `Custom` are profile inputs to one policy engine.

They are not alternate authorization paths.

A UI choice such as:

- Allow once;
- Allow for this run;
- Always allow within this bounded project scope;

is compiled into either:

- a bounded Grant; or
- a durable policy rule that may later issue bounded Grants.

There is no global hidden `allow all tools forever` Grant.

Policy rules remain subordinate to:

- kernel hard constraints;
- organization constraints;
- secret policy;
- runtime capability;
- remote-host policy;
- provenance escalation.

## 17. Remote authorization intersection

For a remote operation, effective authority is the intersection of all mandatory layers:

~~~text
valid request
AND bounded Grant / authorization decision
AND local Kernux policy
AND kernel hard constraints
AND negotiated runtime capability/version
AND remote-host policy
AND required secondary-resource authority
~~~

If the controller allows but the remote host denies, the operation is denied.

If the remote host allows but the controller lacks authority, the operation is denied.

If capability/version negotiation is absent or unknown, the operation is denied.

Reconnect does not resurrect expired/revoked authority.

## 18. Data cannot authorize itself

Untrusted content includes, unless explicitly designated otherwise by human/organization authority:

- browser pages;
- emails;
- documents;
- source/data files;
- images;
- downloaded content;
- tool/MCP output;
- external agent messages;
- provider descriptions.

Such content may:

- suggest an action;
- supply a target;
- explain a task;
- cause policy to classify a request as more risky.

It may not:

- mint a Grant;
- create a policy rule;
- widen resource scope;
- add a new allowed destination;
- extend expiry;
- increase use budget;
- increase delegation depth;
- lower consequence;
- claim its own tool action is safe;
- change runtime/host policy.

High-consequence requests retain provenance so approval can show which untrusted source materially caused the proposal.

No classifier declaring content “safe” is authorization evidence.

## 19. Action-specific notes

### 19.1 Generic tool invocation

`tool.invoke` does not mean “whatever this tool says it can do.”

A trusted registered tool descriptor maps the invocation to:

- exact trusted tool resource;
- capability/version;
- minimum consequence floor;
- secondary-resource needs;
- relevant constraints.

Untrusted tool name/description/output cannot lower those values.

If trusted metadata is insufficient, invocation is denied or escalated.

### 19.2 Process and PTY

A Grant for `pty.write` is not equivalent to unrestricted shell authority.

The effective action-specific constraints must bind the intended PTY/runtime context and any process/command restrictions that policy requires.

Generated/untrusted host execution escalates to at least C3.

### 19.3 Browser/computer

Authenticated browser state does not imply authority for every site action.

Navigation, interaction, upload, download, and external commit are distinct actions.

Computer input authority does not implicitly grant secret use, filesystem access, network egress, or policy modification.

### 19.4 Secrets

`secret.use` permits brokered use within exact destination/service constraints.

`secret.reveal` permits plaintext disclosure and has a higher consequence floor.

Neither action is implied by ordinary network, browser, process, or tool authority.

## 20. Adversarial invariants

Implementations and conformance fixtures must cover at least:

1. `kernux://project/P/fs/src` subtree does not match `.../src-old`.
2. encoded `.`, `..`, slash, NUL, malformed percent escapes, or repeated separators cannot create an alternate authorized resource.
3. uppercase/noncanonical URI variants do not become a second authorization representation.
4. wrong Project/Runtime UUID cannot match a resource solely because later path segments are equal.
5. an unknown action is denied.
6. an unknown resource authority is denied.
7. an unknown mandatory constraint is denied.
8. a provider-native permission string cannot substitute for a Kernux core action.
9. a request cannot lower kernel consequence.
10. untrusted content cannot create a Grant.
11. `secret.use` does not imply `secret.reveal`.
12. `files.write` does not imply `files.delete`.
13. `browser.interact` does not imply `browser.commit`.
14. `git.modify` does not imply `git.publish`.
15. a stale/revoked/expired/exhausted Grant cannot admit a new operation.
16. concurrent use accounting cannot exceed max uses.
17. a delegated child cannot escape parent resource scope.
18. child expiry cannot exceed parent expiry.
19. child/ancestor combined usage cannot multiply the parent budget.
20. controller allow cannot override remote-host deny.
21. remote-host allow cannot override missing controller authority.
22. runtime reconnect cannot resurrect revoked/expired authority.

## 21. Deferred wire/runtime details

This document freezes semantic v1 authorization rules, not the generated wire shape.

Deferred:

- schema source and code generation;
- serialization encoding;
- exact timestamp encoding;
- policy database layout;
- consumption/revocation storage;
- runtime capability-negotiation envelope;
- operation identity/idempotency/cancellation;
- structured runtime errors;
- Event/evidence envelope;
- approval UI implementation;
- organization identity/policy distribution.

Later contracts must preserve this model or replace it through an explicit ADR with security/migration impact.

## 22. Downstream requirements

KX-P01-S03-T01 must preserve:

- exact runtime binding;
- grant admission before side effects;
- expiry/revocation behavior for new admissions;
- no false claim that revocation proves in-flight cancellation;
- remote policy intersection;
- operation identity separate from Grant/request identity.

KX-P01-S04-T01 must preserve provenance, Grant/request decision lineage, and consequence/evidence context without using untrusted content as authority.

KX-P01-S05-T01 and later conformance work must make malformed actions, malformed/noncanonical resources, prefix escapes, unknown constraints, stale/revoked Grants, delegation widening, budget multiplication, and remote-policy disagreement executable negative fixtures.

No later capability adapter may silently weaken these invariants.
