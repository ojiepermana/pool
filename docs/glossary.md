# Glossary

This file owns the project's terms. Each entry names the document that owns the rule behind the term; the rule itself is not repeated here. Use these names for types, configuration keys, metrics, log fields and API fields. When two documents use different words for one thing, the term below is the canonical one and the others are listed as avoided wording.

## Identity and sessions

| Term | Meaning | Owner |
|---|---|---|
| Frontend | The client side connection that POOL accepts and authenticates. | [Data plane](architecture/data-plane.md) |
| Backend | One PostgreSQL connection that POOL opens with its own login and verified TLS. | [Connection pooling](architecture/connection-pooling.md) |
| Verified principal | The frontend identity after authentication succeeds. A startup string is a claim, not a principal. | [Spec 0001](specs/0001-initial-architecture/index.md#safety-contracts), contract 3 |
| Identity map | The operator configured map from verified principal plus requested database to cluster, database, backend login, credential reference and routing profile. An unmapped pair fails. | [Spec 0001](specs/0001-initial-architecture/index.md#safety-contracts), contract 3 |
| Backend login | The actual PostgreSQL role a backend is logged in as. It is never a shared privileged role. | [Protocol analysis](research/protocol-analysis.md#authentication-and-encryption) |
| Session permit | The bounded right of one frontend to exist. It is returned when the frontend ends. Avoid the bare word permit for anything else. | [Data plane](architecture/data-plane.md#resource-proposal) |
| Lease | Exclusive ownership of one backend by one frontend for the whole session. | [Spec 0001](specs/0001-initial-architecture/index.md#safety-contracts), contract 1 |
| Pool key | The full backend identity that partitions reusable backends. | [Connection pooling](architecture/connection-pooling.md#pool-identity-and-lifecycle) |
| Identity ID | The name an operator gives one entry of the identity map. Logs and metrics use it in place of the principal or the backend login. | [Spec 0003](specs/0003-control-and-session-data-model/index.md#what-may-appear-in-default-telemetry) |
| Virtual cancellation credential | The per lease cancel token given to the frontend. The real backend key stays private. | [Spec 0001](specs/0001-initial-architecture/index.md#safety-contracts), contract 9 |

## Profiles

The word profile always takes a qualifier.

| Term | Meaning | Owner |
|---|---|---|
| Routing profile | What the identity map assigns to a session: a primary profile or a replica profile. | [SQL routing](architecture/sql-routing.md) |
| Primary profile | A session pinned to the eligible primary for its lifetime. This is the default. | [SQL routing](architecture/sql-routing.md#decision-matrix) |
| Replica profile | A restricted read only session pinned to one eligible replica, with eventual reads and no primary fallback. Avoid read profile and read only profile as separate terms. | [SQL routing PRD](prd/003-sql-routing.md) |
| Startup profile | The startup parameters a backend was opened with and that cleanup can restore for a new owner. | [Connection pooling](architecture/connection-pooling.md#pool-identity-and-lifecycle) |
| Support profile | The PostgreSQL version, protocol versions, authentication modes and drivers a release claims. Avoid declared profile and protocol profile as separate terms. | [Driver compatibility matrix](testing/compatibility-matrix.md) |

## Authority and topology

| Term | Meaning | Owner |
|---|---|---|
| Observation | One probe sample of a node: role, identity, timeline, positions and error class, with its monotonic sample time. | [Control plane](architecture/control-plane.md#proposed-records) |
| Fresh | An observation younger than its expiry on the local monotonic clock at the moment of admission. | [Cluster management](architecture/cluster-management.md#health-and-topology) |
| Observed role | What a fresh observation reports. It is evidence, never authorization. | [Control plane](architecture/control-plane.md) |
| Approved writer | The node durably authorized to accept writes. Avoid authorized writer and authorized primary. | [Spec 0001](specs/0001-initial-architecture/index.md#safety-contracts), contract 8 |
| Eligible primary | The approved writer while a fresh observation agrees with it. Only this node receives writes. Avoid verified primary. | [Control plane](architecture/control-plane.md) |
| Eligible replica | A replica that passes role, identity, freshness, lag, maintenance and weight checks at admission. | [Load balancing PRD](prd/004-load-balancing.md) |
| Topology snapshot | The immutable, versioned view of eligibility that the data plane reads. | [Spec 0001](specs/0001-initial-architecture/index.md#boundaries) |
| Node | One configured PostgreSQL server of a cluster, with a stable ID, a host and a port. A backend is a connection to a node. | [Spec 0003](specs/0003-control-and-session-data-model/index.md#records) |
| Expected writer | The node that the configuration names as the writer of a cluster. It seeds the approved writer at bootstrap and must keep agreeing with it. | [Spec 0003](specs/0003-control-and-session-data-model/index.md#keys) |
| Journal | The SQLite file that holds the approved topology and quarantine and, later, operations and audit events. One daemon owns one journal. | [Spec 0003](specs/0003-control-and-session-data-model/index.md#the-journal) |
| Authority ID | The random value that names one journal. The configuration repeats it as the authority witness, so that a missing journal is not taken for a first start. | [Spec 0003](specs/0003-control-and-session-data-model/index.md#start-classification) |
| Bootstrap | The explicit operator command that creates a journal once, from the expected writer and matching fresh observations of every node. The daemon never does this by itself. | [Spec 0003](specs/0003-control-and-session-data-model/index.md#bootstrap) |
| Restore hold | The mark a journal carries when it came out of a backup. Admission stays closed until an operator reconciles it. | [Spec 0003](specs/0003-control-and-session-data-model/index.md#walk-4-restore-of-a-backup-taken-before-a-promotion) |
| Conflict latch | The durable mark a cluster gets when a second primary or a later timeline was observed. Writes stay closed until an operation clears it, also when the evidence is gone. | [Spec 0003](specs/0003-control-and-session-data-model/index.md#writer-admission) |
| Admission | The decision to accept work. Initial admission selects a backend at login. Operation admission accepts or refuses later work on the pinned lease. | [Data plane](architecture/data-plane.md) |

## Generations

The word generation always takes a qualifier. The [control model](scope/003-control-and-session-data-model.md) settles what advances each one and what survives restart.

| Term | Meaning | Owner |
|---|---|---|
| Configuration generation | One validated whole configuration. An invalid reload keeps the previous one. | [Security requirements](security/security-requirements.md), SEC-12 |
| Credential generation | One version of a secret reference. Rotation retires the old one. | [Connection pooling](architecture/connection-pooling.md#limits-and-operations) |
| Topology epoch | The durable counter that changes with the approved writer. Avoid durable epoch. | [Control plane](architecture/control-plane.md#proposed-records) |
| Authority revision | The durable counter that rises with every change of the approved topology, the quarantine, a membership decision or the restore hold. A mutation of durable authority names the revision it expects. | [Spec 0003](specs/0003-control-and-session-data-model/index.md#generations) |
| Start sequence | The durable counter that rises by one each time the daemon opens its journal to serve. A generation that lives in memory is only comparable inside one start sequence. | [Spec 0003](specs/0003-control-and-session-data-model/index.md#generations) |
| Snapshot generation | The version of one published topology snapshot. It is rebuilt after restart. | [Control plane](architecture/control-plane.md#proposed-records) |
| Lease generation | The unique value that stops a late cancel or completion from reaching a reused backend. | [Architecture overview](architecture/overview.md#module-contracts) |
| Policy version | The version of a routing or selection policy recorded with a decision. Avoid policy generation. | [SQL routing PRD](prd/003-sql-routing.md) |
| Expected generation | The value a mutation names as its precondition. The API states which generation it compares. | [Control plane](architecture/control-plane.md) |

## Operator actions

The exact effect of each action on active sessions belongs to the [health](scope/008-health-diagnostics-and-drain.md) and [administration](scope/009-safe-topology-administration.md) specs.

| Term | Meaning | Owner |
|---|---|---|
| Maintenance | An operator decision that keeps a healthy node out of new admission. A good probe cannot clear it. | [Cluster management](architecture/cluster-management.md#health-and-topology) |
| Drain | Stop new acquisitions on a node and let existing leases end by a deadline. | [Cluster management](architecture/cluster-management.md#health-and-topology) |
| Detach | Remove a drained node from approved membership until an explicit attach. | [Cluster management PRD](prd/006-cluster-management.md) |
| Quarantine | A durable exclusion of a node that may hold conflicting writes, such as a former primary. It survives restart and ends only with rejoin evidence. | [Failover and recovery](architecture/failover-recovery.md#recovery) |
| Operation | One journaled administrative action with a caller, target, expected generation, phase and outcome. | [Control plane](architecture/control-plane.md#proposed-records) |
| Request ID | The caller supplied value that makes a mutation retry safe. The journal stores it as the idempotency key; both words mean this one value. | [Administration PRD](prd/010-administration.md#interfaces) |
| Fence receipt | Independently verified proof that the old writer is excluded, bound to one operation and one machine identity. | [Failover and recovery](architecture/failover-recovery.md#fencing-contract) |
| Write barrier | Proof that every writer path is stopped, including paths that bypass POOL. Without it the loss window is unknown. | [Failover and recovery](architecture/failover-recovery.md#candidate-and-rpo-policy) |

## Configuration and commands

| Term | Meaning | Owner |
|---|---|---|
| Configuration version | The number in the `config_version` key that says which key schema a file follows. It is not a configuration generation. | [Spec 0001](specs/0001-initial-architecture/index.md#configuration-rules) |
| Reserved capability | A capability outside the release whose configuration table exists only so that asking for it can be refused by name. | [Spec 0001](specs/0001-initial-architecture/index.md#configuration-rules) |
| Result cache | Storing query results to answer a later query without the database. Reserved and off. Avoid query cache as a key or type name. | [Security requirements](security/security-requirements.md), SEC-10 |
| Automatic promotion | POOL promoting a replica without a reviewed operator operation. Reserved and off. Avoid automatic failover as a key or type name; removing a failed node from admission is not promotion. | [Security requirements](security/security-requirements.md), SEC-10 |
| Secret reference | A configuration key that ends in `_file` and names a protected file that holds a secret. The configuration never holds the secret itself. | [Spec 0001](specs/0001-initial-architecture/index.md#configuration-rules) |
| Finding | One reason a command rejected its input or could not complete, with a stable code such as `config.schema`. | [Spec 0001](specs/0001-initial-architecture/index.md#findings) |
| Output version | The number in the `output_version` field of a command's JSON output. It rises when a field is removed or renamed. | [Spec 0001](specs/0001-initial-architecture/index.md#exit-status-and-output) |

## Delivery

| Term | Meaning | Owner |
|---|---|---|
| Slice | One numbered scope feature that delivers an observable outcome through every layer it needs. | [Scope index](scope/index.md) |
| Walking slice | Scope 004, the first real client path. | [Testing strategy](testing/strategy.md#first-walking-slice-acceptance) |
| Core MVP, Routing preview, V1 | Release gates, not version numbers. | [Milestones](plan/milestones.md) |
| Verification record | The per criterion evidence for one slice. | [Testing strategy](testing/strategy.md#acceptance-evidence) |
| Required check | A named check that must pass on a pull request before it can be merged into `main`. The same name is the argument of `scripts/check.sh`, the CI job and the entry in the branch rule. | [Spec 0002](specs/0002-standards-and-tooling/index.md#required-checks) |
| Baseline file | A text file that lists what a workspace guard allows today, such as the package names of a lockfile. A spec changes it on purpose, in the same pull request as the change it allows. | [Spec 0002](specs/0002-standards-and-tooling/index.md#workspace-guards) |
| Branch rule | The GitHub ruleset for `main`, kept as a file in the repository. Avoid branch protection as a name for it; that is an older GitHub feature. | [Spec 0002](specs/0002-standards-and-tooling/index.md#branch-rule-and-change-path) |
| Fresh model | A model session that did not write or design a change and starts without the author's conversation. It may do some reviews in place of a person. | [Spec 0002](specs/0002-standards-and-tooling/index.md#reviewer-roster-and-fresh-model-reviews) |
| Reviewer roster | The table that names who holds each review role and risk owner role. | [Engineering standards](engineering-standards.md#reviewer-roster) |
