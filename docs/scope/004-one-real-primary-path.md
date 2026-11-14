# 4. One real primary path · planned · needs a decision

[Scope index](index.md)

Connect one authenticated TLS client through POOL to one primary, write and read actual data, and close cleanly.

**Done when:** INSERT/SELECT matches direct PostgreSQL, limits and errors work, and an unfinished transaction is rolled back on disconnect.

Use [PRD-001](../prd/001-core-proxy.md#acceptance-criteria): deliver AC-1 and the first supported cases of AC-3/4 on a dedicated backend. The protocol owner pins the startup profile, TLS/authentication path, refusal behavior and bounded limits before code. Implement the minimum durable writer check from 003 here before dispatching application SQL. Use the [walking fixture](../testing/strategy.md#first-walking-slice-acceptance); a successful query alone does not prove teardown or denial.

- [ ] Design it (spec): `/architect primary proxy walking slice`
