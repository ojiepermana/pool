# Competitor analysis

**Evidence date:** 2026-11-14. Facts below come from project documentation and repositories. Performance comparisons are proposed experiments, not measured results.

| Project | Architecture and concurrency | Lifecycle and capabilities | Administration and security | Limits and license |
|---|---|---|---|---|
| Pgpool-II 4.7.3 | C, parent and child processes; Watchdog for proxy coordination | Cached session connections, SQL routing, weighted read balancing, PostgreSQL replication monitoring, failover and recovery hooks, optional cache | Configuration files, PCP, SQL status commands, HBA, SCRAM and TLS | Broad scope and mode specific restrictions. Actual permissive license text requires retained notices. |
| PgBouncer | C, event driven networking, normally one process instance | Session, transaction, and statement pooling; server reuse by pool identity | INI style configuration, administrative database and SHOW commands, TLS, password and SCRAM support | Pooling specialist, no built in SQL read/write splitting or PostgreSQL promotion manager. Transaction mode restricts session features. ISC. |
| PgCat | Rust and Tokio, multicore asynchronous proxy | Session and transaction pools, read/write splitting, replicas, health and backend selection; upstream also describes experimental sharding and mirroring | TOML configuration, administrative database, Prometheus HTTP endpoint, TLS and authentication | Validate each feature at an exact release. Replica failover does not imply fenced PostgreSQL promotion. MIT. |
| Odyssey | C, multithreaded asynchronous routing with globally managed pools | Database and user rules, session and transaction pooling, configurable routing/storage and prepared statement handling | Configuration rules, limits, TLS and multiple authentication methods, connection correlation | Linux focus and more complex operational configuration. No assumption of Pgpool style cluster orchestration. BSD 3 Clause. |
| HAProxy | C, event driven threaded general TCP/HTTP proxy | Backend connections and health checks at transport level, weighted balancing and endpoint HA | Mature configuration, stats, ACL and TLS interfaces | No PostgreSQL session reuse or SQL semantics. Main code GPL 2 or later, some headers LGPL; use as an external program, not a default source donor. |

Sources: [Pgpool](https://www.pgpool.net/docs/4.7/en/html/), [PgBouncer usage](https://www.pgbouncer.org/usage), [PgBouncer configuration](https://github.com/pgbouncer/pgbouncer/blob/master/doc/config.md), [PgBouncer copyright](https://github.com/pgbouncer/pgbouncer/blob/master/COPYRIGHT), [PgCat upstream](https://github.com/postgresml/pgcat), [Odyssey upstream](https://github.com/yandex/odyssey), [Odyssey pooling](https://yandex.github.io/odyssey/features/pooling.html), [HAProxy upstream](https://github.com/haproxy/haproxy/).

## Comparisons that matter

You can compare PgBouncer and POOL on connection startup, session reuse, queueing, memory, and simple forwarding with equivalent pool modes. You can compare Pgpool and PgCat on a separate routing workload. A transaction pooling result cannot establish that a session pooler is slower at an equivalent task when the number of backends differs.

PgBouncer documents protocol prepared statement tracking through `max_prepared_statements`. That does not make every SQL PREPARE or session feature transaction safe. PgCat's feature list is not proof that arbitrary SQL is safe to route. Odyssey's thread model is not proof of better multicore efficiency. Measure each proposition at a pinned release and configuration.

## Configuration and maintenance

Record binary versions, build flags, effective configuration, release tags, source hashes, and license manifests with each comparison. The rolling upstream branches used for research are not benchmark pins. Official release activity and current documentation are maintenance signals, not promises of support. Odyssey's release listing showed stable 1.5.2 and a 1.6 release candidate; recheck before selecting a comparison binary.

## Recommendation

Use Pgpool as the feature reference, PgBouncer as the pooling efficiency reference, PgCat as the Rust routing reference, and Odyssey as a concurrency and pool lifecycle reference. Use HAProxy for endpoint availability comparisons only. Reuse protocols, lessons, and test cases under their license terms; do not transplant a whole implementation without tracing its ownership assumptions.
