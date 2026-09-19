// @generated from protocol/schema/krp.v1.schema.json
// Schema SHA-256: e1fbe682c4a824c8a09619481277cd12cb9c544c1604badec2240b6aa50fe04b
// DO NOT EDIT. Change the schema and regenerate.

export interface ArtifactRef {
  artifact: CanonicalRef;
  media_type?: string;
  role: string;
  sha256?: Sha256Digest;
  size_bytes?: number;
}

export interface CancelRequest {
  operation_id: UuidV7;
  reason?: string;
  request_id: UuidV7;
}

export interface CancelResult {
  operation_id: UuidV7;
  outcome: CancellationOutcome;
  request_id: UuidV7;
}

export type CancellationOutcome =
  | "accepted"
  | "already_terminal"
  | "not_cancellable"
  | "unknown_operation"
  | "denied"
  | "unverifiable";

export interface CanonicalRef {
  id: UuidV7;
  kind: EntityKind;
  revision?: number;
}

export interface CapabilityRequest {
  action: string;
  provenance_event_ids: Array<UuidV7>;
  reason?: string;
  request_id: UuidV7;
  requested_constraints: ConstraintSet;
  resource_uri: string;
  runtime: CanonicalRef;
  subject_scope: SubjectScope;
}

export type ConsequenceClass = "C0" | "C1" | "C2" | "C3" | "C4";

export interface ConstraintSet {
  allowed_roots?: Array<string>;
  max_bytes?: number;
  max_duration_ms?: number;
  max_uses?: number;
  network_hosts?: Array<string>;
}

export type ContactState = "connected" | "degraded" | "disconnected";

export interface DiagnosticField {
  key: string;
  value: string;
}

export type EntityKind =
  | "project"
  | "task"
  | "work_unit"
  | "run"
  | "agent_session"
  | "runtime"
  | "artifact"
  | "evidence"
  | "event";

export type ErrorCategory =
  | "invalid_request"
  | "protocol_mismatch"
  | "capability_unavailable"
  | "authorization_denied"
  | "resource_invalid"
  | "operation_conflict"
  | "operation_unknown"
  | "operation_unverifiable"
  | "operation_not_cancellable"
  | "runtime_unavailable"
  | "runtime_revoked"
  | "timeout"
  | "internal";

export interface Event {
  artifact_refs: Array<ArtifactRef>;
  contract_version: string;
  correlations: EventCorrelations;
  event_id: string;
  event_type: string;
  occurred_at?: string;
  previous_event_id?: string;
  producer: string;
  recorded_at: string;
  redaction: Redaction;
  stream: StreamRef;
  stream_seq: number;
}

export interface EventCorrelations {
  agent_session?: CanonicalRef;
  grant_id?: string;
  operation_id?: string;
  project?: CanonicalRef;
  request_id?: string;
  run?: CanonicalRef;
  runtime?: CanonicalRef;
  task?: CanonicalRef;
  work_unit?: CanonicalRef;
}

export interface Evidence {
  artifact_refs: Array<ArtifactRef>;
  claim: string;
  evidence_id: string;
  implementation_revision?: string;
  limitations: Array<string>;
  observed_event_ids: Array<string>;
  observer: string;
  operation_id?: string;
  recorded_by_event_id: string;
  redaction: Redaction;
  result: string;
  runtime_context?: CanonicalRef;
  source_class: EvidenceSourceClass;
  subject_bindings: Array<CanonicalRef>;
}

export type EvidenceSourceClass =
  | "machine"
  | "external_service"
  | "human_review"
  | "agent_report"
  | "imported";

export type ExecutionState = "live" | "exited" | "unverifiable";

export interface Grant {
  action: string;
  consequence_ceiling: ConsequenceClass;
  constraints: ConstraintSet;
  delegation_depth: number;
  expires_at: string;
  grant_id: UuidV7;
  issued_at: string;
  issuer_authority: string;
  max_uses: number;
  not_before: string;
  parent_grant_id?: UuidV7;
  policy_revision: number;
  resource_match: ResourceMatch;
  resource_uri: string;
  runtime: CanonicalRef;
  subject_scope: SubjectScope;
}

export interface OperationObservation {
  execution_state: ExecutionState;
  observation_generation: number;
  observed_at: string;
  operation_id: UuidV7;
  runtime: CanonicalRef;
  termination_cause?: string;
}

export interface OperationStart {
  action: string;
  capability_version: string;
  constraints: ConstraintSet;
  operation_id: UuidV7;
  payload_sha256: Sha256Digest;
  protocol_contract: string;
  request_id: UuidV7;
  resource_uri: string;
  runtime: CanonicalRef;
  subject_scope: SubjectScope;
}

export interface Redaction {
  affected_roles: Array<string>;
  policy_ref?: string;
  policy_version?: string;
  reason_class?: string;
  source_artifact?: ArtifactRef;
  state: RedactionState;
  transformed_artifact?: ArtifactRef;
}

export type RedactionState = "none" | "excluded" | "transformed";

export type ResourceMatch = "exact" | "subtree";

export type RetryGuidance =
  | "retry_request"
  | "reconcile_operation"
  | "new_operation_if_still_authorized"
  | "do_not_retry";

export interface RuntimeCapability {
  action: string;
  features: Array<string>;
  version: string;
}

export interface RuntimeContactObservation {
  contact_state: ContactState;
  observation_generation: number;
  observed_at: string;
  runtime: CanonicalRef;
}

export interface RuntimeDescriptor {
  capabilities: Array<RuntimeCapability>;
  features: Array<string>;
  observation_generation: number;
  observed_at: string;
  protocol_contract: string;
  runtime: CanonicalRef;
}

export interface RuntimeError {
  category: ErrorCategory;
  code: string;
  details?: Array<DiagnosticField>;
  message: string;
  operation_id?: UuidV7;
  provider_diagnostic?: Array<DiagnosticField>;
  request_id: UuidV7;
  retry_guidance: RetryGuidance;
  side_effect_certainty: SideEffectCertainty;
}

export type Sha256Digest = string;

export type SideEffectCertainty =
  | "definitely_not_started"
  | "operation_known"
  | "may_have_started"
  | "not_applicable";

export type StreamOwnerKind = "project" | "task" | "run" | "runtime";

export interface StreamRef {
  owner: CanonicalRef;
  owner_kind: StreamOwnerKind;
}

export interface SubjectScope {
  agent_session?: CanonicalRef;
  run: CanonicalRef;
}

export type UuidV7 = string;
