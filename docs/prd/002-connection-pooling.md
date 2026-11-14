# Connection pooling PRD

**State:** Proposed product contract, 2026-11-14. **Milestone:** Core MVP.

## Problem statement

Frequent reconnects create unnecessary backend startup cost, while unsafe reuse can leak identity or session state.

## Goals

Reuse clean backend sessions without changing the lifetime semantics of a connected frontend.

## Non goals

No transaction or statement multiplexing in V1; no promise of backend reduction for permanently connected clients.

## Functional requirements

1. Partition by full backend identity and configuration/credential generations.
2. Assign one backend per connected frontend and enforce nested global budgets.
3. Rollback/reset and validate idle state before reuse; close uncertain extension or protocol state.
4. Bound acquisition queues, timeouts, idle age and lifetime.
5. Drain removed nodes and rotated credentials without reassigning in flight leases.

## Non functional requirements

Charge connecting, cleaning and idle backends against the same limits as active ones. Use the [pool budgets](../architecture/connection-pooling.md#limits-and-operations) when testing contention across keys.

## Dependencies

The [clean session reuse slice](../scope/005-clean-session-reuse.md) builds on the dedicated primary path and its identity map. Exercise cleanup with the operations available at that point. Re-run isolation with portals, asynchronous traffic and cancellation as those paths arrive in [protocol breadth](../scope/006-protocol-breadth-and-cancellation.md).

## Data structures

PoolKey uses the [pool identity and lifecycle](../architecture/connection-pooling.md#pool-identity-and-lifecycle). PoolState tracks idle backends and waiters within the configured budgets. Each lease has one owner and a unique generation.

## Interfaces

Core `acquire(identity, profile, deadline)` returns an owned Lease or typed timeout/overload/unavailable error. `release(lease, terminal_state)` returns clean idle or closes. Admin reads expose bounded occupancy, not secrets.

## Failure scenarios

Reset failure, aborted transaction, orphan portal, LISTEN residue, advisory locks, dirty temp objects, credential rotation during acquisition and queued disconnect.

## Security requirements

Pool keys preserve actual database login identity. Test alternating users and databases. Secret references are generation aware and redacted.

## Observability

Active/idle/connecting/waiting, acquisition latency, reset close reason, cap rejection, drain progress and backend reuse counters.

## Tests

Alternating tenant state histories, unknown extension cleanup, queue fairness, permits on cancellation and restart/drain behavior.

## Acceptance criteria

1. **AC-1:** Two successive clients with the same pool key use the same physical PostgreSQL backend in a reusable case. The second sees its startup defaults and none of the first client's GUC changes, temp objects, locks, portals or notifications. A separate test varies the login, database or cluster and confirms that those keys never share a backend.
2. **AC-2:** Aggregate backend count never exceeds configured global limits across many pool keys.
3. **AC-3:** Reset uncertainty closes the socket instead of returning it idle.
4. **AC-4:** Retire a credential or configuration generation while a client is waiting and again during a backend handshake. The client never receives a lease from the retired generation. A backend returned by an earlier owner cannot re-enter the eligible pool under that generation.

## Definition of Done

Apply the shared [acceptance evidence and closure rules](../testing/strategy.md#acceptance-evidence) to the criteria above.

## References

[Pool identity and lifecycle](../architecture/connection-pooling.md#pool-identity-and-lifecycle).
