use rusqlite::Connection;

use crate::{Store, StoreError};

impl Store {
    pub fn verify_integrity(&self) -> Result<(), StoreError> {
        verify_integrity_connection(&self.connection)
    }
}

pub(crate) fn verify_integrity_connection(connection: &Connection) -> Result<(), StoreError> {
    let mut integrity = connection
        .prepare("PRAGMA integrity_check")
        .map_err(|_| StoreError::IntegrityFailure)?;
    let rows = integrity
        .query_map([], |row| row.get::<_, String>(0))
        .map_err(|_| StoreError::IntegrityFailure)?;
    let results = rows
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| StoreError::IntegrityFailure)?;
    if results.as_slice() != ["ok"] {
        return Err(StoreError::IntegrityFailure);
    }

    let mut foreign_keys = connection
        .prepare("PRAGMA foreign_key_check")
        .map_err(|_| StoreError::IntegrityFailure)?;
    let mut violations = foreign_keys
        .query([])
        .map_err(|_| StoreError::IntegrityFailure)?;
    if violations
        .next()
        .map_err(|_| StoreError::IntegrityFailure)?
        .is_some()
    {
        return Err(StoreError::IntegrityFailure);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::TestDb;
    use crate::{CanonicalId, Revision, RevisionedEntity};
    use std::fs;

    const PROJECT_A: &str = "01890f3a-7b2c-7d45-8a61-3c4e5f6071c0";
    const PROJECT_B: &str = "01890f3a-7b2c-7d45-8a61-3c4e5f6071c1";
    const EVENT_A: &str = "01890f3a-7b2c-7d45-8a61-3c4e5f6071d0";
    const MISSING_EVENT: &str = "01890f3a-7b2c-7d45-8a61-3c4e5f6071d1";

    fn id(value: &str) -> CanonicalId {
        CanonicalId::parse(value).unwrap()
    }

    #[test]
    fn valid_database_passes_integrity_and_foreign_key_checks() {
        let db = TestDb::new("integrity-valid");
        let store = Store::open(&db.path).unwrap();
        assert_eq!(store.verify_integrity(), Ok(()));
    }

    #[test]
    fn committed_wal_metadata_survives_close_and_reopen() {
        let db = TestDb::new("wal-reopen");
        let project = id(PROJECT_A);
        {
            let mut store = Store::open(&db.path).unwrap();
            let journal_mode: String = store
                .connection
                .query_row("PRAGMA journal_mode", [], |row| row.get(0))
                .unwrap();
            assert_eq!(journal_mode, "wal");
            assert_eq!(
                store.insert_revisioned(RevisionedEntity::Project, project),
                Ok(Revision::INITIAL)
            );
            let wal_path = db.directory.join("metadata.sqlite3-wal");
            assert!(wal_path.is_file());
            assert!(fs::metadata(wal_path).unwrap().len() > 0);
        }

        let reopened = Store::open(&db.path).unwrap();
        assert_eq!(
            reopened.revision(RevisionedEntity::Project, project),
            Ok(Revision::INITIAL)
        );
        assert_eq!(reopened.verify_integrity(), Ok(()));
    }

    #[test]
    fn uncommitted_transaction_does_not_survive_reopen() {
        let db = TestDb::new("uncommitted-reopen");
        let project = id(PROJECT_B);
        {
            let store = Store::open(&db.path).unwrap();
            store.connection.execute_batch("BEGIN IMMEDIATE").unwrap();
            store
                .connection
                .execute(
                    "INSERT INTO projects(id, revision) VALUES (?1, 1)",
                    [PROJECT_B],
                )
                .unwrap();
            assert_eq!(
                store
                    .connection
                    .query_row(
                        "SELECT revision FROM projects WHERE id = ?1",
                        [PROJECT_B],
                        |row| row.get::<_, i64>(0),
                    )
                    .unwrap(),
                1
            );
        }

        let reopened = Store::open(&db.path).unwrap();
        assert_eq!(
            reopened.revision(RevisionedEntity::Project, project),
            Err(StoreError::NotFound)
        );
        assert_eq!(reopened.verify_integrity(), Ok(()));
    }

    #[test]
    fn foreign_key_corruption_fails_read_only_preflight_without_db_byte_change() {
        let db = TestDb::new("foreign-key-corrupt");
        drop(Store::open(&db.path).unwrap());
        let connection = Connection::open(&db.path).unwrap();
        connection.execute_batch("PRAGMA foreign_keys=OFF").unwrap();
        connection
            .execute(
                "INSERT INTO events(\
                    id, revision, event_type, owner_kind, owner_id, owner_revision, stream_seq, previous_event_id\
                 ) VALUES (?1, 1, 'task.created', 'project', ?2, 1, ?3, ?4)",
                (
                    EVENT_A,
                    PROJECT_A,
                    1_u64.to_be_bytes().as_slice(),
                    MISSING_EVENT,
                ),
            )
            .unwrap();
        drop(connection);
        let before = fs::read(&db.path).unwrap();

        assert_eq!(
            Store::open(&db.path).err(),
            Some(StoreError::IntegrityFailure)
        );
        assert_eq!(fs::read(&db.path).unwrap(), before);
    }

    #[test]
    fn deterministic_corrupt_file_rejects_without_delete_recreate_or_byte_change() {
        let db = TestDb::new("corrupt-file");
        let corrupt = b"Kernux deterministic corrupt SQLite fixture v1\nnot-a-database\n";
        fs::write(&db.path, corrupt).unwrap();
        let before = fs::read(&db.path).unwrap();

        assert_eq!(
            Store::open(&db.path).err(),
            Some(StoreError::IntegrityFailure)
        );
        assert!(db.path.is_file());
        assert_eq!(fs::read(&db.path).unwrap(), before);
    }

    #[test]
    fn integrity_errors_are_generic_and_do_not_embed_database_payloads() {
        let text = StoreError::IntegrityFailure.to_string();
        assert_eq!(text, "metadata store integrity validation failed");
        assert!(!text.contains(PROJECT_A));
        assert!(!text.contains(EVENT_A));
        assert!(!text.contains("SELECT"));
        assert!(!text.contains("PRAGMA"));
    }
}
