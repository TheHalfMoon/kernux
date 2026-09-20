use std::fmt;

use kernux_policy::{
    MandatoryAuthorityInputs, PolicyDecision, TrustedConsequenceFacts, ValidatedCapabilityRequest,
    ValidatedConstraints, classify_consequence,
};
use kernux_store::{CanonicalId, EventAppend, PolicyDecisionContext, Store};

/// Request-bound daemon context for one final pre-side-effect authorization decision.
///
/// The validated request remains a proposal and is not authority. All authority, time,
/// consequence, extension-registration, resource-hierarchy, and audit inputs in this
/// in-process snapshot must come from daemon-owned trusted state.
#[derive(Debug)]
pub struct TrustedAuthorizationSnapshot<'a> {
    request: ValidatedCapabilityRequest,
    mandatory_authorities: MandatoryAuthorityInputs<'a>,
    consequence_facts: TrustedConsequenceFacts,
    trusted_now: kernux_policy::CanonicalUtcSecond,
    trusted_extensions: &'a [&'a str],
    hierarchical_resource: bool,
    audit_event: &'a EventAppend,
}

impl<'a> TrustedAuthorizationSnapshot<'a> {
    /// Bind one validated request to daemon-owned trusted authorization state.
    pub(crate) fn new(
        request: ValidatedCapabilityRequest,
        mandatory_authorities: MandatoryAuthorityInputs<'a>,
        consequence_facts: TrustedConsequenceFacts,
        trusted_now: kernux_policy::CanonicalUtcSecond,
        trusted_extensions: &'a [&'a str],
        hierarchical_resource: bool,
        audit_event: &'a EventAppend,
    ) -> Self {
        Self {
            request,
            mandatory_authorities,
            consequence_facts,
            trusted_now,
            trusted_extensions,
            hierarchical_resource,
            audit_event,
        }
    }
}

/// Result of the daemon authorization gate immediately before any side effect.
///
/// Admitted means the Grant and ancestor use budget plus required allow audit linkage
/// already committed. A later operation failure must not refund that admission.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PreSideEffectAuthorization {
    Admitted {
        grant_id: String,
        effective_constraints: ValidatedConstraints,
    },
    Denied(kernux_policy::DenyReason),
}

/// Fail-closed infrastructure or trusted-input failure at the authorization boundary.
#[derive(Debug)]
pub enum DaemonAuthorizationError {
    DurableDecision,
}

impl fmt::Display for DaemonAuthorizationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DurableDecision => f.write_str("durable authorization decision failed"),
        }
    }
}

impl std::error::Error for DaemonAuthorizationError {}

/// Evaluate and durably commit the final pre-side-effect authorization boundary.
///
/// No host operation is performed here. Only an Admitted result may proceed to a later
/// side-effect executor.
pub fn authorize_pre_side_effect(
    store: &mut Store,
    grant_id: CanonicalId,
    snapshot: TrustedAuthorizationSnapshot<'_>,
) -> Result<PreSideEffectAuthorization, DaemonAuthorizationError> {
    let authoritative_consequence =
        match classify_consequence(&snapshot.request.action, snapshot.consequence_facts) {
            Ok(value) => value,
            Err(_) => {
                let decision = store
                    .record_policy_denial(
                        &snapshot.request,
                        kernux_policy::DenyReason::UnknownInput,
                        snapshot.audit_event,
                        snapshot.trusted_now,
                    )
                    .map_err(|_| DaemonAuthorizationError::DurableDecision)?;
                let PolicyDecision::Deny(reason) = decision else {
                    return Err(DaemonAuthorizationError::DurableDecision);
                };
                return Ok(PreSideEffectAuthorization::Denied(reason));
            }
        };

    let decision = store
        .decide_grant_use(
            grant_id,
            &snapshot.request,
            PolicyDecisionContext {
                mandatory_authorities: snapshot.mandatory_authorities,
                authoritative_consequence,
                trusted_now: snapshot.trusted_now,
                trusted_extensions: snapshot.trusted_extensions,
                hierarchical_resource: snapshot.hierarchical_resource,
                audit_event: snapshot.audit_event,
            },
        )
        .map_err(|_| DaemonAuthorizationError::DurableDecision)?;

    Ok(match decision {
        PolicyDecision::Allow {
            grant_id,
            effective_constraints,
        } => PreSideEffectAuthorization::Admitted {
            grant_id,
            effective_constraints,
        },
        PolicyDecision::Deny(reason) => PreSideEffectAuthorization::Denied(reason),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        path::PathBuf,
        sync::atomic::{AtomicU64, Ordering},
    };

    use kernux_contracts::{CapabilityVersion, RuntimeCapability};
    use kernux_policy::{
        Action, CanonicalResource, CanonicalUtcSecond, ConsequenceClass, DenyReason,
        OperationLocation, RemoteHostDecision, RuntimeCapabilityEvidence,
        SecondaryAuthorityDecision, TrustedAuthorityDecision, ValidatedGrant,
        ValidatedSubjectScope,
    };
    use kernux_store::{
        EventType, ImmutableEntity, Revision, RevisionedEntity, StreamOwnerKind, StreamRef,
    };

    const RUN: &str = "01890f00-0000-7000-8000-000000000102";
    const SESSION: &str = "01890f00-0000-7000-8000-000000000103";
    const PROJECT: &str = "01890f00-0000-7000-8000-000000000104";
    const RUNTIME: &str = "01890f00-0000-7000-8000-000000000105";
    const GRANT: &str = "01890f00-0000-7000-8000-000000000107";
    const ISSUE_EVENT: &str = "01890f00-0000-7000-8000-000000000108";
    const REQUEST: &str = "01890f00-0000-7000-8000-000000000111";

    static NEXT_DB: AtomicU64 = AtomicU64::new(1);

    struct TestDb {
        path: PathBuf,
    }

    impl TestDb {
        fn new(name: &str) -> Self {
            let serial = NEXT_DB.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "kernuxd-auth-{name}-{}-{serial}.sqlite",
                std::process::id()
            ));
            let _ = fs::remove_file(&path);
            let _ = fs::remove_file(format!("{}-wal", path.display()));
            let _ = fs::remove_file(format!("{}-shm", path.display()));
            Self { path }
        }
    }

    impl Drop for TestDb {
        fn drop(&mut self) {
            let _ = fs::remove_file(&self.path);
            let _ = fs::remove_file(format!("{}-wal", self.path.display()));
            let _ = fs::remove_file(format!("{}-shm", self.path.display()));
        }
    }

    fn prepare_store(name: &str) -> (TestDb, Store, CanonicalId) {
        let db = TestDb::new(name);
        let mut store = Store::open(&db.path).unwrap();
        let run = CanonicalId::parse(RUN).unwrap();
        store.insert_immutable(ImmutableEntity::Run, run).unwrap();
        store
            .insert_immutable(
                ImmutableEntity::AgentSession,
                CanonicalId::parse(SESSION).unwrap(),
            )
            .unwrap();
        let runtime = CanonicalId::parse(RUNTIME).unwrap();
        store
            .insert_revisioned(RevisionedEntity::Runtime, runtime)
            .unwrap();
        store
            .compare_and_swap_revision(RevisionedEntity::Runtime, runtime, Revision::INITIAL)
            .unwrap();

        let issue_event = CanonicalId::parse(ISSUE_EVENT).unwrap();
        store
            .append_event(&EventAppend::new(
                issue_event,
                EventType::parse("grant.issued").unwrap(),
                StreamRef::new(StreamOwnerKind::Run, run, None).unwrap(),
                None,
            ))
            .unwrap();
        store
            .persist_validated_grant(&grant(), issue_event)
            .unwrap();
        (db, store, issue_event)
    }

    fn grant() -> ValidatedGrant {
        ValidatedGrant {
            grant_id: GRANT.into(),
            subject: ValidatedSubjectScope {
                run_id: RUN.into(),
                agent_session_id: Some(SESSION.into()),
            },
            action: Action::parse("files.write", &[]).unwrap(),
            resource: CanonicalResource::parse(&format!("kernux://project/{PROJECT}/fs/src"))
                .unwrap(),
            resource_uri: format!("kernux://project/{PROJECT}/fs/src"),
            resource_scope: kernux_policy::ResourceScope::Subtree,
            runtime_id: RUNTIME.into(),
            runtime_revision: 2,
            constraints: ValidatedConstraints::from_parts(
                Some(vec!["src".into()]),
                Some(4096),
                None,
                Some(1),
                None,
            )
            .unwrap(),
            consequence_ceiling: ConsequenceClass::C2,
            issuer_authority: "local-user".into(),
            policy_revision: 1,
            issued_at: CanonicalUtcSecond::parse("2026-09-20T01:00:00Z").unwrap(),
            not_before: CanonicalUtcSecond::parse("2026-09-20T01:00:00Z").unwrap(),
            expires_at: CanonicalUtcSecond::parse("2026-09-20T02:00:00Z").unwrap(),
            max_uses: 1,
            delegation_depth: 0,
            parent_grant_id: None,
        }
    }

    fn request() -> ValidatedCapabilityRequest {
        ValidatedCapabilityRequest {
            request_id: REQUEST.into(),
            subject: ValidatedSubjectScope {
                run_id: RUN.into(),
                agent_session_id: Some(SESSION.into()),
            },
            action: Action::parse("files.write", &[]).unwrap(),
            resource: CanonicalResource::parse(&format!(
                "kernux://project/{PROJECT}/fs/src/main.rs"
            ))
            .unwrap(),
            runtime_id: RUNTIME.into(),
            runtime_revision: 2,
            constraints: ValidatedConstraints::from_parts(
                Some(vec!["src".into()]),
                Some(1024),
                None,
                None,
                None,
            )
            .unwrap(),
            provenance_event_ids: Vec::new(),
        }
    }

    fn runtime_capability(action: &str) -> RuntimeCapability {
        RuntimeCapability {
            action: action.into(),
            features: Vec::new(),
            version: CapabilityVersion::V1,
        }
    }

    fn authorities<'a>(capability: &'a RuntimeCapability) -> MandatoryAuthorityInputs<'a> {
        MandatoryAuthorityInputs {
            controller: TrustedAuthorityDecision::Allow,
            local_policy: TrustedAuthorityDecision::Allow,
            kernel: TrustedAuthorityDecision::Allow,
            principal_policy: TrustedAuthorityDecision::Allow,
            runtime_capability: RuntimeCapabilityEvidence::Negotiated(capability),
            location: OperationLocation::Local,
            remote_host: RemoteHostDecision::NotApplicable,
            secondary_authority: SecondaryAuthorityDecision::NotRequired,
        }
    }

    fn audit_event(event_id: &str, event_type: &str, predecessor: CanonicalId) -> EventAppend {
        EventAppend::new(
            CanonicalId::parse(event_id).unwrap(),
            EventType::parse(event_type).unwrap(),
            StreamRef::new(StreamOwnerKind::Run, CanonicalId::parse(RUN).unwrap(), None).unwrap(),
            Some(predecessor),
        )
    }

    #[test]
    fn admitted_result_commits_budget_and_audit_before_any_side_effect() {
        let (_db, mut store, issue_event) = prepare_store("allow");
        let capability = runtime_capability("files.write");
        let audit = audit_event(
            "01890f00-0000-7000-8000-000000000121",
            "capability.approved",
            issue_event,
        );
        let now = CanonicalUtcSecond::parse("2026-09-20T01:10:00Z").unwrap();
        let decision = authorize_pre_side_effect(
            &mut store,
            CanonicalId::parse(GRANT).unwrap(),
            TrustedAuthorizationSnapshot::new(
                request(),
                authorities(&capability),
                TrustedConsequenceFacts::default(),
                now,
                &[],
                true,
                &audit,
            ),
        )
        .unwrap();

        assert!(matches!(
            decision,
            PreSideEffectAuthorization::Admitted { .. }
        ));
        assert_eq!(
            store
                .persisted_grant(CanonicalId::parse(GRANT).unwrap(), &[], true)
                .unwrap()
                .used_count(),
            1
        );
    }

    #[test]
    fn trusted_consequence_escalation_denies_without_consuming_budget() {
        let (_db, mut store, issue_event) = prepare_store("consequence");
        let capability = runtime_capability("files.write");
        let audit = audit_event(
            "01890f00-0000-7000-8000-000000000122",
            "capability.denied",
            issue_event,
        );
        let now = CanonicalUtcSecond::parse("2026-09-20T01:10:00Z").unwrap();
        let decision = authorize_pre_side_effect(
            &mut store,
            CanonicalId::parse(GRANT).unwrap(),
            TrustedAuthorizationSnapshot::new(
                request(),
                authorities(&capability),
                TrustedConsequenceFacts {
                    generated_or_untrusted_host_code: true,
                    ..TrustedConsequenceFacts::default()
                },
                now,
                &[],
                true,
                &audit,
            ),
        )
        .unwrap();

        assert_eq!(
            decision,
            PreSideEffectAuthorization::Denied(DenyReason::ConsequenceExceeded)
        );
        assert_eq!(
            store
                .persisted_grant(CanonicalId::parse(GRANT).unwrap(), &[], true)
                .unwrap()
                .used_count(),
            0
        );
    }

    #[test]
    fn remote_host_deny_fails_before_budget_consumption() {
        let (_db, mut store, issue_event) = prepare_store("remote-deny");
        let capability = runtime_capability("files.write");
        let mut mandatory = authorities(&capability);
        mandatory.location = OperationLocation::Remote;
        mandatory.remote_host = RemoteHostDecision::Deny;
        let audit = audit_event(
            "01890f00-0000-7000-8000-000000000123",
            "capability.denied",
            issue_event,
        );
        let now = CanonicalUtcSecond::parse("2026-09-20T01:10:00Z").unwrap();

        assert_eq!(
            authorize_pre_side_effect(
                &mut store,
                CanonicalId::parse(GRANT).unwrap(),
                TrustedAuthorizationSnapshot::new(
                    request(),
                    mandatory,
                    TrustedConsequenceFacts::default(),
                    now,
                    &[],
                    true,
                    &audit,
                ),
            )
            .unwrap(),
            PreSideEffectAuthorization::Denied(DenyReason::RemoteHostDenied)
        );
        assert_eq!(
            store
                .persisted_grant(CanonicalId::parse(GRANT).unwrap(), &[], true)
                .unwrap()
                .used_count(),
            0
        );
    }

    #[test]
    fn unknown_trusted_consequence_is_audited_as_deny_without_budget_use() {
        let (_db, mut store, issue_event) = prepare_store("unknown-consequence");
        let mut extension_request = request();
        extension_request.action = Action::parse("ext.example.run", &["ext.example.run"]).unwrap();
        let capability = runtime_capability("ext.example.run");
        let audit = audit_event(
            "01890f00-0000-7000-8000-000000000125",
            "capability.denied",
            issue_event,
        );
        let now = CanonicalUtcSecond::parse("2026-09-20T01:10:00Z").unwrap();

        assert_eq!(
            authorize_pre_side_effect(
                &mut store,
                CanonicalId::parse(GRANT).unwrap(),
                TrustedAuthorizationSnapshot::new(
                    extension_request,
                    authorities(&capability),
                    TrustedConsequenceFacts::default(),
                    now,
                    &["ext.example.run"],
                    true,
                    &audit,
                ),
            )
            .unwrap(),
            PreSideEffectAuthorization::Denied(DenyReason::UnknownInput)
        );
        assert_eq!(
            store
                .persisted_grant(CanonicalId::parse(GRANT).unwrap(), &[], true)
                .unwrap()
                .used_count(),
            0
        );
    }

    #[test]
    fn audit_failure_never_returns_admitted_or_consumes_budget() {
        let (_db, mut store, issue_event) = prepare_store("audit-fail");
        let capability = runtime_capability("files.write");
        let wrong_audit = audit_event(
            "01890f00-0000-7000-8000-000000000124",
            "capability.denied",
            issue_event,
        );
        let now = CanonicalUtcSecond::parse("2026-09-20T01:10:00Z").unwrap();

        assert!(matches!(
            authorize_pre_side_effect(
                &mut store,
                CanonicalId::parse(GRANT).unwrap(),
                TrustedAuthorizationSnapshot::new(
                    request(),
                    authorities(&capability),
                    TrustedConsequenceFacts::default(),
                    now,
                    &[],
                    true,
                    &wrong_audit,
                ),
            ),
            Err(DaemonAuthorizationError::DurableDecision)
        ));
        assert_eq!(
            store
                .persisted_grant(CanonicalId::parse(GRANT).unwrap(), &[], true)
                .unwrap()
                .used_count(),
            0
        );
    }
}
