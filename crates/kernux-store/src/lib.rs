//! Local SQLite metadata-store foundation.
//!
//! This crate owns the bounded SG-000021 SQLite metadata layer: filesystem
//! database opening, deterministic schema-v1 migration, revisioned metadata,
//! append-safe Event stream indexing, and bounded filesystem Artifact CAS bytes.
//! Policy evaluation, secret plaintext, and daemon state-root wiring remain out of scope.

#![forbid(unsafe_code)]

use std::fmt;
use std::fs;
use std::path::Path;
use std::time::Duration;

use rusqlite::{Connection, OpenFlags, TransactionBehavior};

mod artifact;
mod cas;
mod event;
mod metadata;
mod recovery;
pub use artifact::{AdapterConfigRef, ArtifactMetadata, Sha256Digest};
pub use cas::{ArtifactBindingError, ArtifactCas, CasBlob, CasError, VerifiedBlob};
pub use event::{AcceptedEvent, EventAppend, EventType, StreamOwnerKind, StreamRef};
pub use metadata::{CanonicalId, ImmutableEntity, Revision, RevisionedEntity};

const SCHEMA_V1_VERSION: i64 = 1;
const SCHEMA_V2_VERSION: i64 = 2;
const SCHEMA_VERSION: i64 = SCHEMA_V2_VERSION;
const MIGRATION_V1_NAME: &str = "metadata-v1";
const MIGRATION_V2_NAME: &str = "policy-v2";
const BUSY_TIMEOUT_MS: u64 = 5_000;
const WAL_AUTOCHECKPOINT_PAGES: i64 = 1_000;

const EXPECTED_TABLES_V1: &[&str] = &[
    "adapter_config_refs",
    "agent_sessions",
    "artifacts",
    "events",
    "evidence",
    "grants",
    "projects",
    "runs",
    "runtimes",
    "schema_migrations",
    "tasks",
    "work_units",
];

const EXPECTED_TABLES_V2: &[&str] = &[
    "adapter_config_refs",
    "agent_sessions",
    "artifacts",
    "events",
    "evidence",
    "grant_allowed_roots",
    "grant_constraints",
    "grant_issuance",
    "grant_network_hosts",
    "grant_revocations",
    "grant_state",
    "grants",
    "policy_decisions",
    "projects",
    "runs",
    "runtimes",
    "schema_migrations",
    "tasks",
    "work_units",
];

const SCHEMA_V1_SQL: &str = r#"
CREATE TABLE schema_migrations (
    version INTEGER PRIMARY KEY CHECK (version >= 1),
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE projects (
    id TEXT PRIMARY KEY,
    revision INTEGER NOT NULL CHECK (revision BETWEEN 1 AND 4294967295)
);

CREATE TABLE tasks (
    id TEXT PRIMARY KEY,
    revision INTEGER NOT NULL CHECK (revision BETWEEN 1 AND 4294967295)
);

CREATE TABLE work_units (
    id TEXT PRIMARY KEY,
    revision INTEGER NOT NULL CHECK (revision BETWEEN 1 AND 4294967295)
);

CREATE TABLE runs (
    id TEXT PRIMARY KEY,
    revision INTEGER NOT NULL CHECK (revision = 1)
);

CREATE TABLE agent_sessions (
    id TEXT PRIMARY KEY,
    revision INTEGER NOT NULL CHECK (revision = 1)
);

CREATE TABLE runtimes (
    id TEXT PRIMARY KEY,
    revision INTEGER NOT NULL CHECK (revision BETWEEN 1 AND 4294967295)
);

CREATE TABLE artifacts (
    id TEXT PRIMARY KEY,
    revision INTEGER NOT NULL CHECK (revision BETWEEN 1 AND 4294967295),
    sha256 TEXT CHECK (
        sha256 IS NULL OR (
            length(sha256) = 64 AND
            sha256 NOT GLOB '*[^0-9a-f]*'
        )
    ),
    size_bytes INTEGER CHECK (size_bytes IS NULL OR size_bytes >= 0),
    media_type TEXT,
    retention_ref TEXT
);

CREATE TABLE evidence (
    id TEXT PRIMARY KEY,
    revision INTEGER NOT NULL CHECK (revision = 1)
);

CREATE TABLE events (
    id TEXT PRIMARY KEY,
    revision INTEGER NOT NULL CHECK (revision = 1),
    event_type TEXT NOT NULL CHECK (length(event_type) BETWEEN 3 AND 128),
    owner_kind TEXT NOT NULL CHECK (owner_kind IN ('project', 'task', 'run', 'runtime')),
    owner_id TEXT NOT NULL,
    owner_revision INTEGER CHECK (
        owner_revision IS NULL OR owner_revision BETWEEN 1 AND 4294967295
    ),
    stream_seq BLOB NOT NULL CHECK (typeof(stream_seq) = 'blob' AND length(stream_seq) = 8),
    previous_event_id TEXT REFERENCES events(id),
    UNIQUE (owner_kind, owner_id, stream_seq)
);

CREATE TABLE grants (
    id TEXT PRIMARY KEY,
    revision INTEGER NOT NULL CHECK (revision = 1)
);

CREATE TABLE adapter_config_refs (
    owner_kind TEXT NOT NULL CHECK (length(owner_kind) BETWEEN 1 AND 32),
    owner_id TEXT NOT NULL,
    config_type TEXT NOT NULL CHECK (length(config_type) BETWEEN 1 AND 64),
    config_ref TEXT NOT NULL CHECK (length(config_ref) BETWEEN 1 AND 2048),
    PRIMARY KEY (owner_kind, owner_id, config_type)
);
"#;

const SCHEMA_V2_SQL: &str = r#"
CREATE TABLE grant_issuance (
    grant_id TEXT PRIMARY KEY REFERENCES grants(id),
    run_id TEXT NOT NULL REFERENCES runs(id),
    agent_session_id TEXT REFERENCES agent_sessions(id),
    action TEXT NOT NULL CHECK (length(action) BETWEEN 3 AND 128),
    resource_uri TEXT NOT NULL CHECK (length(resource_uri) BETWEEN 10 AND 4096),
    resource_match TEXT NOT NULL CHECK (resource_match IN ('exact', 'subtree')),
    runtime_id TEXT NOT NULL REFERENCES runtimes(id),
    runtime_revision INTEGER NOT NULL CHECK (runtime_revision BETWEEN 1 AND 4294967295),
    consequence_ceiling TEXT NOT NULL CHECK (consequence_ceiling IN ('C0', 'C1', 'C2', 'C3', 'C4')),
    issuer_authority TEXT NOT NULL CHECK (length(issuer_authority) BETWEEN 1 AND 128),
    policy_revision BLOB NOT NULL CHECK (typeof(policy_revision) = 'blob' AND length(policy_revision) = 8 AND policy_revision <> X'0000000000000000'),
    issued_at TEXT NOT NULL CHECK (length(issued_at) = 20),
    not_before TEXT NOT NULL CHECK (length(not_before) = 20),
    expires_at TEXT NOT NULL CHECK (length(expires_at) = 20),
    max_uses BLOB NOT NULL CHECK (typeof(max_uses) = 'blob' AND length(max_uses) = 8 AND max_uses <> X'0000000000000000'),
    delegation_depth BLOB NOT NULL CHECK (typeof(delegation_depth) = 'blob' AND length(delegation_depth) = 8),
    parent_grant_id TEXT REFERENCES grant_issuance(grant_id),
    issued_event_id TEXT NOT NULL REFERENCES events(id),
    CHECK (parent_grant_id IS NULL OR parent_grant_id <> grant_id)
);

CREATE TABLE grant_constraints (
    grant_id TEXT PRIMARY KEY REFERENCES grant_issuance(grant_id),
    max_bytes BLOB CHECK (max_bytes IS NULL OR (typeof(max_bytes) = 'blob' AND length(max_bytes) = 8)),
    max_duration_ms BLOB CHECK (max_duration_ms IS NULL OR (typeof(max_duration_ms) = 'blob' AND length(max_duration_ms) = 8)),
    max_uses BLOB CHECK (max_uses IS NULL OR (typeof(max_uses) = 'blob' AND length(max_uses) = 8 AND max_uses <> X'0000000000000000'))
);

CREATE TABLE grant_allowed_roots (
    grant_id TEXT NOT NULL REFERENCES grant_constraints(grant_id),
    value TEXT NOT NULL CHECK (length(value) BETWEEN 1 AND 1024),
    PRIMARY KEY (grant_id, value)
);

CREATE TABLE grant_network_hosts (
    grant_id TEXT NOT NULL REFERENCES grant_constraints(grant_id),
    value TEXT NOT NULL CHECK (length(value) BETWEEN 1 AND 253),
    PRIMARY KEY (grant_id, value)
);

CREATE TABLE grant_state (
    grant_id TEXT PRIMARY KEY REFERENCES grant_issuance(grant_id),
    used_count BLOB NOT NULL CHECK (typeof(used_count) = 'blob' AND length(used_count) = 8)
);

CREATE TABLE grant_revocations (
    grant_id TEXT PRIMARY KEY REFERENCES grant_issuance(grant_id),
    revoked_at TEXT NOT NULL CHECK (length(revoked_at) = 20),
    event_id TEXT NOT NULL REFERENCES events(id),
    reason_class TEXT NOT NULL CHECK (length(reason_class) BETWEEN 1 AND 64)
);

CREATE TABLE policy_decisions (
    request_id TEXT PRIMARY KEY,
    decision TEXT NOT NULL CHECK (decision IN ('allow', 'deny')),
    deny_reason TEXT CHECK (deny_reason IS NULL OR length(deny_reason) BETWEEN 1 AND 64),
    grant_id TEXT REFERENCES grant_issuance(grant_id),
    event_id TEXT NOT NULL REFERENCES events(id),
    decided_at TEXT NOT NULL CHECK (length(decided_at) = 20),
    CHECK (
        (decision = 'allow' AND grant_id IS NOT NULL AND deny_reason IS NULL) OR
        (decision = 'deny' AND grant_id IS NULL AND deny_reason IS NOT NULL)
    )
);
"#;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoreError {
    InvalidPath,
    Database,
    Configuration,
    FutureSchema,
    SchemaDrift,
    InvalidCanonicalId,
    InvalidRevision,
    NotFound,
    RevisionConflict,
    RevisionOverflow,
    DuplicateConflict,
    InvalidDigest,
    InvalidMetadata,
    ArtifactDigestConflict,
    ArtifactSizeConflict,
    InvalidEventType,
    InvalidStreamOwner,
    EventPredecessorMismatch,
    EventSequenceOverflow,
    EventStreamCorrupt,
    IntegrityFailure,
}

impl fmt::Display for StoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidPath => "metadata store path is invalid",
            Self::Database => "metadata store database operation failed",
            Self::Configuration => "metadata store configuration is invalid",
            Self::FutureSchema => "metadata store schema is newer than this build",
            Self::SchemaDrift => "metadata store schema does not match the expected version",
            Self::InvalidCanonicalId => "metadata store canonical identifier is invalid",
            Self::InvalidRevision => "metadata store revision is invalid",
            Self::NotFound => "metadata store record was not found",
            Self::RevisionConflict => "metadata store revision expectation does not match",
            Self::RevisionOverflow => "metadata store revision cannot advance",
            Self::DuplicateConflict => "metadata store identity already exists",
            Self::InvalidDigest => "metadata store digest is invalid",
            Self::InvalidMetadata => "metadata store metadata is invalid",
            Self::ArtifactDigestConflict => {
                "metadata store artifact digest conflicts with existing byte identity"
            }
            Self::ArtifactSizeConflict => {
                "metadata store artifact size conflicts with existing byte identity"
            }
            Self::InvalidEventType => "metadata store event type is invalid",
            Self::InvalidStreamOwner => "metadata store event stream owner is invalid",
            Self::EventPredecessorMismatch => {
                "metadata store event predecessor does not match the stream tip"
            }
            Self::EventSequenceOverflow => "metadata store event sequence cannot advance",
            Self::EventStreamCorrupt => "metadata store event stream is inconsistent",
            Self::IntegrityFailure => "metadata store integrity validation failed",
        })
    }
}

impl std::error::Error for StoreError {}

pub struct Store {
    connection: Connection,
}

impl Store {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, StoreError> {
        let path = path.as_ref();
        validate_path(path)?;
        let state = preflight_existing(path)?;

        let mut connection = Connection::open(path).map_err(|_| StoreError::Database)?;
        configure_connection(&connection)?;

        match state {
            ExistingState::Fresh => {
                migrate_v1(&mut connection, SCHEMA_V1_SQL)?;
                migrate_v2(&mut connection, SCHEMA_V2_SQL)?;
            }
            ExistingState::VersionOne => migrate_v2(&mut connection, SCHEMA_V2_SQL)?,
            ExistingState::VersionTwo => validate_schema_v2(&connection)?,
        }

        verify_connection_policy(&connection)?;
        validate_schema_v2(&connection)?;
        recovery::verify_integrity_connection(&connection)?;
        Ok(Self { connection })
    }

    pub fn schema_version(&self) -> Result<u32, StoreError> {
        let version = user_version(&self.connection)?;
        u32::try_from(version).map_err(|_| StoreError::SchemaDrift)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ExistingState {
    Fresh,
    VersionOne,
    VersionTwo,
}

fn validate_path(path: &Path) -> Result<(), StoreError> {
    if path.as_os_str().is_empty() || path == Path::new(":memory:") {
        return Err(StoreError::InvalidPath);
    }
    if let Ok(metadata) = fs::metadata(path)
        && metadata.is_dir()
    {
        return Err(StoreError::InvalidPath);
    }
    Ok(())
}

fn preflight_existing(path: &Path) -> Result<ExistingState, StoreError> {
    let metadata = match fs::metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
            return Ok(ExistingState::Fresh);
        }
        Err(_) => return Err(StoreError::InvalidPath),
    };

    if metadata.len() == 0 {
        return Ok(ExistingState::Fresh);
    }

    let connection = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .map_err(|_| StoreError::Database)?;
    let version = connection
        .query_row("PRAGMA user_version", [], |row| row.get::<_, i64>(0))
        .map_err(|_| StoreError::IntegrityFailure)?;

    match version {
        0 => {
            if user_table_names(&connection)?.is_empty() {
                Ok(ExistingState::Fresh)
            } else {
                Err(StoreError::SchemaDrift)
            }
        }
        SCHEMA_V1_VERSION => {
            validate_schema_v1(&connection)?;
            recovery::verify_integrity_connection(&connection)?;
            Ok(ExistingState::VersionOne)
        }
        SCHEMA_V2_VERSION => {
            validate_schema_v2(&connection)?;
            recovery::verify_integrity_connection(&connection)?;
            Ok(ExistingState::VersionTwo)
        }
        version if version > SCHEMA_VERSION => Err(StoreError::FutureSchema),
        _ => Err(StoreError::SchemaDrift),
    }
}

fn configure_connection(connection: &Connection) -> Result<(), StoreError> {
    connection
        .busy_timeout(Duration::from_millis(BUSY_TIMEOUT_MS))
        .map_err(|_| StoreError::Configuration)?;
    connection
        .pragma_update(None, "foreign_keys", "ON")
        .map_err(|_| StoreError::Configuration)?;
    connection
        .pragma_update(None, "trusted_schema", "OFF")
        .map_err(|_| StoreError::Configuration)?;
    connection
        .pragma_update(None, "synchronous", "FULL")
        .map_err(|_| StoreError::Configuration)?;
    connection
        .pragma_update(None, "wal_autocheckpoint", WAL_AUTOCHECKPOINT_PAGES)
        .map_err(|_| StoreError::Configuration)?;

    let journal_mode: String = connection
        .query_row("PRAGMA journal_mode = WAL", [], |row| row.get(0))
        .map_err(|_| StoreError::Configuration)?;
    if journal_mode != "wal" {
        return Err(StoreError::Configuration);
    }
    Ok(())
}

fn verify_connection_policy(connection: &Connection) -> Result<(), StoreError> {
    let busy_timeout: i64 = connection
        .query_row("PRAGMA busy_timeout", [], |row| row.get(0))
        .map_err(|_| StoreError::Configuration)?;
    let foreign_keys: i64 = connection
        .query_row("PRAGMA foreign_keys", [], |row| row.get(0))
        .map_err(|_| StoreError::Configuration)?;
    let journal_mode: String = connection
        .query_row("PRAGMA journal_mode", [], |row| row.get(0))
        .map_err(|_| StoreError::Configuration)?;
    let synchronous: i64 = connection
        .query_row("PRAGMA synchronous", [], |row| row.get(0))
        .map_err(|_| StoreError::Configuration)?;
    let trusted_schema: i64 = connection
        .query_row("PRAGMA trusted_schema", [], |row| row.get(0))
        .map_err(|_| StoreError::Configuration)?;
    let wal_autocheckpoint: i64 = connection
        .query_row("PRAGMA wal_autocheckpoint", [], |row| row.get(0))
        .map_err(|_| StoreError::Configuration)?;

    if busy_timeout != BUSY_TIMEOUT_MS as i64
        || foreign_keys != 1
        || journal_mode != "wal"
        || synchronous != 2
        || trusted_schema != 0
        || wal_autocheckpoint != WAL_AUTOCHECKPOINT_PAGES
    {
        return Err(StoreError::Configuration);
    }
    Ok(())
}

fn migrate_v1(connection: &mut Connection, schema_sql: &str) -> Result<(), StoreError> {
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|_| StoreError::Database)?;
    transaction
        .execute_batch(schema_sql)
        .map_err(|_| StoreError::Database)?;
    transaction
        .execute(
            "INSERT INTO schema_migrations(version, name) VALUES (?1, ?2)",
            (SCHEMA_V1_VERSION, MIGRATION_V1_NAME),
        )
        .map_err(|_| StoreError::Database)?;
    transaction
        .pragma_update(None, "user_version", SCHEMA_V1_VERSION)
        .map_err(|_| StoreError::Database)?;
    transaction.commit().map_err(|_| StoreError::Database)
}

fn migrate_v2(connection: &mut Connection, schema_sql: &str) -> Result<(), StoreError> {
    let transaction = connection
        .transaction_with_behavior(TransactionBehavior::Immediate)
        .map_err(|_| StoreError::Database)?;
    transaction
        .execute_batch(schema_sql)
        .map_err(|_| StoreError::Database)?;
    transaction
        .execute(
            "INSERT INTO schema_migrations(version, name) VALUES (?1, ?2)",
            (SCHEMA_V2_VERSION, MIGRATION_V2_NAME),
        )
        .map_err(|_| StoreError::Database)?;
    transaction
        .pragma_update(None, "user_version", SCHEMA_V2_VERSION)
        .map_err(|_| StoreError::Database)?;
    transaction.commit().map_err(|_| StoreError::Database)
}

fn validate_schema_v1(connection: &Connection) -> Result<(), StoreError> {
    if user_version(connection)? != SCHEMA_V1_VERSION {
        return Err(StoreError::SchemaDrift);
    }
    validate_migration_ledger(connection, &[(SCHEMA_V1_VERSION, MIGRATION_V1_NAME)])?;
    let tables = user_table_names(connection)?;
    if tables.as_slice() != EXPECTED_TABLES_V1 {
        return Err(StoreError::SchemaDrift);
    }
    Ok(())
}

fn validate_schema_v2(connection: &Connection) -> Result<(), StoreError> {
    if user_version(connection)? != SCHEMA_V2_VERSION {
        return Err(StoreError::SchemaDrift);
    }
    validate_migration_ledger(
        connection,
        &[
            (SCHEMA_V1_VERSION, MIGRATION_V1_NAME),
            (SCHEMA_V2_VERSION, MIGRATION_V2_NAME),
        ],
    )?;
    let tables = user_table_names(connection)?;
    if tables.as_slice() != EXPECTED_TABLES_V2 {
        return Err(StoreError::SchemaDrift);
    }
    Ok(())
}

fn validate_migration_ledger(
    connection: &Connection,
    expected: &[(i64, &str)],
) -> Result<(), StoreError> {
    let exists: i64 = connection
        .query_row(
            "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name='schema_migrations'",
            [],
            |row| row.get(0),
        )
        .map_err(|_| StoreError::Database)?;
    if exists != 1 {
        return Err(StoreError::SchemaDrift);
    }

    let mut statement = connection
        .prepare("SELECT version, name FROM schema_migrations ORDER BY version")
        .map_err(|_| StoreError::SchemaDrift)?;
    let rows = statement
        .query_map([], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(|_| StoreError::SchemaDrift)?;
    let values = rows
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| StoreError::SchemaDrift)?;
    let expected = expected
        .iter()
        .map(|(version, name)| (*version, (*name).to_owned()))
        .collect::<Vec<_>>();
    if values != expected {
        return Err(StoreError::SchemaDrift);
    }
    Ok(())
}

fn user_version(connection: &Connection) -> Result<i64, StoreError> {
    connection
        .query_row("PRAGMA user_version", [], |row| row.get(0))
        .map_err(|_| StoreError::Database)
}

fn user_table_names(connection: &Connection) -> Result<Vec<String>, StoreError> {
    let mut statement = connection
        .prepare(
            "SELECT name FROM sqlite_master \
             WHERE type='table' AND name NOT LIKE 'sqlite_%' \
             ORDER BY name",
        )
        .map_err(|_| StoreError::Database)?;
    let rows = statement
        .query_map([], |row| row.get::<_, String>(0))
        .map_err(|_| StoreError::Database)?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|_| StoreError::Database)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static PATH_COUNTER: AtomicU64 = AtomicU64::new(0);

    pub(super) struct TestDb {
        pub(super) directory: std::path::PathBuf,
        pub(super) path: std::path::PathBuf,
    }

    impl TestDb {
        pub(super) fn new(name: &str) -> Self {
            let counter = PATH_COUNTER.fetch_add(1, Ordering::Relaxed);
            let directory = std::env::temp_dir().join(format!(
                "kernux-store-{}-{}-{counter}",
                std::process::id(),
                name
            ));
            fs::create_dir_all(&directory).expect("create test directory");
            let path = directory.join("metadata.sqlite3");
            Self { directory, path }
        }
    }

    impl Drop for TestDb {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.directory);
        }
    }

    #[test]
    fn open_creates_exact_schema_and_required_connection_policy() {
        let db = TestDb::new("open-policy");
        let store = Store::open(&db.path).expect("open fresh store");

        assert_eq!(store.schema_version().unwrap(), SCHEMA_VERSION as u32);
        assert_eq!(
            user_table_names(&store.connection).unwrap(),
            EXPECTED_TABLES_V2
        );
        validate_schema_v2(&store.connection).expect("exact schema and ledger");
        verify_connection_policy(&store.connection).expect("required policy");
    }

    #[test]
    fn version_two_reopens_without_reapplying_migration() {
        let db = TestDb::new("reopen");
        drop(Store::open(&db.path).expect("create store"));
        let reopened = Store::open(&db.path).expect("reopen store");

        let count: i64 = reopened
            .connection
            .query_row("SELECT COUNT(*) FROM schema_migrations", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(count, 2);
        assert_eq!(user_version(&reopened.connection).unwrap(), SCHEMA_VERSION);
    }

    fn table_definitions(connection: &Connection, names: &[&str]) -> Vec<(String, String)> {
        let mut statement = connection
            .prepare(
                "SELECT name, sql FROM sqlite_master \
                 WHERE type = 'table' AND name = ?1",
            )
            .unwrap();
        let mut definitions = Vec::with_capacity(names.len());
        for name in names {
            definitions.push(
                statement
                    .query_row([name], |row| Ok((row.get(0)?, row.get(1)?)))
                    .unwrap(),
            );
        }
        definitions
    }

    #[test]
    fn v2_migration_preserves_every_v1_table_definition_exactly() {
        let db = TestDb::new("v1-ddl-preserved");
        let mut connection = Connection::open(&db.path).unwrap();
        configure_connection(&connection).unwrap();
        migrate_v1(&mut connection, SCHEMA_V1_SQL).unwrap();
        let before = table_definitions(&connection, EXPECTED_TABLES_V1);

        migrate_v2(&mut connection, SCHEMA_V2_SQL).unwrap();
        let after = table_definitions(&connection, EXPECTED_TABLES_V1);

        assert_eq!(after, before);
        validate_schema_v2(&connection).unwrap();
    }

    #[test]
    fn version_one_migrates_additively_to_v2_without_rewriting_prior_rows() {
        let db = TestDb::new("v1-to-v2");
        let mut connection = Connection::open(&db.path).unwrap();
        configure_connection(&connection).unwrap();
        migrate_v1(&mut connection, SCHEMA_V1_SQL).unwrap();
        connection
            .execute(
                "INSERT INTO projects(id, revision) VALUES (?1, 7)",
                ["01890f3a-7b2c-7d45-8a61-3c4e5f6071c0"],
            )
            .unwrap();
        connection
            .execute(
                "INSERT INTO grants(id, revision) VALUES (?1, 1)",
                ["01890f3a-7b2c-7d45-8a61-3c4e5f6071c1"],
            )
            .unwrap();
        drop(connection);

        let migrated = Store::open(&db.path).unwrap();
        assert_eq!(migrated.schema_version().unwrap(), 2);
        let project_revision: i64 = migrated
            .connection
            .query_row(
                "SELECT revision FROM projects WHERE id = ?1",
                ["01890f3a-7b2c-7d45-8a61-3c4e5f6071c0"],
                |row| row.get(0),
            )
            .unwrap();
        let grant_revision: i64 = migrated
            .connection
            .query_row(
                "SELECT revision FROM grants WHERE id = ?1",
                ["01890f3a-7b2c-7d45-8a61-3c4e5f6071c1"],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(project_revision, 7);
        assert_eq!(grant_revision, 1);
        validate_schema_v2(&migrated.connection).unwrap();
    }

    #[test]
    fn tampered_v2_migration_ledger_rejects_without_modifying_database_bytes() {
        let db = TestDb::new("v2-ledger-tamper");
        drop(Store::open(&db.path).unwrap());

        let connection = Connection::open(&db.path).unwrap();
        connection
            .execute(
                "UPDATE schema_migrations SET name = 'tampered-v2' WHERE version = 2",
                [],
            )
            .unwrap();
        connection
            .execute_batch("PRAGMA wal_checkpoint(TRUNCATE);")
            .unwrap();
        drop(connection);
        let before = fs::read(&db.path).unwrap();

        assert!(matches!(
            Store::open(&db.path),
            Err(StoreError::SchemaDrift)
        ));
        assert_eq!(fs::read(&db.path).unwrap(), before);
    }

    #[test]
    fn future_schema_rejects_without_modifying_database_bytes() {
        let db = TestDb::new("future");
        let connection = Connection::open(&db.path).unwrap();
        connection.execute_batch("CREATE TABLE sentinel(value TEXT NOT NULL); INSERT INTO sentinel VALUES ('keep'); PRAGMA user_version = 3;").unwrap();
        drop(connection);
        let before = fs::read(&db.path).unwrap();

        assert!(matches!(
            Store::open(&db.path),
            Err(StoreError::FutureSchema)
        ));
        assert_eq!(fs::read(&db.path).unwrap(), before);
    }

    #[test]
    fn partial_version_zero_schema_rejects_without_modifying_database_bytes() {
        let db = TestDb::new("partial");
        let connection = Connection::open(&db.path).unwrap();
        connection
            .execute_batch("CREATE TABLE partial(value INTEGER NOT NULL);")
            .unwrap();
        drop(connection);
        let before = fs::read(&db.path).unwrap();

        assert!(matches!(
            Store::open(&db.path),
            Err(StoreError::SchemaDrift)
        ));
        assert_eq!(fs::read(&db.path).unwrap(), before);
    }

    #[test]
    fn migration_failure_rolls_back_user_version_and_partial_schema() {
        let db = TestDb::new("rollback");
        let mut connection = Connection::open(&db.path).unwrap();
        configure_connection(&connection).unwrap();
        let invalid = "CREATE TABLE first_ok(id INTEGER PRIMARY KEY); CREATE TABLE broken(";

        assert_eq!(
            migrate_v1(&mut connection, invalid),
            Err(StoreError::Database)
        );
        assert_eq!(user_version(&connection).unwrap(), 0);
        assert!(user_table_names(&connection).unwrap().is_empty());
    }

    #[test]
    fn v2_migration_failure_rolls_back_to_exact_v1() {
        let db = TestDb::new("v2-rollback");
        let mut connection = Connection::open(&db.path).unwrap();
        configure_connection(&connection).unwrap();
        migrate_v1(&mut connection, SCHEMA_V1_SQL).unwrap();
        let invalid = "CREATE TABLE policy_partial(id INTEGER PRIMARY KEY); CREATE TABLE broken(";

        assert_eq!(
            migrate_v2(&mut connection, invalid),
            Err(StoreError::Database)
        );
        assert_eq!(user_version(&connection).unwrap(), SCHEMA_V1_VERSION);
        assert_eq!(user_table_names(&connection).unwrap(), EXPECTED_TABLES_V1);
        validate_schema_v1(&connection).unwrap();
    }

    #[test]
    fn path_must_be_filesystem_database_not_memory_or_directory() {
        assert!(matches!(
            Store::open(":memory:"),
            Err(StoreError::InvalidPath)
        ));
        let directory = TestDb::new("directory");
        assert!(matches!(
            Store::open(&directory.directory),
            Err(StoreError::InvalidPath)
        ));
    }

    #[test]
    fn schema_constants_preserve_positive_revision_and_full_u64_sequence_shapes() {
        assert_eq!(u32::MAX, 4_294_967_295);
        assert!(SCHEMA_V1_SQL.contains("revision BETWEEN 1 AND 4294967295"));
        assert!(SCHEMA_V1_SQL.contains("typeof(stream_seq) = 'blob' AND length(stream_seq) = 8"));
        assert!(
            SCHEMA_V2_SQL
                .contains("typeof(policy_revision) = 'blob' AND length(policy_revision) = 8")
        );
        assert!(SCHEMA_V2_SQL.contains("typeof(max_uses) = 'blob' AND length(max_uses) = 8"));
        assert!(SCHEMA_V2_SQL.contains("CREATE TABLE policy_decisions"));
        assert!(!SCHEMA_V2_SQL.contains("approval_required"));
        assert!(!SCHEMA_V1_SQL.contains("artifact_blob"));
        assert!(!SCHEMA_V2_SQL.contains("secret_value"));
    }
}
