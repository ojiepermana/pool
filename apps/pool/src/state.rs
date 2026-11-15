//! The state directory: its check and its lock (spec 0004, "State directory
//! and protected files").
//!
//! The owner of the state directory is the service account. Every check
//! compares owners with it, so no call asks the operating system who the
//! process is. The lock is an exclusive advisory lock on `pool.lock`, held
//! until the process ends.

use std::fs::{File, OpenOptions, TryLockError};
use std::io;
use std::os::unix::fs::{MetadataExt, OpenOptionsExt, PermissionsExt};
use std::path::{Path, PathBuf};

/// The name of the lock file in the state directory.
pub const LOCK_FILE: &str = "pool.lock";
/// The name of the journal in the state directory.
pub const JOURNAL_FILE: &str = "journal.sqlite3";
/// The files of the state directory that must be regular files with mode
/// `0600` owned by the service account, when they exist.
const PROTECTED_FILES: [&str; 4] = [
    LOCK_FILE,
    JOURNAL_FILE,
    "journal.sqlite3-wal",
    "journal.sqlite3-shm",
];

/// Why the state directory cannot be used. Each is exit status 3.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StateError {
    /// `state.not_found`: the state directory does not exist.
    NotFound,
    /// `state.permissions`: the directory or a journal file has the wrong
    /// type, mode or owner, or cannot be opened.
    Permissions,
    /// `state.locked`: another process holds the lock.
    Locked,
}

impl StateError {
    /// The finding code.
    pub fn code(self) -> &'static str {
        match self {
            StateError::NotFound => "state.not_found",
            StateError::Permissions => "state.permissions",
            StateError::Locked => "state.locked",
        }
    }

    /// The constant message of the finding.
    pub fn message(self) -> &'static str {
        match self {
            StateError::NotFound => "the state directory does not exist",
            StateError::Permissions => {
                "the state directory or a file in it has the wrong type, mode or owner"
            }
            StateError::Locked => "another process is using the state directory",
        }
    }
}

/// A checked and locked state directory. The lock is held while this value
/// lives, and the operating system releases it when the process ends.
#[derive(Debug)]
pub struct StateDirectory {
    path: PathBuf,
    owner: u32,
    _lock: File,
}

impl StateDirectory {
    /// Checks the directory and the files in it, then takes the lock.
    ///
    /// The directory must be a directory with mode exactly `0700`. Each of
    /// `pool.lock`, `journal.sqlite3` and its two side files, when it exists,
    /// must be a regular file with mode exactly `0600` owned by the owner of
    /// the directory. Links are not followed. `pool.lock` is created with
    /// mode `0600` when it is absent, and never removed.
    pub fn open(path: &Path) -> Result<StateDirectory, StateError> {
        let metadata = match std::fs::symlink_metadata(path) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                return Err(StateError::NotFound);
            }
            Err(_) => return Err(StateError::Permissions),
        };
        if !metadata.is_dir() || metadata.permissions().mode() & 0o7777 != 0o700 {
            return Err(StateError::Permissions);
        }
        let owner = metadata.uid();
        for name in PROTECTED_FILES {
            check_protected(&path.join(name), owner)?;
        }
        let lock_path = path.join(LOCK_FILE);
        let lock = match OpenOptions::new().read(true).write(true).open(&lock_path) {
            Ok(file) => file,
            Err(error) if error.kind() == io::ErrorKind::NotFound => create_protected(&lock_path)?,
            Err(_) => return Err(StateError::Permissions),
        };
        // The file may have been made just now, so it is checked again.
        check_protected(&lock_path, owner)?;
        match lock.try_lock() {
            Ok(()) => {}
            Err(TryLockError::WouldBlock) => return Err(StateError::Locked),
            Err(TryLockError::Error(_)) => return Err(StateError::Permissions),
        }
        Ok(StateDirectory {
            path: path.to_path_buf(),
            owner,
            _lock: lock,
        })
    }

    /// The uid of the service account: the owner of the directory.
    pub fn owner(&self) -> u32 {
        self.owner
    }

    /// The path of the journal in the directory.
    pub fn journal(&self) -> PathBuf {
        self.path.join(JOURNAL_FILE)
    }

    /// Creates the empty journal file with mode `0600`, unless a file is
    /// there already. A file that exists is what an interrupted bootstrap
    /// leaves: 0 bytes, or one page with no mark and no table after the WAL
    /// mode was set. The reader counted it as absent, and `Journal::create`
    /// applies the same rule for an empty database and refuses anything
    /// else (spec 0004, decision 59). The check of [`StateDirectory::open`]
    /// already holds for a file that exists.
    pub fn create_empty_journal(&self) -> Result<PathBuf, StateError> {
        let path = self.journal();
        match std::fs::symlink_metadata(&path) {
            Ok(_) => Ok(path),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                create_protected(&path)?;
                Ok(path)
            }
            Err(_) => Err(StateError::Permissions),
        }
    }
}

/// A file of the state directory, when it exists: a regular file, mode
/// exactly `0600`, owned by `owner`. Links are not followed.
fn check_protected(path: &Path, owner: u32) -> Result<(), StateError> {
    match std::fs::symlink_metadata(path) {
        Ok(metadata) => {
            let fits = metadata.is_file()
                && metadata.permissions().mode() & 0o7777 == 0o600
                && metadata.uid() == owner;
            if fits {
                Ok(())
            } else {
                Err(StateError::Permissions)
            }
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Err(_) => Err(StateError::Permissions),
    }
}

/// Creates a new file with mode `0600`, and sets the mode again so that a
/// strange umask cannot leave another one.
fn create_protected(path: &Path) -> Result<File, StateError> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)
        .map_err(|_| StateError::Permissions)?;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))
        .map_err(|_| StateError::Permissions)?;
    Ok(file)
}
