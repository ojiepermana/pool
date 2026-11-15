//! The small domain of `0003 AC-12`: authority is never derived from
//! observation.
//!
//! Cluster `main` with `pg1`, `pg2` and `pg3`, in state `Ready`. Every
//! combination of `expected_writer`, approved writer, quarantine of `pg3`,
//! conflict latch, and one of eight observations per node: 2 * 2 * 2 * 2 *
//! 8^3 = 8192 cases. The expected answer comes from a short rule of this test,
//! written from spec 0003, "Writer admission", and not from the code under
//! test.

use std::time::Duration;

use pool_config::{
    AuthorityId, AuthorityRevision, ClusterId, NodeId, StartSequence, SystemIdentifier,
    TopologyEpoch, ValidatedConfig,
};
use pool_control::{
    ApprovedTopology, AuthorityState, Hold, MonotonicTime, NodeFacts, Observation,
    ObservationOutcome, Quarantine, StoreProbe, StoredAuthority, TimelineId, WriterAdmission,
    classify_startup, writer_admission,
};

const SYSTEM_ID: &str = "7412345678901234567";
const OTHER_SYSTEM_ID: &str = "7412345678901234999";
const WITNESS: &str = "0123456789abcdef0123456789abcdef";
const NODES: [&str; 3] = ["pg1", "pg2", "pg3"];
const EXPIRY: Duration = Duration::from_secs(6);

/// The eight observations a node can have in this domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Seen {
    None,
    Stale,
    Failed,
    PrimaryTimeline1,
    PrimaryTimeline2,
    ReplicaTimeline1,
    ReplicaTimeline2,
    PrimaryOtherSystem,
}

const ALL_SEEN: [Seen; 8] = [
    Seen::None,
    Seen::Stale,
    Seen::Failed,
    Seen::PrimaryTimeline1,
    Seen::PrimaryTimeline2,
    Seen::ReplicaTimeline1,
    Seen::ReplicaTimeline2,
    Seen::PrimaryOtherSystem,
];

fn node(id: &str) -> NodeId {
    NodeId::parse(id).expect("node id")
}

fn config(expected_writer: &str) -> ValidatedConfig {
    let mut text = format!(
        "config_version = 1\n\n[control]\nstate_directory = \"/var/lib/pool\"\nauthority_id = \"{WITNESS}\"\n\n[[cluster]]\nid = \"main\"\nsystem_identifier = \"{SYSTEM_ID}\"\nexpected_writer = \"{expected_writer}\"\n"
    );
    for (index, id) in NODES.iter().enumerate() {
        text.push_str(&format!(
            "\n[[node]]\nid = \"{id}\"\ncluster = \"main\"\nhost = \"10.0.0.{}\"\nport = 5432\n",
            index + 11
        ));
    }
    pool_config::validate(text.as_bytes()).expect("the test configuration is valid")
}

fn stored(approved_writer: &str, pg3_quarantined: bool, latched: bool) -> StoredAuthority {
    let main = ClusterId::parse("main").expect("cluster id");
    StoredAuthority {
        authority_id: AuthorityId::parse(WITNESS).expect("authority id"),
        authority_revision: AuthorityRevision::FIRST,
        start_sequence: StartSequence::FIRST,
        hold: Hold::None,
        clusters: vec![ApprovedTopology {
            cluster: main.clone(),
            system_identifier: SystemIdentifier::parse(SYSTEM_ID).expect("system identifier"),
            approved_writer: node(approved_writer),
            writer_timeline: TimelineId::from_observed(1).expect("timeline"),
            topology_epoch: TopologyEpoch::FIRST,
            members: NODES.iter().map(|id| node(id)).collect(),
            conflict_latched: latched,
        }],
        quarantine: if pg3_quarantined {
            vec![Quarantine {
                cluster: main,
                node: node("pg3"),
            }]
        } else {
            Vec::new()
        },
    }
}

fn observation(id: &str, seen: Seen) -> Option<Observation> {
    let reached = |timeline: u32, in_recovery: bool, system: &str, sampled: u64| {
        Observation::new(
            node(id),
            MonotonicTime::from_origin(Duration::from_secs(sampled)),
            ObservationOutcome::Reached(NodeFacts {
                system_identifier: SystemIdentifier::parse(system).expect("system identifier"),
                timeline: TimelineId::from_observed(timeline).expect("timeline"),
                in_recovery,
            }),
        )
    };
    match seen {
        Seen::None => None,
        Seen::Stale => Some(reached(1, false, SYSTEM_ID, 90)),
        Seen::Failed => Some(Observation::new(
            node(id),
            MonotonicTime::from_origin(Duration::from_secs(98)),
            ObservationOutcome::Failed,
        )),
        Seen::PrimaryTimeline1 => Some(reached(1, false, SYSTEM_ID, 98)),
        Seen::PrimaryTimeline2 => Some(reached(2, false, SYSTEM_ID, 98)),
        Seen::ReplicaTimeline1 => Some(reached(1, true, SYSTEM_ID, 98)),
        Seen::ReplicaTimeline2 => Some(reached(2, true, SYSTEM_ID, 98)),
        Seen::PrimaryOtherSystem => Some(reached(1, false, OTHER_SYSTEM_ID, 98)),
    }
}

/// The test's own rule. Returns the open node, or `None` for closed, and
/// whether a closed answer asks for the conflict latch.
fn expected(
    expected_writer: usize,
    approved_writer: usize,
    pg3_quarantined: bool,
    latched: bool,
    seen: [Seen; 3],
) -> (Option<usize>, bool) {
    // Another node, not quarantined, that fits the journal's system
    // identifier and claims to be a primary or is on timeline 2.
    let conflict = (0..3).any(|n| {
        n != approved_writer
            && !(n == 2 && pg3_quarantined)
            && matches!(
                seen[n],
                Seen::PrimaryTimeline1 | Seen::PrimaryTimeline2 | Seen::ReplicaTimeline2
            )
    });
    let agreed = expected_writer == approved_writer
        && !latched
        && seen[approved_writer] == Seen::PrimaryTimeline1;
    if agreed && !conflict {
        (Some(approved_writer), false)
    } else {
        (None, conflict)
    }
}

#[test]
fn open_only_on_full_agreement() {
    let now = MonotonicTime::from_origin(Duration::from_secs(100));
    let main = ClusterId::parse("main").expect("cluster id");
    let configs = [config("pg1"), config("pg2")];
    let mut cases = 0;
    let mut opened = 0;

    for (expected_writer, config) in configs.iter().enumerate() {
        for (approved_writer, approved_name) in NODES.iter().copied().enumerate().take(2) {
            for pg3_quarantined in [false, true] {
                for latched in [false, true] {
                    let state = classify_startup(
                        config,
                        StoreProbe::Present(stored(approved_name, pg3_quarantined, latched)),
                    );
                    assert!(matches!(state, AuthorityState::Ready(_)));
                    for a in ALL_SEEN {
                        for b in ALL_SEEN {
                            for c in ALL_SEEN {
                                let seen = [a, b, c];
                                let observations: Vec<Observation> = NODES
                                    .iter()
                                    .zip(seen)
                                    .filter_map(|(id, s)| observation(id, s))
                                    .collect();
                                let answer = writer_admission(
                                    config,
                                    &state,
                                    &main,
                                    &observations,
                                    now,
                                    EXPIRY,
                                );
                                let (want, latches) = expected(
                                    expected_writer,
                                    approved_writer,
                                    pg3_quarantined,
                                    latched,
                                    seen,
                                );
                                let case = format!(
                                    "expected {}, approved {}, pg3 quarantined {pg3_quarantined}, latched {latched}, seen {seen:?}",
                                    NODES[expected_writer], approved_name
                                );
                                match (&answer, want) {
                                    (WriterAdmission::Open(primary), Some(writer)) => {
                                        assert_eq!(primary.node(), &node(NODES[writer]), "{case}");
                                        opened += 1;
                                    }
                                    (WriterAdmission::Closed(closed), None) => {
                                        assert!(!closed.reasons().is_empty(), "{case}");
                                        assert_eq!(closed.latches_conflict(), latches, "{case}");
                                    }
                                    _ => panic!("{case}: got {answer:?}, want {want:?}"),
                                }
                                cases += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    assert_eq!(cases, 8192);
    // Open needs agreement on the writer (2 of 4 pairs), no latch, the writer
    // a primary on timeline 1, and no conflict from the other two nodes.
    assert!(opened > 0);
}
