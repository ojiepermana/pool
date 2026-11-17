//! The TLS setup of this slice (spec 0004, "Startup" and "Backend connection").
//!
//! As a server: TLS 1.3 and TLS 1.2 with the defaults of `rustls`, no client
//! certificate and no ALPN protocol. As a client toward a node: the
//! certificates of the node's CA file as the only roots, and the configured
//! server name checked. That client policy is called `VerifyFull` and is a
//! part of the pool key. Both use the `aws-lc-rs` provider.

use std::fmt;
use std::sync::Arc;

use rustls::crypto::CryptoProvider;
use rustls::pki_types::ServerName;
use rustls::{ClientConfig, ServerConfig};

use crate::secret::{CertificateChain, PrivateKey, SecretError, TrustRoots};

/// The one TLS policy toward a node in this slice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TlsPolicy {
    /// The chain must lead to the node's CA and the name must match.
    VerifyFull,
}

fn provider() -> Arc<CryptoProvider> {
    Arc::new(rustls::crypto::aws_lc_rs::default_provider())
}

/// The server configuration of the listener, from its certificate chain and private key.
///
/// [`SecretError::Invalid`] when the key does not belong to the leaf or cannot be used.
pub fn server_config(
    chain: &CertificateChain,
    key: &PrivateKey,
) -> Result<Arc<ServerConfig>, SecretError> {
    let config = ServerConfig::builder_with_provider(provider())
        .with_protocol_versions(&[&rustls::version::TLS13, &rustls::version::TLS12])
        .map_err(|_| SecretError::Invalid)?
        .with_no_client_auth()
        .with_single_cert(chain.0.clone(), key.0.clone_key())
        .map_err(|_| SecretError::Invalid)?;
    Ok(Arc::new(config))
}

/// The TLS client setup for one node: its roots and the name to check.
#[derive(Clone)]
pub struct NodeTls {
    pub(crate) config: Arc<ClientConfig>,
    pub(crate) name: ServerName<'static>,
}

impl fmt::Debug for NodeTls {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // The server name is a host name of the deployment: kept out of default telemetry.
        f.debug_struct("NodeTls")
            .field("policy", &TlsPolicy::VerifyFull)
            .finish_non_exhaustive()
    }
}

impl NodeTls {
    /// The client setup from the node's CA file and `node[].tls_server_name`.
    ///
    /// [`SecretError::Invalid`] when the name is not a DNS name or an IP address.
    pub fn new(roots: &TrustRoots, server_name: &str) -> Result<Self, SecretError> {
        let name =
            ServerName::try_from(server_name.to_string()).map_err(|_| SecretError::Invalid)?;
        let config = ClientConfig::builder_with_provider(provider())
            .with_protocol_versions(&[&rustls::version::TLS13, &rustls::version::TLS12])
            .map_err(|_| SecretError::Invalid)?
            .with_root_certificates(roots.0.clone())
            .with_no_client_auth();
        Ok(Self {
            config: Arc::new(config),
            name,
        })
    }

    /// The policy, for the pool key.
    pub fn policy(&self) -> TlsPolicy {
        TlsPolicy::VerifyFull
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture(name: &str) -> Vec<u8> {
        std::fs::read(format!(
            "{}/tests/fixtures/{name}",
            env!("CARGO_MANIFEST_DIR")
        ))
        .expect("fixture")
    }

    #[test]
    fn the_server_configuration_needs_the_key_of_the_leaf_and_offers_no_alpn() {
        let key = PrivateKey::parse(&fixture("node.key")).expect("key");
        let chain = CertificateChain::parse(&fixture("node.pem")).expect("chain");
        let config = server_config(&chain, &key).expect("server config");
        assert!(config.alpn_protocols.is_empty());
        let other = CertificateChain::parse(&fixture("other-ca.pem")).expect("chain");
        assert_eq!(
            server_config(&other, &key).map(|_| ()),
            Err(SecretError::Invalid)
        );
    }

    #[test]
    fn a_node_name_must_be_a_name_or_an_address() {
        let roots = TrustRoots::parse(&fixture("ca.pem")).expect("roots");
        assert!(NodeTls::new(&roots, "node.test").is_ok());
        assert!(NodeTls::new(&roots, "10.0.0.5").is_ok());
        assert_eq!(
            NodeTls::new(&roots, "not a name").map(|_| ()),
            Err(SecretError::Invalid)
        );
        let tls = NodeTls::new(&roots, "node.test").expect("tls");
        assert_eq!(tls.policy(), TlsPolicy::VerifyFull);
        assert!(!format!("{tls:?}").contains("node.test"));
    }
}
