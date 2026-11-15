# 0004. One real primary path

**Date**: 2026-11-15
**Status**: In Progress

## Summary

This spec designs the first real path through POOL. One `psql` client connects with TLS, proves who it is with SCRAM (a password exchange that never sends the password), is mapped to its own PostgreSQL login, and reads and writes real data on the one node that the journal, the configuration and a fresh probe all name as the writer. When the client leaves, its backend is closed, so unfinished work is rolled back by PostgreSQL itself.

To get there the slice builds four things for the first time: the wire code (two new crates, `pool-protocol` and `pool-core`), the journal file with its bootstrap command, the probe, and the command `pool run`. It also adopts the first runtime, TLS and SQLite dependencies, and with them the first native code in the product.

Everything else waits. There is no backend reuse (scope 005), no extended protocol, COPY or cancellation (scope 006), no replica (scope 007), no status read or drain control (scope 008) and no administration interface (scope 009).

## Requirements

These criteria cover scope 004 only. Cite them as `0004 AC-n`. Names in code font refer to sections of this spec. "The fixture" is the environment that [the integration check](#the-integration-check) starts: one PostgreSQL 18 container, test certificates and test logins. "Hermetic" means a test that needs no network and no container.

**User stories**:

- As an application owner, I want to point `psql` at POOL with TLS and my own password and get the same answers as from PostgreSQL, so that I can trust the proxy with real work.
- As an operator, I want POOL to refuse to send any SQL when it cannot prove which node is the writer, so that a restart, a stale file or a second primary never turns into a wrong write (`R-19`).
- As an operator, I want clear, bounded refusals when a limit is reached, so that an overload or a hostile client cannot take the proxy down (`R-13`).
- As an application owner, I want a lost connection to be reported as a lost connection, never retried and never reported as a rollback, so that I can reconcile the outcome myself (`R-10`).
- As the engineer who builds scopes 005 and 006, I want the session, lease and pool key records and the wire state machine to exist with tests, so that reuse and protocol breadth extend a working path.

**Acceptance criteria**:

Workspace and checks

- **AC-1** (workspace shape, `0002 AC-6`): `cargo metadata --no-deps --format-version 1` lists exactly five workspace members: `pool`, `pool-config`, `pool-control`, `pool-core` and `pool-protocol`. The dependencies between them are exactly the six pairs under [Workspace](#workspace). `pool-protocol` has no dependency at all. `Cargo.lock` holds the five members and the 70 third party entries listed in the [rationale](rationale.md#locked-packages). `apps/pool-cli` does not exist. No package has `dev-dependencies` or a build script. `cargo build --workspace --locked` no longer prints `unused workspace dependency`.
- **AC-2** (source guard): Rule W3 reads the new file format under [Source guard](#source-guard). With the baseline of this spec, a line that names `tokio`, `TcpStream`, `TcpListener` or `std::net` is accepted in `crates/pool-core` and `apps/pool` and rejected in `crates/pool-config`, `crates/pool-control` and `crates/pool-protocol`. `std::thread`, `UdpSocket`, `UnixListener` and `UnixStream` are rejected everywhere. `Instant::now`, `SystemTime`, `std::fs` and `std::env` are rejected anywhere under `crates/pool-control/src`. `rusqlite` is rejected under `crates/pool-control/src` except in `journal.rs`. The names `StoredAuthority`, `StoreProbe`, `classify_startup`, `plan_bootstrap` and `writer_admission` are rejected in every `.rs` file outside `crates/pool-control`. Each of these six groups has a case in `scripts/prove-checks.sh` that fails without the rule.
- **AC-3** (dependency policy and notices, `SEC-13`, `R-17`): `scripts/check.sh deny` passes on the product lockfile and on both fuzz lockfiles with the `deny.toml` under [Dependency policy](#dependency-policy), which adds five license exceptions by crate and no license to the general list. A file `NOTICE` exists at the root and names SQLite and AWS-LC with their versions and license terms. `scripts/check.sh linux-build` passes with the native code compiled by the C compiler of the pinned image and no other tool.
- **AC-4** (proof script and lockfile check): Case 17 of `scripts/prove-checks.sh` rejects a wrong status whatever status scope 004 has. A lockfile entry ends at the next line that starts with `[`, and one new case adds a local package to `Cargo.lock` and expects rule W4. `scripts/prove-checks.sh` passes on a committed tree.

Configuration

- **AC-5** (new keys): Every fixture of spec 0001 and spec 0003 keeps its result and its output, byte for byte. One of them changes its text to stay so: `invalid-unknown-key.toml` of spec 0001 names its unknown table `no_table` in place of `listener`, which is a known table from this slice on. Each `valid-serving*` fixture exits 0 with exactly the valid report of spec 0001. Each `invalid-serving-*` fixture exits 1 with the finding listed under [Fixtures](#fixtures). A file without `[listener]`, `[limits]`, `[probe]` and `[[principal]]` is still valid, and each key that is left out has the default in the table under [Keys](#keys).
- **AC-6** (names): A name or a path that holds a control character from U+0000 to U+001F or from U+007F to U+009F, or one of the direction and separator characters listed under [Names and paths](#names-and-paths), gives `config.invalid_value` at its key. The text `CANARY-9f2c7b1e` in such a value appears in no output.
- **AC-7** (number of findings): `validate` keeps the first 20000 findings in the order that spec 0003 fixes and counts the rest. The report then ends with the line `findings omitted: <n>` in the human format and carries `"findings_omitted":<n>` before `findings` in the JSON format. A report with 20000 findings or fewer is unchanged, byte for byte, so made input 3 of `0003 AC-15` still gives 17268 findings. For made input 5 under [Number of findings](#number-of-findings), a file at the size limit that holds only empty identity entries, the built binary writes exactly 20000 finding lines and less than 4 MiB in each format, and the omitted count is six times the number of entries minus 20000.
- **AC-8** (configuration that can serve): `ValidatedConfig::serving()` returns a view only when the file has everything `pool run` needs, and `ValidatedConfig::probing()` only when it has everything `pool bootstrap` needs. Each missing piece gives one finding `config.incomplete` at the key listed under [What a command needs](#what-a-command-needs). `pool check-config` does not report these.

State, journal, bootstrap and probe

- **AC-9** (protected paths, `SEC-4`, `T-11`): `pool run`, `pool bootstrap` and `pool show-authority` exit 3 with `state.permissions` when the state directory is not a directory with mode `0700`, or when a file in it that belongs to the journal is not a regular file with mode `0600` owned by the owner of the directory. They exit 3 with `secret.permissions` when a secret file has any group or other permission bit, is not a regular file or has another owner. A second command on the same state directory, while the first holds it, exits 3 with `state.locked` and changes nothing.
- **AC-10** (journal on disk, `0003 AC-7`, `0003 AC-8`): After a successful `pool bootstrap`, `journal.sqlite3` has `application_id` 1347374924, `user_version` 1, WAL mode and exactly the four tables of spec 0003, and it holds revision 1, start sequence 0, hold `none` and one approved topology per cluster. The journal reader returns `Absent` for no file and for an empty file, and `Unreadable` for each case that spec 0003 lists (wrong application ID, newer version, failed `quick_check`, missing `store` row, writer without member row, value out of range). Each `pool run` that finds the state `Ready` advances the start sequence by one before it listens. In every other state the content of `journal.sqlite3` does not change. SQLite may create its two side files when it reads.
- **AC-11** (bootstrap and reading the authority ID, `0001 SC-8`): On the fixture, with no journal and no witness, `pool bootstrap` exits 0 and prints a new authority ID. A second `pool bootstrap` exits 1 with `bootstrap.store_present`. `pool show-authority` prints the same ID and the state `unconfirmed`, and after the ID is written into the configuration it prints `ready`. With a node that cannot be reached, bootstrap exits 1 with `bootstrap.evidence_missing` and creates no journal file. No command prints a host, a login, a database name or a path.
- **AC-12** (probe, `SEC-5`, `T-03`): The probe logs in as the monitor login of the cluster, which is not a superuser, has no replication attribute and is a member of no role, over TLS with the CA and server name of the node. It runs the one query under [The probe](#the-probe) and reports the system identifier, the role and the timeline. A probe that gets no answer within `probe.timeout_ms`, a wrong certificate, a failed login or a row of another shape gives a `Failed` observation. The control loop keeps one sample per node, and `build_snapshot` never treats a sample as later than now.
- **AC-13** (no listener without ready authority): `pool run` with a journal state of `Uninitialized`, `Unconfirmed`, `Missing`, `Unreadable`, `Mismatch` or `Held` exits 1 with the one finding of that state (the reason names of spec 0003) and never binds the listener.
- **AC-14** (writer check before any SQL, `0001 SC-8`, `PRD-005 AC-1`): A session reaches PostgreSQL only through a route that the current topology snapshot marks open for its cluster, and POOL checks the snapshot again after the backend is ready and before it sends `AuthenticationOk`. A backend that does not report `in_hot_standby` as `off` is closed. When the snapshot is closed, or older than its `fresh_until`, the client gets SQLSTATE `57P03` and no connection is opened for its login. On the fixture: with the node pointed at a second PostgreSQL instance that has another system identifier, the event `admission.changed` names `evidence.writer_identity`, a login is refused with `57P03`, and the log of that instance shows no connection of the application login. Hermetic: a session that is already running is ended when its cluster closes with a reason from the list under [When a running session is ended](#when-a-running-session-is-ended), and is not ended for `evidence.writer_missing` alone.

The session path

- **AC-15** (TLS and startup, `SEC-1`, `T-01`): `psql` with `sslmode=verify-full` and the test CA connects. A startup packet without TLS is refused with `28000` and no authentication request. `GSSENCRequest` is answered with `N`. A request for protocol 3.2 is answered with `NegotiateProtocolVersion` for 3.0. Protocol major 2 or 4, a `replication` parameter and a startup parameter outside the list under [Startup](#startup) are each refused with the SQLSTATE listed there. A `CancelRequest` is read and dropped with no effect. Toward the node, POOL sends no startup packet unless the node answered `S`, the certificate chain leads to the configured CA and the name matches. A wrong CA or a wrong name gives the client `08006`.
- **AC-16** (frontend authentication, `SEC-2`, `SEC-3`, `T-02`, `R-11`): The frontend offers `SCRAM-SHA-256` and nothing else. The right password for a configured principal passes. A wrong password, and a user name that is no principal, both get `28P01` after a full exchange with the same message sequence. The verifier that POOL holds cannot log in to PostgreSQL, and the fixture uses a different name and a different password for the principal and for the backend login. An authentication message longer than 1024 bytes is refused.
- **AC-17** (identity map on a real connection, `0001 SC-3`, `PRD-001 AC-1`): An authenticated principal that asks for a database with no identity entry gets `28000`, and the PostgreSQL log shows no connection for it. The backend login, the backend database and the credential come from the identity entry alone: in the fixture the principal and the requested database have other names than the backend login and the backend database, and `current_user` and `current_database()` show the latter two.
- **AC-18** (the walking result, `PRD-001 AC-1`): The same SQL file, run through `psql` directly against PostgreSQL as the backend login and through POOL as the principal, gives the same output byte for byte: the committed INSERT, the SELECT of it, and a deliberate error with its SQLSTATE. Through POOL, `pg_is_in_recovery()` is false and the system identifier is the configured one.
- **AC-19** (teardown, `0001 SC-6`): A client that disconnects after an INSERT inside an open transaction leaves no row: a fresh connection does not see it, the backend process is gone from `pg_stat_activity`, and with `limits.session_max_count = 1` the next login succeeds, which shows that the session permit came back. The same holds after a clean `Terminate`.
- **AC-20** (limits, first case of `PRD-001 AC-3`, `SEC-8`): With small configured limits on the fixture: a `Query` message larger than `limits.frontend_message_max_bytes` gets `54000` and no byte of it reaches PostgreSQL; a startup packet larger than `limits.startup_packet_max_bytes` is refused; one session more than `limits.session_max_count` gets `53300`; one waiter more than `limits.backend_wait_max_count` gets `53300` at once; a waiter gets `53300` after `limits.backend_acquire_timeout_ms`, not before it and not more than 2 seconds after it; a client that sends nothing is closed after `limits.startup_timeout_ms`. After each case a normal login succeeds.
- **AC-21** (memory budget, first case of `PRD-001 AC-3`, `R-13`): With 32 sessions that each asked for 256 MiB of rows and then stopped reading, the resident memory of the `pool` process stays below 96 MiB (64 MiB for the process and 1 MiB for each session). A single row of 64 MiB passes through unchanged.
- **AC-22** (lost COMMIT response, first case of `PRD-001 AC-4`, `0001 SC-6`, `T-07`, `R-10`): The connection between POOL and PostgreSQL is cut after PostgreSQL answered a COMMIT and before POOL received the answer. The client gets one error with SQLSTATE `08007` and its connection is closed. The PostgreSQL log shows that COMMIT exactly once and no new connection for that login. A fresh connection finds the row, which shows that POOL made no claim of a rollback.
- **AC-23** (protocol subset and session records, `0001 SC-1`, `0001 SC-2`, `0001 SC-9`): After `ReadyForQuery` POOL accepts `Query` and `Terminate`. Every other frontend message, and a COPY response from the backend, ends the session with `0A000` and closes the backend. A message with an unknown tag or a length below 4 ends it with `08P01`. The `BackendKeyData` of PostgreSQL never reaches the client. Each session has one lease with a new lease generation and a pool key with the eight parts of `0001 SC-2`.
- **AC-24** (log events and redaction, `SEC-11`, `T-12`): Every line that `pool run` writes to stderr after it started is one JSON object with `ts`, `level` and `event` first, as the log rules of spec 0001 ask. Over a whole run of the integration check, no line of stdout or stderr holds a password, a verifier, the canary SQL text, a principal, a login, a database name, a host or a file path of the fixture. `Debug` of every secret type prints `[REDACTED]`.
- **AC-25** (shutdown): On SIGTERM or SIGINT the daemon stops accepting, ends idle sessions with `57P01`, ends the others when they become idle or when `limits.shutdown_timeout_ms` has passed, and exits 0. A session that is ended while a query is in flight gets `08007` and never `57P01`.

Evidence tooling

- **AC-26** (integration check): `scripts/check.sh integration` starts PostgreSQL from the image pinned by digest in that script, runs the tests of [the integration check](#the-integration-check) and removes every container it started, also after a failure. Without a Docker daemon it exits 3. The tests read addresses from the environment the script sets and contact nothing else.
- **AC-27** (fuzzing, `R-12`, `T-01`): `crates/pool-protocol/fuzz/` holds the targets `startup`, `frontend_message` and `scram`. `scripts/check.sh fuzz` builds every target of both fuzz packages and runs each on its committed inputs for a bounded time, and ends without a crash file.
- **AC-28** (frames split at every byte, `R-01`): For each golden startup packet and each golden message, feeding the decoder the bytes in two pieces, split at every position, gives the same result as feeding them whole. No decoder allocates from a length field before it has checked it against the limit.

| Criterion | What it answers |
|---|---|
| AC-15, AC-16, AC-17, AC-18, AC-19 | Scope 004: connect one authenticated TLS client through POOL to one primary, write and read actual data, and close cleanly |
| AC-18, AC-20, AC-23, AC-19 | Scope 004, done when: INSERT and SELECT match direct PostgreSQL, limits and errors work, an unfinished transaction is rolled back on disconnect |
| AC-14, AC-17, AC-18, AC-19 | `PRD-001 AC-1`, every sentence |
| AC-20, AC-21 | `PRD-001 AC-3`, first case: an oversized frame, the connection and waiting limits, memory within the declared budget, permits returned |
| AC-22 | `PRD-001 AC-4`, first case, on a dedicated backend |
| AC-15, AC-16, AC-20, AC-23, and the sections [Startup](#startup), [SQLSTATEs that POOL generates](#sqlstates-that-pool-generates) and [Limits](#limits) | Scope 004: the protocol owner pins the startup profile, the TLS and authentication path, refusal behavior and bounded limits before code |
| AC-10 to AC-14 | Scope 004: the minimum durable writer check from 003, before application SQL is dispatched |
| AC-17, AC-19, AC-20, AC-26 | Scope 004: a successful query alone does not prove teardown or denial |

| Step of the [walking fixture](../../testing/strategy.md#first-walking-slice-acceptance) | Criterion |
|---|---|
| The same fixture through direct PostgreSQL 18 and through POOL, with the mapped login, verified TLS and one dedicated backend | AC-15, AC-17, AC-18 |
| Compare the committed INSERT and SELECT and a deliberate server SQLSTATE | AC-18 |
| Disconnect during a second, uncommitted insert | AC-19 |
| Observe the backend exit, the permit return and the absence of the row | AC-19 |
| With a connection or frame limit exceeded, a bounded rejection and a later successful login | AC-20 |

| Input from another spec or plan | Where it is answered |
|---|---|
| Spec 0003 Follow-up 3: SQLite, native boundary review, license decision, `NOTICE`, one writer | AC-3, [Dependency policy](#dependency-policy), [Native code](#native-code), [One writer](#one-writer), manual step M1 |
| Spec 0003 Follow-up 4: bootstrap command, probe and queries, probe keys, permission checks, principal naming, live timeline, directory lock, reading the authority ID | AC-9 to AC-12, [Commands](#commands), [The probe](#the-probe), [How each method names the principal](#how-each-method-names-the-principal) |
| Spec 0003 Follow-up 4, m1, m2 and m3 | `0003 AC-17`, `0003 AC-19` and `0003 AC-18`, added to spec 0003 with this spec. Built by Build plan steps 2 and 6 |
| Spec 0003 Follow-up 4, n2 | AC-5: the range of `probe.observation_expiry_ms` |
| Spec 0003 Follow-up 4, m6 | [Control loop and topology snapshot](#control-loop-and-topology-snapshot): sessions read the snapshot and never call `writer_admission` |
| Spec 0003 Follow-up 4, m7 | AC-1 |
| Spec 0003 Follow-up 4, t6 | AC-2 |
| Spec 0003 Follow-up 10: where a lockfile entry ends, a new proof case, case 17 | AC-4 |
| Spec 0003 Follow-up 11: the buffer and the limit on findings | AC-7 |
| Spec 0003 Follow-up 12: control characters above U+007F | AC-6 |
| Spec 0003 Follow-up 14: three texts behind the code | Notes of 2026-11-15 in spec 0003 |
| Spec 0001 Follow-up 2: frontend authentication | AC-16, [Frontend authentication](#frontend-authentication) |
| Spec 0001 Follow-up 10: permission check of `SEC-4`, `tracing` | AC-9, AC-24, [Log events](#log-events) |
| Spec 0001, the ten safety contracts | `0001 SC-1`, `SC-2`, `SC-3`, `SC-6`, `SC-8` and `SC-9` are cited by the criteria above. `SC-4`, `SC-5`, `SC-7` and `SC-10` are not touched: this slice has no replica profile, no promotion and no reserved capability |
| Spec 0002, rules for later slices, and Follow-up 6: `integration`, fuzzing, new members, notices | AC-26, AC-27, AC-1, AC-3 |
| [Dependencies](../../plan/dependencies.md), row 004: one disposable primary, the client, TLS identities, nonproduction credentials, least privilege, a direct baseline | AC-26, AC-12, AC-18 |
| Dependencies, row 004, what can wait: shared reuse, replica lag, public administration, the full mutation journal | [What this slice builds and where it stops](#what-this-slice-builds-and-where-it-stops) |

`SEC-14` (independent review) is met by the scope box "Review it (fresh model)". This slice changes protocol, authentication and authority code, so the review needs the Security engineer, Protocol engineer and Control engineer roles of the [reviewer roster](../../engineering-standards.md#reviewer-roster), each of which a fresh model may hold for code.

## Decision

**Chosen option**: Build the whole thin path now, with the wire code in two new crates, the journal and probe wired through a topology snapshot, and no client driver anywhere.

1. `pool-protocol` (new) holds the bytes: startup packets, message headers, error and authentication messages and the SCRAM text messages. It has no dependency and does no input or output, so it can be fuzzed and tested at every byte boundary.
2. `pool-core` (new) holds the data plane: TLS on both sides, SCRAM as server and as client, the backend connection, the session, the lease, the limits and the relay. It is the only library that names the runtime and a socket.
3. `pool-control` gains the journal file (SQLite), the conflict latch after a start, the observation board and the topology snapshot. It still names no socket, no thread, no runtime, no clock and no file call.
4. `apps/pool` gains three commands (`bootstrap`, `show-authority`, `run`), the log writer and the control loop that probes, publishes snapshots and writes the latch.
5. The probe reuses the backend connection of `pool-core`. No PostgreSQL client library is adopted.
6. Six direct dependencies are adopted: `tokio`, `rustls`, `tokio-rustls`, `aws-lc-rs`, `rusqlite` and `tracing`. A person decides the two licenses they bring before the first of them is merged (manual step M1).

The decisions marked heavy in the [rationale](rationale.md#decisions-taken) are public contracts, formats, or parts of the security model. Changing one later means a new decision in this spec.

## What this slice builds and where it stops

| Thing | Scope 004 (this build) | Later |
|---|---|---|
| Frontend transport | TLS 1.2 and 1.3 after `SSLRequest`. No plain text | Direct TLS, client certificates: 006 |
| Frontend authentication | `SCRAM-SHA-256` against a verifier file per principal | `SCRAM-SHA-256-PLUS`, certificate authentication, rotation: 005 and 006 |
| Protocol version | 3.0. A request for a later minor is negotiated down | 3.2: 006 |
| Messages after login | `Query` and `Terminate` | Extended protocol, COPY, function call, cancellation: 006 |
| Backend | One new backend per session, closed when the session ends | Reuse, cleanup, pool partitions, idle and lifetime limits: 005 |
| Routing | The primary profile, on the eligible primary | Replica profile: 007 |
| Limits | Sessions, sessions per principal, handshakes, backend waiters, backends per node, three deadlines, two size limits | Byte and rate budgets, fair queues, slow reader deadlines: 005 |
| Journal | The file, bootstrap, the start sequence, the conflict latch | Operations, audit, backup, reconcile, migrations beyond version 1: 009. Fence receipts: 010 |
| Probe | Role, system identifier and timeline of every configured node | Lag: 007. History, flapping control, drain: 008 |
| Diagnostics | Log events on stderr | Metrics and a status read: 006 and 008 |
| Configuration changes | Read once at start | Reload: 009 |

Not in this build: `apps/pool-cli`, an administration listener, a metric, a parser for SQL, a second process, and any command that changes a journal after bootstrap.

## Workspace

Five members. The two new ones follow the manifest rules of spec 0001: the five inherited package fields, `[lints] workspace = true`, no `dev-dependencies`, no build script.

| Member | Depends on (workspace) | Third party dependencies it takes |
|---|---|---|
| `crates/pool-protocol` | nothing | none |
| `crates/pool-config` | nothing | `serde`, `toml` |
| `crates/pool-control` | `pool-config` | `rusqlite` |
| `crates/pool-core` | `pool-config`, `pool-protocol` | `tokio`, `rustls`, `tokio-rustls`, `aws-lc-rs`, `tracing` |
| `apps/pool` | `pool-config`, `pool-control`, `pool-core` | `serde`, `serde_json`, `tokio`, `tracing` |

`internal-dependencies.txt` (rule W5) holds exactly these six lines: `pool -> pool-config`, `pool -> pool-control`, `pool -> pool-core`, `pool-control -> pool-config`, `pool-core -> pool-config`, `pool-core -> pool-protocol`. `pool-core` and `pool-control` are never a pair. `forbidden-paths.txt` (rule W2) loses `crates/pool-protocol` and `crates/pool-core` and keeps `apps/pool-cli`.

Why two crates now and not modules in the binary: the frame code has to be fuzzed in the `fuzz/` directory of the crate it lives in ([spec 0002](../0002-standards-and-tooling/index.md#rules-for-later-slices)), and a crate with no dependency keeps that fuzz lockfile small. The session code needs the runtime, and keeping it in its own crate is what lets the source guard keep the runtime out of everything else.

Where each piece of code lives:

| Code | Home |
|---|---|
| Startup packet, message header, error and authentication messages, SCRAM message text, base64 | `pool-protocol` |
| Keys, findings, views, generations for configuration and credentials | `pool-config` |
| Authority rules (unchanged), journal, latch, board, snapshot, probe query and row reading | `pool-control` |
| Secret types and file reading, TLS setup, SCRAM, backend connection, session, lease, pool key, limits, relay, listener loop | `pool-core` |
| Argument parsing, commands, state directory checks and lock, log writer, control loop, mapping of the snapshot into the route view, signals | `apps/pool` |

### Adopted dependencies

Declared once under `[workspace.dependencies]` with exactly these versions and features, and taken with `workspace = true`.

| Dependency | Version | Features | Used for |
|---|---|---|---|
| `tokio` | 1.54.1 | no default features; `rt-multi-thread`, `net`, `io-util`, `io-std`, `time`, `sync`, `signal`, `macros` | The runtime, sockets, timers, the snapshot channel, SIGTERM, the log queue. `macros` is taken for `select!`, which the relay and the control loop need to wait on several things at once |
| `rustls` | 0.23.46 | no default features; `std`, `tls12`, `aws_lc_rs` | TLS as server and as client, certificate checks, PEM reading through `rustls::pki_types` |
| `tokio-rustls` | 0.26.6 | no default features | TLS streams on the runtime |
| `aws-lc-rs` | 1.19.0 | no default features; `aws-lc-sys`, `prebuilt-nasm` | SHA-256, HMAC and PBKDF2 for SCRAM, random bytes, comparison in constant time. It is the same package that `rustls` uses, so there is one crypto library |
| `rusqlite` | 0.42.0 | no default features; `bundled` | The journal. `bundled` compiles SQLite 3.54.1 from source, so the format does not depend on the host |
| `tracing` | 0.1.44 | no default features; `std` | Log events |

Not adopted, on purpose: `tokio-postgres` (the probe uses the wire code of this slice), `tracing-subscriber` (the log writer is 100 lines of first party code, see [Log events](#log-events)), `bytes` as a direct dependency (it comes in through `tokio`), `ring`, `base64`, `sha2`, `hmac`, `pbkdf2`, `rand`, `zeroize` and `secrecy` as direct dependencies, `clap`, `thiserror` and `anyhow`.

`Cargo.lock` gains 54 entries, for 70 third party entries in all. The list, with versions and declared licenses, is in the [rationale](rationale.md#locked-packages). It was resolved on 2026-11-15 with Cargo 1.100.0 in a scratch project outside the repository. If Cargo resolves another set of names in the real workspace, stop and report it to the lead. Do not edit the baseline to fit.

The step that adds the dependencies comes before the steps that use them, so Cargo prints `unused dependency` warnings for a short time. That is a known exception, limited in time, to the rule that a dependency is declared only when the current path uses it. It ends with Build plan step 10, and `0004 AC-1` checks that it ended. If a required check fails because of such a warning, stop and report it. Do not silence it.

### Dependency policy

`deny.toml` keeps its general list as it is: `Apache-2.0`, `MIT`, `Unicode-3.0`. It gains five exceptions by crate, placed after the existing one for `libfuzzer-sys`:

```toml
[[licenses.exceptions]]
crate = "aws-lc-rs"
allow = ["ISC"]

[[licenses.exceptions]]
crate = "aws-lc-sys"
allow = ["ISC", "BSD-3-Clause"]

[[licenses.exceptions]]
crate = "rustls-webpki"
allow = ["ISC"]

[[licenses.exceptions]]
crate = "untrusted"
allow = ["ISC"]

[[licenses.exceptions]]
crate = "subtle"
allow = ["BSD-3-Clause"]
```

Exceptions by crate are used, and not two more lines in the general list, for two reasons. The general list is also applied to the fuzz lockfiles, where no package needs `ISC`, and a license in the list that nothing needs fails the check. And a later package under `ISC` or `BSD-3-Clause` then needs its own decision. With this file, `cargo deny check` printed `advisories ok, bans ok, licenses ok, sources ok` for the scratch lockfile and for `crates/pool-config/fuzz/Cargo.lock` on 2026-11-15. An exception that a lockfile does not meet is a warning, as the `NCSA` one already is.

Adding a license exception is a decision of the Release maintainer, who must be a person ([spec 0002](../0002-standards-and-tooling/index.md#dependency-policy)). That is manual step M1, and Build plan step 3 waits for it.

### Native code

This slice brings the first native code into the product. The engineering standards ask for a boundary review of each. This is that review, and the independent review of the slice checks it.

| Library | Enters through | What it is | Boundary |
|---|---|---|---|
| AWS-LC 0.47.0 (`aws-lc-sys`) | `aws-lc-rs`, used by `rustls` and by `pool-core` | A C and assembly crypto library | First party code calls only the safe interface of `aws-lc-rs` (`digest`, `hmac`, `pbkdf2`, `rand`, `constant_time`) and `rustls`. Every input length is bounded by this spec before the call: SCRAM messages at 1024 bytes, the PBKDF2 count at 600000 |
| SQLite 3.54.1 (`libsqlite3-sys`) | `rusqlite`, used by `pool-control` | A C database library | Only `crates/pool-control/src/journal.rs` names `rusqlite` (`0004 AC-2`). Statements are constants with bound parameters. No SQL text is built from a value. The file is opened by path only, with no extension loading and no shared cache |

Both compile with the C compiler of the pinned image `rust:1.100.0-slim-bookworm` and need no other tool. This was run on 2026-11-15 for `aarch64-unknown-linux-gnu` in that image: the scratch program built in 12 seconds and printed `sqlite 3.54.1`. No first party crate gains `unsafe` code, and rule W4 still holds for all five members. A safe wrapper is not proof that the library inside is safe: fuzzing covers the first party parsers in front of it (`0004 AC-27`), and advisories for both libraries are checked on every pull request and every day.

`NOTICE` at the repository root is created by Build plan step 3, as [spec 0002](../0002-standards-and-tooling/index.md#rules-for-later-slices) asks of the first slice that bundles native code. It names the project and its license, and then for each bundled library its name, version, where it comes from and its terms: SQLite (public domain, with the text of its dedication) and AWS-LC (ISC, Apache-2.0, BSD-3-Clause, MIT and the OpenSSL terms that its own `LICENSE` file lists; copy the list from the `LICENSE` file of the locked `aws-lc-sys` and not from memory). The release inventory of scope 011 still owns the full set of license texts.

### Source guard

Rule W3 of spec 0002 has one list for all code. From this slice on, sockets and a runtime exist, so the rule gets a place for each text. `tools/repo-check/baseline/forbidden-source-patterns.txt` has three kinds of line. Words are separated by single spaces, and no text holds a space.

```text
<text>                 the text is forbidden in every .rs file under apps/ and crates/
deny <path> <text>     the text is also forbidden in every .rs file at or under <path>
allow <path> <text>    the text is allowed in every .rs file at or under <path>
```

A text is a problem in a file when a line of the first or second kind forbids it there and no `allow` line whose path is that file, or a directory above it, names the same text. A path is relative to the repository root and is compared by whole components, so `crates/pool-core` does not cover `crates/pool-core-x`. The file of this slice:

```text
std::net
TcpListener
TcpStream
UdpSocket
UnixListener
UnixStream
tokio
std::thread
StoredAuthority
StoreProbe
classify_startup
plan_bootstrap
writer_admission
allow crates/pool-core std::net
allow crates/pool-core TcpListener
allow crates/pool-core TcpStream
allow crates/pool-core tokio
allow apps/pool std::net
allow apps/pool TcpListener
allow apps/pool TcpStream
allow apps/pool tokio
allow crates/pool-control StoredAuthority
allow crates/pool-control StoreProbe
allow crates/pool-control classify_startup
allow crates/pool-control plan_bootstrap
allow crates/pool-control writer_admission
deny crates/pool-control/src Instant::now
deny crates/pool-control/src SystemTime
deny crates/pool-control/src std::fs
deny crates/pool-control/src std::env
deny crates/pool-control/src rusqlite
allow crates/pool-control/src/journal.rs rusqlite
```

What this keeps true:

- A new crate is covered by all thirteen bare lines the day it appears. Opening a socket or starting a runtime anywhere new needs a line in this file, in a pull request that names the spec that allows it.
- `std::thread`, `UdpSocket`, `UnixListener` and `UnixStream` have no `allow` line. No first party code starts a thread by itself. Blocking work goes through the blocking pool of the runtime.
- `pool-config`, `pool-control` and `pool-protocol` have no `allow` line for a socket or the runtime. The rule that core and control never meet is rule W5, and this rule keeps control free of the things that would tempt it to.
- The second half of `0003 AC-11` (no clock, no file call and no environment read in `pool-control`) is now a guard and no longer a `grep` run by hand. That is point t6 of Follow-up 4 of spec 0003. The journal module reads its file through `rusqlite`, which is the one text allowed in the one file.
- A journal value cannot be built, changed or classified outside `pool-control`. Other crates never hold a `StoreProbe`: the journal reader returns a `JournalRead`, which has no public field and can only be asked to classify, to plan a bootstrap or to show its authority ID ([The journal on disk](#the-journal-on-disk)). The four names that would let code go around it are refused outside the crate. That is point m2, and `0003 AC-19`.
- `writer_admission` cannot be called outside `pool-control`. Its one caller is `build_snapshot`, which passes at most one sample per node and never a sample later than now. So the two duties of point m3 are held by code, and the rule of spec 0003 itself does not change (`0003 AC-18`).

The message of rule W3 still names the file, the line number and the text. A line that is empty or starts with `#` is ignored, as in every baseline file. `scripts/prove-checks.sh` gets one case for each of the six groups of `0004 AC-2`. The rule text of spec 0002 gets a note that points here (Follow-up 2).

### Proof script and lockfile check

Two repairs that Follow-up 10 of spec 0003 hands to the next slice that touches `tools/repo-check/`. They are Build plan step 1, because the first one is red on `main` from the moment scope 004 leaves `planned`.

- **Case 17.** The case changes `planned` to `done` in the first line of `docs/scope/004-one-real-primary-path.md`, and changes nothing once that line no longer holds the word. From now on the script makes its own feature for the case, inside its copy of the tree: it appends the row `| 99 | [Proof case](099-proof-case.md) | Proof | planned |` to the table of `docs/scope/index.md` and writes `docs/scope/099-proof-case.md` with the first line `# 99. Proof case · done`. The row says `planned` and the file says `done`, so rule D3 must report it, whatever state the real features are in. The case expects `D3` and `099-proof-case.md` in the output.
- **Case 9** (added on 2026-11-15, from a finding of Build plan step 3). The case creates `crates/pool-core/.keep` and expects rule W2 to name `crates/pool-core`. From step 3 on that folder exists and is no longer forbidden, so the case would be accepted. It creates `apps/pool-cli/.keep` from then on and expects `W2` and `apps/pool-cli`, the one path that `forbidden-paths.txt` still holds. This change belongs to step 3, in the same commit as the baseline line it depends on. No other case rests on a path or a baseline line that step 3 changes: case 8 deletes `zmij`, which stays in the lockfile, case 10 uses `std::thread`, which no path allows, and case 12 uses the pair `pool-config -> pool`, which stays forbidden.
- **Where a lockfile entry ends.** The lockfile check of rule W4 takes an entry to end at the next line that starts with `[`, and no longer only at the next `[[package]]` line. The test `w4_a_table_after_the_last_entry_counts_as_part_of_it` is replaced by one that expects the opposite. One new case in `scripts/prove-checks.sh` appends a `[[package]]` entry with no `source` line and a name that is no member to `Cargo.lock`, followed by a `[[patch.unused]]` table with a `source` line, and expects `W4` and the package name. It also adds that name to its copy of `lock-packages.txt`, so that rule W1 is quiet and the case shows W4 alone.

### One writer

SQLite is synchronous, and rule W3 allows no thread. The answer has three parts.

1. `Journal` is a plain value that owns one connection. Its methods that write take `&mut self`. The compiler therefore allows one writer per value at a time.
2. The daemon owns exactly one `Journal`. It writes at two moments: once at start, before the runtime exists, and later when a conflict latch has to be set. The second runs inside `tokio::task::spawn_blocking`, so no worker of the runtime waits for the disk. The runtime is built with `max_blocking_threads(4)`.
3. The exclusive lock on the state directory ([State directory and protected files](#state-directory-and-protected-files)) keeps a second process away.

The data plane never touches the journal. It reads the route view.

## Configuration

All keys follow the [configuration rules](../0001-initial-architecture/index.md#configuration-rules) of spec 0001. Every new table and key is optional in the schema, so `config_version` stays `1` and every file that was valid before is still valid. What a command needs beyond a valid file is checked by a view ([What a command needs](#what-a-command-needs)).

### Keys

| Key | Type | Required | Default | Validation | Secret | Reload |
|---|---|---|---|---|---|---|
| `listener.host` | string | yes, when `[listener]` exists | none | The syntax of `node[].host` | no | Restart only |
| `listener.port` | integer | yes, when `[listener]` exists | none | 1 to 65535 | no | Restart only |
| `listener.tls_certificate_file` | string | yes, when `[listener]` exists | none | Path syntax. A PEM file with the certificate chain, leaf first | no | Restart only |
| `listener.tls_private_key_file` | string | yes, when `[listener]` exists | none | Path syntax. A secret reference | reference | Restart only |
| `limits.session_max_count` | integer | no | 1000 | 1 to 100000 | no | Restart only |
| `limits.handshake_max_count` | integer | no | 64 | 1 to 10000 | no | Restart only |
| `limits.backend_wait_max_count` | integer | no | 100 | 0 to 100000 | no | Restart only |
| `limits.node_backend_max_count` | integer | no | 100 | 1 to 100000 | no | Restart only |
| `limits.startup_timeout_ms` | integer | no | 5000 | 100 to 60000 | no | Restart only |
| `limits.backend_acquire_timeout_ms` | integer | no | 5000 | 100 to 60000 | no | Restart only |
| `limits.shutdown_timeout_ms` | integer | no | 30000 | 0 to 600000 | no | Restart only |
| `limits.startup_packet_max_bytes` | integer | no | 8192 | 512 to 65536 | no | Restart only |
| `limits.frontend_message_max_bytes` | integer | no | 16777216 | 1024 to 1073741823 | no | Restart only |
| `probe.period_ms` | integer | no | 2000 | 100 to 30000 | no | Restart only |
| `probe.timeout_ms` | integer | no | 1000 | 100 to 30000 | no | Restart only |
| `probe.observation_expiry_ms` | integer | no | 6000 | 1000 to 60000, and not less than `period_ms` plus `timeout_ms` | no | Restart only |
| `cluster[].monitor_login` | string | no | absent | Name syntax | no | Restart only |
| `cluster[].monitor_database` | string | no | `"postgres"` | Name syntax | no | Restart only |
| `cluster[].monitor_credential_file` | string | no | absent | Path syntax. A secret reference | reference | Restart only |
| `node[].tls_ca_file` | string | no | absent | Path syntax. A PEM file with one or more CA certificates | no | Restart only |
| `node[].tls_server_name` | string | no | The value of `host` | The syntax of `node[].host` | no | Restart only |
| `principal[].id` | string | yes | none | The syntax of `IdentityId`. Unique among principals | no | Restart only |
| `principal[].name` | string | yes | none | Name syntax. Unique among principals | no | Restart only |
| `principal[].verifier_file` | string | yes | none | Path syntax. A secret reference | reference | Restart only |
| `principal[].session_max_count` | integer | no | The value of `limits.session_max_count` | 1 to 100000 | no | Restart only |

- **Units.** Every limit ends in `_count`, `_ms` or `_bytes`, as rule 3 of spec 0001 asks.
- **The expiry has a range** (point n2 of Follow-up 4 of spec 0003). A value outside 1000 to 60000 is `config.schema` when it does not fit the integer type and `config.invalid_value` otherwise. An expiry smaller than the period plus the timeout could never see a fresh sample, so it is refused with `config.invalid_value` at `probe.observation_expiry_ms`. The period and the timeout each stop at 30000, so that their sum never passes the largest expiry and every value in a range can be used with some value of the other two. (The period stopped at 60000 in the first version of this table. Above 59900 no expiry could have met the rule, and the finding would have pointed at the expiry, which could not fix it. Changed on 2026-11-15.) The daemon never passes an expiry of its own: `build_snapshot` reads it from the configuration.
- **A principal** is one frontend identity that may log in. `name` is the user name of the startup packet. `id` is the name that logs may show in its place. An identity entry uses the principal by its `name`, in the key `identity[].principal` that spec 0003 defined.
- **Secret references.** There are now four: `identity[].credential_file`, `cluster[].monitor_credential_file`, `principal[].verifier_file` and `listener.tls_private_key_file`. `validate` opens none of them. They are read by `pool run` and `pool bootstrap` ([State directory and protected files](#state-directory-and-protected-files)).
- **Values out of range.** A number below or above the listed range gives `config.invalid_value` at its key, and a value of the wrong type or outside what the type can hold gives `config.schema`, as for `node.port` in spec 0003. Read the limits as `Option<Spanned<u32>>`.

Validation order: the new checks are added to step 5 of the [validation order](../0003-control-and-session-data-model/index.md#validation-order) of spec 0003, after its item 5 and before its cross checks: `listener` keys in the order of the table, then `limits`, then `probe` (the cross rule last), then for each cluster the three monitor keys after `expected_writer`, for each node the two TLS keys after `port`, and each principal in file order with its keys in the order `id`, `name`, `verifier_file`, `session_max_count`. Two cross checks are added at the end of item 6: `config.duplicate_id` for principals at `principal.id`, then `config.duplicate_principal` at `principal.name` of the second and each later entry with the same name.

### Findings

New codes in the family of spec 0001. Both exit 1.

| Code | When | Message |
|---|---|---|
| `config.duplicate_principal` | A principal has the name of an earlier principal | `two principal entries have the same name` |
| `config.incomplete` | A command needs a table or a key that the valid file does not have. Only `serving()` and `probing()` produce it | `a key that this command needs is missing` |

New constant key paths: `listener.host`, `listener.port`, `listener.tls_certificate_file`, `listener.tls_private_key_file`, the nine `limits.*` keys, the three `probe.*` keys, `cluster.monitor_login`, `cluster.monitor_database`, `cluster.monitor_credential_file`, `node.tls_ca_file`, `node.tls_server_name`, `principal.id`, `principal.name`, `principal.verifier_file`, `principal.session_max_count`. A finding about a `listener`, `limits` or `probe` key uses `Location::Key`. A finding inside a cluster, a node or a principal uses `Location::KeyAt`.

### Names and paths

This answers Follow-up 12 of spec 0003. Scope 004 is the first slice that sends a name to PostgreSQL, so the gap is closed before that happens.

The name syntax and the path syntax of spec 0003 refuse more characters from this slice on. A name or a path must not hold:

- a control character: U+0000 to U+001F, and U+007F to U+009F;
- a character that changes the direction of text: U+061C, U+200E, U+200F, U+202A to U+202E, U+2066 to U+2069;
- a line or paragraph separator: U+2028, U+2029.

The list is closed and written out, so the check needs no Unicode table. It applies to every key with name syntax or path syntax, old and new. A value that holds one of these gives `config.invalid_value` at its key. A file that was valid before and holds such a character in a name is now refused. That is the cost of this decision, and it is why it is marked heavy.

### Number of findings

This answers Follow-up 11 of spec 0003. The decision was yours to make. It was taken for you under the delegated mode of this run, it is marked heavy, and it is the first line of the decisions in the [rationale](rationale.md#decisions-taken).

- `validate` keeps at most `MAX_FINDINGS`, which is 20000, and counts every finding beyond that without storing it. The kept ones are the first 20000 in the order that spec 0001 and spec 0003 fix. The file is still rejected as a whole (`SEC-12`).
- `Rejection::findings()` returns the kept findings. `Rejection::omitted()` returns the count of the rest, as a `u64`.
- Human format: when the count is greater than 0, one more line follows the last finding line: `findings omitted: <n>`. The second line of the report counts the findings that were kept, so it reads `findings: 20000`.
- JSON format: when the count is greater than 0, the object carries `"findings_omitted":<n>` directly before `findings`. The field is absent when the count is 0. `output_version` stays `1`, because no field is removed or renamed.
- Every report with 20000 findings or fewer is unchanged, byte for byte. The largest count that an accepted criterion names is 17268 (made input 3 of `0003 AC-15`), which is why the number is not smaller.
- The binary writes its report through a `BufWriter` around the locked `stdout` and flushes it before it exits. A failed flush is exit status 3, as before.

Made input 5, built by the test at run time and not committed: line 1 is `config_version = 1`, line 2 is `identity = [`, then `{}` entries with one `,` between two of them, then `]`, and a line feed. It takes as many entries as fit in 1048576 bytes. Every entry lacks its six required keys.

### What a command needs

A valid file can still be too small to run with. Two views say what is missing. Each returns either a view or a `Rejection` whose findings all have the code `config.incomplete`.

| Needed | By `probing()` (for `pool bootstrap`) | By `serving()` (for `pool run`) | Key of the finding |
|---|---|---|---|
| `control.state_directory` | yes | yes | `control.state_directory` |
| At least one cluster | yes | yes | `cluster.id` |
| `monitor_login` and `monitor_credential_file` in every cluster | yes | yes | `cluster.monitor_login`, `cluster.monitor_credential_file`, with the position of the entry |
| `tls_ca_file` in every node | yes | yes | `node.tls_ca_file`, with the position of the entry |
| The `[listener]` table | no | yes | `listener.host` |
| At least one identity | no | yes | `identity.id` |
| A principal entry for the `principal` of every identity | no | yes | `identity.principal`, with the position of the entry |

`pool show-authority` needs `control.state_directory` only, and reports `config.incomplete` at that key when it is absent. A finding for a missing table uses `Location::Key`. The views borrow from the `ValidatedConfig` and add no second copy of a value.

### Public boundary added to pool-config

```rust
pub const MAX_FINDINGS: usize = 20_000;

impl Rejection { pub fn omitted(&self) -> u64; }

impl ValidatedConfig {
    pub fn listener(&self) -> Option<&ListenerConfig>;
    pub fn limits(&self) -> &Limits;                 // defaults when the table is absent
    pub fn probe(&self) -> &ProbeConfig;             // defaults when the table is absent
    pub fn principals(&self) -> &[PrincipalConfig];
    pub fn resolve_principal(&self, name: &str) -> Option<&PrincipalConfig>;
    pub fn probing(&self) -> Result<ProbingView<'_>, Rejection>;
    pub fn serving(&self) -> Result<ServingView<'_>, Rejection>;
}

impl ListenerConfig {
    pub fn host(&self) -> &str;
    pub fn port(&self) -> u16;
    pub fn tls_certificate_file(&self) -> &std::path::Path;
    pub fn tls_private_key(&self) -> &SecretReference;
}
impl Limits { /* one accessor per key: counts as u32, bytes as u32, times as std::time::Duration */ }
impl ProbeConfig {
    pub fn period(&self) -> std::time::Duration;
    pub fn timeout(&self) -> std::time::Duration;
    pub fn observation_expiry(&self) -> std::time::Duration;
}
impl ClusterConfig {
    pub fn monitor_login(&self) -> Option<&str>;
    pub fn monitor_database(&self) -> &str;
    pub fn monitor_credential(&self) -> Option<&SecretReference>;
}
impl NodeConfig {
    pub fn tls_ca_file(&self) -> Option<&std::path::Path>;
    pub fn tls_server_name(&self) -> &str;
}
impl PrincipalConfig {
    pub fn id(&self) -> &PrincipalId;
    pub fn verifier(&self) -> &SecretReference;
    pub fn session_max_count(&self) -> u32;
}

pub struct PrincipalId(/* private */);              // the shape of IdentityId
pub struct ConfigurationGeneration(/* private */);  // the shape of TopologyEpoch, FIRST is 1
pub struct CredentialGeneration(/* private */);     // the shape of TopologyEpoch, FIRST is 1
```

- `resolve_principal` compares byte for byte and does not walk every entry, like `resolve_identity`. `PrincipalConfig` has no accessor for the name.
- `Debug` is written by hand for `ListenerConfig`, `PrincipalConfig` and the two views, and prints identifiers and numbers only. No host, no port, no path and no name.
- The two generation types follow the [rules of spec 0003](../0003-control-and-session-data-model/index.md#identifier-and-generation-types). In this slice both stay at 1 for the whole run: there is no reload and no rotation. They exist now because the pool key carries one and the handoff check compares the other.
- `FindingCode` gains the two variants. Dependencies of `pool-config` stay `serde` and `toml`.

### Fixtures

All in `crates/pool-config/tests/fixtures/`. "The serving base" is `valid-topology.toml` of spec 0003 with these lines added: the three monitor keys in its cluster, `tls_ca_file` in each node, a `[listener]` table with the four keys, and one `[[principal]]` for each distinct `principal` of its identities.

| Fixture | Content | Result |
|---|---|---|
| `valid-serving.toml` | The serving base | Valid. `serving()` and `probing()` return a view |
| `valid-serving-limits.toml` | The serving base, with every `limits` and `probe` key at its lowest allowed value and `tls_server_name` set | Valid |
| `valid-serving-probe-only.toml` | The serving base without `[listener]` and without principals | Valid. `probing()` returns a view. `serving()` gives `config.incomplete` at `listener.host` and at `identity.principal` for each identity |
| `invalid-serving-port.toml` | `listener.port = 0` | `config.invalid_value` at `listener.port` |
| `invalid-serving-listener-key.toml` | `[listener]` without `tls_private_key_file` | `config.missing_key` at `listener.tls_private_key_file` |
| `invalid-serving-limit.toml` | `limits.session_max_count = 0` | `config.invalid_value` at `limits.session_max_count` |
| `invalid-serving-expiry.toml` | `probe.period_ms = 5000`, `probe.timeout_ms = 2000`, expiry left at its default | `config.invalid_value` at `probe.observation_expiry_ms` |
| `invalid-serving-duplicate-principal.toml` | Two principals with the same `name` and different `id` | `config.duplicate_principal` at `principal.name`, at the position of the second |
| `invalid-serving-unknown-key.toml` | `limits.idle_timeout_ms = 1` | `config.schema` |
| `invalid-name-c1.toml` | The base of spec 0003 and an identity whose `backend_login` holds U+009B and `CANARY-9f2c7b1e` | `config.invalid_value` at `identity.backend_login` |
| `invalid-name-direction.toml` | The same with U+202E in `database` | `config.invalid_value` at `identity.database` |
| `valid-serving-secret-values.toml` | The serving base with `CANARY-9f2c7b1e` inside every new string value | Valid. The canary is in no output and in no `Debug` text |

Positions (line and column) of each finding are written into the test from the fixture file when it is created, as spec 0003 did.

**The start of a fixture name says its result** (added on 2026-11-15, from a finding of Build plan step 3). A file that is valid has a name that starts with `valid-`. A file that is not starts with `invalid-` or `secret-`. The Linux build of spec 0001 runs `pool check-config` on every fixture and takes the exit status it expects from the name, and `0001 AC-9` says that every `secret-*` fixture exits 1. So the valid fixture that carries the canary is named `valid-serving-secret-values.toml`, as spec 0003 named its own `valid-topology-names.toml`. The first version of this table called it `secret-serving-values.toml`, and with that name `scripts/check.sh linux-build` stopped at it. Every other fixture of this table already follows the rule, and a fixture that a later step adds follows it too.

**One fixture of spec 0001 changes its text** (added on 2026-11-15, from a finding of Build plan step 5). `invalid-unknown-key.toml` holds the table `[listener]` with the key `port`, as its example of a table that is not in the schema. This slice makes `[listener]` a known table and `port` a known key in it, so the file would give three `config.missing_key` findings in place of its one `config.schema`. Its line 3 becomes `[no_table]`. Nothing else in the file changes. The name has the eight bytes of the old one, so the file keeps its length and every position. Its finding stays `config.schema` at line 3, column 2, and the output of `pool check-config` for it stays the same bytes in both formats, also for the copy with CRLF line endings that the tests make from the file. This was run on 2026-11-15 with the binary of `main` before step 5. The same change goes into the two cases of `0001 AC-7` that `crates/pool-config/tests/validate.rs` gives to `validate` from memory, `ac7_a_syntax_error_wins_over_schema_and_meaning_findings` and `ac7_a_schema_error_wins_over_meaning_findings`: `[listener]` becomes `[no_table]` in their text, and their expected findings (`config.syntax` at line 5, `config.schema` at line 3) do not change. No other fixture or test of spec 0001 or spec 0003 uses a table or key that this slice makes known. `no_table` is not a glossary term, and rule 2 of the [configuration rules](../0001-initial-architecture/index.md#configuration-rules) builds every name from glossary terms, so it cannot become a table of POOL later.

## Commands

The `pool` binary has four commands. Argument rules, usage errors and the two formats are those of [spec 0001](../0001-initial-architecture/index.md#the-check-config-command), with the command name as the first argument.

```text
pool check-config   --config <PATH> [--format <human|json>]
pool bootstrap      --config <PATH> [--format <human|json>]
pool show-authority --config <PATH> [--format <human|json>]
pool run            --config <PATH>
pool --help
pool --version
```

`--format` is not an option of `run`: there it is an unknown option. The help text gains one line for each new command under `Commands:` and the heading `Options for check-config:` becomes `Options:`. The developer writes the full text into the test, in the style of the existing help.

Every command first reads and validates the file as `check-config` does. When the file is invalid, or a view reports `config.incomplete`, the command prints the report of spec 0001 with its own name in `command` and exits 1. `config_version` and `capabilities` are then `null`, as for any rejected file.

### Findings of the new commands

A finding of these commands has a code and constant text. When it is about a file that a configuration key names, it also carries that key, as a constant key path (`Location::Key`), and never the path of the file. Codes with exit 1 mean that the input (the configuration together with the state it points at) was rejected. Codes with exit 3 mean that the command could not complete.

| Code | Exit | Commands | When | Message |
|---|---|---|---|---|
| `authority.uninitialized`, `authority.unconfirmed`, `authority.missing`, `authority.unreadable`, `authority.mismatch`, `authority.restore_pending` | 1 | `run` | The start was not classified `Ready` | `the journal state does not allow serving` |
| The eight `bootstrap.*` reasons of [spec 0003](../0003-control-and-session-data-model/index.md#bootstrap) | 1 | `bootstrap` | `plan_bootstrap` refused. One finding per reason, in its order | `bootstrap is not possible in this state` |
| `state.not_found` | 3 | all three | The state directory does not exist | `the state directory does not exist` |
| `state.permissions` | 3 | all three | The directory or a journal file fails the check of [State directory and protected files](#state-directory-and-protected-files) | `the state directory or a file in it has the wrong type, mode or owner` |
| `state.locked` | 3 | all three | Another process holds the lock | `another process is using the state directory` |
| `secret.not_found` | 3 | `run`, `bootstrap` | A secret file, a certificate file or a CA file does not exist | `a file named by the configuration does not exist` |
| `secret.permissions` | 3 | `run`, `bootstrap` | A secret file fails its check | `a secret file has the wrong type, mode or owner` |
| `secret.invalid` | 3 | `run`, `bootstrap` | A secret, a verifier, a certificate, a key or a CA file cannot be used as what it is | `a file named by the configuration does not hold what its key needs` |
| `journal.failed` | 3 | `run`, `bootstrap` | A write to the journal failed | `the journal could not be written` |
| `listener.failed` | 3 | `run` | The listener could not be bound | `the listener address could not be bound` |

The human and JSON shapes are the ones of spec 0001: exit 1 gives `<command> refused`, `findings: <n>` and the finding lines on stdout, and exit 3 gives one `error: <code>: <message>` line on stderr, or the JSON document with `result` `error`. In JSON, `result` is `refused` for exit 1. The first line of the human format for a rejected configuration stays `configuration invalid`. A finding about a file reads, for example, `error: secret.permissions at principal.verifier_file: a secret file has the wrong type, mode or owner`. It names the key and not the entry, so with several principals you check the file of each one. That is the same limit the findings of spec 0001 have, for the same reason: output never repeats text from the file.

### pool bootstrap

Steps, in order. The first failure ends the command.

1. Read and validate the file. Take `probing()`.
2. Check the state directory and take the lock.
3. Read the journal with `Journal::read`. Read the monitor credential and the CA file of every cluster and node.
4. Probe every node of every cluster once, all at the same time, each with the deadline `probe.timeout_ms`, and record each sample on an `ObservationBoard`.
5. Call `JournalRead::plan_bootstrap`. A refusal is reported and nothing is written.
6. Make an authority ID from 16 random bytes of the operating system. Create the empty journal file. Call `Journal::create`, which writes it in one transaction.
7. Print the result.

Human format, exit 0:

```text
bootstrap complete
authority_id: 0123456789abcdef0123456789abcdef
clusters: 1
next: add control.authority_id to the configuration
```

JSON format, exit 0:

```json
{"output_version":1,"command":"bootstrap","result":"created","authority_id":"0123456789abcdef0123456789abcdef","clusters":1,"findings":[]}
```

If the journal was written and the output then fails, the command exits 3 and the journal stays. `pool show-authority` reads the ID again.

### pool show-authority

It reads the file, takes the lock, reads the journal without writing, keeps `JournalRead::authority_id` and classifies the start. It contacts no node. It exits 0 whenever it could produce the report, whatever the state is.

```text
authority state: unconfirmed
authority_id: 0123456789abcdef0123456789abcdef
```

```json
{"output_version":1,"command":"show-authority","result":"unconfirmed","authority_id":"0123456789abcdef0123456789abcdef","findings":[]}
```

`result` is one of `uninitialized`, `unconfirmed`, `missing`, `unreadable`, `mismatch`, `restore_pending` and `ready`. The line and the field `authority_id` show the ID of the journal that was read. They are left out (`null` in JSON) when no journal could be read. This is the way to read the authority ID again that Follow-up 4 of spec 0003 asks for. It needs the lock, so it cannot run beside the daemon. That is fine: a daemon that runs has a witness already.

### pool run

Steps, in order. Until step 8 every failure is a finding with exit 1 or 3, written as spec 0001 says. From step 8 on every line on stderr is a log event.

1. Read and validate the file. Take `serving()`. Give the configuration its generation, `ConfigurationGeneration::FIRST`.
2. Check the state directory and take the lock, kept until the process ends.
3. Read the journal without writing and classify it with `JournalRead::classify`. Any state but `Ready` ends here with the finding `authority.` followed by the name of the state (`0004 AC-13`).
4. Read every protected file: the key and the certificate chain of the listener, the verifier of every principal, the credential of every identity and of every cluster, the CA file of every node. Each distinct credential file gets `CredentialGeneration::FIRST`.
5. Open the journal for writing with `Journal::open_ready`, which advances the start sequence in one transaction. A failure is `journal.failed`.
6. Build the runtime.
7. Bind the listener. The runtime resolves `listener.host` and binds the first address that works. A failure is `listener.failed`.
8. Install the log writer.
9. Start the control loop with an empty observation board. Publish the first route view, in which every cluster is closed. Accept connections. Write the event `daemon.started`.
10. On SIGTERM or SIGINT, shut down as [Shutdown](#shutdown) says and exit 0.

The daemon classifies once. A journal state other than `Ready` is refused at the start and not served with every login refused, because a process that looks alive and admits nothing is harder to notice than one that did not start.

## State directory and protected files

The code lives in `apps/pool` (the directory and the lock) and in `pool-core` (secret files), because `pool-control` makes no file call.

**The service account** is the owner of the state directory. Every check below compares owners with it, so the checks need no call that asks the operating system who the process is.

| Path | Must be | Otherwise |
|---|---|---|
| `control.state_directory` | A directory, mode exactly `0700` | `state.not_found` or `state.permissions` |
| `pool.lock`, `journal.sqlite3`, `journal.sqlite3-wal`, `journal.sqlite3-shm` in it, when they exist | A regular file, mode exactly `0600`, owned by the service account | `state.permissions` |
| Any other entry in the state directory | Not looked at | |
| A secret file: a credential, a verifier, the private key of the listener | A regular file after links are followed, no group and no other permission bit (so `0600` or `0400`), owned by the service account | `secret.not_found` or `secret.permissions` |
| A certificate chain or a CA file | A regular file that can be read | `secret.not_found` or `secret.invalid` |

- A process that is not root can read a `0600` file only if it owns it, so for such a process the owner rule adds nothing to the mode rule. For a process that runs as root the owner rule is what keeps a file of another account out. Running as root is not supported, and nothing detects it.
- **The lock** is the file `pool.lock` in the state directory. The command creates it if it is absent (mode `0600`), opens it and calls `File::try_lock` of the standard library, which takes an exclusive advisory lock. When the lock is held by another process the command exits 3 with `state.locked`. The lock is released when the process ends, also when it is killed. The file is never removed. This is the exclusive lock that Follow-up 4 of spec 0003 asks for: two daemons cannot share one state directory.
- Files are read through a reader with a limit: 4096 bytes for a credential or a verifier, 65536 bytes for a private key, 1048576 bytes for a certificate chain or a CA file. A longer file is `secret.invalid`.
- All checks are for Unix. The product has one declared target, which is Linux.

What each file holds:

| File | Content | Rule |
|---|---|---|
| A credential file | The password of the backend login or the monitor login | One trailing line feed, or carriage return and line feed, is dropped. After that: 1 to 1024 bytes, each a printable ASCII character from U+0020 to U+007E. Anything else is `secret.invalid`. Passwords outside ASCII need the SASLprep rules, which this slice does not build |
| A verifier file | One SCRAM verifier in the form PostgreSQL stores: `SCRAM-SHA-256$<count>:<salt>$<stored key>:<server key>` | One trailing line end is dropped. The count is 1 to 600000. The salt is 1 to 64 bytes after base64 decoding. Both keys are 32 bytes after decoding |
| The private key file | One private key in PEM, in a form `rustls` reads (PKCS 8, PKCS 1 or SEC 1) | Exactly one key |
| The certificate chain file | One or more certificates in PEM, leaf first | The key must belong to the leaf, which `rustls` checks when the server configuration is built |
| A CA file | One or more certificates in PEM | At least one. The system trust store is never used |

You get a verifier from PostgreSQL itself: on any PostgreSQL 18, create a role with `NOLOGIN` and the password, read `rolpassword` from `pg_authid`, and drop the role again. No role with the name or the password of the principal has to exist on the cluster that POOL serves. The verifier is not a password and cannot be used to log in to PostgreSQL (`SEC-2`).

## The journal on disk

The format, the schema, the settings on every open and the rules for start and migration are in [spec 0003](../0003-control-and-session-data-model/index.md#the-journal). This section fixes what that spec left to scope 004.

- **Checkpoint limits**: `PRAGMA wal_autocheckpoint = 1000` and `PRAGMA journal_size_limit = 4194304` on every open that writes.
- **Busy handling**: `busy_timeout` is 0. The lock on the state directory means no second process, so a busy answer is an error and not something to wait for.
- **The reader** opens the file with the flags for read only access, runs no statement that writes, and runs `PRAGMA quick_check`. It tells an absent file from one that cannot be opened by asking `Path::try_exists` first. A file that SQLite reports as an empty database (`PRAGMA page_count` is 0) is `Absent` too: it is what a bootstrap leaves behind when it stops between creating the file and writing it, and it must not block the next bootstrap.
- **Times**: `created_unix_ms` and `changed_unix_ms` are passed in by the caller, which reads the wall clock. They are for display. No rule reads them.
- **File mode**: the binary creates the empty file itself, before it calls `Journal::create`: with `create_new` and the mode `0o600` of `std::os::unix::fs::OpenOptionsExt`, followed by `set_permissions` to `0o600`, so that a strange umask cannot leave another mode. It creates `pool.lock` the same way. SQLite gives its side files the mode of the main file.
- **Storage failure while running**: the journal is read once, at the start. After that it is only written, and only to set a conflict latch. When that write fails, the latch still holds in memory for the rest of the run, the event `journal.write_failed` is logged, and the write is tried again in every later round of the control loop. Sessions that already run go on. New sessions follow the snapshot, as before. This settles the sentence that spec 0003 left open under [The journal](../0003-control-and-session-data-model/index.md#the-journal).

```rust
// crates/pool-control/src/journal.rs, the only file that names rusqlite
pub struct Journal { /* private: one connection */ }
pub struct LatchReceipt { /* private */ }      // cluster() -> &ClusterId, authority_revision() -> AuthorityRevision
pub enum JournalError { Open, Busy, Changed, Write }   // as_str(): "journal.open", "journal.busy", "journal.changed", "journal.write"

pub struct JournalRead { /* private: what the reader found */ }
impl JournalRead {
    pub fn classify(self, config: &ValidatedConfig) -> AuthorityState;
    pub fn plan_bootstrap(
        &self,
        config: &ValidatedConfig,
        board: &ObservationBoard,
        now: MonotonicTime,
    ) -> Result<BootstrapPlan, BootstrapRefused>;
    pub fn authority_id(&self) -> Option<AuthorityId>;
}

impl Journal {
    pub fn read(path: &std::path::Path) -> JournalRead;
    pub fn create(
        path: &std::path::Path,
        plan: &BootstrapPlan,
        authority_id: AuthorityId,
        created_unix_ms: u64,
    ) -> Result<(), JournalError>;
    pub fn open_ready(
        path: &std::path::Path,
        authority: &Authority,
    ) -> Result<(Journal, StartSequence), JournalError>;
    pub fn set_conflict_latch(
        &mut self,
        cluster: &ClusterId,
        changed_unix_ms: u64,
    ) -> Result<LatchReceipt, JournalError>;
}
```

- `Journal::read` is the journal reader of spec 0003. `JournalRead` wraps its `StoreProbe` and shows none of it. `classify` calls `classify_startup`. `plan_bootstrap` calls the function of that name with the samples of the board, the expiry of the configuration and the later of `now` and the latest sample time. `authority_id` is the ID of a journal that was read, also when the witness does not match, and `None` when no journal could be read.
- `AuthorityState` gains `name()`, which returns `uninitialized`, `unconfirmed`, `missing`, `unreadable`, `mismatch`, `restore_pending` or `ready`. The commands build their output and their findings from it.
- `create` fails with `Open` when the file is anything but an empty database. It writes the marks, the four tables and the rows in one transaction, with revision 1, start sequence 0, hold `none` and epoch 1 for each cluster of the plan.
- `open_ready` starts a transaction with `BEGIN IMMEDIATE`, reads the authority ID, the revision and the start sequence again, and fails with `Changed` when they are not the ones the `Authority` was classified from. It then applies migrations (none exist for version 1) and advances the start sequence. The returned value is the new start sequence. At the largest value it fails with `Write`.
- `set_conflict_latch` sets `conflict_latched` to 1 for the cluster and advances the authority revision, in one transaction. When the latch is already set it changes nothing and returns a receipt with the current revision. At the largest revision it fails with `Write`, and the latch stays in memory.
- A `JournalError` carries no text from SQLite. `Debug` prints the variant only.
- Only `Journal` builds a `LatchReceipt`.

## The probe

One task per node in the control loop. It keeps one backend connection open as the monitor login of the cluster and asks one question every `probe.period_ms`. When the connection is lost, the next round opens a new one.

**The monitor login** is a PostgreSQL role with `LOGIN` and nothing else: not a superuser, no `REPLICATION`, no `CREATEROLE`, no `CREATEDB`, no `BYPASSRLS`, member of no role. It needs the right to connect to `monitor_database`. Every function below can be executed by any role in PostgreSQL 18. This was run on 2026-11-15 against PostgreSQL 18.6 as such a role (`SEC-5`).

**The query**, sent as one simple `Query` message. It returns one row with three columns in text format:

```sql
SELECT (SELECT system_identifier::text FROM pg_control_system()),
       pg_is_in_recovery()::text,
       (CASE WHEN pg_is_in_recovery() THEN
          GREATEST((SELECT timeline_id FROM pg_control_checkpoint()),
                   (SELECT min_recovery_end_timeline FROM pg_control_recovery()),
                   COALESCE((SELECT received_tli FROM pg_stat_wal_receiver), 0))
        ELSE
          ('x' || substr(pg_walfile_name(pg_current_wal_insert_lsn()), 1, 8))::bit(32)::int
        END)::text
```

- **System identifier**: the first column, parsed with `SystemIdentifier::parse`.
- **Role**: the second column, `true` or `false`.
- **Timeline**: the third column. On a node that is not in recovery it is the timeline of the current write position, taken from the name of the WAL file that position falls in. It is exact, and it changes at the moment of a promotion. This is the timeline a node writes on now, which Follow-up 4 of spec 0003 asks for, and not the timeline of the last checkpoint. On a node in recovery it is the greatest timeline the node shows through plain SQL: its last restart point, its minimum recovery point and, when the role may see it, the timeline it last received. That can be one step behind what the node replays at this instant, and it can be ahead when WAL of a newer timeline was received and not yet replayed. Both directions are safe for the rules of spec 0003: a later timeline closes writes, and no rule opens writes because of a timeline on a node in recovery. The exact replay timeline needs a replication connection, which needs the `REPLICATION` attribute, and that attribute is too much for a monitor. Scope 007 qualifies this on a real replica.
- A row with another number of columns, a null, text that does not parse, a timeline of 0, an error response, more than one row, or more than 4096 bytes of answer gives `ObservationOutcome::Failed`.
- **Sample time**: the monotonic clock of the daemon, read when the row has arrived.
- **Deadline**: `probe.timeout_ms` for the whole round, from the start of a connection attempt or of the query to the row. At the deadline the connection is closed and the outcome is `Failed`.
- A `Failed` outcome is recorded like any sample. It replaces the last good sample of that node, so a node that stops answering loses its evidence at once and not only when the old sample expires.
- The probe connection sets `application_name` to `pool-probe` and counts against the node, not against `limits.node_backend_max_count`. You reserve one connection per node for it in `max_connections` of PostgreSQL.

The query text and the reading of the row live in `pool-control` as a constant and a plain function, so they are tested without a database:

```rust
pub const PROBE_QUERY: &str = /* the text above */;
pub fn observation_from_row(node: NodeId, sampled_at: MonotonicTime, row: Option<&[Option<&str>]>) -> Observation;
```

`row` is `None` when the probe did not get exactly one row. The connection, the TLS check and the login are done by `pool-core` ([Backend connection](#backend-connection)), called by the control loop in `apps/pool`.

## Control loop and topology snapshot

The control loop is one task in `apps/pool`. It owns the `Controller` of `pool-control`, receives samples from the probe tasks over a bounded channel, and publishes a route view to the data plane through a `tokio::sync::watch` channel.

```rust
// pool-control
pub struct SnapshotGeneration(/* private */);   // the shape of TopologyEpoch, FIRST is 1

pub struct ObservationBoard { /* private: at most one sample per node */ }
impl ObservationBoard {
    pub fn new() -> ObservationBoard;
    pub fn record(&mut self, observation: Observation);
    pub fn observations(&self) -> &[Observation];
}

pub struct TopologySnapshot { /* private */ }
impl TopologySnapshot {
    pub fn generation(&self) -> SnapshotGeneration;
    pub fn clusters(&self) -> &[ClusterAdmission];
}
pub struct ClusterAdmission { /* private */ }
impl ClusterAdmission {
    pub fn cluster(&self) -> &ClusterId;
    pub fn admission(&self) -> &WriterAdmission;
    pub fn fresh_until(&self) -> Option<MonotonicTime>;
}
pub fn build_snapshot(
    config: &ValidatedConfig,
    state: &AuthorityState,
    board: &ObservationBoard,
    now: MonotonicTime,
    generation: SnapshotGeneration,
) -> TopologySnapshot;

pub struct Controller { /* private: state, board, generation, latches not yet recorded */ }
impl Controller {
    pub fn new(state: AuthorityState) -> Controller;
    pub fn record(&mut self, observation: Observation);
    pub fn round(&mut self, config: &ValidatedConfig, now: MonotonicTime) -> TopologySnapshot;
    pub fn unrecorded_latches(&self) -> &[ClusterId];
    pub fn latch_recorded(&mut self, receipt: &LatchReceipt);
}
```

Rules:

- **One sample per node.** `ObservationBoard::record` keeps the sample with the later sample time for a node. When the times are equal it keeps the one it has. So the list it returns never holds two samples of one node. This is the first duty of point m3.
- **No sample is later than now.** `build_snapshot` uses the later of `now` and the latest sample time on the board as its now. A later now only makes samples older, so this can close and never open. This is the second duty of point m3, and it no longer rests on the caller.
- **One caller.** `build_snapshot` is the only code that calls `writer_admission` in the product, and the source guard keeps it so. The rule "Fresh" of spec 0003 is not made stricter: with one sample per node and no sample in the future, the two cases it treats as no evidence cannot reach it.
- **`build_snapshot`** calls `writer_admission` once for every cluster of the configuration, with the expiry from `config.probe()`. For an `Open` answer, `fresh_until` is the sample time of the approved writer plus the expiry. For a `Closed` answer it is `None`.
- **`Controller::round`** works in memory only and never waits for the disk. It builds a snapshot. For each cluster whose answer is closed with `latches_conflict()`, it sets the latch in the state with `AuthorityState::latch_conflict` (`0003 AC-17`) and puts the cluster on the list of latches that the journal does not hold yet. It then builds the snapshot again, so the snapshot it returns already shows `authority.conflict_latched`. Each round advances the snapshot generation by one. When the generation cannot advance, the round returns every cluster closed.
- **Closing comes first, the disk second.** The loop publishes the result of a round at once. Only then, when `unrecorded_latches` is not empty, it calls `Journal::set_conflict_latch` for each of them inside `spawn_blocking` and hands each receipt to `latch_recorded`, which takes the cluster off the list and gives the state the authority revision of the receipt. A write that fails leaves the cluster on the list, is logged as `journal.write_failed`, and is tried again after the next round. So a second primary closes the cluster for new sessions as fast as a round runs, whatever the disk does.
- **When the loop runs a round**: after each sample it receives, and at least once per `probe.period_ms`.
- **Publishing.** The loop maps the snapshot into the `RouteView` of `pool-core` ([Route view](#route-view)) and sends it. `pool-core` never sees a type of `pool-control`.
- **Cost** (point m6). A session reads the route view, which is one lookup by cluster. It never calls `writer_admission`. The function runs once per cluster per round. With the node counts of a lab that is nothing. A bound for very large files stays open, and scope 008 owns it, as spec 0003 allows.

The loop reads the monotonic clock with one `Instant` taken at the start of the daemon, and converts with `MonotonicTime::from_origin`. The data plane compares `fresh_until` with the same origin.

### When a running session is ended

Spec 0003 left to this slice whether a session that already runs is ended when admission closes later.

- **Silence does not end a session.** When the cluster closes with `evidence.writer_missing` alone (the probe stopped answering, or its sample expired), running sessions go on. New sessions are refused. A probe that stumbles must not cut work that still flows.
- **A positive conflict ends it.** When the reasons of the cluster hold any of `evidence.writer_identity`, `evidence.writer_in_recovery`, `evidence.writer_timeline`, `evidence.second_primary`, `evidence.timeline_ahead` or `authority.conflict_latched`, every running session of that cluster is ended: the backend is closed, and at a message boundary the client gets `08007` when a query is in flight and `57P01` when none is. Not at a message boundary the connection is closed without a message. The node is then no longer the verified primary, and POOL keeps writes on the verified primary only.

The route view carries one flag per closed cluster that says which of the two it is. Scope 008 extends this with drain.

## The wire subset

This is what the protocol owner pins before code. Everything here is tested hermetically in `pool-protocol` and `pool-core`, and the parts a real client can reach are tested on the fixture.

### Startup

A new connection sends packets without a tag. POOL reads the first 8 bytes (a length and a code) with exact reads on the socket, with no buffer in between, so that no byte a client sent before the TLS handshake can be taken for data after it.

| Code | Packet | What POOL does |
|---|---|---|
| 80877103 | `SSLRequest`, length 8 | Before TLS: answers `S`, runs the TLS handshake, then expects a new first packet. After TLS: `08P01` |
| 80877104 | `GSSENCRequest`, length 8 | Answers `N` and expects a new first packet. Allowed once, before TLS |
| 80877102 | `CancelRequest` | As the first packet, before TLS or after it: reads the rest of the packet, does nothing and closes. No answer, as PostgreSQL gives none |
| Major version 3 | `StartupMessage` | Before TLS: `28000`, in plain text, and close. After TLS: read and check as below |
| Any other major version | | `0A000` and close |
| Anything else | | `08P01` and close |

- A length below 8 is `08P01`. A length above `limits.startup_packet_max_bytes` is `54000`. Both close the connection, and the body is never read.
- A connection may send at most three first packets: an optional `GSSENCRequest`, then `SSLRequest`, then, over TLS, a `StartupMessage` or a `CancelRequest`. A `CancelRequest` may also be the very first packet. Any other order is `08P01`.
- The TLS server offers TLS 1.3 and TLS 1.2 with the defaults of `rustls`, asks for no client certificate and offers no ALPN protocol. Direct TLS (a handshake as the first bytes) is not supported: its first bytes fail the length rule.
- **Parameters.** The body is pairs of text, each ended by a zero byte, and one more zero byte at the end. A name that appears twice, an empty name, or bytes after the end are `08P01`. `user` is required, or `08P01`. `database` defaults to the value of `user`.

| Parameter | What POOL does |
|---|---|
| `user` | The claimed principal. Verified by authentication, never sent to PostgreSQL |
| `database` | The requested database of the identity map. Never sent to PostgreSQL |
| `application_name`, `client_encoding` | Sent to PostgreSQL unchanged in the startup packet of the backend |
| A name that starts with `_pq_.` | Not supported. Named in `NegotiateProtocolVersion` |
| `replication` | `0A000` |
| `options`, and every other name | `0A000`. A client with `PGOPTIONS` set in its environment sends `options` and is refused |

- **Version.** POOL speaks 3.0. For a request with a minor version above 0, or with a `_pq_.` parameter, it sends `NegotiateProtocolVersion` with minor version 0 and the names of those parameters, directly after it read the startup packet and before the authentication request, and goes on with 3.0. A client that needs more closes by itself.
- **The startup profile** of this slice has one value, `DedicatedV1`: the backend is opened with the mapped login, the mapped database and the two parameters above, when the client sent them. It is one part of the pool key.

### How each method names the principal

This slice has one frontend method. With `SCRAM-SHA-256`, the verified principal is the value of `user` in the startup packet, byte for byte, once the SCRAM proof against the verifier of the principal entry with that `name` has passed. The user name inside the SCRAM message is ignored, as PostgreSQL ignores it. The identity map is then asked with that name and the requested database. A slice that adds a method (a client certificate, for example) writes its own row here.

### Messages after login

After `ReadyForQuery` has been sent to the client, every message has a tag, then a length that counts itself and not the tag. POOL reads the 5 bytes of the header, checks them, and then copies the body through a buffer of fixed size. It never holds a whole message.

From the client, read only while no query is in flight:

| Tag | Message | What POOL does |
|---|---|---|
| `Q` | `Query` | Length above `limits.frontend_message_max_bytes`: `54000`, and no byte is sent on. Otherwise the message is sent to the backend and a query is in flight |
| `X` | `Terminate` | Sent to the backend. Both connections are closed |
| `P`, `B`, `D`, `E`, `C`, `H`, `S`, `F`, `d`, `c`, `f` | Extended protocol, function call, COPY data | `0A000`. The backend is closed |
| Any other tag, or a length below 4 | | `08P01`. The backend is closed |

From the backend, at any time:

| Tag | Message | What POOL does |
|---|---|---|
| `T`, `D`, `C`, `I`, `E`, `N`, `S`, `A` | Row description, data row, command complete, empty query, error, notice, parameter status, notification | Sent to the client unchanged, fields and all |
| `Z` | `ReadyForQuery`, length 5 | Sent to the client. The status byte is kept. No query is in flight |
| `G`, `H`, `W` | A COPY response | Not sent. The client gets `0A000`. The backend is closed |
| Any other tag, a length below 4, or a `Z` of another length | | The backend is closed and the session ends as [backend loss](#backend-loss) |

- A session is **at a message boundary** when no message from the backend is partly written to the client. POOL sends an error of its own only at a message boundary. Otherwise it closes the connection without one.
- While a query is in flight POOL does not read from the client. A second `Query` that the client already sent waits in the socket. The price: a client that goes away during a long statement is noticed when the statement produces output or ends.
- The transaction status of the last `ReadyForQuery` (`I`, `T` or `E`) is kept in the session for the log and for shutdown. No rule of this slice depends on it otherwise, because the backend is always closed at the end.
- The real `BackendKeyData` is read during the backend startup, kept in the lease and never sent. POOL sends no `BackendKeyData` of its own in this slice, so a client cannot cancel a query through POOL until scope 006 builds virtual cancellation credentials (`0001 SC-9`).

### SQLSTATEs that POOL generates

This is the shared convention for errors that POOL itself sends, which the [engineering standards](../../engineering-standards.md#shared-conventions) left to this slice. Later slices add rows and do not choose again.

- Every error of this slice is `FATAL` and is followed by closing the connection. The fields are `S` and `V` (both `FATAL`), `C` (the SQLSTATE) and `M` (the message). No other field.
- The message is constant English text from the code. It never holds a name, an address, a number that varies or text from PostgreSQL.
- An error that PostgreSQL sent during a query is never rewritten. It is passed on with every field.
- An error that PostgreSQL sent while POOL was opening the backend is never passed on, because it may name the backend login. The client gets the row "the backend could not be opened".

| Situation | SQLSTATE | Message |
|---|---|---|
| A startup packet without TLS | `28000` | `TLS is required` |
| A protocol version, parameter, message or COPY that this build does not support | `0A000` | `this protocol feature is not supported` |
| A packet or message that breaks the protocol | `08P01` | `protocol violation` |
| A startup packet or a message above its configured size | `54000` | `a message is larger than the configured limit` |
| A wrong password, or a user name that is no principal | `28P01` | `authentication failed` |
| An authenticated principal with no identity entry for the database | `28000` | `no identity entry for this user and database` |
| No session permit, no principal permit, no room among the waiters, or the wait for a backend ran out | `53300` | `too many connections` |
| The writer of the cluster is not admitted | `57P03` | `the database is not accepting connections through this proxy` |
| The backend could not be opened: no connection, TLS failed, the login failed, the node is not a primary, or the deadline passed while connecting | `08006` | `the backend connection could not be established` |
| The backend was lost, or POOL ended the session, while a query was in flight | `08007` | `the connection was ended; the outcome of the statement in flight is unknown` |
| The backend was lost while no query was in flight | `08006` | `the backend connection was lost` |
| Shutdown, or the cluster closed with a positive conflict, while no query was in flight | `57P01` | `the session was ended by the proxy` |

### Frontend authentication

After a valid startup packet over TLS:

1. POOL sends `AuthenticationSASL` with the one mechanism `SCRAM-SHA-256`. It never offers `SCRAM-SHA-256-PLUS`, because this slice does not compute channel binding data (`SEC-3`).
2. The client answers with `SASLInitialResponse`. The mechanism must be `SCRAM-SHA-256`. The first message must start with `n,,` or `y,,` and hold a nonce of printable characters. A message that asks for channel binding (`p=`), an authorization identity or an extension is `08P01`.
3. POOL sends `AuthenticationSASLContinue`: the client nonce followed by its own nonce (18 random bytes in base64), the salt and the count of the verifier.
4. The client answers with its final message. The channel binding field must repeat the header of step 2, the nonce must be the whole nonce, and the proof must be 32 bytes.
5. POOL computes the client key from the proof and the stored key of the verifier, and compares its SHA-256 with the stored key in constant time. A match makes the principal verified. POOL then sends `AuthenticationSASLFinal` with the server signature.
6. No match: `28P01`.

- **A user name that is no principal** goes through the same six steps with a made up verifier: the salt is the first 16 bytes of an HMAC of the name under a key of 32 random bytes that the daemon makes at start, and the count is the count of the verifier of the first principal in the file. A deployment makes its verifiers with one setting, so the count does not tell a real name from a made up one. Step 5 always fails. So a client cannot tell an unknown name from a wrong password, and asking twice gives the same salt.
- Each authentication message from the client is at most 1024 bytes. A longer one is `08P01`.
- The whole exchange, from the accepted connection to step 5, has the one deadline `limits.startup_timeout_ms`. At the deadline the connection is closed without a message.
- `AuthenticationOk` is not sent yet. It follows only when a backend is ready ([Session lifecycle](#session-lifecycle)). A client therefore never sees a successful login without a backend behind it.
- POOL as a server does no PBKDF2 work: the verifier already holds the keys. The cost of one login attempt is a few HMAC calls.

### Backend connection

`pool-core` opens every connection to a node the same way, for a session and for the probe.

1. Connect by TCP to the `host` and `port` of the node. A name is resolved by the runtime at each attempt, and the addresses are tried in the order they come back, all within the deadline of the attempt. A lookup uses a blocking thread of the runtime, so prefer addresses in a lab.
2. Send `SSLRequest` and read exactly one byte. Anything but `S` ends the attempt. POOL never falls back to plain text (`SEC-1`).
3. Run the TLS handshake as a client. The roots are the certificates of `node[].tls_ca_file` and nothing else. The name checked is `node[].tls_server_name`. The one TLS policy of this slice is called `VerifyFull`, and it is a part of the pool key.
4. Send a `StartupMessage` for protocol 3.0 with `user` (the backend login or the monitor login), `database`, and the forwarded parameters.
5. Authenticate. The node must ask for SASL and offer `SCRAM-SHA-256`. POOL chooses `SCRAM-SHA-256`, also when the node offers `SCRAM-SHA-256-PLUS` beside it. Its first message is `n,,n=,r=<nonce>`: the header `n` says that POOL does no channel binding (the header `y` would make PostgreSQL refuse the login as a downgrade), the name is empty because PostgreSQL takes it from the startup packet, and the nonce is 18 random bytes in base64. The first message of the node must repeat that nonce at its start, and its count must be 1 to 600000. The final message of POOL carries `c=biws`. A final message of the node that starts with `e=`, or a server signature that is wrong, ends the attempt. The salted password is derived with PBKDF2 inside `spawn_blocking`, so a high count cannot stall the runtime. The two keys that come out of it are kept in memory for the credential, together with the salt and the count they were made for, in a type that prints `[REDACTED]`. A later login with the same salt and count uses them again. PostgreSQL keeps one salt per role, so this is one derivation per credential and run, and a burst of logins cannot keep the probe from a blocking thread. A node that asks for a clear text password, for MD5, for any other method, or for nothing at all ends the attempt.
6. Read until `ReadyForQuery`. Parameter status messages are kept in order (at most 64, at most 16384 bytes together). `BackendKeyData` is kept privately. A notice is dropped. An error ends the attempt, and its SQLSTATE goes into the log event. The status must be `I`.
7. For a session only: the parameter `in_hot_standby` must have been reported as `off`. A node in recovery reports `on`, and an old or foreign server reports nothing. Both end the attempt. This is a check on the very connection the client will use, on top of the snapshot.

One attempt per session. There is no second try and no other node to try (`0001 SC-6`). For the probe, `pool-core` offers one more call on an open connection: send one constant query and return the text columns of its one row, or nothing.

## Session lifecycle

One task per frontend. The order of steps is the contract.

| Step | What happens | Refusal |
|---|---|---|
| 1 | Accept. Take a handshake permit | None left: the socket is closed at once, with no byte |
| 2 | Startup, TLS and authentication, all within `limits.startup_timeout_ms` from the accept | As [Startup](#startup) and [Frontend authentication](#frontend-authentication) say |
| 3 | Take a session permit, then a permit of the principal. Give the handshake permit back | `53300` |
| 4 | Ask the identity map with the verified principal and the requested database | `28000` |
| 5 | Read the route view for the cluster of the identity entry. It must be open and `fresh_until` must lie in the future | `57P03` |
| 6 | Compute the pool key | |
| 7 | Take a backend permit of the node. When none is free, wait for one, as one of at most `limits.backend_wait_max_count` waiters, until `limits.backend_acquire_timeout_ms` after step 5 | `53300` |
| 8 | Open the backend, within the same deadline | `08006` |
| 9 | Read the route view again. The cluster must still be open with the same node, topology epoch and authority revision, and still fresh. The pool key, computed again, must equal the one of step 6, and the configuration generation must be the current one | The backend is closed. `57P03` |
| 10 | Create the lease with a new lease generation. Send `AuthenticationOk`, the parameter status messages of the backend, and `ReadyForQuery` | |
| 11 | Relay, as [Messages after login](#messages-after-login) says | |
| 12 | End: close the backend, give every permit back, write `session.ended` | |

- Steps 5 and 9 are "the minimum durable writer check" that scope 004 names. No byte of application SQL can reach a node before step 10, and step 10 is reached only through both.
- A permit is given back when the value that holds it is dropped, so every way out of the task returns it, including a panic of that task.
- `session.started` is written at step 10. Each refusal adds to the counters behind `session.refused`.

### Limits

| Limit | Key | What is counted | When it is reached |
|---|---|---|---|
| Handshakes | `limits.handshake_max_count` | Connections from accept to the end of step 2 | The new socket is closed at once |
| Startup deadline | `limits.startup_timeout_ms` | Time from accept to the end of authentication | The connection is closed without a message |
| Startup packet | `limits.startup_packet_max_bytes` | The length field of a first packet | `54000` |
| Sessions | `limits.session_max_count` | Frontends from step 3 to step 12 | `53300` |
| Sessions of one principal | `principal[].session_max_count` | The same, per principal | `53300` |
| Backends of one node | `limits.node_backend_max_count` | Backends of sessions on that node. The probe is not counted | The session waits |
| Waiters | `limits.backend_wait_max_count` | Sessions waiting in step 7, over all nodes | `53300` at once |
| Acquire deadline | `limits.backend_acquire_timeout_ms` | Time from step 5 to the end of step 8 | While waiting: `53300`. While connecting: `08006` |
| Message size | `limits.frontend_message_max_bytes` | The length field of a message from the client | `54000` |
| Authentication message | 1024 bytes, a constant | One SASL message from the client | `08P01` |
| Shutdown deadline | `limits.shutdown_timeout_ms` | Time from the signal | Every session is ended |

Waiters are served in the order they arrived. Per identity limits nest inside the global ones: a principal limit can never raise the session limit. There is no idle limit, no lifetime limit and no deadline on a client that reads slowly. Those belong to scope 005, and until then a slow client holds its own session and its own backend and nothing else.

### Memory budget

`PRD-001 AC-3` asks this spec to declare a budget. It is: **64 MiB for the process plus 1 MiB for each session**, at any setting of the limits.

Where the bound comes from:

- Each direction of a session has one relay buffer of 65536 bytes (the constant `RELAY_BUFFER_BYTES`). A session writes what it read before it reads again, so it never queues more than one buffer per direction.
- Each TLS connection is told to queue at most 65536 bytes for sending (`set_buffer_limit`). What `rustls` holds of received plain text has a limit of its own of about 16 KiB. A session has two connections.
- A startup packet is at most `limits.startup_packet_max_bytes`, a SASL message at most 1024 bytes, and the kept parameter status messages at most 16384 bytes.
- A message body is never collected. A length field is checked before anything is reserved for it, and nothing is reserved from it.

Together that is well under 512 KiB per session. The budget says 1 MiB so that the test has room and still fails for code that buffers results. With the default of 1000 sessions the bound is about 1 GiB. If you run POOL in a machine with less, lower `limits.session_max_count`.

File descriptors have a budget too. A session holds two, a handshake one, and each node one for the probe. With the defaults that is more than 2100, so the service needs a limit on open files (`LimitNOFILE` in a systemd unit) of at least twice `limits.session_max_count` plus `limits.handshake_max_count` plus 64. When an accept fails, for this or any other reason, the listener counts it under `session.refused` with the reason `accept_failed`, waits 100 ms and goes on. It never ends the daemon.

### Backend loss

When a read from or a write to the backend fails, or the backend closes, or it sends something [Messages after login](#messages-after-login) does not accept:

- The session ends. POOL opens no new backend for it and sends nothing again (`0001 SC-6`).
- At a message boundary the client gets one error: `08007` when a query is in flight, `08006` when none is. Not at a message boundary the connection is closed without one.
- A query is in flight from the moment the first byte of a `Query` was written toward the backend until its `ReadyForQuery` was written to the client. So a COMMIT whose answer was lost gives `08007`. POOL does not know whether it was applied and says exactly that.

When the client goes away (it closes, or a read or write fails), POOL closes the backend. PostgreSQL then rolls back whatever was open. POOL sends `Terminate` to the backend first only when no query is in flight.

### Shutdown

On SIGTERM or SIGINT:

1. The listener stops accepting. Connections that are still in steps 1 to 9 are closed.
2. Every session with no query in flight and the status `I` gets `57P01` and is ended.
3. Every other session is ended in the same way as soon as it reaches that state.
4. When `limits.shutdown_timeout_ms` has passed, every session that is left is ended. At a message boundary it gets `08007` when a query is in flight and `57P01` when none is. Otherwise it gets no message. POOL never tells a client whose statement may have been applied that it was simply ended (`R-10`).
5. The control loop stops, the journal is closed, `daemon.stopped` is written, and the process exits 0.

A second signal during shutdown does nothing more. This is the minimum for a clean stop. Drain with diagnostics is scope 008.

### Route view

The type that the data plane reads. `pool-core` owns it, and the control loop in `apps/pool` fills it from a `TopologySnapshot`.

```rust
// pool-core
pub struct RouteView { /* private */ }
impl RouteView {
    pub fn closed(clusters: &[ClusterId]) -> RouteView;
    pub fn new(routes: Vec<(ClusterId, RouteState)>) -> RouteView;
}
pub enum RouteState {
    Open(Route),
    Closed { conflict: bool },
}
pub struct Route {
    pub node: NodeId,
    pub topology_epoch: TopologyEpoch,
    pub authority_revision: AuthorityRevision,
    pub fresh_until: std::time::Duration,   // since the origin the daemon passed at start
}
```

- A cluster that the view does not hold is closed.
- `conflict` is true when the reasons of the cluster hold one of the names under [When a running session is ended](#when-a-running-session-is-ended).
- Sessions hold a `watch::Receiver` of the view. They read it in steps 5 and 9 and wake on a change while they relay.
- If the control loop stops publishing, every route goes stale at its `fresh_until` and new sessions are refused. Nothing has to tell the data plane.

## Records built in this slice

The contract of each record is in the [Records](../0003-control-and-session-data-model/index.md#records) table of spec 0003. This is where each one lands.

| Record | Type and home | Notes for this slice |
|---|---|---|
| Credential | `Credential` in `pool-core`. `Debug` and `Display` print `[REDACTED]`. No `Serialize`, no `Clone` outside the crate | Read once at start. Credential generation 1. The two SCRAM keys derived from it are kept beside it, under the same rules |
| Verifier | `Verifier` in `pool-core`, with the same rules | New in this spec. Memory only. Read once at start |
| Verified principal | `VerifiedPrincipal` in `pool-core`. Made only by the authentication code. Holds the `PrincipalId` and the name | Ends with the session |
| Session | `Session` in `pool-core`. The ID is a `u128`: the start sequence in the high 64 bits and a counter of this run in the low 64 bits | So an ID is unique across restarts without a random source. Logged as 32 hex characters |
| Lease | `Lease` in `pool-core`. Owns the backend connection, the backend permit, the lease generation, the pool key and the private `BackendKeyData` | One per session, never moved (`0001 SC-1`) |
| Lease generation | `LeaseGeneration` in `pool-core`, the shape of the generation types of spec 0003, first value 1 | Advanced for each lease. At the largest value new sessions are refused with `53300` |
| Pool key | `PoolKey` in `pool-core`, with the eight parts of `0001 SC-2`: cluster ID, node ID, backend database, backend login, credential generation, topology epoch, TLS policy (`VerifyFull`) and startup profile (`DedicatedV1`) | Computed in step 6, compared in step 9. Scope 005 partitions reuse by it. `Debug` prints the IDs and generations and `[REDACTED]` for the database and the login |
| Configuration generation | `ConfigurationGeneration` in `pool-config` | 1 for the whole run |
| Topology snapshot and snapshot generation | `TopologySnapshot`, `SnapshotGeneration` in `pool-control` | See [Control loop and topology snapshot](#control-loop-and-topology-snapshot) |
| Virtual cancellation credential | Not built | Scope 006 |

## Log events

`tracing` is adopted for the events. The writer is first party code in `apps/pool`: a type that implements `tracing::Subscriber`, turns each event into one JSON line and ignores spans, which this slice does not use. It does not write the line itself. It puts it into a bounded queue of 1024 lines with `try_send`, and one task takes lines from the queue and writes them to stderr through `tokio::io::stderr`. So a reader of stderr that is slow can never hold a worker of the runtime. When the queue is full the line is dropped and counted, and the next line that fits is preceded by the event `log.dropped` with the count. It follows the [log rules](../0001-initial-architecture/index.md#log-events-and-redaction) of spec 0001.

- A line is one JSON object. `ts`, `level` and `event` come first, in that order, then the fields in the order the code gave them.
- `ts` is UTC in the form `2026-11-15T08:56:01.123Z`, computed from the wall clock by first party code.
- `event` is a field every call must give, from a constant. A call without it is written with the event name `log.unnamed` and the level `error`, so that a mistake shows up in tests.
- A field value is a string, a whole number or a boolean. A list is one string with its items joined by commas.
- The level is fixed at `info` in this slice. There is no key for it yet.
- Nothing writes to stdout while the daemon serves.

The fields below are the only ones. This table, with the [telemetry table](../0003-control-and-session-data-model/index.md#what-may-appear-in-default-telemetry) of spec 0003, is the permission the log rules ask for. New here: the principal ID (an identifier you choose, like the identity ID), the session ID, a SQLSTATE, and counts and durations.

| Event | Level | Fields |
|---|---|---|
| `daemon.started` | info | `start_sequence`, `configuration_generation`, `authority_id` |
| `daemon.stopping` | info | `sessions` |
| `daemon.stopped` | info | none |
| `log.dropped` | warn | `count` |
| `admission.changed` | info | `cluster_id`, `state` (`open` or `closed`), `node_id` when open, `reasons` when closed, `snapshot_generation`. Written when the state or the reasons of a cluster change |
| `authority.conflict_latched` | warn | `cluster_id`, `recorded` (whether the journal holds it) |
| `journal.write_failed` | error | `class` (the name of the `JournalError`) |
| `probe.failed` | warn | `node_id`, `class`: `connect`, `tls`, `login`, `timeout` or `answer`. Written when the class of a node changes, not for every sample |
| `probe.recovered` | info | `node_id` |
| `session.started` | info | `session`, `principal_id`, `identity_id`, `cluster_id`, `node_id`, `lease_generation`, `topology_epoch` |
| `session.ended` | info | `session`, `reason`: `client_terminate`, `client_closed`, `backend_lost`, `unsupported`, `protocol`, `message_limit`, `conflict` or `shutdown`; `status`: `idle`, `in_transaction`, `failed` or `in_flight`; `duration_ms` |
| `session.refused` | warn | `reason`, `count`. One line per reason and second, for the refusals of that second: `handshake_limit`, `startup_timeout`, `plaintext`, `tls`, `protocol`, `unsupported`, `message_limit`, `authentication`, `unmapped`, `session_limit`, `principal_limit`, `closed`, `wait_limit`, `acquire_timeout`, `backend_unavailable`, `cancel_ignored`, `accept_failed` |
| `backend.failed` | warn | `node_id`, `class`: `connect`, `tls`, `auth_method`, `login`, `not_primary`, `protocol` or `timeout`; `sqlstate` when PostgreSQL sent one; `count`. One line per node, class and second |

Refusals are counted and written once a second, so that a flood of bad connections cannot flood the log. Never in any event: a principal name, a login, a database name, a host, a port, a path, SQL, a parameter value, error text from PostgreSQL, a nonce, a proof or a key (`SEC-11`).

## Value sourcing

| Value | Source |
|---|---|
| Principal claimed by a client | `user` in the startup packet |
| Verified principal | The claimed name, after the SCRAM proof against the verifier of the principal entry with that name |
| Verifier | The file named by `principal[].verifier_file`, which you fill from `pg_authid.rolpassword` of a PostgreSQL 18 |
| Salt and count sent to a client | The verifier. For an unknown name: an HMAC of the name under a key made at start, and the count of the first principal's verifier |
| Server nonce, mock key, authority ID | Random bytes of the operating system, through `aws-lc-rs` |
| Requested database | `database` in the startup packet, or the value of `user` when it is absent |
| Cluster, backend login, backend database, credential | The identity entry that `resolve_identity` returns. Nothing from the startup packet |
| Backend password | The file named by `identity[].credential_file` |
| Monitor login, database and password | `cluster[].monitor_login`, `monitor_database`, `monitor_credential_file` |
| Node address | `node[].host` and `node[].port` of the node the route names, in the one configuration generation |
| Trust roots and name for the node | `node[].tls_ca_file` and `node[].tls_server_name` |
| Certificate and key of the listener | `listener.tls_certificate_file` and `listener.tls_private_key_file` |
| Eligible primary of a cluster | The route view, filled from `build_snapshot`, which calls `writer_admission` |
| `fresh_until` | The sample time of the approved writer plus `probe.observation_expiry_ms` |
| Observed system identifier, role and timeline | The probe query |
| `now` and sample times | One `Instant` taken at the start of the daemon, and the time since it |
| Parameter status sent to the client | What the backend reported during its startup, unchanged |
| Transaction status | The status byte of the last `ReadyForQuery` from the backend |
| Session ID | The start sequence that `Journal::open_ready` returned, and a counter |
| Lease generation, snapshot generation | Counters of this run |
| Topology epoch and authority revision in a route | `EligiblePrimary` of the snapshot |
| Wall clock times in the journal and in `ts` | The system clock, read in `apps/pool` |
| Image of the fixture | `postgres:18.7-bookworm`, index digest `sha256:ece55f7868da684e400f82f1030a050718adbdd65c23aa500c486f6e2950dc0f`, read from the registry on 2026-11-15, written once in `scripts/check.sh` |
| Passwords, names and certificates of the fixture | Constants and generated files of `scripts/integration/`. They are test values and are used nowhere else |

## Security model

- **Who may connect.** Anyone who reaches the listener may start a TLS handshake. Before authentication a connection can cost at most one handshake permit, one startup packet of bounded size, a few HMAC calls and `limits.startup_timeout_ms` (`SEC-8`, `T-01`).
- **Who may log in.** A client that proves the password behind the verifier of a configured principal. Nothing else is a principal (`SEC-2`).
- **What a principal may reach.** Exactly the databases for which the identity map has an entry with its name, each as the backend login of that entry. A client cannot choose a backend login, a credential, a cluster or a node (`0001 SC-3`, `T-04`). What that login may do inside PostgreSQL is decided by PostgreSQL (`SEC-5`).
- **Two separate proofs.** The frontend verifier cannot be used against PostgreSQL. The backend password never leaves POOL, and neither side learns the secret of the other. POOL never logs in as a shared privileged role.
- **Transport.** TLS is required from the client and toward every node. Toward a node the chain and the name are checked against the CA of that node, and there is no fallback (`SEC-1`, `T-03`). Channel binding is not offered and not used in this slice. With a checked certificate on both legs that leaves no gap that this slice knows of, and scope 006 adds it.
- **Secrets.** They live in files with owner only access, are read once, are held in types that print `[REDACTED]`, and appear in no output, no log and no error (`SEC-4`, `SEC-11`, `T-12`). Memory is not wiped when a secret is dropped. A core dump can hold them, as the operational defaults of the security requirements already say.
- **Authority.** A node receives SQL only while the journal, the configuration and a fresh probe agree that it is the writer, checked twice per login and once more on the connection itself (`0001 SC-8`, `R-19`).
- **Administration.** There is none on the network. The three commands act on local files and need the rights of the service account (`SEC-6` first applies in scope 009).
- **What this does not defend.** Root on the proxy host, a superuser of PostgreSQL, and a client that holds a valid password. A valid client can hold a session and a backend for as long as it likes, up to the limits. One address that opens connections and sends nothing can hold every handshake permit for `limits.startup_timeout_ms` at a time, and so keep others from logging in. This slice has no limit per address. A firewall in front of the listener is the answer until scope 005.

## Tests and evidence

Tests use the built in `cargo test` harness only. No package gets a dependency for tests. Integration tests of `apps/pool` may use `tokio`, because the binary already depends on it.

### Tests and who owns them

| Package | File | Covers |
|---|---|---|
| `pool-protocol` | Unit tests beside the code, and `crates/pool-protocol/tests/golden.rs` | Golden packets and messages, whole and split at every byte (AC-28). Every row of the tables under [Startup](#startup) and [Messages after login](#messages-after-login) that needs no socket. The bytes of each error message. SCRAM messages and base64 |
| `pool-config` | `crates/pool-config/tests/validate.rs` | Every new fixture, the defaults, the two views, the name rule, the finding limit with made input 5 (AC-5 to AC-8) |
| `pool` | `apps/pool/tests/check_config.rs` | The new fixtures and made input 5 through the built binary, both formats (AC-5 to AC-7) |
| `pool-control` | `crates/pool-control/tests/walkthroughs.rs` | The walk from row d to row o through the real functions, with and without a recorded latch (`0003 AC-17`) |
| `pool-control` | `crates/pool-control/tests/journal.rs` | Create, read, every `Unreadable` case, `open_ready`, the latch, on real files under `CARGO_TARGET_TMPDIR` (AC-10) |
| `pool-control` | `crates/pool-control/tests/controller.rs` | The board, `build_snapshot`, `Controller::round`, a latch that is recorded, one whose write fails and is tried again, `JournalRead`, the probe row (AC-12, `0003 AC-18`) |
| `pool-core` | Unit tests beside the code, and `crates/pool-core/tests/session.rs` | The whole session over in memory streams (`tokio::io::duplex`), with a scripted client and a scripted backend written with the encoders of `pool-protocol`: steps 1 to 12, every refusal, every limit, backend loss, the conflict ending, shutdown, the pool key, the redaction of secret types (hermetic halves of AC-14 to AC-17, AC-19, AC-20, AC-22, AC-23, AC-25) |
| `pool` | `apps/pool/tests/commands.rs` | The three new commands without a database: usage, the views, the state directory checks, the lock, secret file checks, `show-authority` in each state, bootstrap against a node that does not answer, and from Build plan step 10 on `run` in each state that is not ready (AC-9, AC-11, AC-13) |
| `pool` | `apps/pool/tests/integration.rs`, with helpers under `apps/pool/tests/support/` | Everything on the fixture. Every test in it is marked `#[ignore]` with the reason `needs scripts/check.sh integration`, so `scripts/check.sh test` stays free of Docker |
| `pool-repo-check` | `tools/repo-check/` | The new format of rule W3 with one passing and one failing test per kind of line, and the lockfile entry end (AC-2, AC-4) |

Hermetic session tests put a pass through in the place of the TLS handshake, so no private key is committed. Real TLS on both legs is tested on the fixture.

### The integration check

`scripts/check.sh integration` is a new check name, built by Build plan step 8, under the [rules of spec 0002](../0002-standards-and-tooling/index.md#rules-for-later-slices).

1. It checks `docker info`. Without a daemon it prints the reason and counts as could not run (exit 3).
2. It makes a temporary directory with mode `0700` and, in it, the certificates of the fixture, once: a test CA, a server certificate for the name `pg.pool.test` and a certificate for POOL with the name `host.docker.internal`. It makes them with `openssl` in a short run of the same image (`docker run --rm`), so the host needs no tool. A `trap` removes every container and the directory on every exit.
3. It starts two containers from `postgres:18.7-bookworm@sha256:ece55f7868da684e400f82f1030a050718adbdd65c23aa500c486f6e2950dc0f`, each with a name that starts with `pool-it-` and ends with the process ID of the script, with `-p 127.0.0.1::5432`, with `--add-host=host.docker.internal:host-gateway`, with `scripts/integration/initdb/` mounted read only at `/docker-entrypoint-initdb.d`, and with the certificates mounted read only. Both get the same server certificate, the same roles and the same passwords. They differ in one thing that PostgreSQL makes by itself: the system identifier. The second one is the wrong identity of AC-14, and because it passes TLS and the login, the probe really reaches the identity rule.
4. The init script, run by the image as the database owner, copies the server certificate and key into the data directory, turns on `ssl`, allows only `hostssl` with `scram-sha-256` from the network, sets `log_connections` and `log_statement = 'all'`, and creates the roles, the database and the table of the fixture.
   The script then waits until `pg_isready` answers on both, and writes the credential and verifier files into the temporary directory with mode `0600`. For the verifier it creates a role with `NOLOGIN` and the password of the principal on the first container, reads `rolpassword` from `pg_authid` with `docker exec`, and drops the role. So the principal cannot log in to PostgreSQL by any name.
5. It exports the environment below and runs `cargo test -p pool --locked --test integration -- --ignored --test-threads=1`.
6. It removes the containers and reports the result.

| Variable | Meaning |
|---|---|
| `POOL_IT_CONTAINER`, `POOL_IT_CONTAINER_2` | The container names, for `docker exec` and `docker logs` |
| `POOL_IT_PG_PORT`, `POOL_IT_PG_PORT_2` | The published ports on `127.0.0.1` |
| `POOL_IT_DIR` | The temporary directory with the certificates and secret files |
| `POOL_IT_LISTEN_HOST` | The address POOL binds: `127.0.0.1` when `docker info --format '{{.OperatingSystem}}'` holds the text `Docker Desktop`, and otherwise the gateway address of the default bridge network, read with `docker network inspect bridge` |
| `POOL_IT_CLIENT_HOST` | The name `psql` connects to: `host.docker.internal` |

The fixture, all test values:

| Thing | Value |
|---|---|
| Client | `psql` of the same image, run with `docker exec`, with `sslmode=verify-full` and the test CA |
| Principal, as the client logs in | `pz7_principal`, asking for the database `pz7_db` |
| Identity entry | Principal `pz7_principal`, database `pz7_db`, backend login `pz7_rw`, backend database `pz7_appdb` |
| Backend login | `pz7_rw`, with another password than the principal. Owner of the table `pz7_items` in `pz7_appdb`, no other right |
| Monitor login | `pz7_monitor`, `LOGIN` only |
| Direct baseline | `psql` as `pz7_rw` on `pz7_appdb`, straight to PostgreSQL over TLS |
| The SQL file of AC-18 | `TRUNCATE pz7_items`, then a transaction that inserts three rows with fixed keys and values and commits, a SELECT of them in key order, and a division by zero. No sequence and no time, so both runs print the same bytes |

The names start with `pz7_` so that the redaction test of AC-24 can search the whole output for that text and for each password, and a match is always a leak.

Rules for the tests: each test writes its own configuration and state directory under `CARGO_TARGET_TMPDIR`, picks a free port by binding port 0 and releasing it, starts the built `pool` binary, and stops it at the end. A test contacts only the addresses of the table above. The same SQL file is used for the direct run and the run through POOL. Things that happen a moment after the client sees its answer (a backend that exits, a permit that comes back, a log line) are checked by asking again every 50 ms for at most 2 seconds, never by one fixed wait.

How the hard cases are made:

- **Lost COMMIT (AC-22).** The test runs a small relay as a task in its own process: it listens on a free port, passes bytes both ways between POOL and PostgreSQL, and is the `host` and `port` of the node in the configuration. TLS passes through it unchanged, and the name check still holds through `tls_server_name`. After the transaction is open, the test arms the relay. The relay passes the next bytes from POOL on to PostgreSQL, waits until PostgreSQL has answered, drops that answer and closes its connection to POOL. The test then reads the PostgreSQL log with `docker logs` and counts the COMMIT of that session by a marker that is written as a `/* */` comment inside the same statement.
- **Memory (AC-21).** The sessions are `psql` processes inside the container that are stopped with SIGSTOP once their query runs. The test reads the resident memory of the `pool` process with `ps -o rss=`.
- **Waiters and deadlines (AC-20).** Small limits in the configuration: for example `session_max_count = 3`, `node_backend_max_count = 1`, `backend_wait_max_count = 1`, `backend_acquire_timeout_ms = 1000`.
- **Wrong identity (AC-14).** The journal is made against the first container. The node in the configuration then points at the second one, which has the same certificate, roles and passwords and another system identifier. The test waits for the event `admission.changed` with `evidence.writer_identity` before it tries to log in.

### Fuzzing

`crates/pool-protocol/fuzz/` is a package of the same shape as `crates/pool-config/fuzz/`, with its own committed lockfile and three targets:

| Target | Feeds | Must not |
|---|---|---|
| `startup` | The first packet reader and the parameter parser | Panic, or reserve memory from a length field |
| `frontend_message` | The header reader for client and backend messages | The same |
| `scram` | The four SCRAM message parsers and the base64 decoder | The same |

- Each target has a directory `fuzz/regressions/<target>/` with committed inputs. It starts with the golden packets of the tests. An input that made a target crash is committed there with the fix.
- `scripts/check.sh fuzz` now loops over both fuzz packages and every target. For each it runs the target on its corpus, its seed directory and its regression directory for `POOL_FUZZ_SECONDS`, 30 by default. Exit 0 with no file under `fuzz/artifacts/` is a pass.
- `scripts/check.sh fuzz-tools` is a new name. It installs `cargo-fuzz` 0.13.2 under `target/tools` and the toolchain `nightly-2026-10-17` with `rustup`, when they are missing. It exists so that the CI job has a `run:` line that rule C3 accepts.
- Rule W1 gets a baseline file `protocol-fuzz-lock-packages.txt` for the new lockfile, rule W6 lists four lockfiles, `fmt` formats the new manifest, and `deny` checks the new lockfile.

### What proves each criterion

| Criterion | Command | Where it runs |
|---|---|---|
| AC-1 | `cargo metadata --no-deps --format-version 1`, `scripts/check.sh guards`, `cargo build --workspace --locked` | Workstation and CI |
| AC-2, AC-4 | `cargo test --manifest-path tools/repo-check/Cargo.toml --locked --target-dir target/repo-check`, `scripts/prove-checks.sh` | Workstation |
| AC-3 | `scripts/check.sh deny`, `scripts/check.sh linux-build` | Workstation with Docker, and CI |
| AC-5 to AC-8 | `cargo test -p pool-config --locked`, `cargo test -p pool --locked --test check_config` | Workstation and CI |
| AC-9, AC-13 | `cargo test -p pool --locked --test commands` | Workstation and CI |
| AC-10 | `cargo test -p pool-control --locked --test journal`, and `scripts/check.sh integration` for the file that bootstrap wrote | Workstation and CI |
| AC-11, AC-12 | `scripts/check.sh integration`, with `cargo test -p pool-control --locked --test controller` for the library half | PostgreSQL 18.7 in the container |
| AC-14 to AC-17, AC-19, AC-20, AC-22, AC-23, AC-25 | `cargo test -p pool-core --locked` for the hermetic half, `scripts/check.sh integration` for the half on the fixture | Both |
| AC-18, AC-21, AC-24 | `scripts/check.sh integration` | PostgreSQL 18.7 in the container |
| AC-26 | `scripts/check.sh integration`, once with Docker and once with `DOCKER_HOST` pointing at nothing | Workstation |
| AC-27 | `scripts/check.sh fuzz` | Workstation, and CI after manual step M2 |
| AC-28 | `cargo test -p pool-protocol --locked` | Workstation and CI |
| `0003 AC-17`, `0003 AC-18` | `cargo test -p pool-control --locked` | Workstation and CI |
| `0003 AC-19` | `scripts/check.sh guards`, and its case in `scripts/prove-checks.sh` | Workstation and CI |

The declared environment of the evidence is PostgreSQL 18.7 from the pinned image, `psql` 18.7 from the same image, and the `pool` binary built from the commit under test. A run on a macOS workstation is evidence for the behavior against real PostgreSQL 18. It is not Linux evidence. The Linux evidence is the CI job `integration` on the hosted runner. It stays `NOT_RUN` in the verification record until manual step M2 and a pull request have happened. The same holds for the CI job `fuzz`. `scripts/check.sh linux-build` still proves that the tree builds and passes its hermetic tests on Linux.

## CI definition and manual steps

No session edits `.github/` or writes to GitHub. These steps are yours, or the lead's with your permission for each one. Each has `owner: user`.

**M1. License decision.** Needed before Build plan step 3 is merged. You decide, as Release maintainer, whether these licenses may enter the dependency tree, for these packages only:

| Package | Version | Declared license | Why it is there |
|---|---|---|---|
| `aws-lc-rs` | 1.19.0 | `ISC AND (Apache-2.0 OR ISC)` | The crypto library of `rustls`, and SCRAM |
| `aws-lc-sys` | 0.47.0 | `ISC AND (Apache-2.0 OR ISC) AND Apache-2.0 AND MIT AND BSD-3-Clause AND (Apache-2.0 OR ISC OR MIT) AND (Apache-2.0 OR ISC OR MIT-0)` | The C code of AWS-LC |
| `rustls-webpki` | 0.103.17 | `ISC` | Certificate checks of `rustls` |
| `untrusted` | 0.9.0 | `ISC` | Used by `rustls-webpki` |
| `subtle` | 2.6.1 | `BSD-3-Clause` | Constant time helpers of `rustls` |

Two licenses are new: `ISC` and `BSD-3-Clause`. Both are permissive and are compatible with Apache 2.0. SQLite itself is in the public domain and comes in through `libsqlite3-sys`, whose own license is MIT. A yes means the five exceptions under [Dependency policy](#dependency-policy) and the `NOTICE` file. A no for AWS-LC means the runner up, `ring`, which needs the same two licenses and one more exception for a duplicate package, so a no to `ISC` itself stops the TLS decision of spec 0001 and comes back to you.

**M2. CI jobs and the branch rule file.** After Build plan step 12. Add these two jobs to `.github/workflows/ci.yml`, after the job `docs`, and the two contexts `integration` and `fuzz` to `required_status_checks` in `.github/rulesets/main.json`, in one commit, so that rule C2 holds.

```yaml
  integration:
    runs-on: ubuntu-24.04-arm
    timeout-minutes: 30
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
        with:
          persist-credentials: false
      - run: rustup toolchain install
      - run: scripts/check.sh integration

  fuzz:
    runs-on: ubuntu-24.04-arm
    timeout-minutes: 30
    steps:
      - uses: actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1
        with:
          persist-credentials: false
      - run: rustup toolchain install
      - uses: actions/cache@55cc8345863c7cc4c66a329aec7e433d2d1c52a9 # v6.1.0
        with:
          path: target/tools
          key: tools-${{ runner.os }}-${{ runner.arch }}-${{ hashFiles('scripts/check.sh') }}
      - run: scripts/check.sh fuzz-tools
      - run: scripts/check.sh fuzz
```

Rule C3 of spec 0002 allows `scripts/check.sh <name>` where the name is a job ID or `tools`. Build plan step 12 adds `fuzz-tools` to that list in `tools/repo-check/`, with a test. No other CI rule changes: both jobs use the allowed keys, the one runner label and pinned actions.

**M3. Push, pull request, and the rule on the remote.** Push the scope branch, open the pull request, and after the nine checks have run on it, apply `main.json` to the ruleset on the remote, as the [remote proof](../0002-standards-and-tooling/index.md#remote-proof) of spec 0002 did for the first seven. Until then the two new jobs run and do not yet block a merge.

**M4. Docker.** `scripts/check.sh integration`, `scripts/check.sh linux-build` and `scripts/prove-checks.sh --with-linux-build` need a running Docker daemon. Starting it is yours.

Nothing in this slice needs a VM or a PostgreSQL outside the containers that the integration check starts and removes. No value in this spec is one that only you know.

## Document corrections

Edits to files this spec does not own. The Build plan names who makes each one.

| Item | File | Change | Made by |
|---|---|---|---|
| 1 | `docs/engineering-standards.md` | In the shared conventions table, replace `Pending` in the row "SQLSTATEs and messages that POOL itself generates" with: every generated error is `FATAL` with the fields `S`, `V`, `C` and `M`, constant text, one SQLSTATE per situation from a fixed table, server errors during a query passed on unchanged; full rule linked to [SQLSTATEs that POOL generates](#sqlstates-that-pool-generates). First paragraph: the workspace has five members | Build plan step 13 |
| 2 | `docs/repository-structure.md` | First paragraph: add `crates/pool-protocol` and `crates/pool-core` to the paths that exist | Build plan step 13 |
| 3 | `docs/research/technology-evaluation.md` | A section "Adopted in scope 004": the six dependencies with the versions and features Cargo locked, the two native libraries, the licenses decided in M1, the image tag and digest, and the versions observed in the build | Build plan steps 3 and 8 |
| 4 | `docs/index.md` | Rows for `verify.md` of this spec and for the context files of the two new crates | Build plan step 14, and the "Document it" box |
| 5 | `AGENTS.md`, `CLAUDE.md`, `CONTRIBUTING.md`, `README.md`, `CHANGELOG.md`, `apps/pool/AGENTS.md`, the context files of the crates, `docs/testing/strategy.md`, `docs/feature-matrix.md`, `docs/plan/risk-register.md` | Name the new crates, commands, keys and checks. State what has evidence and what does not | The "Document it" box of scope 004 |
| 6 | Spec 0001 and spec 0002 | Dated notes that point here ([Follow-up](#follow-up) 1 and 2) | The lead, after this spec is accepted. Applied on 2026-11-15 |

## Build plan

The order is a tracer bullet (one thin real path first, then thicker). The first real contact with PostgreSQL is step 8, where `pool bootstrap` probes a node over TLS and writes the journal. The first real session is step 10. Each step leaves a tree that passes `scripts/check.sh all`, and each ends with a commit named `<type>(<scope>): spec 0004 step <n>, ...`. Run `scripts/check.sh fmt` and `scripts/check.sh clippy` before each commit.

1. **Proof script and lockfile check.** Change case 17 and the lockfile entry end as [Proof script and lockfile check](#proof-script-and-lockfile-check) says, with the changed test and the new case. This step comes first because case 17 fails on `main` from the moment scope 004 is `in-progress`. Files: `scripts/prove-checks.sh`, `tools/repo-check/src/`. Evidence: the tests of `tools/repo-check`, and `scripts/prove-checks.sh` after the commit. Satisfies **AC-4**.
2. **Source guard.** Teach rule W3 the three kinds of line, replace `forbidden-source-patterns.txt` with the file under [Source guard](#source-guard), and add the six proof cases. Files: `tools/repo-check/src/`, `tools/repo-check/baseline/forbidden-source-patterns.txt`, `scripts/prove-checks.sh`. Evidence: as step 1, and `scripts/check.sh guards`. Satisfies **AC-2**, `0003 AC-19` and the guard half of `0003 AC-18`.
3. **Workspace shape and dependencies.** Needs manual step M1. Create `crates/pool-protocol` and `crates/pool-core` with a manifest and a `src/lib.rs` that holds only the crate documentation. Add the six dependencies and the two members to the root `Cargo.toml`, and the dependencies of each member as the table under [Workspace](#workspace) says. Run `cargo build --workspace` once so that `Cargo.lock` gains the packages, and compare the names with the [rationale](rationale.md#locked-packages). Change `lock-packages.txt`, `forbidden-paths.txt` and `internal-dependencies.txt`, and point case 9 of `scripts/prove-checks.sh` at `apps/pool-cli`. Add the five exceptions to `deny.toml`. Write `NOTICE`. Record the adoption in the technology evaluation. Set this spec's `**Status**` line to `In Progress`. Files: `Cargo.toml`, `Cargo.lock`, the five manifests, the two new `src/lib.rs`, `tools/repo-check/baseline/`, `scripts/prove-checks.sh`, `deny.toml`, `NOTICE`, `docs/research/technology-evaluation.md`. Evidence: `scripts/check.sh all`, `scripts/check.sh linux-build`, and `scripts/prove-checks.sh` after the commit. Satisfies **AC-3** and the shape of **AC-1**.
4. **The wire format.** Build `pool-protocol` as [The wire subset](#the-wire-subset) says: first packets, parameters, message headers, the error message, the authentication messages, `NegotiateProtocolVersion`, the SCRAM text messages and base64, each as a function over bytes. Write the golden tests. Needs step 3. Files: `crates/pool-protocol/`. Evidence: `cargo test -p pool-protocol --locked`. Satisfies **AC-28**, and the byte level half of **AC-15**, **AC-16** and **AC-23**.
5. **Configuration.** Implement [Configuration](#configuration) in `pool-config`: the keys, the two codes, the name rule, the finding limit, the two views, the two generation types and the fixtures, with the one changed line in `invalid-unknown-key.toml` and in the two tests that [Fixtures](#fixtures) names. Wrap the output of the binary in a `BufWriter` and add the omitted line and field to `apps/pool/src/report.rs`. Files: `crates/pool-config/`, `apps/pool/src/report.rs`, `apps/pool/src/main.rs`, `apps/pool/tests/check_config.rs`. Evidence: `cargo test -p pool-config --locked`, `cargo test -p pool --locked`. Satisfies **AC-5** to **AC-8**.
6. **Authority on disk and the control loop.** In `pool-control`: `journal.rs` with `JournalRead`, `AuthorityState::latch_conflict`, `latch_recorded` and `name`, `ObservationBoard`, `SnapshotGeneration`, `build_snapshot`, `Controller`, `PROBE_QUERY` and `observation_from_row`. Bring the interface block and the comments in line with the notes of 2026-11-15 in spec 0003. Needs steps 3 and 5. Files: `crates/pool-control/`. Evidence: `cargo test -p pool-control --locked`. Satisfies `0003 AC-17`, `0003 AC-18`, the library half of **AC-10** and **AC-12**.
7. **Secrets, TLS and the backend connection.** In `pool-core`: the secret types and the reading of protected files, the TLS server and client setup, SCRAM as server and as client, the backend connection of [Backend connection](#backend-connection) and the one row query. Needs steps 4 and 5. Files: `crates/pool-core/`. Evidence: `cargo test -p pool-core --locked`. Satisfies the secret half of **AC-9**, and the library half of **AC-15** and **AC-16**.
8. **Bootstrap, the authority command and the first run against PostgreSQL.** In `apps/pool`: the state directory checks and the lock, `pool bootstrap`, `pool show-authority`, and the help text. Add `integration` to `scripts/check.sh` with `scripts/integration/`, and write the first tests of `apps/pool/tests/integration.rs`: bootstrap, a second bootstrap, `show-authority`, a node that does not answer, the monitor login. Write `apps/pool/tests/commands.rs` for the two commands. Record the image in the technology evaluation. Needs steps 6 and 7. Files: `apps/pool/`, `scripts/check.sh`, `scripts/integration/`, `docs/research/technology-evaluation.md`. Evidence: `cargo test -p pool --locked`, `scripts/check.sh integration`. Satisfies **AC-9**, **AC-11**, **AC-12**, **AC-26**, and completes **AC-10**.
9. **The session.** In `pool-core`: the listener loop, steps 1 to 12 of [Session lifecycle](#session-lifecycle), the limits, the relay, backend loss, the conflict ending, shutdown, the route view, the lease and the pool key, and the log events of the data plane. Write `crates/pool-core/tests/session.rs`. Needs step 7. Files: `crates/pool-core/`. Evidence: `cargo test -p pool-core --locked`. Satisfies the hermetic half of **AC-14** to **AC-17**, **AC-19**, **AC-20**, **AC-22**, **AC-23** and **AC-25**.
10. **The daemon.** In `apps/pool`: `pool run` with its ten steps, the log writer, the probe tasks, the control loop, the mapping into the route view, and signals. Extend `integration.rs` with the walking fixture: the same SQL file direct and through POOL, the mapped login, teardown, the states that are not ready (in `commands.rs`), redaction over the whole run. Needs steps 8 and 9. Files: `apps/pool/`. Evidence: `scripts/check.sh integration`, `cargo build --workspace --locked` without a dependency warning. Satisfies **AC-13**, **AC-18**, **AC-19**, **AC-24**, **AC-25**, and completes **AC-1** and **AC-14**.
11. **Limits, loss and denial on the fixture.** Extend `integration.rs` and its helpers with the relay and the cases of [The integration check](#the-integration-check): every limit, the memory budget, the lost COMMIT, plain text, a wrong CA, a wrong password, an unknown user, an unmapped database, the wrong identity. Needs step 10. Files: `apps/pool/tests/`. Evidence: `scripts/check.sh integration`. Completes **AC-15**, **AC-16**, **AC-17**, **AC-20**, **AC-21** and **AC-22**.
12. **Fuzzing and the CI rule.** Create `crates/pool-protocol/fuzz/` with its three targets, lockfile and regression inputs. Extend `fuzz` and add `fuzz-tools` in `scripts/check.sh`, the new baseline file, rule W6, and `fuzz-tools` in rule C3, each with a test. Needs steps 4 and 8. Files: `crates/pool-protocol/fuzz/`, `crates/pool-config/fuzz/regressions/`, `scripts/check.sh`, `tools/repo-check/`. Evidence: `scripts/check.sh fuzz`, `scripts/check.sh all`, `scripts/prove-checks.sh`. Satisfies **AC-27**. Manual step M2 follows it.
13. **Document corrections.** Apply items 1 and 2 of [Document corrections](#document-corrections). Evidence: `scripts/check.sh docs`.
14. **Evidence.** On the final tree run `scripts/check.sh all`, `scripts/check.sh integration`, `scripts/check.sh fuzz`, `scripts/prove-checks.sh` (with `--with-linux-build` when `docker info` succeeds) and the commands of **AC-1**. Record commands, results, the revision, the image digest and the locked versions in `docs/specs/0004-one-real-primary-path/verify.md`, one section per milestone, and add that file to `docs/index.md`. A step that needs Docker and cannot run is recorded as `BLOCKED` with `owner: user`. The CI runs of `integration` and `fuzz` are recorded as `NOT_RUN` until they exist. Evidence for **AC-1** to **AC-28**.

**Parallel sessions.** Two developers at most, and at most two full Rust builds at a time. The pairs below share no file.

| Developer A | Developer B | Why they do not meet |
|---|---|---|
| Steps 1, 2, 3, in that order | Step 5 | A works in `scripts/`, `tools/`, the manifests and the two new crate folders. B works in `crates/pool-config/` and three files of `apps/pool/` that are not its manifest |
| Step 4 | Step 6 | `crates/pool-protocol/` and `crates/pool-control/`. Both need step 3, and B needs step 5 |
| Step 7, then step 9 | Step 8, after step 7 is merged | `crates/pool-core/`, and `apps/pool/` with `scripts/` |
| Step 12 | Step 10, then step 11 | Step 12 touches `scripts/check.sh`, which step 8 changed before. Steps 10 and 11 stay in `apps/pool/` |

Steps 13 and 14 come last, in one session.

## Consequences

**Positive**:

- No SQL from an application can reach a node unless the journal, the configuration and a fresh probe agree, and that holds by the order of the steps, not by a check someone must remember.
- The client sees the truth about its backend: the real parameters, the real errors, and an honest error when the outcome is unknown.
- Every refusal has a number a client library understands, and none of them names a thing the client did not send.
- The byte level code has no dependency and no input or output, so it is fuzzed and tested at every byte boundary before a real client meets it.
- Memory per session has a bound that does not depend on what the client or the database sends.
- One crypto library does TLS and SCRAM, and no PostgreSQL client library sits in the product.
- Three findings of the review of scope 003 that could open writes (m1, m2, m3) are closed in code, with criteria.
- The source guard still means something after sockets arrive: each crate has a written list of what it may name.

**Negative / tradeoffs**:

- The product now holds native code: AWS-LC and SQLite. A build needs a C compiler, and a flaw in either library is a flaw in POOL. Two more licenses enter the tree.
- The lockfile grows from 16 to 70 third party entries. Most of the new ones are small or are for other platforms, and each one is still a package to watch.
- The path is narrow on purpose. `psql` works. A driver that uses the extended protocol, COPY, protocol 3.2 features, channel binding or `options` is refused, clearly, until scope 006.
- A client cannot cancel a running query through POOL until scope 006. Pressing Ctrl C in `psql` does nothing to the server.
- Every login opens a new backend and every logout closes one. That is slow, and it is the thing scope 005 exists to fix.
- A client that goes away during a long statement is noticed late, and a client that reads slowly holds its backend with no deadline.
- A conflict latch cannot be cleared in this slice. A second primary that was seen once, even for a moment, keeps its cluster closed until scope 009, or until you stop the daemon, remove the journal files and the witness, and bootstrap again, which is the second way out of walk 2 of spec 0003.
- A daemon whose journal is not ready does not start at all. You see that in the service manager and not through a client error.
- `pool show-authority` cannot run beside the daemon.
- The timeline of a node in recovery is the best that plain SQL gives, not the exact replay timeline. It errs toward closing.
- Backend passwords must be printable ASCII. A password with other characters is refused at start.
- A session that runs is ended when its cluster shows a positive conflict. That is the safe side, and it cuts work that might have been fine.
- Configuration that was valid and holds a control or direction character in a name is now refused, and a report with more than 20000 findings is cut. Both change contracts of earlier specs.
- The log writer is first party code. It is small, and it is one more thing that can be wrong where a well used crate exists.
- Certificates are read once. A renewed certificate needs a restart until scope 009.
- The checks grow from seven to nine, and two of them need more minutes than the others.

**Neutral**:

- The key names, the finding codes, the command names, the event names and their fields, the SQLSTATE table and the format of the source guard file are contracts from now on.
- `0001 AC-1`, `0001 AC-12`, `0001 AC-13` and `0003 AC-13` describe earlier shapes of the workspace. `0004 AC-1` is the current statement.
- Rule W3 of spec 0002 keeps its number and its message and gets a new file format.
- The words principal, verifier, monitor login and route view enter the glossary.

## Follow-up

1. Spec 0001 needs dated notes that point here: under "Adopted dependencies" (six more, and native code), "Workspace layout and manifests" (five members, and `0004 AC-1` as the current statement), "The check-config command" (three more commands and the help text), "Exit status and output" (the result `refused`, the omitted line and field), "Findings" (the new codes and the limit of 20000), "Configuration rules" item 6 (four secret references, and where they are read), "Log events and redaction" (the first events and the first party writer), and its Follow-up 2 and 10 (answered for the frontend authentication, the permission check and `tracing`). No session may edit spec 0001 in this run, so this is an item for the lead after acceptance. Applied on 2026-11-15, after this spec was accepted: spec 0001 carries these notes, each dated, each leaving its own text and IDs as they were. Its Follow-up 2 and Follow-up 10 are marked as answered.
2. Spec 0002 needs dated notes that point here: under "Workspace guards" (the new format of rule W3, the baseline lines, four lockfiles for W1 and W6), "CI definition guards" (`fuzz-tools` in rule C3), "Required checks" and "The check entry point" (`integration`, `fuzz` and `fuzz-tools`, nine required checks after manual step M2), "Dependency policy" (the five exceptions), "Proving the checks" (case 17 and the new cases), "Fuzz harness" (the second package and the regression directories), and its Follow-up 6 (answered). Also an item for the lead. Applied on 2026-11-15, after this spec was accepted: spec 0002 carries these notes, each dated, each leaving its own text and IDs as they were. The notes about the two CI jobs and the nine required checks describe the state after manual steps M2 and M3, not the state today. Its Follow-up 6 is marked as answered.
3. Scope 005 adds reuse and with it: idle and lifetime limits, a deadline for a client that reads slowly, the trigger that advances a credential generation, and the startup profiles that cleanup can restore.
4. Scope 006 adds protocol 3.2, the extended protocol, COPY, virtual cancellation credentials, `SCRAM-SHA-256-PLUS`, client certificates, SASLprep for passwords outside ASCII, noticing a client that left during a statement, and the first metrics.
5. Scope 007 qualifies the probe on a real replica, including what the timeline column reports during and after a promotion, and whether the monitor login gets the `pg_read_all_stats` role.
6. Scope 008 owns the bound on the cost of a control round for very large files, a key for the log level, the probe history and drain.
7. Scope 009 owns reload, clearing a conflict latch, and reading the authority state while the daemon runs.
8. Running as root is not detected. If it ever matters, the check needs the effective user of the process, which needs a small dependency or a call this slice avoids.
9. Follow-up 13 of spec 0003 (`PRD-006 AC-2`) is still yours to decide. This spec does not touch it.
10. Manual steps M1 to M4 under [CI definition and manual steps](#ci-definition-and-manual-steps).

## Rationale

Reasoning, alternatives, the decisions taken, the locked packages and the cross check: [rationale.md](rationale.md).
