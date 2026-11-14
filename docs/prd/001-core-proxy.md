# Core proxy PRD

**State:** Proposed product contract, 2026-11-14. **Milestone:** Core MVP.

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

Bound frame sizes, concurrent handshakes and admission time. The [data plane resource proposal](../architecture/data-plane.md#resource-proposal) supplies lab values for the spec to qualify.

## Dependencies

The [walking slice](../scope/004-one-real-primary-path.md) needs a mapped login, an authorized writer with fresh matching identity and role evidence, and a direct PostgreSQL 18 baseline. Start AC-1 and the applicable admission and failure cases of AC-3/AC-4 there. The [protocol breadth slice](../scope/006-protocol-breadth-and-cancellation.md) qualifies the remaining corpus and driver combinations for the Core MVP.

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

1. **AC-1:** A TLS authenticated psql session reaches the mapped login and database on the authorized primary. INSERT, SELECT and a server error match a direct session under that login, including SQLSTATE. A second connection confirms that a write left uncommitted at disconnect was rolled back. An unmapped principal/database pair or conflicting writer evidence is refused before application SQL can run.
2. **AC-2:** The declared extended/COPY/cancellation corpus passes for 3.0 and 3.2 without cross lease effects, including delayed delivery after completion and attempted release/reuse.
3. **AC-3:** Submit an oversized frame and saturate the configured connection and waiting limits. Observe the specified refusal or deadline, memory within the budget declared in the feature spec, and permits returned after disconnect.
4. **AC-4:** Break the backend connection after PostgreSQL receives COMMIT and before POOL receives the response. The client observes connection loss, the session ends, and a backend trace shows one dispatch. POOL never retries the transaction or reports a definite rollback for that lost response.

## Definition of Done

Apply the shared [acceptance evidence and closure rules](../testing/strategy.md#acceptance-evidence) to the criteria above.

## References

[Protocol inventory](../research/protocol-analysis.md) and [protocol engine](../architecture/protocol-engine.md).
