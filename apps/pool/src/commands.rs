//! `pool bootstrap` and `pool show-authority` (spec 0004, "Commands").
//!
//! Each reads and validates the file as `check-config` does, then works on
//! the state directory. The steps run in the order of the spec, and the
//! first failure ends the command.

use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use pool_config::{AuthorityId, FindingCode, ValidatedConfig};
use pool_control::{Journal, MonotonicTime, ObservationBoard};
use pool_core::backend::Login;
use pool_core::load;
use pool_core::random::{self, RandomError};
use pool_core::secret::SecretError;

use crate::output::{Item, Report};
use crate::probe::{self, Target};
use crate::report::IoFinding;
use crate::state::{StateDirectory, StateError};

const KEY_STATE_DIRECTORY: &str = "control.state_directory";
const KEY_MONITOR_CREDENTIAL: &str = "cluster.monitor_credential_file";
const KEY_NODE_CA: &str = "node.tls_ca_file";
const REFUSED_MESSAGE: &str = "bootstrap is not possible in this state";
const JOURNAL_FAILED: &str = "journal.failed";
const JOURNAL_FAILED_MESSAGE: &str = "the journal could not be written";
const RANDOM_FAILED: &str = "random.failed";
const RANDOM_FAILED_MESSAGE: &str = "random bytes could not be obtained";
/// The finding of a runtime that could not be built, in `bootstrap` and `run`.
pub const RUNTIME_FAILED: &str = "runtime.failed";
/// The message of [`RUNTIME_FAILED`].
pub const RUNTIME_FAILED_MESSAGE: &str = "the runtime could not be started";

/// Runs `pool bootstrap` on the file at `path` and returns its report.
pub fn bootstrap_file(path: &Path) -> Report {
    load_config(path)
        .and_then(|config| bootstrap(&config))
        .unwrap_or_else(|report| report)
}

/// Runs `pool show-authority` on the file at `path` and returns its report.
pub fn show_authority_file(path: &Path) -> Report {
    load_config(path)
        .and_then(|config| show_authority(&config))
        .unwrap_or_else(|report| report)
}

/// Reads and validates the file, as `check-config` does.
pub(crate) fn load_config(path: &Path) -> Result<ValidatedConfig, Report> {
    let bytes = crate::read_bounded(path).map_err(|error| {
        let finding = IoFinding::from_kind(error.kind());
        Report::Error(Item::plain(finding.code(), finding.message()))
    })?;
    pool_config::validate(&bytes).map_err(|rejection| Report::invalid(&rejection))
}

pub(crate) fn state_error(error: StateError) -> Report {
    Report::Error(Item::plain(error.code(), error.message()))
}

pub(crate) fn secret_error(error: SecretError, key: &'static str) -> Report {
    Report::Error(Item::at_key(error.code(), key, error.message()))
}

/// `pool show-authority`: reads the journal without writing, and prints the
/// state and the ID of the journal that was read. It contacts no node.
fn show_authority(config: &ValidatedConfig) -> Result<Report, Report> {
    let Some(directory) = config
        .control()
        .and_then(|control| control.state_directory())
    else {
        return Err(Report::Invalid {
            items: vec![Item::at_key(
                FindingCode::Incomplete.as_str(),
                KEY_STATE_DIRECTORY,
                FindingCode::Incomplete.message(),
            )],
            omitted: 0,
        });
    };
    let state = StateDirectory::open(directory).map_err(state_error)?;
    let read = Journal::read(&state.journal());
    let authority_id = read.authority_id();
    let classified = read.classify(config);
    Ok(Report::Authority {
        state: classified.name(),
        authority_id,
    })
}

/// `pool bootstrap`: steps 1 to 7 of spec 0004, "pool bootstrap".
fn bootstrap(config: &ValidatedConfig) -> Result<Report, Report> {
    // 1. The view for bootstrap.
    let view = config
        .probing()
        .map_err(|rejection| Report::invalid(&rejection))?;
    // 2. The state directory and its lock.
    let state = StateDirectory::open(view.state_directory()).map_err(state_error)?;
    // 3. The journal, then the monitor credentials and the CA files.
    let read = Journal::read(&state.journal());
    let targets = targets(config, state.owner())?;
    // 4. One probe of every node, all at the same time. A runtime that
    // cannot be built is `runtime.failed`, and nothing is written.
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()
        .map_err(|_| Report::Error(Item::plain(RUNTIME_FAILED, RUNTIME_FAILED_MESSAGE)))?;
    let origin = tokio::time::Instant::now();
    let mut board = ObservationBoard::new();
    let timeout = config.probe().timeout();
    for observation in runtime.block_on(probe::probe_all(targets, timeout, origin)) {
        board.record(observation);
    }
    let now = MonotonicTime::from_origin(origin.elapsed());
    // 5. The plan, or the reasons it was refused. Nothing is written then.
    let plan = read
        .bootstrap_plan(config, &board, now)
        .map_err(|refused| {
            Report::Refused(
                refused
                    .reasons()
                    .iter()
                    .map(|reason| Item::plain(reason.as_str(), REFUSED_MESSAGE))
                    .collect(),
            )
        })?;
    // 6. A new authority ID, the empty file, and the journal in one
    // transaction. Without random bytes no file is created.
    let journal_failed = || Report::Error(Item::plain(JOURNAL_FAILED, JOURNAL_FAILED_MESSAGE));
    let authority_id = new_authority_id()
        .map_err(|_| Report::Error(Item::plain(RANDOM_FAILED, RANDOM_FAILED_MESSAGE)))?;
    let path = state.create_empty_journal().map_err(state_error)?;
    Journal::create(&path, &plan, authority_id, unix_ms()).map_err(|_| journal_failed())?;
    // 7. The result.
    Ok(Report::Bootstrapped {
        authority_id,
        clusters: plan.clusters().len(),
    })
}

/// Every node of every cluster, with the monitor login of its cluster. A
/// file that cannot be used ends the command with its key.
fn targets(config: &ValidatedConfig, owner: u32) -> Result<Vec<Target>, Report> {
    let mut targets = Vec::new();
    for cluster in config.clusters() {
        let (Some(login), Some(reference)) =
            (cluster.monitor_login(), cluster.monitor_credential())
        else {
            // The view of step 1 refuses this already.
            return Err(secret_error(SecretError::NotFound, KEY_MONITOR_CREDENTIAL));
        };
        let credential = load::credential(reference, owner)
            .map_err(|error| secret_error(error, KEY_MONITOR_CREDENTIAL))?;
        let login = Login::new(login, cluster.monitor_database(), credential);
        for node in config
            .nodes()
            .iter()
            .filter(|node| node.cluster() == cluster.id())
        {
            let target = load::node(node).map_err(|error| secret_error(error, KEY_NODE_CA))?;
            targets.push(Target::new(node.id().clone(), target, login.clone()));
        }
    }
    Ok(targets)
}

/// A new authority ID: 16 random bytes of the operating system.
fn new_authority_id() -> Result<AuthorityId, RandomError> {
    let mut bytes = [0_u8; 16];
    random::fill_random(&mut bytes)?;
    Ok(AuthorityId::from_bytes(bytes))
}

/// The wall clock in milliseconds since 1970, for display in the journal.
pub(crate) fn unix_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |since| {
            u64::try_from(since.as_millis()).unwrap_or(u64::MAX)
        })
}
