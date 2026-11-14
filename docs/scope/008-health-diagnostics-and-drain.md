# 8. Health diagnostics and drain · planned · needs a decision

[Scope index](index.md)

Thicken both traffic paths with freshness, flapping control, maintenance and bounded drain diagnostics.

**Done when:** Stale or wrong-role nodes are excluded, drain is observable and no health failure promotes a database.

Use [PRD-005](../prd/005-health-monitoring.md#acceptance-criteria) and the [cluster lifecycle](../architecture/cluster-management.md). The operations owner defines deadlines, hysteresis and drain outcomes for active sessions. Exercise a stalled probe publisher and drain under load; reuse the freshness checks from 007.

- [ ] Design it (spec): `/architect health diagnostics and drain`
