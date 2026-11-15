//! The route view: what the data plane reads about each cluster (spec 0004, "Route view").
//!
//! `pool-core` owns the type; the control loop in `apps/pool` fills it from
//! a topology snapshot and publishes it on a `tokio::sync::watch` channel.
//! A cluster that the view does not hold is closed. A route is fresh only
//! until its `fresh_until`, so a control loop that stops publishing closes
//! every route by itself.

use std::collections::HashMap;
use std::time::Duration;

use pool_config::{AuthorityRevision, ClusterId, NodeId, TopologyEpoch};

/// The open route of one cluster: the eligible primary and the generations it was admitted with.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Route {
    /// The node that takes the sessions of the cluster.
    pub node: NodeId,
    /// The topology epoch of the admission.
    pub topology_epoch: TopologyEpoch,
    /// The authority revision of the admission.
    pub authority_revision: AuthorityRevision,
    /// The time, since the origin the daemon passed at start, after which the route is stale.
    pub fresh_until: Duration,
}

/// The state of one cluster in the view.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouteState {
    /// New sessions may go to the route.
    Open(Route),
    /// New sessions are refused. `conflict` is true for a positive conflict,
    /// which also ends the running sessions of the cluster.
    Closed {
        /// Whether the reasons hold one of the names under "When a running session is ended".
        conflict: bool,
    },
}

/// The routes of every cluster, as one immutable value.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RouteView {
    routes: HashMap<ClusterId, RouteState>,
}

impl RouteView {
    /// A view in which every listed cluster is closed without a conflict.
    pub fn closed(clusters: &[ClusterId]) -> RouteView {
        RouteView {
            routes: clusters
                .iter()
                .map(|c| (c.clone(), RouteState::Closed { conflict: false }))
                .collect(),
        }
    }

    /// A view with these routes. A cluster listed twice keeps its last state.
    pub fn new(routes: Vec<(ClusterId, RouteState)>) -> RouteView {
        RouteView {
            routes: routes.into_iter().collect(),
        }
    }

    /// The state of a cluster; a cluster that the view does not hold is closed.
    pub fn state(&self, cluster: &ClusterId) -> RouteState {
        self.routes
            .get(cluster)
            .cloned()
            .unwrap_or(RouteState::Closed { conflict: false })
    }

    /// The route of a cluster when it is open and fresh at `now` (time since the origin).
    pub fn fresh_route(&self, cluster: &ClusterId, now: Duration) -> Option<Route> {
        match self.routes.get(cluster) {
            Some(RouteState::Open(route)) if now < route.fresh_until => Some(route.clone()),
            _ => None,
        }
    }

    /// Whether the cluster is closed with a positive conflict.
    pub fn is_conflict(&self, cluster: &ClusterId) -> bool {
        matches!(
            self.routes.get(cluster),
            Some(RouteState::Closed { conflict: true })
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ids() -> (ClusterId, NodeId) {
        let config = pool_config::validate(
            b"config_version = 1\n[control]\nstate_directory = \"/var/lib/pool\"\n[[cluster]]\nid = \"main\"\nsystem_identifier = \"1\"\nexpected_writer = \"pg1\"\n[[node]]\nid = \"pg1\"\ncluster = \"main\"\nhost = \"h\"\nport = 5432\n",
        )
        .expect("config");
        (
            config.clusters()[0].id().clone(),
            config.nodes()[0].id().clone(),
        )
    }

    #[test]
    fn a_route_is_open_only_while_fresh_and_an_unknown_cluster_is_closed() {
        let (cluster, node) = ids();
        let route = Route {
            node,
            topology_epoch: TopologyEpoch::FIRST,
            authority_revision: AuthorityRevision::FIRST,
            fresh_until: Duration::from_secs(10),
        };
        let view = RouteView::new(vec![(cluster.clone(), RouteState::Open(route.clone()))]);
        assert_eq!(
            view.fresh_route(&cluster, Duration::from_secs(9)),
            Some(route)
        );
        assert_eq!(view.fresh_route(&cluster, Duration::from_secs(10)), None);
        assert_eq!(
            RouteView::default().state(&cluster),
            RouteState::Closed { conflict: false }
        );
        assert!(!RouteView::closed(std::slice::from_ref(&cluster)).is_conflict(&cluster));
        let conflict = RouteView::new(vec![(
            cluster.clone(),
            RouteState::Closed { conflict: true },
        )]);
        assert!(conflict.is_conflict(&cluster));
        assert_eq!(conflict.fresh_route(&cluster, Duration::ZERO), None);
    }
}
