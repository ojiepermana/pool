# 13. Checked online recovery · planned · needs a decision

[Scope index](index.md)

Coordinate PostgreSQL rewind or base backup with explicit target checks and quarantine.

**Done when:** Failed recovery stays detached and a rebuilt former primary joins only as a validated replica.

Reopen with a separate recovery spec and the [recovery boundary](../architecture/failover-recovery.md#recovery). V1 can quarantine a former primary and validate an externally rebuilt replica without automating its rebuild.

- [ ] Design it (spec): `/architect checked online recovery`
