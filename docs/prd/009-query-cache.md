# Query cache PRD

**State:** Proposed product contract, 2026-11-14. **Milestone:** V2 research, deferred. This PRD does not authorize implementation or claim working functionality. The feature spec will settle exact APIs, defaults and migrations before building.

## Problem statement

Some repeated reads may benefit from caching, but incorrect keys or invalidation can expose or return invalid data.

## Goals

Evaluate a narrow correctness contract before implementing an optional result cache.

## Non goals

No cache in MVP/V1, no TTL based correctness claim, no arbitrary MVCC snapshot caching or default external cache dependency.

## Functional requirements

1. Keep caching disabled until eligible queries, identities, GUCs and consistency are specified.
2. Include database/role/policy/session semantics and relevant snapshot bounds in any cache key.
3. Account for table dependencies, DDL, out-of-band writers and security policy changes.
4. Bound bytes, entries, TTL and eviction work.
5. Expose invalidation and statistics without allowing unauthorized cache reads.

## Non functional requirements

Correctness and identity isolation precede throughput. All queues, buffers, tasks and external calls have explicit budgets. Provisional limits and measured release gates are in the architecture and testing documents. No feature may silently replay uncertain work.

## Dependencies

Measured bottleneck, explicit consistency spec, routing semantic model and dependency/invalidation source.

## Data structures

Candidate CacheKey: cluster/database/role/policy generation, normalized query and bind identity, required session semantics and consistency token. Entry: payload bytes, dependency versions, insertion/expiry times and byte charge. These are feasibility targets, not a finalized key schema.

## Interfaces

Future cache adapter returns hit only when eligibility and dependency proof pass. Administrative invalidate is an authorized durable or safely idempotent action. No cache protocol or configuration is enabled now.

## Failure scenarios

External writer bypass, role/RLS change, schema change, stale dependencies, eviction storm, backend failover and lost invalidation.

## Security requirements

Cache results must not cross authorization boundaries. Raw SQL and bind values cannot become public metrics or logs. Memory and admin access must be protected.

## Observability

Eligible/miss/hit/bypass reasons, invalidation failures, byte usage, eviction rate and correctness comparison samples.

## Tests

Role/RLS separation, external writes, DDL, transaction snapshots, failover epoch and memory bounds; compare answers with uncached execution.

## Acceptance criteria

1. **AC-1:** Default and omitted configuration produce no result caching.
2. **AC-2:** No implementation is released before an external-write invalidation or deliberately restricted immutable-data contract is proven.
3. **AC-3:** Role and session variants never receive another context's cached result.
4. **AC-4:** Cache outage or uncertainty bypasses safely without replaying already executed work.

## Definition of Done

You can close this feature after its acceptance criteria pass against the declared real environment, critical tests are committed, a fresh reviewer examines the safety changes, operational/configuration documentation is current, and evidence is linked from the scope. Deferred features stay planned. No checkbox in this package records a build or test as passed.

## References

The [feature matrix](../feature-matrix.md) supplies official baseline sources. The [architecture decision](../specs/0001-initial-architecture/index.md), [security requirements](../security/security-requirements.md), and [test strategy](../testing/strategy.md) govern shared constraints.
