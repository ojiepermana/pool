# 10. Fenced manual promotion · planned · needs a decision

[Scope index](index.md)

Let an authorized operator review and execute one fenced promotion with explicit loss boundaries.

**Done when:** No promotion occurs without verified fencing and stale/crashed operations reconcile without replay.

Use [PRD-007](../prd/007-manual-failover.md#acceptance-criteria), the candidate retirement case of [PRD-003 AC-2](../prd/003-sql-routing.md#acceptance-criteria), and [PRD-010 AC-1](../prd/010-administration.md#acceptance-criteria) for promotion access and retries. The database and infrastructure reviewers must settle the provider evidence, all-writer barrier and loss disclosure for the actual lab. The [operation boundary](../architecture/failover-recovery.md#operation-boundary) defines where rollback ends.

- [ ] Design it (spec): `/architect fenced manual promotion`
