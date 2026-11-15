//! The three durable generation types.
//!
//! A generation offers three things: read the number, compare, and advance by
//! one. It has no other arithmetic. Advancing at the largest value returns
//! `None`, and the caller fails closed. The rule is owned by spec 0003,
//! section "Identifier and generation types".

use std::fmt;

/// The largest value a generation holds: 9223372036854775807, the largest
/// whole number SQLite stores.
const MAX_GENERATION: u64 = 9_223_372_036_854_775_807;

macro_rules! generation {
    ($(#[$doc:meta])* $name:ident, first = $first:expr) => {
        $(#[$doc])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct $name(u64);

        impl $name {
            /// The value a new journal starts with.
            pub const FIRST: $name = $name($first);

            /// Builds the generation from a stored number. Returns `None` for
            /// a value below [`Self::FIRST`] or above 9223372036854775807.
            pub fn from_stored(value: u64) -> Option<$name> {
                ($first..=MAX_GENERATION).contains(&value).then_some($name(value))
            }

            /// The number.
            pub fn get(self) -> u64 {
                self.0
            }

            /// The next generation, larger by one. Returns `None` at
            /// 9223372036854775807; it never wraps.
            pub fn checked_next(self) -> Option<$name> {
                (self.0 < MAX_GENERATION).then(|| $name(self.0 + 1))
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

generation! {
    /// The topology epoch of one cluster. It starts at 1 at bootstrap and
    /// advances when the approved writer of that cluster changes.
    ///
    /// Restart rule: it survives a restart, in the journal.
    ///
    /// It compares only with another `TopologyEpoch`:
    ///
    /// ```compile_fail
    /// fn same(epoch: pool_config::TopologyEpoch, revision: pool_config::AuthorityRevision) -> bool {
    ///     epoch == revision
    /// }
    /// ```
    TopologyEpoch, first = 1
}

generation! {
    /// The authority revision of one journal. It starts at 1 at bootstrap and
    /// advances with every transaction that changes durable authority. Every
    /// mutation of durable authority names the revision it expects.
    ///
    /// Restart rule: it survives a restart, in the journal.
    AuthorityRevision, first = 1
}

generation! {
    /// The start sequence of one journal. It starts at 0 at bootstrap and
    /// advances at each start of the daemon that finds its journal ready.
    /// A memory generation is comparable only inside one start sequence.
    ///
    /// Restart rule: it survives a restart, in the journal.
    StartSequence, first = 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_values() {
        assert_eq!(TopologyEpoch::FIRST.get(), 1);
        assert_eq!(AuthorityRevision::FIRST.get(), 1);
        assert_eq!(StartSequence::FIRST.get(), 0);
    }

    #[test]
    fn from_stored_bounds() {
        assert_eq!(TopologyEpoch::from_stored(0), None);
        assert_eq!(AuthorityRevision::from_stored(0), None);
        assert_eq!(StartSequence::from_stored(0), Some(StartSequence::FIRST));
        assert_eq!(TopologyEpoch::from_stored(1), Some(TopologyEpoch::FIRST));
        assert_eq!(
            TopologyEpoch::from_stored(MAX_GENERATION).map(TopologyEpoch::get),
            Some(MAX_GENERATION)
        );
        assert_eq!(TopologyEpoch::from_stored(MAX_GENERATION + 1), None);
        assert_eq!(AuthorityRevision::from_stored(u64::MAX), None);
        assert_eq!(StartSequence::from_stored(MAX_GENERATION + 1), None);
    }

    #[test]
    fn checked_next_advances_by_one_and_stops_at_the_top() {
        assert_eq!(
            TopologyEpoch::FIRST.checked_next().map(TopologyEpoch::get),
            Some(2)
        );
        assert_eq!(
            StartSequence::FIRST.checked_next().map(StartSequence::get),
            Some(1)
        );
        let below_top = AuthorityRevision::from_stored(MAX_GENERATION - 1);
        let top = below_top.and_then(AuthorityRevision::checked_next);
        assert_eq!(top.map(AuthorityRevision::get), Some(MAX_GENERATION));
        assert_eq!(top.and_then(AuthorityRevision::checked_next), None);
        assert_eq!(
            TopologyEpoch::from_stored(MAX_GENERATION).and_then(TopologyEpoch::checked_next),
            None
        );
        assert_eq!(
            StartSequence::from_stored(MAX_GENERATION).and_then(StartSequence::checked_next),
            None
        );
    }

    #[test]
    fn generations_compare_and_display() {
        let one = TopologyEpoch::FIRST;
        let two = one.checked_next();
        assert!(Some(one) < two);
        assert_eq!(two, TopologyEpoch::from_stored(2));
        assert_eq!(one.to_string(), "1");
        assert_eq!(StartSequence::FIRST.to_string(), "0");
    }
}

#[cfg(test)]
mod edge_tests {
    //! Edges added by /test (0003 AC-6): every generation type stops at
    //! 9223372036854775807 and never wraps.

    use super::*;

    const BELOW_TOP: u64 = 9_223_372_036_854_775_806;
    const TOP: u64 = 9_223_372_036_854_775_807;

    macro_rules! assert_edges {
        ($ty:ident) => {{
            let below = $ty::from_stored(BELOW_TOP).expect("below the top");
            let top = $ty::from_stored(TOP).expect("the top");
            assert_eq!(below.get(), BELOW_TOP);
            assert_eq!(top.get(), TOP);
            assert_eq!(below.checked_next(), Some(top));
            assert!(below < top);
            assert_eq!(top.checked_next(), None);
            assert_eq!(top.to_string(), "9223372036854775807");
            assert_eq!(below.to_string(), "9223372036854775806");
            assert_eq!($ty::from_stored(TOP + 1), None);
            assert_eq!($ty::from_stored(u64::MAX), None);
            let mut generation = $ty::FIRST;
            for _ in 0..1000 {
                let next = generation.checked_next().expect("far below the top");
                assert_eq!(next.get(), generation.get() + 1);
                assert!(generation < next);
                generation = next;
            }
            assert_eq!($ty::from_stored($ty::FIRST.get()), Some($ty::FIRST));
        }};
    }

    #[test]
    fn ac6_topology_epoch_stops_exactly_at_the_largest_stored_value() {
        assert_edges!(TopologyEpoch);
    }

    #[test]
    fn ac6_authority_revision_stops_exactly_at_the_largest_stored_value() {
        assert_edges!(AuthorityRevision);
    }

    #[test]
    fn ac6_start_sequence_stops_exactly_at_the_largest_stored_value() {
        assert_edges!(StartSequence);
        assert_eq!(
            StartSequence::from_stored(1).map(StartSequence::get),
            Some(1)
        );
    }
}
