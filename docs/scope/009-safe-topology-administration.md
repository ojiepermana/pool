# 9. Safe topology administration · planned · needs a decision

[Scope index](index.md)

Expose authenticated CLI/API reads and generation checked mutations using the durable operation service.

**Done when:** Role denials, idempotency, audit and restart reconciliation are proven for attach, detach, reload and maintenance.

Use [PRD-006](../prd/006-cluster-management.md#acceptance-criteria) and [PRD-010](../prd/010-administration.md#acceptance-criteria). The control owner specifies role/action permissions, generation comparisons and the retry window. Test a lost response and a reused key with a changed payload. PRD-010 AC-1 also applies when 010 adds promotion.

- [ ] Design it (spec): `/architect safe topology administration`
