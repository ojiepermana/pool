# Clean session reuse rationale

## Context

The current primary path authenticates and checks one new backend for each session and always drops it. PoolKey omits real startup values and generations are constants. Reuse therefore needs ownership, invalidation and cleanup as one boundary. The scope selects Tracer Bullet and GA depth. This is an enhancement to the existing Rust backend workspace, without UI, hosting or persistence changes.

## Options considered

Keep dedicated sessions, which is simplest but does not meet PRD-002 AC-1. Reuse every backend after DISCARD ALL, which offers breadth but cannot prove extension cleanup. Add a conservative pool owner and retire any session outside a qualified built in state profile, which costs reuse opportunities but supplies one provable path without a SQL parser dependency. A separate pool service would add a process and failure boundary without helping this slice.

## Rationale

Choose the conservative owner. DISCARD ALL restores documented PostgreSQL resources but does not prove arbitrary extension or user code state. A sticky eligibility recognizer keeps unknown work on the existing dedicated path. Exact startup isolation and a coherent generation snapshot close two holes before handoff. Permits follow physical ownership so cleanup and idle sockets cannot escape the budget.

## Design completeness and delegated decisions

Decision mode is auto from agent-flow's default, not an invented user answer. Requirements, lifecycle, exact grammar and proof limits, keys, retention, interfaces and value sources, auth and tenant isolation, stack, errors, timeouts, performance bounds, telemetry and protected file rotation are settled in index.md. Existing frontend/backend SCRAM and TLS are inferred from spec 0004. UI, SEO, external services, database migration and new dependencies are not applicable. All runtime state is memory only. Reconfiguration of TOML is outside the target.

The recommendation is a bounded synchronous pool mutex with no await, rather than an owner task with request channels, because existing sessions already own connector futures and cancellation-safe owned values. The recommendation is no direct socket2 dependency, because the slow write deadline supplies the required behavior; kernel keepalive remains documented. The recommendation is credential-only SIGHUP, rather than adding the scope 009 administration/reload surface. Content ratification waits for spec cross check. New specs remain Proposed before building.

## References

Project: scope 005, PRD-002, architecture/connection-pooling.md, spec 0001 safety contracts, spec 0004 Follow-up 3, 13, 14 and 16, engineering standards, protocol analysis and SEC-8/11/12/14. The scout read the actual lifecycle, key, config and test harness. The Leader checked PostgreSQL 18 docs on 2026-11-16 for reset semantics.

* [DISCARD](https://www.postgresql.org/docs/18/sql-discard.html), reset resource list and transaction restriction.
* [RESET](https://www.postgresql.org/docs/18/sql-reset.html), defaults and configuration changes.
* [Message flow](https://www.postgresql.org/docs/18/protocol-flow.html), synchronization and asynchronous traffic.

## Cross check

A fresh context Terra/high worker read the spec and returned seven major decision gaps. The Leader applied recommendations under auto policy: add IdentityId to credential isolation; charge all dedicated physical backends to the same caps; enumerate the baseline and greeting policy; qualify the division operator; make eviction release the exhausted cap; specify notification ordering through DISCARD plus a subsequent barrier and negative race tests; and bound owned SCRAM jobs process wide and per credential. The worker wrote nothing. Exact Leader author family is not exposed, so cross-family independence remains unverified rather than claimed. The second pass found cap 1 eviction and handoff linearization gaps; the Leader added matching checkout before eviction and a shared runtime lock plus short route read guard for handoff/idle publication. The completed design is accepted under the user delegated auto policy. This is technical acceptance, not a delivery pass.

The conservative profile needs visibility to preload settings. This can require pg_read_all_settings; without it, the login stays dedicated. That operator decision is documented rather than silently granting database permissions. Disposable tests explicitly grant the profile permission and TEMP and prove the no-permission fallback.

A code-level design check found signed literal operator resolution under a role path that explicitly puts an attacker schema before pg_catalog. The Leader chose four exact safe baseline paths and dedicated fallback for every other path, with a targeted regression. Internal catalog comparisons are qualified as well. This is a conservative eligibility bound, not SQL rejection.

Real PostgreSQL 18 integration initially passed 33 of 36 cases, with three PID reuse failures. A filtered reproduction exposed search_path in the actual startup ParameterStatus set. The initial allowlist omitted it and correctly fell back to dedicated execution. The profile now requires that observed field and compares it with baseline column 8. The actual positive evidence, rather than the scripted startup list, owns this correction.
