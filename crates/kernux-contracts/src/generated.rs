// @generated from protocol/schema/krp.v1.schema.json
// Schema SHA-256: 904eb86ee9028aca002586187ade53eefea1a51339bff9904182e95d88751b94
// DO NOT EDIT. Change the schema and regenerate.

use serde::{Deserialize, Serialize};

pub const KRP_SCHEMA_SHA256: &str =
    "904eb86ee9028aca002586187ade53eefea1a51339bff9904182e95d88751b94";

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactRef {
    pub artifact: CanonicalRef,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    pub role: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sha256: Option<Sha256Digest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size_bytes: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CancelRequest {
    pub operation_id: UuidV7,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub request_id: UuidV7,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CancelResult {
    pub operation_id: UuidV7,
    pub outcome: CancellationOutcome,
    pub request_id: UuidV7,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CancellationOutcome {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "already_terminal")]
    AlreadyTerminal,
    #[serde(rename = "not_cancellable")]
    NotCancellable,
    #[serde(rename = "unknown_operation")]
    UnknownOperation,
    #[serde(rename = "denied")]
    Denied,
    #[serde(rename = "unverifiable")]
    Unverifiable,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalRef {
    pub id: UuidV7,
    pub kind: EntityKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityRequest {
    pub action: String,
    pub provenance_event_ids: Vec<UuidV7>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    pub request_id: UuidV7,
    pub requested_constraints: ConstraintSet,
    pub resource_uri: String,
    pub runtime: CanonicalRef,
    pub subject_scope: SubjectScope,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CapabilityVersion {
    #[serde(rename = "1")]
    V1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsequenceClass {
    #[serde(rename = "C0")]
    C0,
    #[serde(rename = "C1")]
    C1,
    #[serde(rename = "C2")]
    C2,
    #[serde(rename = "C3")]
    C3,
    #[serde(rename = "C4")]
    C4,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConstraintSet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_roots: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bytes: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_duration_ms: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_uses: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_hosts: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContactState {
    #[serde(rename = "connected")]
    Connected,
    #[serde(rename = "degraded")]
    Degraded,
    #[serde(rename = "disconnected")]
    Disconnected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DaemonHealthState {
    #[serde(rename = "healthy")]
    Healthy,
    #[serde(rename = "unavailable")]
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DaemonLifecycleState {
    #[serde(rename = "starting")]
    Starting,
    #[serde(rename = "serving")]
    Serving,
    #[serde(rename = "shutting_down")]
    ShuttingDown,
    #[serde(rename = "stopped")]
    Stopped,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DaemonProbeKind {
    #[serde(rename = "health_version")]
    HealthVersion,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DaemonProbeRequest {
    pub contract_version: ProtocolContract,
    pub probe: DaemonProbeKind,
    pub request_id: UuidV7,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DaemonProbeResponse {
    pub contract_version: ProtocolContract,
    pub daemon_version: String,
    pub health: DaemonHealthState,
    pub implementation_revision: String,
    pub lifecycle_state: DaemonLifecycleState,
    pub request_id: UuidV7,
    pub schema_sha256: Sha256Digest,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DiagnosticField {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EntityKind {
    #[serde(rename = "project")]
    Project,
    #[serde(rename = "task")]
    Task,
    #[serde(rename = "work_unit")]
    WorkUnit,
    #[serde(rename = "run")]
    Run,
    #[serde(rename = "agent_session")]
    AgentSession,
    #[serde(rename = "runtime")]
    Runtime,
    #[serde(rename = "artifact")]
    Artifact,
    #[serde(rename = "evidence")]
    Evidence,
    #[serde(rename = "event")]
    Event,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorCategory {
    #[serde(rename = "invalid_request")]
    InvalidRequest,
    #[serde(rename = "protocol_mismatch")]
    ProtocolMismatch,
    #[serde(rename = "capability_unavailable")]
    CapabilityUnavailable,
    #[serde(rename = "authorization_denied")]
    AuthorizationDenied,
    #[serde(rename = "resource_invalid")]
    ResourceInvalid,
    #[serde(rename = "operation_conflict")]
    OperationConflict,
    #[serde(rename = "operation_unknown")]
    OperationUnknown,
    #[serde(rename = "operation_unverifiable")]
    OperationUnverifiable,
    #[serde(rename = "operation_not_cancellable")]
    OperationNotCancellable,
    #[serde(rename = "runtime_unavailable")]
    RuntimeUnavailable,
    #[serde(rename = "runtime_revoked")]
    RuntimeRevoked,
    #[serde(rename = "timeout")]
    Timeout,
    #[serde(rename = "internal")]
    Internal,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Event {
    pub artifact_refs: Vec<ArtifactRef>,
    pub contract_version: ProtocolContract,
    pub correlations: EventCorrelations,
    pub event_id: UuidV7,
    pub event_type: String,
    pub lineage: EventLineage,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occurred_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_event_id: Option<UuidV7>,
    pub producer: String,
    pub recorded_at: String,
    pub redaction: Redaction,
    pub sensitivity: String,
    pub stream: StreamRef,
    pub stream_seq: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventCorrelations {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_session: Option<CanonicalRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artifact: Option<CanonicalRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_request_id: Option<UuidV7>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<CanonicalRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_id: Option<UuidV7>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<UuidV7>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project: Option<CanonicalRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_request_id: Option<UuidV7>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run: Option<CanonicalRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime: Option<CanonicalRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<CanonicalRef>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_unit: Option<CanonicalRef>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EventLineage {
    pub caused_by: Vec<UuidV7>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_of: Option<UuidV7>,
    pub derived_from: Vec<UuidV7>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forked_from: Option<UuidV7>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supersedes: Option<UuidV7>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Evidence {
    pub artifact_refs: Vec<ArtifactRef>,
    pub claim: String,
    pub evidence_id: UuidV7,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implementation_revision: Option<String>,
    pub limitations: Vec<String>,
    pub observed_event_ids: Vec<UuidV7>,
    pub observer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<UuidV7>,
    pub recorded_by_event_id: UuidV7,
    pub redaction: Redaction,
    pub result: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runtime_context: Option<CanonicalRef>,
    pub sensitivity: String,
    pub source_class: EvidenceSourceClass,
    pub subject_bindings: Vec<CanonicalRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvidenceSourceClass {
    #[serde(rename = "machine")]
    Machine,
    #[serde(rename = "external_service")]
    ExternalService,
    #[serde(rename = "human_review")]
    HumanReview,
    #[serde(rename = "agent_report")]
    AgentReport,
    #[serde(rename = "imported")]
    Imported,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionState {
    #[serde(rename = "live")]
    Live,
    #[serde(rename = "exited")]
    Exited,
    #[serde(rename = "unverifiable")]
    Unverifiable,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Grant {
    pub action: String,
    pub consequence_ceiling: ConsequenceClass,
    pub constraints: ConstraintSet,
    pub delegation_depth: u64,
    pub expires_at: String,
    pub grant_id: UuidV7,
    pub issued_at: String,
    pub issuer_authority: String,
    pub max_uses: u64,
    pub not_before: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_grant_id: Option<UuidV7>,
    pub policy_revision: u64,
    pub resource_match: ResourceMatch,
    pub resource_uri: String,
    pub runtime: CanonicalRef,
    pub subject_scope: SubjectScope,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperationObservation {
    pub execution_state: ExecutionState,
    pub observation_generation: u64,
    pub observed_at: String,
    pub operation_id: UuidV7,
    pub runtime: CanonicalRef,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub termination_cause: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperationStart {
    pub action: String,
    pub capability_version: CapabilityVersion,
    pub constraints: ConstraintSet,
    pub operation_id: UuidV7,
    pub payload_sha256: Sha256Digest,
    pub protocol_contract: ProtocolContract,
    pub request_id: UuidV7,
    pub resource_uri: String,
    pub runtime: CanonicalRef,
    pub subject_scope: SubjectScope,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProtocolContract {
    #[serde(rename = "krp/1")]
    Krp1,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Redaction {
    pub affected_roles: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_class: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_artifact: Option<ArtifactRef>,
    pub state: RedactionState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transformed_artifact: Option<ArtifactRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RedactionState {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "excluded")]
    Excluded,
    #[serde(rename = "transformed")]
    Transformed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResourceMatch {
    #[serde(rename = "exact")]
    Exact,
    #[serde(rename = "subtree")]
    Subtree,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RetryGuidance {
    #[serde(rename = "retry_request")]
    RetryRequest,
    #[serde(rename = "reconcile_operation")]
    ReconcileOperation,
    #[serde(rename = "new_operation_if_still_authorized")]
    NewOperationIfStillAuthorized,
    #[serde(rename = "do_not_retry")]
    DoNotRetry,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeCapability {
    pub action: String,
    pub features: Vec<String>,
    pub version: CapabilityVersion,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeContactObservation {
    pub contact_state: ContactState,
    pub observation_generation: u64,
    pub observed_at: String,
    pub runtime: CanonicalRef,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeDescriptor {
    pub capabilities: Vec<RuntimeCapability>,
    pub features: Vec<String>,
    pub observation_generation: u64,
    pub observed_at: String,
    pub protocol_contract: ProtocolContract,
    pub runtime: CanonicalRef,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeError {
    pub category: ErrorCategory,
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<DiagnosticField>>,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<UuidV7>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider_diagnostic: Option<Vec<DiagnosticField>>,
    pub request_id: UuidV7,
    pub retry_guidance: RetryGuidance,
    pub side_effect_certainty: SideEffectCertainty,
}

pub type Sha256Digest = String;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SideEffectCertainty {
    #[serde(rename = "definitely_not_started")]
    DefinitelyNotStarted,
    #[serde(rename = "operation_known")]
    OperationKnown,
    #[serde(rename = "may_have_started")]
    MayHaveStarted,
    #[serde(rename = "not_applicable")]
    NotApplicable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum StreamOwnerKind {
    #[serde(rename = "project")]
    Project,
    #[serde(rename = "task")]
    Task,
    #[serde(rename = "run")]
    Run,
    #[serde(rename = "runtime")]
    Runtime,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StreamRef {
    pub owner: CanonicalRef,
    pub owner_kind: StreamOwnerKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SubjectScope {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_session: Option<CanonicalRef>,
    pub run: CanonicalRef,
}

pub type UuidV7 = String;
