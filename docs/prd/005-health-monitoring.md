# Health monitoring PRD

**State:** Proposed product contract, 2026-11-14. **Milestone:** V1. This PRD does not authorize implementation or claim working functionality. The feature spec will settle exact APIs, defaults and migrations before building.

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

Correctness and identity isolation precede throughput. All queues, buffers, tasks and external calls have explicit budgets. Provisional limits and measured release gates are in the architecture and testing documents. No feature may silently replay uncertain work.

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
3. **AC-3:** With proposed 2 second period and 1 second timeout, failures are observed and eligibility expires within the configured 6 second freshness bound plus documented scheduler tolerance.
4. **AC-4:** Health failure alone produces no promotion action.

## Definition of Done

You can close this feature after its acceptance criteria pass against the declared real environment, critical tests are committed, a fresh reviewer examines the safety changes, operational/configuration documentation is current, and evidence is linked from the scope. Deferred features stay planned. No checkbox in this package records a build or test as passed.

## References

The [feature matrix](../feature-matrix.md) supplies official baseline sources. The [architecture decision](../specs/0001-initial-architecture/index.md), [security requirements](../security/security-requirements.md), and [test strategy](../testing/strategy.md) govern shared constraints.
