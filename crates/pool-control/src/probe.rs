//! The probe query and the reading of its one row.
//!
//! The connection, the TLS check and the login belong to `pool-core`; the
//! control loop of the daemon calls them. This module only holds the text of
//! the question and turns the answer into an [`Observation`], so both are
//! tested without a database. The contract is spec 0004, section "The probe".

use pool_config::{NodeId, SystemIdentifier};

use crate::observation::{MonotonicTime, NodeFacts, Observation, ObservationOutcome, TimelineId};

/// The one question the probe asks a node, sent as one simple `Query`
/// message. It returns one row of three text columns: the system identifier,
/// whether the node is in recovery, and the timeline.
///
/// On a node that is not in recovery the timeline is the one of the current
/// write position, taken from the name of its WAL file, so it changes at the
/// moment of a promotion. On a node in recovery it is the greatest timeline
/// that plain SQL shows. Every function it calls can be executed by a role
/// with `LOGIN` and nothing else (`SEC-5`).
pub const PROBE_QUERY: &str = "SELECT (SELECT system_identifier::text FROM pg_control_system()),
       pg_is_in_recovery()::text,
       (CASE WHEN pg_is_in_recovery() THEN
          GREATEST((SELECT timeline_id FROM pg_control_checkpoint()),
                   (SELECT min_recovery_end_timeline FROM pg_control_recovery()),
                   COALESCE((SELECT received_tli FROM pg_stat_wal_receiver), 0))
        ELSE
          ('x' || substr(pg_walfile_name(pg_current_wal_insert_lsn()), 1, 8))::bit(32)::int
        END)::text";

/// The largest answer, in bytes of column text, that a probe accepts.
const MAX_ANSWER_BYTES: usize = 4096;

/// Turns the answer of [`PROBE_QUERY`] into an observation of `node`, sampled
/// at `sampled_at` (the monotonic clock of the daemon, read when the row
/// arrived).
///
/// `row` is `None` when the probe did not get exactly one row: no answer
/// before the deadline, an error response, a failed login or TLS check, or
/// more than one row. That, a row with another number of columns than three,
/// a null, text that does not parse, a timeline of 0, or more than 4096 bytes
/// of text gives [`ObservationOutcome::Failed`]. A failed sample is no
/// evidence of anything.
pub fn observation_from_row(
    node: NodeId,
    sampled_at: MonotonicTime,
    row: Option<&[Option<&str>]>,
) -> Observation {
    let outcome = row
        .and_then(facts_from_row)
        .map_or(ObservationOutcome::Failed, ObservationOutcome::Reached);
    Observation::new(node, sampled_at, outcome)
}

fn facts_from_row(row: &[Option<&str>]) -> Option<NodeFacts> {
    let [Some(system_identifier), Some(in_recovery), Some(timeline)] = row else {
        return None;
    };
    let size = system_identifier.len() + in_recovery.len() + timeline.len();
    if size > MAX_ANSWER_BYTES {
        return None;
    }
    let in_recovery = match *in_recovery {
        "true" => true,
        "false" => false,
        _ => return None,
    };
    Some(NodeFacts {
        system_identifier: SystemIdentifier::parse(system_identifier)?,
        timeline: TimelineId::from_observed(parse_digits(timeline)?)?,
        in_recovery,
    })
}

/// Decimal digits only, as PostgreSQL prints a positive integer: no sign, no
/// space, not empty.
fn parse_digits(text: &str) -> Option<u32> {
    if text.is_empty() || !text.bytes().all(|byte| byte.is_ascii_digit()) {
        return None;
    }
    text.parse().ok()
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    fn read(row: Option<&[Option<&str>]>) -> ObservationOutcome {
        let node = NodeId::parse("pg1").expect("node id");
        let at = MonotonicTime::from_origin(Duration::from_secs(5));
        let observation = observation_from_row(node.clone(), at, row);
        assert_eq!(observation.node(), &node);
        assert_eq!(observation.sampled_at(), at);
        observation.outcome()
    }

    #[test]
    fn a_good_row_gives_its_facts() {
        // covers: 0004 AC-12.
        let outcome = read(Some(&[
            Some("7412345678901234567"),
            Some("false"),
            Some("3"),
        ]));
        assert_eq!(
            outcome,
            ObservationOutcome::Reached(NodeFacts {
                system_identifier: SystemIdentifier::parse("7412345678901234567")
                    .expect("system identifier"),
                timeline: TimelineId::from_observed(3).expect("timeline"),
                in_recovery: false,
            })
        );
        let outcome = read(Some(&[Some("1"), Some("true"), Some("4294967295")]));
        assert!(matches!(
            outcome,
            ObservationOutcome::Reached(NodeFacts {
                in_recovery: true,
                ..
            })
        ));
    }

    #[test]
    fn every_other_answer_is_a_failed_sample() {
        // covers: 0004 AC-12.
        let long = "1".repeat(4090);
        let rows: [&[Option<&str>]; 16] = [
            &[],
            &[Some("1"), Some("false")],
            &[Some("1"), Some("false"), Some("1"), Some("1")],
            &[None, Some("false"), Some("1")],
            &[Some("1"), None, Some("1")],
            &[Some("1"), Some("false"), None],
            &[Some("0"), Some("false"), Some("1")],
            &[Some("x"), Some("false"), Some("1")],
            &[Some("1"), Some("f"), Some("1")],
            &[Some("1"), Some("TRUE"), Some("1")],
            &[Some("1"), Some("false"), Some("0")],
            &[Some("1"), Some("false"), Some("-1")],
            &[Some("1"), Some("false"), Some("+1")],
            &[Some("1"), Some("false"), Some(" 1")],
            &[Some("1"), Some("false"), Some("4294967296")],
            &[Some("1"), Some("false"), Some(&long)],
        ];
        for row in rows {
            assert_eq!(read(Some(row)), ObservationOutcome::Failed, "{row:?}");
        }
        assert_eq!(read(None), ObservationOutcome::Failed);
    }

    #[test]
    fn the_query_asks_the_three_columns_of_spec_0004() {
        assert!(PROBE_QUERY.starts_with("SELECT (SELECT system_identifier::text"));
        assert!(PROBE_QUERY.contains("pg_is_in_recovery()::text"));
        assert!(PROBE_QUERY.contains("pg_current_wal_insert_lsn()"));
        assert!(PROBE_QUERY.contains("received_tli FROM pg_stat_wal_receiver"));
        assert!(PROBE_QUERY.ends_with("END)::text"));
        assert!(!PROBE_QUERY.contains(';'));
    }
}
