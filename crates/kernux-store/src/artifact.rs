use std::fmt;
use std::str::FromStr;

use rusqlite::{Error as SqliteError, ErrorCode, OptionalExtension};

use crate::{CanonicalId, Revision, Store, StoreError};

const SHA256_HEX_LEN: usize = 64;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Sha256Digest(String);

impl Sha256Digest {
    pub fn parse(value: &str) -> Result<Self, StoreError> {
        if value.len() != SHA256_HEX_LEN
            || !value
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        {
            return Err(StoreError::InvalidDigest);
        }
        Ok(Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for Sha256Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sha256Digest({self})")
    }
}

impl fmt::Display for Sha256Digest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl FromStr for Sha256Digest {
    type Err = StoreError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::parse(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ArtifactMetadata {
    id: CanonicalId,
    revision: Revision,
    sha256: Option<Sha256Digest>,
    size_bytes: Option<u64>,
    media_type: Option<String>,
    retention_ref: Option<String>,
}

impl ArtifactMetadata {
    pub fn id(&self) -> CanonicalId {
        self.id
    }

    pub const fn revision(&self) -> Revision {
        self.revision
    }

    pub fn sha256(&self) -> Option<&Sha256Digest> {
        self.sha256.as_ref()
    }

    pub const fn size_bytes(&self) -> Option<u64> {
        self.size_bytes
    }

    pub fn media_type(&self) -> Option<&str> {
        self.media_type.as_deref()
    }

    pub fn retention_ref(&self) -> Option<&str> {
        self.retention_ref.as_deref()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ArtifactMetadataUpdate<'a> {
    pub media_type: Option<&'a str>,
    pub retention_ref: Option<&'a str>,
}

impl Store {
    pub fn insert_artifact_metadata(
        &mut self,
        id: CanonicalId,
        sha256: Option<&Sha256Digest>,
        size_bytes: Option<u64>,
        update: ArtifactMetadataUpdate<'_>,
    ) -> Result<ArtifactMetadata, StoreError> {
        let size_bytes = size_to_sql(size_bytes)?;
        self.connection
            .execute(
                "INSERT INTO artifacts(id, revision, sha256, size_bytes, media_type, retention_ref) \
                 VALUES (?1, 1, ?2, ?3, ?4, ?5)",
                (
                    id.to_canonical_text(),
                    sha256.map(Sha256Digest::as_str),
                    size_bytes,
                    update.media_type,
                    update.retention_ref,
                ),
            )
            .map_err(map_insert_error)?;
        self.artifact_metadata(id)
    }

    pub fn artifact_metadata(&self, id: CanonicalId) -> Result<ArtifactMetadata, StoreError> {
        let row = self
            .connection
            .query_row(
                "SELECT revision, sha256, size_bytes, media_type, retention_ref \
                 FROM artifacts WHERE id = ?1",
                [id.to_canonical_text()],
                |row| {
                    Ok((
                        row.get::<_, i64>(0)?,
                        row.get::<_, Option<String>>(1)?,
                        row.get::<_, Option<i64>>(2)?,
                        row.get::<_, Option<String>>(3)?,
                        row.get::<_, Option<String>>(4)?,
                    ))
                },
            )
            .optional()
            .map_err(|_| StoreError::Database)?
            .ok_or(StoreError::NotFound)?;

        let revision = revision_from_sql(row.0)?;
        let sha256 = row.1.map(|value| Sha256Digest::parse(&value)).transpose()?;
        let size_bytes = row.2.map(size_from_sql).transpose()?;
        Ok(ArtifactMetadata {
            id,
            revision,
            sha256,
            size_bytes,
            media_type: row.3,
            retention_ref: row.4,
        })
    }

    pub fn compare_and_swap_artifact_metadata(
        &mut self,
        id: CanonicalId,
        expected: Revision,
        update: ArtifactMetadataUpdate<'_>,
    ) -> Result<ArtifactMetadata, StoreError> {
        let current = self.artifact_metadata(id)?;
        if current.revision != expected {
            return Err(StoreError::RevisionConflict);
        }
        let next = expected.checked_next()?;

        let changed = self
            .connection
            .execute(
                "UPDATE artifacts \
                 SET revision = revision + 1, media_type = ?1, retention_ref = ?2 \
                 WHERE id = ?3 AND revision = ?4",
                (
                    update.media_type,
                    update.retention_ref,
                    id.to_canonical_text(),
                    i64::from(expected.get()),
                ),
            )
            .map_err(|_| StoreError::Database)?;
        if changed != 1 {
            return Err(StoreError::RevisionConflict);
        }

        let updated = self.artifact_metadata(id)?;
        if updated.revision != next
            || updated.sha256 != current.sha256
            || updated.size_bytes != current.size_bytes
        {
            return Err(StoreError::Database);
        }
        Ok(updated)
    }
}

fn revision_from_sql(value: i64) -> Result<Revision, StoreError> {
    let value = u32::try_from(value).map_err(|_| StoreError::InvalidRevision)?;
    Revision::new(value)
}

fn size_to_sql(value: Option<u64>) -> Result<Option<i64>, StoreError> {
    value
        .map(|value| i64::try_from(value).map_err(|_| StoreError::InvalidMetadata))
        .transpose()
}

fn size_from_sql(value: i64) -> Result<u64, StoreError> {
    u64::try_from(value).map_err(|_| StoreError::InvalidMetadata)
}

fn map_insert_error(error: SqliteError) -> StoreError {
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

    const ARTIFACT_A: &str = "01890f3a-7b2c-7d45-8a61-3c4e5f607190";
    const DIGEST_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

    #[test]
    fn sha256_digest_is_exact_lowercase_hex() {
        let digest = Sha256Digest::parse(DIGEST_A).expect("valid digest");
        assert_eq!(digest.as_str(), DIGEST_A);
        assert_eq!(digest.to_string(), DIGEST_A);

        for invalid in [
            "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaag",
            " aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        ] {
            assert_eq!(Sha256Digest::parse(invalid), Err(StoreError::InvalidDigest));
        }
    }

    #[test]
    fn artifact_insert_round_trips_identity_and_metadata_without_bytes() {
        let db = TestDb::new("artifact-insert");
        let mut store = Store::open(&db.path).unwrap();
        let id = CanonicalId::parse(ARTIFACT_A).unwrap();
        let digest = Sha256Digest::parse(DIGEST_A).unwrap();

        let stored = store
            .insert_artifact_metadata(
                id,
                Some(&digest),
                Some(42),
                ArtifactMetadataUpdate {
                    media_type: Some("text/plain"),
                    retention_ref: Some("retention/default"),
                },
            )
            .unwrap();

        assert_eq!(stored.id(), id);
        assert_eq!(stored.revision(), Revision::INITIAL);
        assert_eq!(stored.sha256(), Some(&digest));
        assert_eq!(stored.size_bytes(), Some(42));
        assert_eq!(stored.media_type(), Some("text/plain"));
        assert_eq!(stored.retention_ref(), Some("retention/default"));
    }

    #[test]
    fn artifact_metadata_revision_preserves_digest_and_size_exactly() {
        let db = TestDb::new("artifact-cas");
        let mut store = Store::open(&db.path).unwrap();
        let id = CanonicalId::parse(ARTIFACT_A).unwrap();
        let digest = Sha256Digest::parse(DIGEST_A).unwrap();
        store
            .insert_artifact_metadata(
                id,
                Some(&digest),
                Some(42),
                ArtifactMetadataUpdate::default(),
            )
            .unwrap();

        let updated = store
            .compare_and_swap_artifact_metadata(
                id,
                Revision::INITIAL,
                ArtifactMetadataUpdate {
                    media_type: Some("application/octet-stream"),
                    retention_ref: Some("retention/pinned"),
                },
            )
            .unwrap();

        assert_eq!(updated.revision().get(), 2);
        assert_eq!(updated.sha256(), Some(&digest));
        assert_eq!(updated.size_bytes(), Some(42));
        assert_eq!(updated.media_type(), Some("application/octet-stream"));
        assert_eq!(updated.retention_ref(), Some("retention/pinned"));
    }

    #[test]
    fn stale_artifact_revision_does_not_mutate_metadata() {
        let db = TestDb::new("artifact-stale");
        let mut store = Store::open(&db.path).unwrap();
        let id = CanonicalId::parse(ARTIFACT_A).unwrap();
        store
            .insert_artifact_metadata(id, None, None, ArtifactMetadataUpdate::default())
            .unwrap();
        store
            .compare_and_swap_artifact_metadata(
                id,
                Revision::INITIAL,
                ArtifactMetadataUpdate {
                    media_type: Some("text/plain"),
                    retention_ref: None,
                },
            )
            .unwrap();

        assert_eq!(
            store.compare_and_swap_artifact_metadata(
                id,
                Revision::INITIAL,
                ArtifactMetadataUpdate {
                    media_type: Some("text/html"),
                    retention_ref: None,
                },
            ),
            Err(StoreError::RevisionConflict)
        );
        let stored = store.artifact_metadata(id).unwrap();
        assert_eq!(stored.revision().get(), 2);
        assert_eq!(stored.media_type(), Some("text/plain"));
    }

    #[test]
    fn artifact_max_revision_fails_before_metadata_mutation_or_wraparound() {
        let db = TestDb::new("artifact-overflow");
        let mut store = Store::open(&db.path).unwrap();
        let id = CanonicalId::parse(ARTIFACT_A).unwrap();
        store
            .insert_artifact_metadata(
                id,
                None,
                None,
                ArtifactMetadataUpdate {
                    media_type: Some("text/plain"),
                    retention_ref: None,
                },
            )
            .unwrap();
        store
            .connection
            .execute(
                "UPDATE artifacts SET revision = ?1 WHERE id = ?2",
                (i64::from(u32::MAX), id.to_canonical_text()),
            )
            .unwrap();

        assert_eq!(
            store.compare_and_swap_artifact_metadata(
                id,
                Revision::new(u32::MAX).unwrap(),
                ArtifactMetadataUpdate {
                    media_type: Some("text/html"),
                    retention_ref: Some("retention/new"),
                },
            ),
            Err(StoreError::RevisionOverflow)
        );
        let stored = store.artifact_metadata(id).unwrap();
        assert_eq!(stored.revision().get(), u32::MAX);
        assert_eq!(stored.media_type(), Some("text/plain"));
        assert_eq!(stored.retention_ref(), None);
    }

    #[test]
    fn artifact_size_rejects_values_outside_sqlite_integer_domain() {
        let db = TestDb::new("artifact-size");
        let mut store = Store::open(&db.path).unwrap();
        let id = CanonicalId::parse(ARTIFACT_A).unwrap();
        assert_eq!(
            store.insert_artifact_metadata(
                id,
                None,
                Some(i64::MAX as u64 + 1),
                ArtifactMetadataUpdate::default(),
            ),
            Err(StoreError::InvalidMetadata)
        );
        assert_eq!(store.artifact_metadata(id), Err(StoreError::NotFound));
    }

    #[test]
    fn duplicate_artifact_identity_fails_closed() {
        let db = TestDb::new("artifact-duplicate");
        let mut store = Store::open(&db.path).unwrap();
        let id = CanonicalId::parse(ARTIFACT_A).unwrap();
        store
            .insert_artifact_metadata(id, None, None, ArtifactMetadataUpdate::default())
            .unwrap();
        assert_eq!(
            store.insert_artifact_metadata(id, None, None, ArtifactMetadataUpdate::default()),
            Err(StoreError::DuplicateConflict)
        );
    }

    #[test]
    fn artifact_api_contains_no_blob_or_cas_operations() {
        let source = include_str!("artifact.rs");
        let production = source
            .split("#[cfg(test)]")
            .next()
            .expect("production source");
        for forbidden in ["blob", "ingest", "deduplic", "garbage_collect", "cas_path"] {
            assert!(!production.to_ascii_lowercase().contains(forbidden));
        }
    }
}
