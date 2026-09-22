//! Bounded secret broker admission for SG-000026.
//!
//! Admission succeeds only when the exact active Grant authorizes the
//! intended secret use under the exact provider and destination boundary.
//! Possession of a SecretRef, a SecretHandle, a provider identity, or a
//! destination grants zero authority by itself.
//!
//! This module performs no operating system credential-store access, no
//! process spawn, no environment read, no network access, no persistence,
//! and no logging. It never handles secret plaintext: there is no
//! SecretValue input, output, or temporary boundary in this slice. Bounded
//! resolution arrives in a later SG-000026 slice. All errors are static
//! strings that never echo untrusted input and never carry secret material.

use crate::{
    DestinationClass, EnvName, HostName, ProviderCapabilities, ProviderId, SecretHandle, SecretRef,
};
use core::fmt;
use kernux_policy::{CanonicalUtcSecond, ConsequenceClass, ResourceScope, ValidatedGrant};

/// Maximum project identifier length in bytes.
pub const MAX_PROJECT_ID_BYTES: usize = 128;
/// Maximum account identifier length in bytes.
pub const MAX_ACCOUNT_ID_BYTES: usize = 128;
/// Maximum operation identifier length in bytes.
pub const MAX_OPERATION_ID_BYTES: usize = 128;

/// Closed egress classification compatible with the canonical local-privacy plan.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EgressClass {
    None,
    DirectDestination,
    ConnectedAccount,
    ExternalModel,
    ExternalTool,
    RemoteRuntime,
    Update,
    Telemetry,
}

/// Exact approved destination with validated details.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum BrokerDestination {
    ProcessEnv { var: EnvName },
    HostCommand,
    EgressSubstitution { host: HostName },
}

impl BrokerDestination {
    pub const fn class(&self) -> DestinationClass {
        match self {
            Self::ProcessEnv { .. } => DestinationClass::ProcessEnv,
            Self::HostCommand => DestinationClass::HostCommand,
            Self::EgressSubstitution { .. } => DestinationClass::EgressSubstitution,
        }
    }
}
/// Fail-closed broker admission errors. Every message is static.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BrokerError {
    MissingGrant,
    GrantExpired,
    GrantRevoked,
    GrantNotActive,
    GrantExhausted,
    WrongCapability,
    WrongProvider,
    UnsupportedCapability,
    WrongDestination,
    WrongHost,
    WrongEnv,
    CrossProject,
    CrossAccount,
    HandleMismatch,
    MalformedDestination,
    StaleBinding,
    PrivilegeWidening,
    PrivacyDenied,
    SubjectMismatch,
    RuntimeMismatch,
    ResourceMismatch,
}

impl fmt::Display for BrokerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::MissingGrant => "missing secret grant",
            Self::GrantExpired => "expired secret grant",
            Self::GrantRevoked => "revoked secret grant",
            Self::GrantNotActive => "secret grant not yet active",
            Self::GrantExhausted => "exhausted secret grant",
            Self::WrongCapability => "secret grant forbids this use",
            Self::WrongProvider => "secret grant forbids this provider",
            Self::UnsupportedCapability => "secret provider forbids this destination",
            Self::WrongDestination => "secret grant forbids this destination",
            Self::WrongHost => "secret grant forbids this host",
            Self::WrongEnv => "secret grant forbids this environment",
            Self::CrossProject => "secret grant forbids cross-project use",
            Self::CrossAccount => "secret grant forbids cross-account use",
            Self::HandleMismatch => "secret handle does not match grant use",
            Self::MalformedDestination => "invalid secret destination",
            Self::StaleBinding => "stale secret operation binding",
            Self::PrivilegeWidening => "secret use widens granted authority",
            Self::PrivacyDenied => "secret use forbidden by privacy policy",
            Self::SubjectMismatch => "secret grant forbids this subject",
            Self::RuntimeMismatch => "secret grant forbids this runtime",
            Self::ResourceMismatch => "secret grant forbids this reference",
        };
        write!(f, "{message}")
    }
}

impl std::error::Error for BrokerError {}

/// Admission request. Trusted fields must originate from kernuxd durable
/// Grant and broker state, never from tool or model output. Untrusted
/// presented fields must exactly match trusted fields or admission fails.
#[derive(Clone, Debug)]
pub struct BrokerRequest {
    pub grant: Option<ValidatedGrant>,
    pub secret_ref: SecretRef,
    pub presented_handle: SecretHandle,
    pub expected_handle: SecretHandle,
    pub presented_provider: ProviderId,
    pub expected_provider: ProviderId,
    pub provider_capabilities: ProviderCapabilities,
    pub presented_destination: BrokerDestination,
    pub expected_destination: BrokerDestination,
    pub presented_run_id: String,
    pub presented_session_id: Option<String>,
    pub presented_runtime_id: String,
    pub presented_runtime_revision: u32,
    pub presented_project_id: String,
    pub presented_account_id: String,
    pub expected_project_id: String,
    pub expected_account_id: String,
    pub presented_operation_id: Option<String>,
    pub expected_operation_id: Option<String>,
    pub trusted_now: CanonicalUtcSecond,
    pub revoked: bool,
    pub uses_remaining: u64,
    pub claimed_egress: EgressClass,
    pub authoritative_egress: EgressClass,
    pub privacy_allows: bool,
}

/// Bounded admitted use. Non-secret metadata only. Constructible solely
/// through admit so tool output alone cannot forge authority.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdmittedUse {
    grant_id: String,
    secret_ref: String,
    handle: SecretHandle,
    provider: ProviderId,
    destination: BrokerDestination,
    run_id: String,
    agent_session_id: Option<String>,
    runtime_id: String,
    runtime_revision: u32,
    egress: EgressClass,
    project_id: String,
    account_id: String,
    operation_id: Option<String>,
}

impl AdmittedUse {
    pub fn grant_id(&self) -> &str {
        &self.grant_id
    }
    pub fn secret_ref(&self) -> &str {
        &self.secret_ref
    }
    pub fn handle(&self) -> &SecretHandle {
        &self.handle
    }
    pub fn provider(&self) -> ProviderId {
        self.provider
    }
    pub fn destination(&self) -> &BrokerDestination {
        &self.destination
    }
    pub fn run_id(&self) -> &str {
        &self.run_id
    }
    pub fn agent_session_id(&self) -> Option<&str> {
        self.agent_session_id.as_deref()
    }
    pub fn runtime_id(&self) -> &str {
        &self.runtime_id
    }
    pub fn runtime_revision(&self) -> u32 {
        self.runtime_revision
    }
    pub fn egress(&self) -> EgressClass {
        self.egress
    }
    pub fn project_id(&self) -> &str {
        &self.project_id
    }
    pub fn account_id(&self) -> &str {
        &self.account_id
    }
    pub fn operation_id(&self) -> Option<&str> {
        self.operation_id.as_deref()
    }
}

fn valid_bounded_id(value: &str, max: usize) -> bool {
    !value.is_empty() && value.len() <= max && !value.chars().any(char::is_control)
}
/// Admit one exact secret use. Fails closed in deterministic order.
pub fn admit(request: &BrokerRequest) -> Result<AdmittedUse, BrokerError> {
    let grant = request.grant.as_ref().ok_or(BrokerError::MissingGrant)?;
    if grant.action.as_str() != "secret.use" {
        return Err(BrokerError::WrongCapability);
    }
    if grant.resource_scope != ResourceScope::Exact {
        return Err(BrokerError::WrongCapability);
    }
    let resource_ok = request
        .secret_ref
        .resource()
        .matches(&grant.resource, ResourceScope::Exact, false)
        .unwrap_or(false);
    if !resource_ok {
        return Err(BrokerError::ResourceMismatch);
    }
    if request.presented_run_id != grant.subject.run_id
        || request.presented_session_id != grant.subject.agent_session_id
    {
        return Err(BrokerError::SubjectMismatch);
    }
    if request.presented_runtime_id != grant.runtime_id
        || request.presented_runtime_revision != grant.runtime_revision
    {
        return Err(BrokerError::RuntimeMismatch);
    }
    if request.trusted_now < grant.not_before {
        return Err(BrokerError::GrantNotActive);
    }
    if request.trusted_now >= grant.expires_at {
        return Err(BrokerError::GrantExpired);
    }
    if request.revoked {
        return Err(BrokerError::GrantRevoked);
    }
    if grant.max_uses == 0 || request.uses_remaining == 0 || request.uses_remaining > grant.max_uses
    {
        return Err(BrokerError::GrantExhausted);
    }
    if !valid_bounded_id(&request.presented_project_id, MAX_PROJECT_ID_BYTES)
        || !valid_bounded_id(&request.expected_project_id, MAX_PROJECT_ID_BYTES)
        || !valid_bounded_id(&request.presented_account_id, MAX_ACCOUNT_ID_BYTES)
        || !valid_bounded_id(&request.expected_account_id, MAX_ACCOUNT_ID_BYTES)
    {
        return Err(BrokerError::MalformedDestination);
    }
    if request.presented_project_id != request.expected_project_id {
        return Err(BrokerError::CrossProject);
    }
    if request.presented_account_id != request.expected_account_id {
        return Err(BrokerError::CrossAccount);
    }
    match (
        &request.expected_operation_id,
        &request.presented_operation_id,
    ) {
        (Some(expected), Some(presented))
            if valid_bounded_id(expected, MAX_OPERATION_ID_BYTES)
                && valid_bounded_id(presented, MAX_OPERATION_ID_BYTES)
                && expected == presented => {}
        (Some(_), _) => return Err(BrokerError::StaleBinding),
        (None, _) => {}
    }
    if request.presented_handle != request.expected_handle {
        return Err(BrokerError::HandleMismatch);
    }
    if request.presented_provider != request.expected_provider {
        return Err(BrokerError::WrongProvider);
    }
    if !request
        .provider_capabilities
        .can_serve(request.presented_destination.class())
    {
        return Err(BrokerError::UnsupportedCapability);
    }
    if request.presented_destination != request.expected_destination {
        if request.presented_destination.class() != request.expected_destination.class() {
            return Err(BrokerError::WrongDestination);
        }
        match (
            &request.presented_destination,
            &request.expected_destination,
        ) {
            (BrokerDestination::ProcessEnv { .. }, BrokerDestination::ProcessEnv { .. }) => {
                return Err(BrokerError::WrongEnv);
            }
            (
                BrokerDestination::EgressSubstitution { .. },
                BrokerDestination::EgressSubstitution { .. },
            ) => return Err(BrokerError::WrongHost),
            _ => return Err(BrokerError::WrongDestination),
        }
    }
    if let BrokerDestination::EgressSubstitution { host } = &request.presented_destination {
        let allowed = grant
            .constraints
            .network_hosts
            .as_ref()
            .is_some_and(|hosts| hosts.iter().any(|entry| entry == host.as_str()));
        if !allowed {
            return Err(BrokerError::WrongHost);
        }
    }
    if request.claimed_egress != request.authoritative_egress {
        return Err(BrokerError::PrivilegeWidening);
    }
    let local = matches!(
        request.presented_destination.class(),
        DestinationClass::ProcessEnv | DestinationClass::HostCommand
    );
    if local && request.authoritative_egress != EgressClass::None {
        return Err(BrokerError::PrivilegeWidening);
    }
    if !local && request.authoritative_egress == EgressClass::None {
        return Err(BrokerError::PrivilegeWidening);
    }
    if !request.privacy_allows {
        return Err(BrokerError::PrivacyDenied);
    }
    if grant.consequence_ceiling < ConsequenceClass::C2 {
        return Err(BrokerError::WrongCapability);
    }
    Ok(AdmittedUse {
        grant_id: grant.grant_id.clone(),
        secret_ref: request.secret_ref.canonical().to_owned(),
        handle: request.presented_handle.clone(),
        provider: request.presented_provider,
        destination: request.presented_destination.clone(),
        run_id: grant.subject.run_id.clone(),
        agent_session_id: grant.subject.agent_session_id.clone(),
        runtime_id: grant.runtime_id.clone(),
        runtime_revision: grant.runtime_revision,
        egress: request.authoritative_egress,
        project_id: request.expected_project_id.clone(),
        account_id: request.expected_account_id.clone(),
        operation_id: request.expected_operation_id.clone(),
    })
}
#[cfg(test)]
mod broker_tests {
    use super::*;
    use kernux_policy::{Action, CanonicalResource, ValidatedConstraints, ValidatedSubjectScope};

    const RUN: &str = "01890f00-0000-7000-8000-000000000021";
    const SESSION: &str = "01890f00-0000-7000-8000-000000000022";
    const RUNTIME: &str = "01890f00-0000-7000-8000-000000000023";
    const GRANT: &str = "01890f00-0000-7000-8000-000000000024";
    const SECRET: &str = "kernux://secret/ref/demo-token-01";
    const HANDLE: &str = "hnd_0123456789abcdef0123456789abcdef";
    const HANDLE_OTHER: &str = "hnd_ffffffffffffffffffffffffffffffff";

    fn grant_with_hosts(hosts: Option<Vec<String>>, action: &str) -> ValidatedGrant {
        ValidatedGrant {
            grant_id: GRANT.to_owned(),
            subject: ValidatedSubjectScope {
                run_id: RUN.to_owned(),
                agent_session_id: Some(SESSION.to_owned()),
            },
            action: Action::parse(action, &[]).expect("valid action"),
            resource: CanonicalResource::parse(SECRET).expect("valid resource"),
            resource_uri: SECRET.to_owned(),
            resource_scope: ResourceScope::Exact,
            runtime_id: RUNTIME.to_owned(),
            runtime_revision: 2,
            constraints: ValidatedConstraints::from_parts(None, None, None, Some(3), hosts)
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
        }
    }

    fn valid_request() -> BrokerRequest {
        BrokerRequest {
            grant: Some(grant_with_hosts(None, "secret.use")),
            secret_ref: SecretRef::parse(SECRET).expect("valid ref"),
            presented_handle: SecretHandle::parse(HANDLE).expect("valid handle"),
            expected_handle: SecretHandle::parse(HANDLE).expect("valid handle"),
            presented_provider: ProviderId::OsMacosKeychain,
            expected_provider: ProviderId::OsMacosKeychain,
            provider_capabilities: ProviderId::OsMacosKeychain.capabilities(),
            presented_destination: BrokerDestination::ProcessEnv {
                var: EnvName::parse("TOKEN_A").expect("valid env"),
            },
            expected_destination: BrokerDestination::ProcessEnv {
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
            presented_operation_id: Some("op-001".to_owned()),
            expected_operation_id: Some("op-001".to_owned()),
            trusted_now: CanonicalUtcSecond::parse("2026-09-20T01:30:00Z").expect("valid time"),
            revoked: false,
            uses_remaining: 2,
            claimed_egress: EgressClass::None,
            authoritative_egress: EgressClass::None,
            privacy_allows: true,
        }
    }

    fn valid_egress_request() -> BrokerRequest {
        let mut request = valid_request();
        let host = HostName::parse("example.com").expect("valid host");
        request.grant = Some(grant_with_hosts(
            Some(vec!["example.com".to_owned()]),
            "secret.use",
        ));
        request.presented_destination =
            BrokerDestination::EgressSubstitution { host: host.clone() };
        request.expected_destination = BrokerDestination::EgressSubstitution { host };
        request.claimed_egress = EgressClass::DirectDestination;
        request.authoritative_egress = EgressClass::DirectDestination;
        request
    }

    #[test]
    fn exact_valid_admission_succeeds() {
        let admitted = admit(&valid_request()).expect("valid admission");
        assert_eq!(admitted.grant_id(), GRANT);
        assert_eq!(admitted.secret_ref(), SECRET);
        assert_eq!(admitted.handle().as_str(), HANDLE);
        assert_eq!(admitted.provider(), ProviderId::OsMacosKeychain);
        assert_eq!(admitted.run_id(), RUN);
        assert_eq!(admitted.runtime_id(), RUNTIME);
        assert_eq!(admitted.egress(), EgressClass::None);
        let egress_admitted = admit(&valid_egress_request()).expect("valid egress");
        assert_eq!(egress_admitted.egress(), EgressClass::DirectDestination);
    }
    #[test]
    fn every_individual_mismatch_fails() {
        let mut request = valid_request();
        request.grant = None;
        assert_eq!(admit(&request), Err(BrokerError::MissingGrant));
        let mut request = valid_request();
        request.trusted_now = CanonicalUtcSecond::parse("2026-09-20T03:00:00Z").expect("time");
        assert_eq!(admit(&request), Err(BrokerError::GrantExpired));
        let mut request = valid_request();
        request.revoked = true;
        assert_eq!(admit(&request), Err(BrokerError::GrantRevoked));
        let mut request = valid_request();
        request.trusted_now = CanonicalUtcSecond::parse("2026-09-20T00:30:00Z").expect("time");
        assert_eq!(admit(&request), Err(BrokerError::GrantNotActive));
        let mut request = valid_request();
        request.uses_remaining = 0;
        assert_eq!(admit(&request), Err(BrokerError::GrantExhausted));
        let mut request = valid_request();
        request.grant = Some(grant_with_hosts(None, "secret.reveal"));
        assert_eq!(admit(&request), Err(BrokerError::WrongCapability));
        let mut request = valid_request();
        request.presented_provider = ProviderId::OsLinuxSecretService;
        assert_eq!(admit(&request), Err(BrokerError::WrongProvider));
        let mut request = valid_request();
        request.presented_destination = BrokerDestination::HostCommand;
        assert_eq!(admit(&request), Err(BrokerError::WrongDestination));
        let mut request = valid_request();
        request.presented_destination = BrokerDestination::ProcessEnv {
            var: EnvName::parse("OTHER_VAR").expect("env"),
        };
        assert_eq!(admit(&request), Err(BrokerError::WrongEnv));
        let mut request = valid_request();
        request.presented_project_id = "project-beta".to_owned();
        assert_eq!(admit(&request), Err(BrokerError::CrossProject));
        let mut request = valid_request();
        request.presented_account_id = "account-beta".to_owned();
        assert_eq!(admit(&request), Err(BrokerError::CrossAccount));
        let mut request = valid_request();
        request.presented_handle = SecretHandle::parse(HANDLE_OTHER).expect("handle");
        assert_eq!(admit(&request), Err(BrokerError::HandleMismatch));
        let mut request = valid_request();
        request.secret_ref = SecretRef::parse("kernux://secret/ref/other-token").expect("ref");
        assert_eq!(admit(&request), Err(BrokerError::ResourceMismatch));
        let mut request = valid_request();
        request.presented_run_id = "01890f00-0000-7000-8000-000000000099".to_owned();
        assert_eq!(admit(&request), Err(BrokerError::SubjectMismatch));
        let mut request = valid_request();
        request.presented_runtime_revision = 3;
        assert_eq!(admit(&request), Err(BrokerError::RuntimeMismatch));
        let mut request = valid_request();
        request.presented_operation_id = Some("op-002".to_owned());
        assert_eq!(admit(&request), Err(BrokerError::StaleBinding));
        let mut request = valid_request();
        request.claimed_egress = EgressClass::DirectDestination;
        assert_eq!(admit(&request), Err(BrokerError::PrivilegeWidening));
        let mut request = valid_request();
        request.privacy_allows = false;
        assert_eq!(admit(&request), Err(BrokerError::PrivacyDenied));
        let mut unsupported = valid_request();
        unsupported.provider_capabilities = ProviderCapabilities {
            supports_process_env: false,
            supports_host_command: true,
            supports_egress_substitution: true,
        };
        assert_eq!(admit(&unsupported), Err(BrokerError::UnsupportedCapability));
        let mut wrong_host = valid_egress_request();
        wrong_host.presented_destination = BrokerDestination::EgressSubstitution {
            host: HostName::parse("other.example").expect("host"),
        };
        assert_eq!(admit(&wrong_host), Err(BrokerError::WrongHost));
        let mut forbidden_host = valid_egress_request();
        forbidden_host.grant = Some(grant_with_hosts(None, "secret.use"));
        assert_eq!(admit(&forbidden_host), Err(BrokerError::WrongHost));
    }

    #[test]
    fn multi_field_mismatch_fails_deterministically() {
        let mut request = valid_request();
        request.grant = None;
        request.revoked = true;
        request.presented_project_id = "project-beta".to_owned();
        assert_eq!(admit(&request), Err(BrokerError::MissingGrant));
    }

    #[test]
    fn unknown_provider_and_hostile_destination_fail_closed() {
        assert!(ProviderId::parse("aws-secrets-manager").is_err());
        assert!(HostName::parse("evil host!").is_err());
        assert!(EnvName::parse("1EVIL-CRED").is_err());
        assert!(HostName::parse("$(hostname)").is_err());
        assert!(SecretRef::parse("kernux://project/p/fs/src").is_err());
    }

    #[test]
    fn errors_never_echo_untrusted_material() {
        let hostile = "evil-handle-material";
        let mut request = valid_request();
        request.presented_project_id = hostile.to_owned();
        let error = admit(&request).expect_err("must deny");
        let rendered = format!("{error}");
        assert!(!rendered.contains(hostile));
        assert!(!rendered.contains("evil"));
        for error in [
            BrokerError::MissingGrant,
            BrokerError::WrongProvider,
            BrokerError::WrongHost,
            BrokerError::HandleMismatch,
            BrokerError::PrivacyDenied,
        ] {
            let text = format!("{error}");
            assert!(!text.contains(HANDLE));
            assert!(!text.contains(SECRET));
        }
    }

    #[test]
    fn admission_does_not_mutate_grant_and_forge_fails() {
        let request = valid_request();
        let before = request.grant.clone().expect("grant");
        let admitted = admit(&request).expect("valid");
        let after = request.grant.clone().expect("grant");
        assert_eq!(before, after);
        assert_eq!(admitted.grant_id(), before.grant_id);
        let mut forged = valid_request();
        forged.presented_project_id = "project-beta".to_owned();
        assert!(admit(&forged).is_err());
    }

    #[test]
    fn admitted_use_debug_carries_no_plaintext() {
        let admitted = admit(&valid_request()).expect("valid");
        let plaintext = b"correct horse battery staple".to_vec();
        let rendered = format!("{admitted:?}");
        assert!(!crate::leaks_plaintext(&rendered, &plaintext));
        crate::assert_no_plaintext(&rendered, &plaintext, "admitted use");
    }

    #[test]
    fn cross_destination_handle_reuse_denied() {
        let admitted = admit(&valid_request()).expect("destination A admission");
        assert_eq!(admitted.handle().as_str(), HANDLE);
        let mut cross_class = valid_request();
        cross_class.presented_destination = BrokerDestination::HostCommand;
        assert_eq!(admit(&cross_class), Err(BrokerError::WrongDestination));
        let mut cross_detail = valid_request();
        cross_detail.presented_destination = BrokerDestination::ProcessEnv {
            var: EnvName::parse("OTHER_VAR").expect("env"),
        };
        assert_eq!(admit(&cross_detail), Err(BrokerError::WrongEnv));
        let readmitted = admit(&valid_request()).expect("destination A still admits");
        assert_eq!(readmitted.handle().as_str(), admitted.handle().as_str());
        assert_eq!(readmitted.destination(), admitted.destination());
    }

    #[test]
    fn spoofed_provider_identity_denied() {
        let mut spoofed = valid_request();
        spoofed.presented_provider = ProviderId::OsLinuxSecretService;
        assert_eq!(admit(&spoofed), Err(BrokerError::WrongProvider));
        assert!(ProviderId::parse("vault-spoof-99").is_err());
        assert!(ProviderId::parse("keychain-substitute").is_err());
    }

    #[test]
    fn handle_possession_authorizes_nothing() {
        let mut no_grant = valid_request();
        no_grant.grant = None;
        assert_eq!(admit(&no_grant), Err(BrokerError::MissingGrant));
        let mut wrong_handle = valid_request();
        wrong_handle.presented_handle = SecretHandle::parse(HANDLE_OTHER).expect("handle");
        assert_eq!(admit(&wrong_handle), Err(BrokerError::HandleMismatch));
        let admitted = admit(&valid_request()).expect("valid");
        assert_eq!(admitted.grant_id(), GRANT);
        assert_eq!(admitted.handle().as_str(), HANDLE);
    }

    #[test]
    fn reveal_grant_never_admits_use() {
        let mut reveal = valid_request();
        reveal.grant = Some(grant_with_hosts(None, "secret.reveal"));
        assert_eq!(admit(&reveal), Err(BrokerError::WrongCapability));
        let mut elevated_reveal = valid_request();
        let mut grant = grant_with_hosts(None, "secret.reveal");
        grant.consequence_ceiling = ConsequenceClass::C3;
        elevated_reveal.grant = Some(grant);
        assert_eq!(admit(&elevated_reveal), Err(BrokerError::WrongCapability));
    }
}
