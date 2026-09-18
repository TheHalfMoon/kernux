// @generated from protocol/schema/krp.v1.schema.json
// Schema SHA-256: b720977df2afa7c74c69bc28d545430260b0998e715aa0400048996d0887a062
// DO NOT EDIT. Change the schema and regenerate.

export interface ArtifactRef {
  artifact: CanonicalRef;
  media_type?: string;
  role: string;
  sha256?: string;
  size_bytes?: number;
}

export interface CanonicalRef {
  id: string;
  kind: EntityKind;
  revision?: number;
}

export interface CapabilityRequest {
  action: string;
  consequence: ConsequenceClass;
  constraints: ConstraintSet;
  request_id: string;
  resource_uri: string;
  runtime: CanonicalRef;
  subject: CanonicalRef;
}

export interface CapabilityVersion {
  action: string;
  features: Array<string>;
  version: string;
}

export type ConsequenceClass = "C0" | "C1" | "C2" | "C3";

export interface ConstraintSet {
  allowed_roots?: Array<string>;
  max_bytes?: number;
  max_duration_ms?: number;
  max_uses?: number;
  network_hosts?: Array<string>;
}

export type ContactState = "connected" | "degraded" | "disconnected";

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
  consequence: ConsequenceClass;
  constraints: ConstraintSet;
  expires_at: string;
  grant_id: string;
  issuer_policy_revision: number;
  max_uses: number;
  request_id: string;
  resource_uri: string;
  runtime: CanonicalRef;
  subject: CanonicalRef;
}

export interface OperationObservation {
  execution_state: ExecutionState;
  observation_generation: number;
  observed_at: string;
  operation_id: string;
  runtime: CanonicalRef;
  termination_cause?: string;
}

export interface OperationStart {
  action: string;
  constraints: ConstraintSet;
  operation_id: string;
  payload_sha256: string;
  request_id: string;
  resource_uri: string;
  runtime: CanonicalRef;
  subject: CanonicalRef;
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

export type RetryGuidance =
  | "retry_request"
  | "reconcile_operation"
  | "new_operation_if_still_authorized"
  | "do_not_retry";

export interface RuntimeDescriptor {
  capabilities: Array<CapabilityVersion>;
  contract_version: string;
  observation_generation: number;
  observed_at: string;
  runtime: CanonicalRef;
}

export interface RuntimeError {
  category: string;
  code: string;
  message: string;
  operation_id?: string;
  request_id: string;
  retry_guidance: RetryGuidance;
  side_effect_certainty: SideEffectCertainty;
}

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
