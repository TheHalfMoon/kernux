# Event and Evidence Model

## 1. Purpose

Kernux needs durable facts that survive reconnects, retries, projection rebuilds, exports, and independent review without turning transport order, wall-clock time, model narration, or mutable UI state into truth.

This document freezes the conceptual v1 semantics for Event envelopes, stream ordering, correlation, lineage, Artifact references, Evidence assertions, evidence independence, redaction/exclusion metadata, and projection integrity. It defines meaning, not final wire encoding or storage layout.

## 2. Core invariants

1. Event is an immutable UUIDv7 revision-1 fact.
2. Evidence is an immutable UUIDv7 revision-1 assertion.
3. Event UUID lexical/time order is never canonical event order.
4. Producer and controller timestamps are not stream-order authority.
5. Every Event belongs to exactly one canonical owner stream.
6. Per-stream order is explicit through stream_seq and previous_event_id.
7. Accepted sequence is contiguous; gaps, duplicate positions, and wrong predecessors fail closed.
8. Corrections and supersessions append new Events; history is never edited in place.
9. Causal lineage is explicit and acyclic but does not replace stream ordering.
10. Artifact byte integrity requires SHA-256 whenever bytes are relied upon.
11. Agent/model self-report is evidence context, not independent proof.
12. Redaction/exclusion is explicit metadata, never silent omission from a view that claims completeness.
13. Transformed/redacted bytes are a new Artifact identity and digest.
14. Ordinary Event/Evidence records do not persist secret plaintext.
15. Canonical projections are rebuildable from accepted Events.
16. Correlation identifiers are context, not authority tokens.

## 3. Event identity

Event identity follows IDENTITY_AND_REVISION_MODEL.md: RFC 9562 UUIDv7, lowercase canonical text, semantic revision 1, immutable record. A changed fact receives a new Event ID.

Event IDs are stable references, not ordering coordinates. UUID lexical order MUST NOT determine canonical stream order.

## 4. Event type grammar

A v1 Event type is a provider-neutral lowercase dotted identifier. Each token matches:

~~~text
[a-z][a-z0-9_]*
~~~

It contains at least two tokens. Examples include:

~~~text
task.created
capability.requested
runtime.contact_lost
operation.started
operation.observed
operation.cancel_requested
operation.exited
artifact.created
evidence.recorded
run.completed
~~~

Provider-native names may be retained as foreign diagnostics but are not canonical event types.

Unknown/unnegotiated event types MUST NOT silently mutate a privileged canonical projection. Later compatibility rules may permit opaque retention without implying understood semantics.

## 5. Stream identity

A stream is an ordering scope, not a new core entity.

One Event belongs to exactly one typed owner stream:

~~~text
StreamRef {
  owner_kind
  owner_id
  owner_revision?
}
~~~

Core v1 owner kinds are project, task, run, and runtime.

WorkUnit and other entity references can appear as exact correlations without creating independent stream kinds in v1.

One Event cannot occupy two canonical stream positions. Multiple views index the same immutable Event by correlation instead.

## 6. Stream sequence

Each Event has a positive unsigned 64-bit conceptual stream_seq:

~~~text
1 <= stream_seq <= 18446744073709551615
~~~

Rules:

- first accepted Event in a stream is sequence 1;
- each accepted successor increments exactly one;
- zero is invalid;
- overflow fails closed;
- positions are never reused for another Event;
- durable canonical append authority owns assignment.

Transport arrival order is not canonical stream order.

## 7. Previous Event linkage

Each Event after sequence 1 binds previous_event_id to the immediately prior Event in the same stream.

~~~text
seq=1 -> previous_event_id absent
seq=n>1 -> previous_event_id == event_id(seq=n-1)
~~~

The predecessor adds an inspectable continuity check in addition to numeric sequence.

## 8. Append validation

Canonical append rejects:

- duplicate Event ID with conflicting fact;
- sequence zero;
- duplicate stream position for a different Event;
- sequence gap or backward move;
- sequence 1 with a predecessor;
- sequence greater than 1 without a predecessor;
- wrong immediate predecessor;
- predecessor from another stream;
- malformed or ambiguous stream owner;
- malformed event type;
- conflicting immutable content under an accepted Event ID.

An idempotent append of the exact same immutable Event may resolve to the existing Event, not a new position.

## 9. Event envelope

Conceptually:

~~~text
Event {
  event_id
  event_type
  contract_version
  stream
  stream_seq
  previous_event_id?
  producer
  occurred_at?
  recorded_at
  correlations
  lineage
  payload
  artifact_refs[]
  sensitivity
  redaction
}
~~~

S05 owns generated encoding.

## 10. Producer and time

Producer metadata may bind Kernux component identity, exact Runtime, AgentSession, adapter identity/version, and foreign provider identifier as diagnostic metadata. Producer identity does not establish authorization by itself.

recorded_at is trusted append-side time. occurred_at is optional source occurrence time. Neither is ordering authority. Clock skew, rollback, timezone conversion, delayed delivery, or provider clocks MUST NOT alter stream_seq.

## 11. Correlations

An Event may correlate exact references, where relevant, to Project ID + revision, Task ID + revision, WorkUnit ID + revision, Run ID, AgentSession ID, Runtime ID + revision, Artifact ID + revision, Evidence ID, CapabilityRequest ID, Grant ID, RequestId, and OperationId.

Revisioned entities bind exact revision when the fact depends on that revision.

Correlation is historical context, not reusable authorization.

## 12. Runtime-operation correlation

S03 facts can be represented without changing their meaning:

~~~text
runtime.contact_lost
runtime.contact_restored
operation.start_requested
operation.started
operation.observed
operation.cancel_requested
operation.cancel_accepted
operation.exited
runtime.error_observed
~~~

Events preserve RequestId, OperationId, exact Runtime identity, observation generation where relevant, and S03 state/error semantics.

An Event never converts cancel_accepted into operation.exited.

## 13. Authority and provenance correlation

Security-relevant Events may correlate CapabilityRequest ID, Grant ID, policy revision/reference, source Event IDs, approval decision, consequence class, and trusted action/resource identity.

Those correlations explain what authority was evaluated. They are not authority tokens.

Untrusted source Events cannot grant privilege through lineage.

## 14. Lineage relationships

Lineage is distinct from stream order. Canonical relationship classes include:

~~~text
caused_by
supersedes
forked_from
checkpoint_of
derived_from
~~~

Relations name immutable Event IDs and, where needed, exact Artifact/Evidence references.

## 15. caused_by

caused_by records material causal/provenance inputs and may cross streams. Multiple parents are allowed when truthful.

Causality does not authorize the child action.

## 16. supersedes

A correction is a new Event referencing the prior Event through supersedes.

The prior Event remains immutable. A projection may show the new Event as the effective interpretation while preserving both facts and their original stream positions.

## 17. Fork and checkpoint lineage

Forking creates new lineage, not mutation.

A new Run/work branch may reference exact source Run, checkpoint Event, checkpoint Artifact/digest, and source Task/WorkUnit revision. The destination has its own stream.

## 18. Acyclic lineage

Canonical causal/supersession/fork lineage MUST be acyclic.

An Event cannot cause or supersede itself, be transitively caused by itself, or close a lineage path back to itself. A cycle fails closed for canonical lineage/projection.

Lineage does not provide a total order across streams.

## 19. Artifact reference

Conceptually:

~~~text
ArtifactRef {
  artifact_id
  artifact_revision
  sha256?
  role
  media_type?
  size_bytes?
}
~~~

Artifact identity follows the proven identity model.

Whenever Event/Evidence semantics rely on the bytes, SHA-256 is REQUIRED and verified. Artifact ID alone is not byte-integrity proof.

A digest mismatch is an explicit integrity failure.

Roles may include stdout, stderr, screenshot, diff, report, transcript, input, output, verification_log, source_capture, or redacted_view. S05 owns final encoding.

Media type and size metadata do not replace digest verification.

## 20. Payload placement

The Event envelope remains bounded. Small structured facts may be inline; large or sensitive payloads SHOULD be stored as Artifacts and referenced.

Examples include screenshots, long terminal/model/tool transcripts, full diffs, binary files, downloads, browser captures, and machine-readable evidence reports.

## 21. Secret minimization

Ordinary Event/Evidence records MUST NOT persist secret plaintext.

They may carry opaque secret reference, destination/service class, sensitivity classification, redaction/exclusion state, or policy metadata.

A digest is not automatically non-sensitive; equality/dictionary risk remains policy-bound.

## 22. Evidence identity

Evidence is UUIDv7 semantic revision 1 and immutable.

A new Evidence ID is required if any of these change: claim, subject binding, observed result, qualifying inputs, source/observer class, relevant Artifact bytes/digest, limitation, implementation revision, or runtime/operation context required by the claim.

Evidence is never edited in place to make a gate pass later.

## 23. Evidence record

Conceptually:

~~~text
Evidence {
  evidence_id
  claim
  subject_bindings
  source_class
  observer
  observed_event_ids[]
  artifact_refs[]
  runtime_context?
  operation_id?
  implementation_revision?
  result
  limitations[]
  recorded_by_event_id
  sensitivity
  redaction
}
~~~

S05 owns generated encoding.

## 24. Claim and exact subject

The claim states what was established. Subject bindings identify exact objects/revisions qualified by that claim.

Examples include exact repository head passed a suite, OperationId authoritatively exited with cause, Artifact bytes match SHA-256, policy denied an exact request, or human reviewer approved an exact candidate.

Evidence cannot claim a broader subject than its actual observation basis.

## 25. Evidence source classes

Conceptual classes:

~~~text
machine
external_service
human_review
agent_report
imported
~~~

machine covers deterministic or inspectable test, analyzer, digest, policy, or runtime observations.

external_service covers inspectable protected CI/provider verification.

human_review is a named human decision bound to an exact candidate.

agent_report is model/agent narration or self-report.

imported preserves evidence from another provenance domain and its limitations.

## 26. Evidence independence

A gate specifies required source strength.

agent_report cannot satisfy a gate requiring machine evidence, external-service evidence, independent human review, or independently verified Artifact bytes.

Model text saying tests passed is not a test result. Agent text saying digest matches is not digest verification. Controller prose saying an operation exited is not authoritative runtime observation. Agent text saying review approved is not human review.

Evidence binds the actual observation basis used.

## 27. Evidence observation basis

Evidence may bind exact Event IDs, Artifact refs/digests, Runtime/OperationId, provider/adapter version, check identifier, implementation revision, or human/external reviewer identity.

The basis must be sufficient for the claim it qualifies.

## 28. Evidence result and limitations

Evidence records an explicit result appropriate to the claim.

Limitations capture material boundaries such as platform not tested, remote operation unverifiable, Artifact omitted, source unavailable, excluded files, or imported rather than independently rerun observation.

Limitations are immutable. Removing one requires new Evidence.

## 29. Redaction state

Canonical data and exported/rendered views distinguish:

~~~text
none
excluded
transformed
~~~

none means no redaction transformation in this view; it does not mean non-sensitive.

excluded means content is omitted and metadata records that omission, affected role, reason/class, and policy/version when available. The manifest MUST NOT embed excluded secret plaintext.

transformed means sanitized/redacted content is included. Transformed bytes are a new Artifact with a new ID and SHA-256, optionally linked to source lineage where policy permits.

Canonical source bytes are never mutated by redaction.

## 30. Redaction metadata

Conceptually:

~~~text
Redaction {
  state
  affected_roles[]
  reason_class?
  policy_ref?
  policy_version?
  source_artifact_ref?
  transformed_artifact_ref?
}
~~~

Redaction metadata does not claim every secret was detected.

## 31. Canonical history versus views

Canonical Event/Evidence records are immutable history. Timelines, support bundles, exports, sync selections, and evidence bundles are views.

A view may filter, exclude, or transform. It may reorder presentation only while preserving canonical sequence metadata.

A view may not mutate canonical history, renumber canonical sequence, silently conceal exclusion while claiming completeness, or place transformed bytes under the source Artifact identity.

## 32. Projection rebuild

A canonical projection rebuild:

1. partitions by canonical stream;
2. validates Event identity uniqueness;
3. validates owner stream;
4. reads trusted stream_seq;
5. validates contiguous sequence;
6. validates predecessor linkage;
7. validates event types required by the projection;
8. validates required lineage/Artifact references;
9. applies deterministic projection rules.

A projection whose required history cannot be validated is incomplete/unverifiable, not silently repaired.

## 33. Sequence integrity

A stream with positions 1, 2, 4 has a gap and cannot treat 4 as the direct successor of 2.

Two different Event IDs at one sequence conflict.

A stale transport delivery can arrive later but cannot change its canonical assigned position.

## 34. Cross-stream chronology

Separate streams have no implicit total order.

Run A sequence 10 is not globally before or after Runtime B sequence 3.

Cross-stream reasoning uses explicit causal references and temporal observations where useful. UUID or wall-clock sorting does not establish total truth.

## 35. Correction semantics

Incorrect observations are corrected append-only:

~~~text
E17 provider.status_observed value=X
E23 provider.status_corrected supersedes=E17 value=Y
~~~

E17 remains immutable. E23 does not rewrite E17 sequence.

## 36. Artifact integrity for Evidence

Before Evidence qualifies Artifact bytes:

1. resolve Artifact identity;
2. resolve required exact metadata revision;
3. read approved bytes;
4. compute SHA-256;
5. compare with bound digest;
6. fail on mismatch.

A missing required Artifact makes byte qualification unavailable/unverifiable, not silently ignored.

## 37. Exact implementation qualification

Repository evidence binds the exact implementation revision it qualifies. A moved head does not inherit proof unless the gate proves the delta irrelevant.

Runtime evidence binds Run/Runtime/Operation context needed to identify the occurrence.

## 38. Evidence bundle semantics

A bundle may contain exact Task/WorkUnit revisions, Run identity, selected Event manifest with stream metadata, approval/policy decisions, Evidence records, Artifact manifest/bytes, runtime/provider/adapter context, provenance, warnings/limitations, and exclusion/transformation manifest.

Bundle creation is a view and does not mutate canonical history.

## 39. Export exclusions

If policy omits secret material, authenticated browser state, sensitive screenshots, unavailable remote Artifacts, or organization-managed content, the bundle records exclusion by role/reason/policy context without embedding omitted sensitive bytes.

Consumers can distinguish absent, excluded, and transformed content.

## 40. Lifecycle interaction

Event/Evidence retention and Artifact retention are related but distinct.

Evidence-referenced Artifact blobs remain protected from garbage collection while policy says Evidence requires them.

If policy later permits payload deletion, Evidence must expose that byte re-verification is unavailable; it cannot fabricate successful digest verification.

## 41. Replay semantics

Replaying Events means rebuilding interpretation/projection, not reissuing side effects.

Historical operation.started does not start the process again. S03 operation deduplication/idempotency remains separate.

Projection replay is pure derivation over durable facts.

## 42. Privileged projection safety

A privileged projection MUST NOT interpret an unknown Event type as a known allow/deny/security/runtime transition.

Unknown required semantic input produces explicit unsupported/incomplete state or a later S05 compatibility behavior proven safe.

Opaque retention and privileged interpretation are distinct.

## 43. Contract version and Event digest

The envelope carries event-contract/schema version metadata sufficient for S05 negotiation.

This task intentionally does not define a whole-Event byte digest because canonical serialization is not frozen. Artifact SHA-256 remains authoritative for Artifact bytes.

Any future Event digest/signature requires canonical serialized representation.

## 44. Adversarial invariants

Conformance work must cover at least:

1. sequence starts at 1;
2. zero sequence rejects;
3. duplicate sequence with different Event rejects;
4. sequence gap rejects;
5. wrong predecessor rejects;
6. predecessor from another stream rejects;
7. duplicate Event ID with conflicting fact rejects;
8. UUID sort cannot override stream sequence;
9. timestamps cannot override stream sequence;
10. unknown type cannot mutate privileged projection;
11. correction does not mutate original Event;
12. causal self-edge rejects;
13. transitive causal cycle rejects;
14. supersession/fork cycles reject;
15. Artifact digest mismatch rejects Evidence qualification;
16. missing required Artifact makes qualification unavailable;
17. agent_report cannot satisfy machine-required gate;
18. agent_report cannot masquerade as human_review;
19. transformed redaction uses new Artifact ID/digest;
20. excluded content has explicit metadata;
21. exclusion metadata contains no secret plaintext;
22. projection rebuild detects missing Events;
23. projection rebuild is deterministic for a valid stream;
24. cross-stream sequences do not create total order;
25. RequestId/OperationId/Grant correlation is not authority.

## 45. Deferred implementation

Deferred to S05 and later work: schema source, generated Rust/TypeScript, canonical serialization, whole-Event digest/signature, compatibility policy, executable fixture corpus, SQLite schema/transactions, CAS implementation, event transport/cursors, export archive, retention/encryption/GC engine, and organization replication.

## 46. Downstream S05 requirements

Generated contracts must preserve immutable Event/Evidence identity, typed owner stream, uint64 sequence, predecessor linkage, exact correlations, lineage, Artifact digest binding, evidence source class, limitations, redaction state, and unknown-event privileged-projection safety.

S05-T03 fixtures must include gaps, duplicates, wrong predecessor, malformed IDs, Artifact digest mismatch, lineage cycles, unknown type, and evidence/redaction negatives.

## 47. Downstream storage requirements

P02 durable storage must provide equivalent atomic append semantics:

- unique Event ID;
- unique stream position;
- expected next sequence/predecessor validation;
- immutable accepted Event;
- durable transaction boundary;
- projection rebuildability.

The physical schema may differ but these properties may not.

## 48. Downstream export requirements

Later export/evidence-bundle work must validate selected stream integrity, validate included Artifact digests, preserve Evidence bindings/limitations, expose exclusion/transformation metadata, avoid ordinary secret plaintext, and distinguish canonical records from transformed views.

No later contract may silently weaken these invariants.
