# Manual failover PRD

**State:** Proposed product contract, 2026-11-14. **Milestone:** V1. This PRD does not authorize implementation or claim working functionality. The feature spec will settle exact APIs, defaults and migrations before building.

## Problem statement

Unfenced promotion can create two writable primaries, and lost responses can cause unsafe repeated operations.

## Goals

Give an authorized operator a reviewable plan and a reconciled, fenced promotion operation.

## Non goals

No automatic promotion, transparent client transaction replay, automatic reverse promotion or implied zero data loss under asynchronous replication.

## Functional requirements

1. Build a candidate plan from fresh role, identity, timeline and replication evidence.
2. Bind operator approval to plan hash, target, expected epoch, RPO disclosure and expiration.
3. Obtain and independently validate a fencing receipt for the exact old-primary infrastructure identity.
4. Write durable intent before fencing and promotion, retire candidate replica leases, and reconcile unknown external outcomes.
5. After observing the new primary, durably commit approved writer, epoch, quarantine and operation result before publishing eligibility. A known final flush target requires a barrier across all writer paths and proof that candidate replay reaches it on compatible timeline history; otherwise keep a required catch-up plan closed or replan with an explicitly acknowledged loss bound.

## Non functional requirements

Correctness and identity isolation precede throughput. All queues, buffers, tasks and external calls have explicit budgets. Provisional limits and measured release gates are in the architecture and testing documents. No feature may silently replay uncertain work.

## Dependencies

Durable control model, authenticated admin roles, actual fencing provider, topology, health and PostgreSQL promotion privilege.

## Data structures

PromotionPlan: UUID, source/candidate IDs, system ID, epoch, evidence timestamps, LSN bound nullable, data-loss acknowledgement, hash and expiry required. FenceReceipt references plan and exact machine identity. Operation phases: Planned, Quiescing, Fencing, Revalidating, Promoting, Reconciling, Completed or Failed/NeedsReview.

## Interfaces

POST `/v1/failover/plans` accepts candidate_id and expected_generation, returns evidence and plan_hash. POST `/v1/failover/plans/{id}/execute` requires plan_hash, request_id and explicit data_loss_ack if unknown. GET `/v1/operations/{id}` returns durable state. Errors: 403 role, 409 stale/conflict, 412 unsafe preconditions, 503 durable/provider unavailable.

## Failure scenarios

Old primary only partitioned, fence request incomplete, fence expiry, competing operation, stale plan, crash before/after promotion, candidate failure and lost promotion response.

## Security requirements

Promoter role distinct from observer/operator. Allowlisted fencing adapter uses structured arguments and restricted credentials. Local tampering/root access is outside the daemon trust boundary and must be controlled operationally.

## Observability

Operation phases and duration, fencing evidence class, candidate and timeline, RPO known/unknown, interruption duration and reconciliation outcome.

## Tests

Crash at each phase, partitioned old primary, independent fence verification failure, duplicate execute and asynchronous commit-loss history.

## Acceptance criteria

1. **AC-1:** Missing, stale or target-mismatched fencing evidence prevents promotion.
2. **AC-2:** A health timeout alone produces no executable promotion operation.
3. **AC-3:** Crash or lost promotion response enters reconciliation and never blindly replays promotion.
4. **AC-4:** Promotion permanently crosses the rollback boundary; the old primary remains fenced/quarantined and approved writer/epoch survive restart. Candidate replica leases are retired before promotion.
5. **AC-5:** Unknown loss window is shown and explicitly acknowledged; a known flush target is claimed only after a verified barrier covering POOL and direct database writers.

## Definition of Done

You can close this feature after its acceptance criteria pass against the declared real environment, critical tests are committed, a fresh reviewer examines the safety changes, operational/configuration documentation is current, and evidence is linked from the scope. Deferred features stay planned. No checkbox in this package records a build or test as passed.

## References

The [feature matrix](../feature-matrix.md) supplies official baseline sources. The [architecture decision](../specs/0001-initial-architecture/index.md), [security requirements](../security/security-requirements.md), and [test strategy](../testing/strategy.md) govern shared constraints.
