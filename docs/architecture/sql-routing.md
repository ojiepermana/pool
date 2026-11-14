# SQL routing

The initial safe policy is primary default plus explicitly configured read only session profiles. An operator managed map resolves a verified frontend principal and requested database to a permitted cluster, database, actual backend login, credential reference/generation and profile. Unmapped combinations fail; startup strings alone cannot select credentials.

Choose the backend at login from profile and fresh topology, before SQL is available. After authentication and ReadyForQuery, admit or reject each operation on that pinned backend according to protocol/session state and profile. The parser is not an input that must exist before initial backend selection.

Read only profiles accept eventual reads. A fresh lag sample screens admission; it is not a bound on the staleness of every later result in a long session. There is no continuous lag guarantee. Initial replica profiles have no writable primary fallback. They cannot switch into write sessions, and affected replica leases are retired before controlled promotion.

## Decision matrix

| Condition | Decision | Reason |
|---|---|---|
| Unknown cluster identity, multiple primaries or expired topology | Refuse new affected admission | No safe authority |
| Ordinary primary profile | Primary and keep lease | Preserves mixed session semantics and read after write |
| DML, DDL, MERGE, COPY FROM, sequence update | Primary; reject in replica profile | Writes require writable authority |
| SELECT FOR UPDATE/SHARE, writable CTE, EXPLAIN ANALYZE of writes | Primary; reject in replica profile | Read spelling does not establish read behavior |
| Ambiguous parse, oversized SQL, unknown function/operator, procedural call | Primary; reject in restricted replica profile | Conservative classification |
| Multiple statements | Ordinary profile stays primary as one unit; restricted replica profile rejects | Preserve order and state |
| Session state, temporary objects, LISTEN, advisory locks, cursor, large object | Ordinary profile stays primary; restricted replica profile rejects unsupported operations | State is backend local |
| Prepared statement or portal | Same backend as Parse/Bind and session | Never guess routing at Execute without its owner |
| Read only profile at login, fresh replica under admission lag budget | Select replica and pin before SQL arrives | Explicit eventual consistency contract |
| Later operation on replica lease has eligible syntax and approved object policy | Admit on existing pinned lease | Syntax narrows operation admission, not initial endpoint selection |
| Later operation fails restricted replica policy | Refuse operation without switching endpoint | No query-derived migration to primary |
| No eligible replica | Bounded wait or explicit unavailable error | Initial primary fallback is disabled |
| Failure after dispatch | Return failure, do not replay or migrate | Outcome may be uncertain |

## Syntax and semantics

Use PostgreSQL 18 raw grammar through an adapter. Classification reports effects, statement boundaries, uncertainty and reason codes. It is not an authorization engine. Views, rules, RLS, overloaded operators, casts, FDWs, SECURITY DEFINER functions and external side effects cannot be proven safe by a raw AST. A read profile needs trusted schema/extension ownership, approved query surface, database permissions and an operational change process. Volatility annotations are promises, not a proof against a malicious extension.

The restricted profile spec must define how to reject an operation before forwarding unsafe bytes. For a simple Query, check the entire message. For extended operations, bind admission to the exact prepared statement and portal on the lease, including replacement of unnamed objects. Cover changes to schema or policy between Parse and Execute. If a refusal occurs after earlier frames reached PostgreSQL, the spec must show how the client and backend stay synchronized through Sync and report the true transaction state, or close the session. See the [protocol state model](protocol-engine.md).

## Consistency

Primary sessions provide normal PostgreSQL behavior on their pinned backend. Replica sessions provide eventual reads, not primary freshness or cross session causality. Lag thresholds screen new session admission only. An LSN based causal token would need cluster identity, timeline history, a commit flush source and replay verification; this is deferred rather than approximated by a time based sticky window. After write, a primary session stays primary for its lifetime.

Before a candidate is promoted, stop new affected leases and retire its existing replica-profile connections so hot standby protection cannot disappear beneath them. Observed unexpected role changes revoke eligibility and affected sessions; operator controlled promotion always performs retirement before its side effect. Database permissions and approved query restrictions remain necessary even on a standby. A future primary fallback would need independently enforced read restrictions that survive role changes and a new explicit decision.

General transparent per query routing is a later feature with a complete session virtualization design. This is a deliberate initial limitation to review with you, because the phrase automatic reads can otherwise imply a much broader guarantee.
