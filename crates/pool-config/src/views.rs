//! The two views that say whether a valid file has what a command needs:
//! [`ProbingView`] for `pool bootstrap` and [`ServingView`] for `pool run`.
//!
//! The contract is spec 0004, section "What a command needs". Each missing
//! piece gives one finding `config.incomplete`, in the order of the rows of
//! that table. A view borrows from the [`ValidatedConfig`] and copies no
//! value.

use std::fmt;
use std::path::Path;

use crate::{Finding, FindingCode, Findings, ListenerConfig, Location, Rejection, ValidatedConfig};

const KEY_STATE_DIRECTORY: &str = "control.state_directory";
const KEY_CLUSTER_ID: &str = "cluster.id";
const KEY_CLUSTER_MONITOR_LOGIN: &str = "cluster.monitor_login";
const KEY_CLUSTER_MONITOR_CREDENTIAL: &str = "cluster.monitor_credential_file";
const KEY_NODE_TLS_CA: &str = "node.tls_ca_file";
const KEY_LISTENER_HOST: &str = "listener.host";
const KEY_IDENTITY_ID: &str = "identity.id";
const KEY_IDENTITY_PRINCIPAL: &str = "identity.principal";

/// A valid configuration that has everything `pool bootstrap` needs: a state
/// directory, at least one cluster, the monitor login and its credential in
/// every cluster, and a CA file in every node.
pub struct ProbingView<'a> {
    config: &'a ValidatedConfig,
    state_directory: &'a Path,
}

impl<'a> ProbingView<'a> {
    /// The whole validated configuration.
    pub fn config(&self) -> &'a ValidatedConfig {
        self.config
    }

    /// The directory that holds the journal, from `control.state_directory`.
    pub fn state_directory(&self) -> &'a Path {
        self.state_directory
    }
}

impl fmt::Debug for ProbingView<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProbingView")
            .field("clusters", &self.config.clusters.len())
            .field("nodes", &self.config.nodes.len())
            .finish()
    }
}

/// A valid configuration that has everything `pool run` needs: all that
/// [`ProbingView`] needs, a `[listener]` table, at least one identity, and a
/// principal entry for the principal of every identity.
pub struct ServingView<'a> {
    config: &'a ValidatedConfig,
    state_directory: &'a Path,
    listener: &'a ListenerConfig,
}

impl<'a> ServingView<'a> {
    /// The whole validated configuration.
    pub fn config(&self) -> &'a ValidatedConfig {
        self.config
    }

    /// The directory that holds the journal, from `control.state_directory`.
    pub fn state_directory(&self) -> &'a Path {
        self.state_directory
    }

    /// The `[listener]` table.
    pub fn listener(&self) -> &'a ListenerConfig {
        self.listener
    }
}

impl fmt::Debug for ServingView<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ServingView")
            .field("clusters", &self.config.clusters.len())
            .field("nodes", &self.config.nodes.len())
            .field("identities", &self.config.identities.len())
            .field("principals", &self.config.principals.len())
            .finish()
    }
}

fn push(findings: &mut Findings, location: Location) {
    findings.push(Finding {
        code: FindingCode::Incomplete,
        location,
    });
}

fn push_at(findings: &mut Findings, key: &'static str, line: u32, column: u32) {
    push(findings, Location::KeyAt { key, line, column });
}

impl ValidatedConfig {
    /// The view for `pool bootstrap`, or a [`Rejection`] whose findings all
    /// have the code `config.incomplete`, one per missing piece.
    pub fn probing(&self) -> Result<ProbingView<'_>, Rejection> {
        let mut findings = Findings::new();
        let state_directory = self.probing_needs(&mut findings);
        match (findings.into_rejection(), state_directory) {
            (None, Some(state_directory)) => Ok(ProbingView {
                config: self,
                state_directory,
            }),
            (Some(rejection), _) => Err(rejection),
            (None, None) => Err(Rejection::single(
                FindingCode::Incomplete,
                Location::Key(KEY_STATE_DIRECTORY),
            )),
        }
    }

    /// The view for `pool run`, or a [`Rejection`] whose findings all have
    /// the code `config.incomplete`, one per missing piece.
    pub fn serving(&self) -> Result<ServingView<'_>, Rejection> {
        let mut findings = Findings::new();
        let state_directory = self.probing_needs(&mut findings);
        if self.listener.is_none() {
            push(&mut findings, Location::Key(KEY_LISTENER_HOST));
        }
        if self.identities.is_empty() {
            push(&mut findings, Location::Key(KEY_IDENTITY_ID));
        }
        for identity in &self.identities {
            if self.resolve_principal(identity.principal()).is_none() {
                let at = identity.entry_at();
                push_at(&mut findings, KEY_IDENTITY_PRINCIPAL, at.line, at.column);
            }
        }
        match (
            findings.into_rejection(),
            state_directory,
            self.listener.as_ref(),
        ) {
            (None, Some(state_directory), Some(listener)) => Ok(ServingView {
                config: self,
                state_directory,
                listener,
            }),
            (Some(rejection), _, _) => Err(rejection),
            (None, _, _) => Err(Rejection::single(
                FindingCode::Incomplete,
                Location::Key(KEY_STATE_DIRECTORY),
            )),
        }
    }

    /// The rows that both views need, in the order of the table: the state
    /// directory, at least one cluster, the monitor keys of each cluster, the
    /// CA file of each node.
    fn probing_needs(&self, findings: &mut Findings) -> Option<&Path> {
        let state_directory = self
            .control
            .as_ref()
            .and_then(|control| control.state_directory());
        if state_directory.is_none() {
            push(findings, Location::Key(KEY_STATE_DIRECTORY));
        }
        if self.clusters.is_empty() {
            push(findings, Location::Key(KEY_CLUSTER_ID));
        }
        for cluster in &self.clusters {
            let at = cluster.entry_at();
            if cluster.monitor_login().is_none() {
                push_at(findings, KEY_CLUSTER_MONITOR_LOGIN, at.line, at.column);
            }
            if cluster.monitor_credential().is_none() {
                push_at(findings, KEY_CLUSTER_MONITOR_CREDENTIAL, at.line, at.column);
            }
        }
        for node in &self.nodes {
            if node.tls_ca_file().is_none() {
                let at = node.entry_at();
                push_at(findings, KEY_NODE_TLS_CA, at.line, at.column);
            }
        }
        state_directory
    }
}
