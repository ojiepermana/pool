//! `pool bootstrap` and `pool show-authority` without a database (spec 0004,
//! "Commands" and "State directory and protected files"; `0004 AC-9`, the
//! hermetic part of `AC-11`).
//!
//! Each test makes its own folder under `CARGO_TARGET_TMPDIR`. A node "that
//! does not answer" is a port on `127.0.0.1` that nothing listens on, or a
//! listener of the test that accepts and never speaks. The CA file is the
//! test CA of `crates/pool-core/tests/fixtures/`.

use std::fs;
use std::net::TcpListener;
use std::os::unix::fs::{PermissionsExt, symlink};
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

const CA: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../crates/pool-core/tests/fixtures/ca.pem"
);
const X: &str = "0123456789abcdef0123456789abcdef";
const REFUSED: &str = "bootstrap is not possible in this state";
const STATE_PERMISSIONS: &str =
    "the state directory or a file in it has the wrong type, mode or owner";

struct Run {
    code: i32,
    stdout: String,
    stderr: String,
}

fn pool(args: &[&str]) -> Run {
    let output: Output = Command::new(env!("CARGO_BIN_EXE_pool"))
        .args(args)
        .stdin(Stdio::null())
        .output()
        .expect("run pool");
    Run {
        code: output.status.code().expect("exit status, not a signal"),
        stdout: String::from_utf8(output.stdout).expect("utf-8"),
        stderr: String::from_utf8(output.stderr).expect("utf-8"),
    }
}

fn command(name: &str, config: &Path, format: &str) -> Run {
    pool(&[
        name,
        "--config",
        config.to_str().expect("utf-8 path"),
        "--format",
        format,
    ])
}

/// A folder of this test, empty, with mode `0700`.
fn folder(name: &str) -> PathBuf {
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("pool-cmd-{name}"));
    if path.exists() {
        // A test may leave a folder without write permission behind.
        let _ = fs::set_permissions(&path, fs::Permissions::from_mode(0o700));
        fs::remove_dir_all(&path).expect("remove old test folder");
    }
    fs::create_dir_all(&path).expect("create test folder");
    fs::set_permissions(&path, fs::Permissions::from_mode(0o700)).expect("chmod");
    path
}

fn write_mode(path: &Path, content: &[u8], mode: u32) {
    fs::write(path, content).expect("write");
    fs::set_permissions(path, fs::Permissions::from_mode(mode)).expect("chmod");
}

/// A port on 127.0.0.1 that nothing listens on.
fn closed_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    listener.local_addr().expect("address").port()
}

/// The layout of one test: a state directory, a monitor credential and a
/// configuration that points at them.
struct Layout {
    root: PathBuf,
    state: PathBuf,
    credential: PathBuf,
    config: PathBuf,
}

impl Layout {
    fn new(name: &str) -> Layout {
        let root = folder(name);
        let state = root.join("state");
        fs::create_dir(&state).expect("state directory");
        fs::set_permissions(&state, fs::Permissions::from_mode(0o700)).expect("chmod");
        let credential = root.join("monitor.password");
        write_mode(&credential, b"pz7-monitor-password\n", 0o600);
        let config = root.join("pool.toml");
        let layout = Layout {
            root,
            state,
            credential,
            config,
        };
        layout.write_config(None, closed_port(), CA, "");
        layout
    }

    fn write_config(&self, witness: Option<&str>, port: u16, ca: &str, extra: &str) {
        let witness = witness.map_or(String::new(), |id| format!("authority_id = \"{id}\"\n"));
        let text = format!(
            "config_version = 1\n\n[control]\nstate_directory = \"{state}\"\n{witness}\n\
             [probe]\nperiod_ms = 200\ntimeout_ms = 200\nobservation_expiry_ms = 1000\n\n\
             [[cluster]]\nid = \"main\"\nsystem_identifier = \"7412345678901234567\"\nexpected_writer = \"pg1\"\n\
             monitor_login = \"pz7_monitor\"\nmonitor_credential_file = \"{credential}\"\n\n\
             [[node]]\nid = \"pg1\"\ncluster = \"main\"\nhost = \"127.0.0.1\"\nport = {port}\n\
             tls_ca_file = \"{ca}\"\ntls_server_name = \"pg.pool.test\"\n{extra}",
            state = self.state.display(),
            credential = self.credential.display(),
        );
        fs::write(&self.config, text).expect("write configuration");
    }

    fn journal(&self) -> PathBuf {
        self.state.join("journal.sqlite3")
    }

    /// The names in the state directory, sorted.
    fn entries(&self) -> Vec<String> {
        let mut names: Vec<String> = fs::read_dir(&self.state)
            .expect("read state directory")
            .map(|entry| {
                entry
                    .expect("entry")
                    .file_name()
                    .to_string_lossy()
                    .into_owned()
            })
            .collect();
        names.sort();
        names
    }
}

/// No output names a path of the test.
#[track_caller]
fn assert_no_path(run: &Run, layout: &Layout) {
    let root = layout.root.to_str().expect("utf-8");
    for text in [&run.stdout, &run.stderr] {
        assert!(!text.contains(root), "a path leaked: {text}");
        assert!(
            !text.contains("pz7-monitor-password"),
            "a secret leaked: {text}"
        );
    }
}

#[track_caller]
fn assert_error(layout: &Layout, name: &str, code: &str, key: Option<&str>, message: &str) {
    let human = command(name, &layout.config, "human");
    let at = key.map_or(String::new(), |key| format!(" at {key}"));
    assert_eq!(human.code, 3, "{} {}", human.stdout, human.stderr);
    assert_eq!(human.stdout, "");
    assert_eq!(human.stderr, format!("error: {code}{at}: {message}\n"));
    assert_no_path(&human, layout);
    let json = command(name, &layout.config, "json");
    assert_eq!(json.code, 3);
    assert_eq!(json.stderr, "");
    let key = key.map_or("null".to_owned(), |key| format!("\"{key}\""));
    assert_eq!(
        json.stdout,
        format!(
            "{{\"output_version\":1,\"command\":\"{name}\",\"result\":\"error\",\"config_version\":null,\"capabilities\":null,\"findings\":[{{\"code\":\"{code}\",\"key\":{key},\"line\":null,\"column\":null,\"message\":\"{message}\"}}]}}\n"
        )
    );
}

#[test]
fn usage_errors_of_the_new_commands_exit_2() {
    for args in [
        &["bootstrap"][..],
        &["show-authority", "--format", "xml", "--config", "p"],
        &["bootstrap", "--config", "p", "extra"],
        &["show-authority", "--config"],
        &["run", "--config", "p", "--format", "json"],
        &["run", "--format", "human", "--config", "p"],
        &["run"],
    ] {
        let run = pool(args);
        assert_eq!(run.code, 2, "{args:?}");
        assert_eq!(run.stdout, "");
        assert!(
            run.stderr.starts_with("error: "),
            "{args:?}: {}",
            run.stderr
        );
        assert!(run.stderr.ends_with("Run 'pool --help' for usage.\n"));
    }
}

#[test]
fn a_file_that_cannot_be_read_or_is_invalid_is_reported_with_the_command_name() {
    let layout = Layout::new("invalid");
    let missing = layout.root.join("absent.toml");
    for name in ["bootstrap", "show-authority"] {
        let run = command(name, &missing, "human");
        assert_eq!(run.code, 3);
        assert_eq!(
            run.stderr,
            "error: io.not_found: the configuration file does not exist\n"
        );
        let json = command(name, &missing, "json");
        assert!(json.stdout.starts_with(&format!(
            "{{\"output_version\":1,\"command\":\"{name}\",\"result\":\"error\""
        )));
    }
    fs::write(&layout.config, "config_version = 2\n").expect("write");
    for name in ["bootstrap", "show-authority"] {
        let run = command(name, &layout.config, "human");
        assert_eq!(run.code, 1);
        assert_eq!(
            run.stdout,
            "configuration invalid\nfindings: 1\nconfig.unsupported_version at config_version: this build supports config_version 1 only\n"
        );
        let json = command(name, &layout.config, "json");
        assert_eq!(json.code, 1);
        assert_eq!(
            json.stdout,
            format!(
                "{{\"output_version\":1,\"command\":\"{name}\",\"result\":\"invalid\",\"config_version\":null,\"capabilities\":null,\"findings\":[{{\"code\":\"config.unsupported_version\",\"key\":\"config_version\",\"line\":null,\"column\":null,\"message\":\"this build supports config_version 1 only\"}}]}}\n"
            )
        );
    }
    assert!(layout.entries().is_empty(), "nothing touched the state");
}

#[test]
fn each_command_reports_what_its_view_misses() {
    // covers: 0004 AC-8 through the binary. Bootstrap needs `probing()`.
    let layout = Layout::new("incomplete");
    let text = format!(
        "config_version = 1\n\n[control]\nstate_directory = \"{}\"\n\n[[cluster]]\nid = \"main\"\nsystem_identifier = \"7412345678901234567\"\nexpected_writer = \"pg1\"\n\n[[node]]\nid = \"pg1\"\ncluster = \"main\"\nhost = \"127.0.0.1\"\nport = 5432\n",
        layout.state.display()
    );
    fs::write(&layout.config, text).expect("write");
    let run = command("bootstrap", &layout.config, "human");
    assert_eq!(run.code, 1);
    assert_eq!(
        run.stdout,
        "configuration invalid\nfindings: 3\n\
         config.incomplete at cluster.monitor_login, line 6, column 1: a key that this command needs is missing\n\
         config.incomplete at cluster.monitor_credential_file, line 6, column 1: a key that this command needs is missing\n\
         config.incomplete at node.tls_ca_file, line 11, column 1: a key that this command needs is missing\n"
    );
    // `show-authority` needs the state directory only.
    let run = command("show-authority", &layout.config, "human");
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(run.stdout, "authority state: uninitialized\n");

    fs::write(&layout.config, "config_version = 1\n").expect("write");
    let run = command("show-authority", &layout.config, "human");
    assert_eq!(run.code, 1);
    assert_eq!(
        run.stdout,
        "configuration invalid\nfindings: 1\nconfig.incomplete at control.state_directory: a key that this command needs is missing\n"
    );
    let json = command("show-authority", &layout.config, "json");
    assert_eq!(
        json.stdout,
        "{\"output_version\":1,\"command\":\"show-authority\",\"result\":\"invalid\",\"config_version\":null,\"capabilities\":null,\"findings\":[{\"code\":\"config.incomplete\",\"key\":\"control.state_directory\",\"line\":null,\"column\":null,\"message\":\"a key that this command needs is missing\"}]}\n"
    );
}

#[test]
fn ac9_a_state_directory_that_does_not_exist_is_state_not_found() {
    // covers: 0004 AC-9.
    let layout = Layout::new("not-found");
    fs::remove_dir(&layout.state).expect("remove");
    for name in ["bootstrap", "show-authority"] {
        assert_error(
            &layout,
            name,
            "state.not_found",
            None,
            "the state directory does not exist",
        );
    }
    assert!(!layout.state.exists(), "no command creates it");
}

#[test]
fn ac9_a_state_directory_of_the_wrong_type_or_mode_is_state_permissions() {
    // covers: 0004 AC-9, SEC-4.
    for mode in [0o755, 0o750, 0o701, 0o710, 0o770, 0o500, 0o1700] {
        let layout = Layout::new(&format!("dir-mode-{mode:o}"));
        fs::set_permissions(&layout.state, fs::Permissions::from_mode(mode)).expect("chmod");
        for name in ["bootstrap", "show-authority"] {
            assert_error(&layout, name, "state.permissions", None, STATE_PERMISSIONS);
        }
        fs::set_permissions(&layout.state, fs::Permissions::from_mode(0o700)).expect("chmod");
        assert!(layout.entries().is_empty(), "{mode:o}: nothing was created");
    }
    // A regular file, and a link to a good directory.
    let layout = Layout::new("dir-file");
    fs::remove_dir(&layout.state).expect("remove");
    write_mode(&layout.state, b"", 0o700);
    assert_error(
        &layout,
        "show-authority",
        "state.permissions",
        None,
        STATE_PERMISSIONS,
    );
    let layout = Layout::new("dir-link");
    let real = layout.root.join("real-state");
    fs::rename(&layout.state, &real).expect("move");
    symlink(&real, &layout.state).expect("link");
    assert_error(
        &layout,
        "show-authority",
        "state.permissions",
        None,
        STATE_PERMISSIONS,
    );
}

#[test]
fn ac9_a_journal_file_of_the_wrong_type_or_mode_is_state_permissions() {
    // covers: 0004 AC-9. Each of the four files, when it exists.
    for name in [
        "pool.lock",
        "journal.sqlite3",
        "journal.sqlite3-wal",
        "journal.sqlite3-shm",
    ] {
        for mode in [0o644, 0o640, 0o604, 0o400, 0o700] {
            let layout = Layout::new(&format!("file-{name}-{mode:o}"));
            write_mode(&layout.state.join(name), b"", mode);
            for command_name in ["bootstrap", "show-authority"] {
                assert_error(
                    &layout,
                    command_name,
                    "state.permissions",
                    None,
                    STATE_PERMISSIONS,
                );
            }
        }
        let layout = Layout::new(&format!("file-{name}-dir"));
        fs::create_dir(layout.state.join(name)).expect("folder");
        assert_error(
            &layout,
            "show-authority",
            "state.permissions",
            None,
            STATE_PERMISSIONS,
        );
        let layout = Layout::new(&format!("file-{name}-link"));
        let target = layout.root.join("target-file");
        write_mode(&target, b"", 0o600);
        symlink(&target, layout.state.join(name)).expect("link");
        assert_error(
            &layout,
            "show-authority",
            "state.permissions",
            None,
            STATE_PERMISSIONS,
        );
    }
    // Any other entry is not looked at.
    let layout = Layout::new("other-entry");
    write_mode(&layout.state.join("notes.txt"), b"x", 0o644);
    fs::create_dir(layout.state.join("old")).expect("folder");
    let run = command("show-authority", &layout.config, "human");
    assert_eq!(run.code, 0, "{}", run.stderr);
}

#[test]
fn ac9_a_second_command_while_the_lock_is_held_is_state_locked_and_changes_nothing() {
    // covers: 0004 AC-9. The test holds the lock as another process would.
    let layout = Layout::new("locked");
    let lock_path = layout.state.join("pool.lock");
    write_mode(&lock_path, b"", 0o600);
    write_mode(&layout.journal(), b"", 0o600);
    let lock = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&lock_path)
        .expect("open the lock");
    lock.try_lock().expect("take the lock");
    let before = layout.entries();
    for name in ["bootstrap", "show-authority"] {
        assert_error(
            &layout,
            name,
            "state.locked",
            None,
            "another process is using the state directory",
        );
    }
    assert_eq!(layout.entries(), before);
    assert_eq!(fs::metadata(layout.journal()).expect("journal").len(), 0);
    drop(lock);
    let run = command("show-authority", &layout.config, "human");
    assert_eq!(run.code, 0, "{}", run.stderr);
}

#[test]
fn show_authority_reports_each_state_it_can_reach_without_a_journal() {
    // covers: 0004 AC-11 (hermetic part), 0003 AC-7.
    let layout = Layout::new("show-states");
    let run = command("show-authority", &layout.config, "human");
    assert_eq!(
        (run.code, run.stdout.as_str()),
        (0, "authority state: uninitialized\n")
    );
    assert_eq!(layout.entries(), ["pool.lock"], "only the lock is created");
    let mode = fs::metadata(layout.state.join("pool.lock"))
        .expect("lock")
        .permissions()
        .mode();
    assert_eq!(mode & 0o7777, 0o600);
    let json = command("show-authority", &layout.config, "json");
    assert_eq!(
        json.stdout,
        "{\"output_version\":1,\"command\":\"show-authority\",\"result\":\"uninitialized\",\"authority_id\":null,\"findings\":[]}\n"
    );

    // An empty journal file is what an interrupted bootstrap leaves: absent.
    write_mode(&layout.journal(), b"", 0o600);
    let run = command("show-authority", &layout.config, "human");
    assert_eq!(run.stdout, "authority state: uninitialized\n");

    // A witness without a journal.
    fs::remove_file(layout.journal()).expect("remove");
    layout.write_config(Some(X), closed_port(), CA, "");
    let run = command("show-authority", &layout.config, "human");
    assert_eq!(
        (run.code, run.stdout.as_str()),
        (0, "authority state: missing\n")
    );

    // A file that is not a journal.
    write_mode(
        &layout.journal(),
        b"not a database at all, only its name",
        0o600,
    );
    let before = fs::read(layout.journal()).expect("read");
    let run = command("show-authority", &layout.config, "human");
    assert_eq!(
        (run.code, run.stdout.as_str()),
        (0, "authority state: unreadable\n")
    );
    assert_eq!(fs::read(layout.journal()).expect("read"), before);
    let json = command("show-authority", &layout.config, "json");
    assert_eq!(
        json.stdout,
        "{\"output_version\":1,\"command\":\"show-authority\",\"result\":\"unreadable\",\"authority_id\":null,\"findings\":[]}\n"
    );
    assert_no_path(&run, &layout);
}

#[test]
fn ac9_a_secret_or_ca_file_that_cannot_be_used_ends_bootstrap_with_its_key() {
    // covers: 0004 AC-9, SEC-4. The finding names the key, never the path.
    let layout = Layout::new("secrets");
    for mode in [0o640, 0o604, 0o644, 0o660] {
        fs::set_permissions(&layout.credential, fs::Permissions::from_mode(mode)).expect("chmod");
        assert_error(
            &layout,
            "bootstrap",
            "secret.permissions",
            Some("cluster.monitor_credential_file"),
            "a secret file has the wrong type, mode or owner",
        );
    }
    fs::set_permissions(&layout.credential, fs::Permissions::from_mode(0o400)).expect("chmod");
    fs::remove_file(&layout.credential).expect("remove");
    assert_error(
        &layout,
        "bootstrap",
        "secret.not_found",
        Some("cluster.monitor_credential_file"),
        "a file named by the configuration does not exist",
    );
    write_mode(&layout.credential, b"pz7-monitor-password\n", 0o600);

    let missing_ca = layout.root.join("absent-ca.pem");
    layout.write_config(None, closed_port(), missing_ca.to_str().expect("utf-8"), "");
    assert_error(
        &layout,
        "bootstrap",
        "secret.not_found",
        Some("node.tls_ca_file"),
        "a file named by the configuration does not exist",
    );
    let bad_ca = layout.root.join("bad-ca.pem");
    write_mode(&bad_ca, b"this is not a certificate\n", 0o644);
    layout.write_config(None, closed_port(), bad_ca.to_str().expect("utf-8"), "");
    assert_error(
        &layout,
        "bootstrap",
        "secret.invalid",
        Some("node.tls_ca_file"),
        "a file named by the configuration does not hold what its key needs",
    );
    assert_eq!(layout.entries(), ["pool.lock"], "no journal file");
}

#[test]
fn ac11_bootstrap_against_a_node_that_does_not_answer_creates_no_journal() {
    // covers: 0004 AC-11 (hermetic part), AC-12. A closed port, then a
    // listener that accepts and never speaks: both within the timeout.
    let layout = Layout::new("no-answer");
    let expected =
        format!("bootstrap refused\nfindings: 1\nbootstrap.evidence_missing: {REFUSED}\n");
    let run = command("bootstrap", &layout.config, "human");
    assert_eq!(run.code, 1, "{}", run.stderr);
    assert_eq!(run.stdout, expected);
    assert_eq!(run.stderr, "");
    assert!(!layout.journal().exists());

    let silent = TcpListener::bind("127.0.0.1:0").expect("bind");
    let port = silent.local_addr().expect("address").port();
    layout.write_config(None, port, CA, "");
    let started = Instant::now();
    let json = command("bootstrap", &layout.config, "json");
    let elapsed = started.elapsed();
    assert_eq!(json.code, 1);
    assert_eq!(
        json.stdout,
        format!(
            "{{\"output_version\":1,\"command\":\"bootstrap\",\"result\":\"refused\",\"config_version\":null,\"capabilities\":null,\"findings\":[{{\"code\":\"bootstrap.evidence_missing\",\"key\":null,\"line\":null,\"column\":null,\"message\":\"{REFUSED}\"}}]}}\n"
        )
    );
    assert!(elapsed < Duration::from_secs(5), "took {elapsed:?}");
    assert!(!layout.journal().exists());
    assert_eq!(layout.entries(), ["pool.lock"]);
    assert_no_path(&json, &layout);
    drop(silent);

    // With a witness, bootstrap is refused for that too, after the probe.
    layout.write_config(Some(X), closed_port(), CA, "");
    let run = command("bootstrap", &layout.config, "human");
    assert_eq!(run.code, 1);
    assert_eq!(
        run.stdout,
        format!(
            "bootstrap refused\nfindings: 2\nbootstrap.witness_present: {REFUSED}\nbootstrap.evidence_missing: {REFUSED}\n"
        )
    );
    assert!(!layout.journal().exists());
}

// `pool run` in the states that do not serve (`0004 AC-13`). Step 3 ends
// before any protected file is read, so the files the configuration names
// need not exist, and nothing ever binds.

impl Layout {
    /// A configuration that `serving()` accepts, with a listener on `port`.
    fn write_serving_config(&self, witness: Option<&str>, port: u16) {
        let extra = format!(
            "\n[listener]\nhost = \"127.0.0.1\"\nport = {port}\n\
             tls_certificate_file = \"{root}/pool.pem\"\ntls_private_key_file = \"{root}/pool.key\"\n\n\
             [[principal]]\nid = \"it-principal\"\nname = \"pz7_principal\"\nverifier_file = \"{root}/principal.verifier\"\n\n\
             [[identity]]\nid = \"it-identity\"\nprincipal = \"pz7_principal\"\ndatabase = \"pz7_db\"\ncluster = \"main\"\n\
             backend_database = \"pz7_appdb\"\nbackend_login = \"pz7_rw\"\ncredential_file = \"{root}/rw.password\"\n",
            root = self.root.display(),
        );
        self.write_config(witness, closed_port(), CA, &extra);
    }
}

/// `pool run` on the layout; it has no `--format`.
fn run_daemon(layout: &Layout) -> Run {
    pool(&[
        "run",
        "--config",
        layout.config.to_str().expect("utf-8 path"),
    ])
}

#[track_caller]
fn assert_not_serving(layout: &Layout, code: &str, port: u16) {
    let started = Instant::now();
    let run = run_daemon(layout);
    assert_eq!(run.code, 1, "{} {}", run.stdout, run.stderr);
    assert_eq!(
        run.stdout,
        format!("run refused\nfindings: 1\n{code}: the journal state does not allow serving\n")
    );
    assert_eq!(run.stderr, "");
    assert!(started.elapsed() < Duration::from_secs(5));
    assert_no_path(&run, layout);
    // The listener was never bound.
    TcpListener::bind(("127.0.0.1", port)).expect("the port is still free");
}

#[test]
fn ac13_run_refuses_every_state_it_can_reach_without_a_journal_and_never_binds() {
    // covers: 0004 AC-13. Uninitialized, missing and unreadable here;
    // unconfirmed, mismatch and ready need a real journal (integration.rs).
    let layout = Layout::new("run-states");
    let port = closed_port();
    layout.write_serving_config(None, port);
    assert_not_serving(&layout, "authority.uninitialized", port);
    assert!(!layout.journal().exists(), "run never creates the journal");
    assert_eq!(layout.entries(), ["pool.lock"]);

    write_mode(&layout.journal(), b"", 0o600);
    assert_not_serving(&layout, "authority.uninitialized", port);
    assert_eq!(fs::metadata(layout.journal()).expect("journal").len(), 0);

    fs::remove_file(layout.journal()).expect("remove");
    layout.write_serving_config(Some(X), port);
    assert_not_serving(&layout, "authority.missing", port);

    write_mode(
        &layout.journal(),
        b"not a database at all, only its name",
        0o600,
    );
    let before = fs::read(layout.journal()).expect("read");
    assert_not_serving(&layout, "authority.unreadable", port);
    assert_eq!(
        fs::read(layout.journal()).expect("read"),
        before,
        "unchanged"
    );
}

#[test]
fn ac13_run_reports_the_configuration_the_view_and_the_state_directory_first() {
    // covers: 0004 AC-8, AC-9 through `pool run`.
    let layout = Layout::new("run-first");
    let run = run_daemon(&layout);
    assert_eq!(run.code, 1);
    assert_eq!(
        run.stdout,
        "configuration invalid\nfindings: 2\n\
         config.incomplete at listener.host: a key that this command needs is missing\n\
         config.incomplete at identity.id: a key that this command needs is missing\n"
    );
    layout.write_serving_config(None, closed_port());
    fs::set_permissions(&layout.state, fs::Permissions::from_mode(0o750)).expect("chmod");
    let run = run_daemon(&layout);
    assert_eq!(run.code, 3);
    assert_eq!(run.stdout, "");
    assert_eq!(
        run.stderr,
        format!("error: state.permissions: {STATE_PERMISSIONS}\n")
    );
    fs::set_permissions(&layout.state, fs::Permissions::from_mode(0o700)).expect("chmod");

    let lock_path = layout.state.join("pool.lock");
    write_mode(&lock_path, b"", 0o600);
    let lock = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&lock_path)
        .expect("open the lock");
    lock.try_lock().expect("take the lock");
    let run = run_daemon(&layout);
    assert_eq!(run.code, 3);
    assert_eq!(
        run.stderr,
        "error: state.locked: another process is using the state directory\n"
    );
    drop(lock);

    let missing = layout.root.join("absent.toml");
    let run = pool(&["run", "--config", missing.to_str().expect("utf-8")]);
    assert_eq!(run.code, 3);
    assert_eq!(
        run.stderr,
        "error: io.not_found: the configuration file does not exist\n"
    );
}
