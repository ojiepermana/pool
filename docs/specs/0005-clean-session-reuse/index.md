# 0005. Clean session reuse

**Date**: 2026-11-16
**Status**: Accepted

## Summary

You can reuse a backend after one frontend disconnects, once POOL has proved that cleanup succeeded. A connected frontend still owns one backend for its whole session. This slice adds bounded admission, generation retirement and a conservative profile of PostgreSQL session state that cleanup can restore.

## Requirements

As an operator, you want reconnecting clients to share clean physical backends without sharing identity or session state. This slice adopts PRD-002 AC-1 through AC-4 and SEC-8, SEC-11, SEC-12 and SEC-14.

- **AC-1**: Two successive authenticated TLS clients with the same eligible key use the same PostgreSQL PID. The later owner sees startup defaults and no earlier GUC, temp object, advisory lock, SQL prepared statement, cursor or LISTEN subscription. Exercise open and failed transactions. IdentityId is part of the key, so distinct mapping entries remain isolated even when they name the same backend login.
- **AC-2**: Keys differing in cluster, node, actual login, database, credential generation, configuration generation, topology epoch, TLS policy or canonical startup settings never share a backend. Every new lease gets a new lease generation.
- **AC-3**: Connecting, leased, cleaning and idle backends count against one global cap and nested node, database and backend identity caps. Saturation over many keys cannot exceed any cap. Idle entries for another key can be evicted to make progress.
- **AC-4**: Acquisition has a bounded FIFO queue and one deadline covering wait, connect and validation. Timeout, cancelled wait and shutdown return all permits. No general mutex is held across await.
- **AC-5**: Known synchronized frontend Terminate or EOF can enter cleanup. A partial message, work in flight, protocol error, backend loss, route cut, shutdown, COPY or unrecognized session state closes the backend. Cleanup error, extra result, malformed packet, nonidle status or deadline closes it. No dispatched work is replayed.
- **AC-6**: Retire a credential or configuration generation while waiting, during handshake and during cleanup. No retired lease is handed out, no retired backend becomes idle and retirement removes idle backends. An existing owner keeps its pinned connection until it ends, unless normal route or shutdown rules cut it.
- **AC-7**: Idle age, physical lifetime, cleanup work and client writes are bounded. Lifetime expiry retires a backend at release or before acquisition, never moves a connected session. A slow reader cannot retain a backend indefinitely while POOL writes.
- **AC-8**: Backend SCRAM derivation is owned by a task that completes and caches its result even when the requesting login disappears. Concurrent logins cannot multiply derivations for the same salt and iteration count. The backend identity question runs once per new physical backend, not once per lease.
- **AC-9**: The tooling debt of spec 0004 Follow-up 14 is enforced with positive and negative proof cases, and Follow-up 16 has regression seams for forced task drop and daemon supervision. Reassess the allocation test without unsafe code and document its actual coverage.
- **AC-10**: Required local checks, real PostgreSQL integration, fresh protocol/security review and documentation have evidence. Capability claims describe only the supported reusable profile. Existing dedicated behavior continues for other SQL and startup profiles.

## Decision

Add an in memory pool owner inside pool-core and retain the existing dedicated path when reuse cannot be proved. Use the locked Rust, Tokio, rustls and PostgreSQL 18 stack. Add no dependency, persistence schema, administration endpoint, replica route, extended protocol or cancellation forwarding. Scope and spec decisions stay with the Leader. Native implementation source and regression tests are an explicit native_task_adaptation.

Technical decisions use agent-flow's auto policy from the user's invocation. The content was ratified under that auto policy after the completeness pass and two fresh context cross check passes. Building starts on 2026-11-16; status reflects delivery, not design acceptance.

## Feature design

### Data and ownership

| Entity | Key and required data | Ownership and relationship |
|---|---|---|
| Runtime generation | ConfigurationGeneration plus per IdentityId CredentialGeneration and Arc credential | One coherent snapshot owned by Server. Starts at FIRST, advances with checked_next, fails closed on exhaustion. |
| Pool key | Existing eight parts plus IdentityId, configuration generation and exact canonical forwarded startup name/value pairs | Equality and hashing include every part. Debug redacts names and values that came from peers. An identity cap uses cluster, database and actual login, not the frontend principal. |
| Physical backend | BackendConnection, key, created_at, idle_since, owned global/node/database/identity permits and validated startup parameters | Move only, one owner in Connecting, Leased, Cleaning or Idle. A lease wrapper adds the unique lease generation and frontend identity entry. Drop closes stream and returns every permit. |
| Pool state | Bounded idle entries, FIFO tickets, bounded notify state and retired generation state | One short synchronous mutex. No await while held. No permanent map entry per attacker supplied startup value. Maps derive from bounded validated configuration or prune empty entries. |
| Cleanup proof | Last transaction status, synchronized boundary and sticky reusable SQL eligibility | Local to the session, cannot be reset by later good SQL after unknown work. |

State transitions are Connecting to Leased, Leased to Cleaning, Cleaning to Idle, Idle to Leased. Any uncertainty goes to Closed. Idle invalidation, expiry and shutdown go straight to Closed. Never send frontend Terminate to a candidate backend before cleanup. A new backend remains in its pool budget throughout authentication and the identity question. Cleanup holds its original permits; it does not reacquire them.

### Reusable profile

The positive profile is protocol 3.0, the primary route, standard PostgreSQL session resources and no forwarded startup parameters. A frontend with forwarded settings follows the dedicated path and never enters idle; the key still carries the settings to prevent future accidental aliasing. This is deliberately smaller than a general PostgreSQL pool.

All SQL is still relayed under the existing simple query contract. Reuse eligibility is sticky and conservative. Inspect at most 4096 bytes of each Query while preserving the relay's fixed memory bound. Longer or malformed SQL is relayed but retires its backend. No SQL is rewritten, denied or retried by this inspection.

A small recognizer may prove only the grammar below, with ASCII case insensitive keywords, optional final semicolon and ASCII whitespace. Reject comments, quoted identifiers, dollar quoting, backslash escapes, embedded NUL, multiple statements and every unrecognized token from the reusable profile. Numeric literals are bounded decimal integers; names are ASCII identifiers. A quoted string uses ordinary SQL single quotes with doubled quotes and contains no backslash or control byte.

* Transaction statements: BEGIN, COMMIT, ROLLBACK, and SELECT 1 or SELECT 1 OPERATOR(pg_catalog./) 0.
* SELECT pg_catalog.pg_backend_pid(), pg_catalog.pg_advisory_lock(integer), pg_catalog.pg_advisory_unlock(integer) or pg_catalog.pg_advisory_unlock_all(), each as the entire select expression.
* SET application_name TO string, SET TIME ZONE string, SET search_path TO pg_catalog, and RESET ALL. SHOW application_name, SHOW TimeZone and SHOW search_path.
* CREATE TEMP TABLE name (value pg_catalog.int4), INSERT INTO pg_temp.name VALUES (integer), DROP TABLE pg_temp.name, and SELECT value FROM pg_temp.name. Only the safe names made in this session may be used for temp operations; otherwise retire. An error retires the candidate if it could have invoked unknown code.
* LISTEN name and UNLISTEN name or *. NOTIFY name with an optional quoted string.
* PREPARE name AS SELECT 1, DEALLOCATE name or ALL, DECLARE name CURSOR WITH HOLD FOR SELECT 1, FETCH ALL FROM name and CLOSE name or ALL. These names must have been made by the known grammar in this session.

This grammar is a cleanup eligibility recognizer, not a SQL router. All other SQL, including user functions, DO, CALL, LOAD, extension GUCs, arbitrary DDL and SQL PREPARE bodies, permanently retires the physical backend after its current owner ends. Unknown backend traffic also retires it. No extension cleanup is claimed. The trusted PostgreSQL server and its administrator must not install hooks that change the meaning of built in commands while POOL runs. Before admitting a physical backend to this profile, a bounded internal catalog question must prove no extension except plpgsql, no event trigger, and empty shared_preload_libraries, session_preload_libraries and local_preload_libraries; otherwise retain dedicated behavior. PostgreSQL catalogs and builtins in internal queries are qualified with pg_catalog. Unknown or unreadable profile evidence disables reuse and never disables primary identity verification. The backend login needs permission to read the three preload settings, which PostgreSQL can restrict to pg_read_all_settings. POOL grants no role or privilege. Operators may grant that read role only when they choose this conservative profile; a login without visibility remains dedicated. TEMP privilege is needed only for the temp-object workload, not for reuse itself. The integration fixture explicitly grants both for its reusable test login and tests refusal without profile visibility.

### Cleanup and handoff

1. Admit only Terminate or EOF exactly at a frontend message boundary with no Query in flight, a complete backend response boundary, known SQL eligibility and a live generation and route. EOF in the middle of a header/body closes. An arbitrary transport error closes. No release during shutdown.
2. Quarantine the backend. Under one cleanup deadline send ROLLBACK when status is T or E and consume exactly a ROLLBACK completion and ReadyForQuery I. A NoticeResponse is permitted only within the total byte bound; errors, unrecognized traffic and malformed packets close.
3. Send DISCARD ALL as its own Query outside a transaction. Consume expected CommandComplete and ReadyForQuery I, permitting bounded ParameterStatus and notices. No application response reaches a later client. NotificationResponse may be consumed only during quarantine within the bound and after UNLISTEN via DISCARD; any uncertainty closes.
4. Verify the profile remains eligible, role and startup settings match the physical backend's initial values and the generation/key/route is still current. Validate parameters rather than replaying stale startup ParameterStatus. Require idle. Use a bounded internal query to read the required startup settings, session_user/current_user and extension/hook evidence. Compare against the initial baseline. Do not silently replace the baseline with state from the earlier owner.
5. Publish idle atomically with eligibility. Recheck expiry, route freshness, node, epoch, authority revision and runtime generation at checkout and just before greeting. If any changed, drop and refuse this acquisition with the existing closed/unavailable error. No relabelling under a new generation. Handoff has one linearization point: while holding the pool/runtime snapshot mutex and a short RouteView read guard, validate generations, phase and route then atomically mark the backend Leased and assign its new lease generation. No await or greeting happens under those guards. Retirement takes the same pool mutex. Retirement before this point refuses handoff; retirement after it treats the frontend as an existing pinned owner. Holding the watch read guard through this short mark orders it against route publication; use lock order pool then route everywhere and never acquire pool from a route publisher. Idle publication uses the same rule. Force both sides of this interleaving in tests.

Cleanup reads at most 64 KiB total and each message at most 16 KiB, with a maximum count of 256. A failed reset closes and is never returned idle. A queued asynchronous packet at the next boundary cannot become a later owner's response: the final internal validation must consume through its own ReadyForQuery, and idle checkout validates again under the acquisition deadline. An idle backend is not trusted merely because time has passed. A backend with a LISTEN history receives DISCARD before publication; its former subscriptions are removed by the trusted server.

### Exact baseline and notification ordering

The internal baseline question is one SELECT returning exactly one row of text fields in this order: current_setting of application_name, client_encoding, DateStyle, IntervalStyle, TimeZone, standard_conforming_strings, default_transaction_read_only and search_path; session_user and current_user cast to pg_catalog.text; and one boolean text field proving no extension except plpgsql, no event trigger, and empty current_setting for shared_preload_libraries, session_preload_libraries and local_preload_libraries. Call each current_setting and every catalog with pg_catalog qualification. No missing, NULL or duplicate row, extra command, extra column, malformed text or unknown response is eligible. The builder uses this exact semantic query in one constant, with the bounded one-row response parser. Baseline eligibility also requires the exact search_path value `"$user", public`, `pg_catalog`, `public`, or `"$user"`. These retain implicit or explicit pg_catalog precedence; every other startup path stays dedicated. This protects recognized signed literals and other core operator/type resolution without claiming a general SQL parser. Internal attestation comparisons also use OPERATOR(pg_catalog.=) and OPERATOR(pg_catalog.<>) to avoid an unsafe baseline influencing the check itself. Baseline eligibility requires client_encoding UTF8, standard_conforming_strings on, default_transaction_read_only off, identical session_user/current_user equal to mapped login and hook evidence t. An unsuccessful profile check leaves a dedicated backend, except an uncertain/protocol-broken connection which closes.

Keep the complete startup ParameterStatus map, limited by spec 0004. Duplicate startup names disqualify reuse. Eligible names are server_version, server_encoding, client_encoding, application_name, is_superuser, session_authorization, DateStyle, IntervalStyle, TimeZone, search_path, integer_datetimes, standard_conforming_strings, default_transaction_read_only, in_hot_standby and scram_iterations. The required names are server_version, server_encoding, client_encoding, application_name, is_superuser, session_authorization, DateStyle, IntervalStyle, TimeZone, search_path, integer_datetimes, standard_conforming_strings, default_transaction_read_only and in_hot_standby. An extra or missing required name disables reuse. During quarantine, collect latest ParameterStatus by name in the same bounds; unknown names close, and final changed values must equal the initial map for every reported mutable setting. Fixed startup values retain their initial value, and the exact baseline row must match again after reset and at checkout. A mismatch closes; never adopt it as the new baseline. The greeting uses the original map only after these comparisons pass. A dedicated first lease keeps spec 0004's greeting.

DISCARD ALL includes UNLISTEN *, which cancels registrations. It runs in its own committed command before a second internal SELECT barrier. PostgreSQL's ordered stream and the UNLISTEN guarantee are the basis of the supported LISTEN history: previously emitted packets must precede this later command's completion; notifications generated after UNLISTEN have no old subscription. Consume bounded NotificationResponse packets only before the DISCARD ReadyForQuery, never publish them to a later owner. An A packet during the post-DISCARD barrier or checkout validation closes the backend. Test a notification queued just after the frontend's last ReadyForQuery and before cleanup, and notifications racing DISCARD and checkout, without forwarding any old payload. Never allow notification traffic observed during idle validation to reach a frontend. Scope 006 must requalify the relay when it adds broader asynchronous handling. This proof assumes the trusted PostgreSQL 18 server; no server extension that changes protocol ordering is supported.

### Admission and configuration

New keys in [limits] are optional u32 integers. They are not secrets and take effect on restart. Unknown keys remain errors. They follow the existing validation order, out of range finding and key location rules of spec 0004. Existing files keep working.

| Key | Default | Inclusive range | Meaning |
|---|---|---|---|
| backend_max_count | 100 | 1..100000 | All physical session backends over all nodes, including dedicated paths, excluding monitor probes |
| database_backend_max_count | 100 | 1..100000 | Per cluster and actual backend database |
| identity_backend_max_count | 100 | 1..100000 | Per cluster, actual database and login |
| backend_idle_timeout_ms | 30000 | 1..3600000 | Maximum idle age before drop |
| backend_lifetime_ms | 3600000 | 1..86400000 | Maximum physical age for a new lease |
| backend_cleanup_timeout_ms | 5000 | 100..60000 | One total reset and validation deadline |
| client_write_timeout_ms | 30000 | 100..3600000 | Maximum time for one relay write/flush, including greeting |

Existing node_backend_max_count and backend_wait_max_count remain nested limits, with the existing acquire timeout. No cross rule requires child caps below parent caps; the effective cap is their minimum. Idle capacity is bounded by backend_max_count, no warm minimum is added. Evict expired entries on acquire and release and use a periodic maintenance tick at most one second apart in Server::serve to close expired or retired idle connections even with no new client. A hermetic caller can drive the same maintenance function. A waiter ticket is removed on cancellation and timeout. A notify is registered before state is inspected so no wakeup can be lost. FIFO applies to queued acquisitions; head of line blocking is accepted and bounded by the deadline. All physical session backends, including those disqualified from reuse, take the same nested permits. For the FIFO head, first select a matching eligible idle backend, validate it under the same acquisition deadline and transfer its existing guards to the new lease without reserving capacity again. Only when no usable match exists, identify the exhausted caps in the single pool state. Evict the oldest idle entry that contributes to at least one exhausted cap, re-evaluate and repeat until all caps can be reserved together or no useful idle entry remains. Ties use insertion order. Do not evict entries unrelated to every exhausted cap. Reserve all counts atomically or none, then transfer their RAII guard to the connecting owner. A cancelled head ticket is removed by Drop and notifies the next ticket. Dedicated backends use the same admission path and release counts only when their physical socket closes. Permit acquisition must not retain a partial set while waiting for another cap.

Keep the current kernel TCP keepalive times. A direct socket2 dependency and platform TCP_USER_TIMEOUT are not needed for the slow reader guarantee because the write deadline owns it. This resolves the dependency choice raised in spec 0004 Follow-up 13 without adding optional socket configuration.

### Rotation and internal interface

| Surface | Inputs and source | Result and errors |
|---|---|---|
| Pool acquire | Verified identity route, canonical startup key, current route, monotonic deadline | An owned physical backend with all permits or typed overload/timeout/unavailable, mapped to existing SQLSTATEs |
| Pool release | Owned backend and terminal proof from relay | Clean idle or closed; never retries application SQL |
| Server::rotate_credentials | Complete Vec of IdentityId and protected, already loaded Arc credential | Atomic checked generation advance and publication, removes old idle. Missing/duplicate/unknown identities or generation exhaustion refuse whole publication. No old waiter/handshake receives a lease. |
| Server retirement | Shutdown and credential replacement; route changes from the existing control watch | Removes new eligibility before any later handoff. Running sessions retain their own snapshot until release. |
| pool run SIGHUP on Unix | Reload only identity credential files with the existing owner/mode checks | Validate all files before publish; failure retains old generation and emits constant reason. No TOML, principal, monitor or node reload. Full configuration reload remains scope 009. |

Runtime configuration generation denotes the immutable serving material snapshot and advances with every credential publication. Credential generation advances for every replaced identity in that same atomic operation. Ordinary configuration edits require restart, so no supported live publication of a different TOML is claimed. Tests can drive the same rotation entry point during wait, handshake and cleanup; do not use test only counters disconnected from actual acquisition state. Shutdown retires the serving snapshot before it waits.

Credential derivation needs a single owned task per active challenge: that task retains the cache lock and publishes the completed result independently of waiter cancellation. Do not start a thread or unbounded detached derivations. Retain the existing challenge work factor bounds and secret redaction. At most one owned derivation runs per Credential. Identical challenges coalesce; a different challenge waits for the current result in the caller future, never in a detached task, before a new derivation may start. A process wide semaphore with 64 slots bounds owned derivation jobs across retired credentials and monitor credentials. Reserve a slot before spawning; exhaustion returns a typed backend authentication/unavailable failure without spawning. The owner retains the cache state and permit through completion and publishes success or a constant failure, including a blocking task panic, then wakes all waiters. Cancelled caller futures cannot release the owner's slot or permit a replacement while it runs. A retained cache has one challenge entry, so varying salts cannot grow memory.

### Value sourcing and observability

| Value | Source |
|---|---|
| Backend identity partition | Validated identity map plus fresh RouteView |
| Generations and eligibility | Server's coherent runtime snapshot, monotonic checked advance, route watch and Phase |
| Startup baseline | New physical backend startup parameters and bounded internal settings query |
| Physical identity and cleanup proof | PostgreSQL PID, catalog responses and complete ReadyForQuery, never a scripted claim in real evidence |
| Ages and deadlines | tokio::time::Instant and validated millisecond keys |
| Lease generation | Existing checked per run counter, once per actual handoff |
| Limits and current occupancy | Validated Limits and permits physically held across all backend states |

Use bounded counters for opened, reused, reset closed, expired and admission refusal, with existing structured reporting where appropriate. The credential signal events are credential.reloaded with numeric identity_count, and credential.reload_failed with constant reason files or generation. Repeated SIGHUP while one load runs is coalesced. At shutdown a pending load loses publication authority; because a running blocking filesystem call cannot be aborted, runtime shutdown waits at most END_TIMEOUT (2 seconds) after serve returns. No file contents or error text is emitted. No new metrics exporter or administration surface. Events may contain only constant reason names, configuration IDs and numeric counts; no SQL, credentials, database/login names, file paths or peer error text. Debug of captured SQL and startup profiles is redacted. New public terms belong in the glossary before code.

### Tooling and regression debt

Complete spec 0004 Follow-up 14 in the existing check programs, without changing the CI job set or installing tools: per crate Clippy disallowed standard library paths for W3; W5 derived from Cargo.lock dependency lists including explicit core/control exclusion both ways and detecting a deny hidden by allow; private key block guard with the sole committed test key exception; exact six environment settings of .cargo/config.toml; CI rejection of POOL_IT_FILTER; NOTICE version comparison with Cargo.lock. Each needs a passing baseline and proof that a deliberately wrong fixture is refused. The owning rule text in spec 0002 gets dated notes from the Leader, not worker edits to specs.

Complete spec 0004 Follow-up 16 by extracting only the minimum test seams for forced task drop and daemon supervision. Prove that a never ending connection task is dropped after the last shutdown bound, and that an ended control/probe/listener/log task ends the daemon and returns exit 3 with daemon.failed. Keep the allocation regression within safe first party Rust: test rejection before reserve through an explicit safe reservation seam or document why allocator level observation remains unavailable; no unsafe counting allocator.

### Tests and who owns them

Use only the existing built in harness and paths from spec 0004. Add core pool/recognizer unit modules, session TLS/SCRAM scripted cases in crates/pool-core/tests/session.rs, backend cleanup scripts in src/backend/tests.rs, config validation in crates/pool-config/tests/validate.rs, daemon unit cases and PostgreSQL tests in apps/pool/tests/integration.rs. The real tests run only via scripts/check.sh integration, in disposable pinned PostgreSQL 18 containers. No dev dependency, fixture private key or fuzz package dependency is added. New recognizer is pure, total on bounded bytes and needs adversarial quote/comment/multistatement tests; it does not widen the supported protocol.

## Build plan

1. [x] Enforce inherited tooling and task lifecycle regressions, AC-9.
2. [x] Add configuration limits and one real pooled reconnect path with the conservative profile, cleanup quarantine, owned nested permits and fresh leases, AC-1 through AC-5, AC-7.
3. [x] Add coherent credential rotation, generation retirement and persistent derivation ownership, AC-2, AC-6, AC-8.
4. [x] Exercise contention, cancellation, expiry, identity and startup isolation, malformed cleanup and profile retirement through hermetic and real PostgreSQL cases, AC-1 through AC-9.
5. [x] Run required checks, integration and Linux validation, independent review, documentation and scope/context reconciliation, AC-10.

Steps 2 and 3 are one shared core ownership task. Tooling is disjoint and can proceed in parallel. Real reconnect evidence, including backend_max_count = 1 with unchanged PID, comes before widening SQL cleanup cases. The Leader integrates stopped writers sequentially and reruns affected checks.

## Consequences

Reconnects within the qualified profile can avoid a backend handshake while preserving session ownership and primary routing. Bounded quarantine and nested budgets cost extra internal queries and can create queue head of line delay. Most arbitrary application SQL continues to work through the dedicated path but does not qualify for reuse. No throughput or latency improvement is claimed without benchmarks. Core/control boundaries and the locked dependency set stay intact.

## Follow-up

Scope 006 must requalify reuse for extended protocol, COPY, asynchronous traffic and virtual cancellation; each new protocol state starts closed for reuse. Scope 008 owns richer diagnostics and operational drain. Scope 009 owns full configuration reload. This spec claims no extension state cleanup or general SQL parser. The recognizer may be expanded only with a spec decision and new isolation evidence.

## Rationale

See [rationale.md](rationale.md).
