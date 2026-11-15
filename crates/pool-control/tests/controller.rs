//! The board, `build_snapshot`, `Controller::round`, a latch that is
//! recorded and one whose write fails and is tried again, `JournalRead` and
//! the probe row (spec 0004, "Control loop and topology snapshot" and "The
//! probe"; `0004 AC-12`, `0003 AC-17`, `0003 AC-18`).
//!
//! Setup: cluster `main` with `pg1` to `pg3` (writer `pg1`) and cluster
//! `side` with `sd1` and `sd2` (writer `sd1`), the default probe keys (an
//! expiry of 6 seconds), samples at 98 seconds and a now of 100 seconds.
//! The journal is a real file under `CARGO_TARGET_TMPDIR`, written by
//! `Journal::create`.

use std::path::{Path, PathBuf};
use std::time::Duration;

use pool_config::{AuthorityId, ClusterId, NodeId, SystemIdentifier, ValidatedConfig};
use pool_control::{
    AuthorityState, ClusterAdmission, Controller, Journal, JournalError, MonotonicTime, NodeFacts,
    Observation, ObservationBoard, ObservationOutcome, SnapshotGeneration, TimelineId,
    TopologySnapshot, WriterAdmission, build_snapshot, observation_from_row,
};
use rusqlite::Connection;

const SYSTEM_ID: &str = "7412345678901234567";
const X: &str = "0123456789abcdef0123456789abcdef";

fn folder(name: &str) -> PathBuf {
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("pool-control-ctl-{name}"));
    if path.exists() {
        std::fs::remove_dir_all(&path).expect("remove old test folder");
    }
    std::fs::create_dir_all(&path).expect("create test folder");
    path
}

fn config_with(witness: Option<&str>, probe: &str) -> ValidatedConfig {
    let mut text =
        String::from("config_version = 1\n\n[control]\nstate_directory = \"/var/lib/pool\"\n");
    if let Some(witness) = witness {
        text.push_str(&format!("authority_id = \"{witness}\"\n"));
    }
    text.push_str(probe);
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

fn config(witness: Option<&str>) -> ValidatedConfig {
    config_with(witness, "")
}

fn at(seconds: u64) -> MonotonicTime {
    MonotonicTime::from_origin(Duration::from_secs(seconds))
}

fn node(id: &str) -> NodeId {
    NodeId::parse(id).expect("node id")
}

fn cluster(id: &str) -> ClusterId {
    ClusterId::parse(id).expect("cluster id")
}

fn reached(id: &str, seconds: u64, timeline: u32, in_recovery: bool) -> Observation {
    Observation::new(
        node(id),
        at(seconds),
        ObservationOutcome::Reached(NodeFacts {
            system_identifier: SystemIdentifier::parse(SYSTEM_ID).expect("system identifier"),
            timeline: TimelineId::from_observed(timeline).expect("timeline"),
            in_recovery,
        }),
    )
}

/// The healthy samples of every node, at `seconds`.
fn healthy(seconds: u64) -> Vec<Observation> {
    [
        ("pg1", false),
        ("pg2", true),
        ("pg3", true),
        ("sd1", false),
        ("sd2", true),
    ]
    .into_iter()
    .map(|(id, in_recovery)| reached(id, seconds, 1, in_recovery))
    .collect()
}

fn board_of(observations: Vec<Observation>) -> ObservationBoard {
    let mut board = ObservationBoard::new();
    for observation in observations {
        board.record(observation);
    }
    board
}

/// A journal written by the real functions, read and classified as `Ready`
/// with the witness `X`, and opened for writing.
fn ready_journal(name: &str) -> (PathBuf, AuthorityState, Journal) {
    let folder = folder(name);
    let path = folder.join("journal.sqlite3");
    std::fs::File::create_new(&path).expect("create the empty journal file");
    let plan = Journal::read(&path)
        .bootstrap_plan(&config(None), &board_of(healthy(98)), at(100))
        .expect("bootstrap is allowed");
    Journal::create(
        &path,
        &plan,
        AuthorityId::parse(X).expect("authority id"),
        0,
    )
    .expect("create");
    let state = Journal::read(&path).classify(&config(Some(X)));
    let AuthorityState::Ready(authority) = &state else {
        panic!("expected ready, got {}", state.name());
    };
    let (journal, sequence) = Journal::open_ready(&path, authority).expect("open");
    assert_eq!(sequence.get(), 1);
    (path, state, journal)
}

fn admission_of<'a>(snapshot: &'a TopologySnapshot, name: &str) -> &'a ClusterAdmission {
    snapshot
        .clusters()
        .iter()
        .find(|entry| entry.cluster() == &cluster(name))
        .expect("the cluster is in the snapshot")
}

fn reasons(snapshot: &TopologySnapshot, name: &str) -> Vec<&'static str> {
    match admission_of(snapshot, name).admission() {
        WriterAdmission::Closed(closed) => closed.reasons().iter().map(|r| r.as_str()).collect(),
        WriterAdmission::Open(_) => Vec::new(),
    }
}

/// The writer and the revision of an open answer.
fn open(snapshot: &TopologySnapshot, name: &str) -> (String, u64) {
    match admission_of(snapshot, name).admission() {
        WriterAdmission::Open(primary) => (
            primary.node().as_str().to_owned(),
            primary.authority_revision().get(),
        ),
        WriterAdmission::Closed(closed) => panic!("{name} is closed: {closed:?}"),
    }
}

#[test]
fn ac18_the_board_keeps_one_sample_per_node() {
    // covers: 0003 AC-18. Later wins; on a tie the one held stays.
    let mut board = ObservationBoard::new();
    assert!(board.observations().is_empty());
    board.record(reached("pg1", 98, 1, false));
    board.record(reached("pg1", 99, 1, false));
    board.record(reached("pg1", 99, 2, true));
    board.record(reached("pg1", 97, 3, true));
    board.record(Observation::new(
        node("pg2"),
        at(50),
        ObservationOutcome::Failed,
    ));
    assert_eq!(
        board.observations(),
        [
            reached("pg1", 99, 1, false),
            Observation::new(node("pg2"), at(50), ObservationOutcome::Failed),
        ]
    );
    // A failed sample that is later replaces a good one.
    board.record(Observation::new(
        node("pg1"),
        at(100),
        ObservationOutcome::Failed,
    ));
    assert_eq!(
        board.observations()[0].outcome(),
        ObservationOutcome::Failed
    );
    assert_eq!(board.observations().len(), 2);
}

#[test]
fn ac18_a_now_earlier_than_the_latest_sample_gives_the_same_snapshot() {
    // covers: 0003 AC-18. The sample of `pg2` that reports a second primary
    // is read as one, whatever now the caller passes.
    let config = config(Some(X));
    let (_path, state, _journal) = ready_journal("clamp");
    let mut samples = healthy(101);
    samples.push(reached("pg2", 103, 1, false));
    let board = board_of(samples);
    let generation = SnapshotGeneration::FIRST;
    let latest = build_snapshot(&config, &state, &board, at(103), generation);
    for now in [0, 50, 100, 102, 103] {
        assert_eq!(
            build_snapshot(&config, &state, &board, at(now), generation),
            latest,
            "now {now}"
        );
    }
    assert_eq!(reasons(&latest, "main"), ["evidence.second_primary"]);
    assert_eq!(open(&latest, "side"), ("sd1".to_owned(), 1));
    // A now after the latest sample is used as it is.
    let later = build_snapshot(&config, &state, &board, at(108), generation);
    assert_eq!(reasons(&later, "side"), ["evidence.writer_missing"]);
}

#[test]
fn ac12_build_snapshot_answers_every_cluster_with_the_expiry_of_the_configuration() {
    // covers: 0004 AC-12, 0003 AC-18.
    let (_path, state, _journal) = ready_journal("expiry");
    let board = board_of(healthy(98));
    let snapshot = build_snapshot(
        &config(Some(X)),
        &state,
        &board,
        at(100),
        SnapshotGeneration::FIRST,
    );
    assert_eq!(snapshot.generation(), SnapshotGeneration::FIRST);
    let names: Vec<&str> = snapshot
        .clusters()
        .iter()
        .map(|entry| entry.cluster().as_str())
        .collect();
    assert_eq!(names, ["main", "side"]);
    for name in ["main", "side"] {
        assert_eq!(
            admission_of(&snapshot, name).fresh_until(),
            Some(at(104)),
            "{name}: 98 seconds plus the expiry of 6"
        );
    }
    // An expiry of 1 second makes the same samples stale.
    let short = config_with(
        Some(X),
        "\n[probe]\nperiod_ms = 100\ntimeout_ms = 100\nobservation_expiry_ms = 1000\n",
    );
    let snapshot = build_snapshot(&short, &state, &board, at(100), SnapshotGeneration::FIRST);
    for name in ["main", "side"] {
        assert_eq!(reasons(&snapshot, name), ["evidence.writer_missing"]);
        assert_eq!(admission_of(&snapshot, name).fresh_until(), None);
    }
    let snapshot = build_snapshot(&short, &state, &board, at(98), SnapshotGeneration::FIRST);
    assert_eq!(
        admission_of(&snapshot, "main").fresh_until(),
        Some(at(99)),
        "98 seconds plus the expiry of 1"
    );
}

#[test]
fn ac17_a_round_closes_in_memory_first_and_the_receipt_follows() {
    // covers: 0003 AC-17, 0004 AC-12. Rows a, d and o of walk 3 for `main`,
    // while `side` keeps its answer.
    let config = config(Some(X));
    let (path, state, mut journal) = ready_journal("round");
    let mut controller = Controller::new(state);
    for observation in healthy(98) {
        controller.record(observation);
    }
    let first = controller.round(&config, at(100));
    assert_eq!(first.generation().get(), 1);
    assert_eq!(open(&first, "main"), ("pg1".to_owned(), 1));
    assert_eq!(open(&first, "side"), ("sd1".to_owned(), 1));
    assert!(controller.unrecorded_latches().is_empty());

    // Row d: `pg2` reports that it is a primary. The round latches `main`
    // in memory, and the snapshot it returns shows the latch.
    controller.record(reached("pg2", 99, 1, false));
    let second = controller.round(&config, at(100));
    assert_eq!(second.generation().get(), 2);
    assert_eq!(
        reasons(&second, "main"),
        ["authority.conflict_latched", "evidence.second_primary"]
    );
    assert_eq!(open(&second, "side"), ("sd1".to_owned(), 1));
    assert_eq!(controller.unrecorded_latches(), [cluster("main")]);
    // Nothing reached the disk yet.
    assert_eq!(latched_on_disk(&path, "main"), 0);

    // The write follows, and the receipt reaches the state.
    let receipt = journal
        .set_conflict_latch(&cluster("main"), 0)
        .expect("write the latch");
    controller.latch_recorded(&receipt);
    assert!(controller.unrecorded_latches().is_empty());
    assert_eq!(latched_on_disk(&path, "main"), 1);

    // Row o: `pg2` is back in recovery, and `main` stays closed. `side` now
    // carries the revision of the receipt.
    controller.record(reached("pg2", 100, 1, true));
    let third = controller.round(&config, at(100));
    assert_eq!(third.generation().get(), 3);
    assert_eq!(reasons(&third, "main"), ["authority.conflict_latched"]);
    assert_eq!(open(&third, "side"), ("sd1".to_owned(), 2));
    assert!(controller.unrecorded_latches().is_empty());
    match controller.state() {
        AuthorityState::Ready(authority) => assert_eq!(authority.authority_revision().get(), 2),
        other => panic!("{}", other.name()),
    }
}

fn latched_on_disk(path: &Path, name: &str) -> i64 {
    Connection::open(path)
        .expect("open")
        .query_row(
            "SELECT conflict_latched FROM approved_topology WHERE cluster_id = ?1",
            [name],
            |row| row.get(0),
        )
        .expect("query")
}

#[test]
fn ac17_a_latch_whose_write_fails_holds_in_memory_and_is_tried_again() {
    // covers: 0003 AC-17. The write fails between the two calls.
    let config = config(Some(X));
    let (path, state, mut journal) = ready_journal("retry");
    let mut controller = Controller::new(state);
    for observation in healthy(98) {
        controller.record(observation);
    }
    controller.record(reached("pg2", 99, 1, false));
    let snapshot = controller.round(&config, at(100));
    assert_eq!(
        reasons(&snapshot, "main"),
        ["authority.conflict_latched", "evidence.second_primary"]
    );

    let holder = Connection::open(&path).expect("open");
    holder
        .execute_batch("BEGIN IMMEDIATE;")
        .expect("hold the lock");
    for name in controller.unrecorded_latches().to_vec() {
        assert_eq!(
            journal.set_conflict_latch(&name, 0),
            Err(JournalError::Busy)
        );
    }
    assert_eq!(controller.unrecorded_latches(), [cluster("main")]);

    // The next rounds keep the cluster closed and listed once.
    controller.record(reached("pg2", 100, 1, true));
    for round in 0..3 {
        let snapshot = controller.round(&config, at(100));
        assert_eq!(
            reasons(&snapshot, "main"),
            ["authority.conflict_latched"],
            "{round}"
        );
        assert_eq!(open(&snapshot, "side"), ("sd1".to_owned(), 1));
        assert_eq!(controller.unrecorded_latches(), [cluster("main")]);
    }

    holder.execute_batch("ROLLBACK;").expect("release");
    let receipt = journal
        .set_conflict_latch(&cluster("main"), 0)
        .expect("the retry");
    controller.latch_recorded(&receipt);
    assert!(controller.unrecorded_latches().is_empty());
    let snapshot = controller.round(&config, at(100));
    assert_eq!(reasons(&snapshot, "main"), ["authority.conflict_latched"]);
    assert_eq!(open(&snapshot, "side"), ("sd1".to_owned(), 2));
}

#[test]
fn ac17_a_latch_the_journal_already_holds_is_not_written_again() {
    // covers: 0003 AC-17.
    let config = config(Some(X));
    let (path, _state, mut journal) = ready_journal("held-latch");
    journal
        .set_conflict_latch(&cluster("main"), 0)
        .expect("latch");
    drop(journal);
    let state = Journal::read(&path).classify(&config);
    let mut controller = Controller::new(state);
    for observation in healthy(98) {
        controller.record(observation);
    }
    controller.record(reached("pg3", 99, 2, true));
    let snapshot = controller.round(&config, at(100));
    assert_eq!(
        reasons(&snapshot, "main"),
        ["authority.conflict_latched", "evidence.timeline_ahead"]
    );
    assert!(controller.unrecorded_latches().is_empty());
    assert_eq!(open(&snapshot, "side"), ("sd1".to_owned(), 2));
}

#[test]
fn ac17_a_receipt_for_a_cluster_not_latched_changes_nothing() {
    // covers: 0003 AC-17.
    let config = config(Some(X));
    let (path, _state, mut journal) = ready_journal("foreign-receipt");
    let state = Journal::read(&path).classify(&config);
    let receipt = journal
        .set_conflict_latch(&cluster("side"), 0)
        .expect("latch");
    let mut controller = Controller::new(state);
    controller.latch_recorded(&receipt);
    for observation in healthy(98) {
        controller.record(observation);
    }
    let snapshot = controller.round(&config, at(100));
    assert_eq!(open(&snapshot, "main"), ("pg1".to_owned(), 1));
}

#[test]
fn ac17_a_state_that_is_not_ready_never_latches() {
    // covers: 0003 AC-17, 0003 AC-7.
    let config = config(None);
    let folder = folder("not-ready");
    let state = Journal::read(&folder.join("journal.sqlite3")).classify(&config);
    assert_eq!(state.name(), "uninitialized");
    let mut controller = Controller::new(state.clone());
    for observation in healthy(98) {
        controller.record(observation);
    }
    controller.record(reached("pg2", 99, 1, false));
    let snapshot = controller.round(&config, at(100));
    for name in ["main", "side"] {
        assert_eq!(reasons(&snapshot, name), ["authority.uninitialized"]);
    }
    assert!(controller.unrecorded_latches().is_empty());
    assert_eq!(controller.state(), &state);
}

#[test]
fn ac12_a_failed_probe_row_takes_the_evidence_away_at_once() {
    // covers: 0004 AC-12. Rows through `observation_from_row` onto the board.
    let config = config(Some(X));
    let (_path, state, _journal) = ready_journal("probe-row");
    let mut controller = Controller::new(state);
    for (id, in_recovery) in [
        ("pg1", "false"),
        ("pg2", "true"),
        ("pg3", "true"),
        ("sd1", "false"),
        ("sd2", "true"),
    ] {
        let row = [Some(SYSTEM_ID), Some(in_recovery), Some("1")];
        controller.record(observation_from_row(node(id), at(98), Some(&row)));
    }
    let snapshot = controller.round(&config, at(100));
    assert_eq!(open(&snapshot, "main"), ("pg1".to_owned(), 1));
    // The next probe of `pg1` gets no row: its good sample is replaced.
    controller.record(observation_from_row(node("pg1"), at(99), None));
    let snapshot = controller.round(&config, at(100));
    assert_eq!(reasons(&snapshot, "main"), ["evidence.writer_missing"]);
    assert!(controller.unrecorded_latches().is_empty());
    assert_eq!(controller.board().observations().len(), 5);
}

#[test]
fn ac19_journal_read_plans_bootstrap_with_the_board_and_the_configuration() {
    // covers: 0003 AC-19, 0003 AC-18. The plan uses the expiry of the
    // configuration and the later of now and the latest sample.
    let folder = folder("journal-read-plan");
    let path = folder.join("journal.sqlite3");
    let read = Journal::read(&path);
    let config = config(None);
    // Samples dated after now still count: the now is the latest sample.
    let plan = read
        .bootstrap_plan(&config, &board_of(healthy(120)), at(100))
        .expect("plan");
    let writers: Vec<&str> = plan
        .clusters()
        .iter()
        .map(|topology| topology.approved_writer.as_str())
        .collect();
    assert_eq!(writers, ["pg1", "sd1"]);
    // An expiry of 1 second makes samples of 98 seconds stale at 100.
    let short = config_with(
        None,
        "\n[probe]\nperiod_ms = 100\ntimeout_ms = 100\nobservation_expiry_ms = 1000\n",
    );
    let refused = read
        .bootstrap_plan(&short, &board_of(healthy(98)), at(100))
        .expect_err("stale");
    let names: Vec<&str> = refused.reasons().iter().map(|r| r.as_str()).collect();
    assert_eq!(names, ["bootstrap.evidence_missing"]);
    // An empty board is no evidence.
    assert!(
        read.bootstrap_plan(&config, &ObservationBoard::new(), at(100))
            .is_err()
    );
}

#[test]
fn authority_state_names_are_stable() {
    // covers: 0003 AC-7. The names the commands print.
    let folder = folder("names");
    let path = folder.join("journal.sqlite3");
    assert_eq!(
        Journal::read(&path).classify(&config(None)).name(),
        "uninitialized"
    );
    assert_eq!(
        Journal::read(&path).classify(&config(Some(X))).name(),
        "missing"
    );
    for (state, name) in [
        (AuthorityState::Uninitialized, "uninitialized"),
        (AuthorityState::Unconfirmed, "unconfirmed"),
        (AuthorityState::Missing, "missing"),
        (AuthorityState::Unreadable, "unreadable"),
        (AuthorityState::Mismatch, "mismatch"),
        (AuthorityState::Held, "restore_pending"),
    ] {
        assert_eq!(state.name(), name);
    }
}
