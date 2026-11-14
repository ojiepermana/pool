# Cluster management PRD

**State:** Proposed product contract, 2026-11-14. **Milestone:** V1. This PRD does not authorize implementation or claim working functionality. The feature spec will settle exact APIs, defaults and migrations before building.

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

Correctness and identity isolation precede throughput. All queues, buffers, tasks and external calls have explicit budgets. Provisional limits and measured release gates are in the architecture and testing documents. No feature may silently replay uncertain work.

## Dependencies

Health, validated config, durable journal, admin authentication and lease draining.

## Data structures

Cluster 1:N Backend; Backend 1:N ProbeObservation. ApprovedTopology has unique cluster ID, approved writer ID, durable epoch, quarantine and transition operation ID. Config generation and topology generation required. Operation ID primary key, actor required, expected generation required, idempotency key unique within actor/action scope, phase and outcome required. Bootstrap approved authority once from config, then require explicit transitions. See control plane record table.

## Interfaces

GET `/v1/cluster`, GET `/v1/nodes`; POST `/v1/nodes/{id}/drain`, `/detach`, `/attach` accept expected_generation and request_id. Returns operation ID, state and generation; 400 invalid, 403 denied, 409 stale/conflict, 503 evidence unavailable.

## Failure scenarios

Concurrent mutations, stale clients, restarted daemon, journal full, role switch during attach, returned old primary and drain deadline expiration.

## Security requirements

Viewer reads status, operator drains/attaches, promoter alone approves promotion. Denied and successful mutations are audited without secret payloads.

## Observability

Topology generation, ineligible node reasons, active operations, drain occupancy, mutation errors and journal health.

## Tests

Concurrent compare-and-swap requests, crash reconciliation, dual-primary observation, maintenance admission and idempotent retries.

## Acceptance criteria

1. **AC-1:** Stale expected_generation requests cannot overwrite newer topology.
2. **AC-2:** Two observed primaries close write admission until explicit reconciliation.
3. **AC-3:** Approved writer, epoch and quarantine survive restart; conflicting config or observations cannot silently change authority, and a returned old primary stays quarantined until rejoin evidence passes.
4. **AC-4:** Journal failure rejects mutations before external side effects.

## Definition of Done

You can close this feature after its acceptance criteria pass against the declared real environment, critical tests are committed, a fresh reviewer examines the safety changes, operational/configuration documentation is current, and evidence is linked from the scope. Deferred features stay planned. No checkbox in this package records a build or test as passed.

## References

The [feature matrix](../feature-matrix.md) supplies official baseline sources. The [architecture decision](../specs/0001-initial-architecture/index.md), [security requirements](../security/security-requirements.md), and [test strategy](../testing/strategy.md) govern shared constraints.
