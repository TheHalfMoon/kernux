# Canonical Identity and Revision Model

## 1. Purpose

Kernux needs durable identity semantics before later phases define capability resources, runtime protocols, event envelopes, persistence schemas, or generated cross-language contracts.

This document freezes the v1 conceptual identity and revision rules for exactly these core entities:

- Project;
- Task;
- WorkUnit;
- Run;
- AgentSession;
- Runtime;
- Artifact;
- Evidence;
- Event.

It is intentionally narrower than the full P01 protocol spine. CapabilityRequest, Grant, resource-URI syntax, operation IDs, runtime negotiation, event-envelope ordering fields, schema-source selection, generated Rust/TypeScript types, and database representation belong to later dependency-ordered P01 tasks.

## 2. Identity classes are not interchangeable

Kernux distinguishes five concepts that MUST NOT be substituted for one another:

1. **Canonical entity ID** — stable Kernux identity for one logical entity.
2. **Entity revision** — version of canonical state for a revisioned entity.
3. **Content digest** — identity of immutable bytes, not of a logical entity.
4. **Foreign/native ID** — identifier owned by a provider, runtime, operating system, repository, model service, protocol peer, or other external authority.
5. **Ordering coordinate** — explicit sequence, timestamp, causal, or stream-order metadata defined by the contract that owns ordering.

A value being unique or time-shaped does not make it interchangeable with another identity class.

## 3. Canonical entity IDs

### 3.1 UUIDv7

Every v1 canonical ID for the nine entities in this document MUST be an RFC 9562 UUID Version 7.

Kernux uses UUIDv7 because it is globally usable across local and remote creators without a central integer allocator while retaining practical locality for storage and indexing. UUIDv7's timestamp field is an implementation property of the identifier, not a business or authorization field.

The normative UUID standard is RFC 9562, Universally Unique IDentifiers (UUIDs), including Section 5.7 for UUIDv7, Section 4.1 for the variant, and Section 4.2 for the version field.

### 3.2 Canonical textual form

The only canonical Kernux textual form is:

~~~text
xxxxxxxx-xxxx-7xxx-[89ab]xxx-xxxxxxxxxxxx
~~~

where every x is a lowercase hexadecimal digit.

Canonical-boundary validation MUST reject rather than silently normalize:

- uppercase hexadecimal;
- braces;
- urn:uuid prefixes;
- missing or extra hyphens;
- non-hexadecimal characters;
- a UUID version other than 7;
- a UUID variant other than RFC 9562 10xx;
- the Nil UUID;
- trailing or leading whitespace;
- alternate compact or base-N encodings presented as a canonical Kernux ID.

An import or adapter boundary MAY accept an external format only if that boundary explicitly owns normalization and then creates or resolves the corresponding canonical Kernux identity.

### 3.3 Opaque and non-secret

Canonical IDs are opaque identifiers.

They MUST NOT encode or be interpreted as:

- provider identity;
- entity kind;
- tenant or organization authority;
- filesystem path;
- repository identity;
- permission level;
- capability scope;
- secret material;
- user identity;
- trust level;
- business semantics.

Possession or knowledge of a canonical ID grants no authority.

The UUIDv7 timestamp MUST NOT be used as:

- authorization evidence;
- freshness proof;
- causal ordering;
- event ordering;
- replay ordering;
- security epoch;
- retention authority;
- a substitute for an explicit timestamp or sequence owned by another contract.

Clock skew, clock rollback, generator monotonicity, or lexicographic UUID ordering therefore MUST NOT grant or remove privilege or determine event truth.

## 4. Entity-specific identity rules

| Entity | Canonical ID | Revision model | Identity rule |
| --- | --- | --- | --- |
| Project | UUIDv7 | revisioned | One durable Kernux work context. Moving or renaming a folder does not by itself create a new Project ID. |
| Task | UUIDv7 | revisioned | One user outcome. Accepted plan or outcome metadata mutations advance revision. |
| WorkUnit | UUIDv7 | revisioned | One bounded unit of planned work. Scope, dependency, or acceptance changes advance revision. |
| Run | UUIDv7 | immutable, revision = 1 | One execution occurrence. Every rerun is a new Run ID even when inputs are otherwise identical. |
| AgentSession | UUIDv7 | immutable, revision = 1 | One logical agent execution or conversation continuity. Mutable live status is event or projection state, not entity revision. |
| Runtime | UUIDv7 | revisioned | One enrolled trust identity for an execution host. Reconnect or restart can retain the ID only while enrollment identity is the same. |
| Artifact | UUIDv7 | revisioned metadata, immutable bytes | One logical immutable-content artifact. Metadata may revise under bounded rules; bytes and digest may not. |
| Evidence | UUIDv7 | immutable, revision = 1 | One assertion bound to observed inputs and results. A changed assertion is new Evidence. |
| Event | UUIDv7 | immutable, revision = 1 | One append-oriented fact. Correction or supersession is another Event, never in-place mutation. |

Revision 1 on immutable entities is semantic and explicit even if a later storage schema optimizes its physical representation.

## 5. Revision semantics

### 5.1 Domain

A revision is a positive unsigned 32-bit semantic integer:

~~~text
1 <= revision <= 4294967295
~~~

Rules:

- initial canonical revision is 1;
- 0 is invalid;
- negative values are invalid;
- fractional values are invalid;
- values above 4294967295 are invalid;
- wraparound is forbidden;
- the domain remains exactly representable by JSON numbers.

### 5.2 Accepted mutation

For a revisioned entity, each accepted canonical mutation MUST advance the revision by exactly one.

If canonical state is at revision n, an accepted mutation produces n + 1.

The following do not advance revision:

- rejected validation;
- rejected authorization;
- failed persistence before canonical commit;
- stale expected revision;
- duplicate or idempotently recognized submission that changes no canonical state;
- projection, cache, or index rebuild;
- external or provider status observed only as a new Event or projection input.

A system MUST NOT skip revisions merely to encode time, retries, or internal processing.

### 5.3 Optimistic compare-and-swap

A mutation of a revisioned entity MUST name the expected current revision.

The canonical mutation succeeds only when:

~~~text
expected_revision == current_revision
~~~

Otherwise the mutation fails with an explicit revision conflict and MUST NOT silently overwrite the current state.

A conflict result SHOULD expose enough non-secret metadata for the caller to re-read and reconcile, but later runtime and error contracts own the exact error envelope.

### 5.4 Overflow

If current revision is 4294967295, a further canonical mutation MUST fail closed.

Revision numbers MUST NOT wrap to zero or one, widen implicitly, or cause a replacement entity to masquerade as the same identity. Any future need to change the revision domain requires an explicit compatibility and migration decision.

## 6. Exact references and locators

Kernux distinguishes an **exact reference** from a **locator**.

For a revisioned entity, an exact reference conceptually binds:

~~~text
(entity_id, revision)
~~~

For an immutable entity, its canonical ID identifies the exact immutable record and its semantic revision is 1.

Durable execution, authorization, replay, lineage, and evidence MUST bind exact references when the target entity is revisioned.

An ID-only or latest locator MAY be used for:

- navigation;
- lookup;
- user-interface selection;
- discovery;
- a request to resolve current state before making a separately authorized action.

An ID-only or latest locator MUST NOT itself be accepted as authoritative evidence of the exact state used for:

- privileged execution;
- policy authorization;
- replay;
- verification;
- evidence binding;
- reproducibility claims.

Resolution of a locator to an exact reference is an observable state decision and later event or evidence contracts SHOULD preserve the resolved revision where consequential.

## 7. Foreign and provider-native identifiers

External identifiers remain foreign metadata.

Examples include:

- provider conversation or session IDs;
- model IDs and revisions;
- operating-system process IDs;
- container IDs;
- SSH host keys or fingerprints;
- Git commit, tree, and blob IDs;
- GitHub issue or pull-request IDs;
- filesystem paths;
- browser profile IDs;
- protocol peer IDs;
- external task IDs.

A foreign ID MUST NOT replace a canonical Kernux ID.

Mappings between Kernux and foreign identities MUST state the owning namespace, provider, or authority and MUST be treated as integration metadata rather than core entity identity.

Provider-specific identifiers MUST NOT appear as provider-specific mandatory fields in the core identity contract.

## 8. Artifact identity and content digest

Artifact has two independent identity layers:

1. Artifact ID — UUIDv7 logical Kernux entity identity.
2. Content digest — immutable byte identity, SHA-256 for the first-generation artifact CAS.

The byte content associated with one Artifact ID MUST NOT change.

Allowed Artifact metadata revisions may update bounded metadata that does not alter bytes, such as:

- retention class;
- pin state;
- user-visible semantic labels;
- redaction or availability metadata when the underlying byte identity remains unchanged;
- other later-defined metadata explicitly declared safe to revise.

The SHA-256 digest MUST remain invariant across all revisions of one Artifact ID.

If bytes change, the system MUST create a new Artifact ID and new digest.

Deduplicated storage MAY cause multiple Artifact IDs to reference the same content digest. Sharing a digest does not merge their logical identity, provenance, retention, producer, or authorization metadata.

Evidence citing an Artifact MUST bind:

- Artifact ID;
- exact Artifact revision when revisioned metadata matters;
- content digest when byte identity matters.

## 9. Occurrence and continuity rules

### 9.1 Run

A Run is one execution occurrence.

Retrying, rerunning, replaying, or starting another candidate execution creates a new Run ID. Similar or identical inputs do not justify identity reuse.

Relationship to an earlier run is lineage metadata owned by later contracts.

### 9.2 AgentSession

An AgentSession ID may survive reconnect or resume only when Kernux establishes continuity of the same logical session.

Examples that may preserve the ID once later adapter contracts prove continuity:

- transport reconnect to the same resumable provider session;
- renderer restart while the same supervised agent session continues;
- controller reconnection to the same session state.

A new AgentSession ID is required when continuity is not established, including a fresh provider conversation or a replacement session after unrecoverable identity ambiguity.

Provider-native session IDs remain foreign metadata and do not become AgentSession IDs.

### 9.3 Runtime

Runtime identity belongs to an enrolled trust identity, not to a transient socket or process.

A reconnect or host restart may retain Runtime ID when the same enrollment or trust identity is cryptographically or otherwise authoritatively re-established by the later runtime and security contract.

Reinstallation, reenrollment, identity-key replacement without an authorized continuity procedure, or ambiguous host replacement requires a new Runtime ID.

## 10. Evidence and immutable facts

Evidence is an immutable assertion. If its claim, bindings, observed result, or qualifying inputs change, a new Evidence ID is required.

Event is an immutable append-oriented fact. Corrections are represented by later facts, not mutation of the original Event.

Run, AgentSession, Evidence, and Event use semantic revision 1. Changing live projections around them does not mutate those immutable identity records.

[`EVENT_AND_EVIDENCE_MODEL.md`](EVENT_AND_EVIDENCE_MODEL.md) defines the v1 event-envelope ordering coordinates. Event ID or UUIDv7 lexical order is not sufficient ordering authority.

## 11. Security and privacy properties

Canonical IDs are safe to log only under the data-class and telemetry policy that owns the surrounding record. Not secret does not mean always non-sensitive.

Security rules:

- do not use ID possession as authentication;
- do not infer trust from UUID timestamp;
- do not infer tenancy or ownership from UUID structure;
- do not use ID order to authorize replay or resolve conflicts;
- do not accept foreign IDs in canonical-ID fields;
- do not silently coerce malformed canonical IDs;
- do not permit revision conflicts to become last-write-wins unless a future bounded contract explicitly defines that behavior.

## 12. Compatibility boundary

This document freezes conceptual v1 semantics, not the generated wire representation.

The following remain intentionally undecided here:

- schema source or language;
- code-generation toolchain;
- Rust or TypeScript branded and newtype representation;
- binary versus JSON or CBOR encoding;
- database column and storage layout;
- capability resource URI format;
- operation ID semantics;
- event stream and sequence fields;
- full event and evidence envelope;
- mixed-version wire compatibility rules.

Those decisions must preserve this identity and revision model unless replaced by an explicit ADR with migration impact.

## 13. Conformance examples

Valid canonical textual shape:

~~~text
01890f3a-7b2c-7d45-8a61-3c4e5f607182
~~~

The example is illustrative. Conformance tooling must validate actual UUIDv7 version and variant bits rather than matching only a surface regex.

Invalid canonical forms include:

~~~text
01890F3A-7B2C-7D45-8A61-3C4E5F607182
{01890f3a-7b2c-7d45-8a61-3c4e5f607182}
urn:uuid:01890f3a-7b2c-7d45-8a61-3c4e5f607182
01890f3a7b2c7d458a613c4e5f607182
01890f3a-7b2c-4d45-8a61-3c4e5f607182
~~~

The last value has the wrong UUID version even though its shape is otherwise UUID-like.

## 14. Downstream requirements

Later P01 work must preserve these invariants:

- capability and resource references bind canonical identity rather than provider identity;
- runtime operations distinguish operation identity from entity identity;
- event envelopes provide explicit ordering rather than relying on UUID sorting;
- generated contracts reject malformed or noncanonical IDs and invalid revisions;
- compatibility fixtures cover stale revision, overflow, foreign-ID confusion, artifact byte mutation, rerun identity, and session or runtime continuity boundaries;
- persisted evidence binds exact revisions and artifact digests required to reproduce its claim.

No later contract may weaken these rules silently.
