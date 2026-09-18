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


> Implementation note: this canonical contract is being landed in bounded R3 slices. Constraint, Grant, delegation, provenance, remote-intersection, and downstream sections remain unauthorized as complete until the successor slice merges and the macro task is canonically PROVEN.
