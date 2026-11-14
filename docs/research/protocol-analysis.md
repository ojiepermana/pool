# PostgreSQL protocol analysis

**Evidence date:** 2026-11-14. PostgreSQL 18 is the target server major. The official documentation reported 18.7 in its current release banner during research; pin the actual package and image digest when the lab is built. [Protocol reference](https://www.postgresql.org/docs/18/protocol.html).

## Compatibility contract

The first production profile should support protocol 3.0 and 3.2, with explicit negotiation tests. Version 3.1 is reserved. Version 2 is outside scope. In 3.2, cancellation keys are variable length; the protocol allows up to 256 bytes. Hardcoding the historical four byte key is incorrect. [Versions](https://www.postgresql.org/docs/18/protocol-overview.html), [message formats](https://www.postgresql.org/docs/18/protocol-message-formats.html).

| Operation | Required behavior and proposed test |
|---|---|
| Startup, negotiation | Validate untyped initial packet, NUL terminated parameter pairs, limits, supported minor version, unknown options, duplicate identity keys |
| SSLRequest, GSSENCRequest | Negotiate or explicitly refuse supported transport choices, apply deadlines, and avoid consuming untrusted extra plaintext into a TLS stream |
| Direct TLS | A separate compatibility item with PostgreSQL ALPN handling; do not advertise it until tested |
| Authentication | Keep frontend and backend exchanges distinct; fail unsupported methods explicitly; enforce message order and bounded challenges |
| Simple Query | Preserve complete message contents, statement order, all result sets, errors and final transaction status |
| Extended Query | Model Parse, Bind, Describe, Execute, Close, Sync, Flush, portal suspension and error recovery; Flush is not Sync |
| Pipelining | Associate each operation with the same lease and backend until synchronization; bound queued bytes without requiring a whole pipeline in memory |
| COPY | Stream bounded chunks for IN and OUT; preserve failure/termination states. COPY BOTH is a dedicated replication profile, not ordinary pool traffic |
| Asynchronous traffic | Forward notices, parameter status and notifications without waiting for a frontend request |
| Cancellation | Session manager owns virtual per lease frontend keys, keeps real backend keys private, serializes dispatch with retirement and closes the physical backend after forwarded cancellation; no cross lease effect |
| Errors | Preserve server SQLSTATE and severity; generated errors use documented SQLSTATEs and omit credentials and internal query text |
| Termination | Frontend Terminate ends frontend ownership; backend socket is retained only after validated cleanup, otherwise closed |

This table is a proposed POOL test contract, not a claim of existing implementation. [Message flow](https://www.postgresql.org/docs/18/protocol-flow.html), [error fields](https://www.postgresql.org/docs/18/protocol-error-fields.html).

## Session state inventory

You need to preserve database and login role, session authorization and effective role, startup options, encoding, DateStyle, TimeZone, search_path, GUCs including extension GUCs, transaction isolation and read mode, failed transaction state, savepoints, named and unnamed prepared statements, result format codes, parameter type OIDs, portals and suspended results, temporary objects, advisory locks, sequence observations, LISTEN registrations, open cursors and holdable cursors, large object descriptors, COPY state, pending results, asynchronous messages, and cancellation identity.

Session pinning preserves these on one backend. It does not make them safe to copy between backends. Unknown extension state may survive a nominal reset. The default cleanup policy should close uncertain sessions, with a positive allowlist for reusable profiles. [DISCARD](https://www.postgresql.org/docs/18/sql-discard.html), [LISTEN](https://www.postgresql.org/docs/18/sql-listen.html), [PREPARE](https://www.postgresql.org/docs/18/sql-prepare.html).

## Authentication and encryption

SCRAM verifier authentication on the frontend does not recover a reusable backend password. Provision backend credentials independently for each mapped login identity or use an equivalent same-role certificate mapping. A global privileged backend role with SET ROLE is outside the proposed initial contract. SCRAM channel binding must be computed for each independently terminated TLS channel. [SASL](https://www.postgresql.org/docs/18/sasl-authentication.html), [password authentication](https://www.postgresql.org/docs/18/auth-password.html).

## Replication and special clients

Physical and logical replication use specialized startup and COPY flows. Administrative probes can use dedicated connections; application session pools should reject replication startup until a dedicated profile is implemented. Logical replication has its own versioned message semantics and selective object scope. Large object function calls need a pinned primary session and explicit compatibility coverage. [Physical replication protocol](https://www.postgresql.org/docs/18/protocol-replication.html), [logical protocol](https://www.postgresql.org/docs/18/protocol-logical-replication.html), [large objects](https://www.postgresql.org/docs/18/largeobjects.html).

## Limits to decide in the protocol feature spec

Recommended initial lab defaults are 64 KiB startup packets, 16 MiB ordinary message bodies, 1 MiB buffered bytes per direction, 1 MiB SQL accepted for classification, and a bounded parser worker budget. These are proposals requiring driver and large value tests, not PostgreSQL protocol limits. Large data frames should be streamed where state validation permits. If a configured cap excludes a valid workload, document the supported profile rather than silently truncating it.
