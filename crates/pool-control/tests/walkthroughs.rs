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
    assert!(
        BootstrapRefusal::NoCluster
            .as_str()
            .starts_with("bootstrap.")
    );
}
