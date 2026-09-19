use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

use sha2::{Digest, Sha256};

use crate::{CanonicalId, Revision, Sha256Digest, Store, StoreError};

const BUFFER_BYTES: usize = 64 * 1024;
const MAX_ARTIFACT_BYTES: u64 = i64::MAX as u64;
const TEMP_ATTEMPTS: usize = 64;
static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CasError {
    InvalidRoot,
    InvalidLimit,
    Io,
    SizeLimitExceeded,
    DigestMismatch,
    BlobNotFound,
    InvalidEntry,
    CorruptBlob,
    TempUnavailable,
}

impl fmt::Display for CasError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::InvalidRoot => "artifact CAS root is invalid",
            Self::InvalidLimit => "artifact CAS byte limit is invalid",
            Self::Io => "artifact CAS filesystem operation failed",
            Self::SizeLimitExceeded => "artifact CAS byte limit was exceeded",
            Self::DigestMismatch => "artifact CAS digest verification failed",
            Self::BlobNotFound => "artifact CAS blob was not found",
            Self::InvalidEntry => "artifact CAS entry type is invalid",
            Self::CorruptBlob => "artifact CAS existing blob failed integrity verification",
            Self::TempUnavailable => "artifact CAS temporary name could not be allocated",
        })
    }
}

impl std::error::Error for CasError {}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactBindingError {
    Cas(CasError),
    Store(StoreError),
}

impl fmt::Display for ArtifactBindingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Cas(_) => "artifact binding failed during CAS verification",
            Self::Store(_) => "artifact binding failed during metadata update",
        })
    }
}

impl std::error::Error for ArtifactBindingError {}

impl From<CasError> for ArtifactBindingError {
    fn from(value: CasError) -> Self {
        Self::Cas(value)
    }
}

impl From<StoreError> for ArtifactBindingError {
    fn from(value: StoreError) -> Self {
        Self::Store(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CasBlob {
    digest: Sha256Digest,
    size_bytes: u64,
}

impl CasBlob {
    pub fn digest(&self) -> &Sha256Digest {
        &self.digest
    }

    pub const fn size_bytes(&self) -> u64 {
        self.size_bytes
    }
}

pub struct VerifiedBlob {
    file: File,
    digest: Sha256Digest,
    size_bytes: u64,
}

impl VerifiedBlob {
    pub fn digest(&self) -> &Sha256Digest {
        &self.digest
    }

    pub const fn size_bytes(&self) -> u64 {
        self.size_bytes
    }
}

impl Read for VerifiedBlob {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        self.file.read(buffer)
    }
}

pub struct ArtifactCas {
    namespace: PathBuf,
}

impl ArtifactCas {
    pub fn open(root: impl AsRef<Path>) -> Result<Self, CasError> {
        let root = root.as_ref();
        validate_existing_directory(root).map_err(|_| CasError::InvalidRoot)?;
        let namespace = root.join("sha256");
        ensure_directory(&namespace)?;
        Ok(Self { namespace })
    }

    pub fn ingest<R: Read>(
        &self,
        mut reader: R,
        max_bytes: u64,
        expected: Option<&Sha256Digest>,
    ) -> Result<CasBlob, CasError> {
        validate_limit(max_bytes)?;
        validate_existing_directory(&self.namespace)?;

        let (incoming_path, mut incoming) = create_temp_file(&self.namespace, "incoming")?;
        let incoming_guard = TempGuard::new(incoming_path.clone());
        let mut hasher = Sha256::new();
        let mut size_bytes = 0_u64;
        let mut buffer = [0_u8; BUFFER_BYTES];

        loop {
            let read = reader.read(&mut buffer).map_err(|_| CasError::Io)?;
            if read == 0 {
                break;
            }
            size_bytes = size_bytes
                .checked_add(read as u64)
                .ok_or(CasError::SizeLimitExceeded)?;
            if size_bytes > max_bytes {
                return Err(CasError::SizeLimitExceeded);
            }
            incoming
                .write_all(&buffer[..read])
                .map_err(|_| CasError::Io)?;
            hasher.update(&buffer[..read]);
        }

        incoming.flush().map_err(|_| CasError::Io)?;
        incoming.sync_all().map_err(|_| CasError::Io)?;
        drop(incoming);

        let digest = digest_from_hasher(hasher)?;
        if expected.is_some_and(|value| value != &digest) {
            return Err(CasError::DigestMismatch);
        }

        let shard = self.ensure_shard(&digest)?;
        let shard_temp = create_temp_link(&incoming_path, &shard, "publish")?;
        let shard_guard = TempGuard::new(shard_temp.clone());
        fs::remove_file(&incoming_path).map_err(|_| CasError::Io)?;
        incoming_guard.disarm();

        let final_path = shard.join(digest.as_str());
        match fs::hard_link(&shard_temp, &final_path) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
                let existing = verify_path(&final_path, &digest, size_bytes)
                    .map_err(|_| CasError::CorruptBlob)?;
                if existing != size_bytes {
                    return Err(CasError::CorruptBlob);
                }
            }
            Err(_) => return Err(CasError::Io),
        }

        fs::remove_file(&shard_temp).map_err(|_| CasError::Io)?;
        shard_guard.disarm();
        Ok(CasBlob { digest, size_bytes })
    }

    pub fn bind_artifact(
        &self,
        store: &mut Store,
        id: CanonicalId,
        expected: Revision,
        blob: &CasBlob,
    ) -> Result<Revision, ArtifactBindingError> {
        let verified = self.open_verified(blob.digest(), blob.size_bytes())?;
        if verified.size_bytes() != blob.size_bytes() {
            return Err(CasError::CorruptBlob.into());
        }

        let current = store.artifact_metadata(id)?;
        if current.revision() != expected {
            return Err(StoreError::RevisionConflict.into());
        }
        store
            .compare_and_swap_artifact_metadata(
                id,
                expected,
                Some(blob.digest()),
                Some(blob.size_bytes()),
                current.media_type(),
                current.retention_ref(),
            )
            .map_err(ArtifactBindingError::from)
    }

    pub fn open_verified(
        &self,
        digest: &Sha256Digest,
        max_bytes: u64,
    ) -> Result<VerifiedBlob, CasError> {
        validate_limit(max_bytes)?;
        validate_existing_directory(&self.namespace)?;
        let shard = self.shard_path(digest);
        match fs::symlink_metadata(&shard) {
            Ok(metadata) if metadata.file_type().is_symlink() || !metadata.is_dir() => {
                return Err(CasError::InvalidEntry);
            }
            Ok(_) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                return Err(CasError::BlobNotFound);
            }
            Err(_) => return Err(CasError::Io),
        }
        let path = shard.join(digest.as_str());
        verify_open_file(&path, digest, max_bytes)
    }

    fn ensure_shard(&self, digest: &Sha256Digest) -> Result<PathBuf, CasError> {
        validate_existing_directory(&self.namespace)?;
        let shard = self.shard_path(digest);
        ensure_directory(&shard)?;
        Ok(shard)
    }

    fn shard_path(&self, digest: &Sha256Digest) -> PathBuf {
        self.namespace.join(&digest.as_str()[..2])
    }

    #[cfg(test)]
    fn blob_path(&self, digest: &Sha256Digest) -> PathBuf {
        self.shard_path(digest).join(digest.as_str())
    }
}

fn validate_limit(max_bytes: u64) -> Result<(), CasError> {
    if max_bytes > MAX_ARTIFACT_BYTES {
        Err(CasError::InvalidLimit)
    } else {
        Ok(())
    }
}

fn validate_existing_directory(path: &Path) -> Result<(), CasError> {
    let metadata = fs::symlink_metadata(path).map_err(|_| CasError::InvalidEntry)?;
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(CasError::InvalidEntry);
    }
    Ok(())
}

fn ensure_directory(path: &Path) -> Result<(), CasError> {
    match fs::create_dir(path) {
        Ok(()) => validate_existing_directory(path),
        Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {
            validate_existing_directory(path)
        }
        Err(_) => Err(CasError::Io),
    }
}

fn create_temp_file(parent: &Path, label: &str) -> Result<(PathBuf, File), CasError> {
    for _ in 0..TEMP_ATTEMPTS {
        let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let path = parent.join(format!(".{label}-{}-{sequence}", std::process::id()));
        match OpenOptions::new().write(true).create_new(true).open(&path) {
            Ok(file) => return Ok((path, file)),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(_) => return Err(CasError::Io),
        }
    }
    Err(CasError::TempUnavailable)
}

fn create_temp_link(source: &Path, parent: &Path, label: &str) -> Result<PathBuf, CasError> {
    for _ in 0..TEMP_ATTEMPTS {
        let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let path = parent.join(format!(".{label}-{}-{sequence}", std::process::id()));
        match fs::hard_link(source, &path) {
            Ok(()) => return Ok(path),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => continue,
            Err(_) => return Err(CasError::Io),
        }
    }
    Err(CasError::TempUnavailable)
}

fn digest_from_hasher(hasher: Sha256) -> Result<Sha256Digest, CasError> {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let output = hasher.finalize();
    let mut text = String::with_capacity(64);
    for byte in output {
        text.push(HEX[(byte >> 4) as usize] as char);
        text.push(HEX[(byte & 0x0f) as usize] as char);
    }
    Sha256Digest::parse(&text).map_err(|_| CasError::DigestMismatch)
}

fn verify_path(path: &Path, digest: &Sha256Digest, max_bytes: u64) -> Result<u64, CasError> {
    Ok(verify_open_file(path, digest, max_bytes)?.size_bytes)
}

fn verify_open_file(
    path: &Path,
    digest: &Sha256Digest,
    max_bytes: u64,
) -> Result<VerifiedBlob, CasError> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            return Err(CasError::BlobNotFound);
        }
        Err(_) => return Err(CasError::Io),
    };
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(CasError::InvalidEntry);
    }
    if metadata.len() > max_bytes {
        return Err(CasError::SizeLimitExceeded);
    }

    let mut file = File::open(path).map_err(|_| CasError::Io)?;
    let mut hasher = Sha256::new();
    let mut size_bytes = 0_u64;
    let mut buffer = [0_u8; BUFFER_BYTES];
    loop {
        let read = file.read(&mut buffer).map_err(|_| CasError::Io)?;
        if read == 0 {
            break;
        }
        size_bytes = size_bytes
            .checked_add(read as u64)
            .ok_or(CasError::SizeLimitExceeded)?;
        if size_bytes > max_bytes {
            return Err(CasError::SizeLimitExceeded);
        }
        hasher.update(&buffer[..read]);
    }
    if size_bytes != metadata.len() || digest_from_hasher(hasher)? != *digest {
        return Err(CasError::DigestMismatch);
    }
    file.seek(SeekFrom::Start(0)).map_err(|_| CasError::Io)?;
    Ok(VerifiedBlob {
        file,
        digest: digest.clone(),
        size_bytes,
    })
}

struct TempGuard {
    path: Option<PathBuf>,
}

impl TempGuard {
    fn new(path: PathBuf) -> Self {
        Self { path: Some(path) }
    }

    fn disarm(mut self) {
        self.path = None;
    }
}

impl Drop for TempGuard {
    fn drop(&mut self) {
        if let Some(path) = self.path.take() {
            let _ = fs::remove_file(path);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::TestDb;
    use std::io::{Cursor, Read};

    const ARTIFACT_A: &str = "01890f3a-7b2c-7d45-8a61-3c4e5f607190";
    const ARTIFACT_B: &str = "01890f3a-7b2c-7d45-8a61-3c4e5f607192";
    const ABC_SHA256: &str = "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad";
    const EMPTY_SHA256: &str = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

    struct TestRoot(PathBuf);

    impl TestRoot {
        fn new(label: &str) -> Self {
            let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!(
                "kernux-cas-test-{label}-{}-{sequence}",
                std::process::id()
            ));
            fs::create_dir(&path).unwrap();
            Self(path)
        }
    }

    impl Drop for TestRoot {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn ingest_verified_read_and_layout_round_trip() {
        let root = TestRoot::new("round-trip");
        let cas = ArtifactCas::open(&root.0).unwrap();
        let blob = cas.ingest(Cursor::new(b"abc"), 3, None).unwrap();
        assert_eq!(blob.digest().as_str(), ABC_SHA256);
        assert_eq!(blob.size_bytes(), 3);
        assert_eq!(
            cas.blob_path(blob.digest()),
            root.0.join("sha256").join("ba").join(ABC_SHA256)
        );

        let mut verified = cas.open_verified(blob.digest(), 3).unwrap();
        let mut bytes = Vec::new();
        verified.read_to_end(&mut bytes).unwrap();
        assert_eq!(bytes, b"abc");
        assert_eq!(verified.size_bytes(), 3);
        assert_eq!(verified.digest(), blob.digest());
    }

    #[test]
    fn empty_content_is_a_valid_blob() {
        let root = TestRoot::new("empty");
        let cas = ArtifactCas::open(&root.0).unwrap();
        let blob = cas.ingest(Cursor::new([]), 0, None).unwrap();
        assert_eq!(blob.digest().as_str(), EMPTY_SHA256);
        assert_eq!(blob.size_bytes(), 0);
        assert!(cas.open_verified(blob.digest(), 0).is_ok());
    }

    #[test]
    fn explicit_size_bound_rejects_without_publication_or_temp_leak() {
        let root = TestRoot::new("limit");
        let cas = ArtifactCas::open(&root.0).unwrap();
        let error = cas.ingest(Cursor::new(b"abcd"), 3, None).unwrap_err();
        assert_eq!(error, CasError::SizeLimitExceeded);
        let entries = fs::read_dir(root.0.join("sha256")).unwrap().count();
        assert_eq!(entries, 0);
    }

    #[test]
    fn expected_digest_mismatch_fails_before_publication() {
        let root = TestRoot::new("expected");
        let cas = ArtifactCas::open(&root.0).unwrap();
        let expected = Sha256Digest::parse(EMPTY_SHA256).unwrap();
        assert_eq!(
            cas.ingest(Cursor::new(b"abc"), 3, Some(&expected)),
            Err(CasError::DigestMismatch)
        );
        assert!(!cas.blob_path(&expected).exists());
    }

    #[test]
    fn identical_content_reuses_one_final_blob_without_mutation() {
        let root = TestRoot::new("dedup");
        let cas = ArtifactCas::open(&root.0).unwrap();
        let first = cas.ingest(Cursor::new(b"abc"), 3, None).unwrap();
        let second = cas.ingest(Cursor::new(b"abc"), 3, None).unwrap();
        assert_eq!(first, second);
        assert_eq!(fs::read(cas.blob_path(first.digest())).unwrap(), b"abc");
    }

    #[test]
    fn missing_blob_returns_not_found() {
        let root = TestRoot::new("missing");
        let cas = ArtifactCas::open(&root.0).unwrap();
        let digest = Sha256Digest::parse(ABC_SHA256).unwrap();
        assert!(matches!(
            cas.open_verified(&digest, 3),
            Err(CasError::BlobNotFound)
        ));
    }

    #[test]
    fn multi_chunk_exact_boundary_round_trips() {
        let root = TestRoot::new("multi-chunk");
        let cas = ArtifactCas::open(&root.0).unwrap();
        let bytes = vec![0x5a; BUFFER_BYTES * 2 + 17];
        let blob = cas
            .ingest(Cursor::new(&bytes), bytes.len() as u64, None)
            .unwrap();
        assert_eq!(blob.size_bytes(), bytes.len() as u64);
        let mut verified = cas
            .open_verified(blob.digest(), bytes.len() as u64)
            .unwrap();
        let mut round_trip = Vec::new();
        verified.read_to_end(&mut round_trip).unwrap();
        assert_eq!(round_trip, bytes);
    }

    #[test]
    fn corrupt_existing_blob_is_not_overwritten_or_repaired() {
        let root = TestRoot::new("corrupt-existing");
        let cas = ArtifactCas::open(&root.0).unwrap();
        let blob = cas.ingest(Cursor::new(b"abc"), 3, None).unwrap();
        let path = cas.blob_path(blob.digest());
        fs::write(&path, b"xyz").unwrap();

        assert_eq!(
            cas.ingest(Cursor::new(b"abc"), 3, None),
            Err(CasError::CorruptBlob)
        );
        assert_eq!(fs::read(path).unwrap(), b"xyz");
    }

    #[test]
    fn tampered_and_truncated_blobs_fail_verified_read() {
        for (label, replacement) in [
            ("tampered", b"xyz".as_slice()),
            ("truncated", b"ab".as_slice()),
        ] {
            let root = TestRoot::new(label);
            let cas = ArtifactCas::open(&root.0).unwrap();
            let blob = cas.ingest(Cursor::new(b"abc"), 3, None).unwrap();
            fs::write(cas.blob_path(blob.digest()), replacement).unwrap();
            assert_eq!(
                cas.open_verified(blob.digest(), 3).err(),
                Some(CasError::DigestMismatch)
            );
        }
    }

    #[test]
    fn non_regular_final_entry_is_rejected() {
        let root = TestRoot::new("non-regular");
        let cas = ArtifactCas::open(&root.0).unwrap();
        let digest = Sha256Digest::parse(ABC_SHA256).unwrap();
        let shard = root.0.join("sha256").join("ba");
        fs::create_dir(&shard).unwrap();
        fs::create_dir(shard.join(ABC_SHA256)).unwrap();

        assert!(matches!(
            cas.open_verified(&digest, 3),
            Err(CasError::InvalidEntry)
        ));
    }

    #[cfg(unix)]
    #[test]
    fn symbolic_link_root_is_rejected() {
        use std::os::unix::fs::symlink;

        let target = TestRoot::new("symlink-target");
        let links = TestRoot::new("symlink-parent");
        let link = links.0.join("cas-link");
        symlink(&target.0, &link).unwrap();
        assert!(matches!(
            ArtifactCas::open(&link),
            Err(CasError::InvalidRoot)
        ));
    }

    #[test]
    fn invalid_root_namespace_and_limit_fail_closed() {
        let root = TestRoot::new("invalid-boundaries");
        let file_root = root.0.join("not-a-directory");
        fs::write(&file_root, b"x").unwrap();
        assert!(matches!(
            ArtifactCas::open(&file_root),
            Err(CasError::InvalidRoot)
        ));

        let bad_namespace = TestRoot::new("bad-namespace");
        fs::write(bad_namespace.0.join("sha256"), b"x").unwrap();
        assert!(matches!(
            ArtifactCas::open(&bad_namespace.0),
            Err(CasError::InvalidEntry)
        ));

        let cas = ArtifactCas::open(&root.0).unwrap();
        let digest = Sha256Digest::parse(ABC_SHA256).unwrap();
        assert_eq!(
            cas.ingest(Cursor::new(b"abc"), u64::MAX, None),
            Err(CasError::InvalidLimit)
        );
        assert_eq!(
            cas.open_verified(&digest, u64::MAX).err(),
            Some(CasError::InvalidLimit)
        );
    }

    #[test]
    fn ordinary_success_and_failure_leave_no_temp_names() {
        fn temp_names(root: &Path) -> Vec<PathBuf> {
            let mut pending = vec![root.to_path_buf()];
            let mut found = Vec::new();
            while let Some(directory) = pending.pop() {
                for entry in fs::read_dir(directory).unwrap() {
                    let entry = entry.unwrap();
                    let path = entry.path();
                    if entry.file_type().unwrap().is_dir() {
                        pending.push(path);
                    } else if entry.file_name().to_string_lossy().starts_with('.') {
                        found.push(path);
                    }
                }
            }
            found
        }

        let root = TestRoot::new("temp-cleanup");
        let cas = ArtifactCas::open(&root.0).unwrap();
        cas.ingest(Cursor::new(b"abc"), 3, None).unwrap();
        assert!(temp_names(&root.0).is_empty());

        assert_eq!(
            cas.ingest(Cursor::new(b"abcd"), 3, None),
            Err(CasError::SizeLimitExceeded)
        );
        assert!(temp_names(&root.0).is_empty());
    }

    #[test]
    fn artifact_binding_preserves_media_retention_and_revision_cas() {
        let root = TestRoot::new("binding");
        let cas = ArtifactCas::open(&root.0).unwrap();
        let blob = cas.ingest(Cursor::new(b"abc"), 3, None).unwrap();
        let db = TestDb::new("cas-binding");
        let mut store = Store::open(&db.path).unwrap();
        let id = CanonicalId::parse(ARTIFACT_A).unwrap();
        store
            .insert_artifact(id, None, None, Some("text/plain"), Some("retention/pinned"))
            .unwrap();

        let revision = cas
            .bind_artifact(&mut store, id, Revision::INITIAL, &blob)
            .unwrap();
        assert_eq!(revision.get(), 2);
        let metadata = store.artifact_metadata(id).unwrap();
        assert_eq!(metadata.revision(), revision);
        assert_eq!(metadata.sha256(), Some(blob.digest()));
        assert_eq!(metadata.size_bytes(), Some(3));
        assert_eq!(metadata.media_type(), Some("text/plain"));
        assert_eq!(metadata.retention_ref(), Some("retention/pinned"));
    }

    #[test]
    fn binding_reverifies_blob_before_any_metadata_mutation() {
        let root = TestRoot::new("binding-reverify");
        let cas = ArtifactCas::open(&root.0).unwrap();
        let blob = cas.ingest(Cursor::new(b"abc"), 3, None).unwrap();
        fs::write(cas.blob_path(blob.digest()), b"xyz").unwrap();

        let db = TestDb::new("binding-reverify");
        let mut store = Store::open(&db.path).unwrap();
        let id = CanonicalId::parse(ARTIFACT_A).unwrap();
        store.insert_artifact(id, None, None, None, None).unwrap();

        assert_eq!(
            cas.bind_artifact(&mut store, id, Revision::INITIAL, &blob),
            Err(ArtifactBindingError::Cas(CasError::DigestMismatch))
        );
        let metadata = store.artifact_metadata(id).unwrap();
        assert_eq!(metadata.revision(), Revision::INITIAL);
        assert_eq!(metadata.sha256(), None);
        assert_eq!(metadata.size_bytes(), None);
    }

    #[test]
    fn digest_identity_conflict_leaves_blob_and_metadata_unchanged() {
        let root = TestRoot::new("binding-digest-conflict");
        let cas = ArtifactCas::open(&root.0).unwrap();
        let blob = cas.ingest(Cursor::new(b"abc"), 3, None).unwrap();

        let db = TestDb::new("binding-digest-conflict");
        let mut store = Store::open(&db.path).unwrap();
        let id = CanonicalId::parse(ARTIFACT_A).unwrap();
        let existing = Sha256Digest::parse(EMPTY_SHA256).unwrap();
        store
            .insert_artifact(
                id,
                Some(&existing),
                Some(0),
                None,
                Some("retention/original"),
            )
            .unwrap();

        assert_eq!(
            cas.bind_artifact(&mut store, id, Revision::INITIAL, &blob),
            Err(ArtifactBindingError::Store(
                StoreError::ArtifactDigestConflict
            ))
        );
        assert!(cas.open_verified(blob.digest(), 3).is_ok());
        let metadata = store.artifact_metadata(id).unwrap();
        assert_eq!(metadata.revision(), Revision::INITIAL);
        assert_eq!(metadata.sha256(), Some(&existing));
        assert_eq!(metadata.size_bytes(), Some(0));
        assert_eq!(metadata.retention_ref(), Some("retention/original"));
    }

    #[test]
    fn stale_metadata_conflict_leaves_published_blob_intact() {
        let root = TestRoot::new("binding-conflict");
        let cas = ArtifactCas::open(&root.0).unwrap();
        let blob = cas.ingest(Cursor::new(b"abc"), 3, None).unwrap();

        let db = TestDb::new("binding-conflict");
        let mut store = Store::open(&db.path).unwrap();
        let id = CanonicalId::parse(ARTIFACT_A).unwrap();
        store.insert_artifact(id, None, None, None, None).unwrap();
        let revision_two = store
            .compare_and_swap_artifact_metadata(
                id,
                Revision::INITIAL,
                None,
                None,
                Some("text/plain"),
                Some("retention/changed"),
            )
            .unwrap();

        assert_eq!(
            cas.bind_artifact(&mut store, id, Revision::INITIAL, &blob),
            Err(ArtifactBindingError::Store(StoreError::RevisionConflict))
        );
        assert!(cas.open_verified(blob.digest(), 3).is_ok());
        let metadata = store.artifact_metadata(id).unwrap();
        assert_eq!(metadata.revision(), revision_two);
        assert_eq!(metadata.sha256(), None);
        assert_eq!(metadata.retention_ref(), Some("retention/changed"));
    }

    #[test]
    fn two_logical_artifacts_share_bytes_without_merging_metadata() {
        let root = TestRoot::new("logical-identity");
        let cas = ArtifactCas::open(&root.0).unwrap();
        let blob = cas.ingest(Cursor::new(b"abc"), 3, None).unwrap();

        let db = TestDb::new("logical-identity");
        let mut store = Store::open(&db.path).unwrap();
        let first_id = CanonicalId::parse(ARTIFACT_A).unwrap();
        let second_id = CanonicalId::parse(ARTIFACT_B).unwrap();
        store
            .insert_artifact(
                first_id,
                None,
                None,
                Some("text/plain"),
                Some("retention/short"),
            )
            .unwrap();
        store
            .insert_artifact(
                second_id,
                None,
                None,
                Some("application/octet-stream"),
                Some("retention/long"),
            )
            .unwrap();

        cas.bind_artifact(&mut store, first_id, Revision::INITIAL, &blob)
            .unwrap();
        cas.bind_artifact(&mut store, second_id, Revision::INITIAL, &blob)
            .unwrap();

        let first = store.artifact_metadata(first_id).unwrap();
        let second = store.artifact_metadata(second_id).unwrap();
        assert_ne!(first.id(), second.id());
        assert_eq!(first.sha256(), second.sha256());
        assert_eq!(first.size_bytes(), second.size_bytes());
        assert_eq!(first.retention_ref(), Some("retention/short"));
        assert_eq!(second.retention_ref(), Some("retention/long"));
        assert_eq!(first.media_type(), Some("text/plain"));
        assert_eq!(second.media_type(), Some("application/octet-stream"));
    }

    #[test]
    fn reopened_cas_verifies_published_blob() {
        let root = TestRoot::new("reopen");
        let digest = {
            let cas = ArtifactCas::open(&root.0).unwrap();
            cas.ingest(Cursor::new(b"abc"), 3, None)
                .unwrap()
                .digest()
                .clone()
        };
        let cas = ArtifactCas::open(&root.0).unwrap();
        let mut verified = cas.open_verified(&digest, 3).unwrap();
        let mut bytes = Vec::new();
        verified.read_to_end(&mut bytes).unwrap();
        assert_eq!(bytes, b"abc");
    }
}
