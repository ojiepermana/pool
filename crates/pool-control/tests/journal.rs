//! The journal on disk (spec 0004, "The journal on disk"; `0004 AC-10`, the
//! library half): create, read, every `Unreadable` case, `open_ready` and the
//! conflict latch, on real files under `CARGO_TARGET_TMPDIR`. A second
//! connection opened here stands in for damage and for a writer that holds
//! the lock.

use std::path::{Path, PathBuf};
use std::time::Duration;

use pool_config::{AuthorityId, ClusterId, ValidatedConfig};
use pool_control::{
    AuthorityState, BootstrapPlan, Journal, JournalError, MonotonicTime, NodeFacts, Observation,
    ObservationBoard, ObservationOutcome, TimelineId,
};
use rusqlite::Connection;

const SYSTEM_ID: &str = "7412345678901234567";
const X: &str = "0123456789abcdef0123456789abcdef";
const Y: &str = "fedcba9876543210fedcba9876543210";

/// A fresh, empty folder of this test's own.
fn folder(name: &str) -> PathBuf {
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("pool-control-{name}"));
    #[expect(clippy::disallowed_methods, reason = "W3 permits fixture path checks")]
    let exists = path.exists();
    if exists {
        #[expect(clippy::disallowed_methods, reason = "W3 permits test file I/O")]
        std::fs::remove_dir_all(&path).expect("remove old test folder");
    }
    #[expect(clippy::disallowed_methods, reason = "W3 permits test file I/O")]
    std::fs::create_dir_all(&path).expect("create test folder");
    path
}

/// The empty file the binary creates before `Journal::create`.
fn empty_journal(folder: &Path) -> PathBuf {
    let path = folder.join("journal.sqlite3");
    #[expect(clippy::disallowed_types, reason = "W3 permits test file I/O")]
    std::fs::File::create_new(&path).expect("create the empty journal file");
    path
}

/// Two clusters: `main` with `pg1` to `pg3` (writer `pg1`) and `side` with
/// `sd1` and `sd2` (writer `sd1`).
fn config(witness: Option<&str>) -> ValidatedConfig {
    let mut text =
        String::from("config_version = 1\n\n[control]\nstate_directory = \"/var/lib/pool\"\n");
    if let Some(witness) = witness {
        text.push_str(&format!("authority_id = \"{witness}\"\n"));
    }
    for (cluster, writer) in [("main", "pg1"), ("side", "sd1")] {
        text.push_str(&format!(
            "\n[[cluster]]\nid = \"{cluster}\"\nsystem_identifier = \"{SYSTEM_ID}\"\nexpected_writer = \"{writer}\"\n"
        ));
    }
    for (index, (node, cluster)) in [
        ("pg1", "main"),
        ("pg2", "main"),
        ("pg3", "main"),
        ("sd1", "side"),
        ("sd2", "side"),
    ]
    .iter()
    .enumerate()
    {
        text.push_str(&format!(
            "\n[[node]]\nid = \"{node}\"\ncluster = \"{cluster}\"\nhost = \"10.0.0.{}\"\nport = 5432\n",
            index + 11
        ));
    }
    pool_config::validate(text.as_bytes()).expect("the test configuration is valid")
}

fn at(seconds: u64) -> MonotonicTime {
    MonotonicTime::from_origin(Duration::from_secs(seconds))
}

fn sample(node: &str, in_recovery: bool) -> Observation {
    Observation::new(
        pool_config::NodeId::parse(node).expect("node id"),
        at(98),
        ObservationOutcome::Reached(NodeFacts {
            system_identifier: pool_config::SystemIdentifier::parse(SYSTEM_ID)
                .expect("system identifier"),
            timeline: TimelineId::from_observed(1).expect("timeline"),
            in_recovery,
        }),
    )
}

fn healthy_board() -> ObservationBoard {
    let mut board = ObservationBoard::new();
    for (node, in_recovery) in [
        ("pg1", false),
        ("pg2", true),
        ("pg3", true),
        ("sd1", false),
        ("sd2", true),
    ] {
        board.record(sample(node, in_recovery));
    }
    board
}

/// The plan that the bootstrap command would write, through the real reader.
fn plan(path: &Path) -> BootstrapPlan {
    Journal::read(path)
        .bootstrap_plan(&config(None), &healthy_board(), at(100))
        .expect("bootstrap is allowed")
}

fn id(text: &str) -> AuthorityId {
    AuthorityId::parse(text).expect("authority id")
}

/// A journal written by `Journal::create`, with authority ID `X`.
fn created(name: &str) -> (PathBuf, PathBuf) {
    let folder = folder(name);
    let path = empty_journal(&folder);
    let plan = plan(&path);
    Journal::create(&path, &plan, id(X), 1_763_200_000_000).expect("create the journal");
    (folder, path)
}

fn cluster(name: &str) -> ClusterId {
    ClusterId::parse(name).expect("cluster id")
}

fn ready(path: &Path) -> AuthorityState {
    let state = Journal::read(path).classify(&config(Some(X)));
    assert_eq!(state.name(), "ready");
    state
}

fn authority(state: &AuthorityState) -> &pool_control::Authority {
    match state {
        AuthorityState::Ready(authority) => authority,
        other => panic!("expected ready, got {}", other.name()),
    }
}

fn sql(path: &Path) -> Connection {
    Connection::open(path).expect("open the journal for the test")
}

fn integer(path: &Path, query: &str) -> i64 {
    sql(path)
        .query_row(query, [], |row| row.get(0))
        .expect("query")
}

/// Changes the journal with SQL of the test, as damage would.
fn damage(path: &Path, statements: &str) {
    sql(path)
        .execute_batch(statements)
        .expect("damage the journal");
}

#[test]
fn ac10_create_writes_the_marks_the_four_tables_and_the_first_rows() {
    // covers: 0004 AC-10, 0003 AC-7.
    let (_folder, path) = created("create");
    assert_eq!(integer(&path, "PRAGMA application_id"), 1_347_374_924);
    assert_eq!(integer(&path, "PRAGMA user_version"), 1);
    let mode: String = sql(&path)
        .query_row("PRAGMA journal_mode", [], |row| row.get(0))
        .expect("journal mode");
    assert_eq!(mode, "wal");
    let connection = sql(&path);
    let mut statement = connection
        .prepare("SELECT name FROM sqlite_master WHERE type = 'table' ORDER BY name")
        .expect("tables");
    let tables: Vec<String> = statement
        .query_map([], |row| row.get(0))
        .expect("tables")
        .collect::<Result<_, _>>()
        .expect("tables");
    assert_eq!(
        tables,
        ["approved_topology", "member", "quarantine", "store"]
    );
    let store: (Vec<u8>, i64, i64, String, i64) = connection
        .query_row(
            "SELECT authority_id, authority_revision, start_sequence, hold, created_unix_ms FROM store",
            [],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?)),
        )
        .expect("store row");
    assert_eq!(store.0, id(X).as_bytes().to_vec());
    assert_eq!(
        (store.1, store.2, store.3.as_str(), store.4),
        (1, 0, "none", 1_763_200_000_000)
    );
    let mut statement = connection
        .prepare(
            "SELECT cluster_id, system_identifier, approved_writer, writer_timeline, topology_epoch, \
             conflict_latched, changed_revision FROM approved_topology ORDER BY cluster_id",
        )
        .expect("topology");
    let topology: Vec<(String, String, String, i64, i64, i64, i64)> = statement
        .query_map([], |row| {
            Ok((
                row.get(0)?,
                row.get(1)?,
                row.get(2)?,
                row.get(3)?,
                row.get(4)?,
                row.get(5)?,
                row.get(6)?,
            ))
        })
        .expect("topology")
        .collect::<Result<_, _>>()
        .expect("topology");
    assert_eq!(
        topology,
        [
            ("main".into(), SYSTEM_ID.into(), "pg1".into(), 1, 1, 0, 1),
            ("side".into(), SYSTEM_ID.into(), "sd1".into(), 1, 1, 0, 1),
        ]
    );
    assert_eq!(integer(&path, "SELECT count(*) FROM member"), 5);
    assert_eq!(integer(&path, "SELECT count(*) FROM quarantine"), 0);
}

#[test]
fn ac10_a_created_journal_reads_back_as_ready_with_its_witness() {
    // covers: 0004 AC-10, 0003 AC-7, AC-8.
    let (_folder, path) = created("read-back");
    let read = Journal::read(&path);
    assert_eq!(read.authority_id(), Some(id(X)));
    assert_eq!(
        Journal::read(&path).classify(&config(None)).name(),
        "unconfirmed"
    );
    assert_eq!(
        Journal::read(&path).classify(&config(Some(Y))).name(),
        "mismatch"
    );
    let state = ready(&path);
    let authority = authority(&state);
    assert_eq!(authority.authority_id(), id(X));
    assert_eq!(authority.authority_revision().get(), 1);
    assert_eq!(authority.start_sequence().get(), 0);
    // The reader refuses a second bootstrap.
    let refused = read
        .bootstrap_plan(&config(None), &healthy_board(), at(100))
        .expect_err("a journal is present");
    let names: Vec<&str> = refused.reasons().iter().map(|r| r.as_str()).collect();
    assert_eq!(names, ["bootstrap.store_present"]);
}

#[test]
fn ac10_no_file_and_an_empty_file_are_absent() {
    // covers: 0004 AC-10.
    let folder = folder("absent");
    let path = folder.join("journal.sqlite3");
    let read = Journal::read(&path);
    assert_eq!(read.authority_id(), None);
    assert_eq!(read.classify(&config(None)).name(), "uninitialized");
    assert_eq!(
        Journal::read(&path).classify(&config(Some(X))).name(),
        "missing"
    );
    #[expect(clippy::disallowed_methods, reason = "W3 permits fixture path checks")]
    let exists = path.exists();
    assert!(!exists, "reading created no file");

    let path = empty_journal(&folder);
    assert_eq!(Journal::read(&path).authority_id(), None);
    assert_eq!(
        Journal::read(&path).classify(&config(None)).name(),
        "uninitialized"
    );
    #[expect(clippy::disallowed_methods, reason = "W3 permits test file I/O")]
    let length = std::fs::metadata(&path).expect("file").len();
    assert_eq!(length, 0);
    assert!(
        Journal::read(&path)
            .bootstrap_plan(&config(None), &healthy_board(), at(100))
            .is_ok(),
        "an empty file does not block the next bootstrap"
    );
}

#[test]
fn ac10_every_case_of_spec_0003_is_unreadable() {
    // covers: 0004 AC-10, 0003 AC-8. Each case starts from a good journal.
    let cases: [(&str, &str); 9] = [
        ("application-id", "PRAGMA application_id = 1;"),
        ("newer-version", "PRAGMA user_version = 2;"),
        ("older-version", "PRAGMA user_version = 0;"),
        ("no-store-row", "DELETE FROM store;"),
        (
            "writer-without-member",
            "DELETE FROM member WHERE cluster_id = 'main' AND node_id = 'pg1';",
        ),
        (
            "timeline-out-of-range",
            "UPDATE approved_topology SET writer_timeline = 4294967296 WHERE cluster_id = 'main';",
        ),
        (
            "cluster-id-syntax",
            "PRAGMA foreign_keys = OFF; UPDATE member SET cluster_id = 'Main' WHERE cluster_id = 'main'; \
             UPDATE approved_topology SET cluster_id = 'Main' WHERE cluster_id = 'main';",
        ),
        (
            "authority-id-length",
            "PRAGMA ignore_check_constraints = ON; UPDATE store SET authority_id = x'0102';",
        ),
        ("missing-table", "DROP TABLE quarantine;"),
    ];
    for (name, statements) in cases {
        let (_folder, path) = created(&format!("unreadable-{name}"));
        damage(&path, statements);
        let read = Journal::read(&path);
        assert_eq!(read.authority_id(), None, "{name}");
        assert_eq!(
            read.classify(&config(Some(X))).name(),
            "unreadable",
            "{name}"
        );
    }
}

#[test]
fn ac10_a_damaged_file_and_a_file_of_another_kind_are_unreadable() {
    // covers: 0004 AC-10, 0003 AC-8. `quick_check` and the open itself.
    let folder = folder("not-a-journal");
    let path = folder.join("journal.sqlite3");
    #[expect(clippy::disallowed_methods, reason = "W3 permits test file I/O")]
    std::fs::write(
        &path,
        b"this is not a database, it only has a name like one",
    )
    .expect("write");
    assert_eq!(
        Journal::read(&path).classify(&config(Some(X))).name(),
        "unreadable"
    );

    let (_folder, path) = created("damaged-pages");
    // Every connection is closed, so the WAL is checkpointed into the file.
    #[expect(clippy::disallowed_methods, reason = "W3 permits test file I/O")]
    let mut bytes = std::fs::read(&path).expect("read the journal");
    assert!(bytes.len() > 4096 * 2, "{} bytes", bytes.len());
    for byte in &mut bytes[4096 + 8..4096 * 2] {
        *byte = 0xa5;
    }
    #[expect(clippy::disallowed_methods, reason = "W3 permits test file I/O")]
    std::fs::write(&path, &bytes).expect("damage the pages");
    assert_eq!(
        Journal::read(&path).classify(&config(Some(X))).name(),
        "unreadable"
    );

    let folder = folder.join("a-folder");
    #[expect(clippy::disallowed_methods, reason = "W3 permits test file I/O")]
    std::fs::create_dir_all(&folder).expect("folder");
    assert_eq!(
        Journal::read(&folder).classify(&config(Some(X))).name(),
        "unreadable"
    );
}

#[test]
fn ac10_reading_never_changes_the_journal() {
    // covers: 0004 AC-10. SQLite may create its side files when it reads.
    let (_folder, path) = created("read-only");
    #[expect(clippy::disallowed_methods, reason = "W3 permits test file I/O")]
    let before = std::fs::read(&path).expect("read");
    for witness in [None, Some(X), Some(Y)] {
        let _ = Journal::read(&path).classify(&config(witness));
    }
    let _ = Journal::read(&path).bootstrap_plan(&config(None), &healthy_board(), at(100));
    #[expect(clippy::disallowed_methods, reason = "W3 permits test file I/O")]
    let after = std::fs::read(&path).expect("read");
    assert_eq!(after, before);
    assert_eq!(integer(&path, "SELECT start_sequence FROM store"), 0);
}

#[test]
fn ac10_create_needs_an_empty_file() {
    // covers: 0004 AC-10. Anything but an empty database is `Open`.
    let folder = folder("create-needs-empty");
    let missing = folder.join("journal.sqlite3");
    let plan = Journal::read(&missing)
        .bootstrap_plan(&config(None), &healthy_board(), at(100))
        .expect("plan");
    assert_eq!(
        Journal::create(&missing, &plan, id(X), 0),
        Err(JournalError::Open)
    );
    #[expect(clippy::disallowed_methods, reason = "W3 permits fixture path checks")]
    let exists = missing.exists();
    assert!(!exists, "create never makes the file itself");

    let (_folder, path) = created("create-twice");
    #[expect(clippy::disallowed_methods, reason = "W3 permits test file I/O")]
    let before = std::fs::read(&path).expect("read");
    assert_eq!(
        Journal::create(&path, &plan, id(Y), 0),
        Err(JournalError::Open)
    );
    #[expect(clippy::disallowed_methods, reason = "W3 permits test file I/O")]
    let after = std::fs::read(&path).expect("read");
    assert_eq!(after, before);
    assert_eq!(Journal::read(&path).authority_id(), Some(id(X)));

    let other = folder.join("text");
    #[expect(clippy::disallowed_methods, reason = "W3 permits test file I/O")]
    std::fs::write(&other, b"not empty").expect("write");
    assert_eq!(
        Journal::create(&other, &plan, id(X), 0),
        Err(JournalError::Open)
    );
}

#[test]
fn ac10_open_ready_advances_the_start_sequence_once_per_start() {
    // covers: 0004 AC-10, 0003 AC-7.
    let (_folder, path) = created("open-ready");
    for expected in 1..=3 {
        let state = ready(&path);
        let (journal, sequence) =
            Journal::open_ready(&path, authority(&state)).expect("open the ready journal");
        assert_eq!(sequence.get(), expected);
        drop(journal);
        assert_eq!(
            integer(&path, "SELECT start_sequence FROM store"),
            i64::try_from(expected).expect("small")
        );
    }
    assert_eq!(integer(&path, "SELECT authority_revision FROM store"), 1);
}

#[test]
fn ac10_open_ready_refuses_a_journal_that_changed_since_it_was_read() {
    // covers: 0004 AC-10. The start sequence, the revision or the ID moved.
    let (_folder, path) = created("open-ready-changed");
    let stale = ready(&path);
    let (journal, _) = Journal::open_ready(&path, authority(&ready(&path))).expect("first start");
    drop(journal);
    assert_eq!(
        Journal::open_ready(&path, authority(&stale)).map(|(_, sequence)| sequence),
        Err(JournalError::Changed)
    );
    assert_eq!(integer(&path, "SELECT start_sequence FROM store"), 1);

    let fresh = ready(&path);
    damage(&path, "UPDATE store SET authority_revision = 5;");
    assert_eq!(
        Journal::open_ready(&path, authority(&fresh)).map(|(_, sequence)| sequence),
        Err(JournalError::Changed)
    );
    damage(&path, "UPDATE store SET authority_revision = 1;");
    assert!(Journal::open_ready(&path, authority(&fresh)).is_ok());

    let missing = path.with_file_name("gone.sqlite3");
    assert_eq!(
        Journal::open_ready(&missing, authority(&ready(&path))).map(|(_, sequence)| sequence),
        Err(JournalError::Open)
    );
    #[expect(clippy::disallowed_methods, reason = "W3 permits fixture path checks")]
    let exists = missing.exists();
    assert!(!exists);
}

#[test]
fn ac10_open_ready_stops_at_the_largest_start_sequence() {
    // covers: 0004 AC-10, 0003 AC-6. It never wraps.
    let (_folder, path) = created("open-ready-top");
    damage(
        &path,
        "UPDATE store SET start_sequence = 9223372036854775807;",
    );
    let state = ready(&path);
    assert_eq!(
        authority(&state).start_sequence().get(),
        9_223_372_036_854_775_807
    );
    assert_eq!(
        Journal::open_ready(&path, authority(&state)).map(|(_, sequence)| sequence),
        Err(JournalError::Write)
    );
    assert_eq!(
        integer(&path, "SELECT start_sequence FROM store"),
        9_223_372_036_854_775_807
    );
}

#[test]
fn ac10_a_writer_that_holds_the_lock_makes_the_journal_busy() {
    // covers: 0004 AC-10. `busy_timeout` is 0: nothing waits.
    let (_folder, path) = created("busy");
    let state = ready(&path);
    let holder = sql(&path);
    holder
        .execute_batch("BEGIN IMMEDIATE;")
        .expect("hold the lock");
    #[expect(clippy::disallowed_methods, reason = "W3 permits test clocks")]
    let started = std::time::Instant::now();
    assert_eq!(
        Journal::open_ready(&path, authority(&state)).map(|(_, sequence)| sequence),
        Err(JournalError::Busy)
    );
    assert!(started.elapsed() < Duration::from_secs(1));
    holder.execute_batch("ROLLBACK;").expect("release");
    assert!(Journal::open_ready(&path, authority(&state)).is_ok());
}

#[test]
fn ac10_the_conflict_latch_is_written_once_and_advances_the_revision() {
    // covers: 0004 AC-10, 0003 AC-17 (the journal half).
    let (_folder, path) = created("latch");
    let state = ready(&path);
    let (mut journal, _) = Journal::open_ready(&path, authority(&state)).expect("open");
    let receipt = journal
        .set_conflict_latch(&cluster("main"), 1_763_200_000_500)
        .expect("set the latch");
    assert_eq!(receipt.cluster(), &cluster("main"));
    assert_eq!(receipt.authority_revision().get(), 2);
    assert_eq!(integer(&path, "SELECT authority_revision FROM store"), 2);
    assert_eq!(
        integer(
            &path,
            "SELECT conflict_latched FROM approved_topology WHERE cluster_id = 'main'"
        ),
        1
    );
    assert_eq!(
        integer(
            &path,
            "SELECT changed_revision FROM approved_topology WHERE cluster_id = 'main'"
        ),
        2
    );
    assert_eq!(
        integer(
            &path,
            "SELECT conflict_latched FROM approved_topology WHERE cluster_id = 'side'"
        ),
        0
    );

    // Again: nothing changes, and the receipt has the current revision.
    let again = journal
        .set_conflict_latch(&cluster("main"), 1_763_200_000_900)
        .expect("already set");
    assert_eq!(again, receipt);
    assert_eq!(integer(&path, "SELECT authority_revision FROM store"), 2);

    // The other cluster advances the revision once more.
    let side = journal
        .set_conflict_latch(&cluster("side"), 1_763_200_001_000)
        .expect("set the second latch");
    assert_eq!(side.authority_revision().get(), 3);

    // A cluster the journal does not hold is a failed write.
    assert_eq!(
        journal.set_conflict_latch(&cluster("other"), 0),
        Err(JournalError::Write)
    );
    assert_eq!(integer(&path, "SELECT authority_revision FROM store"), 3);
    drop(journal);

    // The next start reads the latch back.
    match ready(&path) {
        AuthorityState::Ready(authority) => assert_eq!(authority.authority_revision().get(), 3),
        other => panic!("{}", other.name()),
    }
}

#[test]
fn ac10_a_latch_write_fails_while_another_writer_holds_the_lock_and_succeeds_later() {
    // covers: 0004 AC-10, 0003 AC-17.
    let (_folder, path) = created("latch-busy");
    let state = ready(&path);
    let (mut journal, _) = Journal::open_ready(&path, authority(&state)).expect("open");
    let holder = sql(&path);
    holder
        .execute_batch("BEGIN IMMEDIATE;")
        .expect("hold the lock");
    assert_eq!(
        journal.set_conflict_latch(&cluster("main"), 0),
        Err(JournalError::Busy)
    );
    holder.execute_batch("ROLLBACK;").expect("release");
    assert_eq!(integer(&path, "SELECT authority_revision FROM store"), 1);
    let receipt = journal
        .set_conflict_latch(&cluster("main"), 0)
        .expect("the retry");
    assert_eq!(receipt.authority_revision().get(), 2);
}

#[test]
fn ac10_a_latch_write_refuses_a_journal_that_changed_and_stops_at_the_top() {
    // covers: 0004 AC-10.
    let (_folder, path) = created("latch-changed");
    let state = ready(&path);
    let (mut journal, _) = Journal::open_ready(&path, authority(&state)).expect("open");
    damage(&path, "UPDATE store SET authority_revision = 7;");
    assert_eq!(
        journal.set_conflict_latch(&cluster("main"), 0),
        Err(JournalError::Changed)
    );
    drop(journal);

    let (_folder, path) = created("latch-top");
    damage(
        &path,
        "UPDATE store SET authority_revision = 9223372036854775807;",
    );
    let state = ready(&path);
    let (mut journal, _) = Journal::open_ready(&path, authority(&state)).expect("open");
    assert_eq!(
        journal.set_conflict_latch(&cluster("main"), 0),
        Err(JournalError::Write)
    );
    assert_eq!(
        integer(
            &path,
            "SELECT conflict_latched FROM approved_topology WHERE cluster_id = 'main'"
        ),
        0
    );
}

#[test]
fn journal_errors_have_stable_names_and_no_text() {
    // covers: 0004 AC-10.
    for (error, name) in [
        (JournalError::Open, "journal.open"),
        (JournalError::Busy, "journal.busy"),
        (JournalError::Changed, "journal.changed"),
        (JournalError::Write, "journal.write"),
    ] {
        assert_eq!(error.as_str(), name);
        assert_eq!(error.to_string(), name);
    }
    assert_eq!(format!("{:?}", JournalError::Busy), "Busy");
    let (_folder, path) = created("debug");
    let state = ready(&path);
    let (journal, _) = Journal::open_ready(&path, authority(&state)).expect("open");
    let debug = format!("{journal:?}");
    assert!(
        debug.starts_with("Journal { authority_id: AuthorityId("),
        "{debug}"
    );
    assert!(!debug.contains("journal.sqlite3"), "{debug}");
}

// The rule for an empty database (spec 0004, decision 59): a bootstrap that
// stopped after setting the WAL mode leaves one page with no mark and no
// table. That counts as absent, and no damaged journal ever does.

#[test]
fn ac10_a_file_left_after_only_the_wal_mode_is_absent_and_can_be_created() {
    // covers: 0004 AC-10.
    let folder = folder("only-wal-mode");
    let path = empty_journal(&folder);
    let mode: String = sql(&path)
        .query_row("PRAGMA journal_mode = WAL", [], |row| row.get(0))
        .expect("WAL mode");
    assert_eq!(mode, "wal");
    #[expect(clippy::disallowed_methods, reason = "W3 permits test file I/O")]
    let length = std::fs::metadata(&path).expect("file").len();
    assert!(length > 0, "one page now");
    let read = Journal::read(&path);
    assert_eq!(read.authority_id(), None);
    assert_eq!(read.classify(&config(None)).name(), "uninitialized");
    let plan = plan(&path);
    Journal::create(&path, &plan, id(X), 0).expect("create over the stopped bootstrap");
    assert_eq!(ready(&path).name(), "ready");
}

#[test]
fn ac10_a_journal_cut_to_one_or_two_pages_is_unreadable() {
    // covers: 0004 AC-10.
    for pages in [1_u64, 2] {
        let (_folder, path) = created(&format!("cut-{pages}"));
        #[expect(clippy::disallowed_types, reason = "W3 permits test file I/O")]
        let file = std::fs::OpenOptions::new()
            .write(true)
            .open(&path)
            .expect("open");
        file.set_len(4096 * pages).expect("cut");
        drop(file);
        assert_eq!(
            Journal::read(&path).classify(&config(Some(X))).name(),
            "unreadable",
            "{pages} pages"
        );
    }
}

/// The main file of a created journal with `range` set to zero.
fn zeroed(name: &str, range: std::ops::Range<usize>) -> PathBuf {
    let (folder, path) = created(name);
    #[expect(clippy::disallowed_methods, reason = "W3 permits test file I/O")]
    let mut bytes = std::fs::read(&path).expect("read");
    for byte in &mut bytes[range] {
        *byte = 0;
    }
    let copy = folder.join("zeroed.sqlite3");
    #[expect(clippy::disallowed_methods, reason = "W3 permits test file I/O")]
    std::fs::write(&copy, &bytes).expect("write");
    copy
}

#[test]
fn ac10_a_journal_without_its_marks_or_its_header_is_unreadable() {
    // covers: 0004 AC-10. The marks gone with the tables in place, and the
    // whole header gone.
    let mut both = zeroed("no-marks", 60..64);
    #[expect(clippy::disallowed_methods, reason = "W3 permits test file I/O")]
    let mut bytes = std::fs::read(&both).expect("read");
    for byte in &mut bytes[68..72] {
        *byte = 0;
    }
    #[expect(clippy::disallowed_methods, reason = "W3 permits test file I/O")]
    std::fs::write(&both, &bytes).expect("write");
    assert_eq!(
        Journal::read(&both).classify(&config(Some(X))).name(),
        "unreadable"
    );
    both = zeroed("no-header", 0..100);
    assert_eq!(
        Journal::read(&both).classify(&config(Some(X))).name(),
        "unreadable"
    );
}

#[test]
fn ac10_after_create_the_main_file_alone_is_the_journal() {
    // covers: 0004 AC-10. The checkpoint after the commit moved everything
    // into the main file, so a copy without the side files is complete.
    let (folder, path) = created("main-file-alone");
    let copy_folder = folder.join("copy");
    #[expect(clippy::disallowed_methods, reason = "W3 permits test file I/O")]
    std::fs::create_dir_all(&copy_folder).expect("folder");
    let copy = copy_folder.join("journal.sqlite3");
    #[expect(clippy::disallowed_methods, reason = "W3 permits test file I/O")]
    std::fs::copy(&path, &copy).expect("copy the main file only");
    #[expect(clippy::disallowed_methods, reason = "W3 permits fixture path checks")]
    let wal_exists = copy_folder.join("journal.sqlite3-wal").exists();
    assert!(!wal_exists);
    let read = Journal::read(&copy);
    assert_eq!(read.authority_id(), Some(id(X)));
    assert_eq!(read.classify(&config(None)).name(), "unconfirmed");
    assert_eq!(ready(&copy).name(), "ready");
}

// ---- added by the tests of scope 004 (`/test one real primary path`) ----

#[test]
fn ac10_a_header_with_a_wrong_free_page_count_is_unreadable() {
    // covers: 0004 AC-10. Only `PRAGMA quick_check` sees this damage: the
    // marks, the tables and the rows are all still in place.
    let (folder, path) = created("free-page-count");
    #[expect(clippy::disallowed_methods, reason = "W3 permits test file I/O")]
    let mut bytes = std::fs::read(&path).expect("read");
    bytes[36..40].copy_from_slice(&7u32.to_be_bytes());
    let copy = folder.join("free-pages.sqlite3");
    #[expect(clippy::disallowed_methods, reason = "W3 permits test file I/O")]
    std::fs::write(&copy, &bytes).expect("write");
    let check: String = sql(&copy)
        .query_row("PRAGMA quick_check", [], |row| row.get(0))
        .expect("quick_check runs");
    assert_ne!(check, "ok", "the damage is one that quick_check reports");
    assert_eq!(
        integer(&copy, "SELECT count(*) FROM store"),
        1,
        "the rows can still be read"
    );
    assert_eq!(
        Journal::read(&copy).classify(&config(Some(X))).name(),
        "unreadable"
    );
}

#[test]
fn ac10_open_ready_does_not_wait_for_a_writer_that_holds_the_lock() {
    // covers: 0004 AC-10. `busy_timeout` is 0 on the open that writes: the
    // answer comes before any wait SQLite could be told to make.
    let (_folder, path) = created("busy-no-wait");
    let state = ready(&path);
    let holder = sql(&path);
    holder
        .execute_batch("BEGIN IMMEDIATE;")
        .expect("hold the lock");
    #[expect(clippy::disallowed_methods, reason = "W3 permits test clocks")]
    let started = std::time::Instant::now();
    assert_eq!(
        Journal::open_ready(&path, authority(&state)).map(|(_, sequence)| sequence),
        Err(JournalError::Busy)
    );
    assert!(
        started.elapsed() < Duration::from_millis(200),
        "{:?}",
        started.elapsed()
    );
    holder.execute_batch("ROLLBACK;").expect("release");
}

#[test]
fn ac10_the_reader_does_not_wait_for_a_connection_that_holds_the_file_exclusively() {
    // covers: 0004 AC-10. The reader has `busy_timeout` 0 too.
    let (_folder, path) = created("reader-no-wait");
    let holder = sql(&path);
    holder
        .execute_batch(
            "PRAGMA locking_mode = EXCLUSIVE; BEGIN EXCLUSIVE; UPDATE store SET hold = hold;",
        )
        .expect("hold the file");
    #[expect(clippy::disallowed_methods, reason = "W3 permits test clocks")]
    let started = std::time::Instant::now();
    let _ = Journal::read(&path);
    assert!(
        started.elapsed() < Duration::from_millis(200),
        "{:?}",
        started.elapsed()
    );
    holder.execute_batch("ROLLBACK;").expect("release");
}

#[test]
fn ac10_a_journal_file_that_cannot_be_written_is_still_read() {
    // covers: 0004 AC-10. Reading needs no write access to the main file.
    let (_folder, path) = created("reader-opens-read-only");
    use std::os::unix::fs::PermissionsExt;
    #[expect(clippy::disallowed_methods, reason = "W3 permits test file I/O")]
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o400)).expect("chmod");
    if nix_root() {
        return;
    }
    let read = Journal::read(&path);
    assert_eq!(read.authority_id(), Some(id(X)));
    assert_eq!(read.classify(&config(Some(X))).name(), "ready");
    #[expect(clippy::disallowed_methods, reason = "W3 permits test file I/O")]
    std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).expect("chmod");
}

/// Whether the test runs as root, which may write any file.
fn nix_root() -> bool {
    use std::os::unix::fs::MetadataExt;
    #[expect(clippy::disallowed_methods, reason = "W3 permits fixture path checks")]
    let root = std::fs::metadata(env!("CARGO_TARGET_TMPDIR")).is_ok_and(|m| m.uid() == 0);
    root
}

// ---- added by the tests of scope 004, round four (review findings t6, t7) ----

#[test]
fn ac10_a_journal_on_hold_for_a_restore_classifies_as_restore_pending() {
    // covers: 0004 AC-10, AC-13 (review finding t6): the hold is read from
    // the file, without Docker and without a tool of the host.
    let (_folder, path) = created("hold-restore");
    damage(
        &path,
        "UPDATE store SET hold = 'restore_pending' WHERE singleton = 1;",
    );
    assert_eq!(
        Journal::read(&path).classify(&config(Some(X))).name(),
        "restore_pending"
    );
    damage(&path, "UPDATE store SET hold = 'none' WHERE singleton = 1;");
    assert_eq!(ready(&path).name(), "ready");
}

#[test]
fn ac10_open_ready_refuses_a_journal_whose_authority_id_changed() {
    // covers: 0004 AC-10 (review finding t7): the ID is compared again when
    // the journal is opened for writing, not only the revision and the start
    // sequence.
    let (_folder, path) = created("open-id-changed");
    let state = ready(&path);
    damage(&path, &format!("UPDATE store SET authority_id = X'{Y}';"));
    assert_eq!(
        Journal::open_ready(&path, authority(&state)).map(|(_, sequence)| sequence),
        Err(JournalError::Changed)
    );
    assert_eq!(integer(&path, "SELECT start_sequence FROM store"), 0);
}

#[test]
fn ac10_a_latch_write_refuses_a_journal_whose_authority_id_changed() {
    // covers: 0004 AC-10, 0003 AC-17 (review finding t7).
    let (_folder, path) = created("latch-id-changed");
    let state = ready(&path);
    let (mut journal, _) = Journal::open_ready(&path, authority(&state)).expect("open");
    damage(&path, &format!("UPDATE store SET authority_id = X'{Y}';"));
    assert_eq!(
        journal.set_conflict_latch(&cluster("main"), 0),
        Err(JournalError::Changed)
    );
    assert_eq!(
        integer(
            &path,
            "SELECT conflict_latched FROM approved_topology WHERE cluster_id = 'main'"
        ),
        0
    );
}
