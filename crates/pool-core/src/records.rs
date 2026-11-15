//! The records of a session (spec 0004, "Records built in this slice").

use std::fmt;

use pool_config::{
    ClusterId, CredentialGeneration, IdentityId, NodeId, PrincipalId, TopologyEpoch,
};
use pool_protocol::auth::BackendKeyData;
use tokio::sync::OwnedSemaphorePermit;

use crate::REDACTED;
use crate::backend::BackendConnection;
use crate::tls::TlsPolicy;

/// The largest value of a memory generation, the same as of the durable ones.
const MAX_GENERATION: u64 = 9_223_372_036_854_775_807;

/// The generation of a lease. It starts at 1 and advances for each lease of this run.
///
/// It compares only with another `LeaseGeneration`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LeaseGeneration(u64);

impl LeaseGeneration {
    /// The first lease of a run.
    pub const FIRST: LeaseGeneration = LeaseGeneration(1);

    /// The number.
    pub fn get(self) -> u64 {
        self.0
    }

    /// The next generation; `None` at 9223372036854775807. It never wraps.
    pub fn checked_next(self) -> Option<LeaseGeneration> {
        (self.0 < MAX_GENERATION).then(|| LeaseGeneration(self.0 + 1))
    }
}

impl fmt::Display for LeaseGeneration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// The ID of a session: the start sequence in the high 64 bits and a counter of this run in the low 64 bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SessionId(pub u128);

impl SessionId {
    /// The ID for this start sequence and counter.
    pub fn new(start_sequence: u64, counter: u64) -> SessionId {
        SessionId((u128::from(start_sequence) << 64) | u128::from(counter))
    }
}

impl fmt::Display for SessionId {
    /// 32 lowercase hex characters.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:032x}", self.0)
    }
}

/// A principal whose SCRAM proof passed. Made only by the authentication code of this crate.
#[derive(Clone, PartialEq, Eq)]
pub struct VerifiedPrincipal {
    id: PrincipalId,
    name: String,
}

impl VerifiedPrincipal {
    pub(crate) fn new(id: PrincipalId, name: String) -> Self {
        Self { id, name }
    }

    /// The ID of the principal entry.
    pub fn id(&self) -> &PrincipalId {
        &self.id
    }

    /// The verified name, the value of `user` byte for byte.
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl fmt::Debug for VerifiedPrincipal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("VerifiedPrincipal")
            .field("id", &self.id)
            .field("name", &REDACTED)
            .finish()
    }
}

/// How the backend of a session is opened. This slice has one profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StartupProfile {
    /// The mapped login, the mapped database and the forwarded parameters.
    DedicatedV1,
}

/// The pool key: the eight parts of `0001 SC-2`.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PoolKey {
    /// The cluster of the identity entry.
    pub cluster: ClusterId,
    /// The node of the route.
    pub node: NodeId,
    /// The backend database of the identity entry.
    pub database: String,
    /// The backend login of the identity entry.
    pub login: String,
    /// The generation of the credential.
    pub credential_generation: CredentialGeneration,
    /// The topology epoch of the route.
    pub topology_epoch: TopologyEpoch,
    /// The TLS policy toward the node.
    pub tls_policy: TlsPolicy,
    /// The startup profile.
    pub startup_profile: StartupProfile,
}

impl fmt::Debug for PoolKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PoolKey")
            .field("cluster", &self.cluster)
            .field("node", &self.node)
            .field("database", &REDACTED)
            .field("login", &REDACTED)
            .field("credential_generation", &self.credential_generation)
            .field("topology_epoch", &self.topology_epoch)
            .field("tls_policy", &self.tls_policy)
            .field("startup_profile", &self.startup_profile)
            .finish()
    }
}

/// The one lease of a session: it owns the backend, its permit, its generation, its pool key and the
/// private `BackendKeyData`. It is never moved to another session (`0001 SC-1`).
pub struct Lease<S> {
    pub(crate) backend: BackendConnection<S>,
    pub(crate) _permit: OwnedSemaphorePermit,
    generation: LeaseGeneration,
    key: PoolKey,
    identity: IdentityId,
}

impl<S> Lease<S> {
    pub(crate) fn new(
        backend: BackendConnection<S>,
        permit: OwnedSemaphorePermit,
        generation: LeaseGeneration,
        key: PoolKey,
        identity: IdentityId,
    ) -> Self {
        Self {
            backend,
            _permit: permit,
            generation,
            key,
            identity,
        }
    }

    /// The lease generation.
    pub fn generation(&self) -> LeaseGeneration {
        self.generation
    }

    /// The pool key.
    pub fn key(&self) -> &PoolKey {
        &self.key
    }

    /// The identity entry the lease was made for.
    pub fn identity(&self) -> &IdentityId {
        &self.identity
    }

    /// The real cancellation key of the backend. It never reaches the client.
    pub fn backend_key(&self) -> Option<&BackendKeyData> {
        self.backend.key.as_ref()
    }
}

impl<S> fmt::Debug for Lease<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Lease")
            .field("generation", &self.generation)
            .field("key", &self.key)
            .finish_non_exhaustive()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lease_generations_start_at_1_and_never_wrap() {
        assert_eq!(LeaseGeneration::FIRST.get(), 1);
        assert_eq!(
            LeaseGeneration::FIRST
                .checked_next()
                .map(LeaseGeneration::get),
            Some(2)
        );
        assert_eq!(LeaseGeneration(MAX_GENERATION).checked_next(), None);
    }

    #[test]
    fn a_session_id_is_the_start_sequence_and_a_counter_in_32_hex_characters() {
        let id = SessionId::new(3, 7);
        assert_eq!(id.0, (3u128 << 64) | 7);
        assert_eq!(id.to_string(), "00000000000000030000000000000007");
        assert_eq!(id.to_string().len(), 32);
    }

    // Added by the tests of scope 004, round four (review finding t2).
    #[test]
    fn a_verified_principal_prints_its_id_and_never_its_name() {
        let principal = VerifiedPrincipal::new(
            PrincipalId::parse("orders-app").expect("id"),
            "CANARY-9f2c7b1e".to_owned(),
        );
        assert_eq!(
            format!("{principal:?}"),
            "VerifiedPrincipal { id: PrincipalId(\"orders-app\"), name: \"[REDACTED]\" }"
        );
        assert!(!format!("{principal:#?}").contains("CANARY"));
    }
}
