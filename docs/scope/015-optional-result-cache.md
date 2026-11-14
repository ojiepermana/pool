# 15. Optional result cache · planned · needs a decision

[Scope index](index.md)

Evaluate a measured caching need with authorization, snapshot and external-write invalidation contracts.

**Done when:** No cache is enabled without proven eligibility and invalidation; bounded storage and cross role isolation pass.

Use [PRD-009](../prd/009-query-cache.md#acceptance-criteria) for the feasibility decision. AC-1 is a configuration guard in the [workspace handoff](../implementation-order.md#first-handoff-the-workspace); AC-2 to AC-4 apply if a cache is pursued.

- [ ] Design it (spec): `/architect optional result cache`
