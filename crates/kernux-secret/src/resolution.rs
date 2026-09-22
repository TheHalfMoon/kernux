//! Bounded secret resolution types for SG-000026.
//! SG-000027 corrective re-adoption: executable semantics are intentionally unchanged.
//!
//! Resolution binds one broker-admitted use to one live secret value without
//! performing any credential-store access itself. It performs no operating
//! system credential-store access, no process spawn, no environment read, no
//! network access, no persistence, and no logging. Provider reach happens
//! only through the daemon brokered process authority in a later slice; this
//! slice declares only the fail-closed outcome vocabulary.
//!
//! A [`ResolvedUse`] carries live plaintext in memory only through the owned
//! [`SecretValue`]: values move and never duplicate, render only as the
//! fixed redaction constant, and are zeroized on drop. [`ResolvedUse`]
//! implements neither Clone nor Display so an admitted resolution cannot be
//! duplicated into logs, prompts, process arguments, environments, events,
//! or artifact exports. All errors are static strings that never echo
//! untrusted input and never carry secret material.

use crate::{AdmittedUse, BrokerDestination, ProviderId, SecretHandle, SecretValue};
use core::fmt;

/// Fail-closed provider failure kinds.
///
/// Static metadata only. Never carries provider output, secret material, or
/// untrusted input.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProviderFailure {
    /// The provider store is unavailable or unreachable.
    Unavailable,
    /// No credential exists for the requested service and account.
    NotFound,
    /// The provider answered with malformed output.
    MalformedResponse,
    /// The provider denied the lookup.
    Denied,
}

impl fmt::Display for ProviderFailure {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::Unavailable => "secret provider unavailable",
            Self::NotFound => "secret not found",
            Self::MalformedResponse => "malformed secret provider response",
            Self::Denied => "secret provider denied the lookup",
        };
        write!(f, "{message}")
    }
}

impl std::error::Error for ProviderFailure {}

/// Fail-closed resolution errors.
///
/// Every message is a static string. Error values never echo untrusted input,
/// never contain secret material, and never carry a redaction burden.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResolutionError {
    /// No admitted use was presented for resolution.
    NotAdmitted,
    /// The provider lookup failed without producing a value.
    ProviderUnavailable,
    /// No credential exists for the requested reference.
    ProviderNotFound,
    /// The provider answered with malformed output.
    MalformedProviderOutput,
    /// The provider denied the lookup.
    ProviderDenied,
    /// A presented value was empty or exceeded the retained bound.
    ValueRejected,
}

impl fmt::Display for ResolutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::NotAdmitted => "secret use was not admitted",
            Self::ProviderUnavailable => "secret provider unavailable",
            Self::ProviderNotFound => "secret not found",
            Self::MalformedProviderOutput => "malformed secret provider response",
            Self::ProviderDenied => "secret provider denied the lookup",
            Self::ValueRejected => "secret value rejected",
        };
        write!(f, "{message}")
    }
}

impl std::error::Error for ResolutionError {}

impl From<ProviderFailure> for ResolutionError {
    fn from(failure: ProviderFailure) -> Self {
        match failure {
            ProviderFailure::Unavailable => Self::ProviderUnavailable,
            ProviderFailure::NotFound => Self::ProviderNotFound,
            ProviderFailure::MalformedResponse => Self::MalformedProviderOutput,
            ProviderFailure::Denied => Self::ProviderDenied,
        }
    }
}

/// One bounded live resolution of a broker-admitted secret use.
///
/// Binds the admitted metadata (grant, reference, handle, provider,
/// destination) to exactly one [`SecretValue`]. Values move and never
/// duplicate: there is no Clone. Values never render as text: there is no
/// Display. Debug output carries only non-secret metadata and the fixed
/// redaction constant. Memory is zeroized on drop through the owned value.
pub struct ResolvedUse {
    grant_id: String,
    secret_ref: String,
    handle: SecretHandle,
    provider: ProviderId,
    destination: BrokerDestination,
    value: SecretValue,
}

impl ResolvedUse {
    /// Bind one admitted use to one live value. Moves the value, performs no
    /// lookup, grants no new authority, and duplicates nothing.
    pub fn bind(admitted: &AdmittedUse, value: SecretValue) -> Self {
        Self {
            grant_id: admitted.grant_id().to_owned(),
            secret_ref: admitted.secret_ref().to_owned(),
            handle: admitted.handle().clone(),
            provider: admitted.provider(),
            destination: admitted.destination().clone(),
            value,
        }
    }

    /// Non-secret admitted grant identifier.
    pub fn grant_id(&self) -> &str {
        &self.grant_id
    }

    /// Non-secret canonical secret reference.
    pub fn secret_ref(&self) -> &str {
        &self.secret_ref
    }

    /// Non-secret opaque handle this resolution was admitted for.
    pub fn handle(&self) -> &SecretHandle {
        &self.handle
    }

    /// Provider family that serves this resolution.
    pub fn provider(&self) -> ProviderId {
        self.provider
    }

    /// Exact approved destination this resolution is bound to.
    pub fn destination(&self) -> &BrokerDestination {
        &self.destination
    }

    /// Borrow the live plaintext for the one admitted destination only.
    pub fn expose(&self) -> &[u8] {
        self.value.expose()
    }

    /// Live plaintext length in bytes.
    pub fn len(&self) -> usize {
        self.value.len()
    }

    /// Always false: empty values are rejected at construction.
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    /// Release the owned value for the single admitted use. The resolution
    /// must not be reused afterwards; callers must not retain the plaintext
    /// beyond the one admitted destination.
    pub fn into_value(self) -> SecretValue {
        self.value
    }
}

impl fmt::Debug for ResolvedUse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ResolvedUse({} {} {} [redacted secret value])",
            self.grant_id,
            self.secret_ref,
            self.handle.as_str()
        )
    }
}

#[cfg(test)]
mod resolution_tests {
    use super::*;
    use crate::{
        BrokerRequest, EnvName, SecretRef, admit,
        broker::{BrokerDestination as Destination, EgressClass},
    };
    use kernux_policy::{
        Action, CanonicalResource, CanonicalUtcSecond, ConsequenceClass, ResourceScope,
        ValidatedConstraints, ValidatedSubjectScope,
    };

    const RUN: &str = "01890f00-0000-7000-8000-000000000021";
    const SESSION: &str = "01890f00-0000-7000-8000-000000000022";
    const RUNTIME: &str = "01890f00-0000-7000-8000-000000000023";
    const GRANT: &str = "01890f00-0000-7000-8000-000000000024";
    const SECRET: &str = "kernux://secret/ref/demo-token-01";
    const HANDLE: &str = "hnd_0123456789abcdef0123456789abcdef";

    fn admitted_use() -> AdmittedUse {
        let grant = kernux_policy::ValidatedGrant {
            grant_id: GRANT.to_owned(),
            subject: ValidatedSubjectScope {
                run_id: RUN.to_owned(),
                agent_session_id: Some(SESSION.to_owned()),
            },
            action: Action::parse("secret.use", &[]).expect("valid action"),
            resource: CanonicalResource::parse(SECRET).expect("valid resource"),
            resource_uri: SECRET.to_owned(),
            resource_scope: ResourceScope::Exact,
            runtime_id: RUNTIME.to_owned(),
            runtime_revision: 2,
            constraints: ValidatedConstraints::from_parts(None, None, None, Some(3), None)
                .expect("valid constraints"),
            consequence_ceiling: ConsequenceClass::C2,
            issuer_authority: "local-user".to_owned(),
            policy_revision: 1,
            issued_at: CanonicalUtcSecond::parse("2026-09-20T01:00:00Z").expect("valid time"),
            not_before: CanonicalUtcSecond::parse("2026-09-20T01:00:00Z").expect("valid time"),
            expires_at: CanonicalUtcSecond::parse("2026-09-20T02:00:00Z").expect("valid time"),
            max_uses: 3,
            delegation_depth: 0,
            parent_grant_id: None,
        };
        let request = BrokerRequest {
            grant: Some(grant),
            secret_ref: SecretRef::parse(SECRET).expect("valid ref"),
            presented_handle: SecretHandle::parse(HANDLE).expect("valid handle"),
            expected_handle: SecretHandle::parse(HANDLE).expect("valid handle"),
            presented_provider: ProviderId::OsMacosKeychain,
            expected_provider: ProviderId::OsMacosKeychain,
            provider_capabilities: ProviderId::OsMacosKeychain.capabilities(),
            presented_destination: Destination::ProcessEnv {
                var: EnvName::parse("TOKEN_A").expect("valid env"),
            },
            expected_destination: Destination::ProcessEnv {
                var: EnvName::parse("TOKEN_A").expect("valid env"),
            },
            presented_run_id: RUN.to_owned(),
            presented_session_id: Some(SESSION.to_owned()),
            presented_runtime_id: RUNTIME.to_owned(),
            presented_runtime_revision: 2,
            presented_project_id: "project-alpha".to_owned(),
            presented_account_id: "account-alpha".to_owned(),
            expected_project_id: "project-alpha".to_owned(),
            expected_account_id: "account-alpha".to_owned(),
            presented_operation_id: None,
            expected_operation_id: None,
            trusted_now: CanonicalUtcSecond::parse("2026-09-20T01:30:00Z").expect("valid time"),
            revoked: false,
            uses_remaining: 1,
            claimed_egress: EgressClass::None,
            authoritative_egress: EgressClass::None,
            privacy_allows: true,
        };
        admit(&request).expect("valid admission")
    }

    #[test]
    fn bind_carries_admitted_metadata_and_live_value() {
        let admitted = admitted_use();
        let plaintext = b"correct horse battery staple".to_vec();
        let value = SecretValue::from_bytes(plaintext.clone()).expect("valid value");
        let resolved = ResolvedUse::bind(&admitted, value);
        assert_eq!(resolved.grant_id(), GRANT);
        assert_eq!(resolved.secret_ref(), SECRET);
        assert_eq!(resolved.handle().as_str(), HANDLE);
        assert_eq!(resolved.provider(), ProviderId::OsMacosKeychain);
        assert_eq!(
            resolved.destination(),
            &Destination::ProcessEnv {
                var: EnvName::parse("TOKEN_A").expect("valid env"),
            }
        );
        assert_eq!(resolved.expose(), plaintext.as_slice());
        assert_eq!(resolved.len(), plaintext.len());
        assert!(!resolved.is_empty());
    }

    #[test]
    fn provider_failures_map_to_static_resolution_errors() {
        assert_eq!(
            ResolutionError::from(ProviderFailure::Unavailable),
            ResolutionError::ProviderUnavailable
        );
        assert_eq!(
            ResolutionError::from(ProviderFailure::NotFound),
            ResolutionError::ProviderNotFound
        );
        assert_eq!(
            ResolutionError::from(ProviderFailure::MalformedResponse),
            ResolutionError::MalformedProviderOutput
        );
        assert_eq!(
            ResolutionError::from(ProviderFailure::Denied),
            ResolutionError::ProviderDenied
        );
        assert_eq!(
            format!("{}", ProviderFailure::Unavailable),
            "secret provider unavailable"
        );
        assert_eq!(format!("{}", ProviderFailure::NotFound), "secret not found");
        assert_eq!(
            format!("{}", ProviderFailure::MalformedResponse),
            "malformed secret provider response"
        );
        assert_eq!(
            format!("{}", ProviderFailure::Denied),
            "secret provider denied the lookup"
        );
        assert_eq!(
            format!("{}", ResolutionError::NotAdmitted),
            "secret use was not admitted"
        );
        assert_eq!(
            format!("{}", ResolutionError::ValueRejected),
            "secret value rejected"
        );
    }

    #[test]
    fn resolution_errors_never_echo_untrusted_material() {
        let hostile = "evil-handle-material";
        for error in [
            ResolutionError::NotAdmitted,
            ResolutionError::ProviderUnavailable,
            ResolutionError::ProviderNotFound,
            ResolutionError::MalformedProviderOutput,
            ResolutionError::ProviderDenied,
            ResolutionError::ValueRejected,
        ] {
            let rendered = format!("{error}");
            assert!(!rendered.is_empty());
            assert!(!rendered.contains(hostile));
            assert!(!rendered.contains("evil"));
        }
        for failure in [
            ProviderFailure::Unavailable,
            ProviderFailure::NotFound,
            ProviderFailure::MalformedResponse,
            ProviderFailure::Denied,
        ] {
            let rendered = format!("{failure}");
            assert!(!rendered.contains(hostile));
            assert!(!rendered.contains("evil"));
        }
    }

    #[test]
    fn debug_carries_no_plaintext() {
        let admitted = admitted_use();
        let plaintext = b"correct horse battery staple".to_vec();
        let value = SecretValue::from_bytes(plaintext.clone()).expect("valid value");
        let resolved = ResolvedUse::bind(&admitted, value);
        let rendered = format!("{resolved:?}");
        assert!(rendered.contains("[redacted secret value]"));
        assert!(!crate::leaks_plaintext(&rendered, &plaintext));
        crate::assert_no_plaintext(&rendered, &plaintext, "resolved use");
        assert!(rendered.contains(GRANT));
        assert!(rendered.contains(HANDLE));
    }

    #[test]
    fn value_bounds_enforced_before_resolution() {
        use crate::SecretError;
        assert_eq!(
            SecretValue::from_bytes(Vec::new()).expect_err("must deny"),
            SecretError::EmptyValue
        );
        let big = vec![7u8; crate::MAX_SECRET_VALUE_BYTES + 1];
        assert_eq!(
            SecretValue::from_bytes(big).expect_err("must deny"),
            SecretError::ValueTooLarge
        );
    }

    #[test]
    fn unadmitted_use_never_resolves() {
        let mut request = {
            let grant = kernux_policy::ValidatedGrant {
                grant_id: GRANT.to_owned(),
                subject: ValidatedSubjectScope {
                    run_id: RUN.to_owned(),
                    agent_session_id: Some(SESSION.to_owned()),
                },
                action: Action::parse("secret.use", &[]).expect("valid action"),
                resource: CanonicalResource::parse(SECRET).expect("valid resource"),
                resource_uri: SECRET.to_owned(),
                resource_scope: ResourceScope::Exact,
                runtime_id: RUNTIME.to_owned(),
                runtime_revision: 2,
                constraints: ValidatedConstraints::from_parts(None, None, None, Some(3), None)
                    .expect("valid constraints"),
                consequence_ceiling: ConsequenceClass::C2,
                issuer_authority: "local-user".to_owned(),
                policy_revision: 1,
                issued_at: CanonicalUtcSecond::parse("2026-09-20T01:00:00Z").expect("valid time"),
                not_before: CanonicalUtcSecond::parse("2026-09-20T01:00:00Z").expect("valid time"),
                expires_at: CanonicalUtcSecond::parse("2026-09-20T02:00:00Z").expect("valid time"),
                max_uses: 3,
                delegation_depth: 0,
                parent_grant_id: None,
            };
            BrokerRequest {
                grant: Some(grant),
                secret_ref: SecretRef::parse(SECRET).expect("valid ref"),
                presented_handle: SecretHandle::parse(HANDLE).expect("valid handle"),
                expected_handle: SecretHandle::parse("hnd_ffffffffffffffffffffffffffffffff")
                    .expect("valid handle"),
                presented_provider: ProviderId::OsMacosKeychain,
                expected_provider: ProviderId::OsMacosKeychain,
                provider_capabilities: ProviderId::OsMacosKeychain.capabilities(),
                presented_destination: Destination::ProcessEnv {
                    var: EnvName::parse("TOKEN_A").expect("valid env"),
                },
                expected_destination: Destination::ProcessEnv {
                    var: EnvName::parse("TOKEN_A").expect("valid env"),
                },
                presented_run_id: RUN.to_owned(),
                presented_session_id: Some(SESSION.to_owned()),
                presented_runtime_id: RUNTIME.to_owned(),
                presented_runtime_revision: 2,
                presented_project_id: "project-alpha".to_owned(),
                presented_account_id: "account-alpha".to_owned(),
                expected_project_id: "project-alpha".to_owned(),
                expected_account_id: "account-alpha".to_owned(),
                presented_operation_id: None,
                expected_operation_id: None,
                trusted_now: CanonicalUtcSecond::parse("2026-09-20T01:30:00Z").expect("valid time"),
                revoked: false,
                uses_remaining: 1,
                claimed_egress: EgressClass::None,
                authoritative_egress: EgressClass::None,
                privacy_allows: true,
            }
        };
        assert!(admit(&request).is_err(), "handle mismatch must deny");
        request.expected_handle = SecretHandle::parse(HANDLE).expect("valid handle");
        assert!(admit(&request).is_ok(), "corrected request admits");
    }

    #[test]
    fn resolved_use_binds_exact_admitted_metadata() {
        let admitted = admitted_use();
        let plaintext = b"correct horse battery staple".to_vec();
        let value = SecretValue::from_bytes(plaintext.clone()).expect("valid value");
        let resolved = ResolvedUse::bind(&admitted, value);
        assert_eq!(resolved.grant_id(), GRANT);
        assert_eq!(resolved.secret_ref(), SECRET);
        assert_eq!(resolved.handle().as_str(), HANDLE);
        assert_eq!(resolved.provider(), ProviderId::OsMacosKeychain);
        assert_eq!(resolved.destination(), admitted.destination());
        assert_eq!(resolved.len(), plaintext.len());
        let released = resolved.into_value();
        assert_eq!(released.expose(), plaintext.as_slice());
    }

    #[test]
    fn live_value_never_leaks_through_any_rendering() {
        use crate::BrokerError;
        let admitted = admitted_use();
        let plaintext = b"correct horse battery staple".to_vec();
        let value = SecretValue::from_bytes(plaintext.clone()).expect("valid value");
        let resolved = ResolvedUse::bind(&admitted, value);
        let handle = SecretHandle::parse(HANDLE).expect("handle");
        let secret_ref = SecretRef::parse(SECRET).expect("ref");
        for rendering in [
            format!("{resolved:?}"),
            format!("{admitted:?}"),
            format!("{handle:?}"),
            format!("{handle}"),
            format!("{secret_ref:?}"),
            format!("{secret_ref}"),
        ] {
            crate::assert_no_plaintext(&rendering, &plaintext, "live rendering");
        }
        for error in [
            ResolutionError::NotAdmitted,
            ResolutionError::ProviderUnavailable,
            ResolutionError::ProviderNotFound,
            ResolutionError::MalformedProviderOutput,
            ResolutionError::ProviderDenied,
            ResolutionError::ValueRejected,
        ] {
            crate::assert_no_plaintext(&format!("{error}"), &plaintext, "error display");
            crate::assert_no_plaintext(&format!("{error:?}"), &plaintext, "error debug");
        }
        for failure in [
            ProviderFailure::Unavailable,
            ProviderFailure::NotFound,
            ProviderFailure::MalformedResponse,
            ProviderFailure::Denied,
        ] {
            crate::assert_no_plaintext(&format!("{failure}"), &plaintext, "failure display");
            crate::assert_no_plaintext(&format!("{failure:?}"), &plaintext, "failure debug");
        }
        for error in [
            BrokerError::WrongProvider,
            BrokerError::HandleMismatch,
            BrokerError::PrivacyDenied,
        ] {
            crate::assert_no_plaintext(&format!("{error}"), &plaintext, "broker error");
        }
    }
}
