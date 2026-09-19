use rusqlite::{OptionalExtension, Transaction, TransactionBehavior};

use crate::metadata::revision_from_sql;
use crate::{CanonicalId, Revision, Store, StoreError};

const MIN_EVENT_TYPE_LEN: usize = 3;
const MAX_EVENT_TYPE_LEN: usize = 128;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventType(String);

impl EventType {
    pub fn parse(value: &str) -> Result<Self, StoreError> {
        if !valid_event_type(value) {
            return Err(StoreError::InvalidEventType);
        }
        Ok(Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StreamOwnerKind {
    Project,
    Task,
    Run,
    Runtime,
}

impl StreamOwnerKind {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Project => "project",
            Self::Task => "task",
            Self::Run => "run",
            Self::Runtime => "runtime",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StreamRef {
    owner_kind: StreamOwnerKind,
    owner_id: CanonicalId,
    owner_revision: Option<Revision>,
}

impl StreamRef {
    pub fn new(
        owner_kind: StreamOwnerKind,
        owner_id: CanonicalId,
        owner_revision: Option<Revision>,
    ) -> Result<Self, StoreError> {
        let owner_revision = normalize_owner_revision(owner_kind, owner_revision)?;
        Ok(Self {
            owner_kind,
            owner_id,
            owner_revision,
        })
    }

    pub const fn owner_kind(&self) -> StreamOwnerKind {
        self.owner_kind
    }

    pub const fn owner_id(&self) -> CanonicalId {
        self.owner_id
    }

    pub const fn owner_revision(&self) -> Option<Revision> {
        self.owner_revision
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventAppend {
    event_id: CanonicalId,
    event_type: EventType,
    stream: StreamRef,
    expected_previous_event_id: Option<CanonicalId>,
}

impl EventAppend {
    pub fn new(
        event_id: CanonicalId,
        event_type: EventType,
        stream: StreamRef,
        expected_previous_event_id: Option<CanonicalId>,
    ) -> Self {
        Self {
            event_id,
            event_type,
            stream,
            expected_previous_event_id,
        }
    }

    pub const fn event_id(&self) -> CanonicalId {
        self.event_id
    }

    pub fn event_type(&self) -> &EventType {
        &self.event_type
    }

    pub const fn stream(&self) -> StreamRef {
        self.stream
    }

    pub const fn expected_previous_event_id(&self) -> Option<CanonicalId> {
        self.expected_previous_event_id
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AcceptedEvent {
    event_id: CanonicalId,
    stream_seq: u64,
    previous_event_id: Option<CanonicalId>,
}

impl AcceptedEvent {
    pub const fn event_id(&self) -> CanonicalId {
        self.event_id
    }

    pub const fn stream_seq(&self) -> u64 {
        self.stream_seq
    }

    pub const fn previous_event_id(&self) -> Option<CanonicalId> {
        self.previous_event_id
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct StreamTip {
    event_id: CanonicalId,
    stream_seq: u64,
}

impl Store {
    pub fn append_event(&mut self, event: &EventAppend) -> Result<AcceptedEvent, StoreError> {
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|_| StoreError::Database)?;

        let duplicate = transaction
            .query_row(
                "SELECT 1 FROM events WHERE id = ?1",
                [event.event_id.to_canonical_text()],
                |_| Ok(()),
            )
            .optional()
            .map_err(|_| StoreError::Database)?;
        if duplicate.is_some() {
            return Err(StoreError::DuplicateConflict);
        }

        let tip = validate_stream_chain(&transaction, event.stream)?;
        let (stream_seq, previous_event_id) = match tip {
            None => {
                if event.expected_previous_event_id.is_some() {
                    return Err(StoreError::EventPredecessorMismatch);
                }
                (1, None)
            }
            Some(tip) => {
                if event.expected_previous_event_id != Some(tip.event_id) {
                    return Err(StoreError::EventPredecessorMismatch);
                }
                (next_stream_sequence(tip.stream_seq)?, Some(tip.event_id))
            }
        };

        let stream_seq_blob = encode_stream_sequence(stream_seq);
        let owner_revision = event
            .stream
            .owner_revision
            .map(|revision| i64::from(revision.get()));
        let previous_event_text = previous_event_id.map(CanonicalId::to_canonical_text);

        transaction
            .execute(
                "INSERT INTO events(\
                    id, revision, event_type, owner_kind, owner_id, owner_revision, stream_seq, previous_event_id\
                 ) VALUES (?1, 1, ?2, ?3, ?4, ?5, ?6, ?7)",
                (
                    event.event_id.to_canonical_text(),
                    event.event_type.as_str(),
                    event.stream.owner_kind.as_str(),
                    event.stream.owner_id.to_canonical_text(),
                    owner_revision,
                    stream_seq_blob.as_slice(),
                    previous_event_text.as_deref(),
                ),
            )
            .map_err(map_event_insert_error)?;
        transaction.commit().map_err(|_| StoreError::Database)?;

        Ok(AcceptedEvent {
            event_id: event.event_id,
            stream_seq,
            previous_event_id,
        })
    }
}

fn validate_stream_chain(
    transaction: &Transaction<'_>,
    stream: StreamRef,
) -> Result<Option<StreamTip>, StoreError> {
    let mut statement = transaction
        .prepare(
            "SELECT id, revision, event_type, owner_revision, stream_seq, previous_event_id \
             FROM events WHERE owner_kind = ?1 AND owner_id = ?2 ORDER BY stream_seq ASC",
        )
        .map_err(|_| StoreError::Database)?;
    let rows = statement
        .query_map(
            (
                stream.owner_kind.as_str(),
                stream.owner_id.to_canonical_text(),
            ),
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, Option<i64>>(3)?,
                    row.get::<_, Vec<u8>>(4)?,
                    row.get::<_, Option<String>>(5)?,
                ))
            },
        )
        .map_err(|_| StoreError::Database)?;

    let mut expected_sequence = 1_u64;
    let mut previous: Option<CanonicalId> = None;
    let mut tip = None;

    for row in rows {
        let (id, revision, event_type, owner_revision, stream_seq, previous_event_id) =
            row.map_err(|_| StoreError::EventStreamCorrupt)?;
        if revision != 1 || EventType::parse(&event_type).is_err() {
            return Err(StoreError::EventStreamCorrupt);
        }

        let id = CanonicalId::parse(&id).map_err(|_| StoreError::EventStreamCorrupt)?;
        let owner_revision = owner_revision
            .map(|value| revision_from_sql(value).map_err(|_| StoreError::EventStreamCorrupt))
            .transpose()?;
        normalize_owner_revision(stream.owner_kind, owner_revision)
            .map_err(|_| StoreError::EventStreamCorrupt)?;

        let stream_seq = decode_stream_sequence(&stream_seq)?;
        if stream_seq != expected_sequence {
            return Err(StoreError::EventStreamCorrupt);
        }

        let stored_previous = previous_event_id
            .map(|value| CanonicalId::parse(&value).map_err(|_| StoreError::EventStreamCorrupt))
            .transpose()?;
        if stored_previous != previous {
            return Err(StoreError::EventStreamCorrupt);
        }

        tip = Some(StreamTip {
            event_id: id,
            stream_seq,
        });
        previous = Some(id);
        expected_sequence = stream_seq.saturating_add(1);
    }

    Ok(tip)
}

fn normalize_owner_revision(
    owner_kind: StreamOwnerKind,
    owner_revision: Option<Revision>,
) -> Result<Option<Revision>, StoreError> {
    match owner_kind {
        StreamOwnerKind::Project | StreamOwnerKind::Task | StreamOwnerKind::Runtime => {
            owner_revision
                .ok_or(StoreError::InvalidStreamOwner)
                .map(Some)
        }
        StreamOwnerKind::Run => match owner_revision {
            None => Ok(None),
            Some(revision) if revision == Revision::INITIAL => Ok(None),
            Some(_) => Err(StoreError::InvalidStreamOwner),
        },
    }
}

fn valid_event_type(value: &str) -> bool {
    if !(MIN_EVENT_TYPE_LEN..=MAX_EVENT_TYPE_LEN).contains(&value.len()) {
        return false;
    }

    let mut tokens = value.split('.');
    let Some(first) = tokens.next() else {
        return false;
    };
    if !valid_event_type_token(first) {
        return false;
    }

    let mut token_count = 1_usize;
    for token in tokens {
        token_count += 1;
        if !valid_event_type_token(token) {
            return false;
        }
    }
    token_count >= 2
}

fn valid_event_type_token(token: &str) -> bool {
    let mut bytes = token.bytes();
    let Some(first) = bytes.next() else {
        return false;
    };
    first.is_ascii_lowercase()
        && bytes.all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
}

fn encode_stream_sequence(value: u64) -> [u8; 8] {
    value.to_be_bytes()
}

fn decode_stream_sequence(value: &[u8]) -> Result<u64, StoreError> {
    let bytes: [u8; 8] = value
        .try_into()
        .map_err(|_| StoreError::EventStreamCorrupt)?;
    let sequence = u64::from_be_bytes(bytes);
    if sequence == 0 {
        return Err(StoreError::EventStreamCorrupt);
    }
    Ok(sequence)
}

fn next_stream_sequence(current: u64) -> Result<u64, StoreError> {
    current
        .checked_add(1)
        .ok_or(StoreError::EventSequenceOverflow)
}

fn map_event_insert_error(error: rusqlite::Error) -> StoreError {
    match error {
        rusqlite::Error::SqliteFailure(inner, _)
            if inner.code == rusqlite::ErrorCode::ConstraintViolation =>
        {
            StoreError::EventStreamCorrupt
        }
        _ => StoreError::Database,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::TestDb;

    const PROJECT_A: &str = "01890f3a-7b2c-7d45-8a61-3c4e5f6071a0";
    const PROJECT_B: &str = "01890f3a-7b2c-7d45-8a61-3c4e5f6071a1";
    const RUN_A: &str = "01890f3a-7b2c-7d45-8a61-3c4e5f6071a2";
    const EVENT_A: &str = "01890f3a-7b2c-7d45-8a61-3c4e5f6071b0";
    const EVENT_B: &str = "01890f3a-7b2c-7d45-8a61-3c4e5f6071b1";
    const EVENT_C: &str = "01890f3a-7b2c-7d45-8a61-3c4e5f6071b2";
    const EVENT_D: &str = "01890f3a-7b2c-7d45-8a61-3c4e5f6071b3";

    fn id(value: &str) -> CanonicalId {
        CanonicalId::parse(value).unwrap()
    }

    fn project_stream(value: &str) -> StreamRef {
        StreamRef::new(StreamOwnerKind::Project, id(value), Some(Revision::INITIAL)).unwrap()
    }

    fn append(
        event_id: &str,
        event_type: &str,
        stream: StreamRef,
        predecessor: Option<&str>,
    ) -> EventAppend {
        EventAppend::new(
            id(event_id),
            EventType::parse(event_type).unwrap(),
            stream,
            predecessor.map(id),
        )
    }

    #[test]
    fn event_type_enforces_lowercase_dotted_core_grammar() {
        for valid in [
            "task.created",
            "runtime.contact_lost",
            "operation.cancel_requested",
            "a.b",
        ] {
            assert_eq!(EventType::parse(valid).unwrap().as_str(), valid);
        }

        for invalid in [
            "task",
            "Task.created",
            "task.Created",
            "task-created.now",
            "task..created",
            ".task.created",
            "task.created.",
            "1task.created",
            "task.1created",
            "task.created-now",
            "task created.now",
        ] {
            assert_eq!(EventType::parse(invalid), Err(StoreError::InvalidEventType));
        }
        assert_eq!(
            EventType::parse(&format!("a.{}", "b".repeat(MAX_EVENT_TYPE_LEN))),
            Err(StoreError::InvalidEventType)
        );
    }

    #[test]
    fn stream_owner_revision_rules_preserve_exact_revision_semantics() {
        let project = id(PROJECT_A);
        let run = id(RUN_A);
        assert_eq!(
            StreamRef::new(StreamOwnerKind::Project, project, None),
            Err(StoreError::InvalidStreamOwner)
        );
        assert_eq!(
            StreamRef::new(StreamOwnerKind::Runtime, project, None),
            Err(StoreError::InvalidStreamOwner)
        );
        assert!(
            StreamRef::new(
                StreamOwnerKind::Task,
                project,
                Some(Revision::new(u32::MAX).unwrap())
            )
            .is_ok()
        );

        let run_without_revision = StreamRef::new(StreamOwnerKind::Run, run, None).unwrap();
        let run_revision_one =
            StreamRef::new(StreamOwnerKind::Run, run, Some(Revision::INITIAL)).unwrap();
        assert_eq!(run_revision_one, run_without_revision);
        assert_eq!(
            StreamRef::new(StreamOwnerKind::Run, run, Some(Revision::new(2).unwrap())),
            Err(StoreError::InvalidStreamOwner)
        );
    }

    #[test]
    fn first_event_gets_sequence_one_without_predecessor_and_exact_blob() {
        let db = TestDb::new("event-first");
        let mut store = Store::open(&db.path).unwrap();
        let event = append(EVENT_A, "task.created", project_stream(PROJECT_A), None);

        let accepted = store.append_event(&event).unwrap();
        assert_eq!(accepted.event_id(), id(EVENT_A));
        assert_eq!(accepted.stream_seq(), 1);
        assert_eq!(accepted.previous_event_id(), None);

        let (revision, blob, previous): (i64, Vec<u8>, Option<String>) = store
            .connection
            .query_row(
                "SELECT revision, stream_seq, previous_event_id FROM events WHERE id = ?1",
                [EVENT_A],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .unwrap();
        assert_eq!(revision, 1);
        assert_eq!(blob, 1_u64.to_be_bytes());
        assert_eq!(previous, None);
    }

    #[test]
    fn successors_increment_exactly_one_and_bind_immediate_predecessor() {
        let db = TestDb::new("event-successor");
        let mut store = Store::open(&db.path).unwrap();
        let stream = project_stream(PROJECT_A);
        store
            .append_event(&append(EVENT_A, "task.created", stream, None))
            .unwrap();
        let second = store
            .append_event(&append(EVENT_B, "operation.started", stream, Some(EVENT_A)))
            .unwrap();
        let third = store
            .append_event(&append(
                EVENT_C,
                "operation.observed",
                stream,
                Some(EVENT_B),
            ))
            .unwrap();

        assert_eq!(second.stream_seq(), 2);
        assert_eq!(second.previous_event_id(), Some(id(EVENT_A)));
        assert_eq!(third.stream_seq(), 3);
        assert_eq!(third.previous_event_id(), Some(id(EVENT_B)));
    }

    #[test]
    fn missing_or_wrong_predecessor_rejects_without_write() {
        let db = TestDb::new("event-predecessor");
        let mut store = Store::open(&db.path).unwrap();
        let stream = project_stream(PROJECT_A);
        store
            .append_event(&append(EVENT_A, "task.created", stream, None))
            .unwrap();

        assert_eq!(
            store.append_event(&append(EVENT_B, "task.updated", stream, None)),
            Err(StoreError::EventPredecessorMismatch)
        );
        assert_eq!(
            store.append_event(&append(EVENT_B, "task.updated", stream, Some(EVENT_C))),
            Err(StoreError::EventPredecessorMismatch)
        );
        let count: i64 = store
            .connection
            .query_row("SELECT COUNT(*) FROM events", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn full_u64_sequence_encoding_is_big_endian_order_preserving_and_nonzero() {
        let values = [1_u64, i64::MAX as u64, i64::MAX as u64 + 1, u64::MAX];
        for value in values {
            assert_eq!(
                decode_stream_sequence(&encode_stream_sequence(value)).unwrap(),
                value
            );
        }

        let mut encoded = values.map(encode_stream_sequence);
        encoded.reverse();
        encoded.sort();
        assert_eq!(
            encoded,
            values.map(encode_stream_sequence),
            "fixed-width big-endian BLOB order must match unsigned numeric order"
        );
        assert_eq!(
            decode_stream_sequence(&0_u64.to_be_bytes()),
            Err(StoreError::EventStreamCorrupt)
        );
        assert_eq!(
            decode_stream_sequence(&[1, 2, 3]),
            Err(StoreError::EventStreamCorrupt)
        );
    }

    #[test]
    fn sequence_overflow_fails_closed_without_wraparound() {
        assert_eq!(
            next_stream_sequence(u64::MAX),
            Err(StoreError::EventSequenceOverflow)
        );
        assert_eq!(next_stream_sequence(u64::MAX - 1).unwrap(), u64::MAX);
    }

    #[test]
    fn owner_revision_is_exact_event_metadata_but_does_not_reset_owner_stream_sequence() {
        let db = TestDb::new("event-owner-revision");
        let mut store = Store::open(&db.path).unwrap();
        let owner = id(PROJECT_A);
        let revision_one =
            StreamRef::new(StreamOwnerKind::Project, owner, Some(Revision::INITIAL)).unwrap();
        let revision_two = StreamRef::new(
            StreamOwnerKind::Project,
            owner,
            Some(Revision::new(2).unwrap()),
        )
        .unwrap();

        store
            .append_event(&append(EVENT_A, "task.created", revision_one, None))
            .unwrap();
        let second = store
            .append_event(&append(
                EVENT_B,
                "task.updated",
                revision_two,
                Some(EVENT_A),
            ))
            .unwrap();

        assert_eq!(second.stream_seq(), 2);
        let stored_revision: i64 = store
            .connection
            .query_row(
                "SELECT owner_revision FROM events WHERE id = ?1",
                [EVENT_B],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(stored_revision, 2);
    }

    #[test]
    fn first_event_rejects_any_predecessor_including_cross_stream_event() {
        let db = TestDb::new("event-cross-stream");
        let mut store = Store::open(&db.path).unwrap();
        let stream_a = project_stream(PROJECT_A);
        let stream_b = project_stream(PROJECT_B);
        store
            .append_event(&append(EVENT_A, "task.created", stream_a, None))
            .unwrap();

        assert_eq!(
            store.append_event(&append(EVENT_B, "task.created", stream_b, Some(EVENT_A))),
            Err(StoreError::EventPredecessorMismatch)
        );
        let count_b: i64 = store
            .connection
            .query_row(
                "SELECT COUNT(*) FROM events WHERE owner_id = ?1",
                [PROJECT_B],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count_b, 0);
    }

    #[test]
    fn duplicate_event_id_is_rejected_without_new_stream_position() {
        let db = TestDb::new("event-duplicate");
        let mut store = Store::open(&db.path).unwrap();
        let stream = project_stream(PROJECT_A);
        store
            .append_event(&append(EVENT_A, "task.created", stream, None))
            .unwrap();

        assert_eq!(
            store.append_event(&append(EVENT_A, "task.updated", stream, Some(EVENT_A))),
            Err(StoreError::DuplicateConflict)
        );
        let count: i64 = store
            .connection
            .query_row("SELECT COUNT(*) FROM events", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn corrupted_gap_or_wrong_stored_predecessor_blocks_future_append() {
        let db = TestDb::new("event-corrupt-chain");
        let mut store = Store::open(&db.path).unwrap();
        let stream = project_stream(PROJECT_A);
        store
            .append_event(&append(EVENT_A, "task.created", stream, None))
            .unwrap();
        store
            .connection
            .execute(
                "INSERT INTO events(\
                    id, revision, event_type, owner_kind, owner_id, owner_revision, stream_seq, previous_event_id\
                 ) VALUES (?1, 1, 'task.updated', 'project', ?2, 1, ?3, ?4)",
                (
                    EVENT_B,
                    PROJECT_A,
                    3_u64.to_be_bytes().as_slice(),
                    EVENT_A,
                ),
            )
            .unwrap();

        assert_eq!(
            store.append_event(&append(EVENT_C, "task.updated", stream, Some(EVENT_B))),
            Err(StoreError::EventStreamCorrupt)
        );
        let count: i64 = store
            .connection
            .query_row("SELECT COUNT(*) FROM events", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 2);
    }

    #[test]
    fn stored_cross_stream_predecessor_blocks_future_append() {
        let db = TestDb::new("event-cross-stream-corrupt");
        let mut store = Store::open(&db.path).unwrap();
        let stream_a = project_stream(PROJECT_A);
        let stream_b = project_stream(PROJECT_B);
        store
            .append_event(&append(EVENT_A, "task.created", stream_a, None))
            .unwrap();
        store
            .append_event(&append(EVENT_B, "task.created", stream_b, None))
            .unwrap();

        store
            .connection
            .execute(
                "INSERT INTO events(                    id, revision, event_type, owner_kind, owner_id, owner_revision, stream_seq, previous_event_id                 ) VALUES (?1, 1, 'task.updated', 'project', ?2, 1, ?3, ?4)",
                (
                    EVENT_C,
                    PROJECT_A,
                    2_u64.to_be_bytes().as_slice(),
                    EVENT_B,
                ),
            )
            .unwrap();

        assert_eq!(
            store.append_event(&append(EVENT_D, "task.updated", stream_a, Some(EVENT_C))),
            Err(StoreError::EventStreamCorrupt)
        );
        let count_a: i64 = store
            .connection
            .query_row(
                "SELECT COUNT(*) FROM events WHERE owner_id = ?1",
                [PROJECT_A],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count_a, 2);
    }

    #[test]
    fn malformed_persisted_event_metadata_blocks_future_append() {
        let db = TestDb::new("event-malformed-existing");
        let mut store = Store::open(&db.path).unwrap();
        store
            .connection
            .execute(
                "INSERT INTO events(\
                    id, revision, event_type, owner_kind, owner_id, owner_revision, stream_seq, previous_event_id\
                 ) VALUES (?1, 1, 'Task.created', 'project', ?2, 1, ?3, NULL)",
                (EVENT_A, PROJECT_A, 1_u64.to_be_bytes().as_slice()),
            )
            .unwrap();

        assert_eq!(
            store.append_event(&append(
                EVENT_B,
                "task.updated",
                project_stream(PROJECT_A),
                Some(EVENT_A)
            )),
            Err(StoreError::EventStreamCorrupt)
        );
    }

    #[test]
    fn stream_sequence_never_uses_uuid_lexical_order_as_authority() {
        let db = TestDb::new("event-uuid-order");
        let mut store = Store::open(&db.path).unwrap();
        let stream = project_stream(PROJECT_A);
        store
            .append_event(&append(EVENT_D, "task.created", stream, None))
            .unwrap();
        let accepted = store
            .append_event(&append(EVENT_A, "task.updated", stream, Some(EVENT_D)))
            .unwrap();

        assert!(
            EVENT_A < EVENT_D,
            "fixture requires descending UUID text order"
        );
        assert_eq!(accepted.stream_seq(), 2);
        assert_eq!(accepted.previous_event_id(), Some(id(EVENT_D)));
    }
}
