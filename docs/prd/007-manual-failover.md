# Manual failover PRD

**State:** Proposed product contract, 2026-11-14. **Milestone:** V1.

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
5. After observing the new primary, durably commit approved writer, epoch, quarantine and operation result before publishing eligibility. Follow the [candidate and recovery point policy](../architecture/failover-recovery.md#candidate-and-rpo-policy) for a known flush target or an unknown loss window.

## Non functional requirements

Keep the cluster closed while the outcome of fencing or promotion is unknown. Preserve enough durable evidence to reconcile the exact operation after restart.

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
3. **AC-3:** Drop the provider's fence response or PostgreSQL's promotion response after the effect. Retrying execution or restarting returns the operation to reconciliation. An inconclusive result keeps admission closed and prevents a second irreversible action.
4. **AC-4:** Promotion permanently crosses the rollback boundary; the old primary remains fenced/quarantined and approved writer/epoch survive restart. Candidate replica leases are retired before promotion.
5. **AC-5:** A plan requiring catch up stops if any writer bypasses the barrier or the candidate has not replayed the final flush target on compatible history. Switching to an unknown loss window requires a new plan and explicit approval of that disclosure.

## Definition of Done

Apply the shared [acceptance evidence and closure rules](../testing/strategy.md#acceptance-evidence) to the criteria above.

## References

[Manual promotion scope](../scope/010-fenced-manual-promotion.md) and [fencing contract](../architecture/failover-recovery.md#fencing-contract).
