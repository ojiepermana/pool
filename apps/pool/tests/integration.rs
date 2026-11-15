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

const OTHER_CA: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../crates/pool-core/tests/fixtures/other-ca.pem"
);
const REFUSED: &str = "bootstrap is not possible in this state";

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
        .output()
        .expect("run pool");
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
        .output()
        .expect("docker exec");
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
    ca_settings.ca = PathBuf::from(OTHER_CA);
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
const WALKING_SQL: &str = "TRUNCATE pz7_items;
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
        let dir = fixture_dir();
        let mut settings = self.settings();
        settings.witness = witness;
        self.write(&settings, &[primary_node()]);
        let mut text = fs::read_to_string(&self.config).expect("read");
        text.push_str(&format!(
            "\n[listener]\nhost = \"{host}\"\nport = {listen_port}\n\
             tls_certificate_file = \"{certs}/host.docker.internal.pem\"\ntls_private_key_file = \"{dir}/pool.key\"\n\n\
             [probe]\nperiod_ms = 200\ntimeout_ms = 1000\nobservation_expiry_ms = 3000\n\n\
             [[principal]]\nid = \"it-principal\"\nname = \"pz7_principal\"\nverifier_file = \"{dir}/principal.verifier\"\n\n\
             [[identity]]\nid = \"it-identity\"\nprincipal = \"pz7_principal\"\ndatabase = \"pz7_db\"\ncluster = \"main\"\n\
             backend_database = \"pz7_appdb\"\nbackend_login = \"pz7_rw\"\ncredential_file = \"{dir}/rw.password\"\n",
            host = var("POOL_IT_LISTEN_HOST"),
            certs = dir.join("certs").display(),
            dir = dir.display(),
        ));
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
        Daemon { child, log }
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

    /// Sends SIGTERM and waits for the exit status.
    fn stop(mut self) -> i32 {
        let status = Command::new("kill")
            .args(["-TERM", &self.child.id().to_string()])
            .status()
            .expect("kill");
        assert!(status.success());
        let code = self
            .child
            .wait()
            .expect("wait")
            .code()
            .expect("an exit status");
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
    let output = child.wait_with_output().expect("psql");
    (
        output.status.code().unwrap_or(-1),
        String::from_utf8(output.stdout).expect("utf-8"),
        String::from_utf8(output.stderr).expect("utf-8"),
    )
}

fn direct_conninfo() -> String {
    "host=pg.pool.test hostaddr=127.0.0.1 port=5432 dbname=pz7_appdb user=pz7_rw \
     sslmode=verify-full sslrootcert=/certs/ca.pem"
        .to_owned()
}

fn pool_conninfo(port: u16) -> String {
    format!(
        "host={} port={port} dbname=pz7_db user=pz7_principal sslmode=verify-full sslrootcert=/certs/ca.pem",
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
    for line in log.lines() {
        assert!(line.starts_with("{\"ts\":\""), "{line}");
        let after_ts = &line[32..];
        assert!(after_ts.starts_with(",\"level\":\""), "{line}");
        assert!(line.contains(",\"event\":\""), "{line}");
        let value: serde_json::Value = serde_json::from_str(line).expect("one JSON object");
        assert!(value["event"].is_string(), "{line}");
        assert_ne!(value["event"], "log.unnamed", "{line}");
    }
    let root = case.root.to_str().expect("utf-8").to_owned();
    let dir = fixture_dir().to_str().expect("utf-8").to_owned();
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
    let port = free_port(&var("POOL_IT_LISTEN_HOST"));
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
            .output()
            .expect("run");
        assert_eq!(run.status.code(), Some(1));
        assert_eq!(
            String::from_utf8_lossy(&run.stdout),
            format!("run refused\nfindings: 1\n{code}: the journal state does not allow serving\n")
        );
        assert_eq!(fs::read(case.journal()).expect("read"), before, "{code}");
        TcpListener::bind((var("POOL_IT_LISTEN_HOST").as_str(), port))
            .expect("the listener was never bound");
    }
}
