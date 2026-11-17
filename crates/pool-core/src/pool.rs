//! Move-only physical ownership and one short mutex for budgets, FIFO and runtime material.
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex, MutexGuard, PoisonError, Weak};
use std::time::Duration;

use pool_config::{ConfigurationGeneration, CredentialGeneration, IdentityId, Limits};
use tokio::sync::Notify;
use tokio::time::Instant;

use crate::backend::Credential;
use crate::records::{Lease, LeaseGeneration, PoolKey};
use crate::route::{Route, RouteView};

pub(crate) struct Idle<S> {
    pub lease: Lease<S>,
    pub route: Route,
    pub since: Instant,
}
pub(crate) struct State<S> {
    pub credentials: HashMap<IdentityId, (CredentialGeneration, Arc<Credential>)>,
    pub configuration: ConfigurationGeneration,
    pub next_lease: Option<LeaseGeneration>,
    pub running: bool,
    pub idle: Vec<Idle<S>>,
    occupied: HashMap<u64, PoolKey>,
    queue: VecDeque<u64>,
    next_id: Option<u64>,
}
pub(crate) struct Pool<S> {
    state: Mutex<State<S>>,
    pub changed: Notify,
    global_cap: usize,
    node_cap: usize,
    database_cap: usize,
    identity_cap: usize,
    wait_cap: usize,
    idle_timeout: Duration,
    lifetime: Duration,
}
impl<S> Pool<S> {
    pub fn new(credentials: HashMap<IdentityId, Arc<Credential>>, limits: &Limits) -> Arc<Self> {
        Arc::new(Self {
            state: Mutex::new(State {
                credentials: credentials
                    .into_iter()
                    .map(|(id, c)| (id, (CredentialGeneration::FIRST, c)))
                    .collect(),
                configuration: ConfigurationGeneration::FIRST,
                next_lease: Some(LeaseGeneration::FIRST),
                running: true,
                idle: Vec::new(),
                occupied: HashMap::new(),
                queue: VecDeque::new(),
                next_id: Some(1),
            }),
            changed: Notify::new(),
            global_cap: limits.backend_max_count() as usize,
            node_cap: limits.node_backend_max_count() as usize,
            database_cap: limits.database_backend_max_count() as usize,
            identity_cap: limits.identity_backend_max_count() as usize,
            wait_cap: limits.backend_wait_max_count() as usize,
            idle_timeout: limits.backend_idle_timeout(),
            lifetime: limits.backend_lifetime(),
        })
    }
    pub fn lock(&self) -> MutexGuard<'_, State<S>> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }
    pub fn expired(&self, idle: &Idle<S>) -> bool {
        idle.since.elapsed() >= self.idle_timeout
            || idle.lease.created_at.elapsed() >= self.lifetime
    }
    pub fn too_old(&self, lease: &Lease<S>) -> bool {
        lease.created_at.elapsed() >= self.lifetime
    }
    pub fn current(state: &State<S>, key: &PoolKey) -> bool {
        state.running
            && key.configuration_generation == state.configuration
            && state
                .credentials
                .get(&key.identity)
                .is_some_and(|(generation, _)| *generation == key.credential_generation)
    }
    pub fn route_current(
        key: &PoolKey,
        expected: &Route,
        routes: &RouteView,
        now: Duration,
    ) -> bool {
        routes.fresh_route(&key.cluster, now).is_some_and(|r| {
            r.node == key.node
                && r.topology_epoch == key.topology_epoch
                && r.authority_revision == expected.authority_revision
                && r.system_identifier == expected.system_identifier
                && r.timeline == expected.timeline
        })
    }
    pub fn ticket(self: &Arc<Self>) -> Option<Ticket<S>> {
        let mut state = self.lock();
        if !state.running || (!state.queue.is_empty() && state.queue.len() >= self.wait_cap) {
            return None;
        }
        let id = state.next_id?;
        state.next_id = id.checked_add(1);
        state.queue.push_back(id);
        Some(Ticket {
            pool: Arc::clone(self),
            id,
        })
    }
    pub fn retire(&self) {
        let idle = {
            let mut state = self.lock();
            state.running = false;
            std::mem::take(&mut state.idle)
        };
        drop(idle);
        self.changed.notify_waiters();
    }
}

pub(crate) enum Candidate<S> {
    Idle(Box<Idle<S>>),
    New(Permit<S>),
    Wait,
    Retired,
}
pub(crate) struct Ticket<S> {
    pool: Arc<Pool<S>>,
    id: u64,
}
impl<S> Ticket<S> {
    pub fn take(&self, key: &PoolKey) -> Candidate<S> {
        loop {
            let mut evicted = Vec::new();
            let outcome = {
                let mut state = self.pool.lock();
                let candidate = if !Pool::current(&state, key) {
                    Candidate::Retired
                } else if state.queue.front() != Some(&self.id) {
                    Candidate::Wait
                } else if let Some(index) = state
                    .idle
                    .iter()
                    .position(|idle| idle.lease.key() == key && !self.pool.expired(idle))
                {
                    Candidate::Idle(Box::new(state.idle.remove(index)))
                } else {
                    'reserve: {
                        let global = state.occupied.len() >= self.pool.global_cap;
                        let node = state
                            .occupied
                            .values()
                            .filter(|k| k.node == key.node)
                            .count()
                            >= self.pool.node_cap;
                        let database = state
                            .occupied
                            .values()
                            .filter(|k| k.cluster == key.cluster && k.database == key.database)
                            .count()
                            >= self.pool.database_cap;
                        let identity = state
                            .occupied
                            .values()
                            .filter(|k| {
                                k.cluster == key.cluster
                                    && k.database == key.database
                                    && k.login == key.login
                            })
                            .count()
                            >= self.pool.identity_cap;
                        if !global && !node && !database && !identity {
                            state.occupied.insert(self.id, key.clone());
                            break 'reserve Candidate::New(Permit {
                                pool: Arc::downgrade(&self.pool),
                                id: self.id,
                            });
                        }
                        let victim = state
                            .idle
                            .iter()
                            .enumerate()
                            .filter(|(_, idle)| {
                                let k = idle.lease.key();
                                global
                                    || (node && k.node == key.node)
                                    || (database
                                        && k.cluster == key.cluster
                                        && k.database == key.database)
                                    || (identity
                                        && k.cluster == key.cluster
                                        && k.database == key.database
                                        && k.login == key.login)
                            })
                            .min_by_key(|(_, idle)| idle.since)
                            .map(|(i, _)| i);
                        let Some(index) = victim else {
                            break 'reserve Candidate::Wait;
                        };
                        let idle = state.idle.remove(index);
                        evicted.push(idle);
                        // Close outside the pool mutex, then inspect the released counts again.
                        break 'reserve Candidate::Wait;
                    }
                };
                if matches!(candidate, Candidate::New(_) | Candidate::Idle(_)) {
                    // Physical ownership has left the capacity queue. A connecting or
                    // validating owner retains its backend budget, never a waiter slot.
                    state.queue.pop_front();
                    self.pool.changed.notify_waiters();
                }
                candidate
            };
            let removed = !evicted.is_empty();
            drop(evicted);
            if !removed {
                return outcome;
            }
        }
    }
}
impl<S> Drop for Ticket<S> {
    fn drop(&mut self) {
        self.pool.lock().queue.retain(|id| *id != self.id);
        self.pool.changed.notify_waiters();
    }
}
pub(crate) struct Permit<S> {
    pool: Weak<Pool<S>>,
    id: u64,
}
impl<S> Drop for Permit<S> {
    fn drop(&mut self) {
        if let Some(pool) = self.pool.upgrade() {
            pool.lock().occupied.remove(&self.id);
            pool.changed.notify_waiters();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::records::StartupProfile;
    use crate::secret::Password;
    use crate::tls::TlsPolicy;
    use pool_config::{ClusterId, NodeId, TopologyEpoch};

    fn key() -> PoolKey {
        PoolKey {
            cluster: ClusterId::parse("main").expect("id"),
            node: NodeId::parse("pg1").expect("id"),
            database: "db".to_owned(),
            login: "login".to_owned(),
            identity: IdentityId::parse("app").expect("id"),
            configuration_generation: ConfigurationGeneration::FIRST,
            startup_settings: Vec::new(),
            credential_generation: CredentialGeneration::FIRST,
            topology_epoch: TopologyEpoch::FIRST,
            tls_policy: TlsPolicy::VerifyFull,
            startup_profile: StartupProfile::DedicatedV1,
        }
    }
    fn owner(limits: &str) -> Arc<Pool<tokio::io::DuplexStream>> {
        let config = pool_config::validate(format!("config_version = 1\n[limits]\n{limits}\n[control]\nstate_directory = \"/state\"\n[[cluster]]\nid = \"main\"\nsystem_identifier = \"1\"\nexpected_writer = \"pg1\"\n[[node]]\nid = \"pg1\"\ncluster = \"main\"\nhost = \"node\"\nport = 5432\n").as_bytes()).expect("config");
        Pool::new(
            HashMap::from([(
                key().identity,
                Credential::new(Password::parse(b"secret").expect("password")),
            )]),
            config.limits(),
        )
    }
    fn reserve(
        ticket: &Ticket<tokio::io::DuplexStream>,
        key: &PoolKey,
    ) -> Permit<tokio::io::DuplexStream> {
        match ticket.take(key) {
            Candidate::New(permit) => permit,
            _ => panic!("expected all caps reserved together"),
        }
    }
    #[test]
    fn nested_budgets_reserve_all_or_none_and_cancellation_unblocks_fifo() {
        for caps in [
            "backend_max_count = 1",
            "node_backend_max_count = 1",
            "database_backend_max_count = 1",
            "identity_backend_max_count = 1",
        ] {
            let pool = owner(caps);
            let first = pool.ticket().expect("ticket");
            let permit = reserve(&first, &key());
            assert!(
                pool.lock().queue.is_empty(),
                "connecting owner has its budget, not a waiter slot"
            );
            drop(first);
            let cancelled = pool.ticket().expect("head");
            let next = pool.ticket().expect("next");
            assert!(matches!(cancelled.take(&key()), Candidate::Wait));
            assert!(matches!(next.take(&key()), Candidate::Wait));
            assert_eq!(pool.lock().occupied.len(), 1, "no partial reservation");
            drop(permit);
            assert!(
                matches!(next.take(&key()), Candidate::Wait),
                "head of line blocks later acquisitions"
            );
            drop(cancelled);
            let permit = reserve(&next, &key());
            assert_eq!(pool.lock().occupied.len(), 1);
            drop(permit);
            assert!(pool.lock().occupied.is_empty());
        }
    }
    #[test]
    fn fifo_is_bounded_and_retirement_refuses_existing_tickets() {
        let pool = owner("backend_wait_max_count = 1");
        let ticket = pool.ticket().expect("ticket");
        assert!(pool.ticket().is_none());
        pool.retire();
        assert!(matches!(ticket.take(&key()), Candidate::Retired));
        drop(ticket);
        assert!(pool.lock().queue.is_empty());
        assert!(pool.ticket().is_none());
    }
    #[test]
    fn every_runtime_and_peer_partition_is_in_key_equality() {
        let base = key();
        let mut alternatives = Vec::new();
        let mut k = base.clone();
        k.identity = IdentityId::parse("other").expect("id");
        alternatives.push(k);
        let mut k = base.clone();
        k.configuration_generation = k.configuration_generation.checked_next().expect("next");
        alternatives.push(k);
        let mut k = base.clone();
        k.credential_generation = k.credential_generation.checked_next().expect("next");
        alternatives.push(k);
        let mut k = base.clone();
        k.topology_epoch = k.topology_epoch.checked_next().expect("next");
        alternatives.push(k);
        let mut k = base.clone();
        k.startup_settings
            .push(("application_name".to_owned(), b"PEER-CANARY".to_vec()));
        alternatives.push(k);
        let mut k = base.clone();
        k.cluster = ClusterId::parse("other").expect("id");
        alternatives.push(k);
        let mut k = base.clone();
        k.node = NodeId::parse("other").expect("id");
        alternatives.push(k);
        let mut k = base.clone();
        k.database.push('x');
        alternatives.push(k);
        let mut k = base.clone();
        k.login.push('x');
        alternatives.push(k);
        for k in alternatives {
            assert_ne!(base, k);
            assert!(!format!("{k:?}").contains("PEER-CANARY"));
        }
    }
}
