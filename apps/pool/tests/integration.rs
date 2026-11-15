//! The tests on the fixture of spec 0004, "The integration check": real
//! PostgreSQL 18 in the containers that `scripts/check.sh integration`
//! starts. Build plan step 8: bootstrap, a second bootstrap,
//! `show-authority`, a node that cannot be reached, and the monitor login
//! (`0004 AC-10` to `AC-12`).
//!
//! Every test is ignored by a plain `cargo test`, so `scripts/check.sh test`
//! needs no Docker. The script runs them with `--ignored --test-threads=1`.
//! They contact only the addresses it exports, and run `docker exec` and
//! `docker logs` only on its two containers.

use std::fs;
use std::net::TcpListener;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use pool_config::AuthorityId;
use pool_control::{AuthorityState, Journal};

mod support;

const OTHER_CA: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../crates/pool-core/tests/fixtures/other-ca.pem"
);
const REFUSED: &str = "bootstrap is not possible in this state";

/// A copy of [`OTHER_CA`] with mode 0644. A tree from `git archive` gives
/// the committed file mode 0664, which the rule of decision 88 refuses
/// before any TLS, and these tests are about the TLS check.
fn other_ca_copy() -> PathBuf {
    let path = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join("pool-it-other-ca.pem");
    fs::write(&path, fs::read(OTHER_CA).expect("read the other CA")).expect("write");
    fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).expect("chmod");
    path
}

fn var(name: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| panic!("{name} is set by scripts/check.sh integration"))
}

fn fixture_dir() -> PathBuf {
    PathBuf::from(var("POOL_IT_DIR"))
}

struct Run {
    code: i32,
    stdout: String,
    stderr: String,
}

fn pool(name: &str, config: &Path, format: &str) -> Run {
    let output = Command::new(env!("CARGO_BIN_EXE_pool"))
        .args([
            name,
            "--config",
            config.to_str().expect("utf-8 path"),
            "--format",
            format,
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("run pool");
    let output = output_within(output, 60, &format!("pool {name}"));
    Run {
        code: output.status.code().expect("exit status, not a signal"),
        stdout: String::from_utf8(output.stdout).expect("utf-8"),
        stderr: String::from_utf8(output.stderr).expect("utf-8"),
    }
}

/// `psql` as the superuser inside a container of the fixture, over its
/// local socket.
fn psql(container: &str, sql: &str) -> String {
    let output = Command::new("docker")
        .args([
            "exec",
            container,
            "psql",
            "-At",
            "-v",
            "ON_ERROR_STOP=1",
            "-U",
            "postgres",
            "-d",
            "postgres",
            "-c",
            sql,
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("docker exec");
    let output = output_within(output, 60, "psql as the superuser");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .expect("utf-8")
        .trim()
        .to_owned()
}

fn system_identifier(container: &str) -> String {
    psql(
        container,
        "SELECT system_identifier FROM pg_control_system()",
    )
}

fn logs(container: &str) -> String {
    let output = Command::new("docker")
        .args(["logs", container])
        .output()
        .expect("docker logs");
    format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    )
}

/// A port on 127.0.0.1 that nothing listens on.
fn closed_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    listener.local_addr().expect("address").port()
}

/// Waits at most `seconds` for a child and returns its output (round four,
/// review finding t4). A child that still runs then is killed and the test
/// fails with a message that names `what`. The children of these tests write
/// a few lines, far below what a pipe holds, so polling before reading the
/// pipes cannot block the child.
fn output_within(mut child: std::process::Child, seconds: u64, what: &str) -> std::process::Output {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(seconds);
    while child.try_wait().expect("try_wait").is_none() {
        if std::time::Instant::now() >= deadline {
            let _ = child.kill();
            let _ = child.wait();
            panic!("{what} did not end within {seconds} s and was killed");
        }
        pause();
    }
    child.wait_with_output().expect("the output of a child")
}

/// One node of the configuration.
struct NodeAt {
    id: &'static str,
    port: u16,
}

/// The folder, the state directory and the configuration of one test.
struct Case {
    root: PathBuf,
    state: PathBuf,
    config: PathBuf,
}

/// What a configuration of a test holds beyond its nodes.
struct Settings<'a> {
    witness: Option<&'a str>,
    system_identifier: String,
    credential: PathBuf,
    ca: PathBuf,
    server_name: &'a str,
}

impl Case {
    fn new(name: &str) -> Case {
        let root = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("pool-it-{name}"));
        if root.exists() {
            fs::remove_dir_all(&root).expect("remove old test folder");
        }
        let state = root.join("state");
        fs::create_dir_all(&state).expect("create");
        fs::set_permissions(&root, fs::Permissions::from_mode(0o700)).expect("chmod");
        fs::set_permissions(&state, fs::Permissions::from_mode(0o700)).expect("chmod");
        Case {
            config: root.join("pool.toml"),
            root,
            state,
        }
    }

    fn settings(&self) -> Settings<'static> {
        Settings {
            witness: None,
            system_identifier: system_identifier(&var("POOL_IT_CONTAINER")),
            credential: fixture_dir().join("monitor.password"),
            ca: fixture_dir().join("certs/ca.pem"),
            server_name: "pg.pool.test",
        }
    }

    fn write(&self, settings: &Settings<'_>, nodes: &[NodeAt]) {
        let witness = settings
            .witness
            .map_or(String::new(), |id| format!("authority_id = \"{id}\"\n"));
        let mut text = format!(
            "config_version = 1\n\n[control]\nstate_directory = \"{}\"\n{witness}\n\
             [[cluster]]\nid = \"main\"\nsystem_identifier = \"{}\"\nexpected_writer = \"pg1\"\n\
             monitor_login = \"pz7_monitor\"\nmonitor_credential_file = \"{}\"\n",
            self.state.display(),
            settings.system_identifier,
            settings.credential.display(),
        );
        for node in nodes {
            text.push_str(&format!(
                "\n[[node]]\nid = \"{}\"\ncluster = \"main\"\nhost = \"127.0.0.1\"\nport = {}\n\
                 tls_ca_file = \"{}\"\ntls_server_name = \"{}\"\n",
                node.id,
                node.port,
                settings.ca.display(),
                settings.server_name,
            ));
        }
        fs::write(&self.config, text).expect("write the configuration");
    }

    fn journal(&self) -> PathBuf {
        self.state.join("journal.sqlite3")
    }

    fn assert_quiet(&self, run: &Run) {
        let root = self.root.to_str().expect("utf-8");
        let dir = fixture_dir();
        let dir = dir.to_str().expect("utf-8");
        for text in [&run.stdout, &run.stderr] {
            for leak in [root, dir, "pz7_", "pz7-", "127.0.0.1", "pg.pool.test"] {
                assert!(!text.contains(leak), "{leak:?} reached the output: {text}");
            }
        }
    }
}

fn primary_node() -> NodeAt {
    NodeAt {
        id: "pg1",
        port: var("POOL_IT_PG_PORT").parse().expect("port"),
    }
}

/// Runs bootstrap and returns the authority ID it printed.
fn bootstrap(case: &Case) -> AuthorityId {
    let run = pool("bootstrap", &case.config, "human");
    assert_eq!(run.code, 0, "stdout {} stderr {}", run.stdout, run.stderr);
    assert_eq!(run.stderr, "");
    let lines: Vec<&str> = run.stdout.lines().collect();
    assert_eq!(lines.len(), 4, "{}", run.stdout);
    assert_eq!(lines[0], "bootstrap complete");
    assert_eq!(lines[2], "clusters: 1");
    assert_eq!(
        lines[3],
        "next: add control.authority_id to the configuration"
    );
    let id = lines[1]
        .strip_prefix("authority_id: ")
        .expect("an authority ID line");
    case.assert_quiet(&run);
    AuthorityId::parse(id).expect("32 lowercase hex characters")
}

#[test]
#[ignore = "needs scripts/check.sh integration"]
fn it_bootstrap_writes_the_journal_and_show_authority_reads_it_back() {
    // covers: 0004 AC-10, AC-11, AC-12, 0001 SC-8.
    let case = Case::new("bootstrap");
    let mut settings = case.settings();
    case.write(&settings, &[primary_node()]);
    assert!(!case.journal().exists());
    let id = bootstrap(&case);

    // The file: mode 0600, the marks of spec 0003 and WAL mode.
    let mode = fs::metadata(case.journal())
        .expect("journal")
        .permissions()
        .mode();
    assert_eq!(mode & 0o7777, 0o600);
    let bytes = fs::read(case.journal()).expect("read the journal");
    assert_eq!(&bytes[..16], b"SQLite format 3\0");
    assert_eq!((bytes[18], bytes[19]), (2, 2), "WAL mode");
    assert_eq!(u32::from_be_bytes(bytes[60..64].try_into().expect("4")), 1);
    assert_eq!(
        u32::from_be_bytes(bytes[68..72].try_into().expect("4")),
        1_347_374_924
    );

    // `show-authority` before and after the witness is in the configuration.
    let run = pool("show-authority", &case.config, "human");
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(
        run.stdout,
        format!("authority state: unconfirmed\nauthority_id: {id}\n")
    );
    case.assert_quiet(&run);
    let id_text = id.to_string();
    settings.witness = Some(&id_text);
    case.write(&settings, &[primary_node()]);
    let before = fs::read(case.journal()).expect("read");
    let json = pool("show-authority", &case.config, "json");
    assert_eq!(
        json.stdout,
        format!(
            "{{\"output_version\":1,\"command\":\"show-authority\",\"result\":\"ready\",\"authority_id\":\"{id}\",\"findings\":[]}}\n"
        )
    );
    assert_eq!(fs::read(case.journal()).expect("read"), before, "read only");

    // The reader of pool-control sees revision 1 and start sequence 0.
    let config = pool_config::validate(&fs::read(&case.config).expect("config")).expect("valid");
    match Journal::read(&case.journal()).classify(&config) {
        AuthorityState::Ready(authority) => {
            assert_eq!(authority.authority_id(), id);
            assert_eq!(authority.authority_revision().get(), 1);
            assert_eq!(authority.start_sequence().get(), 0);
        }
        other => panic!("expected ready, got {}", other.name()),
    }

    // A second bootstrap probes and is refused; the journal stays.
    let run = pool("bootstrap", &case.config, "human");
    assert_eq!(run.code, 1);
    assert_eq!(
        run.stdout,
        format!(
            "bootstrap refused\nfindings: 2\nbootstrap.store_present: {REFUSED}\nbootstrap.witness_present: {REFUSED}\n"
        )
    );
    settings.witness = None;
    case.write(&settings, &[primary_node()]);
    let json = pool("bootstrap", &case.config, "json");
    assert_eq!(json.code, 1);
    assert_eq!(
        json.stdout,
        format!(
            "{{\"output_version\":1,\"command\":\"bootstrap\",\"result\":\"refused\",\"config_version\":null,\"capabilities\":null,\"findings\":[{{\"code\":\"bootstrap.store_present\",\"key\":null,\"line\":null,\"column\":null,\"message\":\"{REFUSED}\"}}]}}\n"
        )
    );
    assert_eq!(fs::read(case.journal()).expect("read"), before);
    assert_eq!(Journal::read(&case.journal()).authority_id(), Some(id));
}

#[test]
#[ignore = "needs scripts/check.sh integration"]
fn it_bootstrap_json_prints_the_new_id() {
    // covers: 0004 AC-11.
    let case = Case::new("bootstrap-json");
    case.write(&case.settings(), &[primary_node()]);
    let run = pool("bootstrap", &case.config, "json");
    assert_eq!(run.code, 0, "{}", run.stderr);
    let prefix = "{\"output_version\":1,\"command\":\"bootstrap\",\"result\":\"created\",\"authority_id\":\"";
    let rest = run.stdout.strip_prefix(prefix).expect("the JSON prefix");
    let (id, tail) = rest.split_at(32);
    assert!(AuthorityId::parse(id).is_some(), "{id}");
    assert_eq!(tail, "\",\"clusters\":1,\"findings\":[]}\n");
    assert_eq!(
        Journal::read(&case.journal()).authority_id(),
        AuthorityId::parse(id)
    );
}

#[test]
#[ignore = "needs scripts/check.sh integration"]
fn it_a_node_that_cannot_be_reached_refuses_bootstrap_and_writes_no_file() {
    // covers: 0004 AC-11, AC-12.
    let case = Case::new("unreachable");
    case.write(
        &case.settings(),
        &[
            primary_node(),
            NodeAt {
                id: "pg2",
                port: closed_port(),
            },
        ],
    );
    let run = pool("bootstrap", &case.config, "human");
    assert_eq!(run.code, 1, "{}", run.stderr);
    assert_eq!(
        run.stdout,
        format!("bootstrap refused\nfindings: 1\nbootstrap.evidence_missing: {REFUSED}\n")
    );
    assert!(!case.journal().exists(), "no journal file");
    case.assert_quiet(&run);
}

#[test]
#[ignore = "needs scripts/check.sh integration"]
fn it_the_probe_logs_in_as_the_monitor_login_over_tls_and_asks_one_question() {
    // covers: 0004 AC-12, SEC-5, T-03.
    let container = var("POOL_IT_CONTAINER");
    let attributes = psql(
        &container,
        "SELECT rolsuper, rolreplication, rolcreaterole, rolcreatedb, rolbypassrls, rolcanlogin \
         FROM pg_roles WHERE rolname = 'pz7_monitor'",
    );
    assert_eq!(attributes, "f|f|f|f|f|t", "LOGIN and nothing else");
    assert_eq!(
        psql(
            &container,
            "SELECT count(*) FROM pg_auth_members m JOIN pg_roles r ON r.oid = m.member \
             WHERE r.rolname = 'pz7_monitor'"
        ),
        "0",
        "member of no role"
    );
    let case = Case::new("monitor");
    case.write(&case.settings(), &[primary_node()]);
    let before = logs(&container).len();
    bootstrap(&case);
    let log = logs(&container);
    let new = &log[before.min(log.len())..];
    assert!(
        new.contains("connection authorized: user=pz7_monitor database=postgres application_name=pool-probe SSL enabled"),
        "{new}"
    );
    assert!(new.contains("pg_control_system()"), "{new}");
    assert!(new.contains("pg_current_wal_insert_lsn()"), "{new}");
}

#[test]
#[ignore = "needs scripts/check.sh integration"]
fn it_a_wrong_password_ca_or_server_name_is_a_failed_probe() {
    // covers: 0004 AC-12. Each one gives no evidence and no journal.
    let wrong = Case::new("wrong-password");
    let password = wrong.root.join("wrong.password");
    fs::write(&password, "pz7-not-the-password\n").expect("write");
    fs::set_permissions(&password, fs::Permissions::from_mode(0o600)).expect("chmod");
    let mut settings = wrong.settings();
    settings.credential = password;
    wrong.write(&settings, &[primary_node()]);

    let other_ca = Case::new("wrong-ca");
    let mut ca_settings = other_ca.settings();
    ca_settings.ca = other_ca_copy();
    other_ca.write(&ca_settings, &[primary_node()]);

    let other_name = Case::new("wrong-name");
    let mut name_settings = other_name.settings();
    name_settings.server_name = "other.pool.test";
    other_name.write(&name_settings, &[primary_node()]);

    for case in [&wrong, &other_ca, &other_name] {
        let run = pool("bootstrap", &case.config, "human");
        assert_eq!(run.code, 1, "{}", run.stderr);
        assert_eq!(
            run.stdout,
            format!("bootstrap refused\nfindings: 1\nbootstrap.evidence_missing: {REFUSED}\n")
        );
        assert!(!case.journal().exists());
        case.assert_quiet(&run);
    }
}

#[test]
#[ignore = "needs scripts/check.sh integration"]
fn it_the_second_server_is_another_identity() {
    // covers: 0004 AC-12. Same certificate, roles and passwords, so the
    // probe reaches the identity rule and reads another system identifier.
    let first = system_identifier(&var("POOL_IT_CONTAINER"));
    let second = system_identifier(&var("POOL_IT_CONTAINER_2"));
    assert_ne!(first, second);
    let case = Case::new("identity");
    case.write(
        &case.settings(),
        &[NodeAt {
            id: "pg1",
            port: var("POOL_IT_PG_PORT_2").parse().expect("port"),
        }],
    );
    let run = pool("bootstrap", &case.config, "human");
    assert_eq!(run.code, 1, "{}", run.stderr);
    assert_eq!(
        run.stdout,
        format!("bootstrap refused\nfindings: 1\nbootstrap.identity_mismatch: {REFUSED}\n")
    );
    assert!(!case.journal().exists());
}

// Build plan step 10: `pool run` on the walking fixture (`0004 AC-13`,
// `AC-14`, `AC-18`, `AC-19`, `AC-24`, `AC-25`). The daemon writes its log to
// a file of the test, which the test reads; nothing in it may name a
// principal, a login, a database, a host, a path, a password or SQL.

/// The SQL of AC-18: the same file, straight to PostgreSQL and through POOL.
const WALKING_SQL: &str = "\\set VERBOSITY verbose
TRUNCATE pz7_items;
BEGIN;
INSERT INTO pz7_items (key, value) VALUES (1, 'one'), (2, 'two'), (3, 'three');
COMMIT;
SELECT key, value FROM pz7_items ORDER BY key;
SELECT 1 / 0;
";

fn password(name: &str) -> String {
    fs::read_to_string(fixture_dir().join(name))
        .expect("a password file of the fixture")
        .trim_end()
        .to_owned()
}

/// A port on the listen host that is free now.
fn free_port(host: &str) -> u16 {
    let listener = TcpListener::bind((host, 0)).expect("bind");
    listener.local_addr().expect("address").port()
}

impl Case {
    /// The configuration of the walking fixture: the cluster and the node of
    /// container 1, a listener, the principal and the identity entry.
    fn write_serving(&self, witness: Option<&str>, listen_port: u16) {
        self.write_serving_with(witness, listen_port, primary_node(), None, "");
    }

    /// The same, with another node port (the relay or container 2), another
    /// CA for the node, and more tables, such as small `[limits]`.
    fn write_serving_with(
        &self,
        witness: Option<&str>,
        listen_port: u16,
        node: NodeAt,
        ca: Option<PathBuf>,
        extra: &str,
    ) {
        let dir = fixture_dir();
        let mut settings = self.settings();
        settings.witness = witness;
        if let Some(ca) = ca {
            settings.ca = ca;
        }
        self.write(&settings, &[node]);
        let mut text = fs::read_to_string(&self.config).expect("read");
        text.push_str(&format!(
            "\n[listener]\nhost = \"{host}\"\nport = {listen_port}\n\
             tls_certificate_file = \"{certs}/host.docker.internal.pem\"\ntls_private_key_file = \"{dir}/pool.key\"\n\n\
             [probe]\nperiod_ms = 200\ntimeout_ms = 5000\nobservation_expiry_ms = 8000\n\n\
             [[principal]]\nid = \"it-principal\"\nname = \"pz7_principal\"\nverifier_file = \"{dir}/principal.verifier\"\n\n\
             [[identity]]\nid = \"it-identity\"\nprincipal = \"pz7_principal\"\ndatabase = \"pz7_db\"\ncluster = \"main\"\n\
             backend_database = \"pz7_appdb\"\nbackend_login = \"pz7_rw\"\ncredential_file = \"{dir}/rw.password\"\n",
            host = var("POOL_IT_LISTEN_HOST"),
            certs = dir.join("certs").display(),
            dir = dir.display(),
        ));
        text.push_str(extra);
        fs::write(&self.config, text).expect("write");
    }

    fn log_file(&self) -> PathBuf {
        self.root.join("pool.log")
    }
}

/// Waits 50 ms. The source guard keeps threads out of the binary and its
/// tests, so a small runtime of the test does the waiting.
fn pause() {
    tokio::runtime::Builder::new_current_thread()
        .enable_time()
        .build()
        .expect("a runtime for the test")
        .block_on(async { tokio::time::sleep(std::time::Duration::from_millis(50)).await });
}

/// Asks `check` every 50 ms, for at most `seconds`, until it is true.
fn eventually(seconds: u64, what: &str, mut check: impl FnMut() -> bool) {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(seconds);
    while !check() {
        assert!(std::time::Instant::now() < deadline, "never true: {what}");
        pause();
    }
}

/// A running `pool run`, killed when it is dropped without `stop`.
struct Daemon {
    child: std::process::Child,
    log: PathBuf,
    /// The folder of the test, which the log must never name.
    root: PathBuf,
}

impl Daemon {
    fn start(case: &Case) -> Daemon {
        let log = case.log_file();
        let stderr = fs::File::create(&log).expect("log file");
        let stdout = fs::File::create(case.root.join("pool.stdout")).expect("stdout file");
        let child = Command::new(env!("CARGO_BIN_EXE_pool"))
            .args(["run", "--config", case.config.to_str().expect("utf-8")])
            .stdin(Stdio::null())
            .stdout(stdout)
            .stderr(stderr)
            .spawn()
            .expect("start pool run");
        Daemon {
            child,
            log,
            root: case.root.clone(),
        }
    }

    fn log(&self) -> String {
        fs::read_to_string(&self.log).unwrap_or_default()
    }

    /// Waits, asking every 50 ms, until a log line holds every part.
    fn wait_for(&self, parts: &[&str], seconds: u64) -> String {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(seconds);
        loop {
            let log = self.log();
            if let Some(line) = log
                .lines()
                .find(|line| parts.iter().all(|part| line.contains(part)))
            {
                return line.to_owned();
            }
            assert!(
                std::time::Instant::now() < deadline,
                "no line with {parts:?} in:\n{log}"
            );
            pause();
        }
    }

    /// Waits for the exit status after a signal the test sent itself, at
    /// most 45 seconds, and checks the whole log.
    fn stop_after_signal(self) -> i32 {
        self.finish("after the signal that the test sent", 45)
    }

    /// Sends SIGTERM, waits for the exit status at most 45 seconds (more than
    /// the default `limits.shutdown_timeout_ms` of 30 seconds), and checks the
    /// whole log.
    fn stop(self) -> i32 {
        self.terminate();
        self.finish("after SIGTERM from Daemon::stop", 45)
    }

    fn terminate(&self) {
        let status = Command::new("kill")
            .args(["-TERM", &self.child.id().to_string()])
            .status()
            .expect("kill");
        assert!(status.success());
    }

    /// Waits at most `seconds` for the daemon to exit. One that is still
    /// running then is killed and the test fails with a message that names
    /// `stage`. After the exit, every line of the log must be an event of the
    /// table with only its fields and name nothing of the fixture (round three
    /// of the tests: every run of the daemon is checked, also those that make
    /// the probe fail).
    fn finish(mut self, stage: &str, seconds: u64) -> i32 {
        let deadline = std::time::Instant::now() + std::time::Duration::from_secs(seconds);
        let status = loop {
            if let Some(status) = self.child.try_wait().expect("try_wait") {
                break status;
            }
            if std::time::Instant::now() >= deadline {
                let _ = self.child.kill();
                let _ = self.child.wait();
                panic!(
                    "the daemon did not exit within {seconds} s {stage}; it was killed. Its log:\n{}",
                    self.log()
                );
            }
            pause();
        };
        let log = self.log();
        assert_log_fields(&log);
        assert_log_text_is_clean(&self.root, &log);
        let code = status.code().expect("an exit status, not a signal");
        std::mem::forget(self);
        code
    }
}

impl Drop for Daemon {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

/// `psql` of the image inside container 1, as a client, with the SQL file on
/// its standard input. Returns the exit status, stdout and stderr.
fn client(conninfo: &str, password: &str, sql: &str) -> (i32, String, String) {
    use std::io::Write;
    let mut child = Command::new("docker")
        .args([
            "exec",
            "-i",
            "-e",
            &format!("PGPASSWORD={password}"),
            &var("POOL_IT_CONTAINER"),
            "psql",
            "-X",
            conninfo,
            "-f",
            "-",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("docker exec psql");
    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(sql.as_bytes())
        .expect("send the SQL");
    let output = output_within(child, 60, "psql");
    (
        output.status.code().unwrap_or(-1),
        String::from_utf8(output.stdout).expect("utf-8"),
        String::from_utf8(output.stderr).expect("utf-8"),
    )
}

fn direct_conninfo() -> String {
    "host=pg.pool.test hostaddr=127.0.0.1 port=5432 dbname=pz7_appdb user=pz7_rw \
     sslmode=verify-full sslrootcert=/certs/ca.pem connect_timeout=10"
        .to_owned()
}

fn pool_conninfo(port: u16) -> String {
    format!(
        "host={} port={port} dbname=pz7_db user=pz7_principal sslmode=verify-full sslrootcert=/certs/ca.pem connect_timeout=10",
        var("POOL_IT_CLIENT_HOST")
    )
}

/// Bootstraps a case and writes the witness into its serving configuration.
fn ready_case(name: &str) -> (Case, u16, AuthorityId) {
    let case = Case::new(name);
    let port = free_port(&var("POOL_IT_LISTEN_HOST"));
    case.write_serving(None, port);
    let id = bootstrap(&case);
    let id_text = id.to_string();
    case.write_serving(Some(&id_text), port);
    (case, port, id)
}

/// Every line of the log is one JSON object with `ts`, `level` and `event`
/// first, and nothing in the whole log names what it must not.
fn assert_log_is_clean(case: &Case, log: &str) {
    assert_log_text_is_clean(&case.root, log);
}

/// The same for the folder `root` of a test.
fn assert_log_text_is_clean(root: &Path, log: &str) {
    for line in log.lines() {
        assert!(line.starts_with("{\"ts\":\""), "{line}");
        let after_ts = &line[32..];
        assert!(after_ts.starts_with(",\"level\":\""), "{line}");
        assert!(line.contains(",\"event\":\""), "{line}");
        let value: serde_json::Value = serde_json::from_str(line).expect("one JSON object");
        assert!(value["event"].is_string(), "{line}");
        assert_ne!(value["event"], "log.unnamed", "{line}");
    }
    let root = root.to_str().expect("utf-8").to_owned();
    let dir = fixture_dir().to_str().expect("utf-8").to_owned();
    // Round four (review finding t2): the address the listener binds.
    let listen_host = var("POOL_IT_LISTEN_HOST");
    let secrets = [
        password("principal.password"),
        password("rw.password"),
        password("monitor.password"),
    ];
    let mut forbidden: Vec<&str> = vec![
        &root,
        &dir,
        "pz7",
        "127.0.0.1",
        "pg.pool.test",
        "host.docker.internal",
        listen_host.as_str(),
        "TRUNCATE",
        "SELECT",
        "INSERT",
        "SCRAM-SHA-256$",
    ];
    forbidden.extend(secrets.iter().map(String::as_str));
    for leak in forbidden {
        assert!(!log.contains(leak), "{leak:?} reached the log:\n{log}");
    }
}

#[test]
#[ignore = "needs scripts/check.sh integration"]
fn it_the_walking_fixture_gives_the_same_answers_through_pool() {
    // covers: 0004 AC-14, AC-18, AC-19, AC-24, AC-25, AC-10 (the advance).
    let (case, port, id) = ready_case("walking");
    let daemon = Daemon::start(&case);
    let started = daemon.wait_for(&["\"event\":\"daemon.started\""], 20);
    assert!(started.contains("\"start_sequence\":1"), "{started}");
    assert!(
        started.contains(&format!("\"authority_id\":\"{id}\"")),
        "{started}"
    );
    assert!(
        started.contains("\"configuration_generation\":1"),
        "{started}"
    );
    daemon.wait_for(
        &[
            "\"event\":\"admission.changed\"",
            "\"state\":\"open\"",
            "\"node_id\":\"pg1\"",
        ],
        20,
    );

    // The same SQL file, straight to PostgreSQL and through POOL.
    let direct = client(&direct_conninfo(), &password("rw.password"), WALKING_SQL);
    let pg_log_before = logs(&var("POOL_IT_CONTAINER")).len();
    let through = client(
        &pool_conninfo(port),
        &password("principal.password"),
        WALKING_SQL,
    );
    assert_eq!(direct.0, 0, "{}", direct.2);
    assert!(direct.1.contains(" one"), "{}", direct.1);
    assert!(direct.2.contains("division by zero"), "{}", direct.2);
    assert_eq!(through, direct, "the answers through POOL differ");

    // The mapped login: the session reached PostgreSQL as pz7_rw on
    // pz7_appdb, and the principal is no role of PostgreSQL.
    let pg_log = logs(&var("POOL_IT_CONTAINER"));
    let new = &pg_log[pg_log_before.min(pg_log.len())..];
    assert!(
        new.contains("connection authorized: user=pz7_rw database=pz7_appdb"),
        "{new}"
    );
    assert_eq!(
        psql(
            &var("POOL_IT_CONTAINER"),
            "SELECT count(*) FROM pg_roles WHERE rolname = 'pz7_principal'"
        ),
        "0"
    );

    // Teardown: the backend of the session is gone soon after the client.
    eventually(2, "the backend of the session closed", || {
        psql(
            &var("POOL_IT_CONTAINER"),
            "SELECT count(*) FROM pg_stat_activity WHERE usename = 'pz7_rw'",
        ) == "0"
    });
    daemon.wait_for(
        &[
            "\"event\":\"session.ended\"",
            "\"reason\":\"client_terminate\"",
        ],
        5,
    );

    // A clean stop.
    let log_path = daemon.log.clone();
    assert_eq!(daemon.stop(), 0);
    let log = fs::read_to_string(&log_path).expect("log");
    let last = log.lines().last().expect("a last line");
    assert!(last.contains("\"event\":\"daemon.stopped\""), "{last}");
    assert!(log.contains("\"event\":\"session.started\""), "{log}");
    assert!(log.contains("\"principal_id\":\"it-principal\""), "{log}");
    assert!(log.contains("\"identity_id\":\"it-identity\""), "{log}");
    assert_eq!(
        fs::read_to_string(case.root.join("pool.stdout")).expect("stdout"),
        "",
        "nothing on stdout while serving"
    );
    assert_log_is_clean(&case, &log);

    // The start advanced the start sequence once, before it listened.
    let config = pool_config::validate(&fs::read(&case.config).expect("config")).expect("valid");
    match Journal::read(&case.journal()).classify(&config) {
        AuthorityState::Ready(authority) => assert_eq!(authority.start_sequence().get(), 1),
        other => panic!("expected ready, got {}", other.name()),
    }
}

#[test]
#[ignore = "needs scripts/check.sh integration"]
fn it_run_refuses_a_journal_without_or_with_another_witness_and_never_binds() {
    // covers: 0004 AC-13, AC-10 (the content does not change).
    let case = Case::new("run-not-ready");
    // Round four (review finding t5): the test holds the port for the whole
    // run, so a `pool run` that tried to bind would exit 3 with
    // `listener.failed`; a bind after the run could not have failed.
    let holder = TcpListener::bind((var("POOL_IT_LISTEN_HOST").as_str(), 0)).expect("bind");
    let port = holder.local_addr().expect("address").port();
    case.write_serving(None, port);
    bootstrap(&case);
    let before = fs::read(case.journal()).expect("read");
    for (witness, code) in [
        (None, "authority.unconfirmed"),
        (
            Some("fedcba9876543210fedcba9876543210"),
            "authority.mismatch",
        ),
    ] {
        case.write_serving(witness, port);
        let run = Command::new(env!("CARGO_BIN_EXE_pool"))
            .args(["run", "--config", case.config.to_str().expect("utf-8")])
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("run");
        let run = output_within(run, 30, "pool run");
        assert_eq!(run.status.code(), Some(1));
        assert_eq!(
            String::from_utf8_lossy(&run.stdout),
            format!("run refused\nfindings: 1\n{code}: the journal state does not allow serving\n")
        );
        assert_eq!(fs::read(case.journal()).expect("read"), before, "{code}");
        holder.local_addr().expect("the test still holds the port");
    }
}

// Build plan step 11: limits, loss and denial on the fixture (`0004 AC-14`
// to `AC-17`, `AC-19` to `AC-22`, `AC-25`). Each test bootstraps its own
// journal against container 1 and runs its own daemon.

const TLS_REQUIRED: &str = "TLS is required";
const NOT_SUPPORTED: &str = "this protocol feature is not supported";
const AUTH_FAILED: &str = "authentication failed";
const NO_IDENTITY: &str = "no identity entry for this user and database";
const TOO_MANY: &str = "too many connections";
const TOO_LARGE: &str = "a message is larger than the configured limit";
const NOT_ADMITTED: &str = "the database is not accepting connections through this proxy";
const NOT_OPENED: &str = "the backend connection could not be established";
const UNKNOWN_OUTCOME: &str =
    "the connection was ended; the outcome of the statement in flight is unknown";

/// A bootstrapped case whose daemon is configured with `node`, `ca` and
/// `extra`, started and admitted (or, when `wait_open` is false, started).
fn serving_case(
    name: &str,
    node: NodeAt,
    ca: Option<PathBuf>,
    extra: &str,
    wait_open: bool,
) -> (Case, u16, Daemon) {
    let case = Case::new(name);
    let port = free_port(&var("POOL_IT_LISTEN_HOST"));
    case.write_serving(None, port);
    let id = bootstrap(&case).to_string();
    case.write_serving_with(Some(&id), port, node, ca, extra);
    let daemon = Daemon::start(&case);
    daemon.wait_for(&["\"event\":\"daemon.started\""], 20);
    if wait_open {
        daemon.wait_for(
            &["\"event\":\"admission.changed\"", "\"state\":\"open\""],
            20,
        );
    }
    (case, port, daemon)
}

fn select_one_works(port: u16) {
    let (code, _, stderr) = client(
        &pool_conninfo(port),
        &password("principal.password"),
        "SELECT 1;",
    );
    assert_eq!(code, 0, "a normal login after the case: {stderr}");
}

/// The text after `FATAL:  ` that psql printed, for errors of POOL.
fn fatal(stderr: &str) -> Vec<String> {
    stderr
        .lines()
        .filter_map(|line| {
            line.split_once("FATAL:  ")
                .map(|(_, message)| message.to_owned())
        })
        .collect()
}

fn pg_log_since(container: &str, before: usize) -> String {
    let log = logs(container);
    log[before.min(log.len())..].to_owned()
}

/// `psql` started in the background with the SQL on its stdin, still open.
fn client_spawn(conninfo: &str, password: &str) -> std::process::Child {
    Command::new("docker")
        .args([
            "exec",
            "-i",
            "-e",
            &format!("PGPASSWORD={password}"),
            &var("POOL_IT_CONTAINER"),
            "psql",
            "-X",
            conninfo,
            "-f",
            "-",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("docker exec psql")
}

fn write_stdin(child: &mut std::process::Child, text: &str) {
    use std::io::Write;
    let stdin = child.stdin.as_mut().expect("stdin");
    stdin.write_all(text.as_bytes()).expect("write");
    stdin.flush().expect("flush");
}

fn finish(mut child: std::process::Child) -> (i32, String, String) {
    drop(child.stdin.take());
    let output = output_within(child, 60, "psql in the background");
    (
        output.status.code().unwrap_or(-1),
        String::from_utf8(output.stdout).expect("utf-8"),
        String::from_utf8(output.stderr).expect("utf-8"),
    )
}

fn sessions_of_rw(state: &str) -> String {
    psql(
        &var("POOL_IT_CONTAINER"),
        &format!(
            "SELECT count(*) FROM pg_stat_activity WHERE usename = 'pz7_rw' AND state = '{state}'"
        ),
    )
}

fn ca_pem() -> Vec<u8> {
    fs::read(fixture_dir().join("certs/ca.pem")).expect("the test CA")
}

#[test]
#[ignore = "needs scripts/check.sh integration"]
fn it_startup_parameters_versions_and_authentication_on_the_fixture() {
    // covers: 0004 AC-15, AC-16, AC-17, AC-18. psql of the image over TLS.
    let (case, port, daemon) = serving_case("startup", primary_node(), None, "", true);
    let container = var("POOL_IT_CONTAINER");
    let right = password("principal.password");
    let before = logs(&container).len();
    // Through POOL: the mapped login and database, a primary, the
    // configured system identifier.
    let (code, stdout, stderr) = client(
        &pool_conninfo(port),
        &right,
        "\\pset tuples_only on\n\\pset format unaligned\nSELECT current_user, current_database(), pg_is_in_recovery(), (SELECT system_identifier FROM pg_control_system());\n",
    );
    assert_eq!(code, 0, "{stderr}");
    assert_eq!(
        stdout.lines().last().expect("a row"),
        format!("pz7_rw|pz7_appdb|f|{}", system_identifier(&container))
    );
    let after_good = logs(&container).len();
    // Refusals before or during authentication, none of which reaches PostgreSQL.
    for (conninfo, password, message) in [
        (
            pool_conninfo(port).replace("sslmode=verify-full", "sslmode=disable"),
            right.clone(),
            TLS_REQUIRED,
        ),
        (
            format!("{} replication=database", pool_conninfo(port)),
            right.clone(),
            NOT_SUPPORTED,
        ),
        (
            format!("{} options='-c work_mem=64kB'", pool_conninfo(port)),
            right.clone(),
            NOT_SUPPORTED,
        ),
        (
            pool_conninfo(port),
            "pz7-wrong-password".to_owned(),
            AUTH_FAILED,
        ),
        (
            pool_conninfo(port).replace("user=pz7_principal", "user=pz7_nobody"),
            right.clone(),
            AUTH_FAILED,
        ),
        (
            pool_conninfo(port).replace("dbname=pz7_db", "dbname=pz7_other"),
            right.clone(),
            NO_IDENTITY,
        ),
    ] {
        let (code, _, stderr) = client(&conninfo, &password, "SELECT 1;");
        assert_ne!(code, 0, "{conninfo}");
        assert_eq!(fatal(&stderr), [message.to_owned()], "{conninfo}: {stderr}");
    }
    let refused = pg_log_since(&container, after_good);
    assert!(
        !refused.contains("connection authorized: user=pz7_rw"),
        "{refused}"
    );
    assert!(
        pg_log_since(&container, before)
            .contains("connection authorized: user=pz7_rw database=pz7_appdb")
    );
    // The verifier POOL holds is no password of PostgreSQL, and the
    // principal is no role there.
    let verifier = fs::read_to_string(fixture_dir().join("principal.verifier")).expect("verifier");
    let (code, _, _) = client(
        &direct_conninfo().replace("user=pz7_rw", "user=pz7_principal"),
        verifier.trim(),
        "SELECT 1;",
    );
    assert_ne!(code, 0);
    select_one_works(port);
    let log_path = daemon.log.clone();
    assert_eq!(daemon.stop(), 0);
    assert_log_is_clean(&case, &fs::read_to_string(log_path).expect("log"));
}

#[test]
#[ignore = "needs scripts/check.sh integration"]
fn it_message_and_startup_limits_refuse_and_a_normal_login_follows() {
    // covers: 0004 AC-20 (54000 for a Query and for a startup packet, the
    // startup deadline).
    let extra = "\n[limits]\nfrontend_message_max_bytes = 1024\nstartup_packet_max_bytes = 512\nstartup_timeout_ms = 500\n";
    let (_case, port, daemon) = serving_case("message-limits", primary_node(), None, extra, true);
    let container = var("POOL_IT_CONTAINER");
    let right = password("principal.password");
    // A Query above 1024 bytes: 54000, and no byte of it reaches PostgreSQL.
    let marker = format!("pz7-too-large-{}", "x".repeat(1100));
    let before = logs(&container).len();
    let (code, _, stderr) = client(
        &pool_conninfo(port),
        &right,
        &format!("SELECT 1 /* {marker} */;\n"),
    );
    assert_ne!(code, 0);
    assert_eq!(fatal(&stderr), [TOO_LARGE.to_owned()], "{stderr}");
    assert!(!pg_log_since(&container, before).contains("pz7-too-large-"));
    select_one_works(port);
    // Through a raw client: a Query whose length field is exactly 1024
    // passes, one of 1025 gets 54000.
    let runtime = support::runtime();
    let host = var("POOL_IT_LISTEN_HOST");
    let ca = ca_pem();
    runtime.block_on(async {
        for (length, refused) in [(1024usize, false), (1025, true)] {
            let mut connection = support::login(
                &host,
                port,
                &ca,
                "host.docker.internal",
                "pz7_principal",
                "pz7_db",
                &right,
            )
            .await
            .expect("login");
            // 4 (length) + text + 1 (zero byte); the text is SELECT '...'.
            let text = format!("SELECT '{}'", "y".repeat(length - 5 - 9));
            let bytes = support::query(&text);
            assert_eq!(bytes.len() - 1, length);
            support::send(connection.stream_mut(), &bytes)
                .await
                .expect("send");
            let mut error = None;
            loop {
                match support::read_message(connection.stream_mut()).await {
                    Ok(message) if message.tag == b'E' => {
                        error = message.sqlstate();
                        break;
                    }
                    Ok(message) if message.tag == b'Z' => break,
                    Ok(_) => {}
                    Err(_) => break,
                }
            }
            if refused {
                assert_eq!(error.as_deref(), Some("54000"), "{length}");
            } else {
                assert_eq!(error, None, "{length}");
            }
        }
    });
    // A startup packet above 512 bytes: 54000.
    let long_name = "a".repeat(600);
    let (code, _, stderr) = client(
        &format!("{} application_name={long_name}", pool_conninfo(port)),
        &right,
        "SELECT 1;",
    );
    assert_ne!(code, 0);
    assert_eq!(fatal(&stderr), [TOO_LARGE.to_owned()], "{stderr}");
    select_one_works(port);
    // A client that sends nothing is closed after 500 ms.
    let waited = runtime.block_on(async {
        let mut stream = tokio::net::TcpStream::connect((host.as_str(), port))
            .await
            .expect("connect");
        let started = std::time::Instant::now();
        let bytes = support::read_to_close(&mut stream, std::time::Duration::from_secs(5))
            .await
            .expect("closed");
        assert!(bytes.is_empty(), "closed without a message: {bytes:?}");
        started.elapsed()
    });
    assert!(
        waited >= std::time::Duration::from_millis(450),
        "{waited:?}"
    );
    assert!(
        waited < std::time::Duration::from_millis(2500),
        "{waited:?}"
    );
    select_one_works(port);
    assert_eq!(daemon.stop(), 0);
}

#[test]
#[ignore = "needs scripts/check.sh integration"]
fn it_the_session_limit_and_teardown_give_the_permit_back() {
    // covers: 0004 AC-19, AC-20 (one session more than the limit).
    let extra = "\n[limits]\nsession_max_count = 1\n";
    let (_case, port, daemon) = serving_case("session-limit", primary_node(), None, extra, true);
    let right = password("principal.password");
    let mut holder = client_spawn(&pool_conninfo(port), &right);
    write_stdin(&mut holder, "SELECT pg_sleep(4);\n");
    eventually(10, "the first session runs", || {
        sessions_of_rw("active") == "1"
    });
    let started = std::time::Instant::now();
    let (code, _, stderr) = client(&pool_conninfo(port), &right, "SELECT 1;");
    assert_ne!(code, 0);
    assert_eq!(fatal(&stderr), [TOO_MANY.to_owned()], "{stderr}");
    assert!(
        started.elapsed() < std::time::Duration::from_secs(2),
        "at once"
    );
    let (code, _, stderr) = finish(holder);
    assert_eq!(code, 0, "{stderr}");
    select_one_works(port);
    // A client that leaves inside an open transaction leaves no row, its
    // backend is gone, and the one permit came back.
    let (code, stdout, stderr) = client(
        &pool_conninfo(port),
        &right,
        "BEGIN;\nINSERT INTO pz7_items (key, value) VALUES (77, 'left open');\n",
    );
    assert_eq!(code, 0, "{stderr}");
    // Round four (review finding t9): the INSERT ran before the client left.
    assert!(stdout.contains("INSERT 0 1"), "{stdout}");
    eventually(2, "the backend of the open transaction is gone", || {
        sessions_of_rw("idle in transaction") == "0" && sessions_of_rw("idle") == "0"
    });
    let (code, stdout, stderr) = client(
        &direct_conninfo(),
        &password("rw.password"),
        "\\pset tuples_only on\nSELECT count(*) FROM pz7_items WHERE key = 77;\n",
    );
    assert_eq!(code, 0, "{stderr}");
    assert_eq!(stdout.trim(), "0", "the row was rolled back");
    select_one_works(port);
    assert_eq!(daemon.stop(), 0);
}

#[test]
#[ignore = "needs scripts/check.sh integration"]
fn it_waiters_are_limited_and_time_out_after_the_acquire_deadline() {
    // covers: 0004 AC-20 (one waiter too many, the acquire deadline).
    let extra = "\n[limits]\nsession_max_count = 3\nnode_backend_max_count = 1\nbackend_wait_max_count = 1\nbackend_acquire_timeout_ms = 3000\n";
    let (_case, port, daemon) = serving_case("waiters", primary_node(), None, extra, true);
    let right = password("principal.password");
    let mut holder = client_spawn(&pool_conninfo(port), &right);
    write_stdin(&mut holder, "SELECT pg_sleep(9);\n");
    eventually(10, "the session holding the backend runs", || {
        sessions_of_rw("active") == "1"
    });
    let waiter_started = std::time::Instant::now();
    let mut waiter = client_spawn(&pool_conninfo(port), &right);
    write_stdin(&mut waiter, "SELECT 1;\n");
    pause();
    pause();
    pause();
    pause();
    pause();
    let third = std::time::Instant::now();
    let (code, _, stderr) = client(&pool_conninfo(port), &right, "SELECT 1;");
    assert_ne!(code, 0);
    assert_eq!(fatal(&stderr), [TOO_MANY.to_owned()], "{stderr}");
    let third_took = third.elapsed();
    let (code, _, stderr) = finish(waiter);
    let waited = waiter_started.elapsed();
    assert_ne!(code, 0);
    assert_eq!(fatal(&stderr), [TOO_MANY.to_owned()], "{stderr}");
    assert!(
        third_took < std::time::Duration::from_millis(2000),
        "one waiter too many is refused at once: {third_took:?}"
    );
    assert!(
        waited >= std::time::Duration::from_millis(3000),
        "{waited:?}"
    );
    assert!(
        waited <= std::time::Duration::from_millis(5500),
        "{waited:?}"
    );
    let (code, _, stderr) = finish(holder);
    assert_eq!(code, 0, "{stderr}");
    select_one_works(port);
    let log = daemon.log();
    assert!(log.contains("\"reason\":\"wait_limit\""), "{log}");
    assert!(log.contains("\"reason\":\"acquire_timeout\""), "{log}");
    assert_eq!(daemon.stop(), 0);
}

/// The resident memory of a process, in KiB.
fn rss_kib(pid: u32) -> u64 {
    let output = Command::new("ps")
        .args(["-o", "rss=", "-p", &pid.to_string()])
        .output()
        .expect("ps");
    String::from_utf8_lossy(&output.stdout)
        .trim()
        .parse()
        .expect("a number")
}

#[test]
#[ignore = "needs scripts/check.sh integration"]
fn it_the_memory_budget_holds_with_32_sessions_that_stopped_reading() {
    // covers: 0004 AC-21, R-13. Each session asks for 256 MiB of rows and
    // never reads; then one row of 64 MiB passes through unchanged.
    let (case, port, daemon) = serving_case("memory", primary_node(), None, "", true);
    let host = var("POOL_IT_LISTEN_HOST");
    let right = password("principal.password");
    let runtime = support::runtime();
    let ca = ca_pem();
    let sessions = runtime.block_on(async {
        let mut sessions = Vec::new();
        for _ in 0..32 {
            let mut connection = support::login(
                &host,
                port,
                &ca,
                "host.docker.internal",
                "pz7_principal",
                "pz7_db",
                &right,
            )
            .await
            .expect("a login through POOL");
            support::send(
                connection.stream_mut(),
                &support::query("SELECT repeat('x', 1048576) FROM generate_series(1, 256)"),
            )
            .await
            .expect("send the query");
            sessions.push(connection);
        }
        sessions
    });
    eventually(10, "32 queries run on PostgreSQL", || {
        sessions_of_rw("active") == "32"
    });
    let mut highest = 0;
    for _ in 0..20 {
        highest = highest.max(rss_kib(daemon.child.id()));
        pause();
    }
    assert!(highest < 96 * 1024, "resident memory {highest} KiB");
    drop(sessions);
    drop(runtime);
    eventually(5, "the 32 backends are gone", || {
        sessions_of_rw("active") == "0"
    });
    // One row of 64 MiB, straight and through POOL, the same bytes.
    let digest = |conninfo: &str, password: &str| {
        let output = Command::new("docker")
            .args([
                "exec",
                "-e",
                &format!("PGPASSWORD={password}"),
                &var("POOL_IT_CONTAINER"),
                "sh",
                "-c",
                &format!("psql -X -At \"{conninfo}\" -c \"SELECT repeat('ab', 33554432)\" | tee /dev/null | wc -c; psql -X -At \"{conninfo}\" -c \"SELECT repeat('ab', 33554432)\" | md5sum"),
            ])
            .output()
            .expect("docker exec");
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        String::from_utf8(output.stdout).expect("utf-8")
    };
    let direct = digest(&direct_conninfo(), &password("rw.password"));
    let through = digest(&pool_conninfo(port), &right);
    assert!(direct.starts_with("67108865\n"), "{direct}");
    assert_eq!(through, direct);
    // The number for the verification record.
    fs::write(case.root.join("rss-kib.txt"), format!("{highest}\n")).expect("write");
    assert_eq!(daemon.stop(), 0);
}

#[test]
#[ignore = "needs scripts/check.sh integration"]
fn it_a_lost_commit_answer_gives_08007_and_is_never_replayed() {
    // covers: 0004 AC-22, R-10, T-07. The relay cuts the connection after
    // PostgreSQL answered the COMMIT and before POOL got the answer.
    let runtime = support::runtime();
    let relay = support::Relay::start(&runtime, var("POOL_IT_PG_PORT").parse().expect("port"));
    let (_case, port, daemon) = serving_case(
        "lost-commit",
        NodeAt {
            id: "pg1",
            port: relay.port,
        },
        None,
        "",
        true,
    );
    let container = var("POOL_IT_CONTAINER");
    let rw = password("rw.password");
    let _ = client(
        &direct_conninfo(),
        &rw,
        "DELETE FROM pz7_items WHERE key = 4242;\n",
    );
    let probes = relay.connections();
    let mut session = client_spawn(&pool_conninfo(port), &password("principal.password"));
    write_stdin(
        &mut session,
        "BEGIN;\nINSERT INTO pz7_items (key, value) VALUES (4242, 'lost answer');\n",
    );
    eventually(10, "the transaction is open", || {
        sessions_of_rw("idle in transaction") == "1"
    });
    let session_index = relay.connections();
    assert_eq!(
        session_index,
        probes + 1,
        "the session has the next connection"
    );
    let before = logs(&container).len();
    relay.arm(session_index);
    write_stdin(&mut session, "COMMIT /* pz7-lost-commit-marker */;\n");
    let (_, _, stderr) = finish(session);
    assert!(relay.was_cut(), "the relay dropped the answer");
    assert_eq!(
        fatal(&stderr),
        [UNKNOWN_OUTCOME.to_owned()],
        "one error: {stderr}"
    );
    let pg_log = pg_log_since(&container, before);
    assert_eq!(
        pg_log.matches("pz7-lost-commit-marker").count(),
        1,
        "{pg_log}"
    );
    assert!(
        !pg_log.contains("connection authorized: user=pz7_rw"),
        "no new backend: {pg_log}"
    );
    let (code, stdout, stderr) = client(
        &direct_conninfo(),
        &rw,
        "\\pset tuples_only on\nSELECT count(*) FROM pz7_items WHERE key = 4242;\n",
    );
    assert_eq!(code, 0, "{stderr}");
    assert_eq!(
        stdout.trim(),
        "1",
        "the COMMIT was applied; POOL claimed no rollback"
    );
    assert!(
        daemon.log().contains("\"reason\":\"backend_lost\""),
        "{}",
        daemon.log()
    );
    assert_eq!(daemon.stop(), 0);
    drop(relay);
}

#[test]
#[ignore = "needs scripts/check.sh integration"]
fn it_a_node_that_refuses_tls_gives_08006_and_gets_no_startup_packet() {
    // covers: 0004 AC-15 (toward the node). The probe passes; the session
    // connection gets N to its SSLRequest.
    let runtime = support::runtime();
    let relay = support::Relay::start(&runtime, var("POOL_IT_PG_PORT").parse().expect("port"));
    let (_case, port, daemon) = serving_case(
        "tls-refused",
        NodeAt {
            id: "pg1",
            port: relay.port,
        },
        None,
        "",
        true,
    );
    relay.refuse_tls_from(relay.connections() + 1);
    let (code, _, stderr) = client(
        &pool_conninfo(port),
        &password("principal.password"),
        "SELECT 1;",
    );
    assert_ne!(code, 0);
    assert_eq!(fatal(&stderr), [NOT_OPENED.to_owned()], "{stderr}");
    assert_eq!(daemon.stop(), 0);
    drop(relay);
}

#[test]
#[ignore = "needs scripts/check.sh integration"]
fn it_a_wrong_ca_or_name_toward_the_node_closes_admission() {
    // covers: 0004 AC-15, AC-14. The probe uses the same TLS setup as the
    // sessions, so a wrong CA or name closes the cluster first: the client
    // gets 57P03 and PostgreSQL sees no login of the application.
    for (name, ca, server_name) in [
        ("wrong-ca", Some(other_ca_copy()), None),
        ("wrong-name", None, Some("other.pool.test")),
    ] {
        let (case, port, daemon) = serving_case(name, primary_node(), None, "", true);
        let id = Journal::read(&case.journal())
            .authority_id()
            .expect("id")
            .to_string();
        assert_eq!(daemon.stop(), 0);
        case.write_serving_with(Some(&id), port, primary_node(), ca, "");
        if let Some(server_name) = server_name {
            let text = fs::read_to_string(&case.config).expect("read").replace(
                "tls_server_name = \"pg.pool.test\"",
                &format!("tls_server_name = \"{server_name}\""),
            );
            fs::write(&case.config, text).expect("write");
        }
        let daemon = Daemon::start(&case);
        daemon.wait_for(&["\"event\":\"probe.failed\"", "\"class\":\"tls\""], 20);
        let container = var("POOL_IT_CONTAINER");
        let before = logs(&container).len();
        let (code, _, stderr) = client(
            &pool_conninfo(port),
            &password("principal.password"),
            "SELECT 1;",
        );
        assert_ne!(code, 0);
        assert_eq!(
            fatal(&stderr),
            [NOT_ADMITTED.to_owned()],
            "{name}: {stderr}"
        );
        assert!(!pg_log_since(&container, before).contains("connection authorized: user=pz7_rw"));
        assert_eq!(daemon.stop(), 0);
    }
}

#[test]
#[ignore = "needs scripts/check.sh integration"]
fn it_the_wrong_identity_closes_admission_before_any_login() {
    // covers: 0004 AC-14 on the fixture (cross check finding 1): the node
    // points at container 2, which has the same certificate, roles and
    // passwords and another system identifier.
    let (_case, port, daemon) = serving_case(
        "wrong-identity",
        NodeAt {
            id: "pg1",
            port: var("POOL_IT_PG_PORT_2").parse().expect("port"),
        },
        None,
        "",
        false,
    );
    daemon.wait_for(
        &[
            "\"event\":\"admission.changed\"",
            "\"state\":\"closed\"",
            "evidence.writer_identity",
        ],
        20,
    );
    let second = var("POOL_IT_CONTAINER_2");
    let before = logs(&second).len();
    let (code, _, stderr) = client(
        &pool_conninfo(port),
        &password("principal.password"),
        "SELECT 1;",
    );
    assert_ne!(code, 0);
    assert_eq!(fatal(&stderr), [NOT_ADMITTED.to_owned()], "{stderr}");
    let log = pg_log_since(&second, before);
    assert!(!log.contains("connection authorized: user=pz7_rw"), "{log}");
    assert_eq!(daemon.stop(), 0);
}

#[test]
#[ignore = "needs scripts/check.sh integration"]
fn it_shutdown_ends_idle_sessions_with_57p01_and_busy_ones_with_08007() {
    // covers: 0004 AC-25. One idle session, one with a query in flight,
    // and a shutdown deadline of 1 second.
    let extra = "\n[limits]\nshutdown_timeout_ms = 1000\n";
    let (_case, port, daemon) = serving_case("shutdown", primary_node(), None, extra, true);
    let host = var("POOL_IT_LISTEN_HOST");
    let right = password("principal.password");
    let runtime = support::runtime();
    let ca = ca_pem();
    let (mut idle, mut busy) = runtime.block_on(async {
        let login = || {
            support::login(
                &host,
                port,
                &ca,
                "host.docker.internal",
                "pz7_principal",
                "pz7_db",
                &right,
            )
        };
        let idle = login().await.expect("idle session");
        let mut busy = login().await.expect("busy session");
        support::send(busy.stream_mut(), &support::query("SELECT pg_sleep(10)"))
            .await
            .expect("send");
        (idle, busy)
    });
    eventually(10, "the query runs", || sessions_of_rw("active") == "1");
    let signalled = std::time::Instant::now();
    let status = Command::new("kill")
        .args(["-TERM", &daemon.child.id().to_string()])
        .status()
        .expect("kill");
    assert!(status.success());
    let (idle_code, busy_code, busy_after) = runtime.block_on(async {
        let idle_code = support::next_error(idle.stream_mut()).await;
        let busy_code = support::next_error(busy.stream_mut()).await;
        (idle_code, busy_code, signalled.elapsed())
    });
    assert_eq!(idle_code.as_deref(), Some("57P01"));
    assert_eq!(
        busy_code.as_deref(),
        Some("08007"),
        "never 57P01 while in flight"
    );
    assert!(
        busy_after >= std::time::Duration::from_millis(900),
        "{busy_after:?}"
    );
    assert_eq!(daemon.stop_after_signal(), 0);
    drop(runtime);
}

#[test]
#[ignore = "needs scripts/check.sh integration"]
fn it_a_request_for_protocol_3_2_goes_on_with_3_0() {
    // covers: 0004 AC-15. NegotiateProtocolVersion for 3.0, then the login
    // goes on with 3.0.
    let (_case, port, daemon) = serving_case("protocol-3-2", primary_node(), None, "", true);
    let (code, _, stderr) = client(
        &format!(
            "{} min_protocol_version=3.0 max_protocol_version=3.2",
            pool_conninfo(port)
        ),
        &password("principal.password"),
        "SELECT 1;",
    );
    assert_eq!(code, 0, "{stderr}");
    assert_eq!(daemon.stop(), 0);
}

#[test]
#[ignore = "needs scripts/check.sh integration"]
fn it_messages_of_a_real_server_pass_through_or_end_as_the_spec_says() {
    // covers: 0004 AC-23, AC-18 and the rule "A golden message is not proof
    // of a wire format": NOTICE, a notification, several statements in one
    // query string, an error inside a transaction, an empty query, and the
    // two COPY directions, each straight to PostgreSQL and through POOL.
    let (_case, port, daemon) = serving_case("real-messages", primary_node(), None, "", true);
    let rw = password("rw.password");
    let right = password("principal.password");
    // Notifications name the process ID of the backend, which differs.
    let without_pid = |text: &str| {
        text.lines()
            .map(|line| match line.split_once("server process with PID ") {
                Some((head, _)) => format!("{head}server process with PID N."),
                None => line.to_owned(),
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    for (name, sql) in [
        (
            "notice",
            "DO $$ BEGIN RAISE NOTICE 'pz7 notice %', 42; END $$;\nSELECT 1;\n",
        ),
        (
            "notify",
            "LISTEN pz7_channel;\nNOTIFY pz7_channel, 'pz7 payload';\nSELECT 2;\n",
        ),
        (
            "several",
            "SELECT 1 AS a \\; SELECT 2 AS b \\; SELECT 3 AS c;\n",
        ),
        (
            "aborted",
            "BEGIN;\nSELECT 1 / 0;\nSELECT 1;\nROLLBACK;\nSELECT 4;\n",
        ),
    ] {
        let direct = client(&direct_conninfo(), &rw, sql);
        let through = client(&pool_conninfo(port), &right, sql);
        assert_eq!(direct.0, 0, "{name}: {}", direct.2);
        let direct = (direct.0, without_pid(&direct.1), without_pid(&direct.2));
        let through = (through.0, without_pid(&through.1), without_pid(&through.2));
        assert_eq!(through, direct, "{name}");
    }
    // An empty query string: EmptyQueryResponse, then ReadyForQuery, the
    // same from PostgreSQL and through POOL.
    let runtime = support::runtime();
    let host = var("POOL_IT_LISTEN_HOST");
    let pg_port: u16 = var("POOL_IT_PG_PORT").parse().expect("port");
    let ca = ca_pem();
    let tags = runtime.block_on(async {
        let mut tags = Vec::new();
        for (host, port, name, user, database, password) in [
            (
                "127.0.0.1",
                pg_port,
                "pg.pool.test",
                "pz7_rw",
                "pz7_appdb",
                &rw,
            ),
            (
                host.as_str(),
                port,
                "host.docker.internal",
                "pz7_principal",
                "pz7_db",
                &right,
            ),
        ] {
            let mut connection = support::login(host, port, &ca, name, user, database, password)
                .await
                .expect("login");
            support::send(connection.stream_mut(), &support::query(""))
                .await
                .expect("send");
            let mut seen = Vec::new();
            loop {
                let message = support::read_message(connection.stream_mut())
                    .await
                    .expect("a message");
                seen.push(message.tag);
                if message.tag == b'Z' {
                    break;
                }
            }
            tags.push(seen);
        }
        tags
    });
    assert_eq!(tags[0], b"IZ");
    assert_eq!(tags[1], tags[0]);
    // COPY in both directions works straight and ends with 0A000 through
    // POOL, which closes the backend.
    for (name, sql) in [
        ("copy-out", "COPY (SELECT 1) TO STDOUT;\nSELECT 5;\n"),
        (
            "copy-in",
            "DELETE FROM pz7_items WHERE key = 9001;\nCOPY pz7_items (key, value) FROM STDIN;\n9001\tcopied\n\\.\nDELETE FROM pz7_items WHERE key = 9001;\nSELECT 6;\n",
        ),
    ] {
        let direct = client(&direct_conninfo(), &rw, sql);
        assert_eq!(direct.0, 0, "{name}: {}", direct.2);
        assert_eq!(fatal(&direct.2), Vec::<String>::new(), "{name}");
        let through = client(&pool_conninfo(port), &right, sql);
        assert_eq!(
            fatal(&through.2),
            [NOT_SUPPORTED.to_owned()],
            "{name}: {}",
            through.2
        );
        eventually(2, "the backend of the COPY session is closed", || {
            psql(
                &var("POOL_IT_CONTAINER"),
                "SELECT count(*) FROM pg_stat_activity WHERE usename = 'pz7_rw'",
            ) == "0"
        });
    }
    select_one_works(port);
    assert_eq!(daemon.stop(), 0);
}

/// The main file that a bootstrap leaves when it stops after setting the WAL
/// mode and before its transaction: one page of 4096 bytes with the header
/// of SQLite, no mark (`application_id` and `user_version` 0) and no table.
/// These are the bytes SQLite 3.54 writes for it.
fn one_empty_page() -> Vec<u8> {
    let mut page = vec![0u8; 4096];
    page[..16].copy_from_slice(b"SQLite format 3\0");
    page[16..18].copy_from_slice(&4096u16.to_be_bytes());
    page[18] = 2; // write version: WAL
    page[19] = 2; // read version: WAL
    page[21] = 64;
    page[22] = 32;
    page[23] = 32;
    page[27] = 1; // file change counter
    page[31] = 1; // size of the database in pages
    page[95] = 1; // version-valid-for
    page[96..100].copy_from_slice(&3_054_000u32.to_be_bytes());
    page[100] = 13; // leaf table b-tree page
    page[105] = 16; // cell content area at 4096
    page
}

#[test]
#[ignore = "needs scripts/check.sh integration"]
fn it_a_bootstrap_that_stopped_after_the_wal_mode_does_not_block_the_next() {
    // covers: 0004 AC-10, AC-11 (decision 59, found by the verification of
    // milestone 3): the file left behind is an empty database.
    let case = Case::new("stopped-bootstrap");
    case.write(&case.settings(), &[primary_node()]);
    let journal = case.journal();
    fs::write(&journal, one_empty_page()).expect("write the page");
    fs::set_permissions(&journal, fs::Permissions::from_mode(0o600)).expect("chmod");
    let run = pool("show-authority", &case.config, "human");
    assert_eq!(run.code, 0, "{}", run.stderr);
    assert_eq!(run.stdout, "authority state: uninitialized\n");
    let id = bootstrap(&case);
    assert_eq!(Journal::read(&journal).authority_id(), Some(id));
    let run = pool("show-authority", &case.config, "human");
    assert_eq!(
        run.stdout,
        format!("authority state: unconfirmed\nauthority_id: {id}\n")
    );
}

// ---- added by the tests of scope 004, round two (`/test one real primary path`) ----

/// Ends every connection of the monitor login on container 1, as a restart
/// of the node or a cut network would. Returns how many it ended.
fn end_monitor_connections() -> u32 {
    psql(
        &var("POOL_IT_CONTAINER"),
        "SELECT count(pg_terminate_backend(pid)) FROM pg_stat_activity WHERE usename = 'pz7_monitor'",
    )
    .parse()
    .expect("a count")
}

/// The number of log lines that hold every part.
fn lines_with(daemon: &Daemon, parts: &[&str]) -> usize {
    daemon
        .log()
        .lines()
        .filter(|line| parts.iter().all(|part| line.contains(part)))
        .count()
}

/// Waits, asking every 50 ms for at most `seconds`, until more lines than
/// `before` hold every part: a line written after the moment `before` was counted.
fn wait_for_more(daemon: &Daemon, parts: &[&str], before: usize, seconds: u64) {
    eventually(seconds, &format!("a new line with {parts:?}"), || {
        lines_with(daemon, parts) > before
    });
}

impl Daemon {
    /// Sends SIGTERM and waits at most `seconds` for the exit status, with
    /// the checks of [`Daemon::finish`].
    fn stop_within(self, seconds: u64) -> i32 {
        self.terminate();
        self.finish("after SIGTERM from Daemon::stop_within", seconds)
    }
}

/// Lets the probe run for `seconds`: an observation window, not a wait for an event.
fn watch_for(seconds: u64) {
    let until = std::time::Instant::now() + std::time::Duration::from_secs(seconds);
    while std::time::Instant::now() < until {
        pause();
    }
}

fn session_login(
    runtime: &tokio::runtime::Runtime,
    port: u16,
) -> pool_core::backend::BackendConnection<tokio::net::TcpStream> {
    let host = var("POOL_IT_LISTEN_HOST");
    let ca = ca_pem();
    let right = password("principal.password");
    runtime
        .block_on(support::login(
            &host,
            port,
            &ca,
            "host.docker.internal",
            "pz7_principal",
            "pz7_db",
            &right,
        ))
        .expect("a session through POOL")
}

#[test]
#[ignore = "needs scripts/check.sh integration"]
fn it_ac12_a_probe_that_lost_its_connection_opens_a_new_one_and_logs_each_class_once() {
    // covers: 0004 AC-12 and "The probe": a connection that failed is
    // dropped and the next round opens a new one; `probe.failed` is written
    // when the class of a failure changes, not for every sample, and
    // `probe.recovered` when the node answers again.
    let runtime = support::runtime();
    let relay = support::Relay::start(&runtime, var("POOL_IT_PG_PORT").parse().expect("port"));
    let (_case, _port, daemon) = serving_case(
        "probe-reconnect",
        NodeAt {
            id: "pg1",
            port: relay.port,
        },
        None,
        "",
        true,
    );
    // The kept connection is ended under the probe: one failure, then a new connection.
    assert_eq!(end_monitor_connections(), 1);
    daemon.wait_for(&["\"event\":\"probe.failed\"", "\"node_id\":\"pg1\""], 10);
    daemon.wait_for(
        &["\"event\":\"probe.recovered\"", "\"node_id\":\"pg1\""],
        10,
    );
    // Then no new connection gets TLS for about ten rounds: one line for the class `tls`.
    relay.refuse_tls_from(relay.connections() + 1);
    assert_eq!(end_monitor_connections(), 1);
    daemon.wait_for(&["\"event\":\"probe.failed\"", "\"class\":\"tls\""], 10);
    watch_for(2);
    assert_eq!(
        lines_with(
            &daemon,
            &["\"event\":\"probe.failed\"", "\"class\":\"tls\""]
        ),
        1,
        "{}",
        daemon.log()
    );
    relay.accept_tls_again();
    eventually(10, "the probe recovers a second time", || {
        lines_with(&daemon, &["\"event\":\"probe.recovered\""]) == 2
    });
    assert_eq!(
        lines_with(&daemon, &["\"event\":\"probe.failed\""]),
        3,
        "one line per change of class (answer, answer, tls):\n{}",
        daemon.log()
    );
    assert_eq!(daemon.stop_within(20), 0);
    drop(relay);
}

#[test]
#[ignore = "needs scripts/check.sh integration"]
fn it_ac14_a_writer_that_falls_silent_keeps_running_sessions_and_refuses_new_logins() {
    // covers: 0004 AC-14, "When a running session is ended": silence
    // (`evidence.writer_missing` alone) closes the cluster for new logins
    // and ends no running session.
    let runtime = support::runtime();
    let relay = support::Relay::start(&runtime, var("POOL_IT_PG_PORT").parse().expect("port"));
    let (_case, port, daemon) = serving_case(
        "silence",
        NodeAt {
            id: "pg1",
            port: relay.port,
        },
        None,
        "",
        true,
    );
    let mut running = session_login(&runtime, port);
    assert!(
        runtime
            .block_on(running.query_one_row("SELECT 1"))
            .expect("a row")
            .is_some()
    );
    // The probe can no longer reach the node; the session's own connection
    // still can. The daemon wrote one closed line before its first sample,
    // so the test waits for a new one.
    let closed = [
        "\"event\":\"admission.changed\"",
        "\"state\":\"closed\"",
        "\"reasons\":\"evidence.writer_missing\"",
    ];
    let before = lines_with(&daemon, &closed);
    relay.refuse_tls_from(relay.connections() + 1);
    assert_eq!(end_monitor_connections(), 1);
    wait_for_more(&daemon, &closed, before, 20);
    assert_eq!(
        runtime
            .block_on(running.query_one_row("SELECT 2"))
            .expect("the running session goes on"),
        Some(vec![Some("2".to_owned())])
    );
    let (code, _, stderr) = client(
        &pool_conninfo(port),
        &password("principal.password"),
        "SELECT 1;",
    );
    assert_ne!(code, 0);
    assert_eq!(fatal(&stderr), [NOT_ADMITTED.to_owned()], "{stderr}");
    assert_eq!(
        lines_with(&daemon, &["\"event\":\"session.ended\""]),
        0,
        "no session ended:\n{}",
        daemon.log()
    );
    relay.accept_tls_again();
    drop(running);
    assert_eq!(daemon.stop_within(20), 0);
    drop(relay);
}

#[test]
#[ignore = "needs scripts/check.sh integration"]
fn it_ac14_a_writer_that_changes_its_identity_ends_running_sessions_through_the_daemon() {
    // covers: 0004 AC-14, "When a running session is ended": a positive
    // conflict ends every running session of the cluster. The relay sends
    // the probe's next connection to container 2, which has another system
    // identifier, while the session stays on container 1.
    let runtime = support::runtime();
    let relay = support::Relay::start(&runtime, var("POOL_IT_PG_PORT").parse().expect("port"));
    let (_case, port, daemon) = serving_case(
        "identity-changes",
        NodeAt {
            id: "pg1",
            port: relay.port,
        },
        None,
        "",
        true,
    );
    let mut running = session_login(&runtime, port);
    assert!(
        runtime
            .block_on(running.query_one_row("SELECT 1"))
            .expect("a row")
            .is_some()
    );
    relay.retarget(var("POOL_IT_PG_PORT_2").parse().expect("port"));
    assert_eq!(end_monitor_connections(), 1);
    daemon.wait_for(
        &[
            "\"event\":\"admission.changed\"",
            "\"state\":\"closed\"",
            "evidence.writer_identity",
        ],
        20,
    );
    let ended = runtime.block_on(async {
        tokio::time::timeout(
            std::time::Duration::from_secs(10),
            support::next_error(running.stream_mut()),
        )
        .await
    });
    assert_eq!(
        ended
            .expect("the running session is ended within 10 s")
            .as_deref(),
        Some("57P01")
    );
    daemon.wait_for(
        &[
            "\"event\":\"session.ended\"",
            "\"reason\":\"conflict\"",
            "\"status\":\"idle\"",
        ],
        10,
    );
    assert_eq!(daemon.stop_within(20), 0);
    drop(relay);
}

/// The fields that each event may carry after `ts`, `level` and `event`
/// (spec 0004, "Log events").
fn allowed_fields(event: &str) -> Option<&'static [&'static str]> {
    Some(match event {
        "daemon.started" => &["start_sequence", "configuration_generation", "authority_id"],
        "daemon.stopping" => &["sessions"],
        "daemon.stopped" => &[],
        "daemon.failed" => &["task"],
        "daemon.panicked" => &[],
        "log.dropped" => &["count"],
        "admission.changed" => &[
            "cluster_id",
            "state",
            "node_id",
            "reasons",
            "snapshot_generation",
        ],
        "authority.conflict_latched" => &["cluster_id", "recorded"],
        "journal.write_failed" => &["class"],
        "probe.failed" => &["node_id", "class"],
        "probe.recovered" => &["node_id"],
        "node.lost" => &["node_id", "cluster_id"],
        "session.started" => &[
            "session",
            "principal_id",
            "identity_id",
            "cluster_id",
            "node_id",
            "lease_generation",
            "topology_epoch",
        ],
        "session.ended" => &["session", "reason", "status", "duration_ms"],
        "session.refused" => &["reason", "count"],
        "backend.failed" => &["node_id", "class", "sqlstate", "count"],
        _ => return None,
    })
}

/// Every line of `log` is an event of the table under "Log events" with only
/// its fields, each a string, a whole number or a boolean. Returns the events seen.
fn assert_log_fields(log: &str) -> std::collections::BTreeSet<String> {
    let mut seen = std::collections::BTreeSet::new();
    for line in log.lines() {
        let value: serde_json::Value = serde_json::from_str(line).expect("one JSON object");
        let object = value.as_object().expect("an object");
        let event = object["event"].as_str().expect("event is a string");
        let allowed =
            allowed_fields(event).unwrap_or_else(|| panic!("an event outside the table: {line}"));
        for (key, field) in object {
            if ["ts", "level", "event"].contains(&key.as_str()) {
                continue;
            }
            assert!(
                allowed.contains(&key.as_str()),
                "{key} is not a field of {event}: {line}"
            );
            assert!(
                field.is_string() || field.is_u64() || field.is_i64() || field.is_boolean(),
                "{key} of {event} is not a string, a whole number or a boolean: {line}"
            );
        }
        seen.insert(event.to_owned());
    }
    seen
}

#[test]
#[ignore = "needs scripts/check.sh integration"]
fn it_ac24_the_log_holds_only_the_fields_of_the_table_and_no_text_from_postgresql() {
    // covers: 0004 AC-24: every line is an event of the table with only its
    // fields, each a string, a whole number or a boolean; a canary in the
    // error text of PostgreSQL, in SQL and in a refused login reaches no line.
    let (case, port, daemon) = serving_case("log-fields", primary_node(), None, "", true);
    let canary = "CANARY-9f2c7b1e";
    let sql = format!(
        "SELECT 1;\nDO $$ BEGIN RAISE EXCEPTION '{canary} raised'; END $$;\nSELECT '{canary}'::int;\n"
    );
    let (_, _, stderr) = client(&pool_conninfo(port), &password("principal.password"), &sql);
    assert!(
        stderr.contains(canary),
        "the client sees the error of PostgreSQL: {stderr}"
    );
    let (code, _, _) = client(&pool_conninfo(port), canary, "SELECT 1;");
    assert_ne!(code, 0, "a wrong password");
    daemon.wait_for(
        &[
            "\"event\":\"session.refused\"",
            "\"reason\":\"authentication\"",
        ],
        10,
    );
    daemon.wait_for(&["\"event\":\"session.ended\""], 10);
    let log = daemon.log();
    assert_eq!(daemon.stop_within(20), 0);
    let log = format!(
        "{log}{}",
        fs::read_to_string(case.log_file()).unwrap_or_default()
    );
    assert!(!log.contains(canary), "the canary reached the log:\n{log}");
    let seen = assert_log_fields(&log);
    for event in [
        "daemon.started",
        "admission.changed",
        "session.started",
        "session.ended",
        "session.refused",
        "daemon.stopped",
    ] {
        assert!(seen.contains(event), "{event} missing from:\n{log}");
    }
    assert_log_is_clean(&case, &log);
}

// ---- added by the tests of scope 004, round four (review finding M3) ----

impl Case {
    /// The serving configuration with several node entries in the one cluster.
    fn write_serving_nodes(&self, witness: Option<&str>, listen_port: u16, nodes: &[NodeAt]) {
        let dir = fixture_dir();
        let mut settings = self.settings();
        settings.witness = witness;
        self.write(&settings, nodes);
        let mut text = fs::read_to_string(&self.config).expect("read");
        text.push_str(&format!(
            "\n[listener]\nhost = \"{host}\"\nport = {listen_port}\n\
             tls_certificate_file = \"{certs}/host.docker.internal.pem\"\ntls_private_key_file = \"{dir}/pool.key\"\n\n\
             [probe]\nperiod_ms = 200\ntimeout_ms = 5000\nobservation_expiry_ms = 8000\n\n\
             [[principal]]\nid = \"it-principal\"\nname = \"pz7_principal\"\nverifier_file = \"{dir}/principal.verifier\"\n\n\
             [[identity]]\nid = \"it-identity\"\nprincipal = \"pz7_principal\"\ndatabase = \"pz7_db\"\ncluster = \"main\"\n\
             backend_database = \"pz7_appdb\"\nbackend_login = \"pz7_rw\"\ncredential_file = \"{dir}/rw.password\"\n",
            host = var("POOL_IT_LISTEN_HOST"),
            certs = dir.join("certs").display(),
            dir = dir.display(),
        ));
        fs::write(&self.config, text).expect("write");
    }
}

#[test]
#[ignore = "needs scripts/check.sh integration"]
fn it_ac14_a_second_primary_beside_a_silent_writer_ends_sessions_and_latches_the_journal() {
    // covers: 0004 AC-14, 0003 AC-17 (review finding M3). Two node entries of
    // one cluster point at container 1 through two relays, so both report the
    // same system identifier and both are primaries. `pg2` is silent at first.
    // Then the writer `pg1` falls silent and `pg2` answers: the reasons mix
    // `evidence.writer_missing` with `evidence.second_primary`, the running
    // session is ended, and the latch reaches the journal: a restart with a
    // healthy writer and a silent `pg2` stays closed.
    let runtime = support::runtime();
    let pg_port: u16 = var("POOL_IT_PG_PORT").parse().expect("port");
    let writer = support::Relay::start(&runtime, pg_port);
    let second = support::Relay::start(&runtime, pg_port);
    second.refuse_tls_from(1);
    let case = Case::new("second-primary");
    let port = free_port(&var("POOL_IT_LISTEN_HOST"));
    case.write_serving(None, port);
    let id = bootstrap(&case).to_string();
    let nodes = [
        NodeAt {
            id: "pg1",
            port: writer.port,
        },
        NodeAt {
            id: "pg2",
            port: second.port,
        },
    ];
    case.write_serving_nodes(Some(&id), port, &nodes);
    let daemon = Daemon::start(&case);
    daemon.wait_for(
        &[
            "\"event\":\"admission.changed\"",
            "\"state\":\"open\"",
            "\"node_id\":\"pg1\"",
        ],
        20,
    );
    let mut running = session_login(&runtime, port);
    assert!(
        runtime
            .block_on(running.query_one_row("SELECT 1"))
            .expect("a row")
            .is_some()
    );

    // The writer falls silent first, and its monitor connection is ended
    // while pg2 is still refused, so the count can only see pg1. Only then
    // does pg2 answer: counted after it, its own monitor login could be in
    // the count too.
    writer.refuse_tls_from(writer.connections() + 1);
    assert_eq!(
        end_monitor_connections(),
        1,
        "the monitor connection of pg1"
    );
    second.accept_tls_again();
    daemon.wait_for(
        &[
            "\"event\":\"admission.changed\"",
            "\"state\":\"closed\"",
            "evidence.writer_missing",
            "evidence.second_primary",
        ],
        20,
    );
    let ended = runtime.block_on(support::next_error(running.stream_mut()));
    assert_eq!(
        ended.as_deref(),
        Some("57P01"),
        "the running session is ended"
    );
    daemon.wait_for(
        &[
            "\"event\":\"authority.conflict_latched\"",
            "\"recorded\":true",
        ],
        20,
    );
    assert_eq!(daemon.stop_within(20), 0);

    // A new start: the writer answers again, pg2 is silent, and the latch holds.
    writer.accept_tls_again();
    second.refuse_tls_from(second.connections() + 1);
    let daemon = Daemon::start(&case);
    daemon.wait_for(
        &[
            "\"event\":\"admission.changed\"",
            "\"state\":\"closed\"",
            "\"reasons\":\"authority.conflict_latched\"",
        ],
        20,
    );
    let (code, _, stderr) = client(
        &pool_conninfo(port),
        &password("principal.password"),
        "SELECT 1;",
    );
    assert_ne!(code, 0);
    assert_eq!(fatal(&stderr), [NOT_ADMITTED.to_owned()], "{stderr}");
    assert_eq!(daemon.stop_within(20), 0);
    drop((writer, second));
}

// ---- added by the tests of scope 004, round four (review findings t7, t9) ----

#[test]
#[ignore = "needs scripts/check.sh integration"]
fn it_ac9_ac25_a_serving_daemon_holds_its_state_directory_and_stops_on_sigint() {
    // covers: 0004 AC-9 (review finding t7): while the daemon serves, a
    // second command on its state directory gets `state.locked`; and
    // 0004 AC-25: SIGINT stops it as SIGTERM does.
    let (case, _port, daemon) = serving_case("served-lock", primary_node(), None, "", true);
    let locked = "error: state.locked: another process is using the state directory\n";
    let show = pool("show-authority", &case.config, "human");
    assert_eq!(
        (show.code, show.stderr.as_str()),
        (3, locked),
        "{}",
        show.stdout
    );
    let second = Command::new(env!("CARGO_BIN_EXE_pool"))
        .args(["run", "--config", case.config.to_str().expect("utf-8")])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("a second pool run");
    let second = output_within(second, 30, "a second pool run");
    assert_eq!(second.status.code(), Some(3));
    assert_eq!(String::from_utf8_lossy(&second.stderr), locked);
    let status = Command::new("kill")
        .args(["-INT", &daemon.child.id().to_string()])
        .status()
        .expect("kill");
    assert!(status.success());
    let log_path = case.log_file();
    assert_eq!(
        daemon.stop_after_signal(),
        0,
        "SIGINT stops the daemon cleanly"
    );
    let log = fs::read_to_string(log_path).expect("log");
    assert!(log.contains("\"event\":\"daemon.stopping\""), "{log}");
    assert!(log.contains("\"event\":\"daemon.stopped\""), "{log}");
}

#[test]
#[ignore = "needs scripts/check.sh integration"]
fn it_ac19_a_client_that_vanishes_inside_a_transaction_leaves_no_row_and_gives_the_permit_back() {
    // covers: 0004 AC-19 (review finding t9): the socket is dropped inside an
    // open transaction, with no Terminate, as a client that crashes would.
    let extra = "\n[limits]\nsession_max_count = 1\n";
    let (_case, port, daemon) = serving_case("vanished", primary_node(), None, extra, true);
    let runtime = support::runtime();
    let mut session = session_login(&runtime, port);
    for sql in [
        "BEGIN",
        "INSERT INTO pz7_items (key, value) VALUES (78, 'vanished')",
    ] {
        assert_eq!(
            runtime.block_on(session.query_one_row(sql)),
            Ok(None),
            "{sql}"
        );
    }
    eventually(5, "the transaction is open", || {
        sessions_of_rw("idle in transaction") == "1"
    });
    drop(session);
    eventually(2, "the backend of the open transaction is gone", || {
        sessions_of_rw("idle in transaction") == "0" && sessions_of_rw("idle") == "0"
    });
    let (code, stdout, stderr) = client(
        &direct_conninfo(),
        &password("rw.password"),
        "\\pset tuples_only on\nSELECT count(*) FROM pz7_items WHERE key = 78;\n",
    );
    assert_eq!(code, 0, "{stderr}");
    assert_eq!(stdout.trim(), "0", "the row was rolled back");
    select_one_works(port);
    assert_eq!(daemon.stop(), 0);
}

// ---- added by Build plan step 15 (`0004 AC-29`, `0004 AC-30`) ----

#[test]
#[ignore = "needs scripts/check.sh integration"]
fn it_ac29_a_login_to_an_address_that_moved_to_another_server_gets_08006() {
    // covers: 0004 AC-29, M2 of the independent review. The probe keeps its
    // connection to container 1 through the relay, so the cluster stays
    // open, while every new connection to the address of the node reaches
    // container 2, which has another system identifier. A login is refused
    // with 08006 before any SQL of the client, `backend.failed` has the class
    // `identity`, and container 2 saw no statement but the identity question.
    let runtime = support::runtime();
    let relay = support::Relay::start(&runtime, var("POOL_IT_PG_PORT").parse().expect("port"));
    let (_case, port, daemon) = serving_case(
        "address-moved",
        NodeAt {
            id: "pg1",
            port: relay.port,
        },
        None,
        "",
        true,
    );
    let second = var("POOL_IT_CONTAINER_2");
    let before = logs(&second).len();
    relay.retarget(var("POOL_IT_PG_PORT_2").parse().expect("port"));
    let (code, _, stderr) = client(
        &pool_conninfo(port),
        &password("principal.password"),
        "SELECT 'pz7-ac29-marker';\n",
    );
    assert_ne!(code, 0, "the login was refused: {stderr}");
    assert_eq!(fatal(&stderr), [NOT_OPENED.to_owned()], "{stderr}");
    daemon.wait_for(
        &[
            "\"event\":\"backend.failed\"",
            "\"node_id\":\"pg1\"",
            "\"class\":\"identity\"",
        ],
        10,
    );
    let pg_log = pg_log_since(&second, before);
    assert!(!pg_log.contains("pz7-ac29-marker"), "{pg_log}");
    assert!(
        pg_log.contains("connection authorized: user=pz7_rw"),
        "the session reached container 2: {pg_log}"
    );
    // PostgreSQL writes a statement of several lines with a tab after each
    // line feed, so the first line of the question stands for it.
    let first_line = pool_control::PROBE_QUERY
        .lines()
        .next()
        .expect("a first line");
    let statements = pg_log.matches("statement: ").count();
    let identity = pg_log
        .matches(&format!("statement: {first_line}\n"))
        .count();
    assert!(identity >= 1, "the identity question was asked: {pg_log}");
    assert_eq!(statements, identity, "no other statement: {pg_log}");
    // Back to container 1: the same login works again.
    relay.retarget(var("POOL_IT_PG_PORT").parse().expect("port"));
    select_one_works(port);
    assert_eq!(daemon.stop_within(20), 0);
    drop(relay);
}

/// A paused container, unpaused when this value is dropped, also when the
/// test fails. The backends of `pg_sleep` that the test left behind are then
/// ended, so that the tests after it find the container as it was.
struct Paused(String);

impl Paused {
    fn new(container: &str) -> Paused {
        let status = Command::new("docker")
            .args(["pause", container])
            .stdout(Stdio::null())
            .status()
            .expect("docker pause");
        assert!(status.success(), "docker pause {container}");
        Paused(container.to_owned())
    }
}

impl Drop for Paused {
    fn drop(&mut self) {
        let _ = Command::new("docker")
            .args(["unpause", &self.0])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
        let _ = Command::new("docker")
            .args([
                "exec",
                &self.0,
                "psql",
                "-X",
                "-q",
                "-U",
                "postgres",
                "-d",
                "postgres",
                "-c",
                "SELECT pg_terminate_backend(pid) FROM pg_stat_activity \
                 WHERE usename = 'pz7_rw' AND query LIKE 'SELECT pg_sleep%'",
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
}

#[test]
#[ignore = "needs scripts/check.sh integration"]
fn it_ac30_a_paused_writer_is_marked_lost_and_its_sessions_end() {
    // covers: 0004 AC-30, M1 of the independent review. Container 1 is
    // paused, so it closes nothing and answers nothing. With
    // `probe.node_loss_timeout_ms = 2000` the control loop marks pg1 lost
    // and writes `node.lost`; the session with a query in flight gets 08007
    // and the idle one 08006, and both end with the reason `node_lost`.
    let case = Case::new("paused-writer");
    let port = free_port(&var("POOL_IT_LISTEN_HOST"));
    case.write_serving(None, port);
    let id = bootstrap(&case).to_string();
    case.write_serving(Some(&id), port);
    let text = fs::read_to_string(&case.config).expect("read");
    let text = text.replacen(
        "observation_expiry_ms = 8000\n",
        "observation_expiry_ms = 8000\nnode_loss_timeout_ms = 2000\n",
        1,
    );
    assert!(text.contains("node_loss_timeout_ms = 2000"));
    fs::write(&case.config, text).expect("write");
    let daemon = Daemon::start(&case);
    daemon.wait_for(
        &["\"event\":\"admission.changed\"", "\"state\":\"open\""],
        20,
    );
    let runtime = support::runtime();
    let mut idle = session_login(&runtime, port);
    let mut busy = session_login(&runtime, port);
    runtime
        .block_on(support::send(
            busy.stream_mut(),
            &support::query("SELECT pg_sleep(60)"),
        ))
        .expect("send the query");
    eventually(10, "the query runs on PostgreSQL", || {
        sessions_of_rw("active") == "1"
    });
    let paused = Paused::new(&var("POOL_IT_CONTAINER"));
    daemon.wait_for(
        &[
            "\"event\":\"node.lost\"",
            "\"node_id\":\"pg1\"",
            "\"cluster_id\":\"main\"",
        ],
        30,
    );
    let error_within =
        |connection: &mut pool_core::backend::BackendConnection<tokio::net::TcpStream>,
         what: &str| {
            runtime
                .block_on(async {
                    tokio::time::timeout(
                        std::time::Duration::from_secs(10),
                        support::next_error(connection.stream_mut()),
                    )
                    .await
                })
                .unwrap_or_else(|_| panic!("the {what} session was not ended within 10 s"))
        };
    assert_eq!(error_within(&mut busy, "busy").as_deref(), Some("08007"));
    assert_eq!(error_within(&mut idle, "idle").as_deref(), Some("08006"));
    for status in ["in_flight", "idle"] {
        daemon.wait_for(
            &[
                "\"event\":\"session.ended\"",
                "\"reason\":\"node_lost\"",
                &format!("\"status\":\"{status}\""),
            ],
            10,
        );
    }
    assert_eq!(lines_with(&daemon, &["\"event\":\"node.lost\""]), 1);
    drop(paused);
    // The first good sample removes the mark, and logins work again.
    daemon.wait_for(
        &["\"event\":\"probe.recovered\"", "\"node_id\":\"pg1\""],
        30,
    );
    eventually(20, "a login works again", || {
        client(
            &pool_conninfo(port),
            &password("principal.password"),
            "SELECT 1;",
        )
        .0 == 0
    });
    assert_eq!(daemon.stop_within(20), 0);
}
