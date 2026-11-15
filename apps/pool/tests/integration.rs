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
