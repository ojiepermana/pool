# Cluster management PRD

**State:** Proposed product contract, 2026-11-14. **Milestone:** V1.

## Problem statement

Operators need safe topology changes while application sessions remain owned and mutations remain auditable.

## Goals

Register, validate, maintain, drain, detach and attach configured nodes using generation checked operations.

## Non goals

No discovery by arbitrary DNS, automatic former-primary attach, direct backup engine or distributed writer authority in V1.

## Functional requirements

1. Validate unique stable node IDs and physical cluster identity.
2. Separate observed health/role from operator maintenance and desired membership.
3. Persist approved writer, epoch and quarantine, then publish immutable snapshots only when fresh observed roles agree with that durable authorization.
4. Journal mutations with caller, idempotency key and expected generation.
5. Drain before detach, require fresh evidence before attach and quarantine conflicting primaries.

## Non functional requirements

A completed mutation must survive restart. Use the [control records](../architecture/control-plane.md) to distinguish durable restrictions from observations that must be sampled again.

## Dependencies

Health, validated config, durable journal, admin authentication and lease draining.

## Data structures

The [control plane records](../architecture/control-plane.md) distinguish configured nodes, approved membership and writer authority, live observations and the operation that changed the approved state.

## Interfaces

GET `/v1/cluster` and `/v1/nodes` expose the generation a caller needs for its next mutation. POST operations for drain, detach and attach take that expected generation and follow the [administration retry contract](010-administration.md). They return an operation ID and state.

## Failure scenarios

Concurrent mutations, stale clients, restarted daemon, journal full, role switch during attach, returned old primary and drain deadline expiration.

## Security requirements

Viewer reads status, operator drains/attaches, promoter alone approves promotion. Denied and successful mutations are audited without secret payloads.

## Observability

Topology generation, ineligible node reasons, active operations, drain occupancy, mutation errors and journal health.

## Tests

Concurrent compare-and-swap requests, crash reconciliation, dual-primary observation, maintenance admission and idempotent retries.

## Acceptance criteria

1. **AC-1:** Stale mutations are rejected. For conflicting attach and detach requests against the same generation, at most one takes effect.
2. **AC-2:** Two observed primaries close write admission until explicit reconciliation.
3. **AC-3:** Approved writer, epoch, maintenance, detach and quarantine survive restart. A healthy observation cannot clear a restriction, a conflicting configuration cannot replace authority, and a returned old primary stays quarantined until rejoin evidence passes.
4. **AC-4:** Failure to persist intent prevents the external action. If storage fails after the action, the caller receives no success confirmation and restart reconciles the recorded intent before another action can run.

## Definition of Done

Apply the shared [acceptance evidence and closure rules](../testing/strategy.md#acceptance-evidence) to the criteria above.

## References

[Topology administration](../scope/009-safe-topology-administration.md) and [control plane](../architecture/control-plane.md).
