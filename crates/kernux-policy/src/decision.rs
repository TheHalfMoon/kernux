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
    if grant.parent_grant_id.is_some() {
        return GrantMatchDecision::Denied(DenyReason::DelegationDenied);
    }
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
