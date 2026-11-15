# 0003. Control and session data model

**Date**: 2026-11-15
**Status**: In Progress

## Summary

This spec fixes every record POOL keeps about identity, sessions, configuration, observations and durable authority: who owns it, where its values come from, where it lives and what happens to it at restart. The rule that carries the most weight is that durable authority (which node may take writes) and observed database state (what a probe just saw) are two separate things, and writes are admitted only when the configuration, the journal and a fresh observation all agree.

The build of this slice is small and runs without a listener, a database or an administration API. You get the configuration keys for clusters, nodes and the identity map, checked by `pool check-config`, and a new library, `pool-control`, that holds the authority rules as plain functions with tests for clean startup, a missing journal, conflicting role evidence and a restored backup. The journal file itself, the bootstrap command and the probe are specified here and built in scope 004. The full operation journal and its API belong to scope 009.

## Requirements

These criteria cover scope 003 only. Cite them as `0003 AC-n`. Fixture names refer to [Fixtures](#fixtures). Reason names refer to [Writer admission](#writer-admission) and [Bootstrap](#bootstrap).

**User stories**:

- As an operator, I want to write my clusters, nodes and identity map in one file and have it checked at my desk, so that a wrong reference or a duplicate is found before POOL starts.
- As an operator, I want POOL to stay closed when it cannot tell which node may take writes, so that a restart, a lost journal or a restored backup never sends a write to the wrong node (`R-19`).
- As the engineer who builds scope 004, I want the authority rules as tested functions and the journal as a fixed format, so that the first real write path only has to connect them.
- As a reviewer, I want one table that names the owner, source, storage and restart rule of every record, so that no later slice invents a second source for the same fact.

**Acceptance criteria**:

- **AC-1** (new keys are accepted): Each `valid-topology*` fixture exits 0 in both formats with exactly the valid report of spec 0001, byte for byte. Every fixture of spec 0001 keeps its result. A file with `config_version = 1` alone is still valid.
- **AC-2** (one mistake, one finding): Each `invalid-topology-*` fixture except `invalid-topology-multiple.toml` exits 1 with exactly one finding, with the code, key, line and column listed under [Fixtures](#fixtures). Each row of the table under [Which code a bad value gets](#which-code-a-bad-value-gets) gives the listed code.
- **AC-3** (whole configuration, `SEC-12`): `invalid-topology-multiple.toml` reports its five findings in one run, in the listed order. `pool_config::validate` is still the only way to obtain a `ValidatedConfig`.
- **AC-4** (no file content in output, `SEC-4`, `SEC-11`): For `secret-topology-values.toml` and `valid-topology-names.toml`, in both formats, the text `CANARY-9f2c7b1e` appears in neither stdout nor stderr. It does not appear in the `Debug` or `Display` text of the `Rejection`, nor in the `Debug` text of the `ValidatedConfig`. `validate` opens no file: a `credential_file` or `state_directory` that does not exist is still valid.
- **AC-5** (identity map, `0001 SC-3`, `SEC-2`): `ValidatedConfig::resolve_identity` returns the route for an exact pair of principal and database, and `None` for every other pair: another database, a name that differs only in letter case, a prefix, an empty name. There is no wildcard and no default entry. This is the library half of the unmapped case of `PRD-001 AC-1`; scope 004 proves the refusal on a real connection.
- **AC-6** (identifier and generation types): Each identifier type refuses text outside its syntax and compares exactly. `TopologyEpoch`, `AuthorityRevision` and `StartSequence` can only be read, compared and advanced by one. Advancing past 9223372036854775807 returns `None` and never wraps. Comparing two different kinds, such as a `NodeId` with a `ClusterId` or a `TopologyEpoch` with an `AuthorityRevision`, does not compile.
- **AC-7** (clean startup): With no journal and no `control.authority_id`, `classify_startup` returns `Uninitialized` and `writer_admission` returns `Closed` with `authority.uninitialized`, also when every node is observed fresh and healthy. `plan_bootstrap` returns a plan only when every configured node of every cluster has a fresh observation that fits the configuration; each row of the refusal table under [Bootstrap](#bootstrap) is refused with the listed reason. A journal whose identity the configuration does not name is `Unconfirmed` and closed.
- **AC-8** (missing journal): With `control.authority_id` set and no journal, the state is `Missing`. With a journal that cannot be read, the state is `Unreadable`. With a journal of another identity, the state is `Mismatch`. In all three, admission is closed with the listed reason and `plan_bootstrap` refuses with `bootstrap.witness_present` or `bootstrap.store_present`, whatever the observations say.
- **AC-9** (conflicting role evidence, `0001 SC-8`, `PRD-006 AC-2`, `PRD-005 AC-1`): Every row of the table under [Walk 3](#walk-3-conflicting-role-evidence) gives the listed result and reasons. Only full agreement opens. A result that reports a second primary or a later timeline asks for the conflict latch, and a journal whose latch is set stays closed with `authority.conflict_latched` when the evidence is gone.
- **AC-10** (restore, `PRD-006 AC-3`): A journal whose hold is `restore_pending` is `Held`, and admission is closed with `authority.restore_pending` whatever the observations say. Every row of the table under [Walk 4](#walk-4-restore-of-a-backup-taken-before-a-promotion) gives the listed result.
- **AC-11** (freshness, `PRD-005 AC-3`): An observation whose age is equal to or greater than the expiry, or whose sample time is later than the time given as now, counts as no evidence. `pool-control` never reads a clock: `grep -rnE 'Instant::now|SystemTime|std::fs|std::env' crates/pool-control/src` prints nothing.
- **AC-12** (authority is never derived from observation): Over the whole small domain described under [Tests and who owns them](#tests-and-who-owns-them), `writer_admission` returns `Open` only for the node that is at once the approved writer in the journal and the `expected_writer` in the configuration, and never when a listed conflict is present. Code outside `pool-control` cannot build an `Authority`, an `EligiblePrimary`, a `Closed` or a `BootstrapPlan` by itself: a struct literal of each does not compile.
- **AC-13** (workspace and guards, `0002 AC-6`): `cargo metadata --no-deps --format-version 1` lists exactly three workspace members, `pool`, `pool-config` and `pool-control`. `pool-control` depends on `pool-config` and on nothing else. `Cargo.lock` holds the three workspace packages and the same sixteen third party packages as before. `scripts/check.sh all` and `scripts/prove-checks.sh` pass with the changed baseline files. The addition to rule W4 has one passing and one failing test.
- **AC-14** (records and conventions): Every record that the table under [Records](#records) marks as built in this slice exists as a public type at the named place, and its documentation comment states its restart rule. This half is checked by reading; the verification record lists each type and the line of its comment. The row "Identifier and generation types" of the [shared conventions](../../engineering-standards.md#shared-conventions) names the rule and links to [Identifier and generation types](#identifier-and-generation-types).
- **AC-15** (validation time at the file limit): Made inputs 1, 2 and 3 under [Validation time](#validation-time) have the listed number of entries and bytes, and `pool_config::validate` returns for each within 10 seconds in the debug build that `cargo test` makes. Inputs 1 and 2 are valid. Input 3 gives exactly one finding per entry, `config.invalid_value` at `node.port`, in file order, all on line 2, each at the column where the value of its entry starts in the text the test built; the first is column 60. The built `pool check-config` exits 1 for input 3 in each of the two formats, each run within 10 seconds from start to exit, and reports the same number of findings as there are entries. Made input 4 gives its two findings in the listed order, which is against file order, with the listed lines and columns.

| Criterion | Text of scope 003 it answers |
|---|---|
| AC-14, and the [Records](#records) table | Each record has an owner, lifecycle, source and restart rule |
| AC-9, AC-12 | Durable authority is distinct from observed database state |
| AC-7, AC-8, AC-10 | How first bootstrap differs from lost or restored authority. The walk through of clean startup, missing journal and restore |
| AC-6, and [Generations](#generations) | Which generations survive restart, what a mutation compares and what advances it |
| AC-1 to AC-5 | How the verified principal maps to the actual backend login, credential reference and routing profile |
| AC-9, AC-11 | The walk through of conflicting role evidence |
| AC-13 | Spec 0002 Follow-up 10 and "Rules for later slices" |

`AC-15` answers no sentence of scope 003. It was added on 2026-11-15, after the verification of milestone 2 measured a valid file at the limit at 12 seconds in a release build and 241 seconds in a debug build. It makes the 1 MiB limit of spec 0001 a bound on time as well as on size.

`SEC-14` (independent review) is met by the scope box "Review it (fresh model)". This slice changes authority rules, so the review needs the Control engineer role of the [reviewer roster](../../engineering-standards.md#reviewer-roster), which a fresh model may hold for code.

## Decision

**Chosen option**: Build the decisions as plain, tested code now, and leave everything that touches a disk, a socket or a database to the slice that first needs it.

1. `pool-config` gains the keys for `[control]`, `[[cluster]]`, `[[node]]` and `[[identity]]`, their validation, the identifier and generation types, and the identity map lookup.
2. A third workspace crate, `crates/pool-control`, holds the authority model: how a start is classified, when a bootstrap may happen, and when writes are admitted. It reads no file, no clock and no socket. Its tests are the four walks of this spec.
3. The journal (one SQLite file), the bootstrap command, the probe and the wiring of admission into a session are specified here and built in scope 004. The operation records, the backup, the reconcile operation and the API are specified here as rules and built in scope 009. Fence receipts belong to scope 010.
4. No third party dependency is added. SQLite is adopted in scope 004, with its own review of native code and licenses.

The decisions marked heavy in the [rationale](rationale.md#decisions-taken) are public contracts or formats on disk. Changing one later means a new decision in this spec.

## What this slice builds and where it stops

| Thing | Scope 003 (this build) | Later |
|---|---|---|
| Keys `[control]`, `[[cluster]]`, `[[node]]`, `[[identity]]` and their findings | Built in `pool-config`, shown by `pool check-config` | TLS, probe, limit and listener keys: 004. Weight and replica keys: 007 |
| Identifier types, three durable generation types | Built in `pool-config` | The memory generations are created by the slice that first uses each one, under the rules here |
| Identity map lookup | Built: `ValidatedConfig::resolve_identity` | Called at login: 004 |
| Reading the secret behind `credential_file`, and its permission check (`SEC-4`) | Not built. The key is checked for form only | 004 |
| Authority model: start classification, bootstrap plan, writer admission | Built in `pool-control` as functions with tests | Called by the daemon: 004 |
| Journal file, schema version 1, start sequence | Specified under [The journal](#the-journal) | Built: 004, with the adoption of SQLite |
| Bootstrap command | Rules under [Bootstrap](#bootstrap) | Command syntax and build: 004 |
| Probe that produces observations | Record fixed here | Built: 004 (role, identity, timeline). Lag: 007. History: 008 |
| Session, lease, pool key, cancellation records | Contract rows under [Records](#records) | 004, 005, 006 |
| Operation, audit event, membership decision, backup, reconcile | Contract rows and rules | 009 |
| Fence receipt, change of approved writer | Contract rows and rules | 010 |

Not in this build: any listener, any connection to PostgreSQL, SQLite, a thread, an async runtime, a new command, a change to the output of `pool check-config` for a valid file, and any log event. The forbidden source patterns of rule W3 stay as they are.

## Records

One row per record. "Plane" says who reads it at run time. A record never has two sources: where two columns could disagree, the Source column wins.

| Record | Owner and plane | Lifecycle | Source of its values | Stored in | Restart rule | Built in |
|---|---|---|---|---|---|---|
| Cluster | `pool-config` (`ClusterConfig`). Read by both planes | Made by one validated load. Never changed. Replaced whole by the next configuration generation | The configuration file | Configuration file | Read and validated again. Nothing is remembered from the last run | 003 |
| Node | `pool-config` (`NodeConfig`). Read by both planes | Same as cluster | The configuration file | Configuration file | Same as cluster | 003 |
| Identity route (one entry of the identity map) | `pool-config` (`IdentityRoute`). Read by the data plane at login | Same as cluster | The configuration file. Never a startup parameter (`0001 SC-3`) | Configuration file | Same as cluster | 003 |
| Secret reference | `pool-config` (`SecretReference`) | Same as cluster | The key `credential_file` | Configuration file | Same as cluster | 003 |
| Credential (the secret itself) | `pool-core`, data plane. A wrapper type that prints `[REDACTED]` and cannot be serialized | Read from the protected file at a configuration load. Dropped when its credential generation retires | The file the secret reference names | Memory only | Read again. Its credential generation starts over | 004 |
| Validated configuration | `pool-config` (`ValidatedConfig`). The daemon gives it a configuration generation when it publishes it | Made by `validate`. Published whole or not at all (`SEC-12`) | The configuration file | Memory | Read again. The configuration generation starts at 1 | 003 for the type, 004 for the number |
| Verified principal | `pool-core`, data plane | Made when frontend authentication succeeds. Ends with the session | The authentication exchange. Scope 004 says how each method names the principal | Memory | Gone. The client logs in again | 004 |
| Session | `pool-core`, data plane. Fields as in the data structures of PRD-001 | Made at accept. Ends at disconnect or error | The frontend connection, the verified principal and the identity route | Memory | Gone. Clients reconnect (`0001 SC-5`) | 004 |
| Lease | `pool-core`, data plane | Made at initial admission. Pinned to one frontend until the session ends (`0001 SC-1`). Never moved | Session, eligible primary, pool key | Memory | Gone. No lease, backend or lease generation survives | 004 |
| Pool key (backend identity, `0001 SC-2`) | `pool-core`, data plane | Computed at initial admission. Compared again at handoff | See [What goes into a backend identity](#what-goes-into-a-backend-identity) | Memory | Gone | 004 computes it, 005 partitions reuse by it |
| Virtual cancellation credential | `pool-core`, data plane (`0001 SC-9`) | Made per lease. Revoked before release | Random bytes made by POOL. The real backend key stays private | Memory only. Never on disk, never in telemetry | Gone. A cancel request after a restart matches nothing | 006 |
| Observation | `pool-control` (`Observation`), control plane | One probe sample of one node. Replaced by the next sample of that node | The probe. Role is `pg_is_in_recovery()`. Age is the local monotonic clock | Memory. A bounded history comes with 008 | None survives. Admission stays closed until fresh observations exist | 003 for the type, 004 for the probe |
| Topology snapshot | `pool-control` builds it. The daemon maps it into the route view of `pool-core` | Immutable. A new one replaces it after a change of configuration, authority or observation | Authority, configuration and fresh observations, through `writer_admission` | Memory | Rebuilt. The snapshot generation starts at 1 | 004 |
| Journal identity (the `store` row) | `pool-control`, control plane, one writer | Made once by bootstrap. Changed only inside a transaction. Never deleted by the daemon | Bootstrap: random bytes for the authority ID. Later: the daemon and operations | SQLite | Read first at every start, without writing, and classified ([Start classification](#start-classification)) | 003 for the model, 004 for the file |
| Approved topology (approved writer, members, conflict latch) | `pool-control`, control plane | One row per cluster, plus its member rows. Made by bootstrap. The writer and the members change only by a reviewed operation. The conflict latch is set by the daemon when it sees a second primary or a later timeline, and cleared only by an operation | Bootstrap: `expected_writer`, the configured nodes and matching observations. Later: a promotion (010), an attach or detach, or a reconcile (009). An observation alone can add the latch, which only restricts. It can never set or change the writer | SQLite | Loaded, then compared with the configuration and with fresh observations before any write is admitted (`0001 SC-8`) | 003 for the model, 004 for the file |
| Quarantine | `pool-control`, control plane | Set by a promotion or an operator. Cleared only by an operation with rejoin evidence | An operation. A healthy probe cannot clear it (`PRD-006 AC-3`) | SQLite | Loaded with the approved topology. Still in force | 003 for the model, 010 for the first writer |
| Membership decision (maintenance, drain, detach) | `pool-control`, control plane | Set and cleared by operations | An operation. A healthy probe cannot clear it | SQLite | Loaded before any lease is admitted. Still in force | 008 and 009 |
| Operation | `pool-control`, control plane | See [Operation records](#operation-records) | The authenticated caller and the operation plan | SQLite | A row in a phase that is not final is reconciled before the clusters it names admit anything (`SEC-7`) | 009. Promotion phases: 010 |
| Fence receipt | `pool-control`, control plane | Belongs to one operation | The fencing adapter and its independent check | SQLite | Kept with its operation | 010 |
| Audit event | `pool-control`, control plane | Appended. Never changed. Removed only by the retention rule | Operations. No SQL and no secret | SQLite, plus export | Kept | 009 |

A configured PostgreSQL server is called a node. A backend is one connection to a node, as the [glossary](../../glossary.md) already says. The record that the [control plane](../../architecture/control-plane.md#proposed-records) page calls Backend is the Node of this table.

### What goes into a backend identity

`0001 SC-2` lists the parts. This is where each part comes from. Two backends with a different value in any row are never the same pool key.

| Part | Source |
|---|---|
| Cluster | `IdentityRoute::cluster()` |
| Endpoint | The node ID in the route view of `pool-core`, which the daemon fills from `EligiblePrimary::node()` (for a replica profile, from the chosen replica). The host and port are read from that node in the same configuration generation |
| Database | `IdentityRoute::backend_database()` |
| Actual login role | `IdentityRoute::backend_login()` |
| Credential generation | The generation of the credential read through `IdentityRoute::credential()` |
| Topology epoch | The epoch in the route view of `pool-core`, which the daemon fills from `EligiblePrimary::topology_epoch()`. `pool-core` never sees a `pool-control` type |
| TLS policy | The TLS keys of the node, added in 004 |
| Startup profile | Fixed by 004 for the first path, extended by 005 |

The configuration generation is not a part of the key. It is checked at handoff: a backend that finished its handshake under a retired generation is closed (`PRD-002 AC-4`).

### What may appear in default telemetry

| May appear in a log field, a metric label or default diagnostics | May appear in a log field or default diagnostics, never as a metric label | Never appears by default |
|---|---|---|
| Cluster ID, node ID, identity ID, routing profile, authority state, admission reason names, observed role (in recovery or not) | System identifier, authority ID, timeline ID, every generation | Principal, requested database, backend database, backend login, the path in `credential_file`, host, port, the content of any secret file, cancellation credentials, SQL |

The identity ID exists for this purpose: it is the name you choose for one mapping, so that a log line can say which mapping was used without naming a login. This table is the permission that the [log rules](../0001-initial-architecture/index.md#log-events-and-redaction) of spec 0001 ask for before text from a file may go into a named field. The output of `pool check-config` stays as spec 0001 defines it and never prints any of these.

The middle column holds values that are safe to show and wrong as a label. Default diagnostics includes the `Debug` text of every type of `pool-config` and `pool-control` that holds one of them, and so the `Debug` text of `ValidatedConfig`.

- **System identifier** (configured or observed) and **authority ID**. Neither is a secret, and knowing either gives access to nothing. PostgreSQL returns the system identifier from `pg_control_system()` and to a replication client that sends `IDENTIFY_SYSTEM`. The bootstrap command prints the authority ID, and the status read of 009 may show it. The system identifier does name one cluster for its whole life, and that is the reason it belongs in a log: when admission closes because a node reports another system identifier, you need both numbers to see which server you reached. As a label each would only be a second name for the cluster, which the cluster ID already labels. Both have a syntax that cannot carry other text from the file: digits only for the one, and 32 characters from `0` to `9` and `a` to `f` for the other.
- **Timeline ID** and **every generation**. They are numbers that only grow, so as label values they have no bound (`T-12`). Show one as the value of a metric or as a log field.

The observed role has two values and holds no text, so it may be a label.

## Identifier and generation types

This is the shared convention for identifiers and generations. Later slices add types under these rules and do not choose again.

1. Every identifier and every generation is its own type that wraps one value. Two different kinds never compare with each other and never convert into each other.
2. An identifier is built only by a function that checks its syntax. There is no `From<String>` and no `Default`.
3. A generation offers three things: read the number, compare, and advance by one. It has no other arithmetic. Advancing at the largest value returns `None`, and the caller fails closed.
4. Types that more than one crate needs live in `pool-config`. A type that only one crate needs lives in that crate.
5. A generation that lives in memory is only comparable inside one start sequence. Code that keeps or sends such a number keeps the start sequence with it.

Identifier syntax:

| Type | Crate | Syntax |
|---|---|---|
| `ClusterId`, `NodeId`, `IdentityId` | `pool-config` | 1 to 63 bytes. The first is `a` to `z`. The rest are `a` to `z`, `0` to `9`, `_` or `-` |
| `SystemIdentifier` | `pool-config` | The PostgreSQL system identifier: a whole number from 1 to 18446744073709551615. In text it is decimal digits with no leading zero |
| `AuthorityId` | `pool-config` | 16 bytes. In text it is exactly 32 characters from `0` to `9` and `a` to `f` |
| `TimelineId` | `pool-control` | The PostgreSQL timeline: a whole number from 1 to 4294967295 |

```rust
// pool-config, the same shape for ClusterId, NodeId and IdentityId
pub struct ClusterId(/* private */);
impl ClusterId {
    pub fn parse(text: &str) -> Option<ClusterId>;
    pub fn as_str(&self) -> &str;
}

pub struct SystemIdentifier(/* private */);
impl SystemIdentifier {
    pub fn parse(text: &str) -> Option<SystemIdentifier>;
    pub fn get(self) -> u64;
}

pub struct AuthorityId(/* private */);
impl AuthorityId {
    pub fn parse(text: &str) -> Option<AuthorityId>;
    pub fn from_bytes(bytes: [u8; 16]) -> AuthorityId;
    pub fn as_bytes(&self) -> &[u8; 16];
}

// pool-config, the same shape for TopologyEpoch, AuthorityRevision and StartSequence
pub struct TopologyEpoch(/* private */);
impl TopologyEpoch {
    pub const FIRST: TopologyEpoch;
    pub fn from_stored(value: u64) -> Option<TopologyEpoch>;
    pub fn get(self) -> u64;
    pub fn checked_next(self) -> Option<TopologyEpoch>;
}
```

Every type in this block derives `Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash`. `SystemIdentifier`, `AuthorityId` and the three generation types also derive `Copy`. `Display` is the identifier text for the three ID types, the decimal number for `SystemIdentifier` and for the generation types, and the 32 characters for `AuthorityId`. None of them implements `PartialEq` or `PartialOrd` with another type. `FIRST` is 1 for `TopologyEpoch` and `AuthorityRevision` and 0 for `StartSequence`. `from_stored` returns `None` for a value below `FIRST` or above 9223372036854775807, because that is the largest whole number SQLite stores. `checked_next` returns `None` at that value.

## Generations

| Generation | Type and home | Starts at | What advances it | Survives restart | Who compares it |
|---|---|---|---|---|---|
| Topology epoch | `TopologyEpoch`, `pool-config`. One per cluster | 1, at bootstrap | A change of the approved writer of that cluster, in the same transaction (010). A reconcile that names another writer (009) | Yes, in the journal | The pool key. A mutation that changes the approved writer |
| Authority revision | `AuthorityRevision`, `pool-config`. One per journal | 1, at bootstrap | Every transaction that changes the approved topology, its members, the conflict latch, the quarantine, a membership decision or the hold | Yes, in the journal | Every mutation of durable authority names the revision it expects |
| Start sequence | `StartSequence`, `pool-config`. One per journal | 0, at bootstrap | Each start of the daemon that finds its journal `Ready`, before anything is admitted | Yes, in the journal | Whoever holds a memory generation from before a restart |
| Configuration generation | A type created in 004, in `pool-config` | 1, at each start | Each validated configuration that the daemon publishes. A rejected reload does not advance it (`PRD-010 AC-2`) | No | The handoff check of a backend. A reload request names the generation it expects |
| Credential generation | A type created in 004, in `pool-config`. One per distinct `credential_file` path | 1, when the file is first read in this run | A load that reads different bytes from the file, or an explicit revocation. The trigger before a reload route exists is named by 005 | No | The pool key |
| Snapshot generation | A type created in 004, in `pool-control` | 1, at each start | Each published topology snapshot | No | Diagnostics, and a reader that wants to know if its view is current |
| Lease generation | A type created in 004, in `pool-core` | 1, at each start | Each new lease | No | Cancellation and late completions (`0001 SC-9`) |
| Observation | No number. An observation carries its sample time | Not applicable | Each probe sample replaces the last one of that node | No | Nothing compares it by number. Evidence is read again, fresh, when it is needed |

What a mutation compares. Every durable mutation is one SQLite transaction that starts with `BEGIN IMMEDIATE`, reads the compared value again inside the transaction, and writes nothing when it differs. The caller then gets a conflict and must read the state again (`PRD-006 AC-1`).

| Mutation | Built in | Precondition it names | What it advances |
|---|---|---|---|
| Bootstrap | 004 | No journal file and no `control.authority_id` | Creates revision 1, start sequence 0 and epoch 1 for each cluster |
| Daemon start | 004 | The start was classified `Ready` | Start sequence |
| Set the conflict latch | 004 | None. It only restricts | Authority revision |
| Configuration reload | 009 | Start sequence and configuration generation | Configuration generation, and the credential generation of each secret whose bytes changed |
| Maintenance, drain, detach, attach, quarantine change, a cluster added to the journal | 008 and 009 | Authority ID and authority revision | Authority revision |
| Reconcile, after a restore or to clear a conflict latch | 009 | Authority ID and authority revision | Authority revision. The topology epoch, when the writer changes. After a restore also a new authority ID |
| Change of approved writer | 010 | Authority ID and the topology epoch of that cluster. The promotion plan may bind more | Topology epoch and authority revision |

A probe sample is not a mutation and invalidates nothing by itself. A mutation that needs evidence (bootstrap, attach, reconcile, promotion) reads fresh observations when it executes and fails when they no longer fit. So a reload, a probe sample and a change of approved writer can happen in any order, and each request still says exactly which version it was written against.

## Configuration

All keys follow the [configuration rules](../0001-initial-architecture/index.md#configuration-rules) of spec 0001. Every new key is optional at the top level, so `config_version` stays `1` and every file that was valid before is still valid.

### Keys

| Key | Type | Required | Default | Validation | Reload |
|---|---|---|---|---|---|
| `control.state_directory` | string | Yes, when the file has at least one `[[cluster]]` entry | none | Path syntax | Restart only |
| `control.authority_id` | string | no | absent | `AuthorityId` syntax | Restart only |
| `cluster[].id` | string | yes | none | `ClusterId` syntax. Unique among clusters | Not applicable until a reload exists |
| `cluster[].system_identifier` | string | yes | none | `SystemIdentifier` syntax. A string, because the value can be larger than a TOML integer | Same |
| `cluster[].expected_writer` | string | yes | none | `NodeId` syntax. Names a node whose `cluster` is this cluster | Same |
| `node[].id` | string | yes | none | `NodeId` syntax. Unique among all nodes of the file | Same |
| `node[].cluster` | string | yes | none | `ClusterId` syntax. Names a cluster | Same |
| `node[].host` | string | yes | none | 1 to 253 bytes, each an ASCII letter, a digit, `.`, `-` or `:` | Same |
| `node[].port` | integer | yes | none | 1 to 65535 | Same |
| `identity[].id` | string | yes | none | `IdentityId` syntax. Unique among identities | Same |
| `identity[].principal` | string | yes | none | Name syntax | Same |
| `identity[].database` | string | yes | none | Name syntax. The pair of `principal` and `database` is unique | Same |
| `identity[].cluster` | string | yes | none | `ClusterId` syntax. Names a cluster | Same |
| `identity[].backend_database` | string | no | The value of `database` | Name syntax | Same |
| `identity[].backend_login` | string | yes | none | Name syntax | Same |
| `identity[].credential_file` | string | yes | none | Path syntax. A secret reference. The file is not opened in this build | Same |
| `identity[].routing_profile` | string | no | `"primary"` | `"primary"`. `"replica"` is refused until scope 007 | Same |

- **Name syntax**: 1 to 63 bytes of UTF-8 with no control character (U+0000 to U+001F, and U+007F). Names are compared byte for byte. Nothing is folded to lowercase and nothing is trimmed.
- **Path syntax**: 1 to 4096 bytes, starts with `/`, no control character.
- No key holds a secret. `credential_file` is the first secret reference in the schema. What the file contains, how it is read and the permission check are decided and built in scope 004.
- `expected_writer` is your statement of which node is the writer. It seeds the approved writer at bootstrap. After that it must keep agreeing with the journal, or writes stay closed ([Writer admission](#writer-admission)).
- `authority_id` is the authority witness: the identity of the journal this configuration belongs to. You copy it from the output of the bootstrap command. Its presence says that a journal was created once.
- Two nodes with the same host (compared without regard to ASCII letter case) and port are refused. Two names for one server would make that server look like a second primary and close writes for no reason. The check catches the honest mistake only: a name and an address for one server pass it, and the result is then a closed admission, never an open one.
- The 1 MiB limit on the file is the bound on the number of entries, and it has to bound the time as well. The rule for the whole of `validate`, for a valid file and for an invalid one, is under [Validation time](#validation-time), with the check of `AC-15`. For the uniqueness and reference checks it means: sort or use a map.

### Findings

New codes, in the family of [findings](../0001-initial-architecture/index.md#findings) of spec 0001. All exit 1.

| Code | When | Message |
|---|---|---|
| `config.invalid_value` | A value has the right type but not the allowed syntax or range | `a value is not in the allowed form or range` |
| `config.duplicate_id` | An `id` repeats an earlier `id` of the same table | `an id is used more than once` |
| `config.unknown_reference` | A key names a cluster or node that the file does not define, or `expected_writer` names a node of another cluster | `a reference does not name a matching entry` |
| `config.duplicate_endpoint` | A node has the host and port of an earlier node | `two nodes have the same host and port` |
| `config.duplicate_identity` | An identity has the principal and database of an earlier identity | `two identity entries have the same principal and database` |
| `config.unsupported_value` | A value is known but not supported by this build (`routing_profile = "replica"`) | `this value is not supported by this build` |

`config.missing_key` keeps its meaning and is now also used inside entries.

A finding inside a repeated table carries a key and a position. That is a fourth location, next to the three of spec 0001:

```rust
pub enum Location {
    None,
    Key(&'static str),
    Position { line: u32, column: u32 },
    KeyAt { key: &'static str, line: u32, column: u32 },   // new
}
```

- The key is a constant path without an index: `cluster.id`, `cluster.system_identifier`, `cluster.expected_writer`, `node.id`, `node.cluster`, `node.host`, `node.port`, `identity.id`, `identity.principal`, `identity.database`, `identity.cluster`, `identity.backend_database`, `identity.backend_login`, `identity.credential_file`, `identity.routing_profile`.
- The position is where the value starts. For a missing key it is where the entry starts: the first `[` of its `[[...]]` header, or the `{` of an inline table. Take both from the span that `toml::Spanned` gives, counted as the [validation order](../0001-initial-architecture/index.md#validation-order) of spec 0001 counts lines and columns. How often the text may be walked to get them is fixed under [Validation time](#validation-time).
- A finding about `control.state_directory` or `control.authority_id` uses `Location::Key`, because there is only one `[control]` table.
- Human finding line: `<code> at <key>, line <line>, column <column>: <message>`. The `Display` text of such a `Finding` is this line.
- JSON finding object: the same five fields as before, with `key`, `line` and `column` all set. `output_version` stays `1`, because no field is removed or renamed.

```text
config.duplicate_id at node.id, line 18, column 6: an id is used more than once
```

```json
{"output_version":1,"command":"check-config","result":"invalid","config_version":null,"capabilities":null,"findings":[{"code":"config.duplicate_id","key":"node.id","line":18,"column":6,"message":"an id is used more than once"}]}
```

The report for a valid file does not change. It shows no cluster, node or identity, and no count of them.

### Validation order

Steps 1 to 4 of the [validation order](../0001-initial-architecture/index.md#validation-order) of spec 0001 stay as they are. In step 4 the private structures gain the new tables. Each one that stands for a table is written by hand and accepts a map only, as spec 0001 requires. Strings are read as `Option<Spanned<String>>`, `port` as `Option<Spanned<u16>>`, and each repeated table as `Option<Vec<Spanned<...>>>`. Step 5 collects every finding, in this order:

1. The three checks of spec 0001: `config_version`, `result_cache.enabled`, `automatic_promotion.enabled`.
2. `control.state_directory`: `config.missing_key` when it is absent and the file has at least one `[[cluster]]` entry, `config.invalid_value` when its syntax is wrong. Then `control.authority_id`: `config.invalid_value` when its syntax is wrong.
3. Each cluster in file order, its keys in the order `id`, `system_identifier`, `expected_writer`. A missing required key gives `config.missing_key`, a wrong syntax gives `config.invalid_value`.
4. Each node in file order, keys in the order `id`, `cluster`, `host`, `port`. A port of 0 gives `config.invalid_value`.
5. Each identity in file order, keys in the order `id`, `principal`, `database`, `cluster`, `backend_database`, `backend_login`, `credential_file`, `routing_profile`. For `routing_profile`, `"replica"` gives `config.unsupported_value` and any other text but `"primary"` gives `config.invalid_value`.
6. Cross checks, in this order. Each one walks its entries in file order and reads only values that passed steps 3 to 5.
   1. `config.duplicate_id` for clusters, then nodes, then identities, at the `id` of the second and each later entry.
   2. `config.unknown_reference` at `node.cluster`. Skipped as a whole when any cluster has a missing or invalid `id`.
   3. `config.unknown_reference` at `cluster.expected_writer`. Skipped as a whole when any node has a missing or invalid `id` or `cluster`, or when check 2 found something. A cluster whose own `id` is missing or invalid is passed over alone, and the check still runs for every other cluster: the check compares the `cluster` of the named node with that `id`, and the first sentence of this step lets it read only values that passed.
   4. `config.unknown_reference` at `identity.cluster`. Skipped as a whole when any cluster has a missing or invalid `id`.
   5. `config.duplicate_endpoint` at `node.host` of the second and each later node.
   6. `config.duplicate_identity` at `identity.principal` of the second and each later identity.

The skips keep one mistake from producing two findings. Fixing it can then reveal the next one. When an `id` appears twice, a reference resolves to the first entry.

### Which code a bad value gets

| Input | Code |
|---|---|
| `[node]`, `[cluster]` or `[identity]` written as a table and not as an array of tables; an entry that is not a table, such as `node = [1]`; `control` that is not a table | `config.schema` |
| A key that is not in the schema inside `[control]` or inside an entry, such as `weight = 1` in a node | `config.schema` |
| A string key given a number, or `port` given a string; `port = 70000`; `port = -1` | `config.schema` |
| `system_identifier` given as a TOML integer | `config.schema` |
| `port = 0` | `config.invalid_value` at `node.port` |
| An `id` that is empty, has 64 bytes, starts with a digit, or has an uppercase letter | `config.invalid_value` |
| `system_identifier` of `"0"`, `"007"`, `""`, `"18446744073709551616"` or text with a letter | `config.invalid_value`. `"18446744073709551615"` is valid |
| `authority_id` with 31 or 33 characters, or with an uppercase letter | `config.invalid_value` at `control.authority_id` |
| `state_directory` or `credential_file` that does not start with `/` | `config.invalid_value` |
| `principal`, `database`, `backend_database` or `backend_login` that is empty, has 64 bytes, or holds a line feed | `config.invalid_value` |
| `host` with a space or an underscore | `config.invalid_value` at `node.host` |
| `routing_profile = "Primary"` | `config.invalid_value` |
| `routing_profile = "replica"` | `config.unsupported_value` |
| `cluster = []`, `node = []`, `identity = []`, or an empty `[control]` in a file with no cluster | Valid |

### Public boundary added to pool-config

```rust
impl ValidatedConfig {
    pub fn control(&self) -> Option<&ControlConfig>;
    pub fn clusters(&self) -> &[ClusterConfig];
    pub fn nodes(&self) -> &[NodeConfig];
    pub fn identities(&self) -> &[IdentityRoute];
    pub fn resolve_identity(&self, principal: &str, database: &str) -> Option<&IdentityRoute>;
}

impl ControlConfig {
    pub fn state_directory(&self) -> Option<&std::path::Path>;
    pub fn authority_id(&self) -> Option<AuthorityId>;
}
impl ClusterConfig {
    pub fn id(&self) -> &ClusterId;
    pub fn system_identifier(&self) -> SystemIdentifier;
    pub fn expected_writer(&self) -> &NodeId;
}
impl NodeConfig {
    pub fn id(&self) -> &NodeId;
    pub fn cluster(&self) -> &ClusterId;
    pub fn host(&self) -> &str;
    pub fn port(&self) -> u16;
}
impl IdentityRoute {
    pub fn id(&self) -> &IdentityId;
    pub fn cluster(&self) -> &ClusterId;
    pub fn backend_database(&self) -> &str;
    pub fn backend_login(&self) -> &str;
    pub fn credential(&self) -> &SecretReference;
    pub fn routing_profile(&self) -> RoutingProfile;
}
impl SecretReference { pub fn path(&self) -> &std::path::Path; }

#[non_exhaustive]
pub enum RoutingProfile { Primary }        // as_str() gives "primary"
```

- `control()` is `None` when the file has no `[control]` table. `clusters()`, `nodes()` and `identities()` are in file order.
- `resolve_identity` compares both names byte for byte and must not walk every entry.
- `IdentityRoute` has no accessor for `principal` or `database`. The only way to use them is the lookup.
- `Debug` is written by hand for `ControlConfig`, `NodeConfig`, `IdentityRoute` and `SecretReference`, and prints identifiers and the routing profile only. Paths, host, port, principal, database names and the backend login are left out or shown as `[REDACTED]`. `ValidatedConfig` keeps its derived `Debug`, which is safe only if no field of it prints such text. So whatever private structure serves the lookup (a map keyed by the two names, or a sorted list) is its own private type with a hand written `Debug` that prints the number of entries and nothing else.
- None of these types has a public constructor, `Default` or `Deserialize`. `FindingCode` gains the six new variants with the code and message texts above. Dependencies of `pool-config` stay `serde` and `toml`.

### Fixtures

All in `crates/pool-config/tests/fixtures/`. Every fixture below starts with the same fifteen lines, called the base, unless it says otherwise:

```text
config_version = 1

[control]
state_directory = "/var/lib/pool"

[[cluster]]
id = "main"
system_identifier = "7412345678901234567"
expected_writer = "pg1"

[[node]]
id = "pg1"
cluster = "main"
host = "10.0.0.11"
port = 5432
```

```text
--- valid-topology-minimal.toml
(the base, and nothing more)

--- valid-topology.toml
config_version = 1

[control]
state_directory = "/var/lib/pool"
authority_id = "0123456789abcdef0123456789abcdef"

[[cluster]]
id = "main"
system_identifier = "7412345678901234567"
expected_writer = "pg1"

[[node]]
id = "pg1"
cluster = "main"
host = "10.0.0.11"
port = 5432

[[node]]
id = "pg2"
cluster = "main"
host = "10.0.0.12"
port = 5432

[[node]]
id = "pg3"
cluster = "main"
host = "pg3.db.internal"
port = 5433

[[identity]]
id = "orders-app"
principal = "orders_app"
database = "orders"
cluster = "main"
backend_login = "orders_rw"
credential_file = "/etc/pool/secrets/orders_rw"

[[identity]]
id = "report-app"
principal = "report_app"
database = "orders"
cluster = "main"
backend_database = "orders_reporting"
backend_login = "report_ro"
credential_file = "/etc/pool/secrets/report_ro"
routing_profile = "primary"

--- valid-topology-names.toml
(the base, an empty line, then)
[[identity]]
id = "canary-app"
principal = "CANARY-9f2c7b1e-not-a-real-secret"
database = "CANARY-9f2c7b1e-not-a-real-secret"
cluster = "main"
backend_login = "CANARY-9f2c7b1e-not-a-real-secret"
credential_file = "/etc/pool/secrets/CANARY-9f2c7b1e-not-a-real-secret"

--- invalid-topology-missing-key.toml
(the base, an empty line, then)
[[identity]]
id = "orders-app"
principal = "orders_app"
database = "orders"
cluster = "main"
credential_file = "/etc/pool/secrets/orders_rw"

--- invalid-topology-missing-state-directory.toml
(the base without its lines 3 to 5, so the file has no [control] table)

--- invalid-topology-invalid-value.toml
(the base with line 12 changed to)
id = "PG1"

--- invalid-topology-duplicate-id.toml
(the base, an empty line, then)
[[node]]
id = "pg1"
cluster = "main"
host = "10.0.0.12"
port = 5432

--- invalid-topology-unknown-reference.toml
(the base with line 9 changed to)
expected_writer = "pg9"

--- invalid-topology-duplicate-endpoint.toml
(the base, an empty line, then)
[[node]]
id = "pg2"
cluster = "main"
host = "10.0.0.11"
port = 5432

--- invalid-topology-duplicate-identity.toml
(the base, an empty line, then)
[[identity]]
id = "orders-app"
principal = "orders_app"
database = "orders"
cluster = "main"
backend_login = "orders_rw"
credential_file = "/etc/pool/secrets/orders_rw"

[[identity]]
id = "orders-app-2"
principal = "orders_app"
database = "orders"
cluster = "main"
backend_login = "orders_ro"
credential_file = "/etc/pool/secrets/orders_ro"

--- invalid-topology-unsupported-profile.toml
(the base, an empty line, then)
[[identity]]
id = "report-app"
principal = "report_app"
database = "orders"
cluster = "main"
backend_login = "report_ro"
credential_file = "/etc/pool/secrets/report_ro"
routing_profile = "replica"

--- invalid-topology-multiple.toml
config_version = 1

[control]
state_directory = "/var/lib/pool"
authority_id = "not-an-authority-id"

[[cluster]]
id = "main"
system_identifier = "0"
expected_writer = "pg1"

[[node]]
id = "pg1"
cluster = "main"
host = "10.0.0.11"
port = 0

[[identity]]
id = "orders-app"
principal = "orders_app"
database = "orders"
cluster = "other"
backend_login = "orders_rw"

--- secret-topology-values.toml
(all fifteen lines of the base, with only line 14 changed to)
host = "CANARY-9f2c7b1e not a real secret"
(then, after line 15, an empty line and)
[[identity]]
id = "orders-app"
principal = "orders_app"
database = "orders"
cluster = "main"
backend_login = "orders_rw"
credential_file = "CANARY-9f2c7b1e-not-a-real-secret"
```

| Fixture | Exit | Findings, in order |
|---|---|---|
| `valid-topology-minimal.toml`, `valid-topology.toml`, `valid-topology-names.toml` | 0 | none |
| `invalid-topology-missing-key.toml` | 1 | `config.missing_key` at `identity.backend_login`, line 17, column 1 |
| `invalid-topology-missing-state-directory.toml` | 1 | `config.missing_key` at `control.state_directory` |
| `invalid-topology-invalid-value.toml` | 1 | `config.invalid_value` at `node.id`, line 12, column 6 |
| `invalid-topology-duplicate-id.toml` | 1 | `config.duplicate_id` at `node.id`, line 18, column 6 |
| `invalid-topology-unknown-reference.toml` | 1 | `config.unknown_reference` at `cluster.expected_writer`, line 9, column 19 |
| `invalid-topology-duplicate-endpoint.toml` | 1 | `config.duplicate_endpoint` at `node.host`, line 20, column 8 |
| `invalid-topology-duplicate-identity.toml` | 1 | `config.duplicate_identity` at `identity.principal`, line 27, column 13 |
| `invalid-topology-unsupported-profile.toml` | 1 | `config.unsupported_value` at `identity.routing_profile`, line 24, column 19 |
| `invalid-topology-multiple.toml` | 1 | `config.invalid_value` at `control.authority_id`; `config.invalid_value` at `cluster.system_identifier`, line 9, column 21; `config.invalid_value` at `node.port`, line 16, column 8; `config.missing_key` at `identity.credential_file`, line 18, column 1; `config.unknown_reference` at `identity.cluster`, line 22, column 11 |
| `secret-topology-values.toml` | 1 | `config.invalid_value` at `node.host`, line 14, column 8; `config.invalid_value` at `identity.credential_file`, line 23, column 19 |

These positions were read from `toml` 1.2.1 with the fixture texts above. Tests compare the line and the column exactly, because a value span starts at the value and an entry span starts at its header. The fixture names keep the pattern of spec 0001, so the [Linux build](../0001-initial-architecture/index.md#linux-build) command needs no change: it expects exit 0 for `valid-*`, exit 1 for the rest, and no canary text in any output. The canary is fake.

### Validation time

Spec 0001 says that parsing a file at the 1 MiB limit twice "costs nothing you would notice". This section makes the same true for everything `validate` does with the new tables.

**The rule.** The work that the code of `pool-config` does in `validate` grows no faster than the size of the file times the logarithm of that size. So sorting is fine, and a loop over the text inside a loop over the entries is not. The rule holds for every file within the limit, valid or not, and it includes the work of turning a span into a line and a column. The `toml` parser and the standard maps are taken as they are: they were measured on the made inputs below and are not what this rule is about. Two things follow:

- Never count from the top of the file, or from the start of a line, once for each entry, each value or each collected finding. The one position of a syntax or schema error is found once per run and is fine as it is.
- A file may hold every entry on one line. So a table of line starts alone is not enough: the column of each finding on that line would still be counted from the start of the line.

One way that meets the rule: while collecting, keep the byte offset of each finding in place of its line and column. After the cross checks (item 6 of step 5 in the [validation order](#validation-order)), sort the offsets, walk the text once from the top and note the line and column at each offset, then fill them into the findings without changing the order of the findings. A valid file then needs no line and no column at all. Any other way with the same bound is as good, for example one cursor that only moves forward through the text.

The lines and columns themselves do not change. They are counted as the [validation order](../0001-initial-architecture/index.md#validation-order) of spec 0001 counts them, and every position under [Fixtures](#fixtures) stays as listed. The report of `pool check-config` is written in one pass over the findings.

**The made inputs.** Four inputs are built by the tests at run time and are not committed, like the made inputs of spec 0001: in memory in the `pool-config` tests, and under `env!("CARGO_TARGET_TMPDIR")` with a file name of its own in the `apps/pool` test. Every line ends with a line feed, also the last. In inputs 1 to 3, `<i>` counts from 0 and each input takes as many entries as fit: stop before the entry that would make the whole file longer than 1048576 bytes.

```text
--- made input 1, many lines
config_version = 1

[control]
state_directory = "/var/lib/pool"

[[cluster]]
id = "main"
system_identifier = "7412345678901234567"
expected_writer = "n0"
(then, for each entry, an empty line and)
[[node]]
id = "n<i>"
cluster = "main"
host = "h<i>.db.internal"
port = 5432

--- made input 2, one line
(line 1 is)
config_version = 1
(line 2 is the text "node = [", then the entries with one "," between two of them, then "]")
(then lines 2 to 9 of made input 1)
(each entry is this text, with no space in it)
{id="n<i>",cluster="main",host="h<i>.db.internal",port=5432}

--- made input 3, one line, every entry wrong
(made input 2 with port=0 in place of port=5432 in every entry)

--- made input 4, findings against file order
(the base of the fixtures, an empty line, then)
[[node]]
id = "pg1"
cluster = "main"
host = "10.0.0.12"
port = 0
```

| Made input | Entries | Bytes | Result of `validate` |
|---|---|---|---|
| 1 | 13217 | 1048511 | Valid |
| 2 | 16471 | 1048558 | Valid |
| 3 | 17268 | 1048559 | One `config.invalid_value` at `node.port` for each entry, in file order, all on line 2. Entry 0 is at column 60. There is no other finding: the hosts differ, and the endpoint check does not read a node whose port failed |
| 4 | 2 nodes | not at the limit | `config.invalid_value` at `node.port`, line 21, column 8, then `config.duplicate_id` at `node.id`, line 18, column 6 |

The counts and lengths were read from a program that builds inputs 1 to 3 by the rule above, and the result of input 4 from the code of Build plan step 3. The tests compare them. For input 3 the test finds where each `port=` value starts in the text it built and compares the column of every finding with it, not only the first and the last. Through the built binary, input 3 gives `configuration invalid`, then `findings: 17268`, then 17268 finding lines in the human format, and a `findings` array of 17268 objects in the JSON format, each with the code `config.invalid_value` and the key `node.port`. Input 4 is there because every other input and fixture reports its findings in file order, so code that hands out sorted positions in sorted order would pass them all.

**The limit.** Each call of `validate` on input 1, 2 or 3 returns within 10 seconds, and so does each run of the built binary on input 3, measured from the start of the command to its exit. The test measures with `std::time::Instant`. A test may read a clock; `validate` never does. It is a plain `cargo test` test, so it runs the debug build, and it fails only above the limit. It does not compare two timings with each other, because the ratio of two short timings is noise on a busy machine. A test cannot stop a call that runs long (rule W3 allows no thread), so with slow code the test first runs for minutes and then fails. Check the time after each input, so that the first miss ends the test.

**Why 10 seconds.** Measured on the workstation on 2026-11-15 (Apple M1 Max, debug build): with the position counting taken out of the code, inputs 1 to 3 take 0.34 to 0.39 seconds each. With the code of Build plan step 3, which counted from the top of the file for every value, a valid file of 16000 nodes on many lines took 241 seconds ([verification record](verify.md#milestone-2-identifiers-configuration-keys-and-the-finding-line), milestone 2), which is about 165 seconds for the 13217 entries of input 1. 10 seconds lies about 25 times above the first number and about 16 times below the second. A machine several times slower still passes with correct code, and code that counts once per entry again still fails. In a release build the same work takes a few hundredths of a second. If the limit is missed, report it as a finding with the measured time. Do not raise the number in the test to make it pass.

**What the test does not catch.** It catches code that walks the text once per entry, per value or per finding. A slower search that stays under 10 seconds at this size, such as a linear search through a table of line starts for each finding, passes. Review against the rule covers that.

## The authority model in pool-control

`crates/pool-control` is a library with one dependency, `pool-config`. Everything in this section is a plain function over values. The caller reads the journal, the clock and the probes and passes what it found. That is what lets the four walks run as tests today and lets scope 004 connect real inputs without changing a rule.

```rust
pub struct TimelineId(/* private */);        // from_observed(u32) -> Option<TimelineId>, get() -> u32
pub struct MonotonicTime(/* private */);     // from_origin(Duration), a time since a start point the caller chose

pub struct Observation { /* private */ }
impl Observation {
    pub fn new(node: NodeId, sampled_at: MonotonicTime, outcome: ObservationOutcome) -> Observation;
}
pub enum ObservationOutcome { Reached(NodeFacts), Failed }
pub struct NodeFacts {
    pub system_identifier: SystemIdentifier,
    pub timeline: TimelineId,
    pub in_recovery: bool,
}

pub enum StoreProbe { Absent, Unreadable, Present(StoredAuthority) }
pub struct StoredAuthority {
    pub authority_id: AuthorityId,
    pub authority_revision: AuthorityRevision,
    pub start_sequence: StartSequence,
    pub hold: Hold,
    pub clusters: Vec<ApprovedTopology>,
    pub quarantine: Vec<Quarantine>,
}
pub enum Hold { None, RestorePending }
pub struct ApprovedTopology {
    pub cluster: ClusterId,
    pub system_identifier: SystemIdentifier,
    pub approved_writer: NodeId,
    pub writer_timeline: TimelineId,
    pub topology_epoch: TopologyEpoch,
    pub members: Vec<NodeId>,
    pub conflict_latched: bool,
}
pub struct Quarantine { pub cluster: ClusterId, pub node: NodeId }

pub enum AuthorityState { Uninitialized, Unconfirmed, Missing, Unreadable, Mismatch, Held, Ready(Authority) }
pub struct Authority { /* private; only classify_startup makes one */ }

pub fn classify_startup(config: &ValidatedConfig, probe: StoreProbe) -> AuthorityState;

pub fn plan_bootstrap(
    config: &ValidatedConfig,
    probe: &StoreProbe,
    observations: &[Observation],
    now: MonotonicTime,
    expiry: Duration,
) -> Result<BootstrapPlan, BootstrapRefused>;

pub fn writer_admission(
    config: &ValidatedConfig,
    state: &AuthorityState,
    cluster: &ClusterId,
    observations: &[Observation],
    now: MonotonicTime,
    expiry: Duration,
) -> WriterAdmission;

pub enum WriterAdmission { Open(EligiblePrimary), Closed(Closed) }
pub struct EligiblePrimary { /* private */ }  // cluster(), node(), topology_epoch(), authority_revision()
pub struct Closed { /* private; never empty */ }           // reasons() -> &[ClosedReason], latches_conflict() -> bool
pub struct BootstrapPlan { /* private */ }                 // clusters() -> &[ApprovedTopology]
pub struct BootstrapRefused { /* private; never empty */ } // reasons() -> &[BootstrapRefusal]
```

`ClosedReason` and `BootstrapRefusal` are enums with one variant per reason name below and `as_str()` that gives the name. `Authority`, `EligiblePrimary`, `Closed`, `BootstrapPlan` and `BootstrapRefused` have no public constructor. Every public type of this crate derives `Debug, Clone, PartialEq, Eq`. `TimelineId`, `MonotonicTime`, `NodeFacts`, `ObservationOutcome`, `Hold`, `ClosedReason` and `BootstrapRefusal` also derive `Copy`, and `TimelineId` and `MonotonicTime` also `PartialOrd, Ord`. A list of reasons holds each reason at most once, however many nodes cause it. The control plane page lists more fields for an observation (wall time, health, positions, error class); each is added by the slice that first uses it.

`StoredAuthority` has public fields because the journal reader has to fill it. In the product the only code that builds a `StoreProbe` is that reader, which scope 004 places inside `pool-control`. Tests build one by hand.

**Fresh.** For each node the functions use the observation with the latest `sampled_at`. If two observations of one node share that latest time, the node has no evidence. An observation is fresh when `sampled_at` is not later than `now` and `now` minus `sampled_at` is less than `expiry`. An observation that is not fresh, or whose outcome is `Failed`, is no evidence. The functions look at every node that the configuration lists for the cluster, whether it is a member in the journal or not, and at no other node.

### Start classification

`classify_startup` looks at two facts: what the journal reader found, and whether the configuration carries `control.authority_id` (the witness).

| Journal reader found | No witness | Witness |
|---|---|---|
| `Absent`: the state directory holds no journal file | `Uninitialized` | `Missing` |
| `Unreadable`: a file is there but cannot be opened, is not a POOL journal, has a newer schema, or fails its integrity check | `Unreadable` | `Unreadable` |
| `Present`, and the witness is absent | `Unconfirmed` | not applicable |
| `Present`, and the witness differs from its authority ID | not applicable | `Mismatch` |
| `Present`, the witness equals its authority ID, hold is `restore_pending` | not applicable | `Held` |
| `Present`, the witness equals its authority ID, hold is `none` | not applicable | `Ready` |

A `StoredAuthority` that names a cluster twice, names a member twice, has an approved writer that is not among its members, or has a quarantine row for a cluster it does not hold, is treated as `Unreadable`. Only `Ready` carries an `Authority`. So no other state can lead to an open admission, by construction.

### Writer admission

`writer_admission` answers one question for one cluster: may a write go to a node, and to which one. In any state but `Ready` the answer is `Closed` with the one reason of that state, and the observations are not looked at.

| State | Reason |
|---|---|
| `Uninitialized` | `authority.uninitialized` |
| `Unconfirmed` | `authority.unconfirmed` |
| `Missing` | `authority.missing` |
| `Unreadable` | `authority.unreadable` |
| `Mismatch` | `authority.mismatch` |
| `Held` | `authority.restore_pending` |

In `Ready`, every check below runs and every reason that applies is reported, in this order. Admission is `Open` only when none applies. The node of an `Open` result is always the approved writer of the journal.

| Order | Reason | When |
|---|---|---|
| 1 | `authority.cluster_unknown` | The journal has no approved topology for this cluster, or the configuration has no such cluster. Reported alone |
| 2 | `authority.config_conflict` | `expected_writer` in the configuration is not the approved writer, or the configured system identifier is not the one in the journal, or a member in the journal is not a configured node of this cluster |
| 3 | `authority.conflict_latched` | The conflict latch of the cluster is set in the journal |
| 4 | `authority.writer_quarantined` | The approved writer is in the quarantine of its cluster |
| 5 | `evidence.writer_missing` | There is no fresh `Reached` observation of the approved writer. Checks 6 to 8 are then skipped |
| 6 | `evidence.writer_identity` | The approved writer reports another system identifier than the journal. Checks 7 and 8 are then skipped |
| 7 | `evidence.writer_in_recovery` | The approved writer reports that it is in recovery |
| 8 | `evidence.writer_timeline` | The approved writer reports another timeline than the journal recorded |
| 9 | `evidence.second_primary` | A configured node of the cluster other than the approved writer, not quarantined, with a fresh `Reached` observation and the system identifier of the journal, reports that it is not in recovery |
| 10 | `evidence.timeline_ahead` | A configured node of the cluster other than the approved writer, not quarantined, with a fresh `Reached` observation and the system identifier of the journal, reports a timeline greater than the one the journal recorded for the writer. It does not matter whether that node is in recovery |

These rules sit behind the table.

- **Three way agreement.** The configuration, the journal and a fresh observation must name the same writer. The configuration cannot replace the journal (`PRD-006 AC-3`), the observation cannot replace either, and the journal alone is not enough without fresh evidence (`0001 SC-8`).
- **Positive conflict closes, silence about a replica does not.** A second node that says it is a primary, or a node on a later timeline, closes writes. A replica that is down, slow or not yet probed does not, because otherwise one dead replica would stop every write.
- **Quarantine is the reviewed answer.** A quarantined node that reports itself as a primary does not close writes to the approved writer. It was excluded on purpose, and it never becomes eligible by itself.
- **A conflict stays until someone looks at it.** `Closed::latches_conflict()` is true when the reasons hold `evidence.second_primary` or `evidence.timeline_ahead`. The daemon of scope 004 then sets the conflict latch of that cluster in the journal, in one transaction, and also remembers it in memory for the rest of the run, so that a failed write cannot reopen anything. From then on admission is closed with `authority.conflict_latched`, also when the other primary can no longer be seen. Only an operation clears the latch (`PRD-006 AC-2`). An observation can add this restriction. It can never remove one.
- **A member cannot be dropped by editing the file.** The journal lists the members of each cluster. A configuration that no longer lists one of them is a conflict, so taking a node out of the file cannot hide what that node would have reported. Removing a member is the detach operation of 009.

Scope 004 calls this function when a session is admitted and again after the backend is acquired, before the lease is shown to the client. That is "the minimum durable writer check" that scope 004 names. Whether a session that is already running is ended when admission closes later is decided in 004 and 008.

### Bootstrap

Bootstrap is the only way a journal comes into being. It is a command that an operator runs on purpose; the daemon never creates a journal, so an empty directory by itself authorizes nothing. `plan_bootstrap` decides whether the command may write. It covers every configured cluster at once. Every reason that applies is reported, in this order:

| Order | Refusal | When |
|---|---|---|
| 1 | `bootstrap.store_present` | The journal reader found anything but `Absent` |
| 2 | `bootstrap.witness_present` | The configuration carries `control.authority_id`. A journal existed once, so this is lost authority and not a first start |
| 3 | `bootstrap.no_cluster` | The configuration has no cluster |
| 4 | `bootstrap.evidence_missing` | Some node of some cluster has no fresh `Reached` observation |
| 5 | `bootstrap.identity_mismatch` | Some observed node reports another system identifier than its cluster configures |
| 6 | `bootstrap.writer_in_recovery` | The `expected_writer` of some cluster reports that it is in recovery |
| 7 | `bootstrap.second_primary` | A node other than the `expected_writer` reports that it is not in recovery |
| 8 | `bootstrap.timeline_ahead` | A node reports a timeline greater than that of the `expected_writer` of its cluster |

Checks 5 to 8 look only at nodes with a fresh `Reached` observation. Checks 6 to 8 look only at nodes whose system identifier is the configured one. Check 8 is skipped for a cluster whose `expected_writer` is not such a node.

Bootstrap is stricter than admission on purpose: it needs every node, once. When nothing applies, the plan holds one `ApprovedTopology` per cluster: the `expected_writer`, the configured system identifier, the timeline the writer reported, epoch 1, every configured node of the cluster as a member, and no conflict latch. The command of scope 004 then writes the journal in one transaction, with a new random authority ID, revision 1, start sequence 0 and hold `none`, prints the authority ID, and exits. You add the ID to the configuration as `control.authority_id`. Until you do, the state is `Unconfirmed` and nothing is admitted.

### The four walks

Each walk is a test in `crates/pool-control/tests/walkthroughs.rs`. They share one setup: cluster `main` with the nodes `pg1`, `pg2` and `pg3`, system identifier `7412345678901234567`, `expected_writer = "pg1"`, an expiry of 6 seconds and a `now` of 100 seconds. "Fresh" means sampled at 98 seconds. "Healthy" means `pg1` reports timeline 1 and not in recovery, and `pg2` and `pg3` report timeline 1 and in recovery, all with the system identifier `7412345678901234567`. A row that changes the configuration or the journal leaves the observations as they are. `X` and `Y` are two different authority IDs.

#### Walk 1: clean startup

Test `walk_clean_startup`.

| Step | Configuration | Journal reader | Observations | Result |
|---|---|---|---|---|
| 1. First start on a new machine | no witness | `Absent` | fresh, healthy | `Uninitialized`. Admission `Closed`, `authority.uninitialized` |
| 2. The operator runs bootstrap | no witness | `Absent` | fresh, healthy | A plan: `main`, writer `pg1`, timeline 1, epoch 1 |
| 2a. | no witness | `Absent` | `pg3` has no observation | Refused, `bootstrap.evidence_missing` |
| 2b. | no witness | `Absent` | `pg2` not in recovery | Refused, `bootstrap.second_primary` |
| 2c. | no witness | `Absent` | `pg1` in recovery | Refused, `bootstrap.writer_in_recovery` |
| 2d. | no witness | `Absent` | `pg2` reports another system identifier | Refused, `bootstrap.identity_mismatch` |
| 2e. | no witness | `Absent` | `pg2` reports timeline 2 | Refused, `bootstrap.timeline_ahead` |
| 3. Start after bootstrap, configuration not yet updated | no witness | `Present`, ID `X` | fresh, healthy | `Unconfirmed`. `Closed`, `authority.unconfirmed` |
| 4. The operator adds the witness | witness `X` | `Present`, ID `X` | fresh, healthy | `Ready`. `Open`: `main`, `pg1`, epoch 1, revision 1 |

#### Walk 2: missing journal

Test `walk_missing_journal`.

| Step | Configuration | Journal reader | Observations | Result |
|---|---|---|---|---|
| 1. The journal is gone | witness `X` | `Absent` | fresh, healthy | `Missing`. `Closed`, `authority.missing` |
| 1a. Someone tries bootstrap | witness `X` | `Absent` | fresh, healthy | Refused, `bootstrap.witness_present` |
| 2. The file is damaged | witness `X` | `Unreadable` | fresh, healthy | `Unreadable`. `Closed`, `authority.unreadable` |
| 2a. Someone tries bootstrap | witness `X` | `Unreadable` | fresh, healthy | Refused, `bootstrap.store_present` and `bootstrap.witness_present` |
| 3. A journal of another daemon was copied in | witness `X` | `Present`, ID `Y` | fresh, healthy | `Mismatch`. `Closed`, `authority.mismatch` |

There are two ways out, and both are deliberate. The first is to restore a backup, which leads into walk 4. The second is a new bootstrap: you remove the witness from the configuration and the damaged file from the state directory, make sure `expected_writer` names the node that is the primary now, and run bootstrap. It succeeds only with fresh, fitting evidence from every node. If a promotion happened that the configuration does not reflect, the old writer is fenced or in recovery, and the plan is refused with `bootstrap.evidence_missing` or `bootstrap.writer_in_recovery`. A new bootstrap starts a new authority: the quarantine and every membership decision of the lost journal are gone, and you enter them again once scope 009 exists.

#### Walk 3: conflicting role evidence

Test `walk_conflicting_evidence`. The state is `Ready` with the journal holding `main`, writer `pg1`, timeline 1, epoch 1, the members `pg1`, `pg2` and `pg3`, no conflict latch, and the configuration carrying the matching witness. Each row changes only what it names.

| Row | What differs from healthy | Result |
|---|---|---|
| a | nothing | `Open`, `pg1` |
| b | The configuration says `expected_writer = "pg2"` | `Closed`: `authority.config_conflict` |
| c | `pg1` in recovery, `pg2` not in recovery (a promotion nobody recorded) | `Closed`: `evidence.writer_in_recovery`, `evidence.second_primary`. `pg2` is not adopted |
| d | `pg2` not in recovery, `pg1` unchanged | `Closed`: `evidence.second_primary` |
| e | The observation of `pg1` was sampled at 94 seconds, so its age equals the expiry | `Closed`: `evidence.writer_missing` |
| f | The probe of `pg1` failed | `Closed`: `evidence.writer_missing` |
| g | `pg1` reports another system identifier | `Closed`: `evidence.writer_identity` |
| h | `pg1` reports timeline 2 | `Closed`: `evidence.writer_timeline` |
| i | `pg3` reports timeline 2, still in recovery | `Closed`: `evidence.timeline_ahead` |
| j | The probe of `pg2` failed and `pg3` was sampled at 90 seconds | `Open`, `pg1` |
| k | `pg2` is quarantined and reports that it is not in recovery | `Open`, `pg1` |
| l | The configuration gives `main` another system identifier | `Closed`: `authority.config_conflict` |
| m | The journal has no row for `main` | `Closed`: `authority.cluster_unknown` |
| n | `pg1` is in the quarantine | `Closed`: `authority.writer_quarantined` |
| o | The conflict latch of `main` is set in the journal | `Closed`: `authority.conflict_latched` |
| p | The configuration no longer lists `pg3`, which is a member in the journal | `Closed`: `authority.config_conflict` |

`latches_conflict()` is true for rows c, d and i and false for every other closed row. Read rows d and o together: after row d the daemon sets the latch, and when `pg2` then drops out of sight the answer is row o, not row a.

#### Walk 4: restore of a backup taken before a promotion

Test `walk_restore_before_promotion`. The history: bootstrap gave `main`, writer `pg1`, timeline 1, epoch 1. A backup of the journal was taken. A promotion then made `pg2` the writer on timeline 2 with epoch 2 and put `pg1` in quarantine. The journal was lost and the backup was put back, with witness `X` unchanged.

| Step | Journal reader | Configuration and observations | Result |
|---|---|---|---|
| 1. A supported backup is restored | `Present`, ID `X`, hold `restore_pending`, writer `pg1` | Any. Also when `pg1` is up and reports that it is a primary on timeline 1 | `Held`. `Closed`: `authority.restore_pending` |
| 2. A raw copy of the old file is restored, and the configuration was updated to `pg2` | `Present`, ID `X`, hold `none`, writer `pg1` | `expected_writer = "pg2"`; `pg2` not in recovery on timeline 2, `pg1` no observation | `Closed`: `authority.config_conflict`, `evidence.writer_missing`, `evidence.second_primary`, `evidence.timeline_ahead` |
| 3. The same raw copy, and the configuration is old too | same | `expected_writer = "pg1"`; `pg1` no observation (fenced), `pg2` not in recovery on timeline 2 | `Closed`: `evidence.writer_missing`, `evidence.second_primary`, `evidence.timeline_ahead` |
| 4. The same, and `pg1` is up again as a primary while `pg2` cannot be reached | same | `pg1` not in recovery on timeline 1; `pg2` failed; `pg3` in recovery on timeline 2 | `Closed`: `evidence.timeline_ahead` |

Step 1 is the supported path, and it is safe without any help from the observations. A backup made by the supported procedure (built in 009) is a copy of the journal in which the hold is already `restore_pending`. So a journal that came out of a backup can never be taken for the live one. The way out is the reconcile operation of 009: you name the writer, the members and the quarantine as they are now, POOL checks them against fresh observations of every node that is not quarantined, and in one transaction it writes them, advances the authority revision, advances the topology epoch if the writer changed, gives the journal a new authority ID and clears the hold and the conflict latch. You then put the new ID and the right `expected_writer` into the configuration. The new ID makes sure that no request written against the journal before the restore can match afterwards.

Steps 2 to 4 show what the evidence rules still catch when someone restores a raw file copy, which is not a supported backup. Each of the three results asks for the conflict latch. They do not catch everything: see [Consequences](#consequences).

## The journal

Built in scope 004. This section is the format that the model above reads through `StoreProbe`.

- **File**: `journal.sqlite3` in `control.state_directory`, with the two side files SQLite makes in WAL mode. The directory has mode `0700` and the files `0600`, owned by the service account. Scope 004 builds the check and refuses to start otherwise.
- **One journal per daemon.** It holds every configured cluster. Two daemons never share a state directory.
- **Marks**: `PRAGMA application_id = 1347374924` (the bytes `POOL`) and `PRAGMA user_version = 1` (the schema version). A file with another application ID is not a journal.
- **Settings on every open**: `journal_mode = WAL`, `synchronous = FULL`, `foreign_keys = ON`. Checkpoint limits are set by 004.
- **One writer**: one connection writes. Every mutation is one transaction that starts with `BEGIN IMMEDIATE`. The data plane never reads the journal; it reads the topology snapshot.
- **Schema version 1**:

```sql
CREATE TABLE store (
  singleton           INTEGER PRIMARY KEY CHECK (singleton = 1),
  authority_id        BLOB    NOT NULL CHECK (length(authority_id) = 16),
  authority_revision  INTEGER NOT NULL CHECK (authority_revision >= 1),
  start_sequence      INTEGER NOT NULL CHECK (start_sequence >= 0),
  hold                TEXT    NOT NULL CHECK (hold IN ('none', 'restore_pending')),
  created_unix_ms     INTEGER NOT NULL CHECK (created_unix_ms >= 0)
) STRICT;

CREATE TABLE approved_topology (
  cluster_id          TEXT    PRIMARY KEY,
  system_identifier   TEXT    NOT NULL,
  approved_writer     TEXT    NOT NULL,
  writer_timeline     INTEGER NOT NULL CHECK (writer_timeline >= 1),
  topology_epoch      INTEGER NOT NULL CHECK (topology_epoch >= 1),
  conflict_latched    INTEGER NOT NULL CHECK (conflict_latched IN (0, 1)),
  changed_revision    INTEGER NOT NULL CHECK (changed_revision >= 1),
  changed_unix_ms     INTEGER NOT NULL CHECK (changed_unix_ms >= 0)
) STRICT, WITHOUT ROWID;

CREATE TABLE member (
  cluster_id          TEXT    NOT NULL REFERENCES approved_topology (cluster_id),
  node_id             TEXT    NOT NULL,
  changed_revision    INTEGER NOT NULL CHECK (changed_revision >= 1),
  PRIMARY KEY (cluster_id, node_id)
) STRICT, WITHOUT ROWID;

CREATE TABLE quarantine (
  cluster_id          TEXT    NOT NULL REFERENCES approved_topology (cluster_id),
  node_id             TEXT    NOT NULL,
  since_epoch         INTEGER NOT NULL CHECK (since_epoch >= 1),
  changed_revision    INTEGER NOT NULL CHECK (changed_revision >= 1),
  changed_unix_ms     INTEGER NOT NULL CHECK (changed_unix_ms >= 0),
  PRIMARY KEY (cluster_id, node_id)
) STRICT, WITHOUT ROWID;
```

  The system identifier is decimal text, because it can be larger than a SQLite integer. `member` holds the nodes of each cluster, the approved writer among them. `changed_revision` is the authority revision of the transaction that last wrote the row. The two time columns are wall clock time for display only; no rule reads them. These statements were run against SQLite 3.54.0 on 2026-11-15.

- **Migrations**: forward only, numbered, one transaction each. A binary can read every schema version from 1 up to its own. A file with a version above what the binary knows is `Unreadable`. A migration that fails leaves the version as it was and the daemon closed. Going back to an older binary means restoring a backup, which enters walk 4. Later slices add tables by migration: membership decisions (008 or 009), operations and audit events (009), fence receipts (010).
- **Start**: the daemon first reads the journal without writing to it and classifies the start. Only when the state is `Ready` does it write: it applies the migrations between the file's version and its own, then advances `start_sequence` by one in a transaction. If either write fails, nothing is admitted. In every other state the daemon writes nothing to the file, so it never changes a journal that belongs to another daemon, is not confirmed, or is held. Every run that admits anything has shown that its journal can be written.
- **How the reader maps what it finds**: no file gives `Absent`. A file that cannot be opened, a wrong application ID, a newer version, a failed `PRAGMA quick_check`, a missing `store` row, a cluster with no member row for its approved writer, or a stored value outside the range of its type gives `Unreadable`. Anything else gives `Present`.
- **Storage failure while running**: mutations fail closed. Whether sessions that are already admitted go on is the observation expiry rule of the [architecture overview](../../architecture/overview.md#failure-behavior), settled in 004.

### Operation records

The full record and its API belong to scope 009; the phases of a promotion belong to 010. These rules are fixed now, so that both build on the same base (`SEC-7`, `R-18`).

- An operation row is written, and the transaction is durable, before the first effect outside the journal.
- Every change of phase is one transaction, and it advances the authority revision when it changes authority.
- The final phases are completed, failed and needs review. A row in any other phase at start is reconciled before the clusters it names admit anything. Reconciling never repeats an effect that may already have happened.
- The row names the caller, the kind, the target, the generation it expected and the request ID. The same caller, kind and request ID always lead to the same row (`PRD-010 AC-1`).
- Bootstrap writes no operation row. It has no effect outside the journal, and it is one transaction: either the whole journal exists or none of it.
- A change of the approved writer, its new epoch, the quarantine of the old writer and the outcome of the operation are written in one transaction (`PRD-007 AC-4`).

## Value sourcing

| Value | Source |
|---|---|
| Finding code, key, line, column | `pool-config`: constants, and the span of `toml::Spanned` |
| The made inputs of `AC-15` | Built by the test at run time from the texts under [Validation time](#validation-time). Not committed. The expected columns of input 3 are read by the test from the text it built |
| The time limit of `AC-15`, and the time it is compared with | The constant 10 seconds in the test. The elapsed time is read by the test with `std::time::Instant`, never by `validate` |
| Cluster, node and identity fields | The configuration file, through `ValidatedConfig` |
| `system_identifier` in the configuration | You read it from the primary, for example with `SELECT system_identifier FROM pg_control_system()`, and write it into the file. It is the identity check that bootstrap and admission compare against |
| `expected_writer` | Your statement in the configuration |
| Authority ID | 16 random bytes from the operating system, made by the bootstrap command (004) and by reconcile (009). The witness in the configuration is typed by you from the command's output |
| Approved writer, members and epoch 1 at bootstrap | `expected_writer` and the configured nodes of the cluster, accepted by `plan_bootstrap` |
| Conflict latch | Set by the daemon when `Closed::latches_conflict()` is true. Cleared by reconcile (009) |
| Writer timeline | The timeline that the writer's observation reported when the plan, the promotion or the reconcile was made |
| Observed role, system identifier, timeline | The probe of 004: `pg_is_in_recovery()`, and the identity and timeline source named under [value sourcing](../../architecture/control-plane.md#value-sourcing) of the control plane page. This spec needs the timeline a node writes on or replays on |
| `now` and `sampled_at` | The monotonic clock, read by the daemon. Tests pass fixed values |
| `expiry` | A configuration key added in 004 with the probe keys. The lab value is 6 seconds. Tests pass it |
| `StoreProbe` | The journal reader of 004. Tests build it by hand |
| Backend database when the key is absent | The value of `database` in the same entry |
| Start sequence, authority revision, topology epoch | The journal. Advanced only as [Generations](#generations) says |

## Security model

- **Who may write what.** The configuration file belongs to the operator. The state directory and the journal belong to the service account alone (mode `0700` and `0600`, checked from 004 on). Nothing in this slice listens on a network, so there is no caller to authenticate yet. Every later mutation of the journal comes through the authenticated administration path of 009 (`SEC-6`), except the conflict latch, which the daemon sets by itself and which only restricts.
- **Identity.** A backend login is chosen only by the identity map, from a principal that authentication has verified (`0001 SC-3`, `SEC-2`, `T-04`). Nothing a client sends in its startup message selects a login, a credential or a cluster.
- **Secrets.** The configuration holds references, never secrets (`SEC-4`). No type of this slice holds a secret. The types that hold names from the file print identifiers only (`SEC-11`, `T-12`).
- **Loss and tampering.** A lost, foreign or restored journal is detected and keeps admission closed (`T-11`, `R-18`). A person with root on the proxy host can still edit the journal or the configuration. That is outside what the daemon can defend, as the threat model says.

## Workspace and guards

This slice adds the third workspace member. These are the decisions that [spec 0002](../0002-standards-and-tooling/index.md#rules-for-later-slices) asks for, including its Follow-up 10.

- **Manifest**: `crates/pool-control/Cargo.toml` names the package `pool-control`, inherits the five workspace fields, depends on `pool-config` with `workspace = true`, and has `[lints] workspace = true`. It has no `dev-dependencies` and no build script. The root `Cargo.toml` gets `"crates/pool-control"` in `members` and `pool-control = { path = "crates/pool-control" }` under `[workspace.dependencies]`.
- **`apps/pool` does not depend on `pool-control` yet.** Nothing in the binary calls it in this slice. Scope 004 adds that pair.
- **Baseline files** in `tools/repo-check/baseline/`: add the line `pool-control` to `lock-packages.txt` (W1), remove the line `crates/pool-control` from `forbidden-paths.txt` (W2), add the line `pool-control -> pool-config` to `internal-dependencies.txt` (W5). `forbidden-source-patterns.txt` does not change.
- **How W4 and W5 learn the packages** (Follow-up 10 of spec 0002): they keep reading the manifests and the `members` array, as today. One check is added to W4: every `[[package]]` entry of the root `Cargo.lock` that has no `source` line must be the package name of a member written in `members`. An entry is the lines from one `[[package]]` line to the next, or to the end of the file. A message names `Cargo.lock` and the package. The test helper that writes a lockfile for the tests of `pool-repo-check` writes entries without a `source` line today; give it a `source = "registry+https://github.com/rust-lang/crates.io-index"` line for every name that is not a member, so that the existing good tree still passes. This closes the gap that a package which Cargo treats as a member, without being listed, escaped both rules. Reading the dependency edges from the lockfile is the runner up; take it up when `pool-core` arrives and the forbidden pair starts to matter.
- **`scripts/prove-checks.sh`, case 17**: it replaces `planned` in the first line of the scope 003 file. Once feature 3 is `in-progress` that line no longer holds the word and the case would change nothing. Point the case at `docs/scope/004-one-real-primary-path.md`.
- **Criteria of spec 0001 that named the old shape**: `0001 AC-1` (two members), `0001 AC-12` (two workspace packages in the lockfile) and `0001 AC-13` (`crates/pool-control` does not exist) describe the first build, and their evidence stands. From this slice on, `0003 AC-13` is the current statement.

## Tests and who owns them

Tests use the built in `cargo test` harness only. No package gets a dependency for tests.

| Package | File | Covers |
|---|---|---|
| `pool-config` | `crates/pool-config/tests/validate.rs`, and unit tests beside the code | Every new fixture through `validate`. Each row of "Which code a bad value gets", from memory. Canary absent from `Debug` and `Display` (AC-4). `resolve_identity` (AC-5). Identifier and generation types (AC-6). Made inputs 1 to 4 through `validate`, the first three within the limit (AC-15) |
| `pool` | `apps/pool/tests/check_config.rs` | The built binary with every new fixture, both formats, byte for byte (AC-1 to AC-4). Made input 3 through the built binary, both formats, within the limit (AC-15) |
| `pool-control` | `crates/pool-control/tests/walkthroughs.rs` | The four walks, one test each, every row (AC-7 to AC-11) |
| `pool-control` | `crates/pool-control/tests/admission_model.rs` | The small domain of AC-12 |
| `pool-config`, `pool-control` | `compile_fail` documentation tests beside the types, as `pool-config` already has for `ValidatedConfig` | Comparing a `NodeId` with a `ClusterId` and a `TopologyEpoch` with an `AuthorityRevision` (AC-6). A struct literal of `Authority`, `EligiblePrimary`, `Closed` and `BootstrapPlan` written outside the crate (AC-12) |
| `pool-repo-check` | `tools/repo-check/` | The addition to W4, one passing and one failing case (AC-13) |

The tests of `pool-control` get a `ValidatedConfig` by calling `pool_config::validate` on TOML text written in the test.

**The small domain of AC-12** (test `open_only_on_full_agreement`): cluster `main` with `pg1`, `pg2`, `pg3`, in state `Ready`. Loop over every combination of: `expected_writer` in `pg1`, `pg2`; approved writer in `pg1`, `pg2`; `pg3` quarantined or not; the conflict latch set or not; and for each of the three nodes one of eight observations: none, stale, failed, not in recovery on timeline 1, not in recovery on timeline 2, in recovery on timeline 1, in recovery on timeline 2, not in recovery with another system identifier. The journal records timeline 1 and all three nodes as members. That is 8192 cases. The test computes the expected answer (open or closed, and which node) with a short rule of its own, written from the rules under [Writer admission](#writer-admission) and not by calling the code under test, and compares.

## Document corrections

Edits to files that this spec does not own. The Build plan names who makes each one.

| Item | File | Change | Made by |
|---|---|---|---|
| 1 | `docs/engineering-standards.md` | In the shared conventions table, replace `Pending` in the row "Identifier and generation types" with: one type per identifier and generation, built only by a checking function, generations advance by one and fail closed at the top, shared types live in `pool-config`, memory generations are scoped by the start sequence; full rule linked to this spec | Build plan step 6 |
| 2 | `docs/engineering-standards.md` | First paragraph: the workspace is `apps/pool`, `crates/pool-config` and `crates/pool-control` | Build plan step 6 |
| 3 | `docs/repository-structure.md` | First paragraph: add `crates/pool-control` to the paths that exist | Build plan step 6 |
| 4 | `docs/glossary.md` | The rows "Observation" and "Approved writer" say backend where they mean a configured server. Change that word to node | Build plan step 6 |
| 5 | `docs/index.md` | Add the row for `verify.md` under this spec, and a section for `crates/pool-control` if it gets a context file | Build plan step 7 |
| 6 | `AGENTS.md`, `CLAUDE.md`, `crates/pool-config/AGENTS.md`, a new `crates/pool-control/AGENTS.md`, `CONTRIBUTING.md`, `README.md`, `CHANGELOG.md` | Name the third crate, the new keys and the new commands to run | The "Document it" box of scope 003 |
| 7 | Spec 0001 and spec 0002 | Notes that point here ([Follow-up](#follow-up) 1 and 2) | Applied on 2026-11-15, in the commit after this spec was accepted |

## Build plan

The order is a tracer bullet (one thin real path first): a workspace that still passes every check with the new member, then the configuration path from file to finding, then the authority rules, then the evidence. Each step ends with a commit named `<type>(<scope>): spec 0003 step <n>, ...`. Run `scripts/check.sh fmt` and `scripts/check.sh clippy` before each commit.

1. **Workspace member and guards.** Create `crates/pool-control/Cargo.toml` and a `src/lib.rs` that holds only the crate documentation. Change the root `Cargo.toml`, run `cargo build -p pool-control` once so that `Cargo.lock` gains the package, and change the three baseline files as [Workspace and guards](#workspace-and-guards) says. Add the lockfile check to rule W4 in `tools/repo-check/` with its two tests. Point case 17 of `scripts/prove-checks.sh` at the scope 004 file. Set this spec's `**Status**` line to `In Progress`. Files: `Cargo.toml`, `Cargo.lock`, `crates/pool-control/`, `tools/repo-check/`, `scripts/prove-checks.sh`. Evidence: `scripts/check.sh guards`, `cargo test --manifest-path tools/repo-check/Cargo.toml --locked --target-dir target/repo-check`, and, after the commit of this step because the script refuses a tree with changes, `scripts/prove-checks.sh`. Satisfies **AC-13**.
2. **Identifier and generation types.** Add the types under [Identifier and generation types](#identifier-and-generation-types) to `crates/pool-config/src/` (modules of your choice, exported from the crate root) with unit tests and the two `compile_fail` documentation tests. Evidence: `cargo test -p pool-config --locked`. Satisfies **AC-6**.
3. **Configuration keys in `pool-config`.** Implement the [keys](#keys), the [findings](#findings), the [validation order](#validation-order) and the [public boundary](#public-boundary-added-to-pool-config), including `resolve_identity` and the hand written `Debug`. Add every file under [Fixtures](#fixtures). Extend `crates/pool-config/tests/validate.rs`. The workspace must build after every commit, so this commit also adds the arm for `Location::KeyAt` to the `match` in `apps/pool/src/report.rs`. Evidence: `cargo test -p pool-config --locked` and `cargo build --workspace --locked`. Satisfies **AC-5**, and the library half of **AC-1** to **AC-4**.
4. **The new finding line in `apps/pool`.** Check the rendering of `Location::KeyAt` in the human and JSON formats in `apps/pool/src/report.rs` against [Findings](#findings). Extend `apps/pool/tests/check_config.rs` with every new fixture in both formats. Evidence: `cargo test -p pool --locked`, and `cargo run -q --locked -p pool -- check-config --config crates/pool-config/tests/fixtures/valid-topology.toml` with exit 0. Completes **AC-1** to **AC-4**.
5. **Authority model in `pool-control`.** Implement the section [The authority model in pool-control](#the-authority-model-in-pool-control): the types, `classify_startup`, `plan_bootstrap` and `writer_admission`. Write `tests/walkthroughs.rs` and `tests/admission_model.rs`. Give every public record type a documentation comment that states its restart rule. Needs steps 1, 2 and 3. Evidence: `cargo test -p pool-control --locked walk_`, `cargo test -p pool-control --locked open_only_on_full_agreement`, `cargo test -p pool-control --locked --doc`, and the `grep` of **AC-11**. Satisfies **AC-7** to **AC-12**, and the code half of **AC-14**.
6. **Document corrections.** Apply items 1 to 4 of [Document corrections](#document-corrections). Evidence: `scripts/check.sh docs`. Completes **AC-14**.
7. **Evidence.** On the final tree run `scripts/check.sh all`, `scripts/prove-checks.sh` (with `--with-linux-build` when `docker info` succeeds), `scripts/check.sh fuzz`, and the `cargo metadata` command of **AC-13**. Record commands, results and the revision in `docs/specs/0003-control-and-session-data-model/verify.md`, and add that file to `docs/index.md`. If `docker info` fails, record the Linux build as `BLOCKED` with `owner: user`. Evidence for **AC-1** to **AC-14**.
8. **Validation time.** Added on 2026-11-15, after the verification of milestone 2. Change `crates/pool-config/src/` so that lines and columns are worked out as [Validation time](#validation-time) says. No position, no finding and nothing of the public boundary changes. Add the tests of the four made inputs to `crates/pool-config/tests/validate.rs`, and the test of made input 3 through the built binary to `apps/pool/tests/check_config.rs`. Their names start with `ac15_`. Needs steps 3 and 4. Evidence: `cargo test -p pool-config --locked ac15_`, `cargo test -p pool --locked ac15_`, then `scripts/check.sh all`, recorded in `verify.md` under a milestone of its own. Satisfies **AC-15**.

**Parallel sessions.** Step 1 touches the root manifest, the lockfile, `tools/` and `scripts/`. Steps 2, 3 and 4 touch `crates/pool-config/` and `apps/pool/` and run one after the other in one session. The two groups share no file and can run at the same time. Step 5 needs both. Steps 6 and 7 come last. Step 8 was added after them and touches only `crates/pool-config/` and `apps/pool/`.

## Consequences

**Positive**:

- The rule that protects against `R-19` exists as code with tests before any socket does. Scope 004 connects inputs to it and does not have to decide it under the pressure of a first real connection.
- Every state that is not full agreement leads to closed, and an open admission can only come out of `writer_admission` with a ready authority. A later slice cannot open writes by forgetting a check, as long as the journal reader stays the only code that builds a `StoreProbe`.
- A missing journal, a foreign journal and a restored backup are three different, named states, and none of them looks like a first start.
- An operator sees a wrong reference, a duplicate or a missing key with its key and its place in the file, at the desk.
- Default telemetry has names it may use (identifiers) and a list of names it may not, before the first log event is written.
- No third party package is added. The product still has sixteen.
- A file at the limit is checked in a fraction of a second by a release build, valid or not, and a test fails when that stops being true.

**Negative / tradeoffs**:

- After a bootstrap you must copy the authority ID into the configuration before POOL admits anything. After a reconcile you must do it again. It is one more step, and it is the step that tells a first start from a lost journal.
- After a promotion (010) the configuration must name the new writer before writes open again. Until then admission is closed with `authority.config_conflict`. Scope 010 has to make that step part of its procedure, and it adds to the time a promotion takes.
- A raw file copy of the journal carries no hold. If someone restores one, and the old writer is up and unfenced, and no node on the newer timeline can be reached, POOL cannot tell and opens writes to the old writer (walk 4 shows the cases it does catch). Fencing and the supported backup are the protection. A rollback of the whole proxy VM to a snapshot is the same case.
- A stale configuration without the witness, together with a lost journal, looks like a first start. Bootstrap then still needs fitting evidence from every node, which a cluster after an unrecorded promotion does not give.
- A new bootstrap after a lost journal forgets the quarantine and every membership decision.
- Until scope 009 exists nothing clears a conflict latch and nothing removes a member. A cluster that once showed a second primary, and a cluster whose configuration lost a member, stay closed until the member is put back or you bootstrap anew. That is harsh for a lab, and it is the behavior `PRD-006 AC-2` asks for.
- The configuration is trusted. If you point the `host` of a node at another machine, POOL probes that machine and can no longer see what the old one reports. The journal records node IDs, not addresses.
- Bootstrap needs every configured node reachable once. A node that is down has to be repaired or taken out of the configuration first.
- `pool-control` is built before anything calls it. Its interface may need to change when scope 004 connects real inputs. The rules should not.
- Findings inside entries name the key and the place but still not the text, so you look at the position yourself, as with the findings of spec 0001.
- The identity map is exact. Ten databases for one principal are ten entries.
- The journal schema is fixed here and first run by scope 004. A flaw found then is a change to this spec.
- `AC-15` is a limit on wall clock time. It is set far from both sides, but a machine that stands still for seconds in the middle of a test can still miss it. The tests also take a second or two longer, because inputs of 1 MiB run through a debug build. When the counting goes wrong again, the test cannot cut the call short: it runs for minutes before it fails, and on CI the job limit of 15 minutes may end it first. Either way the check is red.

**Neutral**:

- The key names, the finding codes, the fourth finding line, the reason names, the journal file name, its marks and its schema are contracts from now on.
- The word node now means a configured PostgreSQL server everywhere. Backend keeps its glossary meaning, a connection.
- `0001 AC-1`, `0001 AC-12` and `0001 AC-13` describe the first build. `0003 AC-13` is the current statement of the workspace shape.
- The rule that the first secret reference key arrives in scope 004 changes: the key arrives here, and reading it arrives in 004.
- The system identifier, the authority ID, the timeline ID and the observed role may appear in log fields and default diagnostics. Of these only the observed role may be a metric label. A generation may no longer be a label either, only a value.

## Follow-up

1. Applied on 2026-11-15. Spec 0001 now carries these notes that point here, each dated and each leaving its own text and IDs as they were: under "Configuration rules" item 6, that `identity[].credential_file` is the first secret reference key and that scope 004 adds the reading and the permission check; under "Exit status and output", the fourth finding line; under "Smallest supported configuration", that this spec fixes the identifier and generation types; under its criteria, that `0001 AC-1`, `0001 AC-12` and `0001 AC-13` describe the first build and `0003 AC-13` is the current statement from Build plan step 1 on; under "Findings", the six new `config.*` codes and the new location. Its Follow-up 2 and Follow-up 10 are marked as answered in part.
2. Applied on 2026-11-15. Spec 0002 now carries these notes that point here, each dated and each leaving its own text and IDs as they were: its Follow-up 10 is answered by [Workspace and guards](#workspace-and-guards); the baseline block under "Workspace guards" shows the first build and the current lines are in the files; case 17 under "Proving the checks" names the scope 004 file from Build plan step 1 on; and the details of rule W4 name the lockfile check that this spec adds.
3. Scope 004 adopts SQLite. That is the first native code in the product: it needs the boundary review of the engineering standards, a decision by a person on any new license in the dependency tree, the `NOTICE` file that spec 0002 asks for, and a decision on how one writer runs without breaking the thread rule of W3.
4. Scope 004 fixes the bootstrap command (name, flags, output) under the shared output conventions, the probe and its queries, the keys for probe period and observation expiry, the file permission checks, and how each authentication method names the verified principal that the identity map looks up. Three points it must not miss: the probe has to report the timeline a node writes or replays on now, not the timeline of its last checkpoint, or a node that was just promoted is missed; the state directory needs an exclusive lock, so that two daemons cannot share one; and there has to be a way to read the authority ID of an existing journal again, or a lost command output leaves the state at `Unconfirmed` for good.
5. Scope 005 names the trigger that advances a credential generation before a reload route exists, as the dependency plan asks.
6. Scope 009 builds the backup that sets the hold, the reconcile operation, the operation and audit tables, and the way a cluster is added to an existing journal. Until 009 exists there is no supported backup, and the only way out of a lost journal is a new bootstrap.
7. Scope 010 makes "the configuration names the new writer" a step of the promotion procedure, and decides whether the promotion plan binds the authority revision as well as the topology epoch.
8. The risk register rows `R-18` and `R-19` can name the tests of this spec as their first evidence once the build is verified. That edit belongs to the "Document it" box.
9. [Scope 012](../../scope/012-multiple-proxy-authority.md) needs ordered epochs across several proxies. The authority ID plus the topology epoch is the local form of that; a journal per daemon is not merged into a shared authority.

## Rationale

Reasoning, alternatives, and the list of decisions taken: [rationale.md](rationale.md).
