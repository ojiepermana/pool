# 7. Restricted replica sessions · planned · needs a decision

[Scope index](index.md)

Add role and lag evidence to automatically select replicas for configured read only profiles.

**Done when:** Eligible replica sessions stay pinned, ordinary sessions remain primary, and unsafe/unknown SQL cannot bypass the profile.

Use [PRD-003](../prd/003-sql-routing.md#acceptance-criteria), [PRD-004](../prd/004-load-balancing.md#acceptance-criteria) and the eligibility inputs in [PRD-005 AC-1 to AC-3](../prd/005-health-monitoring.md#acceptance-criteria). The routing and database owners settle the trusted read profile, freshness/lag limits and weight semantics. Prove login selection and later refusals separately; the promotion case of PRD-003 AC-2 is exercised in 010.

- [ ] Design it (spec): `/architect restricted replica sessions`
