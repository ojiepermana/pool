# Review, scope005 tooling, 2026-11-16

**Reviewed by**: gpt-6.1-terra/high (author configured as gpt-6.1-sol/high; exact underlying release execution metadata unavailable)
**Scope**: 18 files, uncommitted against 2dbc037
**Verdict**: Approve

## Summary
The tooling adds resolved standard-library path checks, lockfile-based dependency guards, and checks for native build configuration, private keys, CI filtering and notices. The two major findings from the initial review are resolved: test lint exceptions are now scoped to individual statements, and the key scan considers every PEM begin marker on a line. Focused negative cases cover both fixes.

## Strengths
- The test exceptions no longer disable thread restrictions across entire integration test crates.
- The key scanner considers later markers even after an earlier non-private or incomplete marker; the regression includes a binary prefix.
- The core/control exclusion remains enforced independently of the editable dependency baseline.

## Test coverage
The repo-check suite passed 110 unit and 111 integration tests, and `scripts/check.sh clippy` passed during re-review. An initial suite run failed because the prior review report itself contained a literal private-key marker; after rewriting the report, the suite passed. The author-provided focused mutation log reports passing baselines and rejection of grouped thread creation in each of the three integration test roots, a grouped socket case, and the later private-key marker. The full proof script was not run here. Covered files: pool-control journal and three integration tests; `scripts/prove-checks.sh`; repo-check `ci.rs`, `main.rs`, `text.rs`, `tree.rs`, `workspace.rs`, `hardening.rs` and `tests/repo_check/workspace.rs`; the five workspace `clippy.toml` files; and `baseline/native-library-versions.txt`.
