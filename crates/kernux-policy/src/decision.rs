use crate::{
    CanonicalUtcSecond, ConsequenceClass, ValidatedCapabilityRequest, ValidatedConstraints,
    ValidatedGrant,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DenyReason {
    MalformedInput,
    UnknownInput,
    GrantNotFound,
    GrantNotYetActive,
    GrantRevoked,
    GrantExpired,
    GrantExhausted,
    SubjectMismatch,
    ActionMismatch,
    RuntimeMismatch,
    ResourceMismatch,
    ConstraintMismatch,
    ConsequenceExceeded,
    RuntimeCapabilityUnavailable,
    RuntimeCapabilityVersionMismatch,
    KernelDenied,
    PolicyDenied,
    RemoteHostDenied,
    SecondaryAuthorityDenied,
    DelegationDenied,
    AuditConflict,
    StorageConflict,
}

impl DenyReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MalformedInput => "malformed-input",
            Self::UnknownInput => "unknown-input",
            Self::GrantNotFound => "grant-not-found",
            Self::GrantNotYetActive => "grant-not-yet-active",
            Self::GrantRevoked => "grant-revoked",
            Self::GrantExpired => "grant-expired",
            Self::GrantExhausted => "grant-exhausted",
            Self::SubjectMismatch => "subject-mismatch",
            Self::ActionMismatch => "action-mismatch",
            Self::RuntimeMismatch => "runtime-mismatch",
            Self::ResourceMismatch => "resource-mismatch",
            Self::ConstraintMismatch => "constraint-mismatch",
            Self::ConsequenceExceeded => "consequence-exceeded",
            Self::RuntimeCapabilityUnavailable => "runtime-capability-unavailable",
            Self::RuntimeCapabilityVersionMismatch => "runtime-capability-version-mismatch",
            Self::KernelDenied => "kernel-denied",
            Self::PolicyDenied => "policy-denied",
            Self::RemoteHostDenied => "remote-host-denied",
            Self::SecondaryAuthorityDenied => "secondary-authority-denied",
            Self::DelegationDenied => "delegation-denied",
            Self::AuditConflict => "audit-conflict",
            Self::StorageConflict => "storage-conflict",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GrantMatchDecision {
    Eligible {
        effective_constraints: ValidatedConstraints,
    },
    Denied(DenyReason),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GrantUseDecision {
    BudgetConsumed {
        grant_id: String,
        effective_constraints: ValidatedConstraints,
    },
    Denied(DenyReason),
}

pub fn evaluate_grant_match(
    grant: &ValidatedGrant,
    request: &ValidatedCapabilityRequest,
    authoritative_consequence: ConsequenceClass,
    trusted_now: CanonicalUtcSecond,
    hierarchical_resource: bool,
) -> GrantMatchDecision {
    if request.subject != grant.subject {
        return GrantMatchDecision::Denied(DenyReason::SubjectMismatch);
    }
    if request.action != grant.action {
        return GrantMatchDecision::Denied(DenyReason::ActionMismatch);
    }
    if request.runtime_id != grant.runtime_id || request.runtime_revision != grant.runtime_revision
    {
        return GrantMatchDecision::Denied(DenyReason::RuntimeMismatch);
    }
    if !matches!(
        grant.resource.matches(
            &request.resource,
            grant.resource_scope,
            hierarchical_resource
        ),
        Ok(true)
    ) {
        return GrantMatchDecision::Denied(DenyReason::ResourceMismatch);
    }
    let effective_constraints = match grant.constraints.intersect(&request.constraints) {
        Ok(value) if value.is_subset_of(&grant.constraints) => value,
        Ok(_) | Err(_) => return GrantMatchDecision::Denied(DenyReason::ConstraintMismatch),
    };
    if authoritative_consequence > grant.consequence_ceiling {
        return GrantMatchDecision::Denied(DenyReason::ConsequenceExceeded);
    }
    if trusted_now < grant.not_before {
        return GrantMatchDecision::Denied(DenyReason::GrantNotYetActive);
    }
    if trusted_now >= grant.expires_at {
        return GrantMatchDecision::Denied(DenyReason::GrantExpired);
    }
    GrantMatchDecision::Eligible {
        effective_constraints,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrustedAuthorityDecision {
    Allow,
    Deny,
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DelegationDecision {
    Eligible,
    Denied(DenyReason),
}

pub fn delegation_is_subset(
    parent: &ValidatedGrant,
    child: &ValidatedGrant,
    hierarchical_resource: bool,
) -> bool {
    if child.parent_grant_id.as_deref() != Some(parent.grant_id.as_str())
        || parent.delegation_depth == 0
        || child.delegation_depth >= parent.delegation_depth
        || child.action != parent.action
        || child.runtime_id != parent.runtime_id
        || child.runtime_revision != parent.runtime_revision
        || child.issuer_authority != parent.issuer_authority
        || child.policy_revision != parent.policy_revision
        || !child.constraints.is_subset_of(&parent.constraints)
        || child.consequence_ceiling > parent.consequence_ceiling
        || child.issued_at < parent.issued_at
        || child.not_before < parent.not_before
        || child.expires_at > parent.expires_at
        || child.max_uses > parent.max_uses
        || (child.resource_scope == crate::ResourceScope::Subtree
            && parent.resource_scope != crate::ResourceScope::Subtree)
    {
        return false;
    }
    matches!(
        parent.resource.matches(
            &child.resource,
            parent.resource_scope,
            hierarchical_resource
        ),
        Ok(true)
    )
}

pub fn evaluate_delegation(
    parent: &ValidatedGrant,
    child: &ValidatedGrant,
    delegate_authority: TrustedAuthorityDecision,
    recipient_authority: TrustedAuthorityDecision,
    hierarchical_resource: bool,
) -> DelegationDecision {
    if delegate_authority != TrustedAuthorityDecision::Allow
        || recipient_authority != TrustedAuthorityDecision::Allow
        || !delegation_is_subset(parent, child, hierarchical_resource)
    {
        return DelegationDecision::Denied(DenyReason::DelegationDenied);
    }
    DelegationDecision::Eligible
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GrantRevocationReason {
    Requested,
    PolicyChange,
    Security,
    Superseded,
}

impl GrantRevocationReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Requested => "requested",
            Self::PolicyChange => "policy-change",
            Self::Security => "security",
            Self::Superseded => "superseded",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn delegated_pair() -> (ValidatedGrant, ValidatedGrant) {
        use crate::{Action, CanonicalResource, ResourceScope, ValidatedSubjectScope};

        let parent = ValidatedGrant {
            grant_id: "01890f00-0000-7000-8000-000000000021".into(),
            subject: ValidatedSubjectScope {
                run_id: "01890f00-0000-7000-8000-000000000022".into(),
                agent_session_id: None,
            },
            action: Action::parse("files.write", &[]).unwrap(),
            resource: CanonicalResource::parse(
                "kernux://project/01890f00-0000-7000-8000-000000000023/fs/src",
            )
            .unwrap(),
            resource_uri: "kernux://project/01890f00-0000-7000-8000-000000000023/fs/src".into(),
            resource_scope: ResourceScope::Subtree,
            runtime_id: "01890f00-0000-7000-8000-000000000024".into(),
            runtime_revision: 1,
            constraints: ValidatedConstraints::from_parts(
                Some(vec!["src".into()]),
                Some(4096),
                None,
                Some(4),
                None,
            )
            .unwrap(),
            consequence_ceiling: ConsequenceClass::C2,
            issuer_authority: "local-user".into(),
            policy_revision: 1,
            issued_at: CanonicalUtcSecond::parse("2026-09-20T01:00:00Z").unwrap(),
            not_before: CanonicalUtcSecond::parse("2026-09-20T01:00:00Z").unwrap(),
            expires_at: CanonicalUtcSecond::parse("2026-09-20T02:00:00Z").unwrap(),
            max_uses: 4,
            delegation_depth: 2,
            parent_grant_id: None,
        };
        let mut child = parent.clone();
        child.grant_id = "01890f00-0000-7000-8000-000000000025".into();
        child.resource = CanonicalResource::parse(
            "kernux://project/01890f00-0000-7000-8000-000000000023/fs/src/lib",
        )
        .unwrap();
        child.resource_uri =
            "kernux://project/01890f00-0000-7000-8000-000000000023/fs/src/lib".into();
        child.constraints = ValidatedConstraints::from_parts(
            Some(vec!["src".into()]),
            Some(1024),
            None,
            Some(2),
            None,
        )
        .unwrap();
        child.max_uses = 2;
        child.delegation_depth = 1;
        child.parent_grant_id = Some(parent.grant_id.clone());
        (parent, child)
    }

    #[test]
    fn delegation_requires_independent_authority_and_strict_subset() {
        let (parent, child) = delegated_pair();
        assert_eq!(
            evaluate_delegation(
                &parent,
                &child,
                TrustedAuthorityDecision::Allow,
                TrustedAuthorityDecision::Allow,
                true,
            ),
            DelegationDecision::Eligible
        );
        for decision in [
            TrustedAuthorityDecision::Deny,
            TrustedAuthorityDecision::Unknown,
        ] {
            assert_eq!(
                evaluate_delegation(
                    &parent,
                    &child,
                    decision,
                    TrustedAuthorityDecision::Allow,
                    true,
                ),
                DelegationDecision::Denied(DenyReason::DelegationDenied)
            );
        }

        let mut widened = child.clone();
        widened.max_uses = 5;
        assert!(!delegation_is_subset(&parent, &widened, true));
        let mut widened = child.clone();
        widened.delegation_depth = parent.delegation_depth;
        assert!(!delegation_is_subset(&parent, &widened, true));
        let mut widened = child.clone();
        widened.issuer_authority = "other-authority".into();
        assert!(!delegation_is_subset(&parent, &widened, true));
        let mut widened = child.clone();
        widened.policy_revision = parent.policy_revision + 1;
        assert!(!delegation_is_subset(&parent, &widened, true));
        let mut widened = child;
        widened.expires_at = CanonicalUtcSecond::parse("2026-09-20T03:00:00Z").unwrap();
        assert!(!delegation_is_subset(&parent, &widened, true));
    }

    #[test]
    fn deny_and_revocation_reasons_are_closed_redaction_safe_classes() {
        assert_eq!(DenyReason::GrantExpired.as_str(), "grant-expired");
        assert_eq!(
            DenyReason::RuntimeCapabilityVersionMismatch.as_str(),
            "runtime-capability-version-mismatch"
        );
        assert_eq!(GrantRevocationReason::Security.as_str(), "security");
    }
}
