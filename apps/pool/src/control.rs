//! The control loop of the daemon (spec 0004, "Control loop and topology
//! snapshot").
//!
//! One task owns the `Controller` of `pool-control`. It receives samples
//! from the probe tasks over a bounded channel, runs a round after each
//! sample and at least once per `probe.period_ms`, maps the snapshot into
//! the route view of `pool-core` and publishes it at once. Only then does it
//! write each new conflict latch to the journal, off the workers of the
//! runtime. `pool-core` never sees a type of `pool-control`: the mapping
//! lives here.

use std::collections::HashMap;
use std::sync::Arc;

use pool_config::{ClusterId, ValidatedConfig};
use pool_control::{
    ClosedReason, Controller, Journal, LatchReceipt, MonotonicTime, Observation, TopologySnapshot,
    WriterAdmission,
};
use pool_core::route::{Route, RouteState, RouteView};
use tokio::sync::{mpsc, watch};
use tokio::time::Instant;

/// The reasons that end the running sessions of a cluster (spec 0004, "When
/// a running session is ended"). Silence alone does not.
fn is_conflict(reason: ClosedReason) -> bool {
    matches!(
        reason,
        ClosedReason::WriterIdentity
            | ClosedReason::WriterInRecovery
            | ClosedReason::WriterTimeline
            | ClosedReason::SecondPrimary
            | ClosedReason::TimelineAhead
            | ClosedReason::ConflictLatched
    )
}

/// The route view of a snapshot: an open answer with its freshness becomes
/// an open route, every other answer a closed one, with the conflict flag.
pub fn route_view(snapshot: &TopologySnapshot) -> RouteView {
    let routes = snapshot
        .clusters()
        .iter()
        .map(|cluster| {
            let state = match (cluster.admission(), cluster.fresh_until()) {
                (WriterAdmission::Open(primary), Some(fresh_until)) => RouteState::Open(Route {
                    node: primary.node().clone(),
                    topology_epoch: primary.topology_epoch(),
                    authority_revision: primary.authority_revision(),
                    fresh_until: fresh_until.since_origin(),
                }),
                (WriterAdmission::Open(_), None) => RouteState::Closed { conflict: false },
                (WriterAdmission::Closed(closed), _) => RouteState::Closed {
                    conflict: closed.reasons().iter().any(|reason| is_conflict(*reason)),
                },
            };
            (cluster.cluster().clone(), state)
        })
        .collect();
    RouteView::new(routes)
}

/// What a round said about one cluster, to tell when it changed.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Seen {
    Open(String),
    Closed(String),
}

/// The control loop: its controller, the journal for latch writes, and the
/// sender of the route view.
pub struct ControlLoop {
    config: Arc<ValidatedConfig>,
    controller: Controller,
    journal: Option<Journal>,
    routes: watch::Sender<RouteView>,
    origin: Instant,
    seen: HashMap<ClusterId, Seen>,
    unrecorded: Vec<ClusterId>,
}

impl ControlLoop {
    /// A loop for the state classified at start, with an empty board.
    pub fn new(
        config: Arc<ValidatedConfig>,
        controller: Controller,
        journal: Journal,
        routes: watch::Sender<RouteView>,
        origin: Instant,
    ) -> ControlLoop {
        ControlLoop {
            config,
            controller,
            journal: Some(journal),
            routes,
            origin,
            seen: HashMap::new(),
            unrecorded: Vec::new(),
        }
    }

    /// Runs rounds until `stop` changes: after each sample, and at least
    /// once per `probe.period_ms`. Then tries once more to write every latch
    /// the journal does not hold yet, and returns the journal, to be closed.
    pub async fn run(
        mut self,
        mut samples: mpsc::Receiver<Observation>,
        mut stop: watch::Receiver<bool>,
    ) -> Option<Journal> {
        let mut tick = tokio::time::interval(self.config.probe().period());
        tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
        loop {
            tokio::select! {
                biased;
                _ = stop.changed() => break,
                sample = samples.recv() => match sample {
                    Some(observation) => self.controller.record(observation),
                    None => break,
                },
                _ = tick.tick() => {}
            }
            self.round().await;
        }
        self.last_try().await;
        self.journal
    }

    /// Step 5 of shutdown: one more write for every latch that is still
    /// open, and `authority.conflict_latched` with `recorded` false for each
    /// one that still cannot be written, so that a second primary the
    /// process saw is not forgotten at exit without a line (spec 0004,
    /// decision 96).
    async fn last_try(&mut self) {
        for cluster in self.controller.unrecorded_latches().to_vec() {
            if !self.record_latch(cluster.clone()).await {
                tracing::warn!(
                    event = "authority.conflict_latched",
                    cluster_id = cluster.as_str(),
                    recorded = false
                );
            }
        }
    }

    /// One round: in memory first, published at once, then the disk.
    async fn round(&mut self) {
        let now = MonotonicTime::from_origin(self.origin.elapsed());
        let snapshot = self.controller.round(&self.config, now);
        self.routes.send_replace(route_view(&snapshot));
        self.log_changes(&snapshot);
        for cluster in self.controller.unrecorded_latches() {
            if !self.unrecorded.contains(cluster) {
                tracing::warn!(
                    event = "authority.conflict_latched",
                    cluster_id = cluster.as_str(),
                    recorded = false
                );
                self.unrecorded.push(cluster.clone());
            }
        }
        for cluster in self.controller.unrecorded_latches().to_vec() {
            self.record_latch(cluster).await;
        }
    }

    /// Writes one latch and says so. Returns whether the journal holds it.
    async fn record_latch(&mut self, cluster: ClusterId) -> bool {
        match self.write_latch(cluster).await {
            Ok(receipt) => {
                tracing::warn!(
                    event = "authority.conflict_latched",
                    cluster_id = receipt.cluster().as_str(),
                    recorded = true
                );
                self.unrecorded.retain(|held| held != receipt.cluster());
                self.controller.latch_recorded(&receipt);
                true
            }
            Err(class) => {
                tracing::error!(event = "journal.write_failed", class);
                false
            }
        }
    }

    /// Writes one latch on a blocking thread, so no worker waits for the disk.
    async fn write_latch(&mut self, cluster: ClusterId) -> Result<LatchReceipt, &'static str> {
        let Some(mut journal) = self.journal.take() else {
            return Err("journal.open");
        };
        let changed = crate::commands::unix_ms();
        let task = tokio::task::spawn_blocking(move || {
            let written = journal.set_conflict_latch(&cluster, changed);
            (journal, written)
        });
        match task.await {
            Ok((journal, written)) => {
                self.journal = Some(journal);
                written.map_err(|error| error.as_str())
            }
            // The blocking task did not come back: the journal is gone for
            // this run, and every later write fails the same way.
            Err(_) => Err("journal.write"),
        }
    }

    /// Writes `admission.changed` for each cluster whose state or reasons
    /// differ from the last round.
    fn log_changes(&mut self, snapshot: &TopologySnapshot) {
        let generation = snapshot.generation().get();
        for cluster in snapshot.clusters() {
            let seen = match cluster.admission() {
                WriterAdmission::Open(primary) => Seen::Open(primary.node().as_str().to_owned()),
                WriterAdmission::Closed(closed) => Seen::Closed(
                    closed
                        .reasons()
                        .iter()
                        .map(|reason| reason.as_str())
                        .collect::<Vec<_>>()
                        .join(","),
                ),
            };
            if self.seen.get(cluster.cluster()) == Some(&seen) {
                continue;
            }
            match &seen {
                Seen::Open(node) => tracing::info!(
                    event = "admission.changed",
                    cluster_id = cluster.cluster().as_str(),
                    state = "open",
                    node_id = node.as_str(),
                    snapshot_generation = generation
                ),
                Seen::Closed(reasons) => tracing::info!(
                    event = "admission.changed",
                    cluster_id = cluster.cluster().as_str(),
                    state = "closed",
                    reasons = reasons.as_str(),
                    snapshot_generation = generation
                ),
            }
            self.seen.insert(cluster.cluster().clone(), seen);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_conflict_reasons_are_the_six_of_the_spec() {
        let conflict: Vec<&str> = [
            ClosedReason::Uninitialized,
            ClosedReason::Unconfirmed,
            ClosedReason::Missing,
            ClosedReason::Unreadable,
            ClosedReason::Mismatch,
            ClosedReason::RestorePending,
            ClosedReason::ClusterUnknown,
            ClosedReason::ConfigConflict,
            ClosedReason::ConflictLatched,
            ClosedReason::WriterQuarantined,
            ClosedReason::WriterMissing,
            ClosedReason::WriterIdentity,
            ClosedReason::WriterInRecovery,
            ClosedReason::WriterTimeline,
            ClosedReason::SecondPrimary,
            ClosedReason::TimelineAhead,
        ]
        .into_iter()
        .filter(|reason| is_conflict(*reason))
        .map(ClosedReason::as_str)
        .collect();
        assert_eq!(
            conflict,
            [
                "authority.conflict_latched",
                "evidence.writer_identity",
                "evidence.writer_in_recovery",
                "evidence.writer_timeline",
                "evidence.second_primary",
                "evidence.timeline_ahead",
            ]
        );
    }

    // ---- added by the tests of scope 004, round two (`/test one real primary path`) ----
    //
    // The control loop of the daemon on a real journal under the temporary
    // folder, with a `Controller` of `pool-control` and samples made by the
    // test. Two nodes in one cluster: `pg1` the writer, `pg2` a standby, or a
    // second primary when the test says so.

    use std::path::{Path, PathBuf};
    use std::sync::Mutex;
    use std::time::Duration;

    use pool_config::{AuthorityId, NodeId, SystemIdentifier};
    use pool_control::{
        AuthorityState, NodeFacts, ObservationBoard, ObservationOutcome, TimelineId,
    };

    const X: &str = "0123456789abcdef0123456789abcdef";
    const SYSTEM_ID: &str = "7412345678901234567";
    const EXPIRY: Duration = Duration::from_millis(1000);

    fn config(witness: bool) -> ValidatedConfig {
        let witness = if witness {
            format!("authority_id = \"{X}\"\n")
        } else {
            String::new()
        };
        let text = format!(
            "config_version = 1\n\n[control]\nstate_directory = \"/var/lib/pool\"\n{witness}\n\
             [probe]\nperiod_ms = 100\ntimeout_ms = 100\nobservation_expiry_ms = 1000\n\n\
             [[cluster]]\nid = \"main\"\nsystem_identifier = \"{SYSTEM_ID}\"\nexpected_writer = \"pg1\"\n\n\
             [[node]]\nid = \"pg1\"\ncluster = \"main\"\nhost = \"10.0.0.11\"\nport = 5432\n\n\
             [[node]]\nid = \"pg2\"\ncluster = \"main\"\nhost = \"10.0.0.12\"\nport = 5432\n"
        );
        pool_config::validate(text.as_bytes()).expect("a valid test configuration")
    }

    fn reached(node: &str, at: MonotonicTime, in_recovery: bool) -> Observation {
        Observation::new(
            NodeId::parse(node).expect("node id"),
            at,
            ObservationOutcome::Reached(NodeFacts {
                system_identifier: SystemIdentifier::parse(SYSTEM_ID).expect("system identifier"),
                timeline: TimelineId::from_observed(1).expect("timeline"),
                in_recovery,
            }),
        )
    }

    /// A folder of its own under the temporary folder, with a journal made the
    /// way `pool bootstrap` makes it.
    fn journal_file(name: &str) -> PathBuf {
        let folder =
            std::env::temp_dir().join(format!("pool-control-loop-{}-{name}", std::process::id()));
        let _ = std::fs::remove_dir_all(&folder);
        std::fs::create_dir_all(&folder).expect("folder");
        let path = folder.join("journal.sqlite3");
        std::fs::write(&path, b"").expect("empty journal");
        let now = MonotonicTime::from_origin(Duration::from_secs(100));
        let mut board = ObservationBoard::new();
        board.record(reached("pg1", now, false));
        board.record(reached("pg2", now, true));
        let plan = Journal::read(&path)
            .bootstrap_plan(&config(false), &board, now)
            .expect("bootstrap is allowed");
        Journal::create(
            &path,
            &plan,
            AuthorityId::parse(X).expect("id"),
            1_763_200_000_000,
        )
        .expect("create the journal");
        path
    }

    fn ready(path: &Path) -> AuthorityState {
        let state = Journal::read(path).classify(&config(true));
        assert_eq!(state.name(), "ready");
        state
    }

    /// A control loop on the journal at `path`, opened for writing, and the receiver of its view.
    fn control_loop(path: &Path) -> (ControlLoop, watch::Receiver<RouteView>) {
        let state = ready(path);
        let AuthorityState::Ready(authority) = &state else {
            unreachable!("checked above");
        };
        let (journal, _) = Journal::open_ready(path, authority).expect("open for writing");
        let config = Arc::new(config(true));
        let ids: Vec<ClusterId> = config.clusters().iter().map(|c| c.id().clone()).collect();
        let (routes, receiver) = watch::channel(RouteView::closed(&ids));
        let control = ControlLoop::new(
            config,
            Controller::new(state.clone()),
            journal,
            routes,
            Instant::now(),
        );
        (control, receiver)
    }

    impl ControlLoop {
        fn now(&self) -> MonotonicTime {
            MonotonicTime::from_origin(self.origin.elapsed())
        }

        /// Records one sample of each node, taken now.
        fn sample(&mut self, pg2_is_primary: bool) {
            let now = self.now();
            self.controller.record(reached("pg1", now, false));
            self.controller.record(reached("pg2", now, !pg2_is_primary));
        }
    }

    fn main_cluster() -> ClusterId {
        ClusterId::parse("main").expect("cluster id")
    }

    type Events = Arc<Mutex<Vec<Vec<(String, String)>>>>;

    /// Keeps every event as its fields, in order.
    struct Capture(Events);

    struct Fields(Vec<(String, String)>);

    impl tracing::field::Visit for Fields {
        fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
            self.0.push((field.name().to_owned(), value.to_owned()));
        }

        fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
            self.0.push((field.name().to_owned(), format!("{value:?}")));
        }
    }

    impl tracing::Subscriber for Capture {
        fn enabled(&self, _: &tracing::Metadata<'_>) -> bool {
            true
        }

        fn new_span(&self, _: &tracing::span::Attributes<'_>) -> tracing::span::Id {
            tracing::span::Id::from_u64(1)
        }

        fn record(&self, _: &tracing::span::Id, _: &tracing::span::Record<'_>) {}

        fn record_follows_from(&self, _: &tracing::span::Id, _: &tracing::span::Id) {}

        fn event(&self, event: &tracing::Event<'_>) {
            let mut fields = Fields(Vec::new());
            event.record(&mut fields);
            self.0.lock().expect("lock").push(fields.0);
        }

        fn enter(&self, _: &tracing::span::Id) {}

        fn exit(&self, _: &tracing::span::Id) {}
    }

    fn capture() -> (Events, tracing::subscriber::DefaultGuard) {
        let events: Events = Arc::default();
        let guard = tracing::subscriber::set_default(Capture(Arc::clone(&events)));
        (events, guard)
    }

    /// The events named `name`, each as its fields after `event`.
    fn named(events: &Events, name: &str) -> Vec<Vec<(String, String)>> {
        events
            .lock()
            .expect("lock")
            .iter()
            .filter(|e| e.first().is_some_and(|(k, v)| k == "event" && v == name))
            .map(|e| e[1..].to_vec())
            .collect()
    }

    fn pairs(list: &[(&str, &str)]) -> Vec<(String, String)> {
        list.iter()
            .map(|(k, v)| ((*k).to_owned(), (*v).to_owned()))
            .collect()
    }

    #[test]
    fn ac14_route_view_marks_a_positive_conflict_and_not_silence() {
        // covers: 0004 AC-14, "When a running session is ended": a second
        // primary closes with the conflict flag, a writer with no fresh
        // sample closes without it.
        let path = journal_file("route-view-conflict");
        let config = config(true);
        let mut controller = Controller::new(ready(&path));
        let at = MonotonicTime::from_origin(Duration::from_secs(10));
        controller.record(reached("pg1", at, false));
        controller.record(reached("pg2", at, false));
        let snapshot = controller.round(&config, at);
        assert_eq!(
            route_view(&snapshot).state(&main_cluster()),
            RouteState::Closed { conflict: true }
        );
        let mut silent = Controller::new(ready(&path));
        let later = MonotonicTime::from_origin(Duration::from_secs(20));
        let snapshot = silent.round(&config, later);
        assert_eq!(
            route_view(&snapshot).state(&main_cluster()),
            RouteState::Closed { conflict: false }
        );
    }

    #[test]
    fn ac14_an_open_route_is_fresh_exactly_until_its_snapshot_says() {
        // covers: 0004 AC-14: the route view carries the freshness of the
        // snapshot and no more, so a stale route refuses with 57P03.
        let path = journal_file("route-view-fresh");
        let config = config(true);
        let mut controller = Controller::new(ready(&path));
        let at = MonotonicTime::from_origin(Duration::from_secs(10));
        controller.record(reached("pg1", at, false));
        controller.record(reached("pg2", at, true));
        let snapshot = controller.round(&config, at);
        let fresh_until = snapshot.clusters()[0]
            .fresh_until()
            .expect("an open answer is fresh until a time")
            .since_origin();
        assert!(
            fresh_until <= Duration::from_secs(10) + EXPIRY,
            "{fresh_until:?}"
        );
        match route_view(&snapshot).state(&main_cluster()) {
            RouteState::Open(route) => {
                assert_eq!(route.fresh_until, fresh_until);
                assert_eq!(route.node.as_str(), "pg1");
            }
            other => panic!("open expected: {other:?}"),
        }
        let view = route_view(&snapshot);
        assert!(view.fresh_route(&main_cluster(), fresh_until).is_none());
    }

    #[tokio::test]
    async fn ac14_the_conflict_view_is_published_before_the_latch_is_written() {
        // covers: 0004 AC-14 and decision 12: in memory first, published at
        // once, then the disk. The round is polled once: the latch write is
        // still on its blocking thread, and the view is already out.
        let path = journal_file("published-first");
        let (mut control, receiver) = control_loop(&path);
        control.sample(true);
        {
            // Exactly one poll: a timer would give the blocking write time to finish.
            let mut round = std::pin::pin!(control.round());
            let mut context = std::task::Context::from_waker(std::task::Waker::noop());
            assert!(
                round.as_mut().poll(&mut context).is_pending(),
                "the round waits for its latch write"
            );
        }
        assert_eq!(
            receiver.borrow().state(&main_cluster()),
            RouteState::Closed { conflict: true }
        );
    }

    #[tokio::test]
    async fn ac14_a_second_primary_latches_the_cluster_in_the_journal_through_the_control_loop() {
        // covers: 0004 AC-14, 0003 AC-17: the latch reaches the disk, its two
        // log lines are written, and a later start finds the cluster latched
        // although both nodes look healthy again.
        let (events, _guard) = capture();
        let path = journal_file("latch-written");
        let (mut control, receiver) = control_loop(&path);
        control.sample(true);
        control.round().await;
        assert_eq!(
            receiver.borrow().state(&main_cluster()),
            RouteState::Closed { conflict: true }
        );
        assert!(control.controller.unrecorded_latches().is_empty());
        assert_eq!(
            named(&events, "authority.conflict_latched"),
            [
                pairs(&[("cluster_id", "main"), ("recorded", "false")]),
                pairs(&[("cluster_id", "main"), ("recorded", "true")]),
            ]
        );
        assert!(named(&events, "journal.write_failed").is_empty());
        drop(control);
        // A new start reads the latch back from the file.
        let config = config(true);
        let mut later = Controller::new(ready(&path));
        let at = MonotonicTime::from_origin(Duration::from_secs(10));
        later.record(reached("pg1", at, false));
        later.record(reached("pg2", at, true));
        let snapshot = later.round(&config, at);
        match snapshot.clusters()[0].admission() {
            WriterAdmission::Closed(closed) => {
                assert!(
                    closed.reasons().contains(&ClosedReason::ConflictLatched),
                    "{:?}",
                    closed.reasons()
                );
            }
            WriterAdmission::Open(_) => panic!("the latch did not reach the journal"),
        }
    }

    #[tokio::test]
    async fn ac14_a_latch_write_that_fails_is_logged_and_the_latch_holds_in_memory() {
        // covers: 0004 AC-14, 0003 AC-17: the journal changed under the
        // daemon (another holder advanced its revision), so every write is refused. The
        // cluster stays closed with the conflict flag in later rounds, the
        // unrecorded line is written once, and each failed write is logged.
        let (events, _guard) = capture();
        let path = journal_file("latch-fails");
        let (mut control, receiver) = control_loop(&path);
        let AuthorityState::Ready(authority) = ready(&path) else {
            unreachable!("checked in ready");
        };
        // Another holder of the journal moves the authority revision on, so the
        // revision the daemon holds is no longer the one on disk.
        let (mut other, _) = Journal::open_ready(&path, &authority).expect("a second start");
        other
            .set_conflict_latch(&main_cluster(), 1_763_200_000_001)
            .expect("the other holder writes");
        control.sample(true);
        control.round().await;
        control.sample(false);
        control.round().await;
        assert_eq!(
            receiver.borrow().state(&main_cluster()),
            RouteState::Closed { conflict: true }
        );
        assert_eq!(control.controller.unrecorded_latches(), [main_cluster()]);
        assert_eq!(
            named(&events, "authority.conflict_latched"),
            [pairs(&[("cluster_id", "main"), ("recorded", "false")])]
        );
        assert_eq!(
            named(&events, "journal.write_failed"),
            [
                pairs(&[("class", "journal.changed")]),
                pairs(&[("class", "journal.changed")]),
            ]
        );
    }

    #[tokio::test]
    async fn c9_a_latch_still_open_at_shutdown_is_tried_once_more_and_said_so() {
        // covers: spec 0004 decision 96 (finding c9 of the independent
        // review): when the loop stops, every latch that the journal does
        // not hold is written once more, and one that still fails gets
        // `authority.conflict_latched` with `recorded` false.
        let (events, _guard) = capture();
        let path = journal_file("latch-last-try");
        let (mut control, _receiver) = control_loop(&path);
        let AuthorityState::Ready(authority) = ready(&path) else {
            unreachable!("checked in ready");
        };
        let (mut other, _) = Journal::open_ready(&path, &authority).expect("a second start");
        other
            .set_conflict_latch(&main_cluster(), 1_763_200_000_001)
            .expect("the other holder writes");
        control.sample(true);
        control.round().await;
        control.last_try().await;
        assert_eq!(control.controller.unrecorded_latches(), [main_cluster()]);
        let unrecorded = pairs(&[("cluster_id", "main"), ("recorded", "false")]);
        assert_eq!(
            named(&events, "authority.conflict_latched"),
            [unrecorded.clone(), unrecorded]
        );
        assert_eq!(named(&events, "journal.write_failed").len(), 2);
    }
}
