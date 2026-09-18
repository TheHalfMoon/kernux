# Runtime Operation Model

## 1. Purpose

Kernux coordinates privileged work across local, sandboxed, and remote execution hosts. A controller can lose transport contact while a process continues, receive a timeout after a side effect already started, reconnect to a runtime with newer truth, or ask for cancellation without knowing whether termination completed.

This document freezes the conceptual v1 runtime-operation semantics required before transports, adapters, persistence, generated schemas, or event envelopes are implemented.

It defines:

- RuntimeDescriptor and capability negotiation;
- contact state;
- execution state;
- RequestId and OperationId;
- canonical operation fingerprinting;
- duplicate-operation and replay behavior;
- authoritative runtime observation and reconciliation;
- cancellation semantics;
- structured provider-neutral errors;
- retry/side-effect certainty;
- reconnect, revocation, and identity continuity.

This document does not define the S04 Event envelope or the S05 generated wire representation.

## 2. Runtime-operation invariants

1. Runtime type or provider name never implies capability.
2. Capability advertisement is not authorization.
3. Contact state and execution state are independent.
4. Disconnect, timeout, controller restart, or missing heartbeat never proves execution exit.
5. Only the execution owner may authoritatively establish host-owned execution status.
6. RequestId identifies one protocol attempt; OperationId identifies one logical operation.
7. A side-effecting logical operation keeps one OperationId across transport retries and reconnects.
8. Reusing an OperationId with a different canonical fingerprint is a conflict and MUST NOT execute.
9. A duplicate start for the same OperationId/fingerprint resolves to the same logical operation rather than creating a second side effect.
10. An ambiguous side-effect result is reconciled before a fresh OperationId is used for the same intended effect.
11. Cancellation acceptance is not termination proof.
12. Provider-native status/error strings are diagnostic metadata, not canonical Kernux truth.
13. Reconnect never resurrects expired or revoked authority.
14. Unknown retry safety defaults to reconciliation or denial, not automatic re-execution.
15. Runtime observations are monotonic by trusted observation generation; stale controller cache cannot overwrite newer runtime truth.

## 3. RuntimeDescriptor

A RuntimeDescriptor is the trusted description of one enrolled Runtime identity at one observation point.

Conceptually:

~~~text
RuntimeDescriptor {
  runtime_id
  runtime_revision
  protocol_contract
  capabilities[]
  features[]
  observation_generation
  observed_at
}
~~~

### 3.1 Runtime identity

`runtime_id` is the canonical UUIDv7 Runtime ID frozen by `IDENTITY_AND_REVISION_MODEL.md`.

The descriptor never uses:

- provider display name;
- hostname;
- SSH alias;
- container name;
- cloud instance ID;

as canonical Runtime identity.

Provider/native identifiers may remain foreign metadata.

If enrolled trust identity changes in a way that breaks identity continuity, the replacement receives a different Runtime ID.

### 3.2 Runtime revision

The descriptor references the exact Runtime revision when mutable Runtime metadata matters.

A descriptor for an older Runtime revision is stale metadata and cannot silently overwrite a newer accepted revision.

### 3.3 Protocol contract

The runtime advertises a protocol-contract identifier/version understood by Kernux.

This task freezes negotiation semantics only. S05 owns the final generated schema/version representation and global compatibility rules.

Unknown or malformed required protocol-contract metadata fails closed.

## 4. Capability advertisement

Each advertised capability is provider-neutral and binds at least:

~~~text
RuntimeCapability {
  action
  capability_version
  optional_features[]
}
~~~

The `action` is an exact action from `CAPABILITY_AND_GRANT_MODEL.md` or an explicitly registered/negotiated extension action.

The runtime does not advertise wildcards such as:

~~~text
files.*
all
admin
everything
~~~

unless a later explicitly versioned contract defines a separate non-authorizing discovery abstraction.

### 4.1 Advertisement is not authority

Advertisement means only:

> this runtime claims it can participate in this capability contract.

It does not mean:

- the current subject is authorized;
- a Grant exists;
- remote-host policy allows the operation;
- the requested resource is valid;
- the operation is currently healthy;
- the controller may bypass consequence/provenance policy.

Effective authority remains the S02 intersection.

## 5. Negotiation

Before using a capability, controller and runtime establish a negotiated capability set.

Conceptually:

~~~text
negotiated =
  controller_understands
  INTERSECT runtime_advertises
  INTERSECT protocol_contract_support
~~~

Authorization is evaluated separately and additionally.

Therefore executable authority is conceptually:

~~~text
negotiated capability
AND valid S02 authorization
AND kernel hard constraints
AND runtime/remote-host policy
AND resource-specific trusted resolution
~~~

### 5.1 Unknown and unsupported behavior

The following do not become permissive fallbacks:

- unknown action;
- unknown capability version;
- malformed capability descriptor;
- required feature not advertised;
- protocol contract mismatch;
- runtime type inference;
- provider-specific feature guess.

A mandatory unknown/unsupported item fails closed.

Optional behavior may be omitted only when the caller explicitly allows that omission and the omission cannot change authorization or side-effect semantics.

### 5.2 No provider inference

Examples of invalid inference:

- "SSH runtime" therefore shell is available;
- "browser runtime" therefore upload is available;
- "Docker runtime" therefore filesystem write is available;
- provider X therefore cancellation is reliable.

Kernux uses negotiated declarations, not brand/type assumptions.

## 6. Contact state

Contact state describes the controller's current transport relationship with the runtime.

Closed v1 vocabulary:

~~~text
connected
degraded
disconnected
~~~

### 6.1 connected

The controller has a currently usable authenticated channel and recent protocol interaction.

### 6.2 degraded

Communication exists but is impaired, partial, stale, or below the runtime's declared healthy-contact criteria.

Examples may include:

- repeated transport retries;
- partial stream loss;
- unhealthy heartbeat while another channel still responds;
- known capability channel impairment.

Degraded does not imply operation failure.

### 6.3 disconnected

No currently usable authenticated channel is available.

Disconnected says nothing by itself about host-owned execution.

## 7. Execution state

Execution state describes the authoritative known state of one host-owned operation.

Closed v1 vocabulary:

~~~text
live
exited
unverifiable
~~~

### 7.1 live

The execution owner authoritatively reports the operation remains active or has not reached terminal exit.

### 7.2 exited

The execution owner authoritatively reports a terminal exit.

An exited observation includes a provider-neutral termination cause when known, such as:

- completed;
- failed;
- cancelled;
- terminated;
- killed;
- rejected-before-start.

The exact generated enum belongs to S05.

### 7.3 unverifiable

Kernux cannot currently establish authoritative live-or-exited truth.

Common causes:

- controller lost contact;
- remote runtime rebooted before reconciliation;
- transport returned an ambiguous result;
- observation is stale and no authoritative refresh is available.

Unverifiable is not success and not failure.

## 8. Contact and execution are independent

Valid combinations include:

| Contact | Execution | Meaning |
| --- | --- | --- |
| connected | live | runtime reachable and operation authoritatively active |
| connected | exited | runtime reachable and terminal truth known |
| connected | unverifiable | transport works but this operation cannot yet be authoritatively resolved |
| degraded | live | impaired contact, but live truth is still authoritative |
| degraded | exited | impaired contact, but terminal truth is known |
| degraded | unverifiable | impaired contact and operation truth unresolved |
| disconnected | live | last authoritative observation was live; contact is now gone |
| disconnected | exited | terminal truth was established before contact loss |
| disconnected | unverifiable | contact loss prevents authoritative resolution |

### 8.1 Forbidden inference

This transition is forbidden:

~~~text
contact: disconnected
therefore
execution: exited
~~~

Likewise:

- heartbeat timeout does not imply exited;
- controller restart does not imply exited;
- socket close does not imply exited;
- cancellation request does not imply exited;
- provider CLI disappearance from controller memory does not imply exited.

## 9. Authoritative observation

The execution owner emits or returns an observation with enough trusted ordering metadata to prevent stale overwrite.

Conceptually:

~~~text
OperationObservation {
  operation_id
  runtime_id
  observation_generation
  execution_state
  termination_cause?
  observed_at
}
~~~

### 9.1 Observation generation

`observation_generation` is monotonic for observations of one OperationId within the runtime truth domain.

It is ordering metadata, not a canonical entity ID.

A controller MUST NOT replace a newer generation with an older one.

Equal-generation contradictory observations are a protocol invariant violation and fail closed.

### 9.2 Controller cache

The controller may cache observations for UX and recovery.

Cached observations are not more authoritative than runtime-owned newer truth.

A stale cache can explain what was last known, but cannot manufacture terminal state.

## 10. RequestId

RequestId identifies one protocol request attempt.

Rules:

- canonical UUIDv7 textual/validation rules apply;
- each new request attempt receives a new RequestId;
- RequestId is not a secret;
- RequestId is not authority;
- retrying an attempt normally uses a new RequestId;
- RequestId does not identify the logical side effect.

Examples include:

- runtime describe;
- operation start attempt;
- operation observe;
- cancellation request;
- capability negotiation request.

## 11. OperationId

OperationId identifies one logical side-effecting operation.

Rules:

- canonical UUIDv7 textual/validation rules apply;
- OperationId is non-secret;
- OperationId is not a Grant;
- transport retries for the same logical operation keep the same OperationId;
- reconnect/reconciliation uses the same OperationId;
- a fresh intended side effect gets a fresh OperationId;
- OperationId is not reused after final archival for a different operation.

OperationId is a protocol-operation identity, not a new P01-S01 core entity type.

## 12. Operation fingerprint

Every side-effecting OperationId is bound to one canonical operation fingerprint.

The fingerprint covers at least:

- canonical subject scope;
- exact action;
- canonical primary resource;
- exact Runtime ID;
- effective S02 constraints;
- semantic payload identity;
- protocol/capability contract version relevant to interpretation.

### 12.1 Semantic payload identity

Payload identity uses deterministic canonical semantics.

When payload bytes or a canonical serialized request determine the side effect, a SHA-256 digest may bind them.

The fingerprint must not depend on:

- map/object insertion order;
- whitespace that is semantically irrelevant;
- provider-native request ID;
- volatile transport headers;
- human display text.

S05 owns the exact generated canonical serialization.

### 12.2 Fingerprint conflict

If a runtime has already seen:

~~~text
OperationId = O
Fingerprint = F1
~~~

then a later start with:

~~~text
OperationId = O
Fingerprint = F2
F2 != F1
~~~

is `operation_conflict`.

The runtime MUST NOT:

- execute F2;
- reinterpret O;
- overwrite the original fingerprint;
- treat it as a normal retry.

## 13. Start admission

A side-effecting start is admitted only after:

1. Runtime identity and negotiated capability are valid.
2. S02 authorization is valid for the exact subject/action/resource/runtime.
3. Resource-specific trusted resolution succeeds.
4. OperationId is syntactically valid.
5. Canonical fingerprint is computed.
6. Existing-operation lookup is performed.
7. Fingerprint conflict is rejected.
8. Applicable Grant use accounting/admission occurs.
9. Runtime policy permits execution.
10. Durable/recoverable operation truth is established to the degree the runtime claims before irreversible side effects begin.

This model does not require one physical database transaction across every future adapter; it requires equivalent fail-closed semantics.

## 14. Duplicate start and replay

A repeated start with the same OperationId and same fingerprint represents the same logical operation.

The execution owner returns or reconciles:

- existing live operation;
- existing exited result;
- existing unverifiable/recovery state;

rather than spawning another independent side effect.

### 14.1 Deduplication envelope

A runtime that claims reconnect/restart resumability must retain enough trusted operation identity/fingerprint/status data across that claimed envelope to prevent duplicate side effects.

A runtime unable to retain such state must declare the limitation and cannot pretend exactly-once behavior.

### 14.2 Replay protection

Remote transport replay protection and OperationId deduplication are complementary.

Cryptographic transport replay defense does not replace logical operation deduplication.

Logical deduplication does not replace authenticated transport replay defense.

## 15. Ambiguous start result

A request can fail after the operation may already have started.

Examples:

- transport closes after host admission but before acknowledgement;
- controller times out while host continues;
- remote response is lost;
- controller crashes after sending the start.

The canonical result is not automatically "failed to start."

Instead, side-effect truth is classified for retry/reconciliation.

### 15.1 Reconcile first

If the prior OperationId may have started:

1. preserve the same OperationId;
2. query/reconnect/reconcile that OperationId;
3. use authoritative runtime truth when available;
4. do not create a fresh OperationId for the same intended side effect until the prior one is proven definitely not started or a higher-level recovery policy intentionally accepts duplication risk.

Blind retry under a new OperationId is forbidden.

## 16. Runtime-owned operation truth

For process, PTY, browser action, host mutation, sandbox lifecycle, and other runtime-owned effects, the execution host owns operation lifecycle truth.

The controller owns:

- requests;
- orchestration intent;
- cached observations;
- UI state.

The runtime owns:

- whether admitted execution actually started;
- current host-side live/exited truth;
- terminal cause;
- deduplication record within its declared persistence envelope.

Provider adapters translate provider state into this Kernux contract but do not invent certainty the provider cannot supply.

## 17. Reconciliation

Reconciliation asks the execution owner for current truth about an existing OperationId.

Possible outcomes include:

- authoritative live;
- authoritative exited;
- authoritative known-but-currently-unverifiable;
- unknown OperationId;
- runtime unreachable.

### 17.1 Unknown OperationId

Unknown OperationId is not automatically proof that the side effect never happened.

The interpretation depends on the runtime's declared deduplication/persistence envelope.

If the runtime guarantees durable operation lookup across the relevant period and identity continuity is intact, unknown may support a definitely-not-started conclusion only when that guarantee explicitly says so.

Otherwise unknown remains uncertain.

### 17.2 Reconnect reconciliation

After reconnect:

- refresh Runtime identity/revision;
- renegotiate protocol/capabilities;
- re-evaluate current authorization for any new operation admission;
- reconcile outstanding OperationIds;
- reject stale controller observations that conflict with newer runtime generations.

Reconnect is not implicit resume authorization.


> Implementation note: this contract is landing in bounded R3 slices. Cancellation, structured-error, retry-guidance, reconnect/revocation, adversarial, and downstream sections remain incomplete until the successor slice merges.
