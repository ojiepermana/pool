# crates/pool-core

## Overview

The library of the data plane: TLS as a server and as a client, frontend authentication with SCRAM, the connection to a node, the session of one client, and the route view that says where a session may go. It depends on `pool-config` and `pool-protocol` and never on `pool-control`. The daemon in `apps/pool` maps a topology snapshot of the control plane into the route view. The contracts are [session lifecycle](../../docs/specs/0004-one-real-primary-path/index.md#session-lifecycle) and [frontend authentication](../../docs/specs/0004-one-real-primary-path/index.md#frontend-authentication) in spec 0004.

## Key files

| File | Owns |
|---|---|
| `src/session.rs` | `Server`: one task for each client, from the first packet to the end of the session, with the limits and the shutdown |
| `src/backend.rs` | The connection to a node, for a session and for the probe: TLS, startup, SCRAM as a client, and the check that the node is the server the journal approved |
| `src/route.rs` | The route view, published by the control loop over a watch channel |
| `src/scram.rs` | SCRAM-SHA-256 as a server and as a client |
| `src/tls.rs` | The rustls setup for the listener and toward a node |
| `src/secret.rs` | Reading secret files, certificate files and CA files, with the check of type, mode and owner, and the types that hold their content |
| `src/load.rs` | Reads every file that the configuration names |
| `src/random.rs` | The only source of random bytes, with no fallback |
| `src/records.rs` | The records of a session: `VerifiedPrincipal`, `PoolKey`, `Lease` |
| `src/report.rs` | The log events of the data plane |
| `tests/session.rs` | The whole session with a scripted client and a scripted node over an in memory stream, with no socket |
| `src/backend/tests.rs` | The backend connection against a scripted node |
| `tests/fixtures/` | Two test CA certificates, one node certificate and one test key |

## Clean session reuse

[Spec 0005](../../docs/specs/0005-clean-session-reuse/index.md) owns the current pool contract. `src/pool.rs` owns physical permits, runtime generations and the FIFO; `src/reuse.rs` owns bounded cleanup and baseline validation; `src/recognizer.rs` decides conservative reuse eligibility. The dedicated close rule below applies to sessions outside that profile. `Server::rotate_credentials` publishes a complete snapshot atomically; `Server::maintain_idle` is also the production maintenance tick. Returned physical connections keep their original permits through cleanup and idle.

## Commands

Run only this package's tests:

```bash
cargo test -p pool-core --locked
```

## Conventions

- A backend is pinned to one client for the whole session. At the end it enters bounded cleanup only for the proven reusable profile of spec 0005; dedicated or uncertain state closes it. Nothing is sent again after a loss. When the answer to a statement in flight is lost, the client gets `08007` and the session ends (`0001 SC-6`, `0004 AC-22`).
- No byte of application SQL reaches a node before the client proved its password, the identity map gave an entry, both looks at the route passed and the node answered the identity question (`0004 AC-14`, `0004 AC-29`).
- A secret file must be a regular file with no group or other permission bit, owned by the owner of the state directory. A certificate chain or a CA file must not be writable by the group or by others, and its owner is that account or root. The caller passes the owner in ([state directory and protected files](../../docs/specs/0004-one-real-primary-path/index.md#state-directory-and-protected-files)).
- Every type that holds a secret or content of a peer has a `Debug` written by hand that prints `[REDACTED]` or no content. An error from PostgreSQL while a backend is opened is never passed on to the client (`SEC-11`).
- `aws_lc_rs` may be named only in this crate, and random bytes come only from `random.rs` (rule W3, [source guard](../../docs/specs/0004-one-real-primary-path/index.md#source-guard)).
- The crate has no `dev-dependencies` and no build script. The hermetic tests use streams of `tokio` and no socket.

## Gotchas

- `tests/fixtures/node.key` is the one private key in the repository. It protects nothing. A second key needs a decision in a spec ([tests and who owns them](../../docs/specs/0004-one-real-primary-path/index.md#tests-and-who-owns-them)).
- `git archive` gives a committed file the mode 0664 in the Linux build image, and the CA rule refuses that mode. A test that passes a CA file to a command copies it first with mode 0644.
- `tracing` caches whether a call site has a subscriber. A test that reads log events sets a subscriber in every test of its group, or events of a later test are lost.
- The forced task drop at the last shutdown limit is tested in `src/session.rs`. Safe reservation callbacks in `pool-protocol` test the body reservation site for rejected lengths (`0005 AC-9`); they do not observe every process allocation.

## Related specs

- [Spec 0004, session lifecycle](../../docs/specs/0004-one-real-primary-path/index.md#session-lifecycle), [backend connection](../../docs/specs/0004-one-real-primary-path/index.md#backend-connection), [records built in this slice](../../docs/specs/0004-one-real-primary-path/index.md#records-built-in-this-slice) and [tests and who owns them](../../docs/specs/0004-one-real-primary-path/index.md#tests-and-who-owns-them)

_Drafted by /sync from the introducing change, worth a quick human pass._
