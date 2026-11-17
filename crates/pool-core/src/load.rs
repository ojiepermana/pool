//! Reading what the configuration names: the files of the listener, of the
//! principals, of the logins and of the nodes.
//!
//! `owner` is the service account, the owner of the state directory, which
//! the caller has checked. Each function names its key in the finding the
//! caller writes; none of them returns a path.

use std::sync::Arc;

use pool_config::{ListenerConfig, NodeConfig, PrincipalConfig, SecretReference};
use rustls::ServerConfig;

use crate::backend::{Credential, Node};
use crate::secret::{self, SecretError, Verifier};
use crate::tls::{NodeTls, server_config};

/// The TLS server configuration of the listener: `listener.tls_certificate_file` and `listener.tls_private_key_file`.
pub fn listener_tls(
    listener: &ListenerConfig,
    owner: u32,
) -> Result<Arc<ServerConfig>, SecretError> {
    let chain = secret::read_certificate_chain(listener.tls_certificate_file(), owner)?;
    let key = secret::read_private_key(listener.tls_private_key().path(), owner)?;
    server_config(&chain, &key)
}

/// The verifier of a principal: `principal[].verifier_file`.
pub fn verifier(principal: &PrincipalConfig, owner: u32) -> Result<Verifier, SecretError> {
    secret::read_verifier(principal.verifier().path(), owner)
}

/// A backend or monitor credential: `identity[].credential_file` or `cluster[].monitor_credential_file`.
pub fn credential(reference: &SecretReference, owner: u32) -> Result<Arc<Credential>, SecretError> {
    secret::read_credential(reference.path(), owner).map(Credential::new)
}

/// A node with its TLS setup: `node[].host`, `node[].port`, `node[].tls_ca_file` and `node[].tls_server_name`.
///
/// A node without a CA file is [`SecretError::NotFound`]; the serving and
/// probing views of `pool-config` already refuse such a configuration.
pub fn node(node: &NodeConfig, owner: u32) -> Result<Node, SecretError> {
    let ca = node.tls_ca_file().ok_or(SecretError::NotFound)?;
    let roots = secret::read_trust_roots(ca, owner)?;
    let tls = NodeTls::new(&roots, node.tls_server_name())?;
    Ok(Node::new(node.host(), node.port(), tls))
}
