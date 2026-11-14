# Pgpool baseline

**Verified:** 2026-11-14, Asia/Jakarta. **Selected baseline:** Pgpool-II 4.7.3, released 2026-09-29. The official source page and release notes showed no newer 4.7 release during this check. Parallel maintained branches have their own patch releases. The 4.7 manual identified itself as 4.7.3, but its URL is a series URL and can change. [Release news](https://www.pgpool.net/news/2026-09-29/), [source downloads](https://www.pgpool.net/download/source/), [release notes](https://www.pgpool.net/docs/4.7/en/html/release-4-7-3.html).

## Facts

Pgpool-II is C middleware with a parent and multiple child processes serving PostgreSQL traffic. Backend connection caching is associated with children and connection identities. The product combines pooling, routing, replication awareness, administrative operations, and Watchdog coordination. You should not model it as a transaction multiplexer. The documented connection budget includes the `num_init_children * max_pool` effect per backend. [Pooling reference](https://www.pgpool.net/docs/4.7/en/html/runtime-config-connection-pooling.html).

| Baseline mode | What it means | POOL disposition |
|---|---|---|
| `raw` | No middleware replication or read load balancing | Core primary proxy profile |
| `streaming_replication` | PostgreSQL replicates WAL; Pgpool routes and monitors | Initial operating model |
| `logical_replication` | PostgreSQL logical replication, with object and visibility restrictions | Research only, no initial routing guarantee |
| `native_replication` | Middleware sends writes to multiple nodes | Deferred feasibility decision |
| `snapshot_isolation` | Middleware replication with additional consistency constraints | Deferred feasibility decision |

Logical mode is a distinct documented mode. It is not interchangeable with physical standby semantics. Native and snapshot modes introduce sequence, context value, large object, and temporary table constraints. [Mode reference](https://www.pgpool.net/docs/4.7/en/html/runtime-config-running-mode.html), [restrictions](https://www.pgpool.net/docs/4.7/en/html/restrictions.html).

## Coverage manifest

The [configuration manual](https://www.pgpool.net/docs/4.7/en/html/runtime-config.html) covers parameter interaction, connections and authentication, process management, modes, backend settings, pooling, logging, routing, health, failover and failback, online recovery, streaming checks, cache, TLS, Watchdog, and miscellaneous settings. Every family has a disposition in [the feature matrix](../feature-matrix.md).

The release pinned [streaming sample](https://raw.githubusercontent.com/pgpool/pgpool2/V4_7_3/src/sample/pgpool.conf.sample-stream) contains 207 distinct option spellings, including indexed examples. A separate census of the tagged runtime declaration file found 195 typed records and 186 unique names. The sample normalizes to 181 names; the additional five include retained settings, three hidden aliases, and a health test control. [The configuration inventory](pgpool-configuration-inventory.md) accounts for both corpora and their dispositions. Declaration coverage does not establish runtime defaults or compatibility.

The official [reference index](https://www.pgpool.net/docs/4.7/en/html/reference.html) exposes 15 operational PCP commands, 12 SQL command pages, 7 pgpool_adm wrappers, the daemon, and 6 utilities. These are listed individually in the feature matrix. The common PCP options page is not a sixteenth operation. The tagged source tree has `auth`, `config`, `context`, `include`, `libs`, `main`, `parser`, `pcp_con`, `protocol`, `query_cache`, `rewrite`, `sample`, `sql`, `streaming_replication`, `test`, `tools`, `utils`, `watchdog`, and packaging areas. This research reviewed structure and selected references; it is not a complete source audit.

## Tagged sample and source manifest

The [GitHub tree API](https://api.github.com/repos/pgpool/pgpool2/git/trees/V4_7_3?recursive=1) returned a non truncated tree with tree object SHA `07d24655e42dfdbfe2e7549df6166e057b99b622`. A tree SHA is not a release archive digest or tag commit SHA. Under `src/sample/`, the 15 files are:

| Reference file | Capability and POOL disposition |
|---|---|
| `pcp.conf.sample` | PCP credential format, not adopted by the initial new API |
| `pgpool.conf.sample` | Pointer to streaming sample, not a second parameter corpus |
| `pgpool.conf.sample-stream` | Streaming mode parameter census |
| `pgpool.pam` | PAM integration, outside initial authentication support |
| `pgpool_recovery` | Recovery helper integration, later scoped orchestration |
| `pool_hba.conf.sample` | Access rule examples, intent mapped to default deny policy |
| `scripts/aws_eip_if_cmd.sh.sample` | Cloud VIP helper, not a Proxmox dependency |
| `scripts/aws_rtb_if_cmd.sh.sample` | Cloud route helper, not a Proxmox dependency |
| `scripts/escalation.sh.sample` | Watchdog escalation behavior, future structured adapter |
| `scripts/failover.sh.sample` | Failover hook, replaced by explicit fenced operation design |
| `scripts/follow_primary.sh.sample` | Replica follow orchestration, future explicit operation |
| `scripts/pgpool_remote_start.sample` | Remote startup helper, external privileged workflow |
| `scripts/recovery_1st_stage.sample` | Streaming recovery stage, future checked recovery |
| `scripts/replication_mode_recovery_1st_stage.sample` | Native mode recovery, deferred with that mode |
| `scripts/replication_mode_recovery_2nd_stage.sample` | Native mode recovery second stage, deferred |

These are reference artifacts, not copied or approved deployment scripts. Inspect each script's privileges, arguments and rollback behavior before proposing reuse. Slony mode retirement is recorded for the 4.7 line. Specific 4.7.2 release-note details remain unverified because the research renderer could not retrieve that page.

## License and source reuse

The [official license](https://www.pgpool.net/license/) grants permissive use with retained notices, endorsement restrictions, and a warranty disclaimer. The project [describes it](https://www.pgpool.net/about) as similar to BSD and MIT. Refer to the actual text rather than assigning an unsupported SPDX label.

Recommended policy: implement from documented behavior first. Before copying any source, inspect the exact file and bundled components, retain notices, record origin and modifications, and review compatibility with POOL's chosen license. A permissive top level license does not describe every vendored file. No Pgpool application source has been copied into this workspace.

## Important gaps

This package is a planning inventory, not a certification of full parity. Runtime behaviors have not been exercised against Pgpool or PostgreSQL. Defaults, indexed overrides, all HBA combinations, build options, and minor release edge cases need a release pinned executable conformance corpus. Keep those gaps explicit; a chapter or parameter being listed is not a passed test.
