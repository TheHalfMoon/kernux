# KRP Compatibility and Versioning

## Status

This document defines the canonical compatibility contract for the Kernux Runtime Protocol (KRP) v1 core.

It is subordinate to the already-proven identity, capability/Grant, runtime-operation, and Event/Evidence semantics. Compatibility never creates authority and never overrides runtime truth.

Current generated wire source:

`protocol/schema/krp.v1.schema.json`

Current protocol compatibility family:

`krp/1`

Current generated capability version token:

`1`

## 1. Three identities that must not be conflated

KRP uses three different identities for different purposes.

### 1.1 Protocol contract

`krp/1` identifies the semantic compatibility family understood by the current generated core contract.

A peer that does not support the advertised protocol contract cannot interpret or admit the affected KRP operation through that contract.

### 1.2 Capability version

Capability versions are exact provider-neutral versions attached to exact actions.

The current generated core recognizes capability version `1`.

Capability version is negotiated independently from application version, runtime implementation version, provider name, or transport.

### 1.3 Schema SHA-256

The schema SHA-256 binds exact source bytes to generated Rust/TypeScript artifacts, fixtures, evidence, and drift checks.

It is provenance identity, not a negotiated wire version.

A schema-digest mismatch alone is neither proof of compatibility nor proof of incompatibility. Two schema byte revisions may preserve the same `krp/1` semantic family, while an apparently small semantic change may require a new compatibility contract.

Compatibility is decided from protocol-contract support, exact capability/feature negotiation, and successful validation under the receiving implementation.

## 2. Closed-wire rule

KRP v1 core objects are closed.

The authoritative schema uses `additionalProperties=false`, and generated Rust object types use `serde(deny_unknown_fields)`.

Therefore:

- an unknown wire field is rejected;
- a receiver must not strip an unknown field and continue;
- a partial decode is not evidence of compatibility;
- an unrecognized enum/version token is rejected;
- a malformed required field is rejected.

Rust and Node/schema behavior must remain equivalent on these rules.

## 3. Required and optional fields

A missing required field fails closed.

A missing known optional field means only that the value is absent.

Absence must not synthesize:

- authorization;
- feature support;
- successful execution;
- terminal execution;
- cancellation success;
- `definitely_not_started`;
- retry safety;
- a broader resource scope;
- a weaker consequence class.

If a safe default is ever required, that default must be explicitly versioned into the semantic contract rather than inferred by a decoder.

## 4. Evolution classes

Every proposed KRP schema change is classified before merge.

| Change class | Same `krp/1` family? | Required treatment |
| --- | --- | --- |
| Editorial only | Yes | No wire behavior changes. Regenerate and rebind exact schema digest. |
| Additive optional, semantics-neutral | Conditionally | Requires an explicit feature token and sender suppression for unsupported peers. |
| New enum/discriminator meaning | Not silently | Requires explicit version/compatibility design because existing closed decoders reject unknown values. |
| New required field | No silent compatibility | Requires a new contract/version path. |
| Field removal, type change, or meaning change | No | Requires a new contract/version path and migration policy. |
| Change that can widen authority or alter side-effect meaning | No additive treatment | Requires explicit semantic versioning and security review. |

“Optional” describes wire presence only. It does not make a semantic change automatically backward compatible.

## 5. Additive optional fields

An additive optional field may remain inside `krp/1` only when all of the following are true:

1. omission preserves the prior meaning;
2. the field cannot widen authority;
3. the field cannot change side-effect identity, certainty, cancellation, or retry semantics;
4. the field has an explicit provider-neutral feature token;
5. both sender and receiver advertise support for that feature;
6. the sender omits the field when the peer did not advertise support.

The effective sender rule is intersection:

```text
emit additive field =
  sender_supports(feature)
  AND peer_advertises(feature)
```

If an unsupported peer receives the field anyway, closed-object validation rejects it.

KRP does not use “ignore unknown fields” as a forward-compatibility strategy.

The compatibility matrix uses `operation.trace_context` / `trace_context` only as a policy probe for a future additive field. It does not add that field to the current wire schema.

## 6. Protocol negotiation

The current generated protocol contract recognizes exactly `krp/1`.

Negotiation must not infer compatibility from:

- desktop version;
- `kernuxd` version;
- operating system;
- provider/runtime type;
- transport;
- schema digest alone.

No implicit downgrade or floating range negotiation exists in v1.

A future `krp/2` requires a separately governed contract, migration/interop rules, and conformance evidence before it becomes supported.

## 7. Capability negotiation

Capability support is exact by action + capability version + negotiated features.

The current generated `CapabilityVersion` has one value: `1`.

An unknown action may be structurally discoverable where the containing field grammar allows it, but it cannot satisfy a requested known capability and never becomes authority by presence alone.

An unknown capability version cannot fall back to version `1`.

Adding a future capability version requires an explicit compatibility design. If an older peer cannot parse that version under its closed contract, rejection is the correct fail-closed result.

Authorization remains a separate intersection after compatibility negotiation.

## 8. Protocol mismatch does not rewrite runtime truth

A protocol mismatch is an admission/interpretation failure for the affected protocol exchange.

It is not evidence that:

- the runtime disconnected;
- a process exited;
- cancellation succeeded;
- an operation never started;
- a side effect is retry-safe.

Contact state and execution state remain independent authoritative observations.

If an operation was already `unverifiable` with `may_have_started`, discovering a protocol mismatch does not rewrite those facts to `exited` or `definitely_not_started`.

Existing reconcile-before-retry rules continue to apply.

## 9. Event compatibility

An Event must use a supported `contract_version`. The current generated value is `krp/1`; an unsupported contract version is rejected by the current schema.

`event_type` is a provider-neutral extensible identifier rather than a closed enum.

Therefore a future event type can be structurally valid while still being semantically unknown.

Projection rule:

```text
privileged projection eligibility =
  supported contract_version
  AND known/negotiated event_type interpreter
```

An unknown or unnegotiated event type must not mutate a privileged canonical projection.

A later durable store may preserve such an Event as opaque data only when storage/import policy explicitly permits that retention. Opaque retention is non-interpreting and confers no authority.

No persistent Event store is implemented by this task.

## 10. Compatibility evidence

The focused T02 matrix is:

`protocol/fixtures/v1/compatibility.json`

The executable checker is:

`tools/protocol/compatibility.mjs`

The normal `pnpm check` protocol gate executes it after generator-drift and shared-core conformance checks.

The focused matrix proves:

- exact v1 acceptance;
- unknown-field rejection;
- unsupported protocol/capability/Event-version rejection;
- required-field rejection;
- known optional-field omission without semantic defaulting;
- exact capability action/version admission with unknown-action non-fallback;
- protocol mismatch without execution/side-effect truth rewriting;
- negotiated additive-field sender gating;
- unnegotiated Event types remain non-projecting.

Rust tests independently prove strict generated decoding and optional-absence behavior through:

`crates/kernux-contracts/tests/compatibility.rs`

These are focused compatibility proofs, not the full adversarial protocol corpus.

## 11. S05-T03 adversarial evidence

KX-P01-S05-T03 owns the full negative/adversarial protocol corpus.

The canonical shared corpus is:

`protocol/fixtures/v1/adversarial.json`

The normal Node protocol gate executes:

`tools/protocol/adversarial.mjs`

Rust independently consumes the same tracked adversarial fixture bytes through:

`crates/kernux-contracts/tests/adversarial.rs`

Together these executable checks cover:

- all twenty adversarial invariants in `RUNTIME_OPERATION_MODEL.md` section 28;
- disconnect, timeout, cancellation, restart/cache-loss, and observation-ordering truth preservation;
- duplicate/replayed OperationId plus canonical request-fingerprint behavior;
- side-effect ambiguity and reconcile/no-blind-retry discipline;
- authority/capability intersection and trust-identity continuity;
- malformed unknown/missing/nested/closed-enum payloads;
- RuntimeDescriptor, RuntimeCapability, OperationStart, and Event version mismatch;
- the explicit boundary where JSON Schema string-pattern checks are stricter than generated Rust serde.

This evidence proves contract and policy-fixture conformance only. It does not prove exactly-once runtime execution, durable replay protection, cancellation delivery, reconnect persistence, or live runtime transport behavior.

T02 remains the focused compatibility-policy proof. T03 supplies the broader adversarial corpus without changing the frozen KRP v1 compatibility rules.

## 12. Change rule

A future KRP compatibility change must answer, before implementation:

1. Is this editorial, additive-optional, or semantic/breaking?
2. Which protocol contract and capability version interpret it?
3. Is a feature token required?
4. What does an older peer receive?
5. Does omission preserve authorization and side-effect meaning?
6. Does any unknown value fail closed?
7. Can the change alter durable Event interpretation or projection?
8. What exact cross-language fixtures prove the decision?

If those answers are not explicit, the change is not ready to enter the canonical wire contract.
