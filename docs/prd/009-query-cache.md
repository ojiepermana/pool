# Query cache PRD

**State:** Proposed product contract, 2026-11-14. **Milestone:** V2 research, deferred.

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

The feasibility study must budget memory, eviction and invalidation work, including behavior when its dependency information is unavailable.

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

1. **AC-1:** Until a separate cache contract is approved and proven, omitted or disabled configuration produces no caching and an attempt to enable it is rejected under [SEC-10](../security/security-requirements.md).
2. **AC-2:** No implementation is released before an external-write invalidation or deliberately restricted immutable-data contract is proven.
3. **AC-3:** Role and session variants never receive another context's cached result.
4. **AC-4:** Cache outage or uncertainty bypasses safely without replaying already executed work.

## Definition of Done

Apply the shared [acceptance evidence and closure rules](../testing/strategy.md#acceptance-evidence) to the criteria above.

## References

[Optional result cache scope](../scope/015-optional-result-cache.md) and [feasibility study](../research/feasibility-study.md).
