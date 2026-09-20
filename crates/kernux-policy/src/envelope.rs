use kernux_contracts::{
    CanonicalRef, CapabilityRequest, ConsequenceClass as WireConsequenceClass, EntityKind, Grant,
    ResourceMatch, SubjectScope,
};

use crate::{
    Action, CanonicalResource, CanonicalUtcSecond, ConsequenceClass, PolicyValidationError,
    ResourceScope, ValidatedConstraints, canonical_uuid_v7, validate_action_resource,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidatedSubjectScope {
    pub run_id: String,
    pub agent_session_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidatedCapabilityRequest {
    pub request_id: String,
    pub subject: ValidatedSubjectScope,
    pub action: Action,
    pub resource: CanonicalResource,
    pub runtime_id: String,
    pub runtime_revision: u32,
    pub constraints: ValidatedConstraints,
    pub provenance_event_ids: Vec<String>,
}

impl ValidatedCapabilityRequest {
    pub fn from_wire(
        value: &CapabilityRequest,
        trusted_extensions: &[&str],
    ) -> Result<Self, PolicyValidationError> {
        validate_uuid(&value.request_id)?;
        let subject = validate_subject(&value.subject_scope)?;
        let action = Action::parse(&value.action, trusted_extensions)?;
        let resource = CanonicalResource::parse(&value.resource_uri)?;
        validate_action_resource(&action, &resource)?;
        let runtime_revision = validate_runtime(&value.runtime)?;
        let constraints = ValidatedConstraints::from_wire(&value.requested_constraints)?;
        let provenance_event_ids = value
            .provenance_event_ids
            .iter()
            .map(|id| validate_uuid(id).map(|()| id.clone()))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Self {
            request_id: value.request_id.clone(),
            subject,
            action,
            resource,
            runtime_id: value.runtime.id.clone(),
            runtime_revision,
            constraints,
            provenance_event_ids,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidatedGrant {
    pub grant_id: String,
    pub subject: ValidatedSubjectScope,
    pub action: Action,
    pub resource: CanonicalResource,
    pub resource_uri: String,
    pub resource_scope: ResourceScope,
    pub runtime_id: String,
    pub runtime_revision: u32,
    pub constraints: ValidatedConstraints,
    pub consequence_ceiling: ConsequenceClass,
    pub issuer_authority: String,
    pub policy_revision: u64,
    pub issued_at: CanonicalUtcSecond,
    pub not_before: CanonicalUtcSecond,
    pub expires_at: CanonicalUtcSecond,
    pub max_uses: u64,
    pub delegation_depth: u64,
    pub parent_grant_id: Option<String>,
}

impl ValidatedGrant {
    pub fn from_wire(
        value: &Grant,
        trusted_extensions: &[&str],
        hierarchical_resource: bool,
    ) -> Result<Self, PolicyValidationError> {
        validate_uuid(&value.grant_id)?;
        let subject = validate_subject(&value.subject_scope)?;
        let action = Action::parse(&value.action, trusted_extensions)?;
        let resource = CanonicalResource::parse(&value.resource_uri)?;
        validate_action_resource(&action, &resource)?;
        let resource_scope = match value.resource_match {
            ResourceMatch::Exact => ResourceScope::Exact,
            ResourceMatch::Subtree if hierarchical_resource => ResourceScope::Subtree,
            ResourceMatch::Subtree => return Err(PolicyValidationError::NonHierarchicalSubtree),
        };
        let runtime_revision = validate_runtime(&value.runtime)?;
        let constraints = ValidatedConstraints::from_wire(&value.constraints)?;
        let issued_at = CanonicalUtcSecond::parse(&value.issued_at)?;
        let not_before = CanonicalUtcSecond::parse(&value.not_before)?;
        let expires_at = CanonicalUtcSecond::parse(&value.expires_at)?;
        let issuer = &value.issuer_authority;
        if value.max_uses == 0
            || value.policy_revision == 0
            || constraints
                .max_uses
                .is_some_and(|constraint_uses| constraint_uses != value.max_uses)
            || issued_at > not_before
            || issued_at >= expires_at
            || not_before >= expires_at
            || issuer.is_empty()
            || issuer.len() > 128
            || issuer.trim() != issuer
            || issuer.chars().any(char::is_control)
        {
            return Err(PolicyValidationError::InvalidGrantEnvelope);
        }
        if let Some(parent) = &value.parent_grant_id {
            validate_uuid(parent)?;
            if parent == &value.grant_id {
                return Err(PolicyValidationError::InvalidGrantEnvelope);
            }
        }
        Ok(Self {
            grant_id: value.grant_id.clone(),
            subject,
            action,
            resource,
            resource_uri: value.resource_uri.clone(),
            resource_scope,
            runtime_id: value.runtime.id.clone(),
            runtime_revision,
            constraints,
            consequence_ceiling: wire_consequence(&value.consequence_ceiling),
            issuer_authority: issuer.clone(),
            policy_revision: value.policy_revision,
            issued_at,
            not_before,
            expires_at,
            max_uses: value.max_uses,
            delegation_depth: value.delegation_depth,
            parent_grant_id: value.parent_grant_id.clone(),
        })
    }
}

fn wire_consequence(value: &WireConsequenceClass) -> ConsequenceClass {
    match value {
        WireConsequenceClass::C0 => ConsequenceClass::C0,
        WireConsequenceClass::C1 => ConsequenceClass::C1,
        WireConsequenceClass::C2 => ConsequenceClass::C2,
        WireConsequenceClass::C3 => ConsequenceClass::C3,
        WireConsequenceClass::C4 => ConsequenceClass::C4,
    }
}

fn validate_subject(value: &SubjectScope) -> Result<ValidatedSubjectScope, PolicyValidationError> {
    validate_immutable_ref(&value.run, EntityKind::Run)?;
    if let Some(agent) = &value.agent_session {
        validate_immutable_ref(agent, EntityKind::AgentSession)?;
    }
    Ok(ValidatedSubjectScope {
        run_id: value.run.id.clone(),
        agent_session_id: value.agent_session.as_ref().map(|value| value.id.clone()),
    })
}

fn validate_runtime(value: &CanonicalRef) -> Result<u32, PolicyValidationError> {
    if value.kind != EntityKind::Runtime || !canonical_uuid_v7(&value.id) {
        return Err(PolicyValidationError::InvalidReference);
    }
    value
        .revision
        .filter(|revision| *revision > 0)
        .ok_or(PolicyValidationError::InvalidReference)
}

fn validate_immutable_ref(
    value: &CanonicalRef,
    expected_kind: EntityKind,
) -> Result<(), PolicyValidationError> {
    if value.kind != expected_kind
        || !canonical_uuid_v7(&value.id)
        || !matches!(value.revision, None | Some(1))
    {
        return Err(PolicyValidationError::InvalidReference);
    }
    Ok(())
}

fn validate_uuid(value: &str) -> Result<(), PolicyValidationError> {
    canonical_uuid_v7(value)
        .then_some(())
        .ok_or(PolicyValidationError::InvalidReference)
}

#[cfg(test)]
mod tests {
    use super::*;
    use kernux_contracts::{ConstraintSet, SubjectScope};

    const RUN: &str = "01890f00-0000-7000-8000-000000000002";
    const SESSION: &str = "01890f00-0000-7000-8000-000000000003";
    const RUNTIME: &str = "01890f00-0000-7000-8000-000000000005";

    fn subject() -> SubjectScope {
        SubjectScope {
            run: CanonicalRef {
                id: RUN.into(),
                kind: EntityKind::Run,
                revision: None,
            },
            agent_session: Some(CanonicalRef {
                id: SESSION.into(),
                kind: EntityKind::AgentSession,
                revision: Some(1),
            }),
        }
    }

    fn request() -> CapabilityRequest {
        CapabilityRequest {
            request_id: "01890f00-0000-7000-8000-000000000001".into(),
            subject_scope: subject(),
            action: "files.write".into(),
            resource_uri: "kernux://project/01890f00-0000-7000-8000-000000000004/fs/src/main.rs"
                .into(),
            runtime: CanonicalRef {
                id: RUNTIME.into(),
                kind: EntityKind::Runtime,
                revision: Some(2),
            },
            requested_constraints: ConstraintSet {
                allowed_roots: Some(vec!["src".into()]),
                max_bytes: Some(4096),
                max_duration_ms: None,
                max_uses: None,
                network_hosts: None,
            },
            provenance_event_ids: vec!["01890f00-0000-7000-8000-000000000006".into()],
            reason: Some("display only and not authority".into()),
        }
    }

    fn grant() -> Grant {
        Grant {
            grant_id: "01890f00-0000-7000-8000-000000000007".into(),
            subject_scope: subject(),
            action: "files.write".into(),
            resource_uri: "kernux://project/01890f00-0000-7000-8000-000000000004/fs/src".into(),
            resource_match: ResourceMatch::Subtree,
            runtime: CanonicalRef {
                id: RUNTIME.into(),
                kind: EntityKind::Runtime,
                revision: Some(2),
            },
            constraints: ConstraintSet {
                allowed_roots: Some(vec!["src".into()]),
                max_bytes: Some(4096),
                max_duration_ms: None,
                max_uses: Some(1),
                network_hosts: None,
            },
            consequence_ceiling: WireConsequenceClass::C2,
            issuer_authority: "local-user".into(),
            policy_revision: 4,
            issued_at: "2026-09-19T01:00:00Z".into(),
            not_before: "2026-09-19T01:00:00Z".into(),
            expires_at: "2026-09-19T01:05:00Z".into(),
            max_uses: 1,
            delegation_depth: 0,
            parent_grant_id: None,
        }
    }

    #[test]
    fn generated_request_converts_to_authority_only_validated_form() {
        let value = request();
        let validated = ValidatedCapabilityRequest::from_wire(&value, &[]).unwrap();
        assert_eq!(validated.runtime_revision, 2);
        assert_eq!(validated.constraints.max_bytes, Some(4096));
        assert_eq!(validated.subject.run_id, RUN);
        assert!(!format!("{validated:?}").contains("display only"));
    }

    #[test]
    fn generated_request_rejects_wrong_reference_kind_or_revision() {
        let mut value = request();
        value.runtime.kind = EntityKind::Project;
        assert_eq!(
            ValidatedCapabilityRequest::from_wire(&value, &[]),
            Err(PolicyValidationError::InvalidReference)
        );
        let mut value = request();
        value.subject_scope.run.revision = Some(2);
        assert_eq!(
            ValidatedCapabilityRequest::from_wire(&value, &[]),
            Err(PolicyValidationError::InvalidReference)
        );
    }

    #[test]
    fn generated_grant_enforces_finite_consistent_envelope() {
        let value = grant();
        let validated = ValidatedGrant::from_wire(&value, &[], true).unwrap();
        assert_eq!(validated.max_uses, 1);
        assert_eq!(validated.consequence_ceiling, ConsequenceClass::C2);
        assert_eq!(validated.issuer_authority, "local-user");
    }

    #[test]
    fn generated_grant_rejects_budget_time_scope_and_parent_conflicts() {
        let mut value = grant();
        value.constraints.max_uses = Some(2);
        assert!(ValidatedGrant::from_wire(&value, &[], true).is_err());
        let mut value = grant();
        value.expires_at = value.not_before.clone();
        assert!(ValidatedGrant::from_wire(&value, &[], true).is_err());
        let mut value = grant();
        value.issued_at = "2026-09-19T01:02:00Z".into();
        value.not_before = "2026-09-19T01:01:00Z".into();
        assert!(ValidatedGrant::from_wire(&value, &[], true).is_err());
        let value = grant();
        assert_eq!(
            ValidatedGrant::from_wire(&value, &[], false),
            Err(PolicyValidationError::NonHierarchicalSubtree)
        );
        let mut value = grant();
        value.parent_grant_id = Some(value.grant_id.clone());
        assert_eq!(
            ValidatedGrant::from_wire(&value, &[], true),
            Err(PolicyValidationError::InvalidGrantEnvelope)
        );
    }
}
