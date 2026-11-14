# Load balancing PRD

**State:** Proposed product contract, 2026-11-14. **Milestone:** V1.

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

Selection must finish within the acquisition deadline. Publish selection counts only for configured nodes; never create selector state for arbitrary client input.

## Dependencies

The [restricted replica slice](../scope/007-restricted-replica-sessions.md) consumes approved profiles, admission budgets and fresh topology under the [cluster health contract](../architecture/cluster-management.md). The selector takes eligibility as input.

## Data structures

ReplicaPolicy contains weights keyed by backend ID, an admission lag limit in bytes, an observation age cap and fallback=false. Only eligible replicas with positive weight can receive new sessions. SelectorState keeps bounded state per policy. CandidateSet contains the snapshot generation and eligible IDs.

## Interfaces

`choose(profile, snapshot, selector_state)` returns a backend ID or an unavailable reason. Configuration reload rejects negative weights and a policy with all weights zero. The feature spec must settle whether a single zero weight explicitly disables that replica or makes the configuration invalid. Diagnostics show exclusions and weights.

## Failure scenarios

All replicas stale, role changes during acquisition, zero capacity, selector contention and backend draining. No case silently selects a writable primary for the replica profile.

## Security requirements

Only authorized configuration roles change weights. Enabling primary fallback requires a new decision and independently enforced read restrictions that survive promotion. Client supplied routing hints do not bypass policy.

## Observability

Selections by bounded backend ID, exclusion reason, occupancy, no-eligible-replica count and distribution over observation windows.

## Tests

Deterministic weighted trials, all-stale cases, concurrent selection, maintenance transitions and snapshot replacement.

## Acceptance criteria

1. **AC-1:** No client obtains an ineligible replica. Cover a matching snapshot at selection followed by stale evidence, drain or a role mismatch before lease handoff.
2. **AC-2:** With fixed eligibility, available capacity and unequal positive weights, run at least 10,000 successful synthetic acquisitions. Each replica's share of initial leases falls within five percentage points of its weight divided by the total eligible weight.
3. **AC-3:** Weight reload never moves an already leased session.
4. **AC-4:** With all replicas ineligible, a new replica session fails by its configured admission deadline. The trace shows zero acquisitions on the primary.

## Definition of Done

Apply the shared [acceptance evidence and closure rules](../testing/strategy.md#acceptance-evidence) to the criteria above.

## References

[Selection policies](../architecture/cluster-management.md#selection-policies) and [replica session scope](../scope/007-restricted-replica-sessions.md).
