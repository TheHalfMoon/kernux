use std::fmt;

use rusqlite::OptionalExtension;

use crate::metadata::{map_insert_error, revision_from_sql};
use crate::{CanonicalId, Revision, Store, StoreError};

const MAX_SQLITE_SIZE: u64 = i64::MAX as u64;
const MAX_MEDIA_TYPE_LEN: usize = 255;
const MAX_RETENTION_REF_LEN: usize = 2048;
const MAX_OWNER_KIND_LEN: usize = 32;
const MAX_CONFIG_TYPE_LEN: usize = 64;
const MAX_CONFIG_REF_LEN: usize = 2048;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Sha256Digest(String);

impl Sha256Digest {
    pub fn parse(value: &str) -> Result<Self, StoreError> {
        if value.len() != 64
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
        f.write_str("Sha256Digest(<redacted>)")
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
    pub const fn id(&self) -> CanonicalId {
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdapterConfigRef {
    owner_kind: String,
    owner_id: CanonicalId,
    config_type: String,
    config_ref: String,
}

impl AdapterConfigRef {
    pub fn new(
        owner_kind: &str,
        owner_id: CanonicalId,
        config_type: &str,
        config_ref: &str,
    ) -> Result<Self, StoreError> {
        validate_bounded_text(owner_kind, MAX_OWNER_KIND_LEN)?;
        validate_bounded_text(config_type, MAX_CONFIG_TYPE_LEN)?;
        validate_bounded_text(config_ref, MAX_CONFIG_REF_LEN)?;
        Ok(Self {
            owner_kind: owner_kind.to_owned(),
            owner_id,
            config_type: config_type.to_owned(),
            config_ref: config_ref.to_owned(),
        })
    }

    pub fn owner_kind(&self) -> &str {
        &self.owner_kind
    }

    pub const fn owner_id(&self) -> CanonicalId {
        self.owner_id
    }

    pub fn config_type(&self) -> &str {
        &self.config_type
    }

    pub fn config_ref(&self) -> &str {
        &self.config_ref
    }
}

impl Store {
    pub fn insert_artifact(
        &mut self,
        id: CanonicalId,
        sha256: Option<&Sha256Digest>,
        size_bytes: Option<u64>,
        media_type: Option<&str>,
        retention_ref: Option<&str>,
    ) -> Result<Revision, StoreError> {
        let size_bytes = validate_artifact_metadata(size_bytes, media_type, retention_ref)?;
        self.connection
            .execute(
                "INSERT INTO artifacts(id, revision, sha256, size_bytes, media_type, retention_ref) \
                 VALUES (?1, 1, ?2, ?3, ?4, ?5)",
                (
                    id.to_canonical_text(),
                    sha256.map(Sha256Digest::as_str),
                    size_bytes,
                    media_type,
                    retention_ref,
                ),
            )
            .map_err(map_insert_error)?;
        Ok(Revision::INITIAL)
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

        artifact_from_sql(id, row.0, row.1, row.2, row.3, row.4)
    }

    pub fn compare_and_swap_artifact_metadata(
        &mut self,
        id: CanonicalId,
        expected: Revision,
        sha256: Option<&Sha256Digest>,
        size_bytes: Option<u64>,
        media_type: Option<&str>,
        retention_ref: Option<&str>,
    ) -> Result<Revision, StoreError> {
        let current = self.artifact_metadata(id)?;
        if current.revision != expected {
            return Err(StoreError::RevisionConflict);
        }
        let next = expected.checked_next()?;
        enforce_digest_invariance(current.sha256.as_ref(), sha256)?;
        let size_bytes = validate_artifact_metadata(size_bytes, media_type, retention_ref)?;

        let changed = self
            .connection
            .execute(
                "UPDATE artifacts SET revision = revision + 1, sha256 = ?1, size_bytes = ?2, \
                 media_type = ?3, retention_ref = ?4 WHERE id = ?5 AND revision = ?6",
                (
                    sha256.map(Sha256Digest::as_str),
                    size_bytes,
                    media_type,
                    retention_ref,
                    id.to_canonical_text(),
                    i64::from(expected.get()),
                ),
            )
            .map_err(|_| StoreError::Database)?;
        if changed == 1 {
            Ok(next)
        } else {
            Err(StoreError::RevisionConflict)
        }
    }

    pub fn insert_adapter_config_ref(
        &mut self,
        record: &AdapterConfigRef,
    ) -> Result<(), StoreError> {
        self.connection
            .execute(
                "INSERT INTO adapter_config_refs(owner_kind, owner_id, config_type, config_ref) \
                 VALUES (?1, ?2, ?3, ?4)",
                (
                    record.owner_kind(),
                    record.owner_id().to_canonical_text(),
                    record.config_type(),
                    record.config_ref(),
                ),
            )
            .map_err(map_insert_error)?;
        Ok(())
    }

    pub fn adapter_config_ref(
        &self,
        owner_kind: &str,
        owner_id: CanonicalId,
        config_type: &str,
    ) -> Result<Option<AdapterConfigRef>, StoreError> {
        validate_bounded_text(owner_kind, MAX_OWNER_KIND_LEN)?;
        validate_bounded_text(config_type, MAX_CONFIG_TYPE_LEN)?;
        let row = self
            .connection
            .query_row(
                "SELECT config_ref FROM adapter_config_refs \
                 WHERE owner_kind = ?1 AND owner_id = ?2 AND config_type = ?3",
                (owner_kind, owner_id.to_canonical_text(), config_type),
                |row| row.get::<_, String>(0),
            )
            .optional()
            .map_err(|_| StoreError::Database)?;
        row.map(|config_ref| AdapterConfigRef::new(owner_kind, owner_id, config_type, &config_ref))
            .transpose()
            .map_err(|_| StoreError::SchemaDrift)
    }
}

fn artifact_from_sql(
    id: CanonicalId,
    revision: i64,
    sha256: Option<String>,
    size_bytes: Option<i64>,
    media_type: Option<String>,
    retention_ref: Option<String>,
) -> Result<ArtifactMetadata, StoreError> {
    let revision = revision_from_sql(revision).map_err(|_| StoreError::SchemaDrift)?;
    let sha256 = sha256
        .map(|value| Sha256Digest::parse(&value))
        .transpose()
        .map_err(|_| StoreError::SchemaDrift)?;
    let size_bytes = size_bytes
        .map(|value| u64::try_from(value).map_err(|_| StoreError::SchemaDrift))
        .transpose()?;
    validate_artifact_metadata(size_bytes, media_type.as_deref(), retention_ref.as_deref())
        .map_err(|_| StoreError::SchemaDrift)?;
    Ok(ArtifactMetadata {
        id,
        revision,
        sha256,
        size_bytes,
        media_type,
        retention_ref,
    })
}

fn validate_artifact_metadata(
    size_bytes: Option<u64>,
    media_type: Option<&str>,
    retention_ref: Option<&str>,
) -> Result<Option<i64>, StoreError> {
    if let Some(value) = media_type {
        validate_bounded_text(value, MAX_MEDIA_TYPE_LEN)?;
    }
    if let Some(value) = retention_ref {
        validate_bounded_text(value, MAX_RETENTION_REF_LEN)?;
    }
    size_bytes
        .map(|value| {
            if value > MAX_SQLITE_SIZE {
                Err(StoreError::InvalidMetadata)
            } else {
                Ok(value as i64)
            }
        })
        .transpose()
}

fn validate_bounded_text(value: &str, max_len: usize) -> Result<(), StoreError> {
    if value.is_empty()
        || value.len() > max_len
        || value.trim() != value
        || value.chars().any(char::is_control)
    {
        return Err(StoreError::InvalidMetadata);
    }
    Ok(())
}

fn enforce_digest_invariance(
    current: Option<&Sha256Digest>,
    proposed: Option<&Sha256Digest>,
) -> Result<(), StoreError> {
    match (current, proposed) {
        (Some(current), Some(proposed)) if current == proposed => Ok(()),
        (Some(_), _) => Err(StoreError::ArtifactDigestConflict),
        (None, _) => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::SCHEMA_V1_SQL;
    use crate::tests::TestDb;

    const ARTIFACT_A: &str = "01890f3a-7b2c-7d45-8a61-3c4e5f607190";
    const OWNER_A: &str = "01890f3a-7b2c-7d45-8a61-3c4e5f607191";
    const DIGEST_A: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
    const DIGEST_B: &str = "1123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    #[test]
    fn sha256_digest_is_exact_lowercase_hex() {
        assert_eq!(Sha256Digest::parse(DIGEST_A).unwrap().as_str(), DIGEST_A);
        for invalid in [
            "0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF0123456789ABCDEF",
            "0123",
            "g123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        ] {
            assert_eq!(Sha256Digest::parse(invalid), Err(StoreError::InvalidDigest));
        }
    }

    #[test]
    fn artifact_metadata_inserts_and_advances_with_invariant_digest() {
        let db = TestDb::new("artifact-cas");
        let mut store = Store::open(&db.path).unwrap();
        let id = CanonicalId::parse(ARTIFACT_A).unwrap();
        let digest = Sha256Digest::parse(DIGEST_A).unwrap();

        assert_eq!(
            store
                .insert_artifact(
                    id,
                    Some(&digest),
                    Some(42),
                    Some("text/plain"),
                    Some("retention/default"),
                )
                .unwrap(),
            Revision::INITIAL
        );
        let first = store.artifact_metadata(id).unwrap();
        assert_eq!(first.revision(), Revision::INITIAL);
        assert_eq!(first.sha256(), Some(&digest));
        assert_eq!(first.size_bytes(), Some(42));
        assert_eq!(first.media_type(), Some("text/plain"));

        let next = store
            .compare_and_swap_artifact_metadata(
                id,
                Revision::INITIAL,
                Some(&digest),
                Some(42),
                Some("text/plain"),
                Some("retention/pinned"),
            )
            .unwrap();
        assert_eq!(next.get(), 2);
        let second = store.artifact_metadata(id).unwrap();
        assert_eq!(second.revision().get(), 2);
        assert_eq!(second.sha256(), Some(&digest));
        assert_eq!(second.retention_ref(), Some("retention/pinned"));
    }

    #[test]
    fn artifact_digest_can_be_learned_once_but_never_changed_or_cleared() {
        let db = TestDb::new("artifact-digest");
        let mut store = Store::open(&db.path).unwrap();
        let id = CanonicalId::parse(ARTIFACT_A).unwrap();
        let digest_a = Sha256Digest::parse(DIGEST_A).unwrap();
        let digest_b = Sha256Digest::parse(DIGEST_B).unwrap();
        store.insert_artifact(id, None, None, None, None).unwrap();

        let revision_two = store
            .compare_and_swap_artifact_metadata(
                id,
                Revision::INITIAL,
                Some(&digest_a),
                Some(100),
                None,
                None,
            )
            .unwrap();
        assert_eq!(revision_two.get(), 2);
        assert_eq!(
            store.compare_and_swap_artifact_metadata(
                id,
                revision_two,
                Some(&digest_b),
                Some(100),
                None,
                None,
            ),
            Err(StoreError::ArtifactDigestConflict)
        );
        assert_eq!(
            store
                .compare_and_swap_artifact_metadata(id, revision_two, None, Some(100), None, None,),
            Err(StoreError::ArtifactDigestConflict)
        );
        let persisted = store.artifact_metadata(id).unwrap();
        assert_eq!(persisted.revision(), revision_two);
        assert_eq!(persisted.sha256(), Some(&digest_a));
    }

    #[test]
    fn artifact_stale_revision_and_duplicate_identity_fail_without_write() {
        let db = TestDb::new("artifact-conflict");
        let mut store = Store::open(&db.path).unwrap();
        let id = CanonicalId::parse(ARTIFACT_A).unwrap();
        store.insert_artifact(id, None, None, None, None).unwrap();
        assert_eq!(
            store.insert_artifact(id, None, None, None, None),
            Err(StoreError::DuplicateConflict)
        );
        let revision_two = store
            .compare_and_swap_artifact_metadata(id, Revision::INITIAL, None, Some(1), None, None)
            .unwrap();
        assert_eq!(
            store.compare_and_swap_artifact_metadata(
                id,
                Revision::INITIAL,
                None,
                Some(2),
                None,
                None,
            ),
            Err(StoreError::RevisionConflict)
        );
        assert_eq!(
            store.artifact_metadata(id).unwrap().revision(),
            revision_two
        );
        assert_eq!(store.artifact_metadata(id).unwrap().size_bytes(), Some(1));
    }

    #[test]
    fn artifact_size_and_optional_text_are_bounded() {
        let db = TestDb::new("artifact-bounds");
        let mut store = Store::open(&db.path).unwrap();
        let id = CanonicalId::parse(ARTIFACT_A).unwrap();
        assert_eq!(
            store.insert_artifact(id, None, Some(u64::MAX), None, None),
            Err(StoreError::InvalidMetadata)
        );
        assert_eq!(
            store.insert_artifact(id, None, None, Some(" text/plain"), None),
            Err(StoreError::InvalidMetadata)
        );
        assert_eq!(
            store.insert_artifact(id, None, None, None, Some("line\nbreak")),
            Err(StoreError::InvalidMetadata)
        );
    }

    #[test]
    fn adapter_config_reference_is_bounded_insert_once_non_secret_reference_metadata() {
        let db = TestDb::new("adapter-ref");
        let mut store = Store::open(&db.path).unwrap();
        let owner = CanonicalId::parse(OWNER_A).unwrap();
        let record = AdapterConfigRef::new(
            "project",
            owner,
            "provider_config",
            "credential-ref://provider/default",
        )
        .unwrap();
        store.insert_adapter_config_ref(&record).unwrap();
        assert_eq!(
            store
                .adapter_config_ref("project", owner, "provider_config")
                .unwrap(),
            Some(record.clone())
        );
        assert_eq!(
            store.insert_adapter_config_ref(&record),
            Err(StoreError::DuplicateConflict)
        );
    }

    #[test]
    fn adapter_config_reference_rejects_empty_unbounded_or_control_text() {
        let owner = CanonicalId::parse(OWNER_A).unwrap();
        assert_eq!(
            AdapterConfigRef::new("", owner, "provider", "ref"),
            Err(StoreError::InvalidMetadata)
        );
        assert_eq!(
            AdapterConfigRef::new("project", owner, "provider", " secret-ref"),
            Err(StoreError::InvalidMetadata)
        );
        assert_eq!(
            AdapterConfigRef::new("project", owner, "provider", "line\nbreak"),
            Err(StoreError::InvalidMetadata)
        );
        assert_eq!(
            AdapterConfigRef::new("x".repeat(33).as_str(), owner, "provider", "ref"),
            Err(StoreError::InvalidMetadata)
        );
    }

    #[test]
    fn artifact_schema_has_metadata_only_and_no_blob_or_secret_value_column() {
        let artifact_sql = SCHEMA_V1_SQL
            .split("CREATE TABLE artifacts (")
            .nth(1)
            .and_then(|value| value.split("CREATE TABLE evidence (").next())
            .expect("artifact table SQL");
        assert!(artifact_sql.contains("sha256 TEXT"));
        assert!(artifact_sql.contains("size_bytes INTEGER"));
        assert!(!artifact_sql.contains(" BLOB"));
        assert!(!artifact_sql.contains("content BLOB"));
        assert!(!artifact_sql.contains("blob_bytes"));
        assert!(!artifact_sql.contains("artifact_blob"));
        assert!(!SCHEMA_V1_SQL.contains("secret_value"));

        let source = include_str!("artifact.rs");
        let production = source.split("#[cfg(test)]").next().unwrap();
        assert!(!production.contains("password"));
        assert!(!production.contains("private_key"));
        assert!(!production.contains("secret_value"));
        assert!(!production.contains("bearer_token"));
    }
}
