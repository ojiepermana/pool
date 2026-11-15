# Control plane

The initial control module observes configured nodes, validates configuration, publishes topology, serves administrative reads, and journals explicit operations. It has one writer authority inside one daemon. It is not a distributed controller yet.

## Proposed records

| Record | Required values and relationships | Persistence |
|---|---|---|
| Cluster | Stable ID, expected PostgreSQL system identifier, configuration generation | Config file; one cluster has many backends |
| Backend | Stable ID, cluster ID, host, port, expected role, TLS server name, weight and configured maintenance policy | Config file |
| Observation | Backend ID, monotonic sample time, wall timestamp, health, observed role, system ID, timeline, receive/replay LSN, freshness, error class | Memory, bounded diagnostics history |
| TopologySnapshot | Generation, eligible primary or none, eligible replica IDs, observation expiry, policy version | Memory, rebuilt on startup |
| ApprovedTopology | Cluster ID unique, approved writer ID, durable epoch, quarantined backend IDs, transition operation ID | SQLite, bootstrap once from explicit configuration |
| Operator membership decisions | Maintenance, drain or detach operation affecting a configured node, with the generation at which it took effect | Reconcile durable decisions on restart before admitting a lease |
| Operation | UUID, caller, kind, target, expected generation, plan hash, idempotency key, phase, timestamps, outcome | Local SQLite journal |
| FenceReceipt | Operation ID, node infrastructure identity, fencing action ID, independent verification, expiry, verifier identity | Journal, references one operation |
| AuditEvent | Sequence, operation ID if any, actor, action, target, result, time, redacted detail | Journal and export; no SQL or secrets |

[Spec 0003](../specs/0003-control-and-session-data-model/index.md#records) now owns the contract of each record: its owner, source, storage and restart rule. It calls the configured server a node, so the Backend row here is its Node. These records are a proposed target, not a confirmed migration. Foreign keys and uniqueness would protect operation keys and backend IDs. Actual PostgreSQL role is always a fresh observation. Approved writer identity is a durable authorization decision; neither source substitutes for the other. Admission requires both to agree.

On first initialization, bootstrap approved writer/epoch from explicit validated configuration after identity checks. Subsequent promotions update the durable approved writer, epoch and quarantine transactionally with the completed operation record. Startup first loads that authority, reconciles incomplete external effects, then obtains fresh role/identity evidence before publishing eligibility. If configuration removes or contradicts durable authority, keep affected admission closed until an explicit generation checked reconciliation. Never infer the authorized writer solely from `pg_is_in_recovery() = false`.

## Value sourcing

| Produced value | Source |
|---|---|
| Backend role | Authenticated `pg_is_in_recovery()` observation |
| Authorized writer, epoch and quarantine | ApprovedTopology in SQLite, bootstrapped once from configured intent and changed through explicit operations |
| Backend login and read profile | Operator configured map from verified frontend principal and requested database to permitted cluster/database/login/credential reference and generation |
| Cluster identity and timeline | Dedicated identity/control probe with explicitly granted privileges or replication IDENTIFY_SYSTEM profile |
| Lag bytes | Primary flush LSN minus comparable replica replay LSN on the same system and timeline history |
| Observation age | Local monotonic clock; wall time is display only |
| Membership and expected role | Configuration defines candidate nodes and expected roles. An operator's durable maintenance, detach or quarantine decision can restrict admission further. |
| Operator identity | Local authenticated transport or mTLS certificate to role mapping |
| Promotion authorization | Bound operation plan, caller approval, fresh verified fence receipt |
| Data loss bound | Final old primary flush evidence after a verified barrier across all writer paths; otherwise unknown and explicitly acknowledged |
| Operation result | Durable journal plus reconciliation against current PostgreSQL observations |

Use a dedicated SQLite writer with WAL, synchronous FULL, checkpoint limits, migrations, backup/retention policy, and file permissions. Provisional audit retention is 30 days or 1 GiB, whichever is reached first, with an operator export option. Preserve active operations and every decision still restricting admission. If durability or storage is unavailable, mutations fail closed.

[Spec 0003](../specs/0003-control-and-session-data-model/index.md#the-four-walks) answers the questions of this paragraph and the next, with [Generations](../specs/0003-control-and-session-data-model/index.md#generations) for what each mutation compares. Before implementing [the control model](../scope/003-control-and-session-data-model.md), specify how an operator initializes a new cluster once, how startup handles missing or unreadable authority, and how a deliberate restore enters reconciliation. An empty directory alone cannot authorize a new bootstrap. Exercise this with a backup taken before a promotion. On restore, keep write admission closed until the operator reconciles the authority, quarantine and outstanding external effects against independent evidence.

The same spec defines the generation each mutation compares, what advances it and what survives restart. A configuration reload, a probe sample and a change of approved writer can happen independently. The API must identify the version its action compares and reject a precondition invalidated by an intervening change. The [administration PRD](../prd/010-administration.md) owns caller retry behavior.

Future multiple proxies need a single operation authority and ordered topology epochs. Local SQLite files are not merged into a consensus system. See [HA PRD](../prd/008-high-availability.md).
