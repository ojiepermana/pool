# Failover and recovery

Manual failover is the initial policy. A transport timeout or failed health probe never authorizes promotion. PostgreSQL's [failover documentation](https://www.postgresql.org/docs/18/warm-standby-failover.html) explicitly requires preventing the former primary from returning as another primary.

## Operation boundary

| Stage | Evidence and effect | Abort behavior |
|---|---|---|
| Plan | Authorized operator, explicit candidate, cluster/system identity, role, timeline, lag and expected generation | Safe to discard an unexecuted plan |
| Quiesce | Stop new affected admission, drain primary sessions, retire candidate replica leases, and record uncertain work. Establish an all-writer barrier before claiming final flush evidence | Can resume only after verified topology reconciliation |
| Fence | Durable operation intent precedes the provider action. External provider excludes old primary from writing; independent verification binds receipt to exact infrastructure identity and operation | A fence may be reversible by an operator, but POOL must not automatically undo it |
| Revalidate | Candidate fresh, receipt valid, no conflicting operation, data loss bound known or explicitly unknown | Abort and keep cluster closed if evidence fails |
| Promote | Durable intent written, invoke tightly scoped PostgreSQL promotion primitive once | Irreversible boundary; no automatic rollback to old primary |
| Reconcile | Observe new primary and timeline, durably commit approved writer/new epoch/quarantine with operation outcome, then publish snapshot | Lost response means reconcile, never blindly reissue promotion |

## Candidate and RPO policy

Candidate selection considers configured priority, availability, system identity, timeline ancestry and replay position. Highest LSN alone cannot establish whether every acknowledged old primary commit survived. A planned switchover can claim a final flush target only after an authoritative write barrier covering every writer path, including connections that bypass POOL. Verify that candidate replay reaches that target on compatible timeline history before claiming it was preserved. If an approved plan requires catch up and the candidate falls short, keep admission closed and replan. Draining POOL alone is insufficient. If the barrier or flush evidence cannot be proven, use the unknown loss window path and explicit operator acknowledgement. No zero data loss claim is inferred from asynchronous replication.

Durable approved writer identity, epoch and quarantine survive restart. Fresh observations must match that authority, and any incomplete fencing or promotion effect is reconciled before admission. Reverting the TOML file is not a rollback of a completed promotion.

## Fencing contract

An allowlisted adapter accepts fixed structured arguments, not arbitrary shell fragments from the API. It returns provider action ID, target VM or machine identity, verification method, time and expiry. A successful request to power off is insufficient without verified completion and exclusion from automatic restart or another writer path. The operating environment must define who can reverse fencing. Network isolation must cover all database writer paths to qualify.

## Recovery

V1 promotion ends after reconciliation and durable quarantine of the old writer. Rejoining a former primary requires independent evidence that it is now a valid replica. POOL orchestration of rewind or base backup belongs to [checked online recovery](../scope/013-checked-online-recovery.md).

`pg_rewind` and `pg_basebackup` remain PostgreSQL tools operated through a separate privileged workflow. Rewind prerequisites include suitable WAL history and checksums or wal_log_hints; a failed rewind can leave a data directory needing a fresh base backup. Back up the target and validate prerequisites before modifying it. Direct filesystem recovery does not run inside a query task. [pg_rewind](https://www.postgresql.org/docs/18/app-pgrewind.html), [pg_basebackup](https://www.postgresql.org/docs/18/app-pgbasebackup.html).
