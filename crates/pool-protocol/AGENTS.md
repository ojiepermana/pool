# crates/pool-protocol

## Overview

The library that holds the wire format of this build: the first packets of a connection, the header of every later message, the authentication messages and the text of the SCRAM messages. It parses and encodes bytes and does nothing else. It has no dependency, opens no socket and does no cryptography. The contract is [the wire subset](../../docs/specs/0004-one-real-primary-path/index.md#the-wire-subset) in spec 0004.

## Key files

| File | Owns |
|---|---|
| `src/lib.rs` | The six public modules and `Malformed`, the one error of a parser |
| `src/startup.rs` | The first packets, which carry no tag: `FirstPacketReader`, `decode_first_header`, `Handshake`, `StartupRequest` |
| `src/message.rs` | The tagged messages after login: `HeaderReader` reads the header of five bytes, `MessageReader` reads a whole message under a limit |
| `src/auth.rs` | The authentication and backend startup messages, with their parsers and encoders |
| `src/scram.rs` | The text of the four SCRAM messages, with no cryptography |
| `src/base64.rs` | Strict standard base64 with padding |
| `src/sqlstate.rs` | The errors that POOL itself sends, with constant text |
| `tests/golden.rs` | Golden packets and messages, whole and split at every byte (`0004 AC-28`), and literal bytes for the short messages and for the exchange of RFC 7677 |
| `tests/redaction.rs` | `Debug` of every type that holds content prints no content (`0004 AC-24`) |
| `fuzz/` | The package `pool-protocol-fuzz`, outside the workspace, with the targets `startup`, `frontend_message`, `scram` and `auth` |

The private `feed_with_reserve` seams in `startup.rs` and `message.rs` are shared by production and safe allocation regression tests. Rejected lengths must never reach that reservation callback. No unsafe global allocator is used ([scope 005 evidence](../../docs/specs/0005-clean-session-reuse/verify.md)).

## Commands

Run only this package's tests:

```bash
cargo test -p pool-protocol --locked
```

Run the fuzz targets of both fuzz packages (needs `scripts/check.sh fuzz-tools` once):

```bash
scripts/check.sh fuzz
```

## Conventions

- The crate has no dependency, no `dev-dependencies` and no build script. `pool-core` and the probe use it. It uses no other POOL crate (`0004 AC-1`).
- A parser checks a length field against its limit before it reserves or reads anything. A message above the limit is refused, and the same bytes give the same result when they arrive in two pieces (`0004 AC-28`).
- Every type that holds bytes of a client or of a node has a `Debug` written by hand that prints no content. A derived `Debug` on such a type is a defect (`SEC-11`).
- The text of an error that POOL sends is a constant in `sqlstate.rs`. It never holds a name, an address or text from PostgreSQL ([SQLSTATEs that POOL generates](../../docs/specs/0004-one-real-primary-path/index.md#sqlstates-that-pool-generates)).
- The order of the refusals of a first packet is length, limit, code, then TLS. A `database` that is sent empty takes the value of `user`.

## Gotchas

- `fuzz/` has its own lockfile and its own `[workspace]` table. A change to its packages also changes `tools/repo-check/baseline/protocol-fuzz-lock-packages.txt`.
- An input that made a target crash is committed under `fuzz/regressions/<target>/` with the fix. The first byte of a `scram` input is the length of its nonce.
- `data_row` is a private function of `pool-core`, so no target here reaches it. It has a unit test there.

## Related specs

- [Spec 0004, the wire subset](../../docs/specs/0004-one-real-primary-path/index.md#the-wire-subset), [fuzzing](../../docs/specs/0004-one-real-primary-path/index.md#fuzzing) and [tests and who owns them](../../docs/specs/0004-one-real-primary-path/index.md#tests-and-who-owns-them)

_Drafted by /sync from the introducing change, worth a quick human pass._
