# 4. One real primary path · planned · needs a decision

[Scope index](index.md)

Connect one authenticated TLS client through POOL to one primary, write and read actual data, and close cleanly.

**Done when:** INSERT/SELECT matches direct PostgreSQL, limits and errors work, and an unfinished transaction is rolled back on disconnect.

- [ ] Design it (spec): `/architect primary proxy walking slice`
