//! Redaction-safe secret audit events for SG-000028.
//!
//! Pure event construction for secret broker admission outcomes. This module
//! builds deterministic audit data from safe metadata only. It never emits,
//! transports, persists, logs, or executes the event, and it performs no
//! I/O, clock reads, randomness, environment reads, network access, provider
//! calls, daemon mutation, or global-state access. Emission, delivery,
//! persistence, provider binding, and long-lived borrowing stay with the
//! daemon-execution slice by packet design.
//!
//! Every constructor takes only already-validated, non-secret inputs: an
//! [`AdmittedUse`] or [`BrokerRequest`], a [`BrokerError`] reason, and a
//! caller-supplied [`CanonicalUtcSecond`] timestamp. Same valid input always
//! produces the same event. Timestamps are never generated here; the owning
//! caller passes its trusted time explicitly.
//!
//! The event carries no secret value, no plaintext token, no provider
//! response body, and no secret-bearing URL, argument, header, or log line.
//! Handles and references are non-secret metadata and may appear. Each
//! stored field names its authorization basis on [`SecretAuditEvent`].
//!
//! KRP binding: [`SecretAuditEvent::event_type`],
//! [`SecretAuditEvent::contract_version`],
//! [`SecretAuditEvent::sensitivity`],
//! [`SecretAuditEvent::redaction_state`], and
//! [`SecretAuditEvent::redaction_reason`] name values for the frozen KRP v1
//! `Event`/`Redaction` envelope. No schema field is added or changed.
//!
//! Audit events are non-authoritative by construction: no API path converts
//! an event into an [`AdmittedUse`], a [`ResolvedUse`], or a secret value.
//! [`SecretAuditEvent::authorizes_use`] is always false. Callers enforce
//! audit-before-side-effect by constructing the event first and refusing the
//! side effect whenever the audit context is unavailable; a denial event
//! never admits anything.

use crate::{
    AdmittedUse, BrokerDestination, BrokerError, BrokerRequest, DESTINATION_HOST_COMMAND,
    DestinationClass, EgressClass, ProviderId, ResolvedUse,
};
use core::fmt;
use kernux_policy::CanonicalUtcSecond;

/// KRP event type for an admitted secret use.
pub const SECRET_AUDIT_ADMITTED_EVENT: &str = "secret.use.admitted";
/// KRP event type for a denied secret use.
pub const SECRET_AUDIT_DENIED_EVENT: &str = "secret.use.denied";
/// KRP event type for a consumed (resolved) secret use.
pub const SECRET_AUDIT_CONSUMED_EVENT: &str = "secret.use.consumed";
/// Frozen KRP contract version these events bind to. No wire change.
pub const SECRET_AUDIT_CONTRACT_VERSION: &str = "krp/1";
/// Sensitivity label for the frozen envelope: metadata only, never plaintext.
pub const SECRET_AUDIT_SENSITIVITY: &str = "secret-metadata-only";
/// Redaction state for the frozen envelope: secret values are excluded.
pub const SECRET_AUDIT_REDACTION_STATE: &str = "excluded";
/// Redaction reason class for the frozen envelope.
pub const SECRET_AUDIT_REDACTION_REASON: &str = "secret-plaintext-excluded";
/// Reason code for allow events.
pub const SECRET_AUDIT_ALLOW_REASON: &str = "allow";
/// Reason code for consumption events.
pub const SECRET_AUDIT_CONSUMED_REASON: &str = "consumed";

/// Discriminant for the three audit event shapes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecretAuditKind {
    /// One broker admission allow outcome.
    Admitted,
    /// One broker denial outcome, for every [`BrokerError`].
    Denied,
    /// One resolution consumption of a broker-admitted use.
    Consumed,
}

impl SecretAuditKind {
    /// The frozen-envelope event type string. Stable by contract.
    pub const fn event_type(self) -> &'static str {
        match self {
            Self::Admitted => SECRET_AUDIT_ADMITTED_EVENT,
            Self::Denied => SECRET_AUDIT_DENIED_EVENT,
            Self::Consumed => SECRET_AUDIT_CONSUMED_EVENT,
        }
    }
}

/// Stable kebab-case reason code for one broker denial. Every code is
/// static: codes never echo untrusted input and never carry secret material.
pub const fn denial_code(error: BrokerError) -> &'static str {
    match error {
        BrokerError::MissingGrant => "missing-grant",
        BrokerError::GrantExpired => "grant-expired",
        BrokerError::GrantRevoked => "grant-revoked",
        BrokerError::GrantNotActive => "grant-not-active",
        BrokerError::GrantExhausted => "grant-exhausted",
        BrokerError::WrongCapability => "wrong-capability",
        BrokerError::WrongProvider => "wrong-provider",
        BrokerError::UnsupportedCapability => "unsupported-capability",
        BrokerError::WrongDestination => "wrong-destination",
        BrokerError::WrongHost => "wrong-host",
        BrokerError::WrongEnv => "wrong-env",
        BrokerError::CrossProject => "cross-project",
        BrokerError::CrossAccount => "cross-account",
        BrokerError::HandleMismatch => "handle-mismatch",
        BrokerError::MalformedDestination => "malformed-destination",
        BrokerError::StaleBinding => "stale-binding",
        BrokerError::PrivilegeWidening => "privilege-widening",
        BrokerError::PrivacyDenied => "privacy-denied",
        BrokerError::SubjectMismatch => "subject-mismatch",
        BrokerError::RuntimeMismatch => "runtime-mismatch",
        BrokerError::ResourceMismatch => "resource-mismatch",
    }
}
/// Field authorization basis (secret-safety rule):
/// - `grant_id`: grant correlation identity from the broker decision.
/// - `secret_ref`: secret reference identity, never the value.
/// - `provider`: provider identity/reference.
/// - `destination_class` / `destination_detail`: approved use shape and its
///   validated name (env var, host, or host-command marker).
/// - `operation_id`: evidence/correlation ID where supplied.
/// - `egress`: policy decision (authoritative egress class).
/// - `reason`: result class / safe failure reason code.
/// - `recorded_at`: timestamp supplied by the owning caller.
#[derive(Clone, PartialEq, Eq)]
pub struct SecretAuditEvent {
    kind: SecretAuditKind,
    grant_id: Option<String>,
    secret_ref: String,
    provider: ProviderId,
    destination_class: DestinationClass,
    destination_detail: String,
    operation_id: Option<String>,
    egress: Option<EgressClass>,
    reason: &'static str,
    recorded_at: CanonicalUtcSecond,
}

fn destination_detail(destination: &BrokerDestination) -> String {
    match destination {
        BrokerDestination::ProcessEnv { var } => var.as_str().to_owned(),
        BrokerDestination::HostCommand => DESTINATION_HOST_COMMAND.to_owned(),
        BrokerDestination::EgressSubstitution { host } => host.as_str().to_owned(),
    }
}

impl SecretAuditEvent {
    /// Build the audit event for one admission allow outcome.
    pub fn for_allow(admitted: &AdmittedUse, recorded_at: CanonicalUtcSecond) -> Self {
        Self {
            kind: SecretAuditKind::Admitted,
            grant_id: Some(admitted.grant_id().to_owned()),
            secret_ref: admitted.secret_ref().to_owned(),
            provider: admitted.provider(),
            destination_class: admitted.destination().class(),
            destination_detail: destination_detail(admitted.destination()),
            operation_id: admitted.operation_id().map(str::to_owned),
            egress: Some(admitted.egress()),
            reason: SECRET_AUDIT_ALLOW_REASON,
            recorded_at,
        }
    }

    /// Build the audit event for one denial outcome. `grant_id` is `None`
    /// when no grant existed; nothing is invented.
    pub fn for_denial(
        request: &BrokerRequest,
        error: BrokerError,
        recorded_at: CanonicalUtcSecond,
    ) -> Self {
        Self {
            kind: SecretAuditKind::Denied,
            grant_id: request.grant.as_ref().map(|grant| grant.grant_id.clone()),
            secret_ref: request.secret_ref.canonical().to_owned(),
            provider: request.presented_provider,
            destination_class: request.presented_destination.class(),
            destination_detail: destination_detail(&request.presented_destination),
            operation_id: request.expected_operation_id.clone(),
            egress: Some(request.authoritative_egress),
            reason: denial_code(error),
            recorded_at,
        }
    }

    /// Record consumption of one resolved use. Carries no value and retains
    /// no plaintext beyond the admitted destination binding. Operation
    /// correlation lives on the matching allow event; the daemon-execution
    /// slice joins the two by grant and secret ref.
    pub fn for_consumption(resolved: &ResolvedUse, recorded_at: CanonicalUtcSecond) -> Self {
        Self {
            kind: SecretAuditKind::Consumed,
            grant_id: Some(resolved.grant_id().to_owned()),
            secret_ref: resolved.secret_ref().to_owned(),
            provider: resolved.provider(),
            destination_class: resolved.destination().class(),
            destination_detail: destination_detail(resolved.destination()),
            operation_id: None,
            egress: None,
            reason: SECRET_AUDIT_CONSUMED_REASON,
            recorded_at,
        }
    }

    /// Event discriminant.
    pub const fn kind(&self) -> SecretAuditKind {
        self.kind
    }

    /// Frozen-envelope event type string.
    pub fn event_type(&self) -> &'static str {
        self.kind.event_type()
    }

    /// Grant correlation identity, absent only when no grant existed.
    pub fn grant_id(&self) -> Option<&str> {
        self.grant_id.as_deref()
    }

    /// Secret reference identity. Never a value.
    pub fn secret_ref(&self) -> &str {
        &self.secret_ref
    }

    /// Provider identity/reference.
    pub const fn provider(&self) -> ProviderId {
        self.provider
    }

    /// Approved destination shape.
    pub const fn destination_class(&self) -> DestinationClass {
        self.destination_class
    }

    /// Validated destination name or host-command marker.
    pub fn destination_detail(&self) -> &str {
        &self.destination_detail
    }

    /// Evidence/correlation ID, where supplied.
    pub fn operation_id(&self) -> Option<&str> {
        self.operation_id.as_deref()
    }

    /// Authoritative egress policy decision, where carried.
    pub const fn egress(&self) -> Option<EgressClass> {
        self.egress
    }

    /// Result class or safe failure reason code.
    pub const fn reason_code(&self) -> &'static str {
        self.reason
    }

    /// Caller-supplied timestamp.
    pub const fn recorded_at(&self) -> CanonicalUtcSecond {
        self.recorded_at
    }

    /// Caller-supplied timestamp as canonical text.
    pub fn recorded_at_text(&self) -> String {
        self.recorded_at.to_canonical_text()
    }

    /// Frozen KRP contract version. Value binding, not a schema change.
    pub const fn contract_version(&self) -> &'static str {
        SECRET_AUDIT_CONTRACT_VERSION
    }

    /// Frozen-envelope sensitivity label.
    pub const fn sensitivity(&self) -> &'static str {
        SECRET_AUDIT_SENSITIVITY
    }

    /// Frozen-envelope redaction state.
    pub const fn redaction_state(&self) -> &'static str {
        SECRET_AUDIT_REDACTION_STATE
    }

    /// Frozen-envelope redaction reason class.
    pub const fn redaction_reason(&self) -> &'static str {
        SECRET_AUDIT_REDACTION_REASON
    }

    /// Audit events never authorize use. Always false by construction: no
    /// API path converts an event into an admitted use, a resolution, or a
    /// secret value.
    pub const fn authorizes_use(&self) -> bool {
        false
    }

    /// True when this allow event projects exactly `admitted`. Callers use
    /// this to prove the audit was built before the side effect proceeds.
    pub fn corresponds_to_allow(&self, admitted: &AdmittedUse) -> bool {
        self.kind == SecretAuditKind::Admitted
            && self.grant_id.as_deref() == Some(admitted.grant_id())
            && self.secret_ref == admitted.secret_ref()
            && self.provider == admitted.provider()
            && self.destination_class == admitted.destination().class()
            && self.destination_detail == destination_detail(admitted.destination())
            && self.operation_id == admitted.operation_id().map(str::to_owned)
            && self.reason == SECRET_AUDIT_ALLOW_REASON
    }

    /// Deterministic canonical projection: fixed field order, `|`-joined
    /// `key=value` pairs. Every value is validated metadata without control
    /// characters, so equal inputs render byte-identically.
    pub fn to_canonical_string(&self) -> String {
        format!(
            "event_type={}|grant_id={}|secret_ref={}|provider={}|destination_class={}|destination_detail={}|operation_id={}|egress={:?}|reason={}|recorded_at={}|contract={}|sensitivity={}|redaction={}|redaction_reason={}",
            self.event_type(),
            self.grant_id.as_deref().unwrap_or(""),
            self.secret_ref,
            self.provider.as_str(),
            self.destination_class.as_str(),
            self.destination_detail,
            self.operation_id.as_deref().unwrap_or(""),
            self.egress,
            self.reason,
            self.recorded_at_text(),
            self.contract_version(),
            self.sensitivity(),
            self.redaction_state(),
            self.redaction_reason(),
        )
    }
}

impl fmt::Debug for SecretAuditEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SecretAuditEvent")
            .field("event_type", &self.event_type())
            .field("grant_id", &self.grant_id.as_deref().unwrap_or(""))
            .field("secret_ref", &self.secret_ref)
            .field("provider", &self.provider.as_str())
            .field("destination_class", &self.destination_class.as_str())
            .field("destination_detail", &self.destination_detail)
            .field("operation_id", &self.operation_id.as_deref().unwrap_or(""))
            .field("reason", &self.reason)
            .field("recorded_at", &self.recorded_at_text())
            .finish()
    }
}

impl fmt::Display for SecretAuditEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "secret audit {} grant={} ref={} provider={} destination={} reason={} at={}",
            self.event_type(),
            self.grant_id.as_deref().unwrap_or("-"),
            self.secret_ref,
            self.provider.as_str(),
            self.destination_class.as_str(),
            self.reason,
            self.recorded_at_text(),
        )
    }
}
#[cfg(test)]
mod audit_tests {
    use super::*;
    use crate::{EnvName, SecretHandle, SecretRef, SecretValue, admit};
    use kernux_policy::{
        Action, CanonicalResource, ConsequenceClass, ResourceScope, ValidatedConstraints,
        ValidatedGrant, ValidatedSubjectScope,
    };

    const RUN: &str = "01890f00-0000-7000-8000-000000000021";
    const SESSION: &str = "01890f00-0000-7000-8000-000000000022";
    const RUNTIME: &str = "01890f00-0000-7000-8000-000000000023";
    const GRANT: &str = "01890f00-0000-7000-8000-000000000024";
    const SECRET: &str = "kernux://secret/ref/demo-token-01";
    const HANDLE: &str = "hnd_0123456789abcdef0123456789abcdef";
    const STAMP: &str = "2026-09-20T01:30:00Z";

    fn stamp() -> CanonicalUtcSecond {
        CanonicalUtcSecond::parse(STAMP).expect("valid stamp")
    }

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
            trusted_now: stamp(),
            revoked: false,
            uses_remaining: 2,
            claimed_egress: EgressClass::None,
            authoritative_egress: EgressClass::None,
            privacy_allows: true,
        }
    }

    fn admitted_use() -> AdmittedUse {
        admit(&valid_request()).expect("valid admission")
    }

    fn surfaces(event: &SecretAuditEvent) -> [String; 3] {
        [
            format!("{event:?}"),
            format!("{event}"),
            event.to_canonical_string(),
        ]
    }

    fn assert_surfaces_clean(event: &SecretAuditEvent, plaintext: &[u8], what: &str) {
        for surface in surfaces(event) {
            assert!(
                !crate::leaks_plaintext(&surface, plaintext),
                "plaintext leak in {what}"
            );
            crate::assert_no_plaintext(&surface, plaintext, what);
        }
    }

    #[test]
    fn allow_event_projects_exact_metadata() {
        let admitted = admitted_use();
        let event = SecretAuditEvent::for_allow(&admitted, stamp());
        assert_eq!(event.kind(), SecretAuditKind::Admitted);
        assert_eq!(event.event_type(), SECRET_AUDIT_ADMITTED_EVENT);
        assert_eq!(event.grant_id(), Some(GRANT));
        assert_eq!(event.secret_ref(), SECRET);
        assert_eq!(event.provider(), ProviderId::OsMacosKeychain);
        assert_eq!(event.destination_class(), DestinationClass::ProcessEnv);
        assert_eq!(event.destination_detail(), "TOKEN_A");
        assert_eq!(event.operation_id(), Some("op-001"));
        assert_eq!(event.egress(), Some(EgressClass::None));
        assert_eq!(event.reason_code(), SECRET_AUDIT_ALLOW_REASON);
        assert_eq!(event.recorded_at_text(), STAMP);
        assert!(event.corresponds_to_allow(&admitted));
    }

    #[test]
    fn every_denial_emits_a_stable_reason_code() {
        let request = valid_request();
        let errors = [
            BrokerError::MissingGrant,
            BrokerError::GrantExpired,
            BrokerError::GrantRevoked,
            BrokerError::GrantNotActive,
            BrokerError::GrantExhausted,
            BrokerError::WrongCapability,
            BrokerError::WrongProvider,
            BrokerError::UnsupportedCapability,
            BrokerError::WrongDestination,
            BrokerError::WrongHost,
            BrokerError::WrongEnv,
            BrokerError::CrossProject,
            BrokerError::CrossAccount,
            BrokerError::HandleMismatch,
            BrokerError::MalformedDestination,
            BrokerError::StaleBinding,
            BrokerError::PrivilegeWidening,
            BrokerError::PrivacyDenied,
            BrokerError::SubjectMismatch,
            BrokerError::RuntimeMismatch,
            BrokerError::ResourceMismatch,
        ];
        let expected_count = errors.len();
        let mut codes: Vec<&'static str> = Vec::with_capacity(expected_count);
        for error in errors {
            let event = SecretAuditEvent::for_denial(&request, error, stamp());
            assert_eq!(event.kind(), SecretAuditKind::Denied);
            assert_eq!(event.event_type(), SECRET_AUDIT_DENIED_EVENT);
            assert_eq!(event.reason_code(), denial_code(error));
            assert!(!event.corresponds_to_allow(&admitted_use()));
            codes.push(event.reason_code());
        }
        codes.sort_unstable();
        codes.dedup();
        assert_eq!(codes.len(), expected_count, "reason codes must be distinct");
        for code in codes {
            assert!(
                !code.is_empty()
                    && code.bytes().all(|byte| byte.is_ascii_lowercase()
                        || byte.is_ascii_digit()
                        || byte == b'-'),
                "kebab-case code: {code}"
            );
        }
    }

    #[test]
    fn denial_without_grant_invents_nothing() {
        let mut request = valid_request();
        request.grant = None;
        let event = SecretAuditEvent::for_denial(&request, BrokerError::MissingGrant, stamp());
        assert_eq!(event.grant_id(), None);
        assert_eq!(event.reason_code(), "missing-grant");
        assert_eq!(event.secret_ref(), SECRET);
    }

    #[test]
    fn denial_events_match_real_admission_failures() {
        let cases: [(fn(&mut BrokerRequest), BrokerError); 6] = [
            (|request| request.grant = None, BrokerError::MissingGrant),
            (
                |request| request.presented_provider = ProviderId::OsLinuxSecretService,
                BrokerError::WrongProvider,
            ),
            (
                |request| request.presented_project_id = "project-beta".to_owned(),
                BrokerError::CrossProject,
            ),
            (|request| request.revoked = true, BrokerError::GrantRevoked),
            (
                |request| request.privacy_allows = false,
                BrokerError::PrivacyDenied,
            ),
            (
                |request| {
                    request.presented_destination = BrokerDestination::HostCommand;
                },
                BrokerError::WrongDestination,
            ),
        ];
        for (mutate, expected) in cases {
            let mut request = valid_request();
            mutate(&mut request);
            let failure = admit(&request).expect_err("must deny");
            assert_eq!(failure, expected);
            let event = SecretAuditEvent::for_denial(&request, failure, stamp());
            assert_eq!(event.reason_code(), denial_code(expected));
        }
    }

    #[test]
    fn consumption_records_use_without_plaintext() {
        let plaintext = b"correct horse battery staple".to_vec();
        let resolved = ResolvedUse::bind(
            &admitted_use(),
            SecretValue::from_bytes(plaintext.clone()).expect("value"),
        );
        // The audit path never borrows the live value: no `expose()` call.
        let event = SecretAuditEvent::for_consumption(&resolved, stamp());
        assert_eq!(event.kind(), SecretAuditKind::Consumed);
        assert_eq!(event.event_type(), SECRET_AUDIT_CONSUMED_EVENT);
        assert_eq!(event.grant_id(), Some(GRANT));
        assert_eq!(event.secret_ref(), SECRET);
        assert_eq!(event.reason_code(), SECRET_AUDIT_CONSUMED_REASON);
        assert_eq!(event.operation_id(), None);
        assert_eq!(event.egress(), None);
        assert_surfaces_clean(&event, &plaintext, "consumption event");
    }

    #[test]
    fn renderings_are_deterministic_and_stable() {
        let first = SecretAuditEvent::for_allow(&admitted_use(), stamp());
        let second = SecretAuditEvent::for_allow(&admitted_use(), stamp());
        assert_eq!(first, second);
        let (debug, display, canonical) = (
            format!("{first:?}"),
            format!("{first}"),
            first.to_canonical_string(),
        );
        assert_eq!(debug, format!("{second:?}"));
        assert_eq!(display, format!("{second}"));
        assert_eq!(canonical, second.to_canonical_string());
        assert!(debug.contains(SECRET_AUDIT_ADMITTED_EVENT));
        assert!(display.contains(SECRET_AUDIT_ALLOW_REASON));
        let order = [
            "event_type=",
            "grant_id=",
            "secret_ref=",
            "provider=",
            "reason=",
            "recorded_at=",
        ];
        let mut cursor = 0;
        for key in order {
            let found = canonical[cursor..].find(key).expect("stable key order");
            cursor += found;
        }
    }

    #[test]
    fn envelope_binds_the_frozen_krp_vocabulary() {
        for event_type in [
            SECRET_AUDIT_ADMITTED_EVENT,
            SECRET_AUDIT_DENIED_EVENT,
            SECRET_AUDIT_CONSUMED_EVENT,
        ] {
            let tokens: Vec<&str> = event_type.split('.').collect();
            assert!(tokens.len() >= 2, "KRP event shape: {event_type}");
            for token in tokens {
                let mut bytes = token.bytes();
                assert!(matches!(bytes.next(), Some(b'a'..=b'z')));
                assert!(bytes.all(|byte| byte.is_ascii_lowercase()
                    || byte.is_ascii_digit()
                    || byte == b'_'));
            }
        }
        assert_eq!(
            SecretAuditKind::Admitted.event_type(),
            SECRET_AUDIT_ADMITTED_EVENT
        );
        assert_eq!(
            SecretAuditKind::Denied.event_type(),
            SECRET_AUDIT_DENIED_EVENT
        );
        assert_eq!(
            SecretAuditKind::Consumed.event_type(),
            SECRET_AUDIT_CONSUMED_EVENT
        );
        let event = SecretAuditEvent::for_allow(&admitted_use(), stamp());
        assert_eq!(event.contract_version(), "krp/1");
        assert_eq!(event.sensitivity(), SECRET_AUDIT_SENSITIVITY);
        assert_eq!(event.redaction_state(), "excluded");
        assert_eq!(event.redaction_reason(), SECRET_AUDIT_REDACTION_REASON);
    }

    #[test]
    fn audit_events_never_authorize_use() {
        let admitted = admitted_use();
        let allow = SecretAuditEvent::for_allow(&admitted, stamp());
        let denial =
            SecretAuditEvent::for_denial(&valid_request(), BrokerError::PrivacyDenied, stamp());
        let resolved = ResolvedUse::bind(
            &admitted,
            SecretValue::from_bytes(b"top secret value".to_vec()).expect("value"),
        );
        let consumed = SecretAuditEvent::for_consumption(&resolved, stamp());
        for event in [&allow, &denial, &consumed] {
            assert!(!event.authorizes_use());
        }
        assert!(!denial.corresponds_to_allow(&admitted));
        assert!(!consumed.corresponds_to_allow(&admitted));
        let mut other = valid_request();
        other.expected_operation_id = Some("op-002".to_owned());
        other.presented_operation_id = Some("op-002".to_owned());
        let other_admitted = admit(&other).expect("valid admission");
        assert!(allow.corresponds_to_allow(&admitted));
        assert!(
            !SecretAuditEvent::for_allow(&other_admitted, stamp()).corresponds_to_allow(&admitted)
        );
    }

    #[test]
    fn timestamp_failure_blocks_before_admission() {
        // Constructors require an already-parsed trusted timestamp, so a
        // caller that cannot parse its audit time holds no event and must
        // refuse the side effect: audit failure never admits.
        for bad in [
            "",
            "2026-09-20",
            "2026-13-01T00:00:00Z",
            "2026-09-20T25:00:00Z",
            "2026-09-20 01:30:00Z",
            "not-a-time",
        ] {
            assert!(
                CanonicalUtcSecond::parse(bad).is_err(),
                "must reject {bad:?}"
            );
        }
        assert_eq!(stamp().to_canonical_text(), STAMP);
    }

    #[test]
    fn leak_corpus_never_reaches_event_surfaces() {
        let secrets: [&[u8]; 14] = [
            b"sk-live-51H7x9Kp2mQv8Tn3Yw",
            b"Bearer eyJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIxMjM0In0.AFTo3p0g0uXk7vQ",
            b"postgres://admin:s3cr3t-p@ss@db.internal:5432/app",
            b"Authorization: Basic YWRtaW46c3VwZXItc2VjcmV0",
            b"-----BEGIN PRIVATE KEY-----\nMIIEvQIBADANBgkqhkiG9w0BAQEFAASC",
            b"hunter2-correct-horse",
            b"AKIAIOSFODNN7EXAMPLE",
            b"ghp_0123456789abcdefghijKLMNOP",
            b"slack-synthetic-token-0123456789-abcdefghijklmnopqrstuv",
            b"mongodb+srv://user:p%40ss%3Aw0rd@cluster.example.net/db?secret=1",
            "{\"token\": \"super-secret-value-9\", \"nested\": {\"key\": \"super-secret-value-9\"}}".as_bytes(),
            "pässwörd-sëcret-✓-unicode".as_bytes(),
            b"repeat-secret::repeat-secret::repeat-secret",
            b"a=b;c=d|e=f\ttoken=delimited-secret-value",
        ];
        let resolved = ResolvedUse::bind(
            &admitted_use(),
            SecretValue::from_bytes(b"corpus-holder-value".to_vec()).expect("value"),
        );
        let events = [
            SecretAuditEvent::for_allow(&admitted_use(), stamp()),
            SecretAuditEvent::for_denial(&valid_request(), BrokerError::WrongHost, stamp()),
            SecretAuditEvent::for_consumption(&resolved, stamp()),
        ];
        for secret in &secrets {
            assert!(!secret.is_empty());
            for event in &events {
                assert_surfaces_clean(event, secret, "leak corpus");
            }
            let value = SecretValue::from_bytes(secret.to_vec()).expect("corpus value");
            let debug = format!("{value:?}");
            assert!(!crate::leaks_plaintext(&debug, secret));
            crate::assert_no_plaintext(&debug, secret, "corpus value debug");
        }
        for error in [BrokerError::HandleMismatch, BrokerError::PrivacyDenied] {
            let rendered = format!("{error}");
            for secret in &secrets {
                assert!(!crate::leaks_plaintext(&rendered, secret));
            }
        }
    }
}
