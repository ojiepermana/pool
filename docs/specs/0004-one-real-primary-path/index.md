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

- **AC-1** (workspace shape, `0002 AC-6`): `cargo metadata --no-deps --format-version 1` lists exactly five workspace members: `pool`, `pool-config`, `pool-control`, `pool-core` and `pool-protocol`. The dependencies between them are exactly the six pairs under [Workspace](#workspace). `pool-protocol` has no dependency at all. `Cargo.lock` holds the five members and the 70 third party entries listed in the [rationale](rationale.md#locked-packages). `apps/pool-cli` does not exist. No package has `dev-dependencies` or a build script. Every direct dependency of every member is named in the code of that member: the command under [Adopted dependencies](#adopted-dependencies) prints nothing, and `cargo build --workspace --locked` prints no warning about an unused dependency.
- **AC-2** (source guard): Rule W3 reads the new file format under [Source guard](#source-guard). With the baseline of this spec, a line that names `tokio`, `TcpStream`, `TcpListener` or `std::net` is accepted in `crates/pool-core` and `apps/pool` and rejected in `crates/pool-config`, `crates/pool-control` and `crates/pool-protocol`. `std::thread`, `UdpSocket`, `UnixListener` and `UnixStream` are rejected everywhere. `Instant::now`, `SystemTime`, `std::fs` and `std::env` are rejected anywhere under `crates/pool-control/src`. `rusqlite` is rejected under `crates/pool-control/src` except in `journal.rs`. The names `StoredAuthority`, `StoreProbe`, `classify_startup`, `plan_bootstrap` and `writer_admission` are rejected in every `.rs` file outside `crates/pool-control`. Each of these six groups has a case in `scripts/prove-checks.sh` that fails without the rule.
- **AC-3** (dependency policy and notices, `SEC-13`, `R-17`): `scripts/check.sh deny` passes on the product lockfile and on both fuzz lockfiles with the `deny.toml` under [Dependency policy](#dependency-policy), which adds five license exceptions by crate and no license to the general list. A file `NOTICE` exists at the root and names SQLite and AWS-LC with their versions and license terms. `scripts/check.sh linux-build` passes with the native code compiled by the C compiler of the pinned image and no other tool.
- **AC-4** (proof script and lockfile check): Case 17 of `scripts/prove-checks.sh` rejects a wrong status whatever status scope 004 has. A lockfile entry ends at the next line that starts with `[`, and one new case adds a local package to `Cargo.lock` and expects rule W4. `scripts/prove-checks.sh` passes on a committed tree.

Configuration

- **AC-5** (new keys): Every fixture of spec 0001 and spec 0003 keeps its result and its output, byte for byte. One of them changes its text to stay so: `invalid-unknown-key.toml` of spec 0001 names its unknown table `no_table` in place of `listener`, which is a known table from this slice on. Each `valid-serving*` fixture exits 0 with exactly the valid report of spec 0001. Each `invalid-serving-*` fixture exits 1 with the finding listed under [Fixtures](#fixtures). A file without `[listener]`, `[limits]`, `[probe]` and `[[principal]]` is still valid, and each key that is left out has the default in the table under [Keys](#keys).
- **AC-6** (names): A name or a path that holds a control character from U+0000 to U+001F or from U+007F to U+009F, or one of the direction and separator characters listed under [Names and paths](#names-and-paths), gives `config.invalid_value` at its key. The text `CANARY-9f2c7b1e` in such a value appears in no output.
- **AC-7** (number of findings): `validate` keeps the first 20000 findings in the order that spec 0003 fixes and counts the rest. The report then ends with the line `findings omitted: <n>` in the human format and carries `"findings_omitted":<n>` before `findings` in the JSON format. A report with 20000 findings or fewer is unchanged, byte for byte, so made input 3 of `0003 AC-15` still gives 17268 findings. For made input 5 under [Number of findings](#number-of-findings), a file at the size limit that holds only empty identity entries, the built binary writes exactly 20000 finding lines and less than 4 MiB in each format, and the omitted count is six times the number of entries minus 20000.
- **AC-8** (configuration that can serve): `ValidatedConfig::serving()` returns a view only when the file has everything `pool run` needs, and `ValidatedConfig::probing()` only when it has everything `pool bootstrap` needs. Each missing piece gives one finding `config.incomplete` at the key listed under [What a command needs](#what-a-command-needs). `pool check-config` does not report these.

State, journal, bootstrap and probe

- **AC-9** (protected paths, `SEC-4`, `T-11`): `pool run`, `pool bootstrap` and `pool show-authority` exit 3 with `state.permissions` when the state directory is not a directory with mode `0700`, or when a file in it that belongs to the journal is not a regular file with mode `0600` owned by the owner of the directory. They exit 3 with `secret.permissions` when a secret file has any group or other permission bit, is not a regular file or has another owner. A second command on the same state directory, while the first holds it, exits 3 with `state.locked` and changes nothing. (Note of 2026-11-16, finding r4 of the second review. Decision 88 added one refusal that this criterion does not name: `pool run` and `pool bootstrap` also exit 3 with `secret.permissions` for a certificate chain or a CA file that the group or others can write, or whose owner is neither the service account nor root. The rule is under [State directory and protected files](#state-directory-and-protected-files).)
- **AC-10** (journal on disk, `0003 AC-7`, `0003 AC-8`): After a successful `pool bootstrap`, `journal.sqlite3` has `application_id` 1347374924, `user_version` 1, WAL mode and exactly the four tables of spec 0003, and it holds revision 1, start sequence 0, hold `none` and one approved topology per cluster. The journal reader returns `Absent` for no file and for an empty file (a file of 0 bytes, or a database with no mark and no table, as [The journal on disk](#the-journal-on-disk) defines it), and `Unreadable` for each case that spec 0003 lists (wrong application ID, newer version, failed `quick_check`, missing `store` row, writer without member row, value out of range). Each `pool run` that finds the state `Ready` advances the start sequence by one before it listens. In every other state the content of `journal.sqlite3` does not change. SQLite may create its two side files when it reads.
- **AC-11** (bootstrap and reading the authority ID, `0001 SC-8`): On the fixture, with no journal and no witness, `pool bootstrap` exits 0 and prints a new authority ID. A second `pool bootstrap` exits 1 with `bootstrap.store_present`. `pool show-authority` prints the same ID and the state `unconfirmed`, and after the ID is written into the configuration it prints `ready`. With a node that cannot be reached, bootstrap exits 1 with `bootstrap.evidence_missing` and creates no journal file. No command prints a host, a login, a database name or a path.
- **AC-12** (probe, `SEC-5`, `T-03`): The probe logs in as the monitor login of the cluster, which is not a superuser, has no replication attribute and is a member of no role, over TLS with the CA and server name of the node. It runs the one query under [The probe](#the-probe) and reports the system identifier, the role and the timeline. A probe that gets no answer within `probe.timeout_ms`, a wrong certificate, a failed login or a row of another shape gives a `Failed` observation. The control loop keeps one sample per node, and `build_snapshot` never treats a sample as later than now.
- **AC-13** (no listener without ready authority): `pool run` with a journal state of `Uninitialized`, `Unconfirmed`, `Missing`, `Unreadable`, `Mismatch` or `Held` exits 1 with the one finding of that state (the reason names of spec 0003) and never binds the listener.
- **AC-14** (writer check before any SQL, `0001 SC-8`, `PRD-005 AC-1`): A session reaches PostgreSQL only through a route that the current topology snapshot marks open for its cluster, and POOL checks the snapshot again after the backend is ready and before it sends `AuthenticationOk`. A backend that does not report `in_hot_standby` as `off` is closed. When the snapshot is closed, or older than its `fresh_until`, the client gets SQLSTATE `57P03` and no connection is opened for its login. On the fixture: with the node pointed at a second PostgreSQL instance that has another system identifier, the event `admission.changed` names `evidence.writer_identity`, a login is refused with `57P03`, and the log of that instance shows no connection of the application login. Hermetic: a session that is already running is ended when its cluster closes with a reason from the list under [When a running session is ended](#when-a-running-session-is-ended), and is not ended for `evidence.writer_missing` alone. (Note of 2026-11-16, finding r4 of the second review. The last sentence stands for a cluster that is closed for silence and not marked lost. Silence that lasts `probe.node_loss_timeout_ms` does end the session, by `0004 AC-30`.)

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
- **AC-29** (the server a session reaches is the server that was verified, `0001 SC-8`, `R-19`; added on 2026-11-16, from finding M2 of the independent review): Before POOL sends `AuthenticationOk`, it asks the node the question of the probe on the backend connection of that session. The answer must be one row that names the system identifier and the timeline that the route of step 5 carries, and that says the node is not in recovery. With any other answer, an error or no row, the backend is closed, the client gets `08006`, and no byte of application SQL was sent on that connection. Hermetic: a scripted node that answers with another system identifier, with another timeline, with `true` for recovery, with an error, with two rows or with a null is refused each time and sees no `Query` of the client. On the fixture: the probe holds a connection to the first container while new connections to the address of the node reach the second container, which has another system identifier. A login then gets `08006`, the event `backend.failed` has the class `identity`, and the log of the second container shows no statement of the application login other than the identity question.
- **AC-30** (a peer that vanished without closing, `R-10`, `0001 SC-6`; added on 2026-11-16, from finding M1 of the independent review): Every socket that POOL accepts and every socket that it opens to a node has TCP keepalive turned on. When the probe of the approved writer of a cluster has failed with the class `connect` or `timeout`, and with no other outcome in between, for `probe.node_loss_timeout_ms`, every running session on that node is ended as [Backend loss](#backend-loss) says: the backend is closed, and at a message boundary the client gets `08007` when a query is in flight and `08006` when none is. Hermetic: a scripted node takes a `Query` and never answers, and the client gets `08007` once the route view marks the node as lost; an idle session gets `08006`; a cluster that is closed for silence and not marked lost ends no session. In the control loop: failures of the class `connect` or `timeout` for the whole time mark the node lost and write the event `node.lost` once, and a good sample, or a failure of another class, starts the time again.

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
| AC-29, AC-30 | The [independent review](../../reviews/2026-11-16-scope-004-one-real-primary-path.md) of this scope, findings M2 and M1: the invariant "writes on the verified primary only" on the connection of the session itself, and `R-10` for a peer that is gone without closing (added on 2026-11-16) |

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

`internal-dependencies.txt` (rule W5) holds exactly these six lines: `pool -> pool-config`, `pool -> pool-control`, `pool -> pool-core`, `pool-control -> pool-config`, `pool-core -> pool-config`, `pool-core -> pool-protocol`. `pool-core` and `pool-control` are never a pair.

**Random bytes** have one source in the product: `pool_core::random::fill_random(out: &mut [u8]) -> Result<(), RandomError>`, in `crates/pool-core/src/random.rs`, which fills the slice from the generator of `aws-lc-rs`. The SCRAM code of `pool-core` and the commands of `apps/pool` both call it, so `apps/pool` needs no dependency on `aws-lc-rs`. A failure is never replaced by another value: no clock, no counter, no fixed bytes. In a command before it serves, a failure is the finding `random.failed` with exit 3. While the daemon serves, a failure ends the one login that needed the bytes: the connection is closed without a message and counted under `session.refused` with the reason `random_failed`. `RandomError` carries no detail and prints its name only. (Added on 2026-11-15, from a finding of Build plan step 8: the first version gave the binary no way to get random bytes.) `forbidden-paths.txt` (rule W2) loses `crates/pool-protocol` and `crates/pool-core` and keeps `apps/pool-cli`.

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

A build without that warning does not show that the exception ended. The verification of milestone 1 found that Cargo 1.100.0 warns about some unused dependencies and stays silent about others: after step 3 it named seven entries and said nothing about `tokio`, `rustls` and `aws-lc-rs` in `pool-core` or about `tokio` and `tracing` in `apps/pool`, which no code used yet. So `0004 AC-1` is checked with this command, run from the repository root. It reads the keys under `[dependencies]` of each member and looks for each name, with `-` written as `_` and followed by `::`, in the `.rs` files under the `src` folder of that member. It prints one line for each dependency that no code names, and nothing when every dependency is used.

```sh
for m in apps/pool crates/pool-config crates/pool-control crates/pool-core crates/pool-protocol; do
  awk '/^\[dependencies\]/{d=1;next} /^\[/{d=0} d && /^[a-z]/{sub(/[ .=].*/,""); print}' "$m/Cargo.toml" |
  while read -r dep; do
    name=$(printf %s "$dep" | tr - _)
    grep -rqE "(^|[^A-Za-z0-9_])${name}::" "$m/src" || echo "unused: $m $dep"
  done
done
```

On the tree after step 3 it printed twelve lines, the seven that Cargo named and the five it did not. It is run by hand at steps 10 and 14 and its output goes into the verification record. It is not a required check: it proves that a temporary state ended, and after that the next unused dependency is a matter for review, as before. A name that appears only in a comment would satisfy it, which a reviewer of step 10 looks for.

### Dependency policy

`deny.toml` keeps its general list as it is: `Apache-2.0`, `MIT`, `Unicode-3.0`. It gains five exceptions by crate, placed after the existing one for `libfuzzer-sys`:

```toml
[[licenses.exceptions]]
crate = "aws-lc-rs@=1.19.0"
allow = ["ISC"]

[[licenses.exceptions]]
crate = "aws-lc-sys@=0.47.0"
allow = ["ISC", "BSD-3-Clause"]

[[licenses.exceptions]]
crate = "rustls-webpki@=0.103.17"
allow = ["ISC"]

[[licenses.exceptions]]
crate = "untrusted@=0.9.0"
allow = ["ISC"]

[[licenses.exceptions]]
crate = "subtle@=2.6.1"
allow = ["BSD-3-Clause"]
```

**Each exception names one exact version** (changed on 2026-11-15, after the verification of milestone 1; the first version of this block named the packages without a version). The permission of manual step M1 was given for these packages at these versions. `name@=version` is how `cargo-deny` 0.20.2 writes an exception for one version, and it was run on 2026-11-15 on the product lockfile and on `crates/pool-config/fuzz/Cargo.lock`: both give `advisories ok, bans ok, licenses ok, sources ok`. With `aws-lc-rs@=1.19.1` in the file and 1.19.0 in the lockfile, the check fails with `license is not explicitly allowed` for `aws-lc-rs v1.19.0`. So this is what happens when one of the five gets a new version:

- The exception no longer fits, and `scripts/check.sh deny` fails on the pull request that changes `Cargo.lock`. That holds for every new version, also one that changes only the last number, and also one whose declared license is different. Nothing new can pass without someone looking.
- The pull request that updates the package changes the version in its exception line too. The Release maintainer, a person, compares the declared license of the new version with the one in the table of manual step M1. When the text is the same, changing the number is all there is to do. When it differs, it is a new license decision, recorded in the technology evaluation, as the [rules of spec 0002](../0002-standards-and-tooling/index.md#dependency-policy) ask.
- The price: a security update of one of these five waits for that look. Every change reaches `main` through a pull request that a person merges, so the look costs a line in a review that happens anyway.

Exceptions by crate are used, and not two more lines in the general list, for two reasons. The general list is also applied to the fuzz lockfiles, where no package needs `ISC`, and a license in the list that nothing needs fails the check. And a later package under `ISC` or `BSD-3-Clause` then needs its own decision. With this file, `cargo deny check` printed `advisories ok, bans ok, licenses ok, sources ok` for the scratch lockfile and for `crates/pool-config/fuzz/Cargo.lock` on 2026-11-15. An exception that a lockfile does not meet is a warning, as the `NCSA` one already is.

**The list of allowed licenses is the list of the product** (added on 2026-11-15, from a finding of Build plan step 12). `deny.toml` fails a license in the general list that no package needs, so that the list cannot grow quietly. That rule is meant for the product lockfile, and there it stays as it is. A fuzz lockfile holds a part of the product's packages and the fuzz runtime, so whether it happens to need every license of the list is an accident of what the fuzzed crate depends on. The lockfile of `crates/pool-protocol/fuzz/` shows it: `pool-protocol` has no dependency, so the lockfile holds no package under `Unicode-3.0`, and the check failed with `license-not-encountered` though no package and no license was new. So `scripts/check.sh deny` runs three commands, and the two for the fuzz lockfiles turn that one lint off:

```sh
cargo deny --locked --config deny.toml check
cargo deny --locked --manifest-path crates/pool-config/fuzz/Cargo.toml --config deny.toml check -A license-not-encountered
cargo deny --locked --manifest-path crates/pool-protocol/fuzz/Cargo.toml --config deny.toml check -A license-not-encountered
```

`deny.toml` does not change. On a fuzz lockfile a package under a license that is not allowed still fails, and so does an advisory, a duplicate and a package from another source. Only the complaint about an allowed license that the lockfile does not use is off. This was run on 2026-11-15 with `cargo-deny` 0.20.2 on `crates/pool-config/fuzz/Cargo.lock`: with a license added to the list that nothing uses, the plain command fails and the command with the flag passes, and with `MIT` taken out of the list the command with the flag still fails with `license is not explicitly allowed`. The rule is the same for both fuzz lockfiles, on purpose. The first one passes without the flag today only because `pool-config` pulls in a package under `Unicode-3.0`.

Adding a license exception is a decision of the Release maintainer, who must be a person ([spec 0002](../0002-standards-and-tooling/index.md#dependency-policy)). That is manual step M1, and Build plan step 3 waits for it.

### Native code

This slice brings the first native code into the product. The engineering standards ask for a boundary review of each. This is that review, and the independent review of the slice checks it.

| Library | Enters through | What it is | Boundary |
|---|---|---|---|
| AWS-LC 5.12.0, as bundled in `aws-lc-sys` 0.47.0 | `aws-lc-rs`, used by `rustls` and by `pool-core` | A C and assembly crypto library | First party code calls only the safe interface of `aws-lc-rs` (`digest`, `hmac`, `pbkdf2`, `rand`, `constant_time`) and `rustls`. Every input length is bounded by this spec before the call: SCRAM messages at 1024 bytes, the PBKDF2 count at 600000 |
| SQLite 3.54.1 (`libsqlite3-sys`) | `rusqlite`, used by `pool-control` | A C database library | Only `crates/pool-control/src/journal.rs` names `rusqlite` (`0004 AC-2`). Statements are constants with bound parameters. No SQL text is built from a value. The file is opened by path only, with no extension loading and no shared cache |

Both compile with the C compiler of the pinned image `rust:1.100.0-slim-bookworm` and need no other tool. This was run on 2026-11-15 for `aarch64-unknown-linux-gnu` in that image: the scratch program built in 12 seconds and printed `sqlite 3.54.1`. No first party crate gains `unsafe` code, and rule W4 still holds for all five members. A safe wrapper is not proof that the library inside is safe: fuzzing covers the first party parsers in front of it (`0004 AC-27`), and advisories for both libraries are checked on every pull request and every day.

**The version of AWS-LC** (corrected on 2026-11-16, finding g5 of the independent review). The first version of the table above wrote "AWS-LC 0.47.0". That is the version of the crate `aws-lc-sys`. The library inside it is 5.12.0 (`AWSLC_VERSION_NUMBER_STRING` in the locked crate), and that is the number an advisory names. `NOTICE` gets the same correction: "AWS-LC 5.12.0, as bundled in aws-lc-sys 0.47.0".

**The build never takes a library from the host** (added on 2026-11-16, decision 87, finding c4 of the independent review). Everything above says that AWS-LC and SQLite are built from the source inside the locked crates. Two build scripts decide that from the environment. `aws-lc-sys` 0.47.0 looks for an installed AWS-LC and links it when it finds one that fits, unless `AWS_LC_SYS_USE_SYSTEM` is `0`. `libsqlite3-sys` 0.40.0 links the SQLite of the host when `LIBSQLITE3_SYS_USE_PKG_CONFIG` is set to anything but `0`. On a host that has such a library, the binary would hold another library than the lockfile, the license exceptions, the advisory check and `NOTICE` describe, and no file in the repository would differ. In the pinned image the source build is what happens, because the image holds a C compiler and neither library. From now on that does not rest on the image. The repository gets the file `.cargo/config.toml`, which every `cargo` call under the repository root reads, also those of `tools/repo-check`, of both fuzz packages and of the Linux build:

```toml
# The native libraries are built from the source inside the locked crates, never taken from the host (spec 0004, Native code).
[env]
AWS_LC_SYS_USE_SYSTEM = { value = "0", force = true }
AWS_LC_SYS_USE_SYSTEM_aarch64_unknown_linux_gnu = { value = "0", force = true }
AWS_LC_SYS_USE_SYSTEM_x86_64_unknown_linux_gnu = { value = "0", force = true }
AWS_LC_SYS_USE_SYSTEM_aarch64_apple_darwin = { value = "0", force = true }
AWS_LC_SYS_USE_SYSTEM_x86_64_apple_darwin = { value = "0", force = true }
LIBSQLITE3_SYS_USE_PKG_CONFIG = { value = "0", force = true }
```

`force` makes the file win over a variable that the shell of a developer or of a CI runner already holds. A unit test beside the journal code expects `rusqlite::version()` to be `3.54.1`, so a build that took another SQLite fails `scripts/check.sh test`. AWS-LC has no such test: the safe interface that this slice uses gives no version. For it the two lines are the control, and review keeps the file. A guard rule for the file belongs to the tooling work of Follow-up 14.

**The name with the target** (added on 2026-11-16, decision 99, from the verification of Build plan step 15). The first version of the block had two lines, and the checker found a way past the first one. The build script of `aws-lc-sys` 0.47.0 reads its switch under two names: first `AWS_LC_SYS_USE_SYSTEM_` followed by the build target, with every `-` written as `_`, and only then the plain name. `force` covers a name that the file lists. So with the two lines, a shell that held `AWS_LC_SYS_USE_SYSTEM_aarch64_apple_darwin=1` still made the script look for an installed AWS-LC. The block now lists the name of each target that this project builds on: the two Linux targets of the pinned image, and the two targets of a macOS workstation. This was run on 2026-11-16 in a scratch project with the locked crate, on `aarch64-apple-darwin`, with that variable set to `1` in the shell. With the plain line alone the build script looked for an installed library and failed for want of one. With the line for the target the script saw the value `0` under both names and built from source.

- **What that closes.** A host that has AWS-LC installed, which is the accident, was closed by the plain line already: with no variable set, the plain name decides. The new lines close the deliberate variable for the four targets. With the switch at `0`, a variable that points the script at an install (`AWS_LC_SYS_SYSTEM_DIR`, with or without a target) makes the build fail and not link: the script refuses the two together.
- **What stays open.** A build for a target that the block does not list, with the variable for that target set on purpose. Add a line when a target is added. And nothing stops a person from changing the file, which shows in review. The build that counts, `scripts/check.sh linux-build`, runs in a container that gets three variables from the script and none from the host, so no variable of a shell reaches it. (Added on 2026-11-16, a nit of the second review.) `cargo` reads the file from the directory it runs in and from the directories above it. A `cargo` call from outside the repository that names the manifest with `--manifest-path` does not read it. Every script of this repository changes into the repository root first.
- **SQLite needs one line.** `libsqlite3-sys` 0.40.0 reads the one name and then builds the bundled source. The checker ran it both ways: with the file the version test passes also when the variable is `1` in the shell, and without the file it fails with the SQLite of the host.
- **No test holds the block.** The file is kept by review until the guard rule of Follow-up 14 exists. That rule has to hold all six lines.

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
rusqlite
allow crates/pool-control/src/journal.rs rusqlite
allow crates/pool-control/tests rusqlite
aws_lc_rs
allow crates/pool-core aws_lc_rs
```

What this keeps true:

- A new crate is covered by all fifteen bare lines the day it appears. Opening a socket or starting a runtime anywhere new needs a line in this file, in a pull request that names the spec that allows it.
- **Which crate may name a native library** (changed on 2026-11-16, decision 93, finding g2 of the independent review). The first version of the block refused `rusqlite` only under `crates/pool-control/src`, so `apps/pool` could have taken the crate and opened the journal on a worker of the runtime with every check green. `rusqlite` is now a bare line, allowed in `journal.rs` and in the tests of `pool-control`, which damage real journal files. `aws_lc_rs` is a bare line too, allowed in `pool-core`. So [One writer](#one-writer) and "one crypto caller" are held by the guard in every crate, and `scripts/prove-checks.sh` gets one case that names `rusqlite` in `apps/pool`. The line `deny crates/pool-control/src rusqlite` is gone, because the bare line covers it.
- **What this guard is and is not** (added on 2026-11-16, finding g1 of the independent review). Rule W3 matches text. It is a tripwire, not a proof. It holds well for a name that code must spell to use the thing: the five guarded names of `pool-control`, `rusqlite`, `aws_lc_rs` and `tokio`. It holds less well for a path of the standard library: `use std::{fs, env};` names neither `std::fs` nor `std::env`, a method of `Path` reads a file without either text, and a `build.rs` is outside the walk. The tree is clean today, and the reviewer read it. A check that resolves names (the `disallowed-methods` and `disallowed-types` lists of `clippy`, one `clippy.toml` per crate, run by the required `clippy` check) is the tooling work of Follow-up 14.
- `std::thread`, `UdpSocket`, `UnixListener` and `UnixStream` have no `allow` line. No first party code starts a thread by itself. Blocking work goes through the blocking pool of the runtime.
- `pool-config`, `pool-control` and `pool-protocol` have no `allow` line for a socket or the runtime. The rule that core and control never meet is rule W5, and this rule keeps control free of the things that would tempt it to.
- The second half of `0003 AC-11` (no clock, no file call and no environment read in `pool-control`) is now a guard and no longer a `grep` run by hand. That is point t6 of Follow-up 4 of spec 0003. The journal module reads its file through `rusqlite`, which is the one text allowed in the one file.
- A journal value cannot be built, changed or classified outside `pool-control`. Other crates never hold a `StoreProbe`: the journal reader returns a `JournalRead`, which has no public field and can only be asked to classify, to plan a bootstrap or to show its authority ID ([The journal on disk](#the-journal-on-disk)). The four names that would let code go around it are refused outside the crate. That is point m2, and `0003 AC-19`.
- The guard matches text, so it also refuses a method, a comment or a string that holds one of these names in another crate. That is wanted: it is what keeps an alias or a wrapper with the same name from slipping in. It also means that the three things other crates do call have other names: `JournalRead::classify`, `JournalRead::bootstrap_plan` and `build_snapshot`.
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
| `probe.node_loss_timeout_ms` | integer | no | 30000 | 1000 to 3600000, and not less than twice `period_ms` plus `timeout_ms` (the cross rule was added on 2026-11-16, decision 109) | no | Restart only |
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
- **`probe.node_loss_timeout_ms`** (added on 2026-11-16, decision 84) is how long the approved writer of a cluster may give the probe no answer at all before POOL treats the node as lost and ends its running sessions ([When a running session is ended](#when-a-running-session-is-ended)). It has no cross rule with the other probe keys. A failed sample closes new logins at once, so sessions never end before logins are closed. A value outside the range is `config.invalid_value` or `config.schema`, as for the other limits. It is checked after `probe.observation_expiry_ms`, and the view that `config.probe()` returns gives it as `node_loss_timeout()`. (Changed on 2026-11-16, decision 109, finding r2 of the second review.) The sentence above that it has no cross rule no longer holds. The control loop marks a node as soon as its first failure is as old as the loss time, so with a loss time below the period plus the timeout one probe that stumbles was enough to end every session, before the next probe could answer. The rule now: `probe.node_loss_timeout_ms` is not less than twice `probe.period_ms` plus `probe.timeout_ms`. Then at least one more probe of the node has asked and has had its whole timeout to answer before the mark can come, and a good answer starts the time again. A lower value is `config.invalid_value` at `probe.node_loss_timeout_ms`, also when the key is absent and its default is too low for the period and the timeout of the file, as for the expiry. The rule is checked after the cross rule of the expiry and reads only values that passed their range. The fixture `invalid-serving-node-loss.toml` holds a period of 20000, a timeout of 5000, an expiry of 30000 and a loss time of 44999, and gives exactly that one finding. A test in `validate.rs` shows that 45000 passes. With the defaults the least value is 5000.
- **Leave room in the expiry** (added on 2026-11-16, nit n9 of the independent review). An expiry that equals the period plus the timeout is valid and leaves no margin: one probe round that takes its whole timeout closes the route until the next sample. Keep the expiry at twice the period plus the timeout or more, as the defaults do.
- **A large startup packet and PostgreSQL** (added on 2026-11-16, nit n12). PostgreSQL refuses a startup packet above 10000 bytes. POOL sends a packet of its own to the node, which holds the two forwarded parameters of the client. With `limits.startup_packet_max_bytes` above 10000, a client can send an `application_name` so long that POOL accepts its packet and the node refuses the packet of POOL. The client then gets `08006`. Nothing is written, and the default of 8192 cannot reach it.
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
| The eight `bootstrap.*` reasons of [spec 0003](../0003-control-and-session-data-model/index.md#bootstrap) | 1 | `bootstrap` | `JournalRead::bootstrap_plan` refused. One finding per reason, in its order | `bootstrap is not possible in this state` |
| `state.not_found` | 3 | all three | The state directory does not exist | `the state directory does not exist` |
| `state.permissions` | 3 | all three | The directory or a journal file fails the check of [State directory and protected files](#state-directory-and-protected-files), or `pool.lock` or the empty journal file could not be created, for whatever reason (the second half was added on 2026-11-16, decision 114) | `the state directory or a file in it has the wrong type, mode or owner` |
| `state.locked` | 3 | all three | Another process holds the lock | `another process is using the state directory` |
| `secret.not_found` | 3 | `run`, `bootstrap` | A secret file, a certificate file or a CA file does not exist | `a file named by the configuration does not exist` |
| `secret.permissions` | 3 | `run`, `bootstrap` | A secret file fails its check | `a secret file has the wrong type, mode or owner` |
| `secret.invalid` | 3 | `run`, `bootstrap` | A secret, a verifier, a certificate, a key or a CA file cannot be used as what it is | `a file named by the configuration does not hold what its key needs` |
| `journal.failed` | 3 | `run`, `bootstrap` | A write to the journal failed | `the journal could not be written` |
| `random.failed` | 3 | `run`, `bootstrap` | The operating system gave no random bytes for the authority ID or for the key of the made up verifiers | `random bytes could not be obtained` |
| `listener.failed` | 3 | `run` | The listener could not be bound | `the listener address could not be bound` |
| `runtime.failed` | 3 | `run`, `bootstrap` | The runtime could not be built, or `run` could not install the handler for SIGTERM or SIGINT (added on 2026-11-16, decision 86) | `the runtime could not be started` |

The human and JSON shapes are the ones of spec 0001: exit 1 gives `<command> refused`, `findings: <n>` and the finding lines on stdout, and exit 3 gives one `error: <code>: <message>` line on stderr, or the JSON document with `result` `error`. In JSON, `result` is `refused` for exit 1. The first line of the human format for a rejected configuration stays `configuration invalid`. A finding about a file reads, for example, `error: secret.permissions at principal.verifier_file: a secret file has the wrong type, mode or owner`. It names the key and not the entry, so with several principals you check the file of each one. That is the same limit the findings of spec 0001 have, for the same reason: output never repeats text from the file.

### pool bootstrap

Steps, in order. The first failure ends the command.

1. Read and validate the file. Take `probing()`.
2. Check the state directory and take the lock.
3. Read the journal with `Journal::read`. Read the monitor credential and the CA file of every cluster and node.
4. Probe every node of every cluster once, all at the same time, each with the deadline `probe.timeout_ms`, and record each sample on an `ObservationBoard`.
5. Call `JournalRead::bootstrap_plan`. A refusal is reported and nothing is written.
6. Make an authority ID from 16 random bytes of the operating system, with `pool_core::random::fill_random`. A failure is `random.failed`, and no file is created. Create the empty journal file. Call `Journal::create`, which writes it in one transaction. (Added on 2026-11-16, decision 114: the code did this and no text said so.) When the empty journal file cannot be created, the finding is `state.permissions`, whatever the reason is: a disk that is full, a directory that cannot be written, no inode left. A path that cannot be looked at gives the same code, and so does a `pool.lock` that cannot be created in step 2. The message of that code speaks of type, mode and owner, which is not the cause in those cases, so look at the disk too when you see it. Nothing was written to a journal at that point. Scope 009 may give these failures a code of their own (Follow-up 7).
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

**It does not show a conflict latch** (added on 2026-11-15, after the verification of milestone 4). The result speaks of the authority record: whether the journal can be read and whether it matches the witness. A conflict latch is kept per cluster, apart from that record, and no result of this command names it. So after a second primary was seen, `show-authority` says `ready`, the daemon starts, and every login to that cluster gets `57P03`. The reason is in the log of the daemon: `authority.conflict_latched` in the run that set the latch, and `admission.changed` with the reason `authority.conflict_latched` in that run and in every later one. Scope 009, which owns clearing a latch, also owns a command that shows one (Follow-up 7).

**It reads no secret** (added on 2026-11-16, finding d3 of the independent review). `0004 AC-9` lists `pool show-authority` among the commands that exit 3 with `secret.permissions`. The command reads the configuration and the journal and no secret file, so it can never give that code. The table under [Findings of the new commands](#findings-of-the-new-commands) is right: `secret.*` belongs to `run` and `bootstrap`. Read the second sentence of the criterion for those two. The criterion keeps its text.

### pool run

Steps, in order. Until step 8 every failure is a finding with exit 1 or 3, written as spec 0001 says. From step 8 on every line on stderr is a log event.

1. Read and validate the file. Take `serving()`. Give the configuration its generation, `ConfigurationGeneration::FIRST`.
2. Check the state directory and take the lock, kept until the process ends.
3. Read the journal without writing and classify it with `JournalRead::classify`. Any state but `Ready` ends here with the finding `authority.` followed by the name of the state (`0004 AC-13`).
4. Read every protected file: the key and the certificate chain of the listener, the verifier of every principal, the credential of every identity and of every cluster, the CA file of every node. Each distinct credential file gets `CredentialGeneration::FIRST`. Make the key for the made up verifiers from 32 random bytes. A failure is `random.failed`. (Corrected on 2026-11-16, nit n14: the code reads them in this order, and a file that fails is reported in this order: the certificate chain and then the key of the listener, the verifier of every principal, the credential of every identity, the CA file of every node, the monitor credential of every cluster.)
5. Open the journal for writing with `Journal::open_ready`, which advances the start sequence in one transaction. A failure is `journal.failed`.
6. Build the runtime.
7. Bind the listener. The runtime resolves `listener.host` and binds the first address that works. A failure is `listener.failed`. (Added on 2026-11-16, a nit of the second review: the code did this and no text said so.) The listener socket has address reuse turned on (`SO_REUSEADDR`), so that a restart can bind while connections of the old process still close, and a backlog of 1024.
8. Install the log writer.
9. Start the control loop with an empty observation board. Publish the first route view, in which every cluster is closed. Accept connections. Write the event `daemon.started`.
10. On SIGTERM or SIGINT, shut down as [Shutdown](#shutdown) says and exit 0.

The daemon classifies once. A journal state other than `Ready` is refused at the start and not served with every login refused, because a process that looks alive and admits nothing is harder to notice than one that did not start.

**A start that fails after step 5 uses up a start sequence** (added on 2026-11-15, after the verification of milestone 4). The journal is advanced in step 5 and the listener is bound in step 7, so a port that is taken costs one number. That is fine and stays. The start sequence has one job, which is that no two runs share a session ID. For that the numbers must never repeat. They need not follow one another without a gap. Step 4 comes before step 5 so that the common mistake, a secret file that is missing or has the wrong mode, writes nothing to the journal. The bind comes after step 5, because a bound listener is a port on which the kernel already takes connections, and that should not exist while the start can still fail on the journal.

**What the daemon does when a part of it ends** (added on 2026-11-16, decision 86, findings c1, c2 and d3 of the independent review). Three additions to the steps above.

- **Step 6 and step 8.** A runtime that cannot be built is `runtime.failed`. The first build reported it as `listener.failed`, which named the wrong thing. The handlers for SIGTERM and SIGINT are installed before step 9, and a handler that cannot be installed is `runtime.failed` too. The first build went on without it and could then never be stopped cleanly.
- **The daemon watches its own tasks.** The control loop, each probe task, the listener task and the log writer are tasks of the runtime. When one of them ends without a shutdown having asked for it (it panicked, or it returned), the daemon writes the event `daemon.failed` with the name of the task, shuts down as [Shutdown](#shutdown) says from step 4 on, as if the deadline had passed, and exits with status 3. The reason is the sentence above about a process that looks alive. A daemon whose control loop is gone refuses new logins once the routes are old, which is right, and it can no longer end a running session on a second primary or write a latch, which is not. Ending is the closed side.
- **A panic writes an event.** At step 8 the daemon installs a panic hook that writes the event `daemon.panicked` through the log writer and nothing else. It does not print the message or the place of the panic, because a message can hold a value. So every line on stderr stays one JSON object (`0004 AC-24`), also when a task panics. A panic is always a defect.

## State directory and protected files

The code lives in `apps/pool` (the directory and the lock) and in `pool-core` (secret files), because `pool-control` makes no file call.

**The service account** is the owner of the state directory. Every check below compares owners with it, so the checks need no call that asks the operating system who the process is.

| Path | Must be | Otherwise |
|---|---|---|
| `control.state_directory` | A directory, mode exactly `0700` | `state.not_found` or `state.permissions` |
| `pool.lock`, `journal.sqlite3`, `journal.sqlite3-wal`, `journal.sqlite3-shm` in it, when they exist | A regular file, mode exactly `0600`, owned by the service account | `state.permissions` |
| Any other entry in the state directory | Not looked at | |
| A secret file: a credential, a verifier, the private key of the listener | A regular file after links are followed, no group and no other permission bit (so `0600` or `0400`), owned by the service account | `secret.not_found` or `secret.permissions` |
| A certificate chain or a CA file | A regular file that can be read | `secret.not_found`, `secret.permissions` when the operating system refuses the read, or `secret.invalid` |

- A process that is not root can read a `0600` file only if it owns it, so for such a process the owner rule adds nothing to the mode rule. For a process that runs as root the owner rule is what keeps a file of another account out. Running as root is not supported, and nothing detects it.
- **The lock** is the file `pool.lock` in the state directory. The command creates it if it is absent (mode `0600`), opens it and calls `File::try_lock` of the standard library, which takes an exclusive advisory lock. When the lock is held by another process the command exits 3 with `state.locked`. The lock is released when the process ends, also when it is killed. The file is never removed. This is the exclusive lock that Follow-up 4 of spec 0003 asks for: two daemons cannot share one state directory.
- **A strange umask and the lock file** (added on 2026-11-15, after the verification of milestone 3). `pool.lock` is created with mode `0600` less the umask, and the mode is set to `0600` on the open file right after. A command that opens the file between those two steps sees the mode before the second step. Under a umask that leaves the owner bits alone, which is every umask a service runs under, that mode is `0600` already. Under one that takes read or write away from the owner (the checker ran 0200, 0400, 0600 and 0777), a command that starts in that moment says `state.permissions`: 43 of 1920 commands that were started together. That is what `0004 AC-9` gives for a file whose mode is not `0600`, and in that moment the mode is not `0600`. Nothing is written, the file ends with mode `0600`, and the next start passes. So the edge stays as it is.
- **What the lock does not cover** (added on 2026-11-15, from the session that fixed the race on the lock file; neither case was tested). The lock belongs to the file that was opened, not to its name. If someone removes `pool.lock` while a daemon runs, the next command creates a new file, locks that one and does not see the lock of the daemon. POOL never removes the file, and only the service account or root can, so this takes an act of the operator in a directory that is theirs. Do not remove `pool.lock`, also not to clean up. The second case is a state directory on a network file system. Whether an advisory lock holds between two hosts there depends on the file system and on how it is mounted. SQLite does not support WAL mode there at all, because every process that uses the database must share memory on one host. So the state directory must be on a local file system of the host that runs the daemon. Nothing detects either case. The [Security model](#security-model) names both as outside what this slice defends, and Follow-up 11 says who owns what comes after.
- The message of `secret.permissions` speaks of a secret file. For a certificate chain or a CA file it means that file, and the key in the finding says which one.
- **How a protected file is opened** (added on 2026-11-15, after the verification of step 7). First the path is looked at without opening it, and anything that is not a regular file after links are followed is refused. That keeps a named pipe from making the start wait. Then the file is opened, and the type, the mode and the owner are checked again on the open file, not on the path, and the bytes are read from that same open file. So the file that was checked is the file that is read, also when the path is changed in between. This order holds for secret files and, without the mode and owner rule, for certificate and CA files.
- Files are read through a reader with a limit: 4096 bytes for a credential or a verifier, 65536 bytes for a private key, 1048576 bytes for a certificate chain or a CA file. A longer file is `secret.invalid`.
- All checks are for Unix. The product has one declared target, which is Linux.
- **A certificate chain or a CA file must not be writable by others** (added on 2026-11-16, decision 88, finding c6 of the independent review). The first rule asked only for a regular file that can be read. A CA file is what tells POOL which server is the node, so an account that can write it can add a root of its own and stand between POOL and the node. The trust root was protected less than the password it protects. From now on such a file must have no write bit for the group or for others, and its owner must be the service account or root. Otherwise the finding is `secret.permissions`. It may be readable by everyone, because a certificate is no secret. The check is made on the open file, like the checks of a secret file. A test copies a committed fixture into its own folder and sets the mode there, so that the mode of a checkout does not decide a result.
- **A link at the place of the state directory** is refused with `state.permissions` (added on 2026-11-16, finding d3: the code did this and no text said so). The path is read without following a link, and a link is not a directory.

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
- **The reader** opens the file with the flags for read only access, runs no statement that writes, and runs `PRAGMA quick_check`. It tells an absent file from one that cannot be opened by asking `Path::try_exists` first. An **empty database** is `Absent` too: it is what a bootstrap leaves behind when it stops between creating the file and committing its one transaction, and it must not block the next bootstrap. A file is an empty database when SQLite can open it, `PRAGMA application_id` is 0, `PRAGMA user_version` is 0, the table `sqlite_schema` holds no row and `PRAGMA quick_check` says `ok`. A file of 0 bytes meets that, and so does the one page that `PRAGMA journal_mode = WAL` writes into such a file before the transaction starts. (Changed on 2026-11-15, after the verification of step 6. The first rule asked for a page count of 0, and the checker showed that a bootstrap which stops after it set the journal mode leaves one page, which then read as `Unreadable` and blocked every later bootstrap.)
- **A damaged journal is never an empty database.** This was run on 2026-11-15 with the bundled SQLite 3.54.1. A journal cut to its first page, or to its first two, cannot be read at all (`database disk image is malformed`). One whose header is overwritten is `file is not a database`. One whose two marks are set to 0 while its tables stay has rows in `sqlite_schema`. Each of these is `Unreadable`. The only files that read as empty are a file that never got its transaction and a file that someone put in the place of the journal, and the second is the same as a journal that was removed: with a witness in the configuration the state is `Missing` and stays closed, as `0003 AC-8` says, and without one nothing was ever admitted. So an empty database authorizes exactly as much as an empty directory, which is nothing.
- **Times**: `created_unix_ms` and `changed_unix_ms` are passed in by the caller, which reads the wall clock. They are for display. No rule reads them.
- **File mode**: the binary creates the empty file itself, before it calls `Journal::create`: with `create_new` and the mode `0o600` of `std::os::unix::fs::OpenOptionsExt`, followed by `set_permissions` to `0o600`, so that a strange umask cannot leave another mode. It creates `pool.lock` the same way. SQLite gives its side files the mode of the main file.
- **Storage failure while running**: the journal is read once, at the start. After that it is only written, and only to set a conflict latch. When that write fails, the latch still holds in memory for the rest of the run, the event `journal.write_failed` is logged, and the write is tried again in every later round of the control loop. Sessions that already run go on. New sessions follow the snapshot, as before. This settles the sentence that spec 0003 left open under [The journal](../0003-control-and-session-data-model/index.md#the-journal). (Added on 2026-11-15, after the verification of milestone 4.) The event is written once for each try that fails, so once per round and latch: with two nodes and a period of 200 ms the checker counted 16 lines in about a second, at the level `error`. The tries follow one another, each after the one before has answered, so they do not pile up. Unlike a refusal, no client can cause this line. It needs a conflict that was seen and a disk that fails at the same time. So it stays as it is in this slice, and scope 008 owns writing it once per change, as `probe.failed` is written (Follow-up 6).

```rust
// crates/pool-control/src/journal.rs, the only file that names rusqlite
pub struct Journal { /* private: one connection */ }
pub struct LatchReceipt { /* private */ }      // cluster() -> &ClusterId, authority_revision() -> AuthorityRevision
pub enum JournalError { Open, Busy, Changed, Write }   // as_str(): "journal.open", "journal.busy", "journal.changed", "journal.write"

pub struct JournalRead { /* private: what the reader found */ }
impl JournalRead {
    pub fn classify(self, config: &ValidatedConfig) -> AuthorityState;
    pub fn bootstrap_plan(
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

- `Journal::read` is the journal reader of spec 0003. `JournalRead` wraps its `StoreProbe` and gives no access to it: no field, no accessor that returns it, no way to change it. Its `Debug` text is the derived one and prints the journal value. That is allowed: everything in it (the authority ID, system identifiers, cluster and node IDs, generations) may appear in default diagnostics by the [telemetry table](../0003-control-and-session-data-model/index.md#what-may-appear-in-default-telemetry) of spec 0003, and text that is printed cannot be turned back into a value. `classify` calls `classify_startup`. `bootstrap_plan` calls the function `plan_bootstrap` with the samples of the board, the expiry of the configuration and the later of `now` and the latest sample time. `authority_id` is the ID of a journal that was read, also when the witness does not match, and `None` when no journal could be read. (The method was called `plan_bootstrap` until 2026-11-15. That name holds a text the source guard refuses outside `pool-control`, so `pool bootstrap` could not call it. The rule from now on: a function or method that another crate calls never holds one of the five guarded texts in its name. The guard itself stays as strict as it was.)
- `AuthorityState` gains `name()`, which returns `uninitialized`, `unconfirmed`, `missing`, `unreadable`, `mismatch`, `restore_pending` or `ready`. The commands build their output and their findings from it.
- `create` fails with `Open` when the file is anything but an empty database, by the rule above. After its transaction is committed it runs `PRAGMA wal_checkpoint(TRUNCATE)`, so that the main file itself carries the marks and the tables from the moment bootstrap returns, and not only its side file. Without that, the main file alone reads as an empty database until SQLite checkpoints. A checkpoint that fails does not fail `create`: the transaction is already durable in the side file. It writes the marks, the four tables and the rows in one transaction, with revision 1, start sequence 0, hold `none` and epoch 1 for each cluster of the plan. (Corrected on 2026-11-16, finding d3 of the independent review, a consequence of decision 59.) The checkpoint is tried once and a failure of it is ignored. So "the main file itself carries the journal" holds when the checkpoint worked. When it did not, the journal is complete in the main file and the WAL file together, the reader reads both, and the next open that writes checkpoints again. Nothing is lost as long as the WAL file stays beside the main file. Copy or move the journal only as the three files together, or after a clean stop.
- `open_ready` starts a transaction with `BEGIN IMMEDIATE`, reads the authority ID, the revision and the start sequence again, and fails with `Changed` when they are not the ones the `Authority` was classified from. It then applies migrations (none exist for version 1) and advances the start sequence. The returned value is the new start sequence. At the largest value it fails with `Write`.
- `set_conflict_latch` sets `conflict_latched` to 1 for the cluster and advances the authority revision, in one transaction. When the latch is already set it changes nothing and returns a receipt with the current revision. At the largest revision it fails with `Write`, and the latch stays in memory. It fails with `Changed` when the authority ID or the revision in the file is not the one this `Journal` value last read or wrote, which means something else wrote the file, and with `Write` for a cluster that the file has no row for. Neither can happen while the lock on the state directory holds and the controller asks only for clusters of the state. Both leave the latch in memory.
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
- A row with another number of columns, a null, text that does not parse, a timeline of 0, an error response, more than one row, or more than 4096 bytes of answer gives `ObservationOutcome::Failed`. The 4096 bytes are counted twice, on purpose: by the backend connection, which stops reading the answer there, and by `observation_from_row`, on the text of the three columns together. The role must be exactly `true` or `false`. The timeline must be decimal digits only, with no sign and no space, as PostgreSQL prints it.
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
    pub fn state(&self) -> &AuthorityState;   // read only, for the log and for tests
    pub fn board(&self) -> &ObservationBoard; // read only
}
```

Rules:

- **One sample per node.** `ObservationBoard::record` keeps the sample with the later sample time for a node. When the times are equal it keeps the one it has. So the list it returns never holds two samples of one node. This is the first duty of point m3.
- **No sample is later than now.** `build_snapshot` uses the later of `now` and the latest sample time on the board as its now. A later now only makes samples older, so this can close and never open. This is the second duty of point m3, and it no longer rests on the caller.
- **One caller.** `build_snapshot` is the only code that calls `writer_admission` in the product, and the source guard keeps it so. The rule "Fresh" of spec 0003 is not made stricter: with one sample per node and no sample in the future, the two cases it treats as no evidence cannot reach it.
- **`build_snapshot`** calls `writer_admission` once for every cluster of the configuration, with the expiry from `config.probe()`. For an `Open` answer, `fresh_until` is the sample time of the approved writer plus the expiry. For a `Closed` answer it is `None`.
- **`Controller::round`** works in memory only and never waits for the disk. It builds a snapshot. For each cluster whose answer is closed with `latches_conflict()`, it sets the latch in the state with `AuthorityState::latch_conflict` (`0003 AC-17`) and puts the cluster on the list of latches that the journal does not hold yet. It then builds the snapshot again, so the snapshot it returns already shows `authority.conflict_latched`. Each round advances the snapshot generation by one. When the generation cannot advance, the round returns every cluster closed: it builds the snapshot as if the state were `Unreadable`, so the reason is `authority.unreadable`, and it keeps the last generation. That reason is not true to the letter, and it is one that exists already. The case needs more rounds than a daemon can run in any lifetime, so no new reason name is spent on it.
- **Closing comes first, the disk second.** The loop publishes the result of a round at once. Only then, when `unrecorded_latches` is not empty, it calls `Journal::set_conflict_latch` for each of them inside `spawn_blocking` and hands each receipt to `latch_recorded`, which takes the cluster off the list and gives the state the authority revision of the receipt when that is higher than the one the state holds. A revision in memory never goes back, whatever order receipts arrive in. A write that fails leaves the cluster on the list, is logged as `journal.write_failed`, and is tried again after the next round. So a second primary closes the cluster for new sessions as fast as a round runs, whatever the disk does.
- **A primary that the journal did not approve latches, also when the configuration expects it** (added on 2026-11-16, decision 107, from the checker run of Build plan step 15). When `expected_writer` names another node than the journal approved, the cluster is closed with `authority.config_conflict`. When that other node then answers as a primary with the system identifier of the cluster, the round also sets the conflict latch: it is a primary beside the approved writer, whatever the configuration says. This is intended. A configuration is not an authority. Changing `expected_writer` approves nothing, and a node that was promoted outside POOL is exactly the second primary the latch exists for (`0001 SC-8`, and the invariant that a promotion needs a reviewed operation). It costs nothing in this slice. The cluster was closed anyway, and the one way to make that node the writer is a new bootstrap, which starts a journal that has no latch. The operation of scope 009 and scope 010 that approves a new writer has to deal with a latch that was set this way (Follow-up 7). (Corrected on 2026-11-16, finding a2 of the addendum to the second review: the last three sentences left two things out.) First, the way to a new bootstrap. `pool bootstrap` refuses while a journal or a witness is present. So you stop the daemon, remove the journal files and the witness from the configuration, and bootstrap again, as [Consequences](#consequences) says for every latch. Second, what "it costs nothing" means. The latch does not come from the mistake in the file. The rule that sets it never reads `expected_writer`: it would have been set with a right configuration too, because a primary beside the approved writer was there. So the mistake adds no latch of its own. But putting `expected_writer` right again does not take the latch away either. Nothing does in this slice. A cluster whose configuration was wrong for a while, beside a node that answered as a primary, stays closed after the file is right, until scope 009 or until that new bootstrap.
- **When the loop runs a round**: after each sample it receives, and at least once per `probe.period_ms`.
- **Publishing.** The loop maps the snapshot into the `RouteView` of `pool-core` ([Route view](#route-view)) and sends it. `pool-core` never sees a type of `pool-control`.
- **Cost** (point m6). A session reads the route view, which is one lookup by cluster. It never calls `writer_admission`. The function runs once per cluster per round. With the node counts of a lab that is nothing. A bound for very large files stays open, and scope 008 owns it, as spec 0003 allows.

The loop reads the monotonic clock with one `Instant` taken at the start of the daemon, and converts with `MonotonicTime::from_origin`. The data plane compares `fresh_until` with the same origin.

### When a running session is ended

Spec 0003 left to this slice whether a session that already runs is ended when admission closes later.

- **Silence does not end a session.** When the cluster closes with `evidence.writer_missing` alone (the probe stopped answering, or its sample expired), running sessions go on. New sessions are refused. A probe that stumbles must not cut work that still flows.
- **A positive conflict ends it.** When the reasons of the cluster hold any of `evidence.writer_identity`, `evidence.writer_in_recovery`, `evidence.writer_timeline`, `evidence.second_primary`, `evidence.timeline_ahead` or `authority.conflict_latched`, every running session of that cluster is ended: the backend is closed, and at a message boundary the client gets `08007` when a query is in flight and `57P01` when none is. Not at a message boundary the connection is closed without a message. The node is then no longer the verified primary, and POOL keeps writes on the verified primary only.

- **Silence that lasts ends it too** (added on 2026-11-16, decision 84, `0004 AC-30`). The first point is for a probe that stumbles. It is not for a node that is gone. When the probe of the approved writer of a cluster has failed with the class `connect` or `timeout`, and with no other outcome in between, for `probe.node_loss_timeout_ms`, the control loop marks the node as lost. Every running session on it is then ended as [Backend loss](#backend-loss) says: the backend is closed, the client gets `08007` when a query is in flight and `08006` when none is, and `session.ended` has the reason `node_lost`. A good sample starts the time again. So does a failure of the class `tls`, `login` or `answer`: there the node answered, so it is there, and what is wrong is the probe or its login. The loop writes the event `node.lost` once when it marks a node, and the mark goes with the first good sample. A positive conflict wins over this mark. The probe hands the class of each failure to the loop together with the sample, and the loop keeps, per node, the time of the first failure of those two classes since the last other outcome. `pool-control` does not change for this. (Added on 2026-11-16, decision 105, on how the control loop reads "the approved writer".) The loop takes the node from `expected_writer` of the configuration, because `apps/pool` cannot read a row of the journal. That is the same node whenever it matters: `writer_admission` opens a cluster only while `expected_writer` names the approved writer of the journal, and neither changes while a daemon runs, so a session can only ever run on that node. A cluster whose two values differ never opens in that run and has no session to end. The mark is kept per node and goes only with a good sample. A failure of the class `tls`, `login` or `answer` starts the time again and leaves a mark that is already set, which is right: the sessions of a marked node are ended already, and its cluster stays closed until a good sample opens it. The time runs from the arrival of the first failure, not from the last good sample. And `node.lost` can be written for a writer that never answered in this run, 30 seconds after the start with the defaults. That line is true and ends nothing, because such a cluster never opened.

The route view carries one flag per closed cluster that says which of the two it is. Scope 008 extends this with drain. (Changed on 2026-11-16: there are three cases now, and the view carries two flags per closed cluster, `conflict` and `lost`.)

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

- A length below 8 is `08P01`. A length above `limits.startup_packet_max_bytes` is `54000`. Both close the connection, and the body is never read. (Added on 2026-11-16, decision 115: the rules were each written and their order was not.) The order is: the length below 8, then the length above the limit, then the code, then whether TLS is on. So a `StartupMessage` without TLS that is longer than the limit gets `54000` and not `28000`, one that claims a length below 8 gets `08P01`, and a packet without TLS that names another major version gets `0A000`. Each of these is a refusal in plain text before any byte of a body is read, so the order changes what a client is told and nothing else.
- **The body of a first packet is read in two cases only**: a `StartupMessage` that arrived over TLS, and a `CancelRequest`. For every other packet POOL decides from the 8 header bytes. In particular it never reads the parameters of a `StartupMessage` that came without TLS: it answers `28000` and closes. This must not rest on the caller. `FirstPacketReader` of `pool-protocol` keeps the `Handshake` of its connection, asks it as soon as the 8 bytes are there, and returns a refusal or a step at that moment. One reader serves one connection, before and after its TLS handshake. (Added on 2026-11-15, after the verification of milestone 2: the first build of the reader read and parsed the body without asking, so the order was the caller's duty. Until the reader is changed, a caller uses `decode_first_header` and `Handshake::next` on a connection without TLS, and the reader only after it.)
- A connection may send at most three first packets: an optional `GSSENCRequest`, then `SSLRequest`, then, over TLS, a `StartupMessage` or a `CancelRequest`. A `CancelRequest` may also be the very first packet. Any other order is `08P01`.
- The TLS server offers TLS 1.3 and TLS 1.2 with the defaults of `rustls`, asks for no client certificate and offers no ALPN protocol. Direct TLS (a handshake as the first bytes) is not supported: its first bytes fail the length rule.
- **Parameters.** The body is pairs of text, each ended by a zero byte, and one more zero byte at the end. A name that appears twice, an empty name, or bytes after the end are `08P01`. `user` is required, or `08P01`. `database` defaults to the value of `user`. (Changed on 2026-11-16, decision 116.) A `database` that is sent with an empty value counts as not sent, so it takes the value of `user` too. That is what PostgreSQL does, and the first build did not: it looked up an identity entry for the empty name and answered `28000`. A client can reach the same entry by leaving the parameter out, so this opens nothing. (Added on 2026-11-16, decision 115.) A `user` with an empty value is not `08P01`. Only a `user` that is missing is. An empty name is a name that is no principal: it goes through the whole exchange with a made up verifier and ends with `28P01`, like every other unknown name ([Frontend authentication](#frontend-authentication)). PostgreSQL refuses an empty user name at once with `28000`. POOL does not, on purpose: one rule for every name that is no principal, and no answer that is special for one name.

| Parameter | What POOL does |
|---|---|
| `user` | The claimed principal. Verified by authentication, never sent to PostgreSQL |
| `database` | The requested database of the identity map. Never sent to PostgreSQL |
| `application_name`, `client_encoding` | Sent to PostgreSQL unchanged in the startup packet of the backend |
| A name that starts with `_pq_.` | Not supported. Named in `NegotiateProtocolVersion` |
| `replication` | `0A000` |
| `options`, and every other name | `0A000`. A client with `PGOPTIONS` set in its environment sends `options` and is refused |

- **Version.** POOL speaks 3.0. For a request with a minor version above 0, or with a `_pq_.` parameter, it sends `NegotiateProtocolVersion` with the protocol code of 3.0 and the names of those parameters, directly after it read the startup packet and before the authentication request, and goes on with 3.0. A client that needs more closes by itself.
- **The first number of `NegotiateProtocolVersion` is a whole protocol code**, not a bare minor number: the major version in the high 16 bits and the minor version in the low 16 bits, as in the startup packet. For 3.0 that is 196608, the bytes `00 03 00 00`. It is the version the connection goes on with. Then follows the count of the parameters that are not supported, as an Int32, and each name ended by a zero byte. The message is `v`, and it comes before the authentication request. (Corrected on 2026-11-15. The first version of this section said "minor version 0", the encoder wrote the number 0, and `psql` 18 refused the connection in the fixture of Build plan step 11 with `server requested downgrade to pre-3.0 protocol version`. The documentation of PostgreSQL 18 calls the field the newest minor protocol version, and a real PostgreSQL 18.7 sends the whole code: `00 03 00 02` to a request for 3.3, and `00 03 00 00` to a request for 3.0 with an unknown `_pq_.` parameter. The server is the reference.)
- **The startup profile** of this slice has one value, `DedicatedV1`: the backend is opened with the mapped login, the mapped database and the two parameters above, when the client sent them. It is one part of the pool key.

### How each method names the principal

This slice has one frontend method. With `SCRAM-SHA-256`, the verified principal is the value of `user` in the startup packet, byte for byte, once the SCRAM proof against the verifier of the principal entry with that `name` has passed. The user name inside the SCRAM message is ignored, as PostgreSQL ignores it. The identity map is then asked with that name and the requested database. A slice that adds a method (a client certificate, for example) writes its own row here.

### Messages after login

After `ReadyForQuery` has been sent to the client, every message has a tag, then a length that counts itself and not the tag. POOL reads the 5 bytes of the header, checks them, and then copies the body through a buffer of fixed size. It never holds a whole message.

From the client, read only while no query is in flight:

| Tag | Message | What POOL does |
|---|---|---|
| `Q` | `Query` | Length above `limits.frontend_message_max_bytes`: `54000`, and no byte is sent on. Otherwise the message is sent to the backend and a query is in flight |
| `X` | `Terminate` | The length field must be 4, which is a message with no body. Then it is sent to the backend and both connections are closed. Any other length is `08P01`, and the backend is closed. The size limit is not asked, because there is nothing to measure |
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
| The backend could not be opened: no connection, TLS failed, the login failed, the node is not a primary, the node is not the server that the journal approved (step 8 of [Backend connection](#backend-connection), added on 2026-11-16), or the deadline passed while connecting or while asking | `08006` | `the backend connection could not be established` |
| The backend was lost, the node was marked lost (added on 2026-11-16), or POOL ended the session, while a query was in flight | `08007` | `the connection was ended; the outcome of the statement in flight is unknown` |
| The backend was lost, or the node was marked lost (added on 2026-11-16), while no query was in flight | `08006` | `the backend connection was lost` |
| Shutdown, or the cluster closed with a positive conflict, while no query was in flight | `57P01` | `the session was ended by the proxy` |

### Frontend authentication

After a valid startup packet over TLS:

1. POOL sends `AuthenticationSASL` with the one mechanism `SCRAM-SHA-256`. It never offers `SCRAM-SHA-256-PLUS`, because this slice does not compute channel binding data (`SEC-3`).
2. The client answers with `SASLInitialResponse`. The mechanism must be `SCRAM-SHA-256`. The first message must start with `n,,` or `y,,` and hold a nonce of printable characters. A message that asks for channel binding (`p=`), an authorization identity or an extension is `08P01`. The user name in it (`n=`) is read up to the next comma and then ignored, whatever it holds: POOL does not check its escapes, because PostgreSQL does not, and a client that PostgreSQL accepts must not fail here. The message as a whole must be UTF-8 text, or `08P01`.
3. POOL sends `AuthenticationSASLContinue`: the client nonce followed by its own nonce (18 random bytes in base64), the salt and the count of the verifier.
4. The client answers with its final message. The channel binding field must repeat the header of step 2, the nonce must be the whole nonce, and the proof must be 32 bytes.
5. POOL computes the client key from the proof and the stored key of the verifier, and compares its SHA-256 with the stored key in constant time. A match makes the principal verified. POOL then sends `AuthenticationSASLFinal` with the server signature.
6. No match: `28P01`.

- **A user name that is no principal** goes through the same six steps with a made up verifier: the salt is the first 16 bytes of an HMAC of the name under a key of 32 random bytes that the daemon makes at start, and the count is the count of the verifier of the first principal in the file. A deployment makes its verifiers with one setting, so the count does not tell a real name from a made up one. Step 5 always fails. So a client cannot tell an unknown name from a wrong password, and asking twice gives the same salt. (Added on 2026-11-16, finding c7 of the independent review: what `0004 AC-16` covers and what it does not.) The made up verifier has a salt of 16 bytes and the count of the first principal. So "a client cannot tell" holds when every verifier file has a salt of 16 bytes and the same count, which is what PostgreSQL makes with one setting of `scram_iterations`, and nothing checks that. A principal whose verifier has another salt length or another count answers with another shape than an unknown name. And the key behind the made up salt is new at every start, so the salt of an unknown name changes over a restart and the salt of a real one does not. A client that asks twice, once before and once after a restart, can tell. Make all verifiers with one setting. Scope 006 owns a check at start for verifiers of different shapes and a key that outlives a restart (Follow-up 4).
- Each authentication message from the client has a body of at most 1024 bytes. The body is what follows the tag and the length field, so the length field is at most 1028. A longer one is `08P01`, and its body is not read. Tests and the fixture use this edge: 1024 passes, 1025 is refused.
- The whole exchange, from the accepted connection to step 5, has the one deadline `limits.startup_timeout_ms`. At the deadline the connection is closed without a message.
- `AuthenticationOk` is not sent yet. It follows only when a backend is ready ([Session lifecycle](#session-lifecycle)). A client therefore never sees a successful login without a backend behind it.
- POOL as a server does no PBKDF2 work: the verifier already holds the keys. The cost of one login attempt is a few HMAC calls.

### Backend connection

`pool-core` opens every connection to a node the same way, for a session and for the probe.

1. Connect by TCP to the `host` and `port` of the node. A name is resolved by the runtime at each attempt, and the addresses are tried in the order they come back, all within the deadline of the attempt. A lookup uses a blocking thread of the runtime, so prefer addresses in a lab. (Added on 2026-11-16, decision 84.) Each socket is made with `tokio::net::TcpSocket` and has TCP keepalive turned on before it connects ([A peer that vanished](#a-peer-that-vanished)).
2. Send `SSLRequest` and read exactly one byte. Anything but `S` ends the attempt. POOL never falls back to plain text (`SEC-1`).
3. Run the TLS handshake as a client. The roots are the certificates of `node[].tls_ca_file` and nothing else. The name checked is `node[].tls_server_name`. The one TLS policy of this slice is called `VerifyFull`, and it is a part of the pool key.
4. Send a `StartupMessage` for protocol 3.0 with `user` (the backend login or the monitor login), `database`, and the forwarded parameters.
5. Authenticate. The node must ask for SASL and offer `SCRAM-SHA-256`. POOL chooses `SCRAM-SHA-256`, also when the node offers `SCRAM-SHA-256-PLUS` beside it. Its first message is `n,,n=,r=<nonce>`: the header `n` says that POOL does no channel binding (the header `y` would make PostgreSQL refuse the login as a downgrade), the name is empty because PostgreSQL takes it from the startup packet, and the nonce is 18 random bytes in base64. The first message of the node must repeat that nonce at its start, and its count must be 1 to 600000. The final message of POOL carries `c=biws`. A final message of the node that starts with `e=`, or a server signature that is wrong, ends the attempt. The salted password is derived with PBKDF2 inside `spawn_blocking`, so a high count cannot stall the runtime. The two keys that come out of it are kept in memory for the credential, together with the salt and the count they were made for, in a type that prints `[REDACTED]`. A later login with the same salt and count uses them again. At most one derivation runs for a credential at a time: a login that needs the keys while another login derives them waits for that result, inside its own deadline, and then uses it. PostgreSQL keeps one salt per role, so this is one derivation per credential and run, also when the first logins after a start arrive together, and a burst of logins cannot keep the probe from a blocking thread. The store holds one entry per credential, the last salt and count. Nodes of one cluster share the salt of a role, so one entry is enough, and a salt that changes because the password of the role was set again simply replaces it. The function that derives takes a count that cannot be 0 (`NonZeroU32`), so it has no case in which it panics. (The waiting, the one entry and the count type were written down on 2026-11-15, after the verification of step 7: eight logins at a cold start had each derived the keys.) A node that asks for a clear text password, for MD5, for any other method, or for nothing at all ends the attempt. (Corrected on 2026-11-16, finding c5 of the independent review.) "One derivation at a time" holds while no login is cut. When the login that derives reaches its deadline, its wait ends and the derivation it started runs on, on its blocking thread, and the next login starts a second one. So a cold start with logins that time out can run more than one. The four blocking threads of the runtime also serve the lookups of host names, the latch write and the writer of stderr, so a latch write can wait behind them. Closing a cluster does not wait: the route is published before the write. Scope 005 moves the derivation into a task that owns the kept keys (Follow-up 3).
6. Read until `ReadyForQuery`. From step 5 to this point every message of the node is read whole, and its body may be at most 16384 bytes. A longer one ends the attempt. Parameter status messages are kept in order (at most 64, at most 16384 bytes together). `BackendKeyData` is kept privately. A notice is dropped. An error ends the attempt, and its SQLSTATE goes into the log event. The status must be `I`.
7. For a session only: the last report of the parameter `in_hot_standby` before `ReadyForQuery` must be `off`. A node in recovery reports `on`, and an old or foreign server reports nothing. Both end the attempt. When the parameter is reported more than once, only the last report counts: `off` followed by `on` ends the attempt. (The words "the last report" were added on 2026-11-15. The first build accepted any report of `off`, which does not meet `0004 AC-14`.) This is a check on the very connection the client will use, on top of the snapshot.
8. For a session only (added on 2026-11-16, decision 85, `0004 AC-29`): ask the node who it is. POOL sends the query of [The probe](#the-probe) on this connection, as one simple `Query`, and reads the answer with the limits of the probe: exactly one row, three columns, no null, at most 4096 bytes. The first column must be the system identifier of the route. The second must be `false`. The third must be the timeline of the route. The status after it must be `I`. Anything else ends the attempt with the class `identity`, and the client gets `08006`. The text of the query lives once, in `pool-control`. The daemon hands it to `pool-core` when it builds the server, because `pool-core` cannot name `pool-control`. `pool-core` reads the three columns by the rules of the probe: `SystemIdentifier::parse` of `pool-config`, exactly `false`, and decimal digits. The route carries the two values to compare with ([Route view](#route-view)). (Corrected on 2026-11-16, finding r4 of the second review.) "Anything else" speaks of an answer. A question that gets no answer before the deadline ends the attempt with the class `timeout`, as every wait of the backend connection does, and the client gets `08006` then too.

One attempt per session. There is no second try and no other node to try (`0001 SC-6`).

**What steps 7 and 8 prove together** (added on 2026-11-16, decision 85, finding M2 of the independent review). They are the check on the very connection the client will use. Step 7 says the server is not a standby. Step 8 says it is the server that the journal approved, on the timeline that the journal approved, which are the same two facts the probe needs to open the cluster. The first build had step 7 only. The probe keeps one connection for a whole run, a session opens a new one, and a name is resolved again each time. So when the address of a node came to mean another server while the old connection of the probe stayed up, the probe went on reporting the old server as the fresh primary, and new sessions went to whatever answered the address. A promoted server of the same cluster passed step 7. With step 8 it does not: a promotion moves the timeline, and another cluster has another system identifier.

- **What it costs.** One more round trip per login, inside the deadline of step 7 of the session. From scope 005 on it is asked once per backend, not once per login.
- **What it needs.** The backend login must be able to run the four functions of the query. Every role can in PostgreSQL 18, unless you took that right away from `PUBLIC`. If you did, every login is refused with `08006` and `backend.failed` names the class `identity`. (Added on 2026-11-16, a nit of the second review.) The names in the query are not qualified with `pg_catalog`. A `search_path` that puts another schema before `pg_catalog` for the backend login or for the monitor login could make them mean other functions. Do not set one. The query with qualified names is a change of the probe text, and scope 007, which qualifies the probe on a replica, owns it (Follow-up 5). A parameter status message that arrives while the question runs is dropped.
- **What you see on the node.** With `log_statement = 'all'` the node logs the question once per login, under the backend login. It changes nothing: it reads, it opens no transaction, and it leaves the status `I`.
- **What it cannot tell apart.** Two servers with the same system identifier, on the same timeline, that both say they are not in recovery. That takes a copy of a data directory that was started without a promotion. The probe cannot tell those apart either.
- **What happens to the cluster.** A refusal of step 8 does not close the cluster. The probe still holds its own evidence, and `pool-core` does not report into the control plane. Every login that lands on the wrong server is refused, one by one, and the log shows `backend.failed` with the class `identity`. That is the closed side, and it is loud.

**One node entry is one server** (added on 2026-11-16, same finding). Give each node entry an address that does not move to another server, and a certificate name of its own in `tls_server_name`. A name that an HA tool points at whichever server is the primary now, or a service name in front of several servers, is not a node entry. If you use one anyway, step 8 refuses every login that lands on a server other than the approved one, so the mistake closes and does not write.

**What a client sees when the certificate of a node is wrong** (added on 2026-11-15, from the fixture of Build plan step 11). `0004 AC-15` says a wrong CA or a wrong name gives the client `08006`. That is the answer of the backend connection itself, in step 3 above. In a running daemon the probe makes the same connection with the same CA and the same name before any session does. So when the configuration names a wrong CA or a wrong name, the probe fails first, with the class `tls`, the cluster never opens, and a client gets `57P03` without a session ever trying the node. Both answers refuse the client before a startup packet goes to the node, and that is what the criterion protects. A client gets `08006` when the probe passed and the connection of its own session then fails: the node answers `N` to the `SSLRequest` of that connection, or presents another certificate than it presented to the probe. The two are proven in different places: the certificate cases in the tests of `pool-core`, where the backend connection meets a wrong CA and a wrong name directly, the `N` case on the fixture through a relay that refuses TLS for the connection of the session only, and the closed cluster on the fixture with a wrong CA and a wrong name in the configuration. For the probe, `pool-core` offers one more call on an open connection: send one constant query and return the text columns of its one row, or nothing.

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
| Message size | `limits.frontend_message_max_bytes` | The length field of a `Query` from the client. It is the only client message with a body that POOL sends on: `Terminate` has none, and every other tag is refused before its length matters | `54000` |
| Authentication message | 1024 bytes, a constant | One SASL message from the client | `08P01` |
| Shutdown deadline | `limits.shutdown_timeout_ms` | Time from the signal | Every session is ended |

Waiters are served in the order they arrived. Per identity limits nest inside the global ones: a principal limit can never raise the session limit. There is no idle limit, no lifetime limit and no deadline on a client that reads slowly. Those belong to scope 005, and until then a slow client holds its own session and its own backend and nothing else.

### Memory budget

`PRD-001 AC-3` asks this spec to declare a budget. It is: **64 MiB for the process plus 1 MiB for each session**, at any setting of the limits. (Corrected on 2026-11-16, finding r4 of the second review: "at any setting of the limits" holds for the session term. Connections that are not sessions yet add the term further down, which grows with two other limits.)

Where the bound comes from:

- Each direction of a session has one relay buffer of 65536 bytes (the constant `RELAY_BUFFER_BYTES`). A session writes what it read before it reads again, so it never queues more than one buffer per direction. (Corrected on 2026-11-16, nit n14: the code has one such buffer per session, used for whichever direction has bytes. That is less than this point says, so the bound holds.)
- Each TLS connection is told to queue at most 65536 bytes for sending (`set_buffer_limit`). What `rustls` holds of received plain text has a limit of its own of about 16 KiB. A session has two connections.
- A startup packet is at most `limits.startup_packet_max_bytes`, a SASL message at most 1024 bytes, and the kept parameter status messages at most 16384 bytes.
- A message body is never collected. A length field is checked before anything is reserved for it, and nothing is reserved from it. (Corrected on 2026-11-16, nit n14: said exactly, a reader reserves only after the check, and never more than the limit it checked against. The relay reserves nothing per message.)

Together that is well under 512 KiB per session. The budget says 1 MiB so that the test has room and still fails for code that buffers results. With the default of 1000 sessions the bound is about 1 GiB. If you run POOL in a machine with less, lower `limits.session_max_count`.

**Connections that are not sessions yet** (added on 2026-11-16, decision 91, finding d1 of the independent review). The budget above counts sessions. A connection in steps 1 to 9 holds up to `limits.startup_packet_max_bytes` for its packet and the state of one TLS connection, and up to `limits.handshake_max_count` of them exist at once. So the whole budget is:

```text
64 MiB + 1 MiB x limits.session_max_count
       + limits.handshake_max_count x (limits.startup_packet_max_bytes + 128 KiB)
```

The 128 KiB is an allowance for the TLS state of one connection, taken from the limits of the TLS library (a handshake message of at most 64 KiB and the two record buffers). It is not measured in this slice, and `0004 AC-21` measures sessions only. With the defaults the new term is 64 x 136 KiB, about 8.5 MiB, and that is inside the 64 MiB of the process, so the sentence above holds as it stands. It stops holding when you raise the two keys: at the top of both ranges the term is 10000 x 192 KiB, about 1.8 GiB, and clients that know no password can fill it. If you raise `limits.handshake_max_count`, add the term to the size of the host. "At any setting of the limits" in the first sentence of this section is true for the session term only.

File descriptors have a budget too. A session holds two, a handshake one, and each node one for the probe. With the defaults that is more than 2100, so the service needs a limit on open files (`LimitNOFILE` in a systemd unit) of at least twice `limits.session_max_count` plus `limits.handshake_max_count` plus 64. When an accept fails, for this or any other reason, the listener counts it under `session.refused` with the reason `accept_failed`, waits 100 ms and goes on. It never ends the daemon.

### Backend loss

When a read from or a write to the backend fails, or the backend closes, or it sends something [Messages after login](#messages-after-login) does not accept:

- The session ends. POOL opens no new backend for it and sends nothing again (`0001 SC-6`).
- At a message boundary the client gets one error: `08007` when a query is in flight, `08006` when none is. Not at a message boundary the connection is closed without one.
- A query is in flight from the moment the first byte of a `Query` was written toward the backend until its `ReadyForQuery` was written to the client. So a COMMIT whose answer was lost gives `08007`. POOL does not know whether it was applied and says exactly that.

**Two errors when PostgreSQL ends the backend itself** (added on 2026-11-15, after the verification of milestone 4). When PostgreSQL ends a backend with a message, as `pg_terminate_backend` does, the client gets two errors. First the `FATAL` of PostgreSQL, passed on unchanged like every message of the backend, which is `57P01` in that case. Then the one error of POOL from the list above. Both rules apply, and that is kept on purpose. The first tells the client what the node said. The second tells it what that means for this session: `08007` when a query was in flight, so that its outcome is unknown, and `08006` when none was. A client library reads both and then the end of the connection. Scope 006, which opens the message rules again, may decide that POOL sends nothing of its own after a `FATAL` of the backend (Follow-up 4).

When the client goes away (it closes, or a read or write fails), POOL closes the backend. PostgreSQL then rolls back whatever was open. POOL sends `Terminate` to the backend first only when no query is in flight.

### A peer that vanished

(Added on 2026-11-16, decision 84, from finding M1 of the independent review. The criterion is `0004 AC-30`.)

A peer can be gone without a FIN or a reset: a host that lost power, a network that drops packets, a NAT entry that ran out. A socket that only waits gets no event for that. The first build set `TCP_NODELAY` on both sockets and nothing else, so a client whose node went silent waited for ever, and a client that vanished kept its permits, its backend and the locks of its transaction until the daemon was started again. Two things notice a peer that is gone now.

**The node, by the probe.** POOL already asks every node a question every `probe.period_ms`. When the approved writer gives no answer at all for `probe.node_loss_timeout_ms`, its running sessions are ended ([When a running session is ended](#when-a-running-session-is-ended)). This bound is POOL's own and does not depend on the kernel: the time until the first probe fails, then `probe.node_loss_timeout_ms`, then the round that sees it. With the defaults that is at most 3 seconds, then 30, then at most 2, so about 35 seconds after the node went away. (Corrected on 2026-11-16, finding r4 of the second review: the first version said 30 seconds plus at most 3 and left out the first failed probe.) A client that waits for an answer from a node whose host is gone gets `08007` after about that time. It is told that the outcome is unknown, which is the truth (`R-10`).

**Both peers, by TCP keepalive.** POOL turns keepalive on for every socket. The listener socket is made with `tokio::net::TcpSocket` and gets `set_keepalive(true)` before it is bound, and on Linux an accepted socket takes that setting over from its listener. This was run on 2026-11-16 on the Linux kernel of the pinned PostgreSQL image: a socket accepted from such a listener reports `SO_KEEPALIVE` as 1, and one accepted from a plain listener reports 0. A socket to a node gets the same call before it connects. This needs no new dependency. When keepalive finds the peer gone, the read on that socket fails, and the rules that exist apply: [Backend loss](#backend-loss) for a node, and for a client the closing of its backend, which gives its permits back and lets PostgreSQL roll back.

**The times of keepalive are the kernel's, not POOL's.** `tokio` can turn keepalive on and cannot set its times. Setting them needs a socket crate as a direct dependency, and this slice does not take that decision (Follow-up 13). On Linux an idle peer that is gone is noticed after `net.ipv4.tcp_keepalive_time` plus `net.ipv4.tcp_keepalive_intvl` times `net.ipv4.tcp_keepalive_probes`. With the defaults of the kernel that is 7200 + 75 x 9 = 7875 seconds, a little over two hours, which is also what a client of PostgreSQL itself gets by default. On the host of the proxy, set them lower: 60, 10 and 6 give 120 seconds. They are settings of the host, or of its network namespace, and they reach every program there that uses keepalive.

**What keepalive does not cover.** While POOL has bytes on their way to a peer that is gone, the kernel sends those again and keepalive waits. The write fails when the kernel gives up, after `net.ipv4.tcp_retries2` tries, which is about 15 minutes with the default of 15. That is the client that vanished while a result was on its way to it. Its real answer is the deadline for a slow reader, which scope 005 owns.

| Who is gone | When | Noticed by | After | Result |
|---|---|---|---|---|
| The host of the node | A query is in flight, or the session is idle | The probe | The first failed probe, then `probe.node_loss_timeout_ms`, then one round | The client gets `08007` or `08006` |
| One connection to the node, while the probe still gets answers | Any | Keepalive | The kernel times | The client gets `08007` or `08006` |
| The client | Idle | Keepalive | The kernel times | The backend is closed and the permits come back |
| The client | Waiting for the answer to a statement | Nothing, until the statement gives output or ends. While a query is in flight the relay does not read from the client, so it does not see what keepalive found. The write of the output then fails (corrected on 2026-11-16, finding r4 of the second review: the first version of this table said keepalive) | The length of the statement | The same. Noticing it earlier is scope 006 (Follow-up 4) |
| The client | A result is on its way to it | The kernel giving up the write | About 15 minutes by default | The same |

Until keepalive fires, a client that vanished holds its session permit, its principal permit, its backend and whatever its open transaction locks. With the kernel defaults that is up to two hours, and 100 such clients can hold every backend of a node for that long. So the kernel values above are part of deploying this slice, not tuning.

**What the evidence covers and what it does not** (added on 2026-11-16, decision 108, from the checker run of Build plan step 15). Shown on Linux: every socket that POOL accepts and every socket that it opens to a node has the keepalive timer pending, also when the client never asked for keepalive, and the first probe is due after the time of the kernel. Not shown: that the kernel then ends a connection whose peer is gone. That needs packets to be dropped, which needs a tool that no pinned image has, and it is the work of the kernel and not of POOL. The rule from the probe needs no such evidence: its hermetic tests and the control loop tests run it whole.

### Shutdown

On SIGTERM or SIGINT:

1. The listener stops accepting. Connections that are still in steps 1 to 9 are closed.
2. Every session with no query in flight and the status `I` gets `57P01` and is ended.
3. Every other session is ended in the same way as soon as it reaches that state.
4. When `limits.shutdown_timeout_ms` has passed, every session that is left is ended. At a message boundary it gets `08007` when a query is in flight and `57P01` when none is. Otherwise it gets no message. POOL never tells a client whose statement may have been applied that it was simply ended (`R-10`).
5. The control loop stops, the journal is closed, `daemon.stopped` is written, and the process exits 0.

A second signal during shutdown does nothing more. This is the minimum for a clean stop. Drain with diagnostics is scope 008.

**Two ends that the first build left open** (added on 2026-11-16, findings c1 and c9 of the independent review).

- **No wait without a race, and a last limit.** Every wait of the relay is raced against a conflict, a lost node and the end of shutdown. The first build left one out: the read of the status byte of `ReadyForQuery`. A node that stopped at that byte kept its session alive through a conflict and through the shutdown deadline. That read is raced now like the others. And step 4 gets a last limit: a session task that has not ended 2 seconds after the deadline of step 4 is aborted, which closes both of its sockets without a message. So the daemon exits at most 2 seconds after `limits.shutdown_timeout_ms`, whatever a peer does.
- **A latch that the journal does not hold yet.** In step 5, before the journal is closed, the control loop tries once more to write every latch that is still on the list of [Control loop and topology snapshot](#control-loop-and-topology-snapshot). For each one that still cannot be written it writes `authority.conflict_latched` with `recorded` false. So a second primary that the process saw is not forgotten at exit without a line. With a disk that stays broken the latch can still be lost, as [The journal on disk](#the-journal-on-disk) says. (Added on 2026-11-16, a nit of the second review.) That last write has no limit of its own, and neither has a latch write inside a round. A disk that stalls, where a failing disk answers with an error, holds the exit until the service manager ends the process. Scope 008 gives the latch write a thread and a limit (Follow-up 6).
- **The order at the end of a session** (nit n1). Step 12 of the session closes the backend first and the client after it. The first build closed the client first, which could keep the backend up to one second longer.

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
    Closed { conflict: bool, lost: bool },   // lost: added on 2026-11-16
}
pub struct Route {
    pub node: NodeId,
    pub topology_epoch: TopologyEpoch,
    pub authority_revision: AuthorityRevision,
    pub fresh_until: std::time::Duration,   // since the origin the daemon passed at start
    pub system_identifier: SystemIdentifier, // of the approved writer; added on 2026-11-16
    pub timeline: u32,                       // the approved writer timeline; added on 2026-11-16
}
```

- A cluster that the view does not hold is closed.
- `conflict` is true when the reasons of the cluster hold one of the names under [When a running session is ended](#when-a-running-session-is-ended).
- Sessions hold a `watch::Receiver` of the view. They read it in steps 5 and 9 and wake on a change while they relay.
- If the control loop stops publishing, every route goes stale at its `fresh_until` and new sessions are refused. Nothing has to tell the data plane.
- `lost` is true when the control loop has marked the approved writer of the cluster as lost ([When a running session is ended](#when-a-running-session-is-ended)). A session that relays ends on it as on a lost backend. When both flags are true, `conflict` decides what the client is told. (Added on 2026-11-16, decision 84.)
- `system_identifier` and `timeline` are what step 8 of [Backend connection](#backend-connection) compares the answer of the node with. The daemon fills them from two new accessors of `EligiblePrimary`, `system_identifier()` and `writer_timeline()`, which give the values of the approved topology that `writer_admission` compared the sample with. `pool-core` still sees no type of `pool-control`: `SystemIdentifier` belongs to `pool-config`, and the timeline crosses as a plain number. (Added on 2026-11-16, decision 85.)
- **A conflict is read as a level** (added on 2026-11-16, nit n8 of the independent review). A session wakes when the view changes and reads the flag as it is then. A conflict that came and went between two looks of one session would be missed. No sequence that does this is known: `evidence.second_primary` and `evidence.timeline_ahead` latch, and the other conflict reasons do not go away by themselves in one round. Scope 008 gives the view a count per cluster, which a session compares in place of a level (Follow-up 6).

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
| `daemon.failed` | error | `task`: `control`, `probe`, `listener` or `log`. Written when that task ended and no shutdown asked for it. The daemon then ends its sessions and exits with status 3 (added on 2026-11-16) |
| `daemon.panicked` | error | none. Written by the panic hook, in place of the plain text line that a panic prints (added on 2026-11-16) |
| `log.dropped` | warn | `count` |
| `admission.changed` | info | `cluster_id`, `state` (`open` or `closed`), `node_id` when open, `reasons` when closed, `snapshot_generation`. Written when the state or the reasons of a cluster change |
| `authority.conflict_latched` | warn | `cluster_id`, `recorded` (whether the journal holds it) |
| `journal.write_failed` | error | `class` (the name of the `JournalError`) |
| `probe.failed` | warn | `node_id`, `class`: `connect`, `tls`, `login`, `timeout` or `answer`. Written when the class of a node changes, not for every sample |
| `probe.recovered` | info | `node_id` |
| `node.lost` | warn | `node_id`, `cluster_id`. Written once, when the control loop marks the approved writer of a cluster as lost (added on 2026-11-16) |
| `session.started` | info | `session`, `principal_id`, `identity_id`, `cluster_id`, `node_id`, `lease_generation`, `topology_epoch` |
| `session.ended` | info | `session`, `reason`: `client_terminate`, `client_closed`, `backend_lost`, `node_lost` (added on 2026-11-16), `unsupported`, `protocol`, `message_limit`, `conflict` or `shutdown`; `status`: `idle`, `in_transaction`, `failed` or `in_flight`; `duration_ms` |
| `session.refused` | warn | `reason`, `count`. One line per reason and second, for the refusals of that second: `handshake_limit`, `startup_timeout`, `plaintext`, `tls`, `protocol`, `unsupported`, `message_limit`, `authentication`, `unmapped`, `session_limit`, `principal_limit`, `closed`, `wait_limit`, `acquire_timeout`, `backend_unavailable`, `cancel_ignored`, `accept_failed`, `random_failed` |
| `backend.failed` | warn | `node_id`, `class`: `connect`, `tls`, `auth_method`, `login`, `not_primary`, `identity` (added on 2026-11-16), `protocol` or `timeout`; `sqlstate` when PostgreSQL sent one; `count`. One line per node, class and second |

**Types that hold what a log may not show.** `0004 AC-24` says that `Debug` of every secret type prints `[REDACTED]`. A secret type, for that criterion, is every type that holds a value from the list at the end of this section: a password, a key, a verifier, a proof, a nonce, a SCRAM message, a name or parameter value that a client or a file supplied, SQL, or error text from PostgreSQL. Such a type never derives `Debug`. It writes `Debug` by hand and prints `[REDACTED]` or only values that this section allows, such as an ID or a count. That includes the types that carry a SCRAM exchange between two messages, on both legs: they hold nonces and the first message of the client, which can carry the user name the client wrote. (Added on 2026-11-15, after the verification of step 7 found a derived `Debug` on `ServerExchange`. Nothing logs that text today. The rule of spec 0001 is that redaction is done by type, so that it does not depend on nobody ever writing `{:?}`.) (Added on 2026-11-15, after the verification of milestone 4.) `Debug` of `SaslInitialResponse` prints the mechanism as `[REDACTED]` too. That is right and stays. The field holds the text the client wrote, before anything compared it with the one name POOL offers, so it is a name that a client supplied.

Refusals are counted and written once a second, so that a flood of bad connections cannot flood the log. Never in any event: a principal name, a login, a database name, a host, a port, a path, SQL, a parameter value, error text from PostgreSQL, a nonce, a proof or a key (`SEC-11`).

**A stderr that nobody reads** (added on 2026-11-16, decision 106, from the checker run of Build plan step 15). When the reader of stderr goes away, for example a pipe whose reader has ended, every write of the log task fails. The task goes on, each line is lost, and the daemon serves on: the checker saw it take a login 15 seconds later and end with exit 0 on SIGTERM. Nothing in the process says so, because the log is the one place where it could. That is accepted for this slice, and for the same reason as the bounded queue: the path of the data does not wait for the path of the log, and a log collector that broke should not become an outage of the database path. The price is real. Such a daemon still closes a cluster and still ends sessions on a conflict, and nobody can read why. `daemon.failed` with the task `log` is written when the log task itself ends, which takes a panic. A closed stderr is not that case. So run the daemon with stderr going to something that outlives its reader, such as the journal of systemd. Scope 008, which owns health diagnostics, counts the writes that failed and shows that the log is not being written through a way that is not the log (Follow-up 6).

## Value sourcing

| Value | Source |
|---|---|
| Principal claimed by a client | `user` in the startup packet |
| Verified principal | The claimed name, after the SCRAM proof against the verifier of the principal entry with that name |
| Verifier | The file named by `principal[].verifier_file`, which you fill from `pg_authid.rolpassword` of a PostgreSQL 18 |
| Salt and count sent to a client | The verifier. For an unknown name: an HMAC of the name under a key made at start, and the count of the first principal's verifier |
| Server nonce, client nonce toward a node, mock key, authority ID | Random bytes of the operating system, through `pool_core::random::fill_random`, which calls `aws-lc-rs` |
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
- **Transport.** TLS is required from the client and toward every node. Toward a node the chain and the name are checked against the CA of that node, and there is no fallback (`SEC-1`, `T-03`). Channel binding is not offered and not used in this slice. With a checked certificate on both legs that leaves no gap that this slice knows of, and scope 006 adds it. (Added on 2026-11-16, finding d2 of the independent review.) That last sentence needs the client to check the certificate of POOL. A `libpq` client with its default settings does not check it. On a direct connection such a client still gets channel binding from PostgreSQL. Through POOL it gets neither, so someone in the path can relay its login. And a client that sets `channel_binding=require` cannot connect at all, because `SCRAM-SHA-256-PLUS` is not offered. Until scope 006, clients of POOL need `sslmode=verify-full`.
- **Secrets.** They live in files with owner only access, are read once, are held in types that print `[REDACTED]`, and appear in no output, no log and no error (`SEC-4`, `SEC-11`, `T-12`). Memory is not wiped when a secret is dropped. A core dump can hold them, as the operational defaults of the security requirements already say.
- **Authority.** A node receives SQL only while the journal, the configuration and a fresh probe agree that it is the writer, checked twice per login and once more on the connection itself (`0001 SC-8`, `R-19`). (Corrected on 2026-11-16, findings M2 and d3 of the independent review.) Said exactly: a login reaches a node only while the three agree. That is checked twice in the route view and then on the connection itself, where the node must say that it is not a standby and must name the system identifier and the timeline of the journal (`0004 AC-29`). A session that already runs is not checked again on every statement. It goes on while the probe is silent for a short time. It is ended when the probe shows a positive conflict, and when the node gives no answer for `probe.node_loss_timeout_ms` (`0004 AC-30`).
- **Administration.** There is none on the network. The three commands act on local files and need the rights of the service account (`SEC-6` first applies in scope 009).
- **How old evidence can be** (added on 2026-11-16, finding c8 of the independent review). A sample is dated when its row arrives, not when the question was sent, so evidence can be older than `probe.observation_expiry_ms` by up to `probe.timeout_ms`. And the clock that measures freshness does not run while the host is suspended or a virtual machine is paused. After such a pause a sample from before it looks as fresh as it did, until the next probe replaces it or the rest of its expiry runs out, which is at most the expiry. A failover inside such a pause can let a login through to the old writer for that window. Step 8 of the backend connection narrows it: the old writer must still answer as a primary on the approved timeline. Do not suspend the host of the proxy. Scope 008 owns dating a sample at the start of its question and comparing with the wall clock (Follow-up 6).
- **Trust roots** (added on 2026-11-16, decision 88). A CA file and the certificate chain of the listener are no secrets, and they are refused when the group or others can write them, or when their owner is neither the service account nor root. An account that can write a CA file could otherwise choose which server POOL takes for the node.
- **A peer that is gone** (added on 2026-11-16, decision 84). A node that stops answering is noticed by POOL itself within `probe.node_loss_timeout_ms`. A client that is gone is noticed by the kernel, and with the kernel defaults that takes about two hours, during which it holds a backend. The values to set are under [A peer that vanished](#a-peer-that-vanished).
- **What the build links** (added on 2026-11-16, decision 87). The two native libraries are built from the locked source and never taken from the build host ([Native code](#native-code)).
- **What this does not defend.** Root on the proxy host, a superuser of PostgreSQL, and a client that holds a valid password. A valid client can hold a session and a backend for as long as it likes, up to the limits. One address that opens connections and sends nothing can hold every handshake permit for `limits.startup_timeout_ms` at a time, and so keep others from logging in. This slice has no limit per address. A firewall in front of the listener is the answer until scope 005. (Added on 2026-11-15.) It also does not defend against an operator who removes `pool.lock` under a running daemon, or who puts the state directory on a network file system. The promise that two daemons cannot share one state directory holds for a lock file that stays in place, on a local file system of one host ([State directory and protected files](#state-directory-and-protected-files)). (Added on 2026-11-16, finding a3 of the addendum to the second review.) It also does not defend against a daemon that has no log. When nothing reads its stderr the daemon serves on and writes nothing ([Log events](#log-events), decision 106). A conflict latch shows only in the log. So with stderr closed a latched cluster answers every login with `57P03`, `pool show-authority` says `ready`, and nothing tells you why. Keep stderr of the daemon with something that outlives its reader, and treat a daemon whose log has stopped as a daemon to restart. Scope 008 owns a sign of a log that is not written, and scope 009 a command that shows a latch (Follow-up 6 and 7).

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

Hermetic session tests put a pass through in the place of the TLS handshake. Real TLS on both legs is tested on the fixture.

**A golden message is not proof of a wire format** (added on 2026-11-15, after the fixture of step 11 found the wrong first number in `NegotiateProtocolVersion`). The golden bytes of `pool-protocol` were written from this spec, and the checker compared the code with a model made from this spec, so a mistake in the spec passed both. From now on, every message that POOL sends to a client, and every message it expects from a node, counts as verified only when a real `psql` 18 or a real PostgreSQL 18 has been on the other end of it in the integration check, or when its golden bytes were captured from a real server and the verification record says so. The integration tests of step 11 therefore also run, through POOL and directly, and compare: a request for protocol 3.2 (`max_protocol_version=3.2`), a `COPY ... TO STDOUT` and a `COPY ... FROM STDIN` (both must end with `0A000`), a `NOTICE` raised by a `DO` block, a `LISTEN` with a `NOTIFY`, an empty query string, and a query string with several statements. What no real client of the fixture can be made to send stays hermetic and is named as such in the verification record: a `GSSENCRequest`, a `CancelRequest`, a startup packet with another major version, and a `_pq_.` parameter.

**Two things that only the verification record holds** (added on 2026-11-15, from the open gaps of the tester). The checker checked both, by reading the code and with a probe of its own. No committed test holds them yet, and neither changes behavior.

- **The settings of the journal connection**: the four of every open that writes (`synchronous`, `foreign_keys`, `wal_autocheckpoint`, `journal_size_limit`) and the read only flag of the reader. SQLite reports them only to the connection that set them, and no caller can reach that connection. A unit test beside the code can. A `#[cfg(test)]` module in `crates/pool-control/src/journal.rs` sees the private connection of a `Journal` after `open_ready` and asks each `PRAGMA` back. That adds no public function, and `pool-control` has unit tests beside the code already, in `probe.rs` and `snapshot.rs`. For the read only flag, the open of the reader first has to become a private function of its own, so that the test can ask `Connection::is_readonly` on what it returns. That moves three lines and changes nothing they do.
- **A node certificate that has expired or is not valid yet.** No CA key has to be in the repository for that, and no second private key. A CA that is made once, and whose key is thrown away as the keys of the two test CAs were, signs two more certificates for the public key of `node.key`: one whose time has passed and one whose time starts far in the future. That is three more files under `crates/pool-core/tests/fixtures/` (that CA and the two certificates) and one test in `pool-core` that expects the class `tls` and no startup packet for each. `node.key` stays the only private key in the repository.

Follow-up 12 says who takes them.

**One test key is committed** (changed on 2026-11-15, after the verification of step 7; the first version of this paragraph said that no private key is committed). The backend connection of `pool-core` checks a certificate chain and a name, and `SEC-1` asks for a positive and a negative certificate corpus. Without a key and a certificate that check could only run with Docker. So `crates/pool-core/tests/fixtures/` holds four files: `ca.pem` and `other-ca.pem` (two test CA certificates), `node.pem` (a certificate for the name `node.test`, issued by the first CA) and `node.key` (the private key of that certificate). The rules for them:

- `node.key` is a test value and not a secret in the sense of `SEC-4`. It belongs to a certificate for a name under `.test`, which no real host can have. The private keys of the two CAs are in no repository, so nobody can issue a second certificate under them. Knowing `node.key` lets someone pose as `node.test` to a test, and nothing else.
- It is read only by code under `#[cfg(test)]`. No product code path names `tests/fixtures`. (Corrected on 2026-11-16, finding d5 of the independent review, a consequence of decision 62.) Said exactly: it is read by test code only. That is the unit tests under `#[cfg(test)]` and the integration tests in `crates/pool-core/tests/`, which use it as the key of a listener of the test. No product code path names it.
- Its first line is plain text that says what it is, before the PEM block: `test key for node.test, used only by the tests of pool-core; it protects nothing`. A person who finds the file, and a reviewer of a scanner report, reads that line first. Text outside the PEM block is allowed by the PEM format. If the reader of `rustls` refuses it, stop and report it: the line then goes into a text file beside the key.
- It is the only private key in the repository. A second one, or a key anywhere outside a `tests/fixtures/` folder, needs a decision in a spec. The fixture of the integration check still makes its keys at run time and commits none. (Added on 2026-11-16, same finding.) No check holds this rule today. Review keeps it. A guard rule that refuses a private key block in any other file is part of the tooling work of Follow-up 14, and a line in the security documents that points to this paragraph belongs to the "Document it" step of the scope.
- A secret scanner on the remote may report the file. The answer to such a report is this paragraph. Dismissing it on the remote is a step for you.

### The integration check

`scripts/check.sh integration` is a new check name, built by Build plan step 8, under the [rules of spec 0002](../0002-standards-and-tooling/index.md#rules-for-later-slices).

1. It checks `docker info`. Without a daemon it prints the reason and counts as could not run (exit 3).
2. It makes a temporary directory with mode `0700` and, in it, the certificates of the fixture, once: a test CA, a server certificate for the name `pg.pool.test` and a certificate for POOL with the name `host.docker.internal`. It makes them with `openssl` in a short run of the same image (`docker run --rm`), so the host needs no tool. A `trap` removes every container and the directory on every exit.
3. It starts two containers from `postgres:18.7-bookworm@sha256:ece55f7868da684e400f82f1030a050718adbdd65c23aa500c486f6e2950dc0f`, each with a name that starts with `pool-it-` and ends with the process ID of the script, with `-p 127.0.0.1::5432`, with `--add-host=host.docker.internal:host-gateway`, with `scripts/integration/initdb/` mounted read only at `/docker-entrypoint-initdb.d`, and with the certificates mounted read only. Both get the same server certificate, the same roles and the same passwords. They differ in one thing that PostgreSQL makes by itself: the system identifier. The second one is the wrong identity of AC-14, and because it passes TLS and the login, the probe really reaches the identity rule. (Changed on 2026-11-16, decision 103, finding g4 of the independent review.) The script keeps the ID that `docker run -d` prints for each container and removes by that ID, never by the name. So it cannot remove a container that another run made under the same name.
4. The init script, run by the image as the database owner, copies the server certificate and key into the data directory, turns on `ssl`, allows only `hostssl` with `scram-sha-256` from the network, sets `log_connections` and `log_statement = 'all'`, and creates the roles, the database and the table of the fixture.
   The script then waits until `pg_isready` answers on both, and writes the credential and verifier files into the temporary directory with mode `0600`. For the verifier it creates a role with `NOLOGIN` and the password of the principal on the first container, reads `rolpassword` from `pg_authid` with `docker exec`, and drops the role. So the principal cannot log in to PostgreSQL by any name.
5. It exports the environment below and runs `cargo test -p pool --locked --test integration -- --ignored --test-threads=1`. (Changed on 2026-11-16, decision 103.) The test command runs in the background, in a process group of its own, and the script waits for it. A watchdog ends that group after 1800 seconds and says so, so that the cleanup always runs and a test that hangs cannot hold the check for ever. (Added on 2026-11-16, decision 104.) When the variable `POOL_IT_FILTER` is set, the script passes its value on as the filter of test names, so that only the tests whose name holds that text run. It is there to run one test of the fixture again and again, which is how the fix of a race was shown ten runs in a row. A filter that matches no test still ends with exit 1 and `no test ran`. A run with the filter set is not a run of the integration check: it proves the tests it ran and nothing about the others, and a verification record that used it names the filter. The CI job never sets it. When it is set, the script says so in one line on stderr, so that a green result is never read as a full run. (Added on 2026-11-16, after the checker run of step 15.) The watchdog has a process group of its own too. The script ends that group on every way out, so nothing of a run outlives the script and nothing holds the output of the caller open.
6. It removes the containers and reports the result. (Changed on 2026-11-16, decision 103.) A run in which no test passed is a failure: the script adds up the tests that passed in the output of the test command and exits 1 with `no test ran` when the sum is 0. A filter or a build that runs nothing does not look green.

**A signal to the script alone** (added on 2026-11-15, after the verification of milestone 4). The `trap` of step 2 runs on every exit, and a shell runs a trap for a signal only after the command in the foreground has ended. So SIGTERM sent to `scripts/integration/run.sh` alone removes the containers and the directory when the test command of step 5 has returned, not before. A signal to the whole process group, which is what Ctrl C in a terminal sends, ends the test command too, and the cleanup follows at once. In both cases nothing is left behind once the script has ended. The next slice that changes the script (scope 007, for the replica) may run the test command in the background and wait for it, so that the trap runs at once (Follow-up 5). **What was built on 2026-11-16 replaces the above** (decision 103, findings g4 and t4 of the independent review). The trap is set before anything is made. The test command no longer runs in the foreground: it runs in the background in a process group of its own, and the script waits. A shell runs a trap at once while it waits, so SIGTERM, SIGINT or SIGHUP sent to `scripts/integration/run.sh` alone ends the group of the test command, removes the containers by their IDs and the temporary directory, and exits 1, without waiting for the tests. The part of Follow-up 5 that spoke of this is done in this feature. (Added on 2026-11-16, after the checker run of step 15.) The cleanup also ends the group of the watchdog, so its `sleep` does not stay behind after a signal or after a normal end.

**The host needs Docker and nothing else, also for a test** (added on 2026-11-16, decision 101, from the verification of Build plan step 15). Step 2 says the host needs no tool, and that holds for the tests too. One test broke it: `it_ac13_run_refuses_a_journal_on_hold_for_a_restore_and_never_binds` set the hold with the `sqlite3` tool of the host. The pinned image has no `sqlite3`, and `apps/pool` may not name `rusqlite` ([Source guard](#source-guard)), so that test cannot set the hold without a tool from outside. It does not need the fixture either: a journal on hold has nothing to do with PostgreSQL. So the test goes, and the hold is proven in two hermetic halves that meet at one typed value, the `AuthorityState`:

- A journal file whose `hold` is `restore_pending` classifies as that state. `ac10_a_journal_on_hold_for_a_restore_classifies_as_restore_pending` in `crates/pool-control/tests/journal.rs` holds this already.
- That state gives the finding `authority.restore_pending`. A unit test beside `not_ready` in `apps/pool/src/daemon.rs` runs all six states that are not ready against the six codes of [Findings of the new commands](#findings-of-the-new-commands).

What is given up is one run of the built binary on a held file. The path from the classified state to the refusal, and the rule that the listener is never bound, are the same code for all six states, and `apps/pool/tests/commands.rs` runs the binary on the others. The first half must be on `main` before the fixture test is removed, and the second half comes with the removal.

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
- **Memory (AC-21).** The 32 sessions are clients that the test itself runs: each logs in through POOL over TLS with the code of `pool-core`, sends one query that returns 256 rows of 1 MiB, and never reads. The test waits until PostgreSQL shows 32 active queries, then reads the resident memory of the `pool` process with `ps -o rss=` twenty times and takes the highest value. (The first version of this point used `psql` processes stopped with SIGSTOP. `psql` collects a whole result in its own memory before it shows it, so it is the wrong tool to hold a result back. What is measured is the memory of POOL, and for that the client only has to be one that does not read.)
- **Waiters and deadlines (AC-20).** Small limits in the configuration: `session_max_count = 3`, `node_backend_max_count = 1`, `backend_wait_max_count = 1`, `backend_acquire_timeout_ms = 3000`. The deadline is 3000 and not shorter, because the test has to tell a refusal that comes at once from one that comes at the deadline, and on a busy machine one second does not separate the two. The bound of the criterion does not move: not before the deadline, and not more than 2 seconds after it.
- **The probe numbers of the fixture.** The configurations of the tests use `probe.period_ms = 200`, `probe.timeout_ms = 5000` and `probe.observation_expiry_ms = 8000`. The timeout is long so that a bootstrap on a busy machine does not run out of time on its one probe of each node, and the period is short so that a test does not wait long for the first sample. They are values for tests. The defaults of [Keys](#keys) are the values for a deployment.
- **Wrong identity (AC-14).** The journal is made against the first container. The node in the configuration then points at the second one, which has the same certificate, roles and passwords and another system identifier. The test waits for the event `admission.changed` with `evidence.writer_identity` before it tries to log in.

### Fuzzing

`crates/pool-protocol/fuzz/` is a package of the same shape as `crates/pool-config/fuzz/`, with its own committed lockfile and three targets:

| Target | Feeds | Must not |
|---|---|---|
| `startup` | The first packet reader and the parameter parser | Panic, or reserve memory from a length field |
| `frontend_message` | The header reader for client and backend messages | The same |
| `scram` | The four SCRAM message parsers and the base64 decoder | The same |
| `auth` (added on 2026-11-16, decision 94) | The parsers of `auth.rs`, with `parse_sasl_initial_response` first, and `error_sqlstate`. The first version of this row also named `data_row`, which decision 102 took out | The same |

- **Four targets** (changed on 2026-11-16, decision 94, finding t10 of the independent review). `parse_sasl_initial_response` reads bytes of a client before any proof, and it was in no target. The target `auth` feeds it and the other parsers of `auth.rs`. The target `scram` took its nonce from the first bytes of the same input, so the parser of the final client message never got past its nonce check. It now takes the nonce from a part of the input of its own, so that the decoding of the proof is reached. `0004 AC-27` names three targets and stays true. The golden tests get literal bytes for the six short messages whose bytes the PostgreSQL documentation fixes, and the verifier of RFC 7677 as a literal, so that an encoder and a parser with the same mistake no longer pass each other.
- **`data_row` is not in a target** (changed on 2026-11-16, decision 102, from the tester who built the target `auth`). Decision 94 put it on the list from the finding of the review, without looking where it lives. It is a private function of `pool-core` (`crates/pool-core/src/backend.rs`) that reads the columns of one `DataRow`, and the fuzz package of `pool-protocol` cannot reach it: `pool-core` would have to become a dependency of that package, which changes its lockfile and its baseline of rule W1 and pulls the runtime and both native libraries into a fuzz build. What it reads is also not the input that fuzzing is for in this slice. The bytes come from a node, after the TLS check and after the node proved the password, and the message they are in is at most 4096 bytes long. So it gets a unit test in place of a target: in `crates/pool-core/src/backend/tests.rs`, which can call the private function, feed it every prefix of a good body, a count that is larger than the columns that follow, a column length that goes past the body, a negative length other than -1, bytes left over after the last column, and text that is not UTF-8. Each must give the protocol error and none may panic. No test fed it a body that lies before this. Scope 006 reads rows for more than one query. It moves this reader into `pool-protocol`, where a target can reach it (Follow-up 4).
- Each target has a directory `fuzz/regressions/<target>/` with committed inputs. It starts with the golden packets of the tests. An input that made a target crash is committed there with the fix.
- `scripts/check.sh fuzz` now loops over both fuzz packages and every target. For each it runs the target on its corpus, its seed directory and its regression directory for `POOL_FUZZ_SECONDS`, 30 by default. Exit 0 with no file under `fuzz/artifacts/` is a pass. (Added on 2026-11-16, decision 100.) `cargo fuzz run` of `cargo-fuzz` 0.13.2 has no `--locked`. So before it runs the targets of a package, the check runs `cargo metadata --locked` on the manifest of that package with the fuzz toolchain, and that fails when the lockfile would have to change. A fuzz run therefore never builds from a lockfile other than the committed one, which is what the nit n13 of the independent review asked for.
- `scripts/check.sh fuzz-tools` is a new name. It installs `cargo-fuzz` 0.13.2 under `target/tools` and the toolchain `nightly-2026-10-17` with `rustup`, when they are missing. It exists so that the CI job has a `run:` line that rule C3 accepts.
- Rule W1 gets a baseline file `protocol-fuzz-lock-packages.txt` for the new lockfile, rule W6 lists four lockfiles, `fmt` formats the new manifest, and `deny` checks the new lockfile, with the lint for an unused allowed license turned off for both fuzz lockfiles ([Dependency policy](#dependency-policy)).

### What proves each criterion

| Criterion | Command | Where it runs |
|---|---|---|
| AC-1 | `cargo metadata --no-deps --format-version 1`, `scripts/check.sh guards`, `cargo build --workspace --locked`, and the use check under [Adopted dependencies](#adopted-dependencies), run by hand | Workstation. The first three also in CI |
| AC-2, AC-4 | `cargo test --manifest-path tools/repo-check/Cargo.toml --locked --target-dir target/repo-check`, `scripts/prove-checks.sh` | Workstation |
| AC-3 | `scripts/check.sh deny`, `scripts/check.sh linux-build` | Workstation with Docker, and CI |
| AC-5 to AC-8 | `cargo test -p pool-config --locked`, `cargo test -p pool --locked --test check_config` | Workstation and CI |
| AC-9, AC-13 | `cargo test -p pool --locked --test commands` | Workstation and CI |
| AC-10 | `cargo test -p pool-control --locked --test journal`, and `scripts/check.sh integration` for the file that bootstrap wrote | Workstation and CI |
| AC-11, AC-12 | `scripts/check.sh integration`, with `cargo test -p pool-control --locked --test controller` for the library half | PostgreSQL 18.7 in the container |
| AC-14 to AC-17, AC-19, AC-20, AC-22, AC-23, AC-25 | `cargo test -p pool-core --locked` for the hermetic half, `scripts/check.sh integration` for the half on the fixture | Both |
| AC-29 | `cargo test -p pool-core --locked` for the six answers of the scripted node, `scripts/check.sh integration` for the address that moved | Both |
| AC-30 | `cargo test -p pool-core --locked` (the node that never answers, the idle session, silence without the mark; and `TcpSocket::keepalive()` is true on the socket to a node before it connects), `cargo test -p pool --locked` for the control loop. That an accepted socket has keepalive on is shown on Linux by the checker, by the keepalive timer that is pending for each socket in `/proc/net/tcp` (the value 2), read in the container in which the Linux build of `pool` runs, with a socket without keepalive as the control, and recorded in the verification record (corrected on 2026-11-16, decision 108: the first version of this cell named `ss -tno`, and no pinned image has `ss`) | Workstation and CI, and Linux for the accepted socket |
| AC-18, AC-21, AC-24 | `scripts/check.sh integration` | PostgreSQL 18.7 in the container |
| AC-26 | `scripts/check.sh integration`, once with Docker and once with `DOCKER_HOST` pointing at nothing | Workstation |
| AC-27 | `scripts/check.sh fuzz` | Workstation, and CI after manual step M2 |
| AC-28 | `cargo test -p pool-protocol --locked` | Workstation and CI |
| `0003 AC-17`, `0003 AC-18` | `cargo test -p pool-control --locked` | Workstation and CI |
| `0003 AC-19` | `scripts/check.sh guards`, and its case in `scripts/prove-checks.sh` | Workstation and CI |

For the node half of **AC-15**, read "gives the client `08006`" with the paragraph under [Backend connection](#backend-connection) that says where `08006` and where `57P03` is the answer.

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

The exceptions in `deny.toml` name exactly the versions of this table. A later version of one of these packages needs the look described under [Dependency policy](#dependency-policy) before it can be merged.

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
3. **Workspace shape and dependencies.** Needs manual step M1. Create `crates/pool-protocol` and `crates/pool-core` with a manifest and a `src/lib.rs` that holds only the crate documentation. Add the six dependencies and the two members to the root `Cargo.toml`, and the dependencies of each member as the table under [Workspace](#workspace) says. Run `cargo build --workspace` once so that `Cargo.lock` gains the packages, and compare the names with the [rationale](rationale.md#locked-packages). Change `lock-packages.txt`, `forbidden-paths.txt` and `internal-dependencies.txt`, and point case 9 of `scripts/prove-checks.sh` at `apps/pool-cli`. Add the five exceptions to `deny.toml`, each with its exact version. (The versions were added to this spec on 2026-11-15, after step 3 was built and verified. The tree of that time names the packages without a version, and gets the five changed lines as a commit of its own, named for this step, before step 12.) Write `NOTICE`. Record the adoption in the technology evaluation. Set this spec's `**Status**` line to `In Progress`. Files: `Cargo.toml`, `Cargo.lock`, the five manifests, the two new `src/lib.rs`, `tools/repo-check/baseline/`, `scripts/prove-checks.sh`, `deny.toml`, `NOTICE`, `docs/research/technology-evaluation.md`. Evidence: `scripts/check.sh all`, `scripts/check.sh linux-build`, and `scripts/prove-checks.sh` after the commit. Satisfies **AC-3** and the shape of **AC-1**.
4. **The wire format.** Build `pool-protocol` as [The wire subset](#the-wire-subset) says: first packets, parameters, message headers, the error message, the authentication messages, `NegotiateProtocolVersion`, the SCRAM text messages and base64, each as a function over bytes. Write the golden tests. Needs step 3. Files: `crates/pool-protocol/`. Evidence: `cargo test -p pool-protocol --locked`. Satisfies **AC-28**, and the byte level half of **AC-15**, **AC-16** and **AC-23**. (Added on 2026-11-15, after this step was built and its milestone verified: three changes to the built crate, as a commit of their own named for this step, before step 9. `encode_negotiate_protocol_version` writes the whole protocol code, 196608 for 3.0, and the parser for scripted clients reads it the same way. `classify_frontend` refuses a `Terminate` whose length is not 4 with `08P01`. `parse_client_first` no longer checks the escapes of the user name. `FirstPacketReader` keeps the `Handshake` and asks it after the 8 header bytes. Each with its tests.)
5. **Configuration.** Implement [Configuration](#configuration) in `pool-config`: the keys, the two codes, the name rule, the finding limit, the two views, the two generation types and the fixtures, with the one changed line in `invalid-unknown-key.toml` and in the two tests that [Fixtures](#fixtures) names. Wrap the output of the binary in a `BufWriter` and add the omitted line and field to `apps/pool/src/report.rs`. Files: `crates/pool-config/`, `apps/pool/src/report.rs`, `apps/pool/src/main.rs`, `apps/pool/tests/check_config.rs`. Evidence: `cargo test -p pool-config --locked`, `cargo test -p pool --locked`. Satisfies **AC-5** to **AC-8**.
6. **Authority on disk and the control loop.** In `pool-control`: `journal.rs` with `JournalRead`, `AuthorityState::latch_conflict`, `latch_recorded` and `name`, `ObservationBoard`, `SnapshotGeneration`, `build_snapshot`, `Controller`, `PROBE_QUERY` and `observation_from_row`. Bring the interface block and the comments in line with the notes of 2026-11-15 in spec 0003. (Added on 2026-11-15, after this step was built: the method `JournalRead::plan_bootstrap` is renamed to `bootstrap_plan`, in `crates/pool-control/src/journal.rs` and in the tests that call it, as a commit of its own named for this step. The same commit brings the rule for an empty database into `Journal::read` and `Journal::create`, and the checkpoint at the end of `create`. `AuthorityState::latch_recorded` takes a revision only when it is higher, in `crates/pool-control/src/authority.rs`.) Needs steps 3 and 5. Files: `crates/pool-control/`. Evidence: `cargo test -p pool-control --locked`. Satisfies `0003 AC-17`, `0003 AC-18`, the library half of **AC-10** and **AC-12**.
7. **Secrets, TLS and the backend connection.** In `pool-core`: the secret types and the reading of protected files, the TLS server and client setup, SCRAM as server and as client, the backend connection of [Backend connection](#backend-connection) and the one row query, and `random.rs` with `fill_random`. (Added on 2026-11-15, after this step was built and verified, as commits of their own named for this step: the last report of `in_hot_standby` decides; `Debug` by hand on the SCRAM exchange types; one derivation at a time per credential, and a count type that cannot be 0; protected files checked on the open file; the text line at the top of `node.key`.) Needs steps 4 and 5. Files: `crates/pool-core/`. Evidence: `cargo test -p pool-core --locked`. Satisfies the secret half of **AC-9**, and the library half of **AC-15** and **AC-16**.
8. **Bootstrap, the authority command and the first run against PostgreSQL.** In `apps/pool`: the state directory checks and the lock, `pool bootstrap`, `pool show-authority`, and the help text. Add `integration` to `scripts/check.sh` with `scripts/integration/`, and write the first tests of `apps/pool/tests/integration.rs`: bootstrap, a second bootstrap, `show-authority`, a node that does not answer, the monitor login. Write `apps/pool/tests/commands.rs` for the two commands. Record the image in the technology evaluation. Needs steps 6 and 7. Files: `apps/pool/`, `scripts/check.sh`, `scripts/integration/`, `docs/research/technology-evaluation.md`. Evidence: `cargo test -p pool --locked`, `scripts/check.sh integration`. Satisfies **AC-9**, **AC-11**, **AC-12**, **AC-26**, and completes **AC-10**.
9. **The session.** In `pool-core`: the listener loop, steps 1 to 12 of [Session lifecycle](#session-lifecycle), the limits, the relay, backend loss, the conflict ending, shutdown, the route view, the lease and the pool key, and the log events of the data plane. Write `crates/pool-core/tests/session.rs`. Needs step 7. Files: `crates/pool-core/`. Evidence: `cargo test -p pool-core --locked`. Satisfies the hermetic half of **AC-14** to **AC-17**, **AC-19**, **AC-20**, **AC-22**, **AC-23** and **AC-25**.
10. **The daemon.** In `apps/pool`: `pool run` with its ten steps, the log writer, the probe tasks, the control loop, the mapping into the route view, and signals. Extend `integration.rs` with the walking fixture: the same SQL file direct and through POOL, the mapped login, teardown, the states that are not ready (in `commands.rs`), redaction over the whole run. Needs steps 8 and 9. Files: `apps/pool/`. Evidence: `scripts/check.sh integration`, `cargo build --workspace --locked` without a dependency warning, and the use check of **AC-1** with no output. Satisfies **AC-13**, **AC-18**, **AC-19**, **AC-24**, **AC-25**, and completes **AC-1** and **AC-14**.
11. **Limits, loss and denial on the fixture.** Extend `integration.rs` and its helpers with the relay and the cases of [The integration check](#the-integration-check): every limit, the memory budget, the lost COMMIT, plain text, a wrong CA, a wrong password, an unknown user, an unmapped database, the wrong identity. Needs step 10. Files: `apps/pool/tests/`. Evidence: `scripts/check.sh integration`. Completes **AC-15**, **AC-16**, **AC-17**, **AC-20**, **AC-21** and **AC-22**.
12. **Fuzzing and the CI rule.** Create `crates/pool-protocol/fuzz/` with its three targets, lockfile and regression inputs. Extend `fuzz` and add `fuzz-tools` in `scripts/check.sh`, the third `deny` command and the flag on both fuzz lockfile commands as [Dependency policy](#dependency-policy) writes them, the new baseline file, rule W6, and `fuzz-tools` in rule C3, each with a test. Needs steps 4 and 8. Files: `crates/pool-protocol/fuzz/`, `crates/pool-config/fuzz/regressions/`, `scripts/check.sh`, `tools/repo-check/`. Evidence: `scripts/check.sh fuzz`, `scripts/check.sh all`, `scripts/prove-checks.sh`. Satisfies **AC-27**. Manual step M2 follows it.
13. **Document corrections.** Apply items 1 and 2 of [Document corrections](#document-corrections). Evidence: `scripts/check.sh docs`.
14. **Evidence.** On the final tree run `scripts/check.sh all`, `scripts/check.sh integration`, `scripts/check.sh fuzz`, `scripts/prove-checks.sh` (with `--with-linux-build` when `docker info` succeeds) and the commands of **AC-1**. Record commands, results, the revision, the image digest and the locked versions in `docs/specs/0004-one-real-primary-path/verify.md`, one section per milestone, and add that file to `docs/index.md`. A step that needs Docker and cannot run is recorded as `BLOCKED` with `owner: user`. The CI runs of `integration` and `fuzz` are recorded as `NOT_RUN` until they exist. Evidence for **AC-1** to **AC-28**.

15. **What the independent review asked for** (added on 2026-11-16). The [review](../../reviews/2026-11-16-scope-004-one-real-primary-path.md) found no blocker and three major points. This step builds what the triage in the [rationale](rationale.md#triage-of-the-independent-review) marks as now. Each part is a commit of its own that names this step and the finding.
    - **M2, `0004 AC-29`.** `crates/pool-control/src/admission.rs`: the accessors `system_identifier()` and `writer_timeline()` of `EligiblePrimary`. `crates/pool-core/src/route.rs`: the two fields of `Route`. `apps/pool/src/control.rs`: fill them in `route_view`. `crates/pool-core/src/backend.rs` and `session.rs`: step 8 of the backend connection, the class `identity`, the query text handed in by `apps/pool/src/daemon.rs`. Tests in `crates/pool-core/tests/session.rs` and `apps/pool/tests/integration.rs`.
    - **M1, `0004 AC-30`.** `crates/pool-config/`: the key `probe.node_loss_timeout_ms`, its default, its range, its fixtures and tests. `apps/pool/src/probe.rs` and `control.rs`: the class beside each sample, the time per node, the flag `lost`, the event `node.lost`. `crates/pool-core/src/route.rs` and `session.rs`: the flag, the ending, the reason `node_lost`. `apps/pool/src/daemon.rs` and `crates/pool-core/src/backend.rs`: sockets made with `TcpSocket` and keepalive on. Tests as the criterion lists them.
    - **The daemon and its tasks** (c1, c2, c9, d3, n1). `crates/pool-core/src/session.rs`: the read of the status byte raced like the others, the last limit of 2 seconds, the backend closed before the client. `apps/pool/src/daemon.rs` and `log.rs`: `runtime.failed`, the watch on the tasks with `daemon.failed` and exit 3, the panic hook with `daemon.panicked`. `apps/pool/src/control.rs`: the last try for a latch at shutdown. `apps/pool/src/commands.rs`: `runtime.failed` for `bootstrap`.
    - **The startup parser** (c3). `crates/pool-protocol/src/startup.rs`: the check for a repeated parameter keeps the names it has seen in a set, so its cost grows with the packet and not with its square. No answer of the tables under [Startup](#startup) changes.
    - **Files and build** (c4, c6, g2, g5, n5, n13). `.cargo/config.toml` as [Native code](#native-code) writes it, and the version test in `crates/pool-control/src/journal.rs`. `crates/pool-core/src/secret.rs`: the rule for a CA file and a certificate chain. `tools/repo-check/baseline/forbidden-source-patterns.txt` as [Source guard](#source-guard) writes it, with one case in `scripts/prove-checks.sh`. `NOTICE`: the version of AWS-LC. The two comments in `crates/pool-control/src/lib.rs` and `observation.rs` that still say `JournalRead::plan_bootstrap`. `scripts/check.sh`: the fuzz lockfile checked with `cargo metadata --locked` before `cargo fuzz run`, which has no `--locked` of its own in `cargo-fuzz` 0.13.2 (corrected on 2026-11-16, decision 100; the first version of this line asked for the flag on `cargo fuzz run`), and `POOL_FUZZ_SECONDS=0` refused.
    - **Tests** (M3, t1 to t10, g4, n15), by the tester, as the triage lists them, and the target `auth` of [Fuzzing](#fuzzing).
    - **The second review** (added on 2026-11-16, the [triage](rationale.md#triage-of-the-second-review)). `scripts/integration/run.sh`: the script must work under `/bin/sh` of Linux with no terminal (r1), and a container that was created and did not start is removed (a nit). `crates/pool-config/`: the cross rule of `probe.node_loss_timeout_ms`, its fixture and its tests (r2, decision 109). The tests of Follow-up 16. `crates/pool-protocol/src/startup.rs`: a `database` with an empty value takes the value of `user` (decision 116), with a row in the parameter tests and one hermetic login in `crates/pool-core/tests/session.rs`.
    - Evidence: `scripts/check.sh all`, `scripts/check.sh integration`, `scripts/check.sh fuzz` and `scripts/prove-checks.sh` on the final tree, and a verification of this step in `verify.md`.

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
- A conflict latch shows in the log of the daemon and in no command. `pool show-authority` says `ready` for a journal that holds one. (Added on 2026-11-15.)
- The lock on the state directory holds on one host and for a lock file that stays in place. A network file system, or an operator who removes `pool.lock` under a running daemon, is outside it, and nothing detects either. (Added on 2026-11-15.)
- While a latch cannot be written to the journal, the log gets one `journal.write_failed` line per round. (Added on 2026-11-15.)
- A client whose backend was ended by PostgreSQL with a message gets two errors: the one of PostgreSQL, then the one of POOL. (Added on 2026-11-15.)
- Every login asks the node one more question before the client is let in. That is one more round trip, and one more line in the log of a node that logs all statements. (Added on 2026-11-16.)
- A node whose probe gets no answer for `probe.node_loss_timeout_ms` loses its running sessions, also when those sessions could still reach it. A network that cuts the proxy off from a node for longer than that time cuts the work with it. (Added on 2026-11-16.) (Added on 2026-11-16, finding r2 of the second review.) The class `timeout` counts as no answer. So a primary that can be reached and answers the probe slower than `probe.timeout_ms` for the whole loss time loses its sessions too, most of them with `08007`. A host name of a node that cannot be looked up for that long counts as `connect` and does the same. Keep `probe.timeout_ms` well above the slowest answer you still call healthy, and raise the loss time if your primary stalls for longer than 30 seconds under load.
- A client that is gone is noticed only as fast as the kernel is set to notice it. POOL sets no keepalive time of its own in this slice. (Added on 2026-11-16.)
- A daemon ends when one of its own tasks ends. That is a restart by the service manager where the first build would have gone on half alive. (Added on 2026-11-16.)
- A daemon whose stderr has no reader serves on without a log, and nothing in the process says so. (Added on 2026-11-16.)
- A CA file or a certificate chain that the group or others can write, or that belongs to another account than the service account or root, is refused at start. A deployment that kept such a file must change its mode. (Added on 2026-11-16.)
- The repository holds a `.cargo/config.toml`. A developer who wants the SQLite or the AWS-LC of the host for a local experiment has to change that file, and must not commit the change. (Added on 2026-11-16.)
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
3. Scope 005 adds reuse and with it: idle and lifetime limits, a deadline for a client that reads slowly, the trigger that advances a credential generation, and the startup profiles that cleanup can restore. Added on 2026-11-16, from the independent review: the derivation of the backend keys as a task that owns the kept keys, so that a login that is cut does not start a second one (c5); the identity question of `0004 AC-29` once per backend; and the pool key and the configuration generation of step 9 driven from real state (n2).
4. Scope 006 adds protocol 3.2, the extended protocol, COPY, virtual cancellation credentials, `SCRAM-SHA-256-PLUS`, client certificates, SASLprep for passwords outside ASCII, noticing a client that left during a statement, and the first metrics. Added on 2026-11-15: it may also decide whether POOL still sends an error of its own after a `FATAL` of the backend ([Backend loss](#backend-loss)). Added on 2026-11-16, from the independent review: a check at start for verifiers of different salt lengths or counts and a key for the made up salt that outlives a restart (c7), secret types without a derived `PartialEq` (n3), and private fields for `Header` (n4). Added on 2026-11-16, decision 102: the reader of a `DataRow` moves from `pool-core` into `pool-protocol` and into a fuzz target.
5. Scope 007 qualifies the probe on a real replica, including what the timeline column reports during and after a promotion, and whether the monitor login gets the `pg_read_all_stats` role. Added on 2026-11-15: when it changes `scripts/integration/run.sh` for the replica, it can make a signal to the script alone clean up at once ([The integration check](#the-integration-check)). Added on 2026-11-16: the standby cases that the fixture does not show (t9), and the superuser of the fixture no longer reachable over the published port (n13). (Added on 2026-11-16, decision 103: the signal to the script alone is done in this feature. It no longer waits for scope 007.) Added on 2026-11-16, a nit of the second review: the probe query with every name qualified by `pg_catalog`.
6. Scope 008 owns the bound on the cost of a control round for very large files, a key for the log level, the probe history and drain. Added on 2026-11-15: it also owns writing `journal.write_failed` once per change and not once per round ([The journal on disk](#the-journal-on-disk)). Added on 2026-11-16, from the independent review: a sample dated at the start of its question and a comparison with the wall clock (c8), a count per cluster in the route view in place of the conflict level (n8), one list of latches that are not recorded in place of two (n7), a bootstrap that does not wait for a lookup that still runs (n10), a thread of its own for the latch write (c5), and the invisible characters U+00AD, U+200B, U+2060 and U+FEFF in a name (n11). Added on 2026-11-16, decision 106: it also counts the log writes that failed and shows a log that is not being written, through a way that is not the log. Added on 2026-11-16, two nits of the second review: the event `daemon.panicked` gets the file and the line of the source where the panic was, which are constants of the build and hold no value of a run; and a latch write gets a limit, at shutdown and inside a round. Added on 2026-11-16, a question from the addendum to the second review, which nobody ran: when the reader of stderr stalls and does not go away, does the write then hold one of the four blocking threads of the runtime for good?
7. Scope 009 owns reload, clearing a conflict latch, and reading the authority state while the daemon runs. Added on 2026-11-15: it also owns a command that shows a conflict latch, which an operator needs before the one that clears it ([pool show-authority](#pool-show-authority)). Added on 2026-11-16: `quick_check` once and no journal mode set before the checks in `open_ready` (n6). Added on 2026-11-16, decision 107: the operation that approves a new writer, in scope 009 and scope 010, has to clear or carry a latch that was set because the configuration expected a primary that the journal had not approved. Added on 2026-11-16, decision 114: a code of its own for a file of the state directory that could not be created, apart from `state.permissions`.
8. Running as root is not detected. If it ever matters, the check needs the effective user of the process, which needs a small dependency or a call this slice avoids.
9. Follow-up 13 of spec 0003 (`PRD-006 AC-2`) is still yours to decide. This spec does not touch it.
10. Manual steps M1 to M4 under [CI definition and manual steps](#ci-definition-and-manual-steps).
11. The lock covers one host and a lock file that stays in place ([State directory and protected files](#state-directory-and-protected-files)). A state directory on a network file system, and a `pool.lock` that was removed under a running daemon, were not tested and are not supported. Scope 009 may let the daemon notice that the file at the path is no longer the file it holds. Two daemons on purpose are scope 012. (Added on 2026-11-15.)
12. Two tests close gaps that the tester named, and neither changes behavior ([Tests and who owns them](#tests-and-who-owns-them)): the settings of the journal connection, in a unit test in `crates/pool-control/src/journal.rs`, and the two certificates with a wrong time, as fixtures of `pool-core` with one test. They do not hold up this feature, because the verification record carries both as checked. The lead may give them to the tester of this feature. If not, scope 009 takes the first, with the first real write load on the journal, and scope 006 takes the second, with client certificates. (Added on 2026-11-15.)
13. Keepalive times of POOL's own (added on 2026-11-16, decision 84). This slice turns keepalive on and leaves the times to the kernel. Keys for the idle time, the interval and the count, and a limit on unanswered bytes (`TCP_USER_TIMEOUT`), need a socket crate as a direct dependency. `socket2` is in the lockfile already, under `tokio`, with the license `MIT OR Apache-2.0`, so it would add no lock entry and no license exception. It is still a dependency decision, and it is yours. Scope 005 owns it, together with the idle limit and the deadline for a slow reader.
14. Tooling work, as the first step of scope 005 (added on 2026-11-16, findings g1, g3, g5 and d5 of the independent review): a `clippy.toml` per crate with `disallowed-methods` and `disallowed-types` for the paths of the standard library that rule W3 only matches as text; rule W5 read from the dependency lists of `Cargo.lock`, with the pair of `pool-core` and `pool-control` refused in code in both directions, and a report for a `deny` line that an `allow` line covers; a rule that refuses a private key block outside `crates/pool-core/tests/fixtures/node.key`; a rule that holds the six lines of `.cargo/config.toml` (corrected on 2026-11-16: the first version said two lines, which was the block before decision 99); a rule for the CI file that refuses `POOL_IT_FILTER` there (added on 2026-11-16); and a rule that compares the versions in `NOTICE` with the lockfile, with its proof cases. Each changes a rule of spec 0002, so each needs a dated note there. The notices of the Rust crates whose terms ask for a notice in a binary are due before the first binary, which is scope 011.
15. Status texts that are behind (finding d4 of the independent review): the scope index, the root `AGENTS.md`, `CLAUDE.md` and `apps/pool/AGENTS.md` still say that no proxy behavior exists, and `docs/research/technology-evaluation.md` has a stale sentence on unused dependencies (n14). No session of the architect may change these files. They belong to the "Document it" step of the scope. (Added on 2026-11-16.) Added on 2026-11-16, findings r4 and r5 of the second review, three more for that step: `docs/research/technology-evaluation.md` still says "AWS-LC 0.47.0 through `aws-lc-sys`" in the row of `aws-lc-rs`, where the right words are "AWS-LC 5.12.0, as bundled in `aws-lc-sys` 0.47.0"; the security documents have no line that points to the rule of the committed test key under [Tests and who owns them](#tests-and-who-owns-them); and the table of criteria in `verify.md`, in the part of Build plan step 14, still names the hold `restore_pending` among the states that `integration.rs` proves, where decision 101 moved it to `crates/pool-control/tests/journal.rs` and a unit test of `not_ready`. That part of `verify.md` belongs to the session that wrote the evidence of step 14.
16. What the second review found without an owner (added on 2026-11-16, findings r3 and r5). The tester of this feature works on these now. What is still open when the feature closes goes to the first step of scope 005, beside the tooling work of Follow-up 14, and the lead writes the list of what is left into the scope.
    - Nine wrong changes to the new code pass every test. Each fails closed or was shown to work by a probe, so none is a fault of the product, and each needs a test: the tasks that are left after the last limit of shutdown are dropped; the write of the greeting is raced against a cut; the question of step 8 runs under the deadline; the route carries the approved timeline, shown with a timeline above 1; the status after the answer of step 8 must be `I`; the last try for a latch runs when the loop stops through `run`; a task that ends makes the daemon end at once and not through a slow shutdown; a probe task that ends is noticed; and the listener socket has keepalive on before it listens.
    - `Debug` tests with an exact text for `Lease`, `ServerParts` and `Server` (the rest of t2).
    - The two holes of n15: a test that takes a `Terminate` alone where the backend must be closed, and a pattern that reads the start sequences 10 to 19 as 1.
    - The second sentence of `0004 AC-28` gets no committed test, by decision 112.
    - The first review spoke of seven behaviors of the code without spec text and named two. Those two are written. The second review names one more, the listener socket, which is written now and may be one of the five that were left. The rest are still not named. The lead asks the reviewer for them, and each then gets a sentence here or a change in the code. (Closed on 2026-11-16.) The reviewer named them. Four had text already: the runtime that cannot be built in `pool run` and in `pool bootstrap`, the link at the place of the state directory, and the signal handler that cannot be installed. The other three and one more are written now, under decisions 114 to 116: the empty journal file that cannot be created, the order of the first refusals, the empty `database`, and the empty `user`.

## Rationale

Reasoning, alternatives, the decisions taken, the locked packages and the cross check: [rationale.md](rationale.md).
