//! The journal on disk: the one file of this crate that names its SQLite
//! binding.
//!
//! The format, the schema and the rules for start are owned by spec 0003,
//! "The journal". Spec 0004, "The journal on disk", fixes the checkpoint
//! limits, the busy handling, the reader, and the two writes of the running
//! daemon: the start sequence at a `Ready` start and a conflict latch.
//!
//! A [`Journal`] owns one connection, and every method that writes takes
//! `&mut self`, so one value has one writer. The daemon owns one value and
//! locks the state directory, so no second process writes either. The
//! caller reads the wall clock and passes its times, which are for display
//! only. No error carries text from SQLite.

use std::fmt;
use std::path::Path;

use pool_config::{
    AuthorityId, AuthorityRevision, ClusterId, NodeId, StartSequence, SystemIdentifier,
    TopologyEpoch, ValidatedConfig,
};
use rusqlite::{Connection, ErrorCode, OpenFlags, TransactionBehavior};

use crate::authority::{
    ApprovedTopology, Authority, AuthorityState, Hold, Quarantine, StoreProbe, StoredAuthority,
    classify_startup,
};
use crate::bootstrap::{BootstrapPlan, BootstrapRefused, plan_bootstrap};
use crate::observation::{MonotonicTime, ObservationBoard, TimelineId};

/// `PRAGMA application_id`: the bytes `POOL`.
const APPLICATION_ID: i64 = 1_347_374_924;
/// `PRAGMA user_version`: the schema version this build writes and reads.
const SCHEMA_VERSION: i64 = 1;

/// Schema version 1, as spec 0003 writes it.
const SCHEMA: &str = "
CREATE TABLE store (
  singleton           INTEGER PRIMARY KEY CHECK (singleton = 1),
  authority_id        BLOB    NOT NULL CHECK (length(authority_id) = 16),
  authority_revision  INTEGER NOT NULL CHECK (authority_revision >= 1),
  start_sequence      INTEGER NOT NULL CHECK (start_sequence >= 0),
  hold                TEXT    NOT NULL CHECK (hold IN ('none', 'restore_pending')),
  created_unix_ms     INTEGER NOT NULL CHECK (created_unix_ms >= 0)
) STRICT;

CREATE TABLE approved_topology (
  cluster_id          TEXT    PRIMARY KEY,
  system_identifier   TEXT    NOT NULL,
  approved_writer     TEXT    NOT NULL,
  writer_timeline     INTEGER NOT NULL CHECK (writer_timeline >= 1),
  topology_epoch      INTEGER NOT NULL CHECK (topology_epoch >= 1),
  conflict_latched    INTEGER NOT NULL CHECK (conflict_latched IN (0, 1)),
  changed_revision    INTEGER NOT NULL CHECK (changed_revision >= 1),
  changed_unix_ms     INTEGER NOT NULL CHECK (changed_unix_ms >= 0)
) STRICT, WITHOUT ROWID;

CREATE TABLE member (
  cluster_id          TEXT    NOT NULL REFERENCES approved_topology (cluster_id),
  node_id             TEXT    NOT NULL,
  changed_revision    INTEGER NOT NULL CHECK (changed_revision >= 1),
  PRIMARY KEY (cluster_id, node_id)
) STRICT, WITHOUT ROWID;

CREATE TABLE quarantine (
  cluster_id          TEXT    NOT NULL REFERENCES approved_topology (cluster_id),
  node_id             TEXT    NOT NULL,
  since_epoch         INTEGER NOT NULL CHECK (since_epoch >= 1),
  changed_revision    INTEGER NOT NULL CHECK (changed_revision >= 1),
  changed_unix_ms     INTEGER NOT NULL CHECK (changed_unix_ms >= 0),
  PRIMARY KEY (cluster_id, node_id)
) STRICT, WITHOUT ROWID;
";

/// Why a journal operation failed. It carries no text from SQLite, and its
/// `Debug` prints the variant only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JournalError {
    /// `journal.open`: the file cannot be opened as needed, or for
    /// [`Journal::create`] it is anything but an empty database.
    Open,
    /// `journal.busy`: SQLite answered busy. The state directory is locked,
    /// so nothing waits for it.
    Busy,
    /// `journal.changed`: the journal is not the one the state was
    /// classified from.
    Changed,
    /// `journal.write`: a write failed, or a counter is at its largest value.
    Write,
}

impl JournalError {
    /// The stable name of the error.
    pub fn as_str(self) -> &'static str {
        match self {
            JournalError::Open => "journal.open",
            JournalError::Busy => "journal.busy",
            JournalError::Changed => "journal.changed",
            JournalError::Write => "journal.write",
        }
    }
}

impl fmt::Display for JournalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::error::Error for JournalError {}

/// Maps a SQLite error to `Busy` when SQLite said busy or locked, and to
/// `otherwise` for anything else. The SQLite text is dropped.
fn map_error(error: &rusqlite::Error, otherwise: JournalError) -> JournalError {
    match error.sqlite_error_code() {
        Some(ErrorCode::DatabaseBusy | ErrorCode::DatabaseLocked) => JournalError::Busy,
        _ => otherwise,
    }
}

/// The proof that the journal holds the conflict latch of one cluster, at an
/// authority revision.
///
/// Only [`Journal::set_conflict_latch`] makes one. Code outside this crate
/// cannot build one:
///
/// ```compile_fail,E0451
/// fn build(
///     cluster: pool_config::ClusterId,
///     authority_revision: pool_config::AuthorityRevision,
/// ) -> pool_control::LatchReceipt {
///     pool_control::LatchReceipt { cluster, authority_revision }
/// }
/// ```
///
/// The same paths through the public functions compile, so the snippet
/// above can fail only because the fields are private:
///
/// ```
/// fn read(receipt: &pool_control::LatchReceipt) -> (pool_config::ClusterId, u64) {
///     (receipt.cluster().clone(), receipt.authority_revision().get())
/// }
/// ```
///
/// Restart rule: memory only. What survives is the latch in the journal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LatchReceipt {
    cluster: ClusterId,
    authority_revision: AuthorityRevision,
}

impl LatchReceipt {
    /// The cluster whose latch the journal holds.
    pub fn cluster(&self) -> &ClusterId {
        &self.cluster
    }

    /// The authority revision of the journal after the write.
    pub fn authority_revision(&self) -> AuthorityRevision {
        self.authority_revision
    }
}

/// What the journal reader found, kept opaque (`0003 AC-19`).
///
/// Only [`Journal::read`] makes one, and it shows nothing of what it holds.
/// Code outside this crate gets an [`AuthorityState`] from
/// [`JournalRead::classify`] and a [`BootstrapPlan`] from
/// [`JournalRead::bootstrap_plan`], and in no other way. So no other crate
/// can build a journal value or change one that was read. Code outside this
/// crate cannot build one:
///
/// ```compile_fail,E0451
/// fn build() -> pool_control::JournalRead {
///     pool_control::JournalRead { probe: todo!() }
/// }
/// ```
///
/// The same type through the public functions compiles:
///
/// ```
/// fn read(path: &std::path::Path) -> Option<pool_config::AuthorityId> {
///     pool_control::Journal::read(path).authority_id()
/// }
/// ```
///
/// Restart rule: memory only. Each start reads the journal again.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JournalRead {
    probe: StoreProbe,
}

impl JournalRead {
    /// Classifies the start from what was read and from the witness of
    /// `config`.
    pub fn classify(self, config: &ValidatedConfig) -> AuthorityState {
        classify_startup(config, self.probe)
    }

    /// Decides whether `pool bootstrap` may write a journal, from what was
    /// read, the samples of `board`, the expiry of `config.probe()` and the
    /// later of `now` and the latest sample time on the board.
    pub fn bootstrap_plan(
        &self,
        config: &ValidatedConfig,
        board: &ObservationBoard,
        now: MonotonicTime,
    ) -> Result<BootstrapPlan, BootstrapRefused> {
        plan_bootstrap(
            config,
            &self.probe,
            board.observations(),
            board.clamp(now),
            config.probe().observation_expiry(),
        )
    }

    /// The ID of the journal that was read, also when the witness does not
    /// match it. `None` when no journal could be read.
    pub fn authority_id(&self) -> Option<AuthorityId> {
        match &self.probe {
            StoreProbe::Present(stored) => Some(stored.authority_id),
            StoreProbe::Absent | StoreProbe::Unreadable => None,
        }
    }
}

/// The journal of one daemon, open for writing. It owns one connection.
///
/// Restart rule: the file survives a restart. This value lives for one run:
/// [`Journal::open_ready`] makes it at a `Ready` start.
pub struct Journal {
    connection: Connection,
    authority_id: AuthorityId,
    authority_revision: AuthorityRevision,
}

impl fmt::Debug for Journal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Journal")
            .field("authority_id", &self.authority_id)
            .field("authority_revision", &self.authority_revision)
            .finish()
    }
}

impl Journal {
    /// Reads the journal at `path` without writing to it: the journal reader
    /// of spec 0003.
    ///
    /// No file, and an empty database (a file of 0 bytes, or a database with
    /// no mark and no table), give `Absent`: that is what a bootstrap leaves
    /// when it stops between creating the file and writing it. A file that cannot be
    /// opened, a wrong application ID, another schema version, a failed
    /// `PRAGMA quick_check`, a missing `store` row, a cluster with no member
    /// row for its approved writer, or a stored value outside the range of
    /// its type give `Unreadable`. Anything else gives what was read.
    pub fn read(path: &Path) -> JournalRead {
        let probe = match path.try_exists() {
            Ok(false) => StoreProbe::Absent,
            Ok(true) => read_file(path),
            Err(_) => StoreProbe::Unreadable,
        };
        JournalRead { probe }
    }

    /// Writes a new journal into the empty file at `path`, in one
    /// transaction: the marks, the four tables, the `store` row with
    /// revision 1, start sequence 0 and hold `none`, and one approved
    /// topology with its members per cluster of `plan`, at epoch 1.
    ///
    /// The caller creates the empty file with mode `0600` first. It must be
    /// an empty database: a file of 0 bytes, or a database with no mark and
    /// no table, which is what a create that stopped after setting the WAL
    /// mode leaves. Any other file, also a missing one, fails with `Open`,
    /// and nothing is written. After the commit, the WAL is checkpointed
    /// into the main file; a checkpoint that fails does not fail the create.
    pub fn create(
        path: &Path,
        plan: &BootstrapPlan,
        authority_id: AuthorityId,
        created_unix_ms: u64,
    ) -> Result<(), JournalError> {
        let created = i64::try_from(created_unix_ms).map_err(|_| JournalError::Write)?;
        let mut connection = open_for_writing(path)?;
        if is_empty_database(&connection) != Ok(true) {
            return Err(JournalError::Open);
        }
        set_write_pragmas(&connection)?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|error| map_error(&error, JournalError::Write))?;
        let write = |error: rusqlite::Error| map_error(&error, JournalError::Write);
        transaction
            .pragma_update(None, "application_id", APPLICATION_ID)
            .map_err(write)?;
        transaction
            .pragma_update(None, "user_version", SCHEMA_VERSION)
            .map_err(write)?;
        transaction.execute_batch(SCHEMA).map_err(write)?;
        let revision = to_sql(AuthorityRevision::FIRST.get())?;
        transaction
            .execute(
                "INSERT INTO store (singleton, authority_id, authority_revision, start_sequence, hold, created_unix_ms) \
                 VALUES (1, ?1, ?2, ?3, 'none', ?4)",
                rusqlite::params![
                    authority_id.as_bytes().as_slice(),
                    revision,
                    to_sql(StartSequence::FIRST.get())?,
                    created
                ],
            )
            .map_err(write)?;
        for topology in plan.clusters() {
            transaction
                .execute(
                    "INSERT INTO approved_topology (cluster_id, system_identifier, approved_writer, writer_timeline, \
                     topology_epoch, conflict_latched, changed_revision, changed_unix_ms) \
                     VALUES (?1, ?2, ?3, ?4, ?5, 0, ?6, ?7)",
                    rusqlite::params![
                        topology.cluster.as_str(),
                        topology.system_identifier.to_string(),
                        topology.approved_writer.as_str(),
                        i64::from(topology.writer_timeline.get()),
                        to_sql(TopologyEpoch::FIRST.get())?,
                        revision,
                        created
                    ],
                )
                .map_err(write)?;
            for member in &topology.members {
                transaction
                    .execute(
                        "INSERT INTO member (cluster_id, node_id, changed_revision) VALUES (?1, ?2, ?3)",
                        rusqlite::params![topology.cluster.as_str(), member.as_str(), revision],
                    )
                    .map_err(write)?;
            }
        }
        transaction.commit().map_err(write)?;
        // Move the transaction into the main file, so that a copy of that
        // file alone is the journal. The transaction is durable in the WAL
        // already, so a failed checkpoint does not fail the create.
        let _ = connection.query_row("PRAGMA wal_checkpoint(TRUNCATE)", [], |_| Ok(()));
        Ok(())
    }

    /// Opens the journal of a `Ready` start for writing and advances its
    /// start sequence by one. Returns the journal and the new start sequence.
    ///
    /// In one transaction that starts with `BEGIN IMMEDIATE` it reads the
    /// authority ID, the revision and the start sequence again and fails with
    /// `Changed` when they are not the ones `authority` was classified from.
    /// It then applies the migrations (none exist for version 1) and writes
    /// the new start sequence. At the largest start sequence it fails with
    /// `Write`. When it fails, nothing may be admitted.
    pub fn open_ready(
        path: &Path,
        authority: &Authority,
    ) -> Result<(Journal, StartSequence), JournalError> {
        if !path.try_exists().map_err(|_| JournalError::Open)? {
            return Err(JournalError::Open);
        }
        let mut connection = open_for_writing(path)?;
        set_write_pragmas(&connection)?;
        let transaction = connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(|error| map_error(&error, JournalError::Write))?;
        let version: i64 = transaction
            .pragma_query_value(None, "user_version", |row| row.get(0))
            .map_err(|error| map_error(&error, JournalError::Changed))?;
        let store = read_store(&transaction).map_err(|()| JournalError::Changed)?;
        if version != SCHEMA_VERSION
            || store.authority_id != authority.authority_id()
            || store.authority_revision != authority.authority_revision()
            || store.start_sequence != authority.start_sequence()
        {
            return Err(JournalError::Changed);
        }
        let next = store
            .start_sequence
            .checked_next()
            .ok_or(JournalError::Write)?;
        transaction
            .execute(
                "UPDATE store SET start_sequence = ?1 WHERE singleton = 1",
                [to_sql(next.get())?],
            )
            .map_err(|error| map_error(&error, JournalError::Write))?;
        transaction
            .commit()
            .map_err(|error| map_error(&error, JournalError::Write))?;
        Ok((
            Journal {
                connection,
                authority_id: store.authority_id,
                authority_revision: store.authority_revision,
            },
            next,
        ))
    }

    /// Sets the conflict latch of `cluster` and advances the authority
    /// revision, in one transaction that starts with `BEGIN IMMEDIATE`.
    ///
    /// When the latch is already set it changes nothing and returns a receipt
    /// with the current revision. It fails with `Changed` when the journal no
    /// longer holds the revision this value last wrote or read, and with
    /// `Write` when the cluster has no row or the revision is at its largest
    /// value. When it fails, the latch still holds in the memory of the
    /// daemon, which tries again in a later round.
    pub fn set_conflict_latch(
        &mut self,
        cluster: &ClusterId,
        changed_unix_ms: u64,
    ) -> Result<LatchReceipt, JournalError> {
        let changed = i64::try_from(changed_unix_ms).map_err(|_| JournalError::Write)?;
        let write = |error: rusqlite::Error| map_error(&error, JournalError::Write);
        let transaction = self
            .connection
            .transaction_with_behavior(TransactionBehavior::Immediate)
            .map_err(write)?;
        let store = read_store(&transaction).map_err(|()| JournalError::Changed)?;
        if store.authority_id != self.authority_id
            || store.authority_revision != self.authority_revision
        {
            return Err(JournalError::Changed);
        }
        let latched: Option<i64> = transaction
            .query_row(
                "SELECT conflict_latched FROM approved_topology WHERE cluster_id = ?1",
                [cluster.as_str()],
                |row| row.get(0),
            )
            .map(Some)
            .or_else(|error| match error {
                rusqlite::Error::QueryReturnedNoRows => Ok(None),
                other => Err(other),
            })
            .map_err(write)?;
        match latched {
            None => return Err(JournalError::Write),
            Some(1) => {
                return Ok(LatchReceipt {
                    cluster: cluster.clone(),
                    authority_revision: store.authority_revision,
                });
            }
            Some(_) => {}
        }
        let next = store
            .authority_revision
            .checked_next()
            .ok_or(JournalError::Write)?;
        let next_sql = to_sql(next.get())?;
        transaction
            .execute(
                "UPDATE approved_topology SET conflict_latched = 1, changed_revision = ?1, changed_unix_ms = ?2 \
                 WHERE cluster_id = ?3",
                rusqlite::params![next_sql, changed, cluster.as_str()],
            )
            .map_err(write)?;
        transaction
            .execute(
                "UPDATE store SET authority_revision = ?1 WHERE singleton = 1",
                [next_sql],
            )
            .map_err(write)?;
        transaction.commit().map_err(write)?;
        self.authority_revision = next;
        Ok(LatchReceipt {
            cluster: cluster.clone(),
            authority_revision: next,
        })
    }
}

/// A number of a generation type as a SQLite integer. Every generation stops
/// at the largest SQLite integer, so this fails only on a defect.
fn to_sql(value: u64) -> Result<i64, JournalError> {
    i64::try_from(value).map_err(|_| JournalError::Write)
}

/// Opens an existing file for reading and writing. It never creates one.
fn open_for_writing(path: &Path) -> Result<Connection, JournalError> {
    let flags = OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX;
    let connection = Connection::open_with_flags(path, flags).map_err(|_| JournalError::Open)?;
    connection
        .busy_timeout(std::time::Duration::ZERO)
        .map_err(|_| JournalError::Open)?;
    Ok(connection)
}

/// The settings of every open that writes: WAL, full sync, foreign keys and
/// the checkpoint limits of spec 0004.
fn set_write_pragmas(connection: &Connection) -> Result<(), JournalError> {
    let mode: String = connection
        .query_row("PRAGMA journal_mode = WAL", [], |row| row.get(0))
        .map_err(|error| map_error(&error, JournalError::Open))?;
    if !mode.eq_ignore_ascii_case("wal") {
        return Err(JournalError::Open);
    }
    for (name, value) in [
        ("synchronous", "FULL"),
        ("foreign_keys", "ON"),
        ("wal_autocheckpoint", "1000"),
        ("journal_size_limit", "4194304"),
    ] {
        connection
            .pragma_update_and_check(None, name, value, |_| Ok(()))
            .or_else(|error| match error {
                rusqlite::Error::QueryReturnedNoRows => Ok(()),
                other => Err(other),
            })
            .map_err(|error| map_error(&error, JournalError::Open))?;
    }
    Ok(())
}

/// The `store` row: ID, revision, start sequence and hold.
struct StoreRow {
    authority_id: AuthorityId,
    authority_revision: AuthorityRevision,
    start_sequence: StartSequence,
    hold: Hold,
}

fn read_store(connection: &Connection) -> Result<StoreRow, ()> {
    let (id, revision, sequence, hold): (Vec<u8>, i64, i64, String) = connection
        .query_row(
            "SELECT authority_id, authority_revision, start_sequence, hold FROM store WHERE singleton = 1",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
        )
        .map_err(|_| ())?;
    let id: [u8; 16] = id.try_into().map_err(|_| ())?;
    Ok(StoreRow {
        authority_id: AuthorityId::from_bytes(id),
        authority_revision: from_sql(revision)
            .and_then(AuthorityRevision::from_stored)
            .ok_or(())?,
        start_sequence: from_sql(sequence)
            .and_then(StartSequence::from_stored)
            .ok_or(())?,
        hold: match hold.as_str() {
            "none" => Hold::None,
            "restore_pending" => Hold::RestorePending,
            _ => return Err(()),
        },
    })
}

fn from_sql(value: i64) -> Option<u64> {
    u64::try_from(value).ok()
}

/// The rule for an empty database (spec 0004, "The journal on disk"):
/// SQLite opens the file, `application_id` and `user_version` are 0, the
/// schema has no row, and `quick_check` says `ok`. A file of 0 bytes meets it,
/// and so does one page after `journal_mode = WAL`. A damaged journal never
/// does: SQLite refuses it, or its marks or its tables are there. `Err` when
/// a query failed.
fn is_empty_database(connection: &Connection) -> Result<bool, ()> {
    let pragma = |name: &str| -> Result<i64, ()> {
        connection
            .pragma_query_value(None, name, |row| row.get(0))
            .map_err(|_| ())
    };
    let application_id = pragma("application_id")?;
    let user_version = pragma("user_version")?;
    let schema_rows: i64 = connection
        .query_row("SELECT count(*) FROM sqlite_schema", [], |row| row.get(0))
        .map_err(|_| ())?;
    let check: String = connection
        .query_row("PRAGMA quick_check", [], |row| row.get(0))
        .map_err(|_| ())?;
    Ok(application_id == 0 && user_version == 0 && schema_rows == 0 && check == "ok")
}

/// The reader proper, for a file that exists.
fn read_file(path: &Path) -> StoreProbe {
    let flags = OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX;
    let Ok(connection) = Connection::open_with_flags(path, flags) else {
        return StoreProbe::Unreadable;
    };
    if connection.busy_timeout(std::time::Duration::ZERO).is_err() {
        return StoreProbe::Unreadable;
    }
    match is_empty_database(&connection) {
        Ok(true) => return StoreProbe::Absent,
        Ok(false) => {}
        Err(()) => return StoreProbe::Unreadable,
    }
    match read_stored(&connection) {
        Ok(stored) => StoreProbe::Present(stored),
        Err(()) => StoreProbe::Unreadable,
    }
}

fn read_stored(connection: &Connection) -> Result<StoredAuthority, ()> {
    let pragma = |name: &str| -> Result<i64, ()> {
        connection
            .pragma_query_value(None, name, |row| row.get(0))
            .map_err(|_| ())
    };
    if pragma("application_id")? != APPLICATION_ID || pragma("user_version")? != SCHEMA_VERSION {
        return Err(());
    }
    let check: String = connection
        .query_row("PRAGMA quick_check", [], |row| row.get(0))
        .map_err(|_| ())?;
    if check != "ok" {
        return Err(());
    }
    let store = read_store(connection)?;

    let mut clusters = Vec::new();
    let mut statement = connection
        .prepare(
            "SELECT cluster_id, system_identifier, approved_writer, writer_timeline, topology_epoch, conflict_latched \
             FROM approved_topology ORDER BY cluster_id",
        )
        .map_err(|_| ())?;
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, i64>(3)?,
                row.get::<_, i64>(4)?,
                row.get::<_, i64>(5)?,
            ))
        })
        .map_err(|_| ())?;
    for row in rows {
        let (cluster, system_identifier, writer, timeline, epoch, latched) = row.map_err(|_| ())?;
        clusters.push(ApprovedTopology {
            cluster: ClusterId::parse(&cluster).ok_or(())?,
            system_identifier: SystemIdentifier::parse(&system_identifier).ok_or(())?,
            approved_writer: NodeId::parse(&writer).ok_or(())?,
            writer_timeline: u32::try_from(timeline)
                .ok()
                .and_then(TimelineId::from_observed)
                .ok_or(())?,
            topology_epoch: from_sql(epoch)
                .and_then(TopologyEpoch::from_stored)
                .ok_or(())?,
            members: Vec::new(),
            conflict_latched: match latched {
                0 => false,
                1 => true,
                _ => return Err(()),
            },
        });
    }

    let mut statement = connection
        .prepare("SELECT cluster_id, node_id FROM member ORDER BY cluster_id, node_id")
        .map_err(|_| ())?;
    let rows = statement
        .query_map([], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(|_| ())?;
    for row in rows {
        let (cluster, node) = row.map_err(|_| ())?;
        let cluster = ClusterId::parse(&cluster).ok_or(())?;
        let node = NodeId::parse(&node).ok_or(())?;
        let topology = clusters
            .iter_mut()
            .find(|topology| topology.cluster == cluster)
            .ok_or(())?;
        topology.members.push(node);
    }
    // A cluster with no member row for its approved writer is not a journal
    // this build wrote.
    if clusters
        .iter()
        .any(|topology| !topology.members.contains(&topology.approved_writer))
    {
        return Err(());
    }

    let mut quarantine = Vec::new();
    let mut statement = connection
        .prepare(
            "SELECT cluster_id, node_id, since_epoch FROM quarantine ORDER BY cluster_id, node_id",
        )
        .map_err(|_| ())?;
    let rows = statement
        .query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i64>(2)?,
            ))
        })
        .map_err(|_| ())?;
    for row in rows {
        let (cluster, node, since) = row.map_err(|_| ())?;
        from_sql(since)
            .and_then(TopologyEpoch::from_stored)
            .ok_or(())?;
        quarantine.push(Quarantine {
            cluster: ClusterId::parse(&cluster).ok_or(())?,
            node: NodeId::parse(&node).ok_or(())?,
        });
    }

    Ok(StoredAuthority {
        authority_id: store.authority_id,
        authority_revision: store.authority_revision,
        start_sequence: store.start_sequence,
        hold: store.hold,
        clusters,
        quarantine,
    })
}

#[cfg(test)]
mod tests {
    /// The SQLite that is linked is the one inside the locked `libsqlite3-sys`, never one of the
    /// host (spec 0004, Native code, decision 87): `.cargo/config.toml` holds the switch.
    #[test]
    fn the_linked_sqlite_is_the_bundled_3_54_1() {
        assert_eq!(rusqlite::version(), "3.54.1");
    }
}
