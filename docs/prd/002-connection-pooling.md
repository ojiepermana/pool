# Connection pooling PRD

**State:** Proposed product contract, 2026-11-14. **Milestone:** Core MVP. This PRD does not authorize implementation or claim working functionality. The feature spec will settle exact APIs, defaults and migrations before building.

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

Correctness and identity isolation precede throughput. All queues, buffers, tasks and external calls have explicit budgets. Provisional limits and measured release gates are in the architecture and testing documents. No feature may silently replay uncertain work.

## Dependencies

Core proxy, authentication boundary and configuration model.

## Data structures

PoolKey: cluster ID, endpoint ID, login role, database, startup profile, TLS policy hash, credential generation and epoch, all required. PoolState: idle deque and waiters bounded by config. Lease uses unique owner and generation. No shared session state is persisted across frontend owners.

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

1. **AC-1:** Repeated disconnect/reconnect cycles reuse validated backends without another user seeing GUCs, temp objects, locks, portals or notifications.
2. **AC-2:** Aggregate backend count never exceeds configured global limits across many pool keys.
3. **AC-3:** Reset uncertainty closes the socket instead of returning it idle.
4. **AC-4:** Credential/config generation changes prevent new acquisition of retired backends.

## Definition of Done

You can close this feature after its acceptance criteria pass against the declared real environment, critical tests are committed, a fresh reviewer examines the safety changes, operational/configuration documentation is current, and evidence is linked from the scope. Deferred features stay planned. No checkbox in this package records a build or test as passed.

## References

The [feature matrix](../feature-matrix.md) supplies official baseline sources. The [architecture decision](../specs/0001-initial-architecture/index.md), [security requirements](../security/security-requirements.md), and [test strategy](../testing/strategy.md) govern shared constraints.
