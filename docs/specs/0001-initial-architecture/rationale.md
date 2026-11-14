# Initial architecture rationale

## Context

POOL is intended to carry arbitrary PostgreSQL application traffic and eventually manage failure sensitive infrastructure. Incorrect routing, session reuse, or promotion can damage data even when the Rust program has no memory safety defect. Your first environment has one proxy VM and three PostgreSQL VMs. No existing implementation, source files, manifests, Git history, or project conventions were present on 2026-11-14.

The full Pgpool feature set is larger than a safe first release. Your initial scenario still requires health, replica lag, routing, diagnostics, and manual failover. The roadmap therefore distinguishes a core proxy MVP from an operational V1 rather than quietly removing those requirements.

## Options considered

| Option | Benefit | Cost |
|---|---|---|
| Modular Rust daemon and CLI, recommended | Small deployment surface and direct socket ownership | Control and data share process failures; extraction requires stable contracts |
| Proxy and controller services from the beginning | Fault isolation and independent release cadence | Requires authentication, availability, versioning, and reconciliation across processes before the first proxy works |
| Build orchestration around existing poolers | Fastest path to an operating cluster | Does not fulfill the goal of a new Rust protocol and routing implementation; still a useful operational comparison |
| Reproduce Pgpool process and feature architecture | Familiar operational model and clearer compatibility ambition | Imports legacy constraints and a large parity burden before correctness is established |

The recommendation follows your deployment size and the practice of proving a thin working path before adding distributed coordination. Separate services remain appropriate when a measured isolation or ownership requirement outweighs their operating cost. (basis: your brief, vertical delivery, explicit failure domains)

## Rationale

The protocol engine, authentication, and session cleanup dominate the initial correctness risk. They benefit from explicit state and narrow ownership. A large set of small crates would make interfaces rigid before this behavior is understood. A single daemon makes observations and configuration publication simpler; it does not make promotion safe by itself.

The syntax parser choice favors PostgreSQL 18 grammar coverage over a pure Rust build. Its native code must be isolated, bounded, reviewed, and fuzzed. The raw parser cannot resolve functions, rules, views, or operators against the live catalog. Replica eligibility therefore requires a declared application contract and backend enforcement in addition to syntax. (basis: PostgreSQL parser and hot standby documentation)

Session pooling is the first safe mode because it preserves a real backend session throughout a client connection. Its scaling benefit is smaller than transaction pooling. That limitation is accepted explicitly rather than hidden in a pooling performance claim.

## References

The user requested verified sources, so references are included. The research check date is 2026-11-14. Registry versions are a dated snapshot, not a claim that a compatible dependency graph has been compiled.

| Basis | Verified source |
|---|---|
| Pgpool functional baseline | [Pgpool 4.7 manual](https://www.pgpool.net/docs/4.7/en/html/) |
| Protocol and cancellation | [PostgreSQL 18 protocol](https://www.postgresql.org/docs/18/protocol.html) |
| Parser semantics limit | [PostgreSQL parser stage](https://www.postgresql.org/docs/18/parser-stage.html) |
| Replica semantics | [PostgreSQL hot standby](https://www.postgresql.org/docs/18/hot-standby.html) |
| Fencing requirement | [PostgreSQL failover](https://www.postgresql.org/docs/18/warm-standby-failover.html) |
| Compiler choice | [Rust 1.100.0 announcement](https://blog.rust-lang.org/2026/11/12/Rust-1.100.0/) |
| Native parser | [pg_query upstream](https://github.com/pganalyze/pg_query.rs) |
| Future consensus | [etcd guarantees](https://etcd.io/docs/v3.6/learning/api_guarantees/) |

The requested human research views live in `docs/research/`. They include the feature inventory, competitor evidence, and dependency evaluation. This location follows your explicit requested deliverable structure.

## Independent critique

A different model reviewed the draft without fetching its reference links or changing files. It found gaps in durable writer authority, login selection versus later SQL admission, lag semantics and cancellation delivery. It also requested explicit credential mapping, initial acquisition retry boundaries, crate type ownership and an all-writer barrier for known WAL evidence. You approved all seven recommended resolutions. They are recorded in the [decision ledger](../../plan/decision-ledger.md) and incorporated into the contracts. This is design evidence only; no runtime behavior has been tested.
