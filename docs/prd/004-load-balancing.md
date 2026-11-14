# Load balancing PRD

**State:** Proposed product contract, 2026-11-14. **Milestone:** V1. This PRD does not authorize implementation or claim working functionality. The feature spec will settle exact APIs, defaults and migrations before building.

## Problem statement

Read capacity is useful only when selection excludes stale, draining or incorrectly identified nodes.

## Goals

Distribute new read sessions across eligible replicas with explicit unavailable behavior.

## Non goals

No per query backend switching, latency feedback policy or least connections performance claim in the initial policy.

## Functional requirements

1. Filter by role, cluster identity, observation freshness, maintenance and configured lag budget.
2. Use deterministic weighted round robin for initial new session selection.
3. Pin each selected session and recheck eligibility before acquisition completes.
4. Provide a bounded wait or explicit refusal when no replica is eligible. Primary fallback is disabled in V1.
5. Retain per database/user policy and a testable tie break order.

## Non functional requirements

Correctness and identity isolation precede throughput. All queues, buffers, tasks and external calls have explicit budgets. Provisional limits and measured release gates are in the architecture and testing documents. No feature may silently replay uncertain work.

## Dependencies

Routing profiles, topology snapshots, admission budgets and health checks.

## Data structures

ReplicaPolicy: positive weights keyed by backend ID, admission-time lag byte limit, observation age cap and fallback=false. SelectorState: bounded per policy sequence counter. CandidateSet: snapshot generation plus eligible IDs. Weight changes affect new sessions only. The lag screen does not guarantee ongoing result freshness.

## Interfaces

`choose(profile, snapshot, selector_state)` returns backend ID and reason. Configuration reload rejects negative/all-zero unusable weights. Diagnostics show eligibility exclusions and weights.

## Failure scenarios

All replicas stale, role changes during acquisition, zero capacity, selector contention and backend draining. No case silently selects a writable primary for the replica profile.

## Security requirements

Only authorized configuration roles change weights. Enabling primary fallback requires a new decision and independently enforced read restrictions that survive promotion. Client supplied routing hints do not bypass policy.

## Observability

Selections by bounded backend ID, exclusion reason, occupancy, no-eligible-replica count and distribution over observation windows.

## Tests

Deterministic weighted trials, all-stale cases, concurrent selection, maintenance transitions and snapshot replacement.

## Acceptance criteria

1. **AC-1:** An ineligible replica is selected zero times in the controlled corpus.
2. **AC-2:** At least 10,000 equal-duration synthetic acquisitions follow configured weights within a documented five percentage point absolute tolerance.
3. **AC-3:** Weight reload never moves an already leased session.
4. **AC-4:** No eligible replica produces a bounded wait or explicit refusal, with zero primary fallback selections in V1.

## Definition of Done

You can close this feature after its acceptance criteria pass against the declared real environment, critical tests are committed, a fresh reviewer examines the safety changes, operational/configuration documentation is current, and evidence is linked from the scope. Deferred features stay planned. No checkbox in this package records a build or test as passed.

## References

The [feature matrix](../feature-matrix.md) supplies official baseline sources. The [architecture decision](../specs/0001-initial-architecture/index.md), [security requirements](../security/security-requirements.md), and [test strategy](../testing/strategy.md) govern shared constraints.
