# Core proxy PRD

**State:** Proposed product contract, 2026-11-14. **Milestone:** Core MVP. This PRD does not authorize implementation or claim working functionality. The feature spec will settle exact APIs, defaults and migrations before building.

## Problem statement

Applications need a predictable PostgreSQL endpoint that preserves server protocol behavior under fragmentation, authentication, load, and disconnects.

## Goals

Prove one real client path, then qualify the declared PostgreSQL 18 protocol profile.

## Non goals

No automatic replay, query cache, live session migration, PostgreSQL replication engine, or complete driver compatibility claim.

## Functional requirements

1. Negotiate supported startup and protocol 3.0/3.2 with bounded framing.
2. Terminate TLS and frontend authentication independently of the identity preserving backend login.
3. Relay simple, extended, asynchronous and COPY traffic on the owning backend lease.
4. Forward cancellation only through the live lease owner, serialize it with retirement, and retire the physical backend after forwarding so later clients cannot be affected.
5. Apply bounded admission, deadlines and graceful teardown without runtime blocking.

## Non functional requirements

Correctness and identity isolation precede throughput. All queues, buffers, tasks and external calls have explicit budgets. Provisional limits and measured release gates are in the architecture and testing documents. No feature may silently replay uncertain work.

## Dependencies

Initial architecture, protocol contract, identity mapping and a PostgreSQL 18 test instance.

## Data structures

Session: id u128 required, verified principal ID required, mapped cluster/database/backend login/profile required, protocol enum required, state enum required, lease ID optional until acquired. The mapping comes from operator configuration and secret references, not unverified startup claims. Lease: id and generation u64 required, backend ID required, owner session ID unique. CancellationEntry: per lease opaque frontend token, session/lease generation and expiry required; real backend key data remains private. Sensitive keys are memory only.

## Interfaces

PostgreSQL listener accepts Startup/SSL/Cancel packets and typed frames. Client supplied identity is validated before use. Backend errors preserve SQLSTATE; overload uses documented resource errors. `pool check-config` validates without connecting.

## Failure scenarios

Partial frame, oversized packet, wrong auth sequence, TLS timeout, half closed socket, backend loss during COMMIT, late cancellation and interrupted COPY.

## Security requirements

Verify backend server names and CA chains. Bound SCRAM CPU and token storage. No password, SQL parameter, raw SQL or cancellation secret in logs. Never impersonate all users through a privileged shared backend role.

## Observability

Connections accepted/rejected, auth failure class, bytes, protocol error class, lease lifetime, cancellation outcome and bounded latency histograms.

## Tests

Golden fragmented frames, state model properties, malformed protocol fuzzing, libpq and driver tests, lost COMMIT response and late cancel race.

## Acceptance criteria

1. **AC-1:** A TLS authenticated psql session can INSERT and SELECT through one primary with results matching direct PostgreSQL.
2. **AC-2:** The declared extended/COPY/cancellation corpus passes for 3.0 and 3.2 without cross lease effects, including delayed delivery after completion and attempted release/reuse.
3. **AC-3:** Exceeding configured frame, connection and queue limits produces bounded memory and an explicit failure.
4. **AC-4:** Backend loss after COMMIT dispatch never causes POOL to replay the transaction.

## Definition of Done

You can close this feature after its acceptance criteria pass against the declared real environment, critical tests are committed, a fresh reviewer examines the safety changes, operational/configuration documentation is current, and evidence is linked from the scope. Deferred features stay planned. No checkbox in this package records a build or test as passed.

## References

The [feature matrix](../feature-matrix.md) supplies official baseline sources. The [architecture decision](../specs/0001-initial-architecture/index.md), [security requirements](../security/security-requirements.md), and [test strategy](../testing/strategy.md) govern shared constraints.
