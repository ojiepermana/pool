# Health monitoring PRD

**State:** Proposed product contract, 2026-11-14. **Milestone:** V1.

## Problem statement

Reachability alone cannot identify a safe writable primary or a sufficiently current replica.

## Goals

Publish fresh authenticated observations with explicit unknown values and flapping control.

## Non goals

No promotion caused by health failure and no freshness guarantee derived only from time since last replay.

## Functional requirements

1. Run bounded authenticated probes through dedicated monitoring credentials and capacity.
2. Observe role, system identity, timeline and relevant WAL positions.
3. Compute lag only for comparable fresh positions and report missing evidence as unknown.
4. Apply failure/recovery thresholds and monotonic expiry.
5. Keep monitoring failures separate from administrative mutation authority.

## Non functional requirements

Reserve probe capacity and measure the age of the observation at admission. A delayed publisher cannot extend its lifetime. See [health and topology](../architecture/cluster-management.md#health-and-topology).

## Dependencies

Monitor identity, connection budgets, PostgreSQL probes and topology model.

## Data structures

ProbeObservation: backend ID and sampled monotonic instant required; role/system ID/timeline/LSNs nullable on failure; error class and freshness required. HealthState: Unknown, Healthy, Suspect, Unavailable, Recovering. Config has units and finite deadlines.

## Interfaces

Monitor SQL/identity probes return typed observations. `GET /v1/nodes` includes sampled time, age, role, eligibility and reason. Authentication/timeout/role mismatch remain distinct.

## Failure scenarios

TCP works but SQL fails, revoked monitor permission, clock jump, slow/hung query, WAL replay pause, primary isolation and incompatible cluster.

## Security requirements

Least privilege monitor role, fixed queries and verified TLS. Privileged identity functions receive only explicit grants tested against PostgreSQL 18.

## Observability

Probe latency and outcome, state transitions, observation age, receive/replay/flush positions where permitted, unknown lag and flapping counters.

## Tests

Network delay/drop, permission removal, replay pause, wrong system ID and monotonic time tests with a controllable clock.

## Acceptance criteria

1. **AC-1:** A reachable wrong-role or wrong-cluster node cannot become eligible.
2. **AC-2:** Missing or expired WAL evidence is visible as unknown and excludes replica admission.
3. **AC-3:** With the probe publisher paused past the configured freshness deadline, a new acquisition refuses the expired observation even if the last published status was Healthy. Using the [lab settings](../architecture/cluster-management.md), inject failed probes and record the first refusal and first diagnostic update. Eligibility must expire by its freshness bound; set a separate scheduler tolerance for reporting in the feature spec and test against it.
4. **AC-4:** Health failure alone produces no promotion action.

## Definition of Done

Apply the shared [acceptance evidence and closure rules](../testing/strategy.md#acceptance-evidence) to the criteria above.

## References

[Health diagnostics and drain](../scope/008-health-diagnostics-and-drain.md).
