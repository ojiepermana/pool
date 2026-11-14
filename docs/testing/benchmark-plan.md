# Benchmark plan

## Method

Compare direct PostgreSQL, POOL, Pgpool-II, PgBouncer and PgCat in equivalent profiles. Include Odyssey where it answers the concurrency question. Pin releases, source/image hashes, compiler flags, effective config, client driver, protocol, prepared statement behavior, TLS/authentication, dataset and connection budgets. Keep primary/replica topology and workload identical within each comparison.

Separate session forwarding, reconnect reuse, transaction pooling references, and read routing into different experiments. POOL V1 is not a transaction pooler. Reporting a transaction multiplexer's TPS beside a session pool with different backend occupancy is not an equivalent performance result.

Use pgbench for PostgreSQL data traffic and a purpose built connection/protocol workload generator for cancellation, pipeline, COPY and idle clients. Use k6 for the HTTP administration API only when needed. Criterion measures codec and route microbenchmarks; it cannot establish application TPS or end to end latency.

## Workload matrix

| Dimension | Lab levels |
|---|---|
| Connected clients | 1, 32, 128, 512, 1,000; increase only within resource budgets |
| Active queries | 1, 8, 32, 64; hold database backend budgets equal |
| Transactions | Read only, mixed pgbench, short writes, long transaction, uncertain COMMIT failure |
| Protocol | Simple, extended named/unnamed, pipeline and COPY |
| Data sizes | Small indexed queries, larger rows and streaming payloads |
| Replica state | Healthy, paused replay, lag threshold crossing, unavailable |
| Security | Verified TLS/SCRAM always for production profile; plaintext isolated lab run labelled separately |

Provisional environment: 4 vCPU/8 GiB proxy, 4 vCPU/16 GiB database VMs and a separate generator. Record actual CPU model, host/NUMA placement, memory, kernel, PostgreSQL settings, storage, network bandwidth/RTT, frequency scaling and Proxmox overcommit. Shared host contention is part of the report, not hidden noise.

Warm for 60 seconds, measure at least 5 minutes, repeat at least five times in varied order, and preserve raw results. Separate cold/warm connection runs and cold/warm data cache. Report median and spread or confidence intervals, latency distributions and completed versus failed/censored requests. Coordinated omission and closed loop client load must be described.

## KPIs and proposed gates

| KPI | Measurement | Proposed criterion |
|---|---|---|
| Connections per second | Completed authenticated startups per second | Report cold and reused paths, no promised absolute value |
| Client/backend utilization | Connected/active/idle/waiting and backend budgets | Match configured caps and expected session semantics |
| TPS/QPS | Successfully completed units per second | Compare only equivalent modes and transaction mixes |
| p50/p95/p99 latency | Client wall clock, separate acquire/connect/query | Publish distributions; initial investigation trigger is over 10 percent TPS loss or over 1 ms p99 added LAN latency on small queries |
| Routing overhead | Same workload with routing disabled/enabled | Profile classification and selection separately; no assumed speedup |
| Memory per connection | RSS delta by idle and active client tier plus allocator data | Bounded growth, no unexplained post-drain retention; absolute budget set from measured baseline |
| CPU | Proxy cores, cycles/query and server CPU | Attribute TLS, parse, relay and telemetry cost |
| Error rate | Expected overload versus unexpected failures | Zero unexpected protocol/identity/safety errors in qualification corpus |
| Recovery time | Fault to eligibility restoration and operator action to new admissions | Publish distributions and operator/fence delay separately; no universal RTO |
| Failover safety | Writer histories and operation evidence | Zero unfenced promotions or silent transaction replays in the declared fault corpus |

The numeric overhead triggers are investigation goals for the lab, not published performance claims or proof of superiority. Recalibrate after the first baseline. Correctness gates are mandatory even if performance exceeds expectations. Attach CPU/memory profiles, configs and raw data to every published result.
