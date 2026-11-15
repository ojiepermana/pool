//! The four walks of spec 0003, "The four walks": one test each, every row.
//!
//! Shared setup: cluster `main` with `pg1`, `pg2` and `pg3`, system
//! identifier `7412345678901234567`, `expected_writer = "pg1"`, an expiry of
//! 6 seconds and a `now` of 100 seconds. Fresh means sampled at 98 seconds.

use std::time::Duration;

use pool_config::{
    AuthorityId, AuthorityRevision, ClusterId, NodeId, StartSequence, SystemIdentifier,
    TopologyEpoch, ValidatedConfig,
};
use pool_control::{
    ApprovedTopology, AuthorityState, BootstrapRefusal, ClosedReason, Hold, MonotonicTime,
    NodeFacts, Observation, ObservationOutcome, Quarantine, StoreProbe, StoredAuthority,
    TimelineId, WriterAdmission, classify_startup, plan_bootstrap, writer_admission,
};

const SYSTEM_ID: &str = "7412345678901234567";
const OTHER_SYSTEM_ID: &str = "7412345678901234999";
const X: &str = "0123456789abcdef0123456789abcdef";
const Y: &str = "fedcba9876543210fedcba9876543210";
const EXPIRY: Duration = Duration::from_secs(6);

fn now() -> MonotonicTime {
    at(100)
}

fn at(seconds: u64) -> MonotonicTime {
    MonotonicTime::from_origin(Duration::from_secs(seconds))
}

fn node(id: &str) -> NodeId {
    NodeId::parse(id).expect("node id")
}

fn main_cluster() -> ClusterId {
    ClusterId::parse("main").expect("cluster id")
}

fn timeline(value: u32) -> TimelineId {
    TimelineId::from_observed(value).expect("timeline")
}

fn system_id(text: &str) -> SystemIdentifier {
    SystemIdentifier::parse(text).expect("system identifier")
}

/// The configuration of the shared setup, with what a row changes.
struct Setup<'a> {
    witness: Option<&'a str>,
    expected_writer: &'a str,
    system_identifier: &'a str,
    nodes: &'a [&'a str],
}

impl Default for Setup<'_> {
    fn default() -> Self {
        Setup {
            witness: None,
            expected_writer: "pg1",
            system_identifier: SYSTEM_ID,
            nodes: &["pg1", "pg2", "pg3"],
        }
    }
}

impl Setup<'_> {
    fn config(&self) -> ValidatedConfig {
        let mut text =
            String::from("config_version = 1\n\n[control]\nstate_directory = \"/var/lib/pool\"\n");
        if let Some(witness) = self.witness {
            text.push_str(&format!("authority_id = \"{witness}\"\n"));
        }
        text.push_str(&format!(
            "\n[[cluster]]\nid = \"main\"\nsystem_identifier = \"{}\"\nexpected_writer = \"{}\"\n",
            self.system_identifier, self.expected_writer
        ));
        for (index, id) in self.nodes.iter().enumerate() {
            text.push_str(&format!(
                "\n[[node]]\nid = \"{id}\"\ncluster = \"main\"\nhost = \"10.0.0.{}\"\nport = 5432\n",
                index + 11
            ));
        }
        pool_config::validate(text.as_bytes()).expect("the test configuration is valid")
    }
}

fn witness(witness: &str) -> Setup<'_> {
    Setup {
        witness: Some(witness),
        ..Setup::default()
    }
}

fn reached(id: &str, sampled: u64, tl: u32, in_recovery: bool, sys: &str) -> Observation {
    Observation::new(
        node(id),
        at(sampled),
        ObservationOutcome::Reached(NodeFacts {
            system_identifier: system_id(sys),
            timeline: timeline(tl),
            in_recovery,
        }),
    )
}

fn primary(id: &str, tl: u32) -> Observation {
    reached(id, 98, tl, false, SYSTEM_ID)
}

fn replica(id: &str, tl: u32) -> Observation {
    reached(id, 98, tl, true, SYSTEM_ID)
}

fn failed(id: &str) -> Observation {
    Observation::new(node(id), at(98), ObservationOutcome::Failed)
}

fn healthy() -> Vec<Observation> {
    vec![primary("pg1", 1), replica("pg2", 1), replica("pg3", 1)]
}

/// The healthy observations with the samples of the named nodes replaced.
fn healthy_with(replaced: Vec<Observation>) -> Vec<Observation> {
    let mut observations: Vec<Observation> = healthy()
        .into_iter()
        .filter(|o| !replaced.iter().any(|r| r.node() == o.node()))
        .collect();
    observations.extend(replaced);
    observations
}

/// Healthy observations without any sample of the named node.
fn healthy_without(id: &str) -> Vec<Observation> {
    healthy()
        .into_iter()
        .filter(|o| o.node().as_str() != id)
        .collect()
}

/// The journal of walk 3: `main`, writer `pg1`, timeline 1, epoch 1, the
/// members `pg1`, `pg2` and `pg3`, no conflict latch.
fn journal(id: &str) -> StoredAuthority {
    StoredAuthority {
        authority_id: AuthorityId::parse(id).expect("authority id"),
        authority_revision: AuthorityRevision::FIRST,
        start_sequence: StartSequence::FIRST,
        hold: Hold::None,
        clusters: vec![ApprovedTopology {
            cluster: main_cluster(),
            system_identifier: system_id(SYSTEM_ID),
            approved_writer: node("pg1"),
            writer_timeline: timeline(1),
            topology_epoch: TopologyEpoch::FIRST,
            members: vec![node("pg1"), node("pg2"), node("pg3")],
            conflict_latched: false,
        }],
        quarantine: Vec::new(),
    }
}

fn admission(
    config: &ValidatedConfig,
    state: &AuthorityState,
    observations: &[Observation],
) -> WriterAdmission {
    writer_admission(config, state, &main_cluster(), observations, now(), EXPIRY)
}

#[track_caller]
fn assert_closed(admission: &WriterAdmission, expected: &[&str], latches: bool) {
    match admission {
        WriterAdmission::Closed(closed) => {
            let names: Vec<&str> = closed.reasons().iter().map(|r| r.as_str()).collect();
            assert_eq!(names, expected);
            assert_eq!(closed.latches_conflict(), latches, "latches_conflict");
        }
        WriterAdmission::Open(primary) => panic!("expected closed, got open {primary:?}"),
    }
}

#[track_caller]
fn assert_open(admission: &WriterAdmission, writer: &str, epoch: u64, revision: u64) {
    match admission {
        WriterAdmission::Open(primary) => {
            assert_eq!(primary.cluster(), &main_cluster());
            assert_eq!(primary.node(), &node(writer));
            assert_eq!(primary.topology_epoch().get(), epoch);
            assert_eq!(primary.authority_revision().get(), revision);
        }
        WriterAdmission::Closed(closed) => panic!("expected open, got closed {closed:?}"),
    }
}

#[track_caller]
fn assert_refused(
    config: &ValidatedConfig,
    probe: &StoreProbe,
    observations: &[Observation],
    expected: &[&str],
) {
    match plan_bootstrap(config, probe, observations, now(), EXPIRY) {
        Ok(plan) => panic!("expected a refusal, got {plan:?}"),
        Err(refused) => {
            let names: Vec<&str> = refused.reasons().iter().map(|r| r.as_str()).collect();
            assert_eq!(names, expected);
        }
    }
}

#[test]
fn walk_clean_startup() {
    let config = Setup::default().config();

    // 1. First start on a new machine.
    let state = classify_startup(&config, StoreProbe::Absent);
    assert_eq!(state, AuthorityState::Uninitialized);
    assert_closed(
        &admission(&config, &state, &healthy()),
        &["authority.uninitialized"],
        false,
    );

    // 2. The operator runs bootstrap.
    let plan = plan_bootstrap(&config, &StoreProbe::Absent, &healthy(), now(), EXPIRY)
        .expect("a bootstrap plan");
    assert_eq!(
        plan.clusters(),
        &[ApprovedTopology {
            cluster: main_cluster(),
            system_identifier: system_id(SYSTEM_ID),
            approved_writer: node("pg1"),
            writer_timeline: timeline(1),
            topology_epoch: TopologyEpoch::FIRST,
            members: vec![node("pg1"), node("pg2"), node("pg3")],
            conflict_latched: false,
        }]
    );
    assert_eq!(plan.clusters()[0].topology_epoch.get(), 1);

    // 2a to 2e. Refusals.
    let absent = StoreProbe::Absent;
    assert_refused(
        &config,
        &absent,
        &healthy_without("pg3"),
        &["bootstrap.evidence_missing"],
    );
    assert_refused(
        &config,
        &absent,
        &healthy_with(vec![primary("pg2", 1)]),
        &["bootstrap.second_primary"],
    );
    assert_refused(
        &config,
        &absent,
        &healthy_with(vec![replica("pg1", 1)]),
        &["bootstrap.writer_in_recovery"],
    );
    assert_refused(
        &config,
        &absent,
        &healthy_with(vec![reached("pg2", 98, 1, true, OTHER_SYSTEM_ID)]),
        &["bootstrap.identity_mismatch"],
    );
    assert_refused(
        &config,
        &absent,
        &healthy_with(vec![replica("pg2", 2)]),
        &["bootstrap.timeline_ahead"],
    );

    // 3. Start after bootstrap, configuration not yet updated.
    let state = classify_startup(&config, StoreProbe::Present(journal(X)));
    assert_eq!(state, AuthorityState::Unconfirmed);
    assert_closed(
        &admission(&config, &state, &healthy()),
        &["authority.unconfirmed"],
        false,
    );

    // 4. The operator adds the witness.
    let config = witness(X).config();
    let state = classify_startup(&config, StoreProbe::Present(journal(X)));
    assert!(matches!(state, AuthorityState::Ready(_)));
    assert_open(&admission(&config, &state, &healthy()), "pg1", 1, 1);
}

#[test]
fn walk_missing_journal() {
    let config = witness(X).config();

    // 1. The journal is gone.
    let state = classify_startup(&config, StoreProbe::Absent);
    assert_eq!(state, AuthorityState::Missing);
    assert_closed(
        &admission(&config, &state, &healthy()),
        &["authority.missing"],
        false,
    );
    // 1a. Someone tries bootstrap.
    assert_refused(
        &config,
        &StoreProbe::Absent,
        &healthy(),
        &["bootstrap.witness_present"],
    );

    // 2. The file is damaged.
    let state = classify_startup(&config, StoreProbe::Unreadable);
    assert_eq!(state, AuthorityState::Unreadable);
    assert_closed(
        &admission(&config, &state, &healthy()),
        &["authority.unreadable"],
        false,
    );
    // 2a. Someone tries bootstrap.
    assert_refused(
        &config,
        &StoreProbe::Unreadable,
        &healthy(),
        &["bootstrap.store_present", "bootstrap.witness_present"],
    );

    // 3. A journal of another daemon was copied in.
    let state = classify_startup(&config, StoreProbe::Present(journal(Y)));
    assert_eq!(state, AuthorityState::Mismatch);
    assert_closed(
        &admission(&config, &state, &healthy()),
        &["authority.mismatch"],
        false,
    );
    // Bootstrap is refused over a present journal too.
    assert_refused(
        &config,
        &StoreProbe::Present(journal(Y)),
        &healthy(),
        &["bootstrap.store_present", "bootstrap.witness_present"],
    );
}

#[test]
fn walk_conflicting_evidence() {
    let ready = |config: &ValidatedConfig, stored: StoredAuthority| {
        let state = classify_startup(config, StoreProbe::Present(stored));
        assert!(matches!(state, AuthorityState::Ready(_)), "{state:?}");
        state
    };
    let config = witness(X).config();
    let state = ready(&config, journal(X));

    // a. Nothing differs.
    assert_open(&admission(&config, &state, &healthy()), "pg1", 1, 1);

    // b. The configuration names another writer.
    let config_b = Setup {
        expected_writer: "pg2",
        ..witness(X)
    }
    .config();
    let state_b = ready(&config_b, journal(X));
    assert_closed(
        &admission(&config_b, &state_b, &healthy()),
        &["authority.config_conflict"],
        false,
    );

    // c. A promotion nobody recorded: pg2 is not adopted.
    assert_closed(
        &admission(
            &config,
            &state,
            &healthy_with(vec![replica("pg1", 1), primary("pg2", 1)]),
        ),
        &["evidence.writer_in_recovery", "evidence.second_primary"],
        true,
    );

    // d. A second primary.
    let row_d = admission(&config, &state, &healthy_with(vec![primary("pg2", 1)]));
    assert_closed(&row_d, &["evidence.second_primary"], true);

    // e. The writer's sample is as old as the expiry.
    assert_closed(
        &admission(
            &config,
            &state,
            &healthy_with(vec![reached("pg1", 94, 1, false, SYSTEM_ID)]),
        ),
        &["evidence.writer_missing"],
        false,
    );

    // f. The probe of the writer failed.
    assert_closed(
        &admission(&config, &state, &healthy_with(vec![failed("pg1")])),
        &["evidence.writer_missing"],
        false,
    );

    // g. The writer reports another system identifier.
    assert_closed(
        &admission(
            &config,
            &state,
            &healthy_with(vec![reached("pg1", 98, 1, false, OTHER_SYSTEM_ID)]),
        ),
        &["evidence.writer_identity"],
        false,
    );

    // h. The writer reports timeline 2.
    assert_closed(
        &admission(&config, &state, &healthy_with(vec![primary("pg1", 2)])),
        &["evidence.writer_timeline"],
        false,
    );

    // i. A replica is on a later timeline.
    assert_closed(
        &admission(&config, &state, &healthy_with(vec![replica("pg3", 2)])),
        &["evidence.timeline_ahead"],
        true,
    );

    // j. Silence about replicas does not close.
    assert_open(
        &admission(
            &config,
            &state,
            &healthy_with(vec![failed("pg2"), reached("pg3", 90, 1, true, SYSTEM_ID)]),
        ),
        "pg1",
        1,
        1,
    );

    // k. A quarantined node that says it is a primary does not close.
    let mut quarantined_pg2 = journal(X);
    quarantined_pg2.quarantine.push(Quarantine {
        cluster: main_cluster(),
        node: node("pg2"),
    });
    let state_k = ready(&config, quarantined_pg2);
    assert_open(
        &admission(&config, &state_k, &healthy_with(vec![primary("pg2", 1)])),
        "pg1",
        1,
        1,
    );

    // l. The configuration gives main another system identifier.
    let config_l = Setup {
        system_identifier: OTHER_SYSTEM_ID,
        ..witness(X)
    }
    .config();
    let state_l = ready(&config_l, journal(X));
    assert_closed(
        &admission(&config_l, &state_l, &healthy()),
        &["authority.config_conflict"],
        false,
    );

    // m. The journal has no row for main.
    let mut no_main = journal(X);
    no_main.clusters.clear();
    let state_m = ready(&config, no_main);
    assert_closed(
        &admission(&config, &state_m, &healthy()),
        &["authority.cluster_unknown"],
        false,
    );

    // n. The writer is in the quarantine.
    let mut quarantined_pg1 = journal(X);
    quarantined_pg1.quarantine.push(Quarantine {
        cluster: main_cluster(),
        node: node("pg1"),
    });
    let state_n = ready(&config, quarantined_pg1);
    assert_closed(
        &admission(&config, &state_n, &healthy()),
        &["authority.writer_quarantined"],
        false,
    );

    // o. The conflict latch is set. Read with row d: once pg2 is out of
    // sight, the answer stays closed.
    let mut latched = journal(X);
    latched.clusters[0].conflict_latched = true;
    let state_o = ready(&config, latched);
    assert_closed(
        &admission(&config, &state_o, &healthy()),
        &["authority.conflict_latched"],
        false,
    );
    assert_closed(
        &admission(&config, &state_o, &healthy_without("pg2")),
        &["authority.conflict_latched"],
        false,
    );

    // p. The configuration no longer lists pg3, a member in the journal.
    let config_p = Setup {
        nodes: &["pg1", "pg2"],
        ..witness(X)
    }
    .config();
    let state_p = ready(&config_p, journal(X));
    assert_closed(
        &admission(&config_p, &state_p, &healthy()),
        &["authority.config_conflict"],
        false,
    );

    // Another cluster that neither side knows.
    let unknown = ClusterId::parse("other").expect("cluster id");
    match writer_admission(&config, &state, &unknown, &healthy(), now(), EXPIRY) {
        WriterAdmission::Closed(closed) => {
            assert_eq!(closed.reasons(), &[ClosedReason::ClusterUnknown]);
        }
        WriterAdmission::Open(primary) => panic!("expected closed, got {primary:?}"),
    }
}

#[test]
fn walk_restore_before_promotion() {
    // The journal of the backup: writer pg1, timeline 1, epoch 1.
    let backup = journal(X);
    let mut supported = backup.clone();
    supported.hold = Hold::RestorePending;

    // 1. A supported backup is restored: held whatever the observations say.
    let config = witness(X).config();
    let state = classify_startup(&config, StoreProbe::Present(supported.clone()));
    assert_eq!(state, AuthorityState::Held);
    for observations in [
        healthy(),
        Vec::new(),
        healthy_with(vec![primary("pg1", 1), primary("pg2", 2)]),
    ] {
        assert_closed(
            &admission(&config, &state, &observations),
            &["authority.restore_pending"],
            false,
        );
    }
    let config_pg2 = Setup {
        expected_writer: "pg2",
        ..witness(X)
    }
    .config();
    let state_pg2 = classify_startup(&config_pg2, StoreProbe::Present(supported));
    assert_eq!(state_pg2, AuthorityState::Held);
    assert_closed(
        &admission(&config_pg2, &state_pg2, &healthy()),
        &["authority.restore_pending"],
        false,
    );

    // 2. A raw copy, and the configuration was updated to pg2.
    let state_raw = classify_startup(&config_pg2, StoreProbe::Present(backup.clone()));
    assert!(matches!(state_raw, AuthorityState::Ready(_)));
    let promoted = vec![primary("pg2", 2), replica("pg3", 1)];
    assert_closed(
        &admission(&config_pg2, &state_raw, &promoted),
        &[
            "authority.config_conflict",
            "evidence.writer_missing",
            "evidence.second_primary",
            "evidence.timeline_ahead",
        ],
        true,
    );

    // 3. The same raw copy, and the configuration is old too.
    let state_old = classify_startup(&config, StoreProbe::Present(backup));
    assert!(matches!(state_old, AuthorityState::Ready(_)));
    assert_closed(
        &admission(&config, &state_old, &promoted),
        &[
            "evidence.writer_missing",
            "evidence.second_primary",
            "evidence.timeline_ahead",
        ],
        true,
    );

    // 4. pg1 is up again as a primary while pg2 cannot be reached.
    assert_closed(
        &admission(
            &config,
            &state_old,
            &[primary("pg1", 1), failed("pg2"), replica("pg3", 2)],
        ),
        &["evidence.timeline_ahead"],
        true,
    );
}

#[test]
fn freshness_rule() {
    let config = witness(X).config();
    let state = classify_startup(&config, StoreProbe::Present(journal(X)));

    // Age just below the expiry is fresh.
    assert_open(
        &admission(
            &config,
            &state,
            &healthy_with(vec![Observation::new(
                node("pg1"),
                MonotonicTime::from_origin(Duration::from_millis(94_001)),
                ObservationOutcome::Reached(NodeFacts {
                    system_identifier: system_id(SYSTEM_ID),
                    timeline: timeline(1),
                    in_recovery: false,
                }),
            )]),
        ),
        "pg1",
        1,
        1,
    );
    // A sample later than now is no evidence.
    assert_closed(
        &admission(
            &config,
            &state,
            &healthy_with(vec![reached("pg1", 101, 1, false, SYSTEM_ID)]),
        ),
        &["evidence.writer_missing"],
        false,
    );
    // The latest sample wins, even when an older one was fine.
    assert_closed(
        &admission(
            &config,
            &state,
            &[
                reached("pg1", 97, 1, false, SYSTEM_ID),
                reached("pg1", 98, 1, true, SYSTEM_ID),
                replica("pg2", 1),
                replica("pg3", 1),
            ],
        ),
        &["evidence.writer_in_recovery"],
        false,
    );
    // Two samples that share the latest time give no evidence.
    assert_closed(
        &admission(
            &config,
            &state,
            &[
                primary("pg1", 1),
                primary("pg1", 1),
                replica("pg2", 1),
                replica("pg3", 1),
            ],
        ),
        &["evidence.writer_missing"],
        false,
    );
    // A sample of a node outside the configuration is ignored.
    let mut observations = healthy();
    observations.push(primary("pg9", 3));
    assert_open(&admission(&config, &state, &observations), "pg1", 1, 1);
}

#[test]
fn inconsistent_journal_is_unreadable() {
    let config = witness(X).config();
    let cases: Vec<StoredAuthority> = {
        let mut twice = journal(X);
        twice.clusters.push(twice.clusters[0].clone());
        let mut member_twice = journal(X);
        member_twice.clusters[0].members.push(node("pg2"));
        let mut writer_outside = journal(X);
        writer_outside.clusters[0].members = vec![node("pg2"), node("pg3")];
        let mut foreign_quarantine = journal(X);
        foreign_quarantine.quarantine.push(Quarantine {
            cluster: ClusterId::parse("other").expect("cluster id"),
            node: node("pg2"),
        });
        vec![twice, member_twice, writer_outside, foreign_quarantine]
    };
    for stored in cases {
        assert_eq!(
            classify_startup(&config, StoreProbe::Present(stored.clone())),
            AuthorityState::Unreadable
        );
        // Also without a witness: an inconsistent journal is never Unconfirmed.
        assert_eq!(
            classify_startup(&Setup::default().config(), StoreProbe::Present(stored)),
            AuthorityState::Unreadable
        );
    }
}

#[test]
fn bootstrap_refusals_need_every_node_and_a_cluster() {
    // No cluster at all.
    let empty = pool_config::validate(b"config_version = 1\n").expect("valid");
    assert_refused(&empty, &StoreProbe::Absent, &[], &["bootstrap.no_cluster"]);

    // Reasons are reported once each, in order, however many nodes cause them.
    let config = Setup::default().config();
    assert_refused(
        &config,
        &StoreProbe::Absent,
        &[replica("pg1", 1), primary("pg2", 3), primary("pg3", 3)],
        &[
            "bootstrap.writer_in_recovery",
            "bootstrap.second_primary",
            "bootstrap.timeline_ahead",
        ],
    );

    // Check 8 is skipped when the writer does not fit; check 4 still applies.
    assert_refused(
        &config,
        &StoreProbe::Absent,
        &[failed("pg1"), replica("pg2", 5), replica("pg3", 1)],
        &["bootstrap.evidence_missing"],
    );

    // The plan carries the timeline the writer reported.
    let plan = plan_bootstrap(
        &config,
        &StoreProbe::Absent,
        &[primary("pg1", 4), replica("pg2", 4), replica("pg3", 3)],
        now(),
        EXPIRY,
    )
    .expect("a plan");
    assert_eq!(plan.clusters()[0].writer_timeline, timeline(4));
    assert_eq!(BootstrapRefusal::NoCluster.as_str(), "bootstrap.no_cluster");
}

// Edge cases beyond the rows of the walks, added by /test. Each one names the
// criterion it locks. The rules are those of spec 0003, "Writer admission",
// "Bootstrap", "Start classification" and the paragraph "Fresh".

/// A sample of a node, at a time given in nanoseconds since the origin.
fn reached_at(id: &str, nanos: u64, tl: u32, in_recovery: bool, sys: &str) -> Observation {
    Observation::new(
        node(id),
        MonotonicTime::from_origin(Duration::from_nanos(nanos)),
        ObservationOutcome::Reached(NodeFacts {
            system_identifier: system_id(sys),
            timeline: timeline(tl),
            in_recovery,
        }),
    )
}

const SECOND: u64 = 1_000_000_000;

#[track_caller]
fn ready(config: &ValidatedConfig, stored: StoredAuthority) -> AuthorityState {
    let state = classify_startup(config, StoreProbe::Present(stored));
    assert!(matches!(state, AuthorityState::Ready(_)), "{state:?}");
    state
}

fn side_cluster() -> ClusterId {
    ClusterId::parse("side").expect("cluster id")
}

/// Two clusters with the same system identifier: `main` (`pg1`, `pg2`,
/// `pg3`, writer `pg1`) and `side` (`pg4`, `pg5`, writer `pg4`).
fn two_clusters(witness: Option<&str>) -> ValidatedConfig {
    let mut text =
        String::from("config_version = 1\n\n[control]\nstate_directory = \"/var/lib/pool\"\n");
    if let Some(witness) = witness {
        text.push_str(&format!("authority_id = \"{witness}\"\n"));
    }
    for (cluster, writer) in [("main", "pg1"), ("side", "pg4")] {
        text.push_str(&format!(
            "\n[[cluster]]\nid = \"{cluster}\"\nsystem_identifier = \"{SYSTEM_ID}\"\nexpected_writer = \"{writer}\"\n"
        ));
    }
    for (index, (id, cluster)) in [
        ("pg1", "main"),
        ("pg2", "main"),
        ("pg3", "main"),
        ("pg4", "side"),
        ("pg5", "side"),
    ]
    .iter()
    .enumerate()
    {
        text.push_str(&format!(
            "\n[[node]]\nid = \"{id}\"\ncluster = \"{cluster}\"\nhost = \"10.0.0.{}\"\nport = 5432\n",
            index + 11
        ));
    }
    pool_config::validate(text.as_bytes()).expect("the test configuration is valid")
}

/// The journal of walk 3 plus an approved topology for `side`.
fn two_journal(id: &str) -> StoredAuthority {
    let mut stored = journal(id);
    stored.clusters.push(ApprovedTopology {
        cluster: side_cluster(),
        system_identifier: system_id(SYSTEM_ID),
        approved_writer: node("pg4"),
        writer_timeline: timeline(1),
        topology_epoch: TopologyEpoch::FIRST,
        members: vec![node("pg4"), node("pg5")],
        conflict_latched: false,
    });
    stored
}

fn side_admission(
    config: &ValidatedConfig,
    state: &AuthorityState,
    observations: &[Observation],
) -> WriterAdmission {
    writer_admission(config, state, &side_cluster(), observations, now(), EXPIRY)
}

fn plan_names(plan: &pool_control::BootstrapPlan) -> Vec<(&str, &str, u32, Vec<&str>)> {
    plan.clusters()
        .iter()
        .map(|t| {
            assert_eq!(t.topology_epoch, TopologyEpoch::FIRST);
            assert!(!t.conflict_latched);
            assert_eq!(t.system_identifier, system_id(SYSTEM_ID));
            (
                t.cluster.as_str(),
                t.approved_writer.as_str(),
                t.writer_timeline.get(),
                t.members.iter().map(NodeId::as_str).collect(),
            )
        })
        .collect()
}

#[test]
fn ac11_freshness_is_exact_at_the_expiry_and_at_now() {
    let config = witness(X).config();
    let state = ready(&config, journal(X));
    let with_pg1 = |nanos: u64| healthy_with(vec![reached_at("pg1", nanos, 1, false, SYSTEM_ID)]);
    let admit = |observations: &[Observation], now_nanos: u64, expiry: Duration| {
        writer_admission(
            &config,
            &state,
            &main_cluster(),
            observations,
            MonotonicTime::from_origin(Duration::from_nanos(now_nanos)),
            expiry,
        )
    };
    let now = 100 * SECOND;

    // Age equal to the expiry is no evidence; one nanosecond younger is.
    assert_closed(
        &admit(&with_pg1(94 * SECOND), now, EXPIRY),
        &["evidence.writer_missing"],
        false,
    );
    assert_open(&admit(&with_pg1(94 * SECOND + 1), now, EXPIRY), "pg1", 1, 1);
    // Sampled exactly at now: age zero, fresh.
    assert_open(&admit(&with_pg1(now), now, EXPIRY), "pg1", 1, 1);
    // One nanosecond later than now: no evidence, whatever the expiry.
    assert_closed(
        &admit(&with_pg1(now + 1), now, EXPIRY),
        &["evidence.writer_missing"],
        false,
    );
    assert_closed(
        &admit(&with_pg1(now + 1), now, Duration::MAX),
        &["evidence.writer_missing"],
        false,
    );
    // An expiry of zero makes every sample stale; one nanosecond keeps a
    // sample taken at now.
    assert_closed(
        &admit(&with_pg1(now), now, Duration::ZERO),
        &["evidence.writer_missing"],
        false,
    );
    let all_at_now = [
        reached_at("pg1", now, 1, false, SYSTEM_ID),
        reached_at("pg2", now, 1, true, SYSTEM_ID),
    ];
    assert_open(
        &admit(&all_at_now, now, Duration::from_nanos(1)),
        "pg1",
        1,
        1,
    );
    // Before the origin moved past the sample: now of zero, sample at zero.
    assert_open(
        &admit(
            &[reached_at("pg1", 0, 1, false, SYSTEM_ID)],
            0,
            Duration::from_nanos(1),
        ),
        "pg1",
        1,
        1,
    );

    // A second primary seen exactly at the expiry is silence, not conflict;
    // one nanosecond younger it closes.
    let second = |nanos: u64| healthy_with(vec![reached_at("pg2", nanos, 1, false, SYSTEM_ID)]);
    assert_open(&admit(&second(94 * SECOND), now, EXPIRY), "pg1", 1, 1);
    assert_closed(
        &admit(&second(94 * SECOND + 1), now, EXPIRY),
        &["evidence.second_primary"],
        true,
    );
    assert_open(&admit(&second(now + 1), now, EXPIRY), "pg1", 1, 1);
}

#[test]
fn ac11_bootstrap_uses_the_same_freshness_rule() {
    let config = Setup::default().config();
    let plan = |observations: &[Observation]| {
        plan_bootstrap(&config, &StoreProbe::Absent, observations, now(), EXPIRY)
    };
    let with_pg3 = |nanos: u64| healthy_with(vec![reached_at("pg3", nanos, 1, true, SYSTEM_ID)]);
    assert!(plan(&with_pg3(94 * SECOND + 1)).is_ok());
    assert!(plan(&with_pg3(100 * SECOND)).is_ok());
    assert_refused(
        &config,
        &StoreProbe::Absent,
        &with_pg3(94 * SECOND),
        &["bootstrap.evidence_missing"],
    );
    assert_refused(
        &config,
        &StoreProbe::Absent,
        &with_pg3(100 * SECOND + 1),
        &["bootstrap.evidence_missing"],
    );
    // A stale second primary is missing evidence, not a second primary.
    assert_refused(
        &config,
        &StoreProbe::Absent,
        &healthy_with(vec![reached("pg2", 94, 1, false, SYSTEM_ID)]),
        &["bootstrap.evidence_missing"],
    );
    // A tie at the latest time is missing evidence too.
    let mut tied = healthy();
    tied.push(replica("pg3", 1));
    assert_refused(
        &config,
        &StoreProbe::Absent,
        &tied,
        &["bootstrap.evidence_missing"],
    );
    // A failed probe of the writer is missing evidence.
    assert_refused(
        &config,
        &StoreProbe::Absent,
        &healthy_with(vec![failed("pg1")]),
        &["bootstrap.evidence_missing"],
    );
}

#[test]
fn ac11_the_latest_sample_decides_even_when_it_is_from_the_future() {
    let config = witness(X).config();
    let state = ready(&config, journal(X));
    // A good sample at 98 seconds and a later one at 101 seconds: the later
    // one is the latest and it is not fresh, so the writer has no evidence.
    let mut observations = healthy();
    observations.push(reached("pg1", 101, 1, false, SYSTEM_ID));
    assert_closed(
        &admission(&config, &state, &observations),
        &["evidence.writer_missing"],
        false,
    );
    // The same for another node: a future sample hides its older conflict.
    let mut observations = healthy_with(vec![primary("pg2", 1)]);
    observations.push(reached("pg2", 101, 1, true, SYSTEM_ID));
    assert_open(&admission(&config, &state, &observations), "pg1", 1, 1);
}

#[test]
fn ac11_a_tie_hides_evidence_only_at_the_latest_time() {
    let config = witness(X).config();
    let state = ready(&config, journal(X));
    // Two samples of pg1 tie at 97 seconds, one sample at 98 is the latest.
    let observations = vec![
        reached("pg1", 97, 1, true, SYSTEM_ID),
        reached("pg1", 97, 2, false, SYSTEM_ID),
        primary("pg1", 1),
        replica("pg2", 1),
        replica("pg3", 1),
    ];
    assert_open(&admission(&config, &state, &observations), "pg1", 1, 1);
    // The latest sample listed first, then older ones and a tie among them.
    let observations = vec![
        primary("pg1", 1),
        reached("pg1", 97, 1, true, SYSTEM_ID),
        reached("pg1", 97, 1, true, SYSTEM_ID),
        reached("pg1", 50, 3, true, SYSTEM_ID),
        replica("pg2", 1),
        replica("pg3", 1),
    ];
    assert_open(&admission(&config, &state, &observations), "pg1", 1, 1);
    // A tie at 98 for pg1 even after a later, then an earlier sample order.
    let observations = vec![
        reached("pg1", 50, 1, false, SYSTEM_ID),
        primary("pg1", 1),
        reached("pg1", 60, 1, false, SYSTEM_ID),
        primary("pg1", 1),
        replica("pg2", 1),
    ];
    assert_closed(
        &admission(&config, &state, &observations),
        &["evidence.writer_missing"],
        false,
    );
    // Two samples of pg2 tie at the latest time, one of them a primary: pg2
    // has no evidence, and silence about a replica does not close.
    let mut observations = healthy();
    observations.push(primary("pg2", 2));
    assert_open(&admission(&config, &state, &observations), "pg1", 1, 1);
    // A failed latest sample after a good one is no evidence.
    let observations = vec![
        reached("pg1", 97, 1, false, SYSTEM_ID),
        failed("pg1"),
        replica("pg2", 1),
    ];
    assert_closed(
        &admission(&config, &state, &observations),
        &["evidence.writer_missing"],
        false,
    );
}

#[test]
fn ac9_a_configured_node_outside_the_journal_members_still_counts() {
    // The configuration lists pg4 for main; the journal does not have it as a
    // member. The functions look at every node the configuration lists.
    let setup = Setup {
        witness: Some(X),
        nodes: &["pg1", "pg2", "pg3", "pg4"],
        ..Setup::default()
    };
    let config = setup.config();
    let state = ready(&config, journal(X));
    assert_open(&admission(&config, &state, &healthy()), "pg1", 1, 1);
    let mut observations = healthy();
    observations.push(primary("pg4", 1));
    assert_closed(
        &admission(&config, &state, &observations),
        &["evidence.second_primary"],
        true,
    );
    let mut observations = healthy();
    observations.push(replica("pg4", 2));
    assert_closed(
        &admission(&config, &state, &observations),
        &["evidence.timeline_ahead"],
        true,
    );
    let mut observations = healthy();
    observations.push(replica("pg4", 1));
    assert_open(&admission(&config, &state, &observations), "pg1", 1, 1);
}

#[test]
fn ac9_a_second_cluster_does_not_affect_the_first() {
    let config = two_clusters(Some(X));
    let state = ready(&config, two_journal(X));
    // Both clusters healthy: each opens on its own writer.
    let mut observations = healthy();
    observations.extend([primary("pg4", 1), replica("pg5", 1)]);
    assert_open(&admission(&config, &state, &observations), "pg1", 1, 1);
    match side_admission(&config, &state, &observations) {
        WriterAdmission::Open(primary) => {
            assert_eq!(primary.cluster(), &side_cluster());
            assert_eq!(primary.node(), &node("pg4"));
        }
        WriterAdmission::Closed(closed) => panic!("side closed: {closed:?}"),
    }

    // The primary of side, and a side replica on a later timeline, share the
    // system identifier of main and still do not close main.
    let mut observations = healthy();
    observations.extend([primary("pg4", 1), replica("pg5", 3)]);
    assert_open(&admission(&config, &state, &observations), "pg1", 1, 1);
    assert_closed(
        &side_admission(&config, &state, &observations),
        &["evidence.timeline_ahead"],
        true,
    );

    // A latch on side does not close main, and a latch on main does not
    // close side.
    let mut latched = two_journal(X);
    latched.clusters[1].conflict_latched = true;
    let state = ready(&config, latched);
    let mut observations = healthy();
    observations.extend([primary("pg4", 1), replica("pg5", 1)]);
    assert_open(&admission(&config, &state, &observations), "pg1", 1, 1);
    assert_closed(
        &side_admission(&config, &state, &observations),
        &["authority.conflict_latched"],
        false,
    );
    let mut latched = two_journal(X);
    latched.clusters[0].conflict_latched = true;
    let state = ready(&config, latched);
    assert!(matches!(
        side_admission(&config, &state, &observations),
        WriterAdmission::Open(_)
    ));

    // A quarantine row of side names nodes of main: it is not a quarantine
    // of main, so pg1 is not quarantined there and pg2 still conflicts.
    let mut quarantined = two_journal(X);
    quarantined.quarantine = vec![
        Quarantine {
            cluster: side_cluster(),
            node: node("pg1"),
        },
        Quarantine {
            cluster: side_cluster(),
            node: node("pg2"),
        },
    ];
    let state = ready(&config, quarantined);
    let mut observations = healthy();
    observations.extend([primary("pg4", 1), replica("pg5", 1)]);
    assert_open(&admission(&config, &state, &observations), "pg1", 1, 1);
    let mut observations = healthy_with(vec![primary("pg2", 1)]);
    observations.extend([primary("pg4", 1), replica("pg5", 1)]);
    assert_closed(
        &admission(&config, &state, &observations),
        &["evidence.second_primary"],
        true,
    );
}

#[test]
fn ac9_cluster_unknown_is_reported_alone() {
    // The configuration has side, the journal does not; the journal carries
    // a latch and a quarantined writer for main that must not show up.
    let config = two_clusters(Some(X));
    let mut stored = journal(X);
    stored.clusters[0].conflict_latched = true;
    stored.quarantine.push(Quarantine {
        cluster: main_cluster(),
        node: node("pg1"),
    });
    let state = ready(&config, stored);
    assert_closed(
        &side_admission(&config, &state, &[primary("pg4", 1), primary("pg5", 2)]),
        &["authority.cluster_unknown"],
        false,
    );
    // The journal has side, the configuration does not.
    let config = witness(X).config();
    let state = ready(&config, two_journal(X));
    assert_closed(
        &side_admission(&config, &state, &[primary("pg4", 1)]),
        &["authority.cluster_unknown"],
        false,
    );
    // main is still judged on its own rules.
    assert_open(&admission(&config, &state, &healthy()), "pg1", 1, 1);
    // A cluster that neither names.
    let other = ClusterId::parse("other").expect("cluster id");
    match writer_admission(&config, &state, &other, &healthy(), now(), EXPIRY) {
        WriterAdmission::Closed(closed) => assert_eq!(
            closed
                .reasons()
                .iter()
                .map(|r| r.as_str())
                .collect::<Vec<_>>(),
            ["authority.cluster_unknown"]
        ),
        WriterAdmission::Open(primary) => panic!("open for an unknown cluster: {primary:?}"),
    }
}

#[test]
fn ac9_every_ready_reason_is_reported_once_in_the_listed_order() {
    // Checks 2, 3, 4, 5, 9 and 10 at once.
    let config = Setup {
        witness: Some(X),
        expected_writer: "pg2",
        ..Setup::default()
    }
    .config();
    let mut stored = journal(X);
    stored.clusters[0].conflict_latched = true;
    stored.quarantine.push(Quarantine {
        cluster: main_cluster(),
        node: node("pg1"),
    });
    let state = ready(&config, stored);
    assert_closed(
        &admission(&config, &state, &[primary("pg2", 1), replica("pg3", 2)]),
        &[
            "authority.config_conflict",
            "authority.conflict_latched",
            "authority.writer_quarantined",
            "evidence.writer_missing",
            "evidence.second_primary",
            "evidence.timeline_ahead",
        ],
        true,
    );

    // Checks 7 and 8 together, in that order.
    let config = witness(X).config();
    let state = ready(&config, journal(X));
    assert_closed(
        &admission(&config, &state, &healthy_with(vec![replica("pg1", 2)])),
        &["evidence.writer_in_recovery", "evidence.writer_timeline"],
        false,
    );

    // Two nodes cause the same two reasons; each is listed once.
    assert_closed(
        &admission(
            &config,
            &state,
            &healthy_with(vec![primary("pg2", 2), primary("pg3", 2)]),
        ),
        &["evidence.second_primary", "evidence.timeline_ahead"],
        true,
    );

    // All three config conflicts at once still give one reason.
    let config = Setup {
        witness: Some(X),
        expected_writer: "pg2",
        system_identifier: OTHER_SYSTEM_ID,
        nodes: &["pg1", "pg2"],
    }
    .config();
    let state = ready(&config, journal(X));
    assert_closed(
        &admission(&config, &state, &healthy()),
        &["authority.config_conflict"],
        false,
    );
}

#[test]
fn ac9_a_writer_with_another_identity_skips_the_recovery_and_timeline_checks() {
    let config = witness(X).config();
    let state = ready(&config, journal(X));
    assert_closed(
        &admission(
            &config,
            &state,
            &healthy_with(vec![reached("pg1", 98, 2, true, OTHER_SYSTEM_ID)]),
        ),
        &["evidence.writer_identity"],
        false,
    );
    // Checks 9 and 10 still run after check 6.
    assert_closed(
        &admission(
            &config,
            &state,
            &healthy_with(vec![
                reached("pg1", 98, 2, true, OTHER_SYSTEM_ID),
                primary("pg2", 2),
            ]),
        ),
        &[
            "evidence.writer_identity",
            "evidence.second_primary",
            "evidence.timeline_ahead",
        ],
        true,
    );
    // A missing writer also keeps checks 9 and 10.
    assert_closed(
        &admission(&config, &state, &[primary("pg2", 1), replica("pg3", 1)]),
        &["evidence.writer_missing", "evidence.second_primary"],
        true,
    );
}

#[test]
fn ac9_other_nodes_with_another_system_identifier_are_not_a_conflict() {
    let config = witness(X).config();
    let state = ready(&config, journal(X));
    assert_open(
        &admission(
            &config,
            &state,
            &healthy_with(vec![
                reached("pg2", 98, 3, false, OTHER_SYSTEM_ID),
                reached("pg3", 98, 4, true, OTHER_SYSTEM_ID),
            ]),
        ),
        "pg1",
        1,
        1,
    );
}

#[test]
fn ac9_a_quarantined_node_never_closes_and_never_opens() {
    let config = witness(X).config();
    let mut stored = journal(X);
    stored.quarantine.push(Quarantine {
        cluster: main_cluster(),
        node: node("pg3"),
    });
    let state = ready(&config, stored);
    for pg3 in [replica("pg3", 2), primary("pg3", 2), primary("pg3", 1)] {
        assert_open(
            &admission(&config, &state, &healthy_with(vec![pg3])),
            "pg1",
            1,
            1,
        );
    }
    // With the writer gone, the quarantined primary is not adopted.
    assert_closed(
        &admission(&config, &state, &[replica("pg2", 1), primary("pg3", 2)]),
        &["evidence.writer_missing"],
        false,
    );
}

#[test]
fn ac9_timelines_compare_with_the_journal_not_with_each_other() {
    let config = witness(X).config();
    let mut stored = journal(X);
    stored.clusters[0].writer_timeline = timeline(3);
    let state = ready(&config, stored);
    // Replicas behind the journal timeline are no conflict.
    assert_open(
        &admission(
            &config,
            &state,
            &[primary("pg1", 3), replica("pg2", 2), replica("pg3", 1)],
        ),
        "pg1",
        1,
        1,
    );
    // The writer on a higher or a lower timeline than the journal closes.
    for tl in [2, 4] {
        assert_closed(
            &admission(
                &config,
                &state,
                &[primary("pg1", tl), replica("pg2", 3), replica("pg3", 3)],
            ),
            &["evidence.writer_timeline"],
            false,
        );
    }
    // A replica one above the journal is ahead, also when the writer itself
    // reports that higher timeline.
    assert_closed(
        &admission(
            &config,
            &state,
            &[primary("pg1", 4), replica("pg2", 4), replica("pg3", 3)],
        ),
        &["evidence.writer_timeline", "evidence.timeline_ahead"],
        true,
    );
    // At the largest timeline nothing wraps.
    let mut stored = journal(X);
    stored.clusters[0].writer_timeline = timeline(u32::MAX);
    let state = ready(&config, stored);
    assert_open(
        &admission(
            &config,
            &state,
            &[
                primary("pg1", u32::MAX),
                replica("pg2", u32::MAX),
                replica("pg3", 1),
            ],
        ),
        "pg1",
        1,
        1,
    );
}

#[test]
fn ac7_ac8_ac10_start_classification_edges() {
    let no_witness = Setup::default().config();
    let with_x = witness(X).config();

    // Unreadable without a witness is still Unreadable, and bootstrap is
    // refused for the store alone.
    let state = classify_startup(&no_witness, StoreProbe::Unreadable);
    assert_eq!(state, AuthorityState::Unreadable);
    assert_closed(
        &admission(&no_witness, &state, &healthy()),
        &["authority.unreadable"],
        false,
    );
    assert_refused(
        &no_witness,
        &StoreProbe::Unreadable,
        &healthy(),
        &["bootstrap.store_present"],
    );

    // A held journal without a witness is Unconfirmed, not Held.
    let mut held = journal(X);
    held.hold = Hold::RestorePending;
    assert_eq!(
        classify_startup(&no_witness, StoreProbe::Present(held.clone())),
        AuthorityState::Unconfirmed
    );
    // A held journal of another identity is a Mismatch.
    assert_eq!(
        classify_startup(&witness(Y).config(), StoreProbe::Present(held.clone())),
        AuthorityState::Mismatch
    );
    // Held stays closed with one reason, whatever else the journal says.
    held.clusters[0].conflict_latched = true;
    held.quarantine.push(Quarantine {
        cluster: main_cluster(),
        node: node("pg1"),
    });
    let state = classify_startup(&with_x, StoreProbe::Present(held));
    assert_eq!(state, AuthorityState::Held);
    assert_closed(
        &admission(&with_x, &state, &[]),
        &["authority.restore_pending"],
        false,
    );

    // A witness that differs by one bit is a Mismatch.
    let near_x = "0123456789abcdef0123456789abcdee";
    assert_eq!(
        classify_startup(&witness(near_x).config(), StoreProbe::Present(journal(X))),
        AuthorityState::Mismatch
    );

    // Ready carries the stored generations.
    let mut stored = journal(X);
    stored.authority_revision = AuthorityRevision::from_stored(42).expect("revision");
    stored.start_sequence = StartSequence::from_stored(7).expect("start sequence");
    match classify_startup(&with_x, StoreProbe::Present(stored)) {
        AuthorityState::Ready(authority) => {
            assert_eq!(authority.authority_id().to_string(), X);
            assert_eq!(authority.authority_revision().get(), 42);
            assert_eq!(authority.start_sequence().get(), 7);
        }
        other => panic!("expected Ready, got {other:?}"),
    }

    // A journal with no cluster at all is consistent; admission is closed
    // for main because the journal has no topology for it.
    let mut empty = journal(X);
    empty.clusters.clear();
    let state = ready(&with_x, empty);
    assert_closed(
        &admission(&with_x, &state, &healthy()),
        &["authority.cluster_unknown"],
        false,
    );
}

#[test]
fn ac7_ac8_bootstrap_reasons_combine_in_the_listed_order() {
    // Witness and missing evidence.
    assert_refused(
        &witness(X).config(),
        &StoreProbe::Absent,
        &healthy_without("pg3"),
        &["bootstrap.witness_present", "bootstrap.evidence_missing"],
    );
    // A store and a second primary.
    assert_refused(
        &Setup::default().config(),
        &StoreProbe::Present(journal(X)),
        &healthy_with(vec![primary("pg2", 1)]),
        &["bootstrap.store_present", "bootstrap.second_primary"],
    );
    // A witness and no cluster.
    let text = format!("config_version = 1\n\n[control]\nauthority_id = \"{X}\"\n");
    let lone_witness = pool_config::validate(text.as_bytes()).expect("valid");
    assert_refused(
        &lone_witness,
        &StoreProbe::Absent,
        &[],
        &["bootstrap.witness_present", "bootstrap.no_cluster"],
    );
    // An unreadable store and no cluster.
    let empty = pool_config::validate(b"config_version = 1\n").expect("valid");
    assert_refused(
        &empty,
        &StoreProbe::Unreadable,
        &healthy(),
        &["bootstrap.store_present", "bootstrap.no_cluster"],
    );
    // Seven reasons at once, in order: pg1 in recovery, pg2 on another
    // system, pg3 unseen, pg4 a primary on timeline 2.
    let config = Setup {
        witness: Some(X),
        nodes: &["pg1", "pg2", "pg3", "pg4"],
        ..Setup::default()
    }
    .config();
    assert_refused(
        &config,
        &StoreProbe::Present(journal(X)),
        &[
            replica("pg1", 1),
            reached("pg2", 98, 1, true, OTHER_SYSTEM_ID),
            primary("pg4", 2),
        ],
        &[
            "bootstrap.store_present",
            "bootstrap.witness_present",
            "bootstrap.evidence_missing",
            "bootstrap.identity_mismatch",
            "bootstrap.writer_in_recovery",
            "bootstrap.second_primary",
            "bootstrap.timeline_ahead",
        ],
    );
}

#[test]
fn ac7_bootstrap_checks_six_to_eight_look_only_at_fitting_nodes() {
    let config = Setup::default().config();
    // The writer reports another system identifier and is in recovery: only
    // the identity is reported, and check 8 is skipped for the cluster.
    assert_refused(
        &config,
        &StoreProbe::Absent,
        &[
            reached("pg1", 98, 1, true, OTHER_SYSTEM_ID),
            replica("pg2", 5),
            replica("pg3", 1),
        ],
        &["bootstrap.identity_mismatch"],
    );
    // Check 7 still applies to a fitting node.
    assert_refused(
        &config,
        &StoreProbe::Absent,
        &[
            reached("pg1", 98, 1, true, OTHER_SYSTEM_ID),
            primary("pg2", 5),
            replica("pg3", 1),
        ],
        &["bootstrap.identity_mismatch", "bootstrap.second_primary"],
    );
    // A node on another system that claims to be a primary on a later
    // timeline gives the identity finding only.
    assert_refused(
        &config,
        &StoreProbe::Absent,
        &healthy_with(vec![reached("pg2", 98, 9, false, OTHER_SYSTEM_ID)]),
        &["bootstrap.identity_mismatch"],
    );
    // A node behind the writer's timeline is fine; the plan takes the
    // writer's timeline.
    let plan = plan_bootstrap(
        &config,
        &StoreProbe::Absent,
        &[primary("pg1", 3), replica("pg2", 1), replica("pg3", 2)],
        now(),
        EXPIRY,
    )
    .expect("a plan");
    assert_eq!(plan.clusters()[0].writer_timeline, timeline(3));
}

#[test]
fn ac7_bootstrap_judges_each_cluster_on_its_own_and_ignores_unconfigured_nodes() {
    let config = two_clusters(None);
    let main_on_3 = vec![primary("pg1", 3), replica("pg2", 3), replica("pg3", 3)];
    let mut observations = main_on_3.clone();
    observations.extend([primary("pg4", 1), replica("pg5", 1), primary("pg9", 7)]);
    let plan =
        plan_bootstrap(&config, &StoreProbe::Absent, &observations, now(), EXPIRY).expect("a plan");
    assert_eq!(
        plan_names(&plan),
        [
            ("main", "pg1", 3, vec!["pg1", "pg2", "pg3"]),
            ("side", "pg4", 1, vec!["pg4", "pg5"]),
        ]
    );

    // One node of the second cluster missing refuses the whole plan.
    let mut observations = main_on_3.clone();
    observations.push(primary("pg4", 1));
    assert_refused(
        &config,
        &StoreProbe::Absent,
        &observations,
        &["bootstrap.evidence_missing"],
    );

    // A later timeline is judged against the writer of the node's own
    // cluster: pg5 on 2 is ahead of pg4 on 1, though behind pg1 on 3.
    let mut observations = main_on_3;
    observations.extend([primary("pg4", 1), replica("pg5", 2)]);
    assert_refused(
        &config,
        &StoreProbe::Absent,
        &observations,
        &["bootstrap.timeline_ahead"],
    );
}

// Spec 0003 AC-16 (build plan step 9): a Ready state stays tied to the
// witness of the configuration it was classified from.

/// Calls admission with `config` for the healthy observations, for a second
/// primary, and for a cluster the configuration does not have, and expects
/// the one reason `reason` each time, without the latch.
#[track_caller]
fn assert_only_reason(config: &ValidatedConfig, state: &AuthorityState, reason: &str) {
    for observations in [healthy(), healthy_with(vec![primary("pg2", 1)])] {
        assert_closed(&admission(config, state, &observations), &[reason], false);
    }
    let other = ClusterId::parse("other").expect("cluster id");
    match writer_admission(
        config,
        state,
        &other,
        &healthy_with(vec![primary("pg2", 1)]),
        now(),
        EXPIRY,
    ) {
        WriterAdmission::Closed(closed) => {
            let names: Vec<&str> = closed.reasons().iter().map(|r| r.as_str()).collect();
            assert_eq!(names, [reason]);
            assert!(!closed.latches_conflict());
        }
        WriterAdmission::Open(primary) => panic!("expected closed, got open {primary:?}"),
    }
}

#[track_caller]
fn assert_unchanged_with_its_own_configuration(config: &ValidatedConfig, state: &AuthorityState) {
    assert_open(&admission(config, state, &healthy()), "pg1", 1, 1);
    assert_closed(
        &admission(config, state, &healthy_with(vec![primary("pg2", 1)])),
        &["evidence.second_primary"],
        true,
    );
}

#[test]
fn ac16_a_ready_state_with_a_configuration_without_witness_is_unconfirmed() {
    // covers: 0003 AC-16
    let classified_from = witness(X).config();
    let state = ready(&classified_from, journal(X));
    assert_only_reason(&Setup::default().config(), &state, "authority.unconfirmed");
    assert_unchanged_with_its_own_configuration(&classified_from, &state);
}

#[test]
fn ac16_a_ready_state_with_a_configuration_of_another_witness_is_a_mismatch() {
    // covers: 0003 AC-16
    let classified_from = witness(X).config();
    let state = ready(&classified_from, journal(X));
    assert_only_reason(&witness(Y).config(), &state, "authority.mismatch");
    // A witness one hexadecimal digit off is another witness too.
    assert_only_reason(
        &witness("0123456789abcdef0123456789abcdee").config(),
        &state,
        "authority.mismatch",
    );
    assert_unchanged_with_its_own_configuration(&classified_from, &state);
}

#[test]
fn ac16_the_witness_check_comes_before_every_other_reason() {
    // covers: 0003 AC-16. A configuration that differs in more than the
    // witness still gives the one witness reason; with the right witness the
    // other differences give their own reasons as before.
    let classified_from = witness(X).config();
    let mut stored = journal(X);
    stored.clusters[0].conflict_latched = true;
    stored.quarantine.push(Quarantine {
        cluster: main_cluster(),
        node: node("pg1"),
    });
    let state = ready(&classified_from, stored);
    let differs = |witness: Option<&str>| {
        Setup {
            witness,
            expected_writer: "pg2",
            system_identifier: OTHER_SYSTEM_ID,
            nodes: &["pg1", "pg2"],
        }
        .config()
    };
    assert_only_reason(&differs(None), &state, "authority.unconfirmed");
    assert_only_reason(&differs(Some(Y)), &state, "authority.mismatch");
    assert_closed(
        &admission(&differs(Some(X)), &state, &[primary("pg2", 1)]),
        &[
            "authority.config_conflict",
            "authority.conflict_latched",
            "authority.writer_quarantined",
            "evidence.writer_missing",
            "evidence.second_primary",
        ],
        true,
    );
}

// Walk 3 from row d to row o through the real functions (`0003 AC-17`):
// `writer_admission`, `AuthorityState::latch_conflict`, the journal write
// and `AuthorityState::latch_recorded`. The state of row a is the one of
// the walk; no journal value is built by hand for row o. The receipt comes
// from a real journal file that `Journal::create` wrote from the plan of
// walk 1, with the same identity, revision and start sequence.

/// A journal file of the walk: `main`, writer `pg1`, ID `X`, opened for
/// writing at a `Ready` start of `state`.
fn walk_journal(name: &str, state: &AuthorityState) -> (std::path::PathBuf, pool_control::Journal) {
    let folder = std::path::PathBuf::from(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("pool-control-walk-{name}"));
    if folder.exists() {
        std::fs::remove_dir_all(&folder).expect("remove old test folder");
    }
    std::fs::create_dir_all(&folder).expect("create test folder");
    let path = folder.join("journal.sqlite3");
    std::fs::File::create_new(&path).expect("create the empty journal file");
    let plan = plan_bootstrap(
        &Setup::default().config(),
        &StoreProbe::Absent,
        &healthy(),
        now(),
        EXPIRY,
    )
    .expect("the plan of walk 1");
    pool_control::Journal::create(&path, &plan, AuthorityId::parse(X).expect("id"), 0)
        .expect("create");
    let AuthorityState::Ready(authority) = state else {
        panic!("expected ready, got {}", state.name());
    };
    let (journal, _) = pool_control::Journal::open_ready(&path, authority).expect("open");
    (path, journal)
}

fn revision_of(state: &AuthorityState) -> u64 {
    match state {
        AuthorityState::Ready(authority) => authority.authority_revision().get(),
        other => panic!("expected ready, got {}", other.name()),
    }
}

#[test]
fn ac17_walk_from_row_d_to_row_o_through_the_real_functions() {
    // covers: 0003 AC-17. Once with the receipt of the journal, once with
    // the write failing between `latch_conflict` and `latch_recorded`.
    for write_succeeds in [true, false] {
        let config = witness(X).config();
        let mut state = classify_startup(&config, StoreProbe::Present(journal(X)));
        let (path, mut journal_file) = walk_journal(&format!("d-to-o-{write_succeeds}"), &state);

        // Row a.
        assert_open(&admission(&config, &state, &healthy()), "pg1", 1, 1);

        // Row d: `pg2` not in recovery. The answer asks for the latch.
        let row_d = healthy_with(vec![primary("pg2", 1)]);
        let answer = admission(&config, &state, &row_d);
        assert_closed(&answer, &["evidence.second_primary"], true);
        let WriterAdmission::Closed(closed) = &answer else {
            panic!("row d is closed");
        };
        assert!(state.latch_conflict(&main_cluster(), closed));
        // The latch closes in memory at once, before any write.
        assert_closed(
            &admission(&config, &state, &row_d),
            &["authority.conflict_latched", "evidence.second_primary"],
            true,
        );

        if write_succeeds {
            let receipt = journal_file
                .set_conflict_latch(&main_cluster(), 0)
                .expect("write the latch");
            assert_eq!(receipt.authority_revision().get(), 2);
            state.latch_recorded(&receipt);
            assert_eq!(revision_of(&state), 2);
        } else {
            let holder = rusqlite::Connection::open(&path).expect("open");
            holder
                .execute_batch("BEGIN IMMEDIATE;")
                .expect("hold the lock");
            assert_eq!(
                journal_file.set_conflict_latch(&main_cluster(), 0),
                Err(pool_control::JournalError::Busy)
            );
            holder.execute_batch("ROLLBACK;").expect("release");
            assert_eq!(revision_of(&state), 1);
        }

        // Row o: `pg2` drops out of sight. The answer is row o, not row a.
        for observations in [healthy_without("pg2"), healthy()] {
            assert_closed(
                &admission(&config, &state, &observations),
                &["authority.conflict_latched"],
                false,
            );
        }
    }
}

#[test]
fn ac17_latch_conflict_changes_nothing_unless_a_ready_state_is_asked_to() {
    // covers: 0003 AC-17.
    let config = witness(X).config();
    let mut state = classify_startup(&config, StoreProbe::Present(journal(X)));
    let row_d = healthy_with(vec![primary("pg2", 1)]);
    let WriterAdmission::Closed(latching) = admission(&config, &state, &row_d) else {
        panic!("row d is closed");
    };
    let WriterAdmission::Closed(silent) = admission(&config, &state, &healthy_without("pg1"))
    else {
        panic!("row f is closed");
    };
    assert!(!silent.latches_conflict());

    // An answer that does not ask for the latch.
    assert!(!state.latch_conflict(&main_cluster(), &silent));
    assert_open(&admission(&config, &state, &healthy()), "pg1", 1, 1);
    // A cluster the state does not hold.
    let other = ClusterId::parse("other").expect("cluster id");
    assert!(!state.latch_conflict(&other, &latching));
    assert_open(&admission(&config, &state, &healthy()), "pg1", 1, 1);
    // A state that is not `Ready`.
    for mut other_state in [
        AuthorityState::Uninitialized,
        AuthorityState::Unconfirmed,
        AuthorityState::Missing,
        AuthorityState::Unreadable,
        AuthorityState::Mismatch,
        AuthorityState::Held,
    ] {
        let before = other_state.clone();
        assert!(!other_state.latch_conflict(&main_cluster(), &latching));
        assert_eq!(other_state, before);
    }
    // Once set, the latch stays set, also for an answer that does not ask.
    assert!(state.latch_conflict(&main_cluster(), &latching));
    assert!(state.latch_conflict(&main_cluster(), &silent));
    assert_closed(
        &admission(&config, &state, &healthy()),
        &["authority.conflict_latched"],
        false,
    );
}
