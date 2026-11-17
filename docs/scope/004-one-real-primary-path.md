# 4. One real primary path · done

[Scope index](index.md)

Connect one authenticated TLS client through POOL to one primary, write and read actual data, and close cleanly.

**Done when:** INSERT/SELECT matches direct PostgreSQL, limits and errors work, and an unfinished transaction is rolled back on disconnect.

Use [PRD-001](../prd/001-core-proxy.md#acceptance-criteria): deliver AC-1 and the first supported cases of AC-3/4 on a dedicated backend. The protocol owner pins the startup profile, TLS/authentication path, refusal behavior and bounded limits before code. Implement the minimum durable writer check from 003 here before dispatching application SQL. Use the [walking fixture](../testing/strategy.md#first-walking-slice-acceptance); a successful query alone does not prove teardown or denial.

- [x] Design it (spec): `/architect primary proxy walking slice`
- [x] Build it: `/develop one real primary path`
  - [x] Guards, workspace shape and dependencies: Build plan steps 1 to 3 (`AC-1` shape, `AC-2`, `AC-3`, `AC-4`, `0003 AC-19`), step 3 needs manual step M1 (the license decision)
  - [x] Configuration and wire format: steps 4 and 5 (`AC-5` to `AC-8`, `AC-28`), step 5 can run beside the milestone above, step 4 after its step 3
  - [x] Authority on disk and the first contact with PostgreSQL: steps 6 to 8 (`AC-9` to `AC-12`, `AC-26`, `0003 AC-17`, `0003 AC-18`), needs the two milestones above
  - [x] The session path: steps 9 to 11 (`AC-13` to `AC-25`), step 9 after step 7, steps 10 and 11 need the milestone above
  - [x] Fuzzing, documents and evidence: steps 12 to 14 (`AC-27`, evidence for `AC-1` to `AC-28`), after the milestone above; manual steps M2 and M3 follow step 12
- [x] Verify it: `/check verify one real primary path`
- [x] Test it: `/test one real primary path`
- [x] Review it (fresh model): `/check review one real primary path`
- [x] Document it: `/document one real primary path`

Spec [0004](../specs/0004-one-real-primary-path/index.md) · code in `tools/repo-check/`, `scripts/prove-checks.sh`, `crates/pool-protocol/`, `crates/pool-core/`, `crates/pool-config/`, `crates/pool-control/`, `apps/pool/`, `scripts/check.sh`, `scripts/integration/`, `crates/pool-protocol/fuzz/`.
