use std::fmt;
use std::str::FromStr;

use rusqlite::{Error as SqliteError, ErrorCode, OptionalExtension};
use uuid::{Uuid, Variant};

use crate::{Store, StoreError};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct CanonicalId(Uuid);

impl CanonicalId {
    pub fn parse(value: &str) -> Result<Self, StoreError> {
        let uuid = Uuid::parse_str(value).map_err(|_| StoreError::InvalidCanonicalId)?;
        if uuid.get_version_num() != 7
            || uuid.get_variant() != Variant::RFC4122
            || uuid.hyphenated().to_string() != value
        {
            return Err(StoreError::InvalidCanonicalId);
        }
        Ok(Self(uuid))
    }

    pub fn to_canonical_text(self) -> String {
        self.0.hyphenated().to_string()
    }
}

impl fmt::Debug for CanonicalId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CanonicalId({self})")
    }
}

impl fmt::Display for CanonicalId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.hyphenated())
    }
}

impl FromStr for CanonicalId {
    type Err = StoreError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Revision(u32);

impl Revision {
    pub const INITIAL: Self = Self(1);

    pub fn new(value: u32) -> Result<Self, StoreError> {
        if value == 0 {
            Err(StoreError::InvalidRevision)
        } else {
            Ok(Self(value))
        }
    }

    pub const fn get(self) -> u32 {
        self.0
    }

    pub fn checked_next(self) -> Result<Self, StoreError> {
        self.0
            .checked_add(1)
            .map(Self)
            .ok_or(StoreError::RevisionOverflow)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RevisionedEntity {
    Project,
    Task,
    WorkUnit,
    Runtime,
}

impl RevisionedEntity {
    const fn table(self) -> &'static str {
        match self {
            Self::Project => "projects",
            Self::Task => "tasks",
            Self::WorkUnit => "work_units",
            Self::Runtime => "runtimes",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImmutableEntity {
    Run,
    AgentSession,
    Evidence,
    Grant,
}

impl ImmutableEntity {
    const fn table(self) -> &'static str {
        match self {
            Self::Run => "runs",
            Self::AgentSession => "agent_sessions",
            Self::Evidence => "evidence",
            Self::Grant => "grants",
        }
    }
}

impl Store {
    pub fn insert_revisioned(
        &mut self,
        entity: RevisionedEntity,
        id: CanonicalId,
    ) -> Result<Revision, StoreError> {
        let sql = format!(
            "INSERT INTO {}(id, revision) VALUES (?1, 1)",
            entity.table()
        );
        self.connection
            .execute(&sql, [id.to_canonical_text()])
            .map_err(map_insert_error)?;
        Ok(Revision::INITIAL)
    }

    pub fn revision(
        &self,
        entity: RevisionedEntity,
        id: CanonicalId,
    ) -> Result<Revision, StoreError> {
        let sql = format!("SELECT revision FROM {} WHERE id = ?1", entity.table());
        let value = self
            .connection
            .query_row(&sql, [id.to_canonical_text()], |row| row.get::<_, i64>(0))
            .optional()
            .map_err(|_| StoreError::Database)?
            .ok_or(StoreError::NotFound)?;
        revision_from_sql(value)
    }

    pub fn compare_and_swap_revision(
        &mut self,
        entity: RevisionedEntity,
        id: CanonicalId,
        expected: Revision,
    ) -> Result<Revision, StoreError> {
        let current = self.revision(entity, id)?;
        if current != expected {
            return Err(StoreError::RevisionConflict);
        }
        let next = expected.checked_next()?;
        let sql = format!(
            "UPDATE {} SET revision = revision + 1 WHERE id = ?1 AND revision = ?2",
            entity.table()
        );
        let changed = self
            .connection
            .execute(&sql, (id.to_canonical_text(), i64::from(expected.get())))
            .map_err(|_| StoreError::Database)?;
        if changed == 1 {
            Ok(next)
        } else {
            Err(StoreError::RevisionConflict)
        }
    }

    pub fn insert_immutable(
        &mut self,
        entity: ImmutableEntity,
        id: CanonicalId,
    ) -> Result<(), StoreError> {
        let sql = format!(
            "INSERT INTO {}(id, revision) VALUES (?1, 1)",
            entity.table()
        );
        self.connection
            .execute(&sql, [id.to_canonical_text()])
            .map_err(map_insert_error)?;
        Ok(())
    }

    pub fn immutable_exists(
        &self,
        entity: ImmutableEntity,
        id: CanonicalId,
    ) -> Result<bool, StoreError> {
        let sql = format!("SELECT 1 FROM {} WHERE id = ?1", entity.table());
        self.connection
            .query_row(&sql, [id.to_canonical_text()], |_| Ok(()))
            .optional()
            .map(|value| value.is_some())
            .map_err(|_| StoreError::Database)
    }
}

pub(crate) fn revision_from_sql(value: i64) -> Result<Revision, StoreError> {
    let value = u32::try_from(value).map_err(|_| StoreError::InvalidRevision)?;
    Revision::new(value)
}

pub(crate) fn map_insert_error(error: SqliteError) -> StoreError {
    if matches!(
        error,
        SqliteError::SqliteFailure(ref inner, _) if inner.code == ErrorCode::ConstraintViolation
    ) {
        StoreError::DuplicateConflict
    } else {
        StoreError::Database
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::TestDb;

    const PROJECT_A: &str = "01890f3a-7b2c-7d45-8a61-3c4e5f607182";
    const PROJECT_B: &str = "01890f3a-7b2c-7d45-8a61-3c4e5f607183";

    #[test]
    fn canonical_id_accepts_only_exact_lowercase_hyphenated_uuidv7() {
        let id = CanonicalId::parse(PROJECT_A).expect("canonical UUIDv7");
        assert_eq!(id.to_string(), PROJECT_A);
        assert_eq!(id.to_canonical_text(), PROJECT_A);

        for invalid in [
            "01890F3A-7B2C-7D45-8A61-3C4E5F607182",
            "01890f3a7b2c7d458a613c4e5f607182",
            "{01890f3a-7b2c-7d45-8a61-3c4e5f607182}",
            " 01890f3a-7b2c-7d45-8a61-3c4e5f607182",
            "00000000-0000-0000-0000-000000000000",
            "67e55044-10b1-426f-9247-bb680e5fe0c8",
        ] {
            assert_eq!(
                CanonicalId::parse(invalid),
                Err(StoreError::InvalidCanonicalId)
            );
        }
    }

    #[test]
    fn revision_domain_is_positive_checked_u32() {
        assert_eq!(Revision::INITIAL.get(), 1);
        assert_eq!(Revision::new(0), Err(StoreError::InvalidRevision));
        assert_eq!(Revision::new(u32::MAX).unwrap().get(), u32::MAX);
        assert_eq!(
            Revision::new(u32::MAX).unwrap().checked_next(),
            Err(StoreError::RevisionOverflow)
        );
        assert_eq!(Revision::INITIAL.checked_next().unwrap().get(), 2);
    }

    #[test]
    fn revisioned_entities_start_at_one_and_advance_exactly_once() {
        let db = TestDb::new("revision-cas");
        let mut store = Store::open(&db.path).unwrap();
        let id = CanonicalId::parse(PROJECT_A).unwrap();

        assert_eq!(
            store
                .insert_revisioned(RevisionedEntity::Project, id)
                .unwrap(),
            Revision::INITIAL
        );
        assert_eq!(
            store.revision(RevisionedEntity::Project, id).unwrap(),
            Revision::INITIAL
        );
        assert_eq!(
            store
                .compare_and_swap_revision(RevisionedEntity::Project, id, Revision::INITIAL)
                .unwrap()
                .get(),
            2
        );
        assert_eq!(
            store.revision(RevisionedEntity::Project, id).unwrap().get(),
            2
        );
    }

    #[test]
    fn stale_revision_conflict_never_writes() {
        let db = TestDb::new("stale-cas");
        let mut store = Store::open(&db.path).unwrap();
        let id = CanonicalId::parse(PROJECT_A).unwrap();
        store.insert_revisioned(RevisionedEntity::Task, id).unwrap();
        store
            .compare_and_swap_revision(RevisionedEntity::Task, id, Revision::INITIAL)
            .unwrap();

        assert_eq!(
            store.compare_and_swap_revision(RevisionedEntity::Task, id, Revision::INITIAL),
            Err(StoreError::RevisionConflict)
        );
        assert_eq!(store.revision(RevisionedEntity::Task, id).unwrap().get(), 2);
    }

    #[test]
    fn stale_max_expectation_is_conflict_not_overflow() {
        let db = TestDb::new("stale-max-cas");
        let mut store = Store::open(&db.path).unwrap();
        let id = CanonicalId::parse(PROJECT_A).unwrap();
        store
            .insert_revisioned(RevisionedEntity::Project, id)
            .unwrap();

        assert_eq!(
            store.compare_and_swap_revision(
                RevisionedEntity::Project,
                id,
                Revision::new(u32::MAX).unwrap(),
            ),
            Err(StoreError::RevisionConflict)
        );
        assert_eq!(
            store.revision(RevisionedEntity::Project, id).unwrap(),
            Revision::INITIAL
        );
    }

    #[test]
    fn max_revision_fails_before_mutation_or_wraparound() {
        let db = TestDb::new("overflow-cas");
        let mut store = Store::open(&db.path).unwrap();
        let id = CanonicalId::parse(PROJECT_A).unwrap();
        store
            .insert_revisioned(RevisionedEntity::Runtime, id)
            .unwrap();
        store
            .connection
            .execute(
                "UPDATE runtimes SET revision = ?1 WHERE id = ?2",
                (i64::from(u32::MAX), id.to_canonical_text()),
            )
            .unwrap();

        let maximum = Revision::new(u32::MAX).unwrap();
        assert_eq!(
            store.compare_and_swap_revision(RevisionedEntity::Runtime, id, maximum),
            Err(StoreError::RevisionOverflow)
        );
        assert_eq!(
            store.revision(RevisionedEntity::Runtime, id).unwrap(),
            maximum
        );
    }

    #[test]
    fn missing_revisioned_identity_is_not_confused_with_conflict() {
        let db = TestDb::new("missing-cas");
        let mut store = Store::open(&db.path).unwrap();
        let id = CanonicalId::parse(PROJECT_A).unwrap();
        assert_eq!(
            store.compare_and_swap_revision(RevisionedEntity::WorkUnit, id, Revision::INITIAL),
            Err(StoreError::NotFound)
        );
    }

    #[test]
    fn duplicate_revisioned_identity_fails_closed() {
        let db = TestDb::new("duplicate-revisioned");
        let mut store = Store::open(&db.path).unwrap();
        let id = CanonicalId::parse(PROJECT_A).unwrap();
        store
            .insert_revisioned(RevisionedEntity::Project, id)
            .unwrap();
        assert_eq!(
            store.insert_revisioned(RevisionedEntity::Project, id),
            Err(StoreError::DuplicateConflict)
        );
    }

    #[test]
    fn immutable_records_insert_once_and_have_no_mutation_api() {
        let db = TestDb::new("immutable");
        let mut store = Store::open(&db.path).unwrap();
        let ids = [
            CanonicalId::parse(PROJECT_A).unwrap(),
            CanonicalId::parse(PROJECT_B).unwrap(),
        ];

        for entity in [
            ImmutableEntity::Run,
            ImmutableEntity::AgentSession,
            ImmutableEntity::Evidence,
            ImmutableEntity::Grant,
        ] {
            let id = ids[(entity as usize) % ids.len()];
            store.insert_immutable(entity, id).unwrap();
            assert!(store.immutable_exists(entity, id).unwrap());
            assert_eq!(
                store.insert_immutable(entity, id),
                Err(StoreError::DuplicateConflict)
            );
        }

        let source = include_str!("metadata.rs");
        let production = source
            .split("#[cfg(test)]")
            .next()
            .expect("production source");
        assert!(!production.contains("update_immutable"));
        assert!(!production.contains("mutate_immutable"));
    }

    #[test]
    fn sqlite_revision_checks_reject_zero_and_out_of_range_values() {
        let db = TestDb::new("revision-checks");
        let store = Store::open(&db.path).unwrap();
        assert!(
            store
                .connection
                .execute(
                    "INSERT INTO projects(id, revision) VALUES (?1, 0)",
                    [PROJECT_A],
                )
                .is_err()
        );
        assert!(
            store
                .connection
                .execute(
                    "INSERT INTO tasks(id, revision) VALUES (?1, 4294967296)",
                    [PROJECT_B],
                )
                .is_err()
        );
    }
}
