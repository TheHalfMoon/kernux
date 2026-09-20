use kernux_policy::{
    Action, CanonicalResource, CanonicalUtcSecond, ConsequenceClass, DelegationDecision,
    DenyReason, GrantMatchDecision, GrantRevocationReason, GrantUseDecision, ResourceScope,
    TrustedAuthorityDecision, ValidatedCapabilityRequest, ValidatedConstraints, ValidatedGrant,
    ValidatedSubjectScope, evaluate_delegation, evaluate_grant_match, validate_action_resource,
};
use rusqlite::{OptionalExtension, TransactionBehavior, params};

use crate::{CanonicalId, Store, StoreError};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PersistedGrant {
    grant: ValidatedGrant,
    issued_event_id: CanonicalId,
    used_count: u64,
}

impl PersistedGrant {
    pub fn grant(&self) -> &ValidatedGrant {
        &self.grant
    }

    pub const fn issued_event_id(&self) -> CanonicalId {
        self.issued_event_id
    }

    pub const fn used_count(&self) -> u64 {
        self.used_count
    }
}

#[derive(Clone, Copy, Debug)]
pub struct DelegationPersistenceContext<'a> {
    pub trusted_now: CanonicalUtcSecond,
    pub delegate_authority: TrustedAuthorityDecision,
    pub recipient_authority: TrustedAuthorityDecision,
    pub trusted_extensions: &'a [&'a str],
    pub hierarchical_resource: bool,
}

impl Store {
    pub fn persist_validated_grant(
        &mut self,
        grant: &ValidatedGrant,
        issued_event_id: CanonicalId,
    ) -> Result<(), StoreError> {
        if grant.parent_grant_id.is_some() {
            return Err(StoreError::InvalidGrantDelegation);
        }
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| StoreError::Database)?;
        persist_grant_tx(&transaction, grant, issued_event_id, "grant.issued")?;
        transaction.commit().map_err(|_| StoreError::Database)
    }

    pub fn persist_delegated_grant(
        &mut self,
        child: &ValidatedGrant,
        delegated_event_id: CanonicalId,
        context: DelegationPersistenceContext<'_>,
    ) -> Result<DelegationDecision, StoreError> {
        let parent_text = child
            .parent_grant_id
            .as_deref()
            .ok_or(StoreError::InvalidGrantDelegation)?;
        let parent_id = CanonicalId::parse(parent_text)?;
        let parent = self.persisted_grant(
            parent_id,
            context.trusted_extensions,
            context.hierarchical_resource,
        )?;
        let decision = evaluate_delegation(
            parent.grant(),
            child,
            context.delegate_authority,
            context.recipient_authority,
            context.hierarchical_resource,
        );
        if decision != DelegationDecision::Eligible {
            return Ok(decision);
        }

        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| StoreError::Database)?;
        if grant_revoked(&transaction, parent_id)? {
            return Ok(DelegationDecision::Denied(DenyReason::GrantRevoked));
        }
        if context.trusted_now < parent.grant().not_before {
            return Ok(DelegationDecision::Denied(DenyReason::GrantNotYetActive));
        }
        if context.trusted_now >= parent.grant().expires_at {
            return Ok(DelegationDecision::Denied(DenyReason::GrantExpired));
        }
        let used_count = grant_used_count(&transaction, parent_id)?;
        if used_count > parent.grant().max_uses {
            return Err(StoreError::GrantStateCorrupt);
        }
        if used_count == parent.grant().max_uses {
            return Ok(DelegationDecision::Denied(DenyReason::GrantExhausted));
        }
        let remaining = parent.grant().max_uses - used_count;
        if child.max_uses > remaining {
            return Ok(DelegationDecision::Denied(DenyReason::DelegationDenied));
        }

        persist_grant_tx(&transaction, child, delegated_event_id, "grant.delegated")?;
        transaction.commit().map_err(|_| StoreError::Database)?;
        Ok(DelegationDecision::Eligible)
    }

    pub fn persisted_grant(
        &self,
        grant_id: CanonicalId,
        trusted_extensions: &[&str],
        hierarchical_resource: bool,
    ) -> Result<PersistedGrant, StoreError> {
        let row = self
            .connection
            .query_row(
                "SELECT i.run_id, i.agent_session_id, i.action, i.resource_uri, i.resource_match, \
                        i.runtime_id, i.runtime_revision, i.consequence_ceiling, i.issuer_authority, \
                        i.policy_revision, i.issued_at, i.not_before, i.expires_at, i.max_uses, \
                        i.delegation_depth, i.parent_grant_id, i.issued_event_id, \
                        c.max_bytes, c.max_duration_ms, c.max_uses, s.used_count \
                 FROM grant_issuance i \
                 JOIN grant_constraints c ON c.grant_id = i.grant_id \
                 JOIN grant_state s ON s.grant_id = i.grant_id \
                 WHERE i.grant_id = ?1",
                [grant_id.to_canonical_text()],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, Option<String>>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                        row.get::<_, String>(4)?,
                        row.get::<_, String>(5)?,
                        row.get::<_, i64>(6)?,
                        row.get::<_, String>(7)?,
                        row.get::<_, String>(8)?,
                        row.get::<_, Vec<u8>>(9)?,
                        row.get::<_, String>(10)?,
                        row.get::<_, String>(11)?,
                        row.get::<_, String>(12)?,
                        row.get::<_, Vec<u8>>(13)?,
                        row.get::<_, Vec<u8>>(14)?,
                        row.get::<_, Option<String>>(15)?,
                        row.get::<_, String>(16)?,
                        row.get::<_, Option<Vec<u8>>>(17)?,
                        row.get::<_, Option<Vec<u8>>>(18)?,
                        row.get::<_, Option<Vec<u8>>>(19)?,
                        row.get::<_, Vec<u8>>(20)?,
                    ))
                },
            )
            .optional()
            .map_err(|_| StoreError::GrantStateCorrupt)?
            .ok_or(StoreError::NotFound)?;

        let allowed_roots = read_set(&self.connection, "grant_allowed_roots", grant_id)?;
        let network_hosts = read_set(&self.connection, "grant_network_hosts", grant_id)?;
        let action =
            Action::parse(&row.2, trusted_extensions).map_err(|_| StoreError::GrantStateCorrupt)?;
        let resource =
            CanonicalResource::parse(&row.3).map_err(|_| StoreError::GrantStateCorrupt)?;
        validate_action_resource(&action, &resource).map_err(|_| StoreError::GrantStateCorrupt)?;
        let resource_scope = match row.4.as_str() {
            "exact" => ResourceScope::Exact,
            "subtree" if hierarchical_resource => ResourceScope::Subtree,
            _ => return Err(StoreError::GrantStateCorrupt),
        };
        let runtime_revision = u32::try_from(row.6)
            .ok()
            .filter(|value| *value > 0)
            .ok_or(StoreError::GrantStateCorrupt)?;
        CanonicalId::parse(&row.0).map_err(|_| StoreError::GrantStateCorrupt)?;
        if let Some(agent) = &row.1 {
            CanonicalId::parse(agent).map_err(|_| StoreError::GrantStateCorrupt)?;
        }
        CanonicalId::parse(&row.5).map_err(|_| StoreError::GrantStateCorrupt)?;
        let parent_grant_id = row
            .15
            .map(|value| {
                CanonicalId::parse(&value)
                    .map(|_| value)
                    .map_err(|_| StoreError::GrantStateCorrupt)
            })
            .transpose()?;
        let issued_event_id =
            CanonicalId::parse(&row.16).map_err(|_| StoreError::GrantStateCorrupt)?;
        let constraints = ValidatedConstraints::from_parts(
            allowed_roots,
            decode_optional(row.17.as_deref())?,
            decode_optional(row.18.as_deref())?,
            decode_optional(row.19.as_deref())?,
            network_hosts,
        )
        .map_err(|_| StoreError::GrantStateCorrupt)?;
        let max_uses = decode_u64(&row.13)?;
        let policy_revision = decode_u64(&row.9)?;
        let delegation_depth = decode_u64(&row.14)?;
        let used_count = decode_u64(&row.20)?;
        let issued_at =
            CanonicalUtcSecond::parse(&row.10).map_err(|_| StoreError::GrantStateCorrupt)?;
        let not_before =
            CanonicalUtcSecond::parse(&row.11).map_err(|_| StoreError::GrantStateCorrupt)?;
        let expires_at =
            CanonicalUtcSecond::parse(&row.12).map_err(|_| StoreError::GrantStateCorrupt)?;
        if max_uses == 0
            || policy_revision == 0
            || used_count > max_uses
            || constraints.max_uses.is_some_and(|value| value != max_uses)
            || issued_at > not_before
            || issued_at >= expires_at
            || not_before >= expires_at
            || row.8.is_empty()
            || row.8.len() > 128
            || row.8.trim() != row.8
            || row.8.chars().any(char::is_control)
            || parent_grant_id.as_deref() == Some(&grant_id.to_canonical_text())
        {
            return Err(StoreError::GrantStateCorrupt);
        }
        let consequence_ceiling = parse_consequence(&row.7)?;
        Ok(PersistedGrant {
            grant: ValidatedGrant {
                grant_id: grant_id.to_canonical_text(),
                subject: ValidatedSubjectScope {
                    run_id: row.0,
                    agent_session_id: row.1,
                },
                action,
                resource,
                resource_uri: row.3,
                resource_scope,
                runtime_id: row.5,
                runtime_revision,
                constraints,
                consequence_ceiling,
                issuer_authority: row.8,
                policy_revision,
                issued_at,
                not_before,
                expires_at,
                max_uses,
                delegation_depth,
                parent_grant_id,
            },
            issued_event_id,
            used_count,
        })
    }
    pub fn consume_grant_use(
        &mut self,
        grant_id: CanonicalId,
        request: &ValidatedCapabilityRequest,
        authoritative_consequence: ConsequenceClass,
        trusted_now: CanonicalUtcSecond,
        trusted_extensions: &[&str],
        hierarchical_resource: bool,
    ) -> Result<GrantUseDecision, StoreError> {
        let persisted =
            match self.persisted_grant(grant_id, trusted_extensions, hierarchical_resource) {
                Ok(value) => value,
                Err(StoreError::NotFound) => {
                    return Ok(GrantUseDecision::Denied(DenyReason::GrantNotFound));
                }
                Err(error) => return Err(error),
            };
        let grant = persisted.grant();
        if grant.parent_grant_id.is_some() {
            return Ok(GrantUseDecision::Denied(DenyReason::DelegationDenied));
        }
        let effective_constraints = match evaluate_grant_match(
            grant,
            request,
            authoritative_consequence,
            trusted_now,
            hierarchical_resource,
        ) {
            GrantMatchDecision::Eligible {
                effective_constraints,
            } => effective_constraints,
            GrantMatchDecision::Denied(reason) => {
                return Ok(GrantUseDecision::Denied(reason));
            }
        };

        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| StoreError::Database)?;
        if grant_revoked(&transaction, grant_id)? {
            return Ok(GrantUseDecision::Denied(DenyReason::GrantRevoked));
        }
        let used_count = grant_used_count(&transaction, grant_id)?;
        if used_count > grant.max_uses {
            return Err(StoreError::GrantStateCorrupt);
        }
        if used_count == grant.max_uses {
            return Ok(GrantUseDecision::Denied(DenyReason::GrantExhausted));
        }
        let next_used = used_count
            .checked_add(1)
            .ok_or(StoreError::GrantStateCorrupt)?;
        let changed = transaction
            .execute(
                "UPDATE grant_state SET used_count = ?1 WHERE grant_id = ?2",
                params![
                    encode_u64(next_used).as_slice(),
                    grant_id.to_canonical_text()
                ],
            )
            .map_err(|_| StoreError::Database)?;
        if changed != 1 {
            return Err(StoreError::GrantStateCorrupt);
        }
        transaction.commit().map_err(|_| StoreError::Database)?;

        Ok(GrantUseDecision::BudgetConsumed {
            grant_id: grant_id.to_canonical_text(),
            effective_constraints,
        })
    }

    pub fn revoke_grant(
        &mut self,
        grant_id: CanonicalId,
        revoked_at: CanonicalUtcSecond,
        revoked_event_id: CanonicalId,
        reason: GrantRevocationReason,
    ) -> Result<(), StoreError> {
        let issuance = self
            .connection
            .query_row(
                "SELECT run_id, issued_at FROM grant_issuance WHERE grant_id = ?1",
                [grant_id.to_canonical_text()],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
            )
            .optional()
            .map_err(|_| StoreError::Database)?
            .ok_or(StoreError::NotFound)?;
        let run_id = CanonicalId::parse(&issuance.0).map_err(|_| StoreError::GrantStateCorrupt)?;
        let issued_at =
            CanonicalUtcSecond::parse(&issuance.1).map_err(|_| StoreError::GrantStateCorrupt)?;
        if revoked_at < issued_at {
            return Err(StoreError::InvalidGrantAudit);
        }

        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| StoreError::Database)?;
        let audit = transaction
            .query_row(
                "SELECT event_type, owner_kind, owner_id FROM events WHERE id = ?1",
                [revoked_event_id.to_canonical_text()],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                    ))
                },
            )
            .optional()
            .map_err(|_| StoreError::Database)?
            .ok_or(StoreError::InvalidGrantAudit)?;
        if audit
            != (
                "grant.revoked".to_owned(),
                "run".to_owned(),
                run_id.to_canonical_text(),
            )
        {
            return Err(StoreError::InvalidGrantAudit);
        }

        transaction
            .execute(
                "INSERT INTO grant_revocations(grant_id, revoked_at, event_id, reason_class)                  VALUES (?1, ?2, ?3, ?4)",
                params![
                    grant_id.to_canonical_text(),
                    revoked_at.to_canonical_text(),
                    revoked_event_id.to_canonical_text(),
                    reason.as_str(),
                ],
            )
            .map_err(|error| match error {
                rusqlite::Error::SqliteFailure(inner, _)
                    if inner.code == rusqlite::ErrorCode::ConstraintViolation =>
                {
                    StoreError::DuplicateConflict
                }
                _ => StoreError::Database,
            })?;
        transaction.commit().map_err(|_| StoreError::Database)
    }
}

fn persist_grant_tx(
    transaction: &rusqlite::Transaction<'_>,
    grant: &ValidatedGrant,
    audit_event_id: CanonicalId,
    expected_event_type: &str,
) -> Result<(), StoreError> {
    let grant_id = CanonicalId::parse(&grant.grant_id)?;
    let run_id = CanonicalId::parse(&grant.subject.run_id)?;

    let runtime_revision = transaction
        .query_row(
            "SELECT revision FROM runtimes WHERE id = ?1",
            [&grant.runtime_id],
            |row| row.get::<_, i64>(0),
        )
        .optional()
        .map_err(|_| StoreError::Database)?
        .ok_or(StoreError::NotFound)?;
    if runtime_revision != i64::from(grant.runtime_revision) {
        return Err(StoreError::RevisionConflict);
    }

    let audit = transaction
        .query_row(
            "SELECT event_type, owner_kind, owner_id FROM events WHERE id = ?1",
            [audit_event_id.to_canonical_text()],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                ))
            },
        )
        .optional()
        .map_err(|_| StoreError::Database)?
        .ok_or(StoreError::InvalidGrantAudit)?;
    if audit
        != (
            expected_event_type.to_owned(),
            "run".to_owned(),
            run_id.to_canonical_text(),
        )
    {
        return Err(StoreError::InvalidGrantAudit);
    }

    transaction
        .execute(
            "INSERT INTO grants(id, revision) VALUES (?1, 1)",
            [grant_id.to_canonical_text()],
        )
        .map_err(|error| match error {
            rusqlite::Error::SqliteFailure(inner, _)
                if inner.code == rusqlite::ErrorCode::ConstraintViolation =>
            {
                StoreError::DuplicateConflict
            }
            _ => StoreError::Database,
        })?;

    transaction
        .execute(
            "INSERT INTO grant_issuance(                grant_id, run_id, agent_session_id, action, resource_uri, resource_match,                 runtime_id, runtime_revision, consequence_ceiling, issuer_authority,                 policy_revision, issued_at, not_before, expires_at, max_uses,                 delegation_depth, parent_grant_id, issued_event_id             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18)",
            params![
                grant_id.to_canonical_text(),
                grant.subject.run_id,
                grant.subject.agent_session_id,
                grant.action.as_str(),
                grant.resource_uri,
                scope_text(grant.resource_scope),
                grant.runtime_id,
                i64::from(grant.runtime_revision),
                grant.consequence_ceiling.as_str(),
                grant.issuer_authority,
                encode_u64(grant.policy_revision).as_slice(),
                grant.issued_at.to_canonical_text(),
                grant.not_before.to_canonical_text(),
                grant.expires_at.to_canonical_text(),
                encode_u64(grant.max_uses).as_slice(),
                encode_u64(grant.delegation_depth).as_slice(),
                grant.parent_grant_id,
                audit_event_id.to_canonical_text(),
            ],
        )
        .map_err(|_| StoreError::Database)?;

    let max_bytes = grant.constraints.max_bytes.map(encode_u64);
    let max_duration = grant.constraints.max_duration_ms.map(encode_u64);
    let constraint_uses = grant.constraints.max_uses.map(encode_u64);
    transaction
        .execute(
            "INSERT INTO grant_constraints(grant_id, max_bytes, max_duration_ms, max_uses)              VALUES (?1, ?2, ?3, ?4)",
            params![
                grant_id.to_canonical_text(),
                max_bytes.as_ref().map(|value| value.as_slice()),
                max_duration.as_ref().map(|value| value.as_slice()),
                constraint_uses.as_ref().map(|value| value.as_slice()),
            ],
        )
        .map_err(|_| StoreError::Database)?;

    insert_set(
        transaction,
        "grant_allowed_roots",
        grant_id,
        grant.constraints.allowed_roots.as_deref(),
    )?;
    insert_set(
        transaction,
        "grant_network_hosts",
        grant_id,
        grant.constraints.network_hosts.as_deref(),
    )?;
    transaction
        .execute(
            "INSERT INTO grant_state(grant_id, used_count) VALUES (?1, ?2)",
            params![grant_id.to_canonical_text(), encode_u64(0).as_slice()],
        )
        .map_err(|_| StoreError::Database)?;
    Ok(())
}

fn grant_revoked(
    transaction: &rusqlite::Transaction<'_>,
    grant_id: CanonicalId,
) -> Result<bool, StoreError> {
    transaction
        .query_row(
            "SELECT 1 FROM grant_revocations WHERE grant_id = ?1",
            [grant_id.to_canonical_text()],
            |_| Ok(()),
        )
        .optional()
        .map(|value| value.is_some())
        .map_err(|_| StoreError::Database)
}

fn grant_used_count(
    transaction: &rusqlite::Transaction<'_>,
    grant_id: CanonicalId,
) -> Result<u64, StoreError> {
    let value = transaction
        .query_row(
            "SELECT used_count FROM grant_state WHERE grant_id = ?1",
            [grant_id.to_canonical_text()],
            |row| row.get::<_, Vec<u8>>(0),
        )
        .optional()
        .map_err(|_| StoreError::Database)?
        .ok_or(StoreError::GrantStateCorrupt)?;
    decode_u64(&value)
}

fn insert_set(
    transaction: &rusqlite::Transaction<'_>,
    table: &str,
    grant_id: CanonicalId,
    values: Option<&[String]>,
) -> Result<(), StoreError> {
    let Some(values) = values else { return Ok(()) };
    let sql = format!("INSERT INTO {table}(grant_id, value) VALUES (?1, ?2)");
    for value in values {
        transaction
            .execute(&sql, params![grant_id.to_canonical_text(), value])
            .map_err(|_| StoreError::Database)?;
    }
    Ok(())
}

fn read_set(
    connection: &rusqlite::Connection,
    table: &str,
    grant_id: CanonicalId,
) -> Result<Option<Vec<String>>, StoreError> {
    let sql = format!("SELECT value FROM {table} WHERE grant_id = ?1 ORDER BY value");
    let mut statement = connection
        .prepare(&sql)
        .map_err(|_| StoreError::GrantStateCorrupt)?;
    let values = statement
        .query_map([grant_id.to_canonical_text()], |row| {
            row.get::<_, String>(0)
        })
        .map_err(|_| StoreError::GrantStateCorrupt)?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| StoreError::GrantStateCorrupt)?;
    Ok((!values.is_empty()).then_some(values))
}

fn scope_text(value: ResourceScope) -> &'static str {
    match value {
        ResourceScope::Exact => "exact",
        ResourceScope::Subtree => "subtree",
    }
}

fn parse_consequence(value: &str) -> Result<ConsequenceClass, StoreError> {
    match value {
        "C0" => Ok(ConsequenceClass::C0),
        "C1" => Ok(ConsequenceClass::C1),
        "C2" => Ok(ConsequenceClass::C2),
        "C3" => Ok(ConsequenceClass::C3),
        "C4" => Ok(ConsequenceClass::C4),
        _ => Err(StoreError::GrantStateCorrupt),
    }
}

fn encode_u64(value: u64) -> [u8; 8] {
    value.to_be_bytes()
}

fn decode_u64(value: &[u8]) -> Result<u64, StoreError> {
    let bytes: [u8; 8] = value
        .try_into()
        .map_err(|_| StoreError::GrantStateCorrupt)?;
    Ok(u64::from_be_bytes(bytes))
}

fn decode_optional(value: Option<&[u8]>) -> Result<Option<u64>, StoreError> {
    value.map(decode_u64).transpose()
}

#[cfg(test)]
mod tests {
    use super::*;
    use kernux_policy::{CanonicalUtcSecond, ConsequenceClass, ValidatedConstraints};

    use crate::tests::TestDb;
    use crate::{
        EventAppend, EventType, ImmutableEntity, Revision, RevisionedEntity, StreamOwnerKind,
        StreamRef,
    };

    const RUN: &str = "01890f00-0000-7000-8000-000000000002";
    const SESSION: &str = "01890f00-0000-7000-8000-000000000003";
    const RUNTIME: &str = "01890f00-0000-7000-8000-000000000005";
    const GRANT: &str = "01890f00-0000-7000-8000-000000000007";
    const EVENT: &str = "01890f00-0000-7000-8000-000000000008";

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
        let event = CanonicalId::parse(EVENT).unwrap();
        store
            .append_event(&EventAppend::new(
                event,
                EventType::parse("grant.issued").unwrap(),
                StreamRef::new(StreamOwnerKind::Run, run, None).unwrap(),
                None,
            ))
            .unwrap();
        (db, store, event)
    }

    fn grant() -> ValidatedGrant {
        ValidatedGrant {
            grant_id: GRANT.into(),
            subject: ValidatedSubjectScope {
                run_id: RUN.into(),
                agent_session_id: Some(SESSION.into()),
            },
            action: Action::parse("files.write", &[]).unwrap(),
            resource: CanonicalResource::parse(
                "kernux://project/01890f00-0000-7000-8000-000000000004/fs/src",
            )
            .unwrap(),
            resource_uri: "kernux://project/01890f00-0000-7000-8000-000000000004/fs/src".into(),
            resource_scope: ResourceScope::Subtree,
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
            policy_revision: u64::MAX,
            issued_at: CanonicalUtcSecond::parse("2026-09-19T01:00:00Z").unwrap(),
            not_before: CanonicalUtcSecond::parse("2026-09-19T01:00:00Z").unwrap(),
            expires_at: CanonicalUtcSecond::parse("2026-09-19T01:05:00Z").unwrap(),
            max_uses: 1,
            delegation_depth: 0,
            parent_grant_id: None,
        }
    }

    #[test]
    fn validated_grant_persists_and_reopens_without_losing_u64_or_constraints() {
        let (db, mut store, event) = prepare_store("grant-reopen");
        let value = grant();
        store.persist_validated_grant(&value, event).unwrap();
        let id = CanonicalId::parse(GRANT).unwrap();
        let stored = store.persisted_grant(id, &[], true).unwrap();
        assert_eq!(stored.grant(), &value);
        assert_eq!(stored.issued_event_id(), event);
        assert_eq!(stored.used_count(), 0);
        drop(store);

        let reopened = Store::open(&db.path).unwrap();
        let stored = reopened.persisted_grant(id, &[], true).unwrap();
        assert_eq!(stored.grant(), &value);
        assert_eq!(stored.used_count(), 0);
    }

    #[test]
    fn duplicate_or_stale_runtime_grant_fails_without_partial_rows() {
        let (_db, mut store, event) = prepare_store("grant-failures");
        let value = grant();
        store.persist_validated_grant(&value, event).unwrap();
        assert_eq!(
            store.persist_validated_grant(&value, event),
            Err(StoreError::DuplicateConflict)
        );

        let mut other = value.clone();
        other.grant_id = "01890f00-0000-7000-8000-000000000009".into();
        other.runtime_revision = 1;
        assert_eq!(
            store.persist_validated_grant(&other, event),
            Err(StoreError::RevisionConflict)
        );
        assert!(
            !store
                .immutable_exists(
                    ImmutableEntity::Grant,
                    CanonicalId::parse(&other.grant_id).unwrap()
                )
                .unwrap()
        );
    }

    #[test]
    fn issue_requires_exact_grant_issued_event_owned_by_same_run() {
        let (_db, mut store, event) = prepare_store("grant-audit");
        let value = grant();
        let missing = CanonicalId::parse("01890f00-0000-7000-8000-000000000010").unwrap();
        assert_eq!(
            store.persist_validated_grant(&value, missing),
            Err(StoreError::InvalidGrantAudit)
        );
        assert!(
            !store
                .immutable_exists(ImmutableEntity::Grant, CanonicalId::parse(GRANT).unwrap())
                .unwrap()
        );
        store.persist_validated_grant(&value, event).unwrap();
    }

    fn request() -> ValidatedCapabilityRequest {
        ValidatedCapabilityRequest {
            request_id: "01890f00-0000-7000-8000-000000000011".into(),
            subject: ValidatedSubjectScope {
                run_id: RUN.into(),
                agent_session_id: Some(SESSION.into()),
            },
            action: Action::parse("files.write", &[]).unwrap(),
            resource: CanonicalResource::parse(
                "kernux://project/01890f00-0000-7000-8000-000000000004/fs/src/main.rs",
            )
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

    #[test]
    fn admission_consumes_once_and_restart_preserves_exhaustion() {
        let (db, mut store, event) = prepare_store("grant-admit");
        let value = grant();
        store.persist_validated_grant(&value, event).unwrap();
        let id = CanonicalId::parse(GRANT).unwrap();
        let now = CanonicalUtcSecond::parse("2026-09-19T01:01:00Z").unwrap();

        let admitted = store
            .consume_grant_use(id, &request(), ConsequenceClass::C2, now, &[], true)
            .unwrap();
        match admitted {
            GrantUseDecision::BudgetConsumed {
                effective_constraints,
                ..
            } => {
                assert_eq!(effective_constraints.max_bytes, Some(1024));
                assert_eq!(effective_constraints.max_uses, Some(1));
            }
            other => panic!("expected consumed-budget decision, got {other:?}"),
        }
        assert_eq!(
            store.persisted_grant(id, &[], true).unwrap().used_count(),
            1
        );
        assert_eq!(
            store
                .consume_grant_use(id, &request(), ConsequenceClass::C2, now, &[], true)
                .unwrap(),
            GrantUseDecision::Denied(DenyReason::GrantExhausted)
        );
        drop(store);

        let mut reopened = Store::open(&db.path).unwrap();
        assert_eq!(
            reopened
                .consume_grant_use(id, &request(), ConsequenceClass::C2, now, &[], true)
                .unwrap(),
            GrantUseDecision::Denied(DenyReason::GrantExhausted)
        );
    }

    #[test]
    fn concurrent_admissions_cannot_exceed_one_use_budget() {
        use std::sync::{Arc, Barrier};
        use std::thread;

        let (db, mut store, event) = prepare_store("grant-concurrent");
        store.persist_validated_grant(&grant(), event).unwrap();
        drop(store);

        let barrier = Arc::new(Barrier::new(3));
        let mut handles = Vec::new();
        for _ in 0..2 {
            let path = db.path.clone();
            let barrier = Arc::clone(&barrier);
            handles.push(thread::spawn(move || {
                let mut store = Store::open(&path).unwrap();
                let id = CanonicalId::parse(GRANT).unwrap();
                let now = CanonicalUtcSecond::parse("2026-09-19T01:01:00Z").unwrap();
                barrier.wait();
                store
                    .consume_grant_use(id, &request(), ConsequenceClass::C2, now, &[], true)
                    .unwrap()
            }));
        }
        barrier.wait();
        let decisions: Vec<_> = handles
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .collect();
        assert_eq!(
            decisions
                .iter()
                .filter(|decision| matches!(decision, GrantUseDecision::BudgetConsumed { .. }))
                .count(),
            1
        );
        assert_eq!(
            decisions
                .iter()
                .filter(|decision| {
                    **decision == GrantUseDecision::Denied(DenyReason::GrantExhausted)
                })
                .count(),
            1
        );

        let reopened = Store::open(&db.path).unwrap();
        assert_eq!(
            reopened
                .persisted_grant(CanonicalId::parse(GRANT).unwrap(), &[], true)
                .unwrap()
                .used_count(),
            1
        );
    }

    #[test]
    fn delegated_issuance_requires_authority_subset_and_dedicated_path() {
        let (_db, mut store, event) = prepare_store("grant-parent-deny");
        let mut parent = grant();
        parent.delegation_depth = 1;
        store.persist_validated_grant(&parent, event).unwrap();

        let child_event = CanonicalId::parse("01890f00-0000-7000-8000-000000000013").unwrap();
        store
            .append_event(&EventAppend::new(
                child_event,
                EventType::parse("grant.delegated").unwrap(),
                StreamRef::new(StreamOwnerKind::Run, CanonicalId::parse(RUN).unwrap(), None)
                    .unwrap(),
                Some(event),
            ))
            .unwrap();
        let mut child = grant();
        child.grant_id = "01890f00-0000-7000-8000-000000000014".into();
        child.parent_grant_id = Some(GRANT.into());

        assert_eq!(
            store.persist_validated_grant(&child, child_event),
            Err(StoreError::InvalidGrantDelegation)
        );
        let now = CanonicalUtcSecond::parse("2026-09-19T01:01:00Z").unwrap();
        assert_eq!(
            store
                .persist_delegated_grant(
                    &child,
                    child_event,
                    DelegationPersistenceContext {
                        trusted_now: now,
                        delegate_authority: TrustedAuthorityDecision::Unknown,
                        recipient_authority: TrustedAuthorityDecision::Allow,
                        trusted_extensions: &[],
                        hierarchical_resource: true,
                    },
                )
                .unwrap(),
            DelegationDecision::Denied(DenyReason::DelegationDenied)
        );
        assert_eq!(
            store
                .persist_delegated_grant(
                    &child,
                    child_event,
                    DelegationPersistenceContext {
                        trusted_now: now,
                        delegate_authority: TrustedAuthorityDecision::Allow,
                        recipient_authority: TrustedAuthorityDecision::Allow,
                        trusted_extensions: &[],
                        hierarchical_resource: true,
                    },
                )
                .unwrap(),
            DelegationDecision::Eligible
        );

        let child_id = CanonicalId::parse(&child.grant_id).unwrap();
        assert_eq!(
            store
                .consume_grant_use(child_id, &request(), ConsequenceClass::C2, now, &[], true,)
                .unwrap(),
            GrantUseDecision::Denied(DenyReason::DelegationDenied)
        );
        assert_eq!(
            store
                .persisted_grant(child_id, &[], true)
                .unwrap()
                .used_count(),
            0
        );
    }

    #[test]
    fn rejected_admission_does_not_consume_use_budget() {
        let (_db, mut store, event) = prepare_store("grant-reject");
        store.persist_validated_grant(&grant(), event).unwrap();
        let id = CanonicalId::parse(GRANT).unwrap();
        let now = CanonicalUtcSecond::parse("2026-09-19T01:01:00Z").unwrap();
        let mut wrong_action = request();
        wrong_action.action = Action::parse("files.delete", &[]).unwrap();

        assert_eq!(
            store
                .consume_grant_use(id, &wrong_action, ConsequenceClass::C2, now, &[], true)
                .unwrap(),
            GrantUseDecision::Denied(DenyReason::ActionMismatch)
        );
        assert_eq!(
            store
                .consume_grant_use(id, &request(), ConsequenceClass::C3, now, &[], true)
                .unwrap(),
            GrantUseDecision::Denied(DenyReason::ConsequenceExceeded)
        );
        assert_eq!(
            store.persisted_grant(id, &[], true).unwrap().used_count(),
            0
        );
    }

    #[test]
    fn time_window_and_revocation_block_new_admissions_without_consumption() {
        let (db, mut store, event) = prepare_store("grant-revoke");
        store.persist_validated_grant(&grant(), event).unwrap();
        let id = CanonicalId::parse(GRANT).unwrap();
        let before = CanonicalUtcSecond::parse("2026-09-19T00:59:59Z").unwrap();
        let expiry = CanonicalUtcSecond::parse("2026-09-19T01:05:00Z").unwrap();
        assert_eq!(
            store
                .consume_grant_use(id, &request(), ConsequenceClass::C2, before, &[], true)
                .unwrap(),
            GrantUseDecision::Denied(DenyReason::GrantNotYetActive)
        );
        assert_eq!(
            store
                .consume_grant_use(id, &request(), ConsequenceClass::C2, expiry, &[], true)
                .unwrap(),
            GrantUseDecision::Denied(DenyReason::GrantExpired)
        );

        let revoke_event = CanonicalId::parse("01890f00-0000-7000-8000-000000000012").unwrap();
        store
            .append_event(&EventAppend::new(
                revoke_event,
                EventType::parse("grant.revoked").unwrap(),
                StreamRef::new(StreamOwnerKind::Run, CanonicalId::parse(RUN).unwrap(), None)
                    .unwrap(),
                Some(event),
            ))
            .unwrap();
        let revoked_at = CanonicalUtcSecond::parse("2026-09-19T01:01:00Z").unwrap();
        store
            .revoke_grant(
                id,
                revoked_at,
                revoke_event,
                GrantRevocationReason::Security,
            )
            .unwrap();
        assert_eq!(
            store
                .consume_grant_use(id, &request(), ConsequenceClass::C2, revoked_at, &[], true,)
                .unwrap(),
            GrantUseDecision::Denied(DenyReason::GrantRevoked)
        );
        assert_eq!(
            store.persisted_grant(id, &[], true).unwrap().used_count(),
            0
        );
        drop(store);

        let mut reopened = Store::open(&db.path).unwrap();
        assert_eq!(
            reopened
                .consume_grant_use(id, &request(), ConsequenceClass::C2, revoked_at, &[], true,)
                .unwrap(),
            GrantUseDecision::Denied(DenyReason::GrantRevoked)
        );
    }

    #[test]
    fn corrupted_used_count_or_constraint_fails_closed_on_read() {
        let (_db, mut store, event) = prepare_store("grant-corrupt");
        let value = grant();
        store.persist_validated_grant(&value, event).unwrap();
        store
            .connection
            .execute(
                "UPDATE grant_state SET used_count = X'0000000000000002' WHERE grant_id = ?1",
                [GRANT],
            )
            .unwrap();
        assert_eq!(
            store.persisted_grant(CanonicalId::parse(GRANT).unwrap(), &[], true),
            Err(StoreError::GrantStateCorrupt)
        );
    }
}
