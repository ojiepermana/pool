//! Bootstrap: the one way a journal comes into being.

use std::time::Duration;

use pool_config::{TopologyEpoch, ValidatedConfig};

use crate::authority::{ApprovedTopology, StoreProbe};
use crate::observation::{Evidence, MonotonicTime, NodeFacts, Observation};

/// One reason why a bootstrap is refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BootstrapRefusal {
    /// `bootstrap.store_present`: the journal reader found something.
    StorePresent,
    /// `bootstrap.witness_present`: the configuration carries a witness, so a
    /// journal existed once.
    WitnessPresent,
    /// `bootstrap.no_cluster`: the configuration has no cluster.
    NoCluster,
    /// `bootstrap.evidence_missing`: some node has no fresh observation that
    /// reached it.
    EvidenceMissing,
    /// `bootstrap.identity_mismatch`: some node reports another system
    /// identifier than its cluster configures.
    IdentityMismatch,
    /// `bootstrap.writer_in_recovery`: some `expected_writer` is in recovery.
    WriterInRecovery,
    /// `bootstrap.second_primary`: a node other than the `expected_writer`
    /// reports that it is a primary.
    SecondPrimary,
    /// `bootstrap.timeline_ahead`: a node is on a later timeline than the
    /// `expected_writer` of its cluster.
    TimelineAhead,
}

impl BootstrapRefusal {
    /// The reason name.
    pub fn as_str(self) -> &'static str {
        match self {
            BootstrapRefusal::StorePresent => "bootstrap.store_present",
            BootstrapRefusal::WitnessPresent => "bootstrap.witness_present",
            BootstrapRefusal::NoCluster => "bootstrap.no_cluster",
            BootstrapRefusal::EvidenceMissing => "bootstrap.evidence_missing",
            BootstrapRefusal::IdentityMismatch => "bootstrap.identity_mismatch",
            BootstrapRefusal::WriterInRecovery => "bootstrap.writer_in_recovery",
            BootstrapRefusal::SecondPrimary => "bootstrap.second_primary",
            BootstrapRefusal::TimelineAhead => "bootstrap.timeline_ahead",
        }
    }
}

/// What a bootstrap may write: one approved topology per configured cluster.
///
/// Only [`plan_bootstrap`] makes one. Code outside this crate cannot build
/// one:
///
/// ```compile_fail,E0451
/// fn build(clusters: Vec<pool_control::ApprovedTopology>) -> pool_control::BootstrapPlan {
///     pool_control::BootstrapPlan { clusters }
/// }
/// ```
///
/// The same paths through the public functions compile, so the snippet
/// above can fail only because the field is private (the code after the
/// comma is not checked by the stable compiler):
///
/// ```
/// fn read(plan: &pool_control::BootstrapPlan) -> Vec<pool_control::ApprovedTopology> {
///     plan.clusters().to_vec()
/// }
/// ```
///
/// Restart rule: it lives in memory for one run of the bootstrap command and
/// is gone after it. What survives is the journal the command writes from it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapPlan {
    clusters: Vec<ApprovedTopology>,
}

impl BootstrapPlan {
    /// The approved topology of each configured cluster, in the order of the
    /// configuration.
    pub fn clusters(&self) -> &[ApprovedTopology] {
        &self.clusters
    }
}

/// A refused bootstrap and its reasons, never empty, each at most once, in
/// the order of the rules.
///
/// Restart rule: it lives in memory for one run of the bootstrap command and
/// is gone after it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootstrapRefused {
    reasons: Vec<BootstrapRefusal>,
}

impl BootstrapRefused {
    /// The reasons, in the order of the rules.
    pub fn reasons(&self) -> &[BootstrapRefusal] {
        &self.reasons
    }
}

/// Decides whether the bootstrap command may write a journal, for every
/// configured cluster at once.
///
/// It needs no journal, no witness, at least one cluster, and a fresh
/// observation that reached every configured node and fits the
/// configuration: the `expected_writer` of each cluster is a primary, every
/// other node is in recovery and no node is on a later timeline than the
/// writer. Every reason that applies is reported.
pub fn plan_bootstrap(
    config: &ValidatedConfig,
    probe: &StoreProbe,
    observations: &[Observation],
    now: MonotonicTime,
    expiry: Duration,
) -> Result<BootstrapPlan, BootstrapRefused> {
    let evidence = Evidence::new(observations, now, expiry);
    let store_present = !matches!(probe, StoreProbe::Absent);
    let witness_present = config
        .control()
        .and_then(|control| control.authority_id())
        .is_some();
    let no_cluster = config.clusters().is_empty();
    let mut evidence_missing = false;
    let mut identity_mismatch = false;
    let mut writer_in_recovery = false;
    let mut second_primary = false;
    let mut timeline_ahead = false;
    let mut clusters = Vec::new();

    for cluster in config.clusters() {
        let system_identifier = cluster.system_identifier();
        let writer = cluster.expected_writer();
        let nodes: Vec<_> = config
            .nodes()
            .iter()
            .filter(|node| node.cluster() == cluster.id())
            .collect();

        // Facts of the nodes whose system identifier is the configured one.
        let mut fitting: Vec<(&pool_config::NodeId, NodeFacts)> = Vec::new();
        for node in &nodes {
            match evidence.facts(node.id()) {
                None => evidence_missing = true,
                Some(facts) if facts.system_identifier != system_identifier => {
                    identity_mismatch = true;
                }
                Some(facts) => fitting.push((node.id(), facts)),
            }
        }

        let writer_facts = fitting
            .iter()
            .find(|(node, _)| *node == writer)
            .map(|(_, facts)| *facts);
        for (node, facts) in &fitting {
            if *node == writer {
                writer_in_recovery |= facts.in_recovery;
            } else {
                second_primary |= !facts.in_recovery;
            }
            if let Some(writer_facts) = writer_facts {
                timeline_ahead |= facts.timeline > writer_facts.timeline;
            }
        }

        if let Some(writer_facts) = writer_facts {
            clusters.push(ApprovedTopology {
                cluster: cluster.id().clone(),
                system_identifier,
                approved_writer: writer.clone(),
                writer_timeline: writer_facts.timeline,
                topology_epoch: TopologyEpoch::FIRST,
                members: nodes.iter().map(|node| node.id().clone()).collect(),
                conflict_latched: false,
            });
        }
    }

    let reasons: Vec<BootstrapRefusal> = [
        (store_present, BootstrapRefusal::StorePresent),
        (witness_present, BootstrapRefusal::WitnessPresent),
        (no_cluster, BootstrapRefusal::NoCluster),
        (evidence_missing, BootstrapRefusal::EvidenceMissing),
        (identity_mismatch, BootstrapRefusal::IdentityMismatch),
        (writer_in_recovery, BootstrapRefusal::WriterInRecovery),
        (second_primary, BootstrapRefusal::SecondPrimary),
        (timeline_ahead, BootstrapRefusal::TimelineAhead),
    ]
    .into_iter()
    .filter_map(|(applies, reason)| applies.then_some(reason))
    .collect();

    if reasons.is_empty() {
        Ok(BootstrapPlan { clusters })
    } else {
        Err(BootstrapRefused { reasons })
    }
}
