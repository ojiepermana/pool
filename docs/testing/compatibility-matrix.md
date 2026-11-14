# Driver compatibility matrix

**State:** Planned, not tested. PostgreSQL 18 is the initial server major. The walking slice uses one pinned psql/libpq build. Core MVP qualifies the first two rows, including each listed Rust client. V1 must cover those rows and the remaining clients on its release candidate.

| Client | Required paths | Important cases | Release gate |
|---|---|---|---|
| libpq / psql | 3.0 plus explicit 3.2, TLS/SCRAM, simple/extended, COPY, cancel | SSLRequest/direct TLS where claimed, variable cancel keys, large objects, notifications | MVP |
| Rust tokio-postgres and postgres | Binary/text binds, prepared statements, async notifications | Named/unnamed objects, pipeline errors, disconnect cancellation | MVP |
| Node.js node-postgres | Simple and extended, pool reconnect, TLS | Named statements, query errors and cancel where supported | V1 |
| Bun SQL/PostgreSQL client | Prepared/binary behavior, TLS, transactions | Version-specific protocol and pooling defaults | V1 |
| Go pgx | Simple/extended and prepared cache modes | Batches, binary formats, cancel and transaction errors | V1 |
| Go lib/pq | Text paths and notifications | LISTEN, COPY and session assumptions | V1 |
| Python psycopg 3 | Simple/extended, pipeline and COPY | Auto prepare thresholds, transaction contexts, cancel | V1 |
| Python asyncpg | Prepared and binary protocol | Type codecs, portals and server parameters | V1 |
| JDBC pgjdbc | Extended, fetch sizes and prepared thresholds | Suspended portals, cursors, batches and cancel | V1 |

Each client runs direct PostgreSQL and proxied variants with equivalent SQL and connection state. Compare rows, errors, transaction outcome, notification semantics, metadata and isolation. Do not compare unstable timestamps or server process IDs as if they must match. Confirm expected differences such as virtual cancellation identity explicitly.

Test all supported protocol/auth combinations, including unsupported option errors. Protocol 2, legacy MD5, GSS encryption/auth, OAuth, logical replication clients and arbitrary PostgreSQL extensions are outside the initial support claim until separately qualified. A refused unsupported feature is different from silent corruption.

For each result record the client and server builds, requested and negotiated protocol, auth/TLS mode, direct and proxied observations, and a link to the [verification record](strategy.md#acceptance-evidence). Record any driver limitation alongside the applicable case. For example, a run negotiated at 3.0 supplies no evidence for the 3.2 cancellation path. List unrun and excluded combinations separately from passing ones. Link results for the candidate from its release report.
