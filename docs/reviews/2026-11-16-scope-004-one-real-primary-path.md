# Review, main (scope 004, one real primary path), 2026-11-16

**Reviewed by**: `claude-fable-5-2` for the code, the manifests, the guards, the scripts and the tests (author on `claude-opus-5-5`), with a second independent read of the production code by `claude-sonnet-5-5`, and `claude-opus-5-5` for spec 0004, the notes in spec 0001 to 0003 and the other documents (author on `claude-fable-5-2`). Each part was read by a model that did not write it. The main reviewer is a session that had not seen the implementation before this review.
**Scope**: the range `c4d5714..b2aec30` on `main` (249 commits), read with `git diff c4d5714..b2aec30 -- <path>`. 147 files without `graphify-out/` and `docs/scope/coordination/`, which were left out on purpose.
**Verdict**: Changes requested. No blocker. 3 major findings, 29 minor findings and 15 nits. The six reviewers rated ten findings major between them. I checked each against the code, ran what could be run, and keep three as major. The reasons for the other seven are with the findings (c4, t1, t2, t4, t5, d1, d2).

## Summary

The change builds the first real path: two new crates for the wire code (`pool-protocol`, `pool-core`), the journal on disk and the snapshot in `pool-control`, three commands and the daemon in `apps/pool`, and the first runtime, TLS and SQLite dependencies.

The code is careful, and the heart of it holds. I found no way for a byte of application SQL to reach a node before the client proved its password, the identity map gave an entry, and both looks at the route passed. I changed the code on purpose 23 times. Of the 12 changes that I chose on the safety lines of the session, the backend connection, the journal and the file checks, 11 were caught by a named test and one made the tests hang.

What is left falls into three groups. First, two real gaps in how the slice meets its own promise. A peer that vanishes without closing is never noticed (M1), and the server that the probe saw is not tied to the server that a session reaches (M2). Second, one line that decides whether running sessions end on a second primary has no test that would see it break (M3). Third, a long list of smaller points, most of them tests that pass when the behavior they name is removed. All 11 changes that the test reviewers predicted would go unseen did go unseen.

Nothing here says the slice writes to a wrong node today. M1 and M2 say where it can stop telling you the truth about a lost or a moved node.

## How this review was run

- I read spec 0004 `index.md` first: the criteria, the commands, the state directory, the journal, the probe, the control loop, the wire subset, the session lifecycle, the records, the log events and the security model. Then I read the production code myself: every file of `crates/pool-core/src/`, the files `startup.rs`, `auth.rs`, `scram.rs` and `message.rs` of `crates/pool-protocol/src/`, the files `journal.rs`, `snapshot.rs`, `probe.rs` and the changes to the three older files of `crates/pool-control/src/`, and `control.rs`, `probe.rs`, `daemon.rs`, `state.rs`, `commands.rs`, `log.rs`, `main.rs` and `output.rs` of `apps/pool/src/`. Of `crates/pool-config/src/` I read only the limits, the probe keys and the principal checks of `serving.rs`. The rest of that crate, `base64.rs`, `sqlstate.rs` and `cli.rs` were read by the second reviewer and not by me.
- Six reviewers ran in parallel and did not see each other's results: two on `claude-sonnet-5-5` for the production code (wire and data plane, control plane and daemon), three on `claude-fable-5-2` (the tests of the two new crates, the tests of the daemon and the control crate, the scripts and guards) and one on `claude-opus-5-5` for the documents. None of them built or ran anything. I checked each of their major findings against the code before it went into this file.
- I ran `cargo test --workspace --locked` on the tree at `b2aec30` (debug build, macOS, Rust 1.100.0): 489 tests and 18 documentation tests passed, 0 failed, 26 ignored (the tests of `apps/pool/tests/integration.rs`, which need the fixture).
- I ran two probes with temporary tests and removed them again. One makes the scripted node stop inside `ReadyForQuery` (c1). One measures the startup parser on the largest packet the configuration allows (c3).
- I changed the code on purpose 23 times (a mutation probe: if the tests still pass, nothing pins that line), ran the tests of the package each time, and put the file back with `git checkout`. 11 changes were caught by a failing test, 1 made the test binary hang for ten minutes until I stopped it, and 11 passed every test. The changes are named in M3, t1 to t7 and under [Test coverage](#test-coverage).
- I read the build scripts of the locked `aws-lc-sys` 0.47.0 and `libsqlite3-sys` 0.40.0 in the local registry to check finding c4.
- I did not run `scripts/check.sh all`, `scripts/check.sh integration`, `scripts/prove-checks.sh`, the Linux build or the fuzz targets. The lead and the verifier ran those. A run on macOS is not Linux evidence. The tree was clean after every probe.

## The seven questions of the brief

**1. Do the invariants hold?** Mostly, with the gaps M1 and M2.
- Writes on the verified primary: a login needs an open route that is still fresh (`crates/pool-core/src/route.rs:75`, `crates/pool-core/src/session.rs:610`), a backend that reports `in_hot_standby` as `off` in its last report (`crates/pool-core/src/backend.rs:523`), and the same route again after the backend is ready (`session.rs:666`). Removing any of the three fails a named test. What the checks do not show is that the session reached the server the probe saw (M2).
- Closed when the role is unknown or the evidence is old: every probe answer that is not exactly one row of three well formed columns is a failed sample (`crates/pool-control/src/probe.rs:55`), a failed sample replaces the good one, and the reader of the route compares `fresh_until` itself, so a control loop that stops closes new logins without telling anyone. Two limits: the clock does not run while the host is suspended (c8), and a control loop that dies leaves running sessions with no one to end them (c2).
- Never send again: there is no second attempt anywhere. `in_flight` is set before the first byte of a `Query` goes out (`session.rs:1022`), and clearing it fails five tests. A lost answer gives `08007`. But a node that goes silent gives nothing at all (M1).
- Uncertain state is closed: the lease owns the backend and is dropped on every way out of the task. I found no path that keeps a backend after its session, apart from the stuck read of c1.
- A health failure does not allow promotion: nothing in the range changes an approved writer. `Journal::create` writes it once, from a plan that needs a fresh sample of every node, and `set_conflict_latch` can only close.
- No secret in telemetry: every type that holds a secret, a nonce, a name from a client or SQL has a hand written `Debug`, no error type carries text from a peer or from SQLite, and the four `expect` calls outside tests have constant text. The guard for this is weaker than the code (t2).

**2. Can client bytes reach a node early, and can a backend stay open?** I found no way. The first packets are read with exact reads and decided on eight header bytes (`crates/pool-protocol/src/startup.rs:353`), so nothing read before the TLS handshake is used after it. A `StartupMessage` without TLS is refused before its body is read. Before step 10 a node gets only the startup packet and the SCRAM messages of POOL. The two values of the client in that packet (`application_name`, `client_encoding`) cannot hold a zero byte, so they cannot add a parameter. After login POOL frames by length only and copies exactly the bytes of a `Query`. A backend is dropped on every refusal after step 8, when shutdown cuts a login, and when the task panics. It is closed up to one second late when the client does not take the close (n1).

**3. Native code and crypto.** The use of the interfaces is right. Proofs and signatures are compared with `constant_time::verify_slices_are_equal` (`crates/pool-core/src/scram.rs:270`, `:337`). All random bytes come from one call with no fallback (`crates/pool-core/src/random.rs:22`), and a nonce is 18 bytes. The count from a node is held to 600000 before PBKDF2 runs, and the function cannot be called with 0. Every length from a peer is checked before anything is reserved (`crates/pool-protocol/src/message.rs:144`, `startup.rs:67`, `backend.rs:322`). The journal is opened without the flag that creates a file, with WAL checked, `synchronous = FULL` and no wait on busy. Two points: which native library gets linked can change with the build host (c4), and the promise of one derivation at a time does not hold when a login is cut (c5).

**4. The heavy decisions taken in delegated mode.** I judge all nine that the brief names as right: 57, 59, 61, 62, 66, 69, 70, 71 and 83. Decision 71 matches the server: PostgreSQL sends the whole protocol code. Three have a consequence that is not written down. For 59, a checkpoint that fails after `create` is ignored (`crates/pool-control/src/journal.rs:344`), so the sentence that the main file alone carries the journal holds only when the checkpoint worked (d3). For 62, the rule text says the key is read only under `#[cfg(test)]`, and an integration test reads it too (d5). For 66, the last report proves "not a standby" and not "the server the probe saw" (M2). Outside the nine: decision 4 leaves clients that do not check the certificate of POOL worse off than a direct connection, and that is not written (d2), and the security model contradicts decision 10 (d3). The rationale holds 29 decisions marked heavy, not 21.

**5. Do the fixes close the class?**
- The relay that did not race with a cut: closed for every write and for the main loop. One sibling is open and proven: the read of the status byte of `ReadyForQuery` (c1).
- `in_hot_standby`: closed. The last report decides and a missing one fails, and both have a test that fails without them.
- `Debug` that printed content: closed in the code. No derived `Debug` is left on a type with content. The tests that should keep it so cannot see a field of bytes (t2).
- `NegotiateProtocolVersion`: the bytes are pinned. That the listener sends the message at all is not pinned by any hermetic test (t1).
- The bootstrap that stopped: every network wait is under the deadline. A lookup of a host name that stalls can still make the command outlive its deadline (n10).
- The race on the lock file: closed as far as I can read it. The file is checked on the open descriptor against the path, and the loser of a create race opens the file of the winner. The test for it does not show that a race took place (t7).

**6. Do the tests guard behavior?** The safety core is guarded well, and many edges are not. Of 23 changes, 11 passed every test. The tests that were unstable: the "never binds" check of `ac13_run_refuses_every_state_...` cannot fail and its port handling is a race (t5), the lock race test is bounded now and asserts no race (t7), and `Daemon::stop` still waits without a limit at 13 places (t4). The hold test that needs the `sqlite3` tool fails with the name of the tool when it is absent, it is not skipped, and it can be replaced by three hermetic cases (t6).

**7. Scripts and guards.** The baselines match the blocks of the spec line by line, each of the six groups of rule W3 has a proof case, the image is pinned by digest in one place, and the cleanup removes by exact name on every exit. The limits: W3 is a text match that the usual grouped import passes (g1), "one writer" holds by convention outside `pool-control` (g2), W5 can be passed by a rename (g3), and the integration check is green when no test runs (g4).

## Major

### 🟠 M1. A peer that vanishes without closing is never noticed, `crates/pool-core/src/session.rs:340`, `crates/pool-core/src/backend.rs:424`

**Problem**: Both sockets get `set_nodelay` and nothing else. A search over `crates`, `apps`, `scripts` and spec 0004 finds no keepalive and no deadline on a read. The relay waits on both sockets with no limit (`session.rs:957`), and while a query is in flight it reads only from the node. A peer that is gone without a FIN or a reset (a host that lost power, a network that drops packets, a NAT entry that expired) gives no event on such a socket. Two results. If the node goes silent after it took a `Query`, the client waits for ever and never gets the `08007` of `0004 AC-22`. The probe does see the loss, but silence closes new logins only and ends no session, by decision 10. If the client goes silent, its session permit, its principal permit, its backend permit and its PostgreSQL backend stay, with every lock an open transaction holds. PostgreSQL turns keepalive on for its own client sockets, so before POOL such a client was removed by the node in time.
**Why it matters**: The loss of the host of the primary is the failure this product exists for, and it is the one case where the user story of `R-10` ("a lost connection is reported as a lost connection") is not met. With the default of 100 backends per node, 100 clients that vanished close the node for every login until the daemon is started again. The spec moves idle limits and slow readers to scope 005 and says nothing about a dead peer, so I read this as a gap and not as a decision. The second reviewer rated it major on its own, and I agree.
**Suggested fix**: Decide it in spec 0004, in "Backend loss" and in the security model. Three shapes would work. Turn keepalive on with short times for every accepted socket and every node socket (the times need a small socket crate as a direct dependency, which is a decision of the spec). Or end the sessions of a node with `08007` or `08006` when its probe has failed for longer than a stated time. Or write down that it stays open until scope 005 and name what an operator sets in the kernel until then. Then one hermetic test with a stream that never answers. Owner: the architect, then the developer and the tester.

### 🟠 M2. The server the probe saw and the server a session reaches are not tied together, `apps/pool/src/probe.rs:121`, `crates/pool-core/src/backend.rs:421`, `:523`

**Problem**: The probe opens one connection and keeps it for the whole run. It connects again only after a failed round. A session opens a new connection to `node[].host` and `node[].port` each time, and a name is resolved again at each attempt. The one check on the connection of the session is that the last report of `in_hot_standby` is `off`. It compares no system identifier and no timeline. So if the address of a node comes to mean another server while the old connection of the probe stays up (a name that an HA tool moves, a service name in front of several servers), the probe goes on reporting the old server as the fresh primary, and new sessions go to whatever answers the address now. A promoted server of the same cluster passes the check. TLS ties a name to a CA and not to one server. The spec says a node gets SQL only while journal, configuration and a fresh probe agree, "checked twice per login and once more on the connection itself" (`docs/specs/0004-one-real-primary-path/index.md:1134`), which reads as more than the code does.
**Why it matters**: This is the invariant "writes on the verified primary only". The evidence is about one server and the writes go to another, and POOL cannot see it, because the second server is no configured node. How likely it is depends on how a deployment names its nodes, and I could not build the case on the fixture. Two reviewers on two models found it and rated it major. I keep it major because the least fix is one paragraph and a reader who misses that paragraph can lose the one thing the slice promises.
**Suggested fix**: At the least, write it into "Backend connection" and the security model: a node entry must name one server whose address does not move, each node needs a certificate name of its own, and the check on the connection covers the role only. Better, close it in code with one of two changes: let the probe open a new connection every round or every few rounds, so that it dials what a session dials, or ask the node for its system identifier on the connection of the session before `AuthenticationOk`. A test then moves the address between a probe and a login. Owner: the architect, then the developer and the tester if code is chosen.

### 🟠 M3. No test fails when a second primary beside a silent writer stops ending sessions, `apps/pool/src/control.rs:54`

**Problem**: `route_view` sets the conflict flag when any reason of a closed cluster is a conflict. `writer_admission` reports every reason that applies, so a writer the probe cannot reach beside another node that reports as a primary gives `evidence.writer_missing` together with `evidence.second_primary`. No test passes a list that mixes a conflict with a reason that is none. I changed `any` to `all` and `cargo test -p pool --locked` passed in full. The unit test checks the six reasons one at a time, the route test uses two conflicts or one silence, the control loop tests record both nodes in every sample, and the one conflict of the fixture is a wrong system identifier on a cluster of one node.
**Why it matters**: This is the case where ending running sessions matters most: clients still hold connections to the old writer, the probe has lost it, and another node was promoted. With `all` the flag is false, the rule "silence does not end a session" applies, and the sessions go on writing to the old primary. The same holds after a restart with a latched journal and an empty board. The code is right today. The line is one word from wrong, and nothing would say so.
**Suggested fix**: Two cases in `ac14_route_view_marks_a_positive_conflict_and_not_silence`, both expecting a closed route with the conflict flag set: a controller in which `pg1` has no sample or a failed one and `pg2` reports as a primary, and one built from a latched journal with an empty board. On the fixture, a second node entry whose port is a relay toward the first container gives two primaries with one system identifier without a base backup (t9). Owner: the tester.

## Minor

Code

### 🟡 c1. A session that waits for the status byte of `ReadyForQuery` cannot be ended, and shutdown then never returns, `crates/pool-core/src/session.rs:1090`, `:388`

**Problem**: Every wait of the relay is raced against a conflict and the end of shutdown, except the one that reads the sixth byte of `ReadyForQuery`. I ran it: a scripted node writes the five header bytes and stops. After `h.close(true)` the session was still alive and the backend was not closed. With `shutdown_timeout_ms = 200`, `Server::shutdown` had not returned after 5 seconds. The wait after `Phase::Ending` has no limit (`session.rs:388`), a second signal does nothing by design, and when a signal handler cannot be installed the daemon waits for that signal for ever with no finding (`apps/pool/src/daemon.rs:234`).
**Why it matters**: It breaks `0004 AC-14` (a positive conflict ends every running session) and `0004 AC-25` (the daemon exits when the deadline has passed) for a node that stalls at that byte. That needs a node that misbehaves or a loss exactly between two records, so it is rare. It is the same class as the relay defect that was fixed, and it is the one sibling still open.
**Suggested fix**: Read the status byte inside `guarded`, give the wait after `Ending` a last limit after which the remaining tasks are aborted, and treat a handler that cannot be installed as a failed start. Keep the probe as a test. Owner: the developer.

### 🟡 c2. Nothing watches the control loop, the probes and the listener task, `apps/pool/src/daemon.rs:243` to `:290`, `apps/pool/src/control.rs:117`

**Problem**: The four kinds of task are spawned and looked at again only at shutdown. When the control loop ends (a panic, or every probe task gone, which ends the loop without a line at `control.rs:117`), new logins are refused once the routes are old, which is right. But running sessions can no longer be ended on a conflict (`session.rs:959`), a latch can no longer be written, and no event says so. A listener task that ends leaves a process that accepts nothing.
**Why it matters**: The spec says a process that looks alive and admits nothing is harder to notice than one that did not start. This is that process, and its running sessions are blind to a second primary.
**Suggested fix**: Wait on the join handles beside the signals. On an end that was not asked for, write an event, end every session and exit with a status that is not 0. Owner: the architect for the event and the status, then the developer.

### 🟡 c3. The check for a repeated startup parameter is quadratic, before authentication, `crates/pool-protocol/src/startup.rs:234`

**Problem**: Each pair is compared with every pair before it. I measured a release build: a packet of 8192 bytes with 1636 names takes 7.0 ms, and one of 65536 bytes (the largest value of `limits.startup_packet_max_bytes`) with 11561 names takes 107.7 ms. It runs on a worker of the runtime, after the TLS handshake and before any proof.
**Why it matters**: At the default the parser already costs several times a handshake. With the limit raised, 64 connections that send one packet each hold every core for a tenth of a second per packet, from a client that knows no password.
**Suggested fix**: Keep the names in a set, or stop at the first name that is not one of the known ones, since such a name is refused anyway. Owner: the developer.

### 🟡 c4. Which native library is linked depends on the build host, `Cargo.toml:23`, `:24`, `scripts/check.sh:34`

**Problem**: Spec 0004 and `NOTICE` say AWS-LC and SQLite are built from the source inside the locked crates. Two build scripts decide otherwise from the environment. `aws-lc-sys` 0.47.0 looks for an installed AWS-LC through `OPENSSL_DIR` and `pkg-config` unless `AWS_LC_SYS_USE_SYSTEM=0` is set, and links it when it finds one that fits (`builder/main.rs:737` of the locked crate, which I read). `libsqlite3-sys` 0.40.0 links the SQLite of the host when `LIBSQLITE3_SYS_USE_PKG_CONFIG` is set (`build.rs:81`). The repository sets neither, and has no `.cargo/config.toml`.
**Why it matters**: A host with AWS-LC installed gives a binary with another crypto library than the one the lockfile, the license exceptions, the advisory check and `NOTICE` describe, and no file in the repository changes. The reviewer of the scripts rated this major. I rate it minor, because in the pinned image, which has a C compiler and nothing else, the source build is what happens, and the build host is a trusted input. The fix is two lines.
**Suggested fix**: Set both variables to 0 where every build passes (a committed `.cargo/config.toml` with an `[env]` table, or two exports in `scripts/check.sh` and in the Linux build), decide it in "Native code", and add one test that expects `rusqlite::version()` to be `3.54.1`. Owner: the architect, then the developer.

### 🟡 c5. "One derivation at a time" does not hold when a login is cut, and four blocking threads serve four jobs, `crates/pool-core/src/backend.rs:156`, `apps/pool/src/daemon.rs:79`

**Problem**: `Credential::keys` holds the lock across `spawn_blocking`. When the login that derives reaches its deadline, the lock is released while the PBKDF2 job runs on, and the next waiter starts a second one. The runtime has four blocking threads, and they also serve the lookups of host names, the latch write and the writer of stderr. A lookup that stalls, or a burst of cut logins, can keep the latch write waiting. Closing is not delayed, because the route is published before the write.
**Why it matters**: The spec states the bound as a fact (`index.md:931`), and the latch write is the one job on that pool that should never wait behind a client.
**Suggested fix**: Let the derivation run as a task that owns the cache update, so a cut login does not free the slot, and give the latch write a thread of its own or resolve names once at start. Owner: the developer.

### 🟡 c6. A CA file or a certificate chain that others can write is accepted, `crates/pool-core/src/secret.rs:146`

**Problem**: A secret file must have no group or other bit and the right owner. A CA file and the chain of the listener need only be a regular file that can be read. The CA file is the one thing that tells POOL which server is the node.
**Why it matters**: A local account that can write the file can add a root and then stand between POOL and the node. The trust root is protected less than the password it protects. The spec chose the rule (`index.md:627`), so this is a hardening gap, and it is not named under what the slice does not defend.
**Suggested fix**: Refuse a CA file or chain that group or others can write, or that is owned by neither the service account nor root, with `secret.permissions`. Or name the risk in the security model. Owner: the architect.

### 🟡 c7. An unknown name can be told from a real one in three ways, `crates/pool-core/src/session.rs:234`, `crates/pool-core/src/scram.rs:27`, `apps/pool/src/daemon.rs:152`

**Problem**: The made up verifier has a salt of 16 bytes and the count of the first principal. A verifier file may hold a salt of 1 to 64 bytes and its own count (`secret.rs:229`), so a principal whose values differ from the first one answers with another shape. And the key behind the made up salt is new at every start, so the salt of an unknown name changes over a restart while a real one never does.
**Why it matters**: `0004 AC-16` says a client cannot tell an unknown name from a wrong password. The spec names the count as an assumption, and nothing checks it. The salt length and the restart are not named.
**Suggested fix**: Refuse at start a set of verifiers whose counts or salt lengths differ, or log it once. For the restart, keep the key in the state directory, or limit the sentence to one run. Owner: the architect.

### 🟡 c8. Freshness is measured by a clock that stops while the host is suspended, and from the arrival of the row, `apps/pool/src/control.rs:128`, `apps/pool/src/probe.rs:97`

**Problem**: `now` and every sample time come from one `Instant`. On Linux that clock does not run while a machine is suspended or a virtual machine is paused. After a long pause, a sample from before it still looks as fresh as it did, for up to the rest of the expiry (60 seconds at most), until the next probe replaces it. Apart from that, a sample is dated when its row arrives, so the age of the evidence is understated by up to `probe.timeout_ms`, which may be 30 seconds. The second point is what the spec says.
**Why it matters**: A failover during the pause is not seen for that window. It is small, and it is exactly the case "old evidence serves a login".
**Suggested fix**: Name both in the security model. If you want the first closed, compare with the wall clock and close every route when the two clocks disagree by more than a small bound. Date a sample at the start of its query, which can only make it older. Owner: the architect.

### 🟡 c9. A latch that the journal does not hold yet is dropped at shutdown without a line, `apps/pool/src/control.rs:112` to `:124`

**Problem**: When `stop` changes, the loop ends. It makes no last try for the clusters of `unrecorded_latches()` and writes nothing about them.
**Why it matters**: With a disk that failed, the second primary that the process saw is forgotten at exit, and the next start can open the cluster again. The spec accepts that the latch can be lost with a failing disk. One more try and one line cost nothing.
**Suggested fix**: One more write for each cluster after `stop`, and `authority.conflict_latched` with `recorded` false for each one that is still open. Owner: the developer.

Tests

### 🟡 t1. Three sentences of the handshake have no hermetic test, `crates/pool-core/tests/session.rs:576`

**Problem**: The scripted client keeps what the listener sends only for a refusal. I ran three changes, and `cargo test -p pool-core --locked` passed after each: `NegotiateProtocolVersion` is never sent (`session.rs:500` of the source), the server signature is never sent to the client (`:551`), and `AuthenticationOk` is sent right after the proof, before the route checks and the backend (`:717`).
**Why it matters**: These are sentences of `0004 AC-15`, `AC-16` and `AC-14`. The first is the message that already went out wrong once. The fixture would catch the second, and it is not part of `scripts/check.sh all`. The test reviewer rated this major. I rate it minor, because a real client fails closed on the second, and the first and third change what a client is told and not where its SQL goes.
**Suggested fix**: Let the helper return the whole sequence of tags and authentication codes and assert it exactly, with `v` first and its golden body for a login that asks for 3.2. In the refusal helpers assert that no `AuthenticationOk` was seen. Owner: the tester.

### 🟡 t2. The redaction tests cannot fail for a field of bytes, `crates/pool-protocol/tests/redaction.rs:13`

**Problem**: The tests look for the canary text in the `Debug` output. A derived `Debug` prints a `Vec<u8>` as a list of numbers, which holds no such text. I put `#[derive(Debug)]` on `StartupRequest` in place of its hand written one, which prints `user` and `database`, and the tests of both crates passed. The same blindness covers `ParameterStatus`, `Forwarded` and the readers in the middle of a body. Five types that hold a name or a secret have no `Debug` test at all (`VerifiedPrincipal`, `Lease`, `Credential`, `ServerParts`, `Server`). On the fixture, the log is checked in 3 of 17 daemon runs (`apps/pool/tests/integration.rs:686`), and the listen host is not in the forbidden list.
**Why it matters**: `0004 AC-24` rests on these hand written impls, and the likely slip is one derive. The code is right today, which is why I rate this minor and not major as the test reviewer did.
**Suggested fix**: Assert the exact `Debug` text of each type, as line 31 does for `Message`. Run the log check inside the stop helper so every test gets it. Owner: the tester.

### 🟡 t3. "No byte of it reaches PostgreSQL" is counted in whole messages, `crates/pool-core/tests/session.rs:1049`

**Problem**: The scripted node records a query only when a whole message arrived. I made the relay write the five header bytes of an oversized `Query` to the node before the refusal, and every test passed.
**Why it matters**: `0004 AC-20` says no byte. Half a frame is harmless while the backend is closed afterwards, and it is a broken pooled connection in scope 005.
**Suggested fix**: Let the node count every byte after its `ReadyForQuery` and assert 0. Owner: the tester.

### 🟡 t4. A defect of the kind these tests exist for makes the run hang, `crates/pool-core/tests/session.rs:203`, `apps/pool/tests/integration.rs:588`

**Problem**: The reads of the scripted client have no time limit. When I removed the conflict check at the top of the relay loop, the test binary did not fail. It hung for ten minutes until I stopped it. On the fixture, `Daemon::stop` and `stop_after_signal` wait for the child without a limit at 14 places, the shutdown test reads without a deadline (`integration.rs:1558`), the helpers that run `pool` and `psql` wait without a limit, and `scripts/integration/run.sh:141` runs the tests without a limit, so its cleanup runs only when they end and the containers stay.
**Why it matters**: The harness has no timeout per test. A hang shows no test name and no assertion, it costs a job its whole time limit (15 or 30 minutes in the CI file), and it happened once already in the verification record. The test reviewer rated this major. It harms the feedback and not the product, so I rate it minor.
**Suggested fix**: One helper with a limit around every read of the scripted client and the two bare `shutdown()` calls. On the fixture, give `stop` the polling that `stop_within` has, with 20 seconds as the default, and add `connect_timeout` to both connection strings. Owner: the tester.

### 🟡 t5. The "never binds" check of `0004 AC-13` cannot fail, and its port handling is the intermittent failure, `apps/pool/tests/commands.rs:617`, `:99`

**Problem**: `assert_not_serving` waits for `pool run` to exit and then binds the port. The listener of a process that has exited is gone, so the bind works whether or not the port was bound. I added a bind before the journal is classified, and the tests passed. What the line does test is whether another thread of the test binary holds that port at that moment, because `closed_port()` binds port 0 and drops it. The same check is at `apps/pool/tests/integration.rs:842` and `:2056`.
**Why it matters**: The verification record cites "the port is free after each" as the proof. And a required check fails now and then for a reason that is not the product.
**Suggested fix**: Hold a listener of the test on the port for the whole of each `pool run`. A run that reached the bind then exits 3 with `listener.failed`, so the check means something and the race is gone. Owner: the tester.

### 🟡 t6. A journal on hold is refused only in a test that needs Docker and the `sqlite3` tool, `crates/pool-control/src/journal.rs:547`

**Problem**: I made `restore_pending` read as no hold, and the tests of `pool-control` and of `pool` passed. No hermetic test sets the hold in a file, and `unconfirmed` and `mismatch` are tested through `pool run` only on the fixture. When `sqlite3` is absent, the fixture test fails and names the tool. It is not skipped and it does not pass.
**Why it matters**: Half of `0004 AC-13` is proven only in a job that CI does not have yet. No command sets the hold in this slice, which keeps this minor.
**Suggested fix**: One case in `crates/pool-control/tests/journal.rs` that sets the hold with the existing `damage` helper, a unit test of `not_ready` over the six states, and two cases in `commands.rs` with `real_journal`. That also removes the need for the host tool. Owner: the tester.

### 🟡 t7. Four more lines that no test holds, `apps/pool/src/daemon.rs:213`, `:235`, `crates/pool-control/src/journal.rs:374`, `:424`

**Problem**: Each of these passed every test of its package. The lock on the state directory is dropped as soon as the daemon serves: the holder in the tests is the test itself or a `pool bootstrap`, never a running daemon. SIGINT is not handled: every stop is a SIGTERM. The authority ID is not compared again in `open_ready` and in `set_conflict_latch`: the tests move the revision and the start sequence only. And the lock race test accepts exit 0 or 3 from every child and asserts no count, so it passes when no race took place (`apps/pool/tests/commands.rs:715`).
**Why it matters**: "Two daemons cannot share one state directory" is the purpose of the lock, and `0004 AC-25` names both signals.
**Suggested fix**: On the fixture, start `pool show-authority` and a second `pool run` while the daemon serves and expect `state.locked`, and stop one daemon with SIGINT. Two `damage` cases for the ID. Assert that the count of locked children is above 0. Owner: the tester.

### 🟡 t8. Sleeps and narrow time windows decide several results, `crates/pool-core/tests/session.rs:711`, `:1098`, `apps/pool/tests/integration.rs:1205`, `apps/pool/src/control.rs:506`

**Problem**: `settle()` is a fixed 50 ms before an assertion. In two tests it must be long enough for the relay to take a `Query` before a conflict or a shutdown arrives, and the relay picks at random when both are ready. The waiter test allows 250 ms for a whole login and has a deadline of 300 ms. On the fixture five pauses decide who is the waiter, and the lower bound of the deadline is measured from before the client process starts, so it proves less than the "not before it" of `0004 AC-20`. One test of the control loop polls once and needs a blocking write to be still running.
**Why it matters**: A test that fails once in a few hundred runs teaches people to run it again.
**Suggested fix**: Wait on the state itself in place of a sleep, use `eventually` for every count, widen the waiter test as the fixture did, and start the clock of the waiter when its connection is made. Owner: the tester.

### 🟡 t9. What the fixture does not show, `apps/pool/tests/integration.rs:1170`, `:897`

**Problem**: The rollback test ends with a clean `Terminate`, because `psql` sends one at the end of its input, and it does not check that the INSERT ran. No fixture test drops the socket inside a transaction, which is what the scope and the walking fixture name. The refusals of POOL are compared by message text and never by SQLSTATE, and the deliberate error of `0004 AC-18` is compared without its code. There is no second primary, no node that restarts and no sample that expires with nothing to replace it.
**Why it matters**: The hermetic tests hold the abrupt case and the codes, so nothing is unproven. It is unproven against PostgreSQL.
**Suggested fix**: Assert the output of the INSERT, add the abrupt case with the raw client, add `\set VERBOSITY verbose` to the SQL file, and add the second primary through a relay (see M3). Leave the standby to scope 007 and say so in the record. Owner: the tester.

### 🟡 t10. Two fuzz targets reach less than they claim, and the golden messages are checked against the code that wrote them, `crates/pool-protocol/fuzz/fuzz_targets/scram.rs:19`, `crates/pool-protocol/tests/golden.rs:245`

**Problem**: The `scram` target takes the nonce from the first bytes of the same input, so `parse_client_final` can never get past its nonce check and the decoding of the proof is not reached. The parsers of `auth.rs` (among them `parse_sasl_initial_response`, which reads bytes of a client before any proof), `error_sqlstate` and `data_row` are in no target. Literal bytes exist only for the twelve errors and for `NegotiateProtocolVersion`. Every other golden message is written by the encoder and read back by the parser of the crate. The second sentence of `0004 AC-28` (nothing is reserved before the limit check) has no test that could see a reservation.
**Why it matters**: I read each parser and found no defect. The safety of the parsers of the two peers rests on reading, and a mistake that is the same in encoder and parser passes `scripts/check.sh all`.
**Suggested fix**: Take the nonce from a part of the input of its own, add a target for the parsers of `auth.rs`, and add literal bytes for the six short messages that the PostgreSQL documentation fixes, with the verifier of RFC 7677 as a literal. Owner: the architect for the target list, then the tester.

Guards and scripts

### 🟡 g1. Rule W3 is a text match that everyday Rust passes, `tools/repo-check/src/workspace.rs:175`

**Problem**: The tree is clean today. But `use std::{fs, env};` or `use std::{sync::Arc, thread};` names no forbidden text. File calls that are methods of `Path` pass, and `journal.rs:259` already calls one by a written decision. A directory named `target` is skipped at any depth, links are skipped, only `apps/` and `crates/` are walked, a `build.rs` is outside the `src` lines, and `UnixDatagram` has no line.
**Why it matters**: Spec 0004 retires the hand run search of `0003 AC-11` in favor of this guard. "No thread" and "control reads no clock, file or environment" now rest on it.
**Suggested fix**: Keep W3 for the five guarded names and for `rusqlite` and `tokio`, where a use has to spell the name. For the paths of the standard library use what resolves names: a `clippy.toml` per crate with `disallowed-methods` and `disallowed-types`, which the required `clippy` check already enforces. Say in the spec that W3 is a tripwire. Owner: the architect.

### 🟡 g2. "One writer" and "only `journal.rs` names `rusqlite`" hold by convention outside `pool-control`, `tools/repo-check/baseline/forbidden-source-patterns.txt:36`

**Problem**: `rusqlite` is refused only under `crates/pool-control/src`. No guard says which member may take which third party dependency, so `apps/pool` can take `rusqlite` and open the journal on a worker of the runtime with every check green.
**Suggested fix**: Make `rusqlite` a bare line with an allow for `journal.rs` and for the tests of `pool-control`, the same for `aws_lc_rs` with `pool-core`, and one proof case. Owner: the architect, then the developer.

### 🟡 g3. Rule W5 can be passed, and a `deny` line can be dead, `tools/repo-check/src/workspace.rs:382`, `:165`

**Problem**: A dependency whose inline table spans lines hides its `package =` rename from the one line parser, so a renamed dependency on `pool-core` is no pair for W5 (traced by reading, not run). Nothing in code refuses the pair `pool-core` and `pool-control` if a change adds it to the baseline, and no test compares that baseline with the six lines of the spec. Apart from that, a `deny` line under a wider `allow` of the same text has no effect, and the tool says nothing.
**Why it matters**: W5 is the one check of "core and control never depend on each other".
**Suggested fix**: Take the edges from the `dependencies` lists of `Cargo.lock`, refuse the pair in code in both directions, and report a `deny` line that an `allow` line covers. Owner: the developer.

### 🟡 g4. The integration check is green when no test runs, and its cleanup removes by a name built from a process ID, `scripts/integration/run.sh:141`, `:30`

**Problem**: The result is the exit status of `cargo test ... -- --ignored`, which is 0 for zero tests, and a test added without `#[ignore]` never runs on the fixture. The containers are named with `$$` and removed by name whether or not this run made them, so two runs in different process namespaces on one Docker daemon can remove each other's containers.
**Why it matters**: The spec says the check is never skipped in silence, and `0004 AC-26` says it removes every container it started. It can remove one more.
**Suggested fix**: Fail unless the output holds a result line with a count above 0, keep the IDs that `docker run -d` prints and remove by ID. Owner: the developer.

### 🟡 g5. `NOTICE` gives the crate version as the version of AWS-LC, and two policies have no proof case, `NOTICE:28`, `scripts/prove-checks.sh:218`

**Problem**: "AWS-LC 0.47.0" is the version of `aws-lc-sys`. The library inside is 5.12.0. Nothing ties the versions in `NOTICE` to `Cargo.lock`. No proof case shows that a license exception stops fitting when its version moves, or that the two `deny` commands for the fuzz lockfiles still refuse a license.
**Why it matters**: A reader who matches an advisory needs 5.12.0, and a notice that is behind is a license fault that nothing reports.
**Suggested fix**: Write "AWS-LC 5.12.0, as bundled in aws-lc-sys 0.47.0" in `NOTICE` and in the spec, add a rule that compares the two crate versions with the lockfile, and add the proof cases. The Rust crates whose terms ask for a notice in a binary are due before the first binary (scope 011). Owner: the architect, then the developer.

Documents

### 🟡 d1. The memory budget "at any setting of the limits" leaves out connections that are not sessions yet, `docs/specs/0004-one-real-primary-path/index.md:982`

**Problem**: The budget is 64 MiB plus 1 MiB per session. A connection in startup holds up to `limits.startup_packet_max_bytes` for its packet and the buffers of TLS, and up to `limits.handshake_max_count` of them run at once. At the top of both ranges (10000 and 65536) clients that know no password can hold about 650 MiB plus TLS state with no session open. `0004 AC-21` measures sessions only.
**Why it matters**: An operator sizes the host from this sentence. At the defaults the amount is small, which is why I rate it minor and not major as the document reviewer did.
**Suggested fix**: Add a term for handshakes to the formula, or bound the product of the two keys in validation. Owner: the architect.

### 🟡 d2. A client that does not check the certificate of POOL loses what channel binding gives it on a direct connection, and that is not written, `index.md:1132`

**Problem**: The security model says that with a checked certificate on both legs there is no gap without channel binding. That is true as written. A `libpq` client with default settings does not check the certificate, and it gets channel binding from PostgreSQL directly. Through POOL it does not, because `SCRAM-SHA-256-PLUS` is never offered, so someone in the path can relay its login. A client with `channel_binding=require` cannot connect at all.
**Why it matters**: A quiet step down from a direct connection that no text warns an operator about. The sentence itself is not wrong, so minor.
**Suggested fix**: Say in the security model and in the operator text that clients need `sslmode=verify-full` until scope 006, and name the two cases. Owner: the architect.

### 🟡 d3. Places where the spec says more than the code does, `index.md:1134`, `:44`, `:702`, `:62`

**Problem**: The "Authority" line of the security model says a node receives SQL only while a fresh probe agrees. For a running session that is false by decision 10, and a fixture test proves that sessions outlive silence. `0004 AC-9` lists `pool show-authority` for `secret.permissions`, and that command reads no secret. The text on `Journal::create` says the main file alone carries the journal after bootstrap, and a checkpoint that fails is ignored (`crates/pool-control/src/journal.rs:344`). `0004 AC-24` says every line on stderr is one JSON object, and there is no panic hook, so a task that panics writes a line of plain text. Seven small behaviors have no spec text, among them the code for a runtime that cannot be built and a state directory that is a link.
**Suggested fix**: Correct the four sentences, and add the seven behaviors or drop them. For the panic line, install a hook that writes an event, or write the exception down. Owner: the architect.

### 🟡 d4. Status text that is behind, and criteria in the wrong house, `docs/scope/index.md:9`, `docs/specs/0003-control-and-session-data-model/index.md:14`

**Problem**: The scope index, the root `AGENTS.md`, `CLAUDE.md` and `apps/pool/AGENTS.md` still say that no proxy behavior exists and that `check-config` is the one command. These are beyond the stale texts the brief lists as known. `0003 AC-17` to `AC-19` stand under a line that says the criteria cover scope 003 only, and they were built and verified by scope 004 after feature 3 was done. The rationale of spec 0004 marks 29 decisions heavy, and the acceptance summary counted fewer.
**Suggested fix**: One dated sentence under the heading in spec 0003, the count in the rationale, and the status texts in the "Document it" box. Owner: the lead and the architect.

### 🟡 d5. The rules for the committed test key are misstated and held by no check, `index.md:1171`

**Problem**: The key itself is sound: only test code reads it, the certificate names `node.test` only, and the first line of the file says what it is. But the spec says only code under `#[cfg(test)]` reads it, and the integration tests of `pool-core` read it too, as the key of the listener. "The only private key in the repository" is checked by nothing, and no line in `docs/security/` points to decision 62.
**Suggested fix**: Correct the sentence, add a rule that refuses a private key block outside that one file or say that review keeps it, and add the pointer. Owner: the architect.

## Nits

- ⚪ n1. `crates/pool-core/src/session.rs:766`, the client is closed (up to one second) before the lease is dropped. Step 12 says the backend first.
- ⚪ n2. `session.rs:671` and `:672`, the pool key and the configuration generation are compared with values that cannot differ in this slice. Drive them from real state in scope 005 or remove them.
- ⚪ n3. `crates/pool-core/src/secret.rs:163`, `:199`, `crates/pool-core/src/scram.rs:79`, `:323`, the secret types derive `PartialEq`, a comparison that is not constant time. Nothing outside tests uses it.
- ⚪ n4. `crates/pool-protocol/src/message.rs:35`, `Header::body_len` subtracts 4 from a public field. Make the fields private.
- ⚪ n5. `crates/pool-control/src/lib.rs:20` and `crates/pool-control/src/observation.rs:174` still name `JournalRead::plan_bootstrap`. Decision 57 renamed it to `bootstrap_plan`.
- ⚪ n6. `crates/pool-control/src/journal.rs:589` to `:613`, `quick_check` runs twice on a real journal, and `open_ready` sets the journal mode before it checks anything.
- ⚪ n7. `apps/pool/src/control.rs:132`, the loop and the controller both keep a list of latches that are not recorded.
- ⚪ n8. `session.rs:920`, the conflict flag is read as a level. A conflict that came and went between two polls of a session would be missed. I could not build a real sequence for it.
- ⚪ n9. `crates/pool-config/src/serving.rs:604`, an expiry equal to the period plus the timeout leaves no margin, so the route can close for one round under load.
- ⚪ n10. `apps/pool/src/commands.rs:104`, dropping the runtime waits for a lookup of a host name that still runs, so `pool bootstrap` can outlive `probe.timeout_ms`.
- ⚪ n11. `crates/pool-config/src/topology.rs`, the closed list of characters leaves the invisible ones U+00AD, U+200B, U+2060 and U+FEFF in a name.
- ⚪ n12. `startup.rs:256`, with `limits.startup_packet_max_bytes` above 10000 a packet that POOL accepted can pass the limit of PostgreSQL once it is sent on, and the client then gets `08006`.
- ⚪ n13. Scripts: `scripts/integration/run.sh:32` sets the trap after `mktemp`, `:127` does not check `LISTEN_HOST` for an empty value, `scripts/check.sh:210` reads `POOL_FUZZ_SECONDS=0` as no limit, `:191` runs `cargo fuzz run` without `--locked`, and `scripts/integration/initdb/10-pool-fixture.sh:28` lets the superuser log in over the published port.
- ⚪ n14. Spec: `index.md:986` says one relay buffer per direction and the code has one (`session.rs:749`), `:989` says nothing is reserved from a length field and the readers reserve after the check, `:603` lists the files of step 4 in another order than `apps/pool/src/daemon.rs:110`, and `docs/research/technology-evaluation.md:152` has a stale sentence on unused dependencies.
- ⚪ n15. Tests: `crates/pool-core/tests/session.rs:1035` accepts a `Terminate` alone where the backend must be closed, `apps/pool/tests/integration.rs:728` matches start sequence 10 to 19 as 1, and the test folders under the temporary directory are never removed.

## Strengths

- The order of the session steps is the contract, and the code keeps it. Each comparison of the second look at the route has its own test, and removing the look fails six tests.
- Closing comes before the disk. A round latches in memory, publishes, and only then writes (`apps/pool/src/control.rs:127` to `:157`), so a journal that fails never leaves a route open.
- The reader of the route enforces the expiry with the origin the publisher used, so a control loop that stops closes logins by itself.
- First packets are read with exact reads and decided on the header, and the relay frames by length and never holds a whole message.
- SCRAM is pinned to the vectors of RFC 7677 on both sides, and the test reviewer computed them again outside the repository.
- The journal never creates a file, checks its marks and its schema, compares the revision inside the transaction, and returns no text of SQLite. The journal tests damage real files in more than a dozen ways.
- The lock file is checked on the open descriptor against its path, and a create race opens the file of the winner.
- Every manifest, feature set, pair of members and license exception matches the spec, and the lockfile has no second crypto provider in the build.
- Every correction after acceptance is dated and explained, and all 146 test names that `verify.md` cites exist.

## Test coverage

The hermetic suite holds the safety core. These 11 changes were each caught by a named test: the second look at the route removed, `in_hot_standby` not checked, the first report deciding in place of the last, a stale route counted as fresh, the identity lookup widened, the flag for a query in flight never set, `AuthenticationOk` from a node without SASL accepted, the signature of the node not checked, a changed revision accepted by `open_ready`, the mode of the state directory not checked, and the owner of a secret file not checked. One change, the conflict check removed from the top of the relay loop, made the session tests hang (t4).

These 11 changes passed every test of their package: `any` to `all` in the conflict flag (M3), `NegotiateProtocolVersion` not sent, the server signature not sent, `AuthenticationOk` moved before the route checks (t1), a derived `Debug` on `StartupRequest` (t2), the header of an oversized `Query` written to the node (t3), a bind before the journal is classified (t5), `restore_pending` read as no hold (t6), the lock dropped while serving, the authority ID not compared again, and SIGINT not handled (t7).

I did not run the 26 tests of the fixture. The verification record has them passing on `af23fa4`. A reader should take from the list above that the fixture, which is not part of `scripts/check.sh all` and has no CI job yet, is today the only proof for several sentences that a hermetic test could hold.

## Second review, the fix round, 2026-11-16

**Reviewed by**: the same split as the first review. `claude-fable-5-2` for the code, the scripts, the guards and the tests (author on `claude-opus-5-5`), with a second independent read of the production code by `claude-sonnet-5-5`, and `claude-opus-5-5` for spec 0004, its rationale and its verification record (author on `claude-fable-5-2`). The main reviewer is the session that wrote the first review. It had not seen the fixes before this round.
**Scope**: the range `b2aec30..1492e17` on `main` (123 commits), read with `git diff b2aec30..1492e17 -- <path>`. 71 files without `graphify-out/` and `docs/scope/coordination/`. This is Build plan step 15 of spec 0004, with decisions 84 to 105.
**Verdict**: Changes requested. The three major findings of the first review are closed: M1, M2 and M3. One new major finding holds the verdict: the integration script that the fix round changed does not work under the shell of Linux when no terminal is attached (r1). Beside it there are 4 new minor findings and 7 new nits. Nothing else holds the verdict.

### Summary of this round

The fix round did what the first review asked, and it did it well. A node that stops answering now ends its sessions within a time that POOL owns. A session asks the node who it is before the client is told that it is logged in. The line that decides whether a second primary ends sessions has two unit tests and one test on the fixture. Of the 12 wrong changes that went unseen or hung in the first round, 11 are now caught by a named test, and the twelfth was a change of mine that the new check has no reason to see.

The new defect is in the tooling and not in the product. `scripts/integration/run.sh` now uses job control of the shell to give the test command and its watchdog a process group each. `dash`, which is `/bin/sh` on Debian and Ubuntu, turns job control off when there is no terminal. On such a host the script waits 1800 seconds after the tests have ended, the watchdog can end nothing, and a signal leaves the test command behind. I ran it on the pinned Linux image. The check was verified on macOS only, where `/bin/sh` is another shell.

### How this round was run

- I read the fix in full for the production code and the scripts: `crates/pool-core/src/` (`session.rs`, `backend.rs`, `route.rs`, `secret.rs`, `records.rs`, `report.rs`, `load.rs`), `apps/pool/src/` (`control.rs`, `daemon.rs`, `probe.rs`, `log.rs`, `commands.rs`, `main.rs`), `crates/pool-control/src/` (`admission.rs`, `journal.rs` and two comments), `crates/pool-config/src/serving.rs`, `crates/pool-protocol/src/startup.rs`, `.cargo/config.toml`, `scripts/check.sh`, `scripts/prove-checks.sh`, the guard baseline, `NOTICE`, and the whole of `scripts/integration/run.sh`. Of the tests I read the names of everything that was added and the bodies of the ones I cite. I judged the tests mainly by changing the code under them.
- Two reviewers ran in parallel and did not see each other's results: `claude-sonnet-5-5` on the production code and the scripts, `claude-opus-5-5` on the documents. I checked their findings against the code. One finding of the code reviewer was wrong and is left out (a lost node does close its route at once, because a failed sample replaces the good one).
- `cargo test --workspace --locked` on the tree at `1492e17` (debug build, macOS, Rust 1.100.0): 537 tests and 18 documentation tests passed, 0 failed, 30 ignored (the tests of the fixture).
- `scripts/check.sh integration` on the same tree, on this workstation (macOS, Docker Desktop, the pinned image): exit 0 after 71 seconds, 30 of 30 tests passed, no container left.
- 30 wrong changes to the code, one at a time, each file put back with `git checkout`. 21 were caught by a named test, 3 of them on the fixture with `POOL_IT_FILTER`. 9 passed, and they are listed under r3 and t5.
- The probe for r1: the lines of the script that start the test command and the watchdog, with `sleep 3` as the test and `sleep 20` as the watchdog, under `/bin/bash`, under `/bin/dash` of macOS, and under `sh` in a container of the pinned PostgreSQL image, which was removed afterwards. No input, output or error stream was a terminal.
- I did not run `scripts/check.sh all`, `scripts/prove-checks.sh`, the Linux build or the fuzz targets. The checker ran them on `099a597` and I read that record. The tree was clean after every probe.

### The three major findings

| Finding | State | Evidence |
|---|---|---|
| M1, a peer that vanished | Closed | The control loop keeps, per node, the time of the first probe failure of the class `connect` or `timeout` since another outcome (`apps/pool/src/control.rs:132`), marks the expected writer lost when that time reaches `probe.node_loss_timeout_ms` (`control.rs:158`), and publishes the mark in the route view (`control.rs:51`). A session then ends as backend loss does: `08007` with a query in flight, `08006` without (`crates/pool-core/src/session.rs:1022`, `:1302`). Three wrong changes of mine fail the four `ac30_` tests of the control loop: the time ignored, the class `tls` counted as silence, a good sample that does not start the time again. On the fixture `it_ac30_a_paused_writer_is_marked_lost_and_its_sessions_end` passed in my run. Keepalive is on for a socket to a node (`crates/pool-core/src/backend.rs:490`, with a test) and for the listener (`apps/pool/src/daemon.rs:383`, with no test, r3). A client that vanished is noticed by the kernel, in about two hours with its defaults. The spec says so, names the kernel values as part of deploying the slice, and gives keys of POOL's own to scope 005 (Follow-up 13). That is the third shape the first review offered, so I count this half closed as decided. |
| M2, the server a session reaches | Closed | `verify_identity` sends the question of the probe on the connection of the session and takes exactly one row with the system identifier and the timeline of the route, `false` for recovery, and the status `I` (`backend.rs:362`). The route carries both values from the approved topology of the journal (`crates/pool-control/src/admission.rs:373`). The call runs inside the acquire deadline, after the backend is ready and before the second look at the route and the greeting (`session.rs:709` to `:723`). A server built without the question refuses every login. Four wrong changes of mine fail the `ac29_` tests: the system identifier not compared, a timeline that is not digits only, a missing question that lets the session through, and the log class of the refusal. `it_ac29_a_login_to_an_address_that_moved_to_another_server_gets_08006` passed in my run. A promoted server of the same cluster has another timeline and is refused. |
| M3, the conflict flag | Closed | `any` changed to `all` at `control.rs:71` now fails `ac14_a_second_primary_beside_a_silent_writer_is_a_conflict_although_silence_is_not` and `ac14_a_latched_journal_with_an_empty_board_is_a_conflict`, and on the fixture it fails `it_ac14_a_second_primary_beside_a_silent_writer_ends_sessions_and_latches_the_journal`. |

### The minor findings and nits of the first review

"Closed" means the fix is in the range and I saw it hold. "Deferred, recorded" means spec 0004 names a decision, a scope and a Follow-up item for it. I judged whether the record is right, not whether I would defer it. None of the deferred points is major in my view.

| Finding | State | Evidence |
|---|---|---|
| c1 | Closed | The status byte is read inside `guarded` (`session.rs:1188`), the greeting write too, and shutdown drops the tasks that are left 2 seconds after its deadline (`session.rs:439`). My probe of the first round is now six `c1_` tests, three places to stop times two cuts. Two parts have no test (r3). |
| c2 | Closed in the code | One `select!` watches the control loop, the probes, the listener and the log writer (`daemon.rs:306` to `:320`). A task that ends gives `daemon.failed`, `shutdown_now` and exit 3. A handler that cannot be installed is `runtime.failed`. No test holds it (r3). The checker showed it with a fault in a private copy. |
| c3 | Closed | A set of the names seen (`crates/pool-protocol/src/startup.rs:238`). Accepting a repeated name fails `parameter_rows`. |
| c4 | Closed | `.cargo/config.toml` forces both switches to 0, and a unit test expects SQLite 3.54.1. The checker ran the build both ways. No test holds the six lines, which the spec says and gives to Follow-up 14. |
| c5 | Deferred, recorded | Decision 95, text corrected, Follow-up 3 (scope 005) and 6 (scope 008). |
| c6 | Closed | `certificate_allowed` (`crates/pool-core/src/secret.rs:141`), with a test, and with the modes and the owner run on Linux by the checker. `0004 AC-9` does not name the new refusal yet (r4). |
| c7 | Deferred, recorded | Decision 89, text, Follow-up 4 (scope 006). |
| c8 | Deferred, recorded | Decision 90, text, Follow-up 6 (scope 008). |
| c9 | Closed in the code | `last_try` runs when the loop stops (`control.rs:203`). The test calls `last_try` itself (`control.rs:743`), so the call inside `run` is held by nothing: I removed it and every test passed (r3). |
| t1 | Closed | The three changes of the first round now fail `ac15_a_request_for_3_2_...`, `ac16_a_login_for_3_0_...` and `ac14_no_authentication_ok_comes_before_a_refusal_after_the_proof`. |
| t2 | Closed, with a remainder | A derived `Debug` on `StartupRequest` fails two tests, and `VerifiedPrincipal` and `Credential` have an exact text. `Lease`, `ServerParts` and `Server` still have none (r5). The log check runs in the stop helper. |
| t3 | Closed | The header of an oversized `Query` written to the node fails `ac20_no_byte_of_an_oversized_query_reaches_the_node`. |
| t4 | Closed | The change that hung the tests for ten minutes now fails eight tests in seconds. The stop helpers of the fixture have a limit. The watchdog of the script is where r1 is. |
| t5 | Closed | The test holds a listener on the port while `pool run` runs, so a run that reaches the bind exits 3. A bind that I added with its error ignored still passes, which is a change no real edit would make. |
| t6 | Closed | The hold read as none fails `ac10_a_journal_on_hold_for_a_restore_classifies_as_restore_pending`. The test that needed the `sqlite3` tool is gone. |
| t7 | Closed | The authority ID not compared fails two tests of the journal. On the fixture, the lock dropped while serving and SIGINT not handled each fail `it_ac9_ac25_a_serving_daemon_holds_its_state_directory_and_stops_on_sigint`. |
| t8 | Open, in work | The tester is on it. The triage says it is no gate, and I agree. |
| t9 | Closed for the four cases taken | The output of the INSERT, the client that vanishes in a transaction, the verbose SQLSTATE and the second primary are in the 30 tests that passed in my run. The standby cases are Follow-up 5 (scope 007). |
| t10 | Closed, with a remainder | The target `auth`, a nonce of its own for `scram`, and literal bytes in `golden.rs`. `data_row` has a unit test (decision 102). The second sentence of `0004 AC-28` still has no test and no decision (r5). |
| g1, g3 | Deferred, recorded | Decision 97, Follow-up 14, and the spec now calls rule W3 a tripwire. |
| g2 | Closed | `rusqlite` and `aws_lc_rs` are bare lines with their allows, and proof case 36 exists. I did not run the proof script. The checker tried the case by hand. |
| g4 | Closed for what it asked, with a new defect | No test gives exit 1, and containers are removed by ID. The watchdog that came with it is r1. |
| g5 | Closed, with a remainder | `NOTICE` and the spec say 5.12.0. `docs/research/technology-evaluation.md:159` still says 0.47.0 (r5). The rule that compares versions is Follow-up 14. |
| d1 | Closed | A term for connections that are not sessions yet (decision 91). The headline still says "at any setting of the limits" (r4). |
| d2 | Closed | Decision 92, in the security model. |
| d3 | Closed, with a remainder | The four sentences, the panic hook and two of the behaviors. Five behaviors are left "for the reviewer to name" (r5). |
| d4 | Closed for spec 0003, the rest recorded | Follow-up 15 for the status texts. |
| d5 | Closed for the text, the rest recorded | The rule is Follow-up 14. The pointer from the security documents is in no Follow-up item (r5). |
| n1, n5 | Closed | Closing the client first again fails `step_12_closes_the_backend_before_the_client_that_does_not_read`. Both comments name `bootstrap_plan`. |
| n9, n12, n14 | Closed, text | Under Keys and in three sentences of the spec. |
| n13 | Closed for three of four | The trap before `mktemp`, an empty listen host, and `POOL_FUZZ_SECONDS=0` are refused. `cargo fuzz run` has no `--locked`, so the check asks `cargo metadata --locked` first (decision 100). The superuser on the published port is scope 007. |
| n15 | Open | `crates/pool-core/tests/session.rs:1206` still takes a `Terminate` alone, and `apps/pool/tests/integration.rs:800` still matches 10 to 19. The triage says no gate. No Follow-up item names it (r5). |
| n2, n3, n4, n6, n7, n8, n10, n11 | Deferred, recorded | Follow-up 3, 4, 6 and 7. |

### New findings

#### 🟠 r1. Under `dash` with no terminal the integration script waits 1800 seconds and its watchdog and its cleanup end nothing, `scripts/integration/run.sh:170` to `:194`

**Problem**: The script starts with `#!/bin/sh` and `scripts/check.sh:165` runs it that way. It turns on job control with `set -m` so that the test command and the watchdog each get a process group, and it ends them with `kill -TERM -- "-$TEST_PID"` and `kill -TERM -- "-$WATCHDOG"`. `dash` is `/bin/sh` on Debian and Ubuntu, and it turns job control off when it finds no terminal. I ran the same lines three ways with no terminal. Under `bash` each job is the leader of its own group and the script is done after 3 seconds. Under `dash` of macOS, and under `sh` in a container of the pinned PostgreSQL image (`/usr/bin/dash`), the shell prints `set: can't access tty; job control turned off`, both jobs keep the group of the script, the kill of the group fails, and the script is done only after the 20 seconds of the watchdog. Every kill in the script ends in `2>/dev/null || :`, so nothing says that it failed. In the real script that means three things on such a host. After the tests end, `wait "$WATCHDOG"` at line 194 blocks until `sleep 1800` is over. A test that hangs is never ended. And a signal to the script removes the containers and leaves `cargo test` and the daemons it started.
**Why it matters**: A host with `dash` and no terminal is a CI runner, and it is the pinned Linux image. The CI job for this check is manual step `F4-3`, and its first run would sit for half an hour after a green result and then pass the time limit of the job. The watchdog and the cleanup are what finding g4 and decision 103 asked for, and on Linux the script gives neither. The checker ran the check on macOS only, where `/bin/sh` is `bash`. The code reviewer found the same thing on its own. No product code is wrong, which is why this is not a blocker. It is major because the one check that proves the slice against PostgreSQL does not work where it has to run.
**Suggested fix**: Do not rest on job control. End the test command and the watchdog by their own process IDs and let each end its children, or run the test command under `setsid`, or let the script require a shell that it names. At the least, fail at once when `set -m` did not take (`case $- in *m*) ;; *) fail ...`). Then run the check once in a shell with no terminal on Linux and record it. Owner: the developer, then the checker.

#### 🟡 r2. One probe that stumbles can end every session when the loss time is set low, and a slow primary counts as a lost one, `crates/pool-config/src/serving.rs:620`, `apps/pool/src/control.rs:158`

**Problem**: `probe.node_loss_timeout_ms` may be 1000, and it has no cross rule with the other probe keys, which the spec says on purpose. The control loop runs a round at every tick and at every sample of any node, and it marks the writer as soon as the first failure is older than the limit. With a limit below the period plus the timeout, one failed probe is enough: the mark comes at the next round, before the next probe of the writer can answer. Apart from that, the class `timeout` counts as no answer at all. A primary that is reachable and answers the probe slower than `probe.timeout_ms` for the whole limit (a disk that stalls, a processor that is full) loses all its sessions, most of them with `08007`. A lookup of a host name that fails counts as `connect` in the same way.
**Why it matters**: Decision 10 says a probe that stumbles must not cut work that still flows. With the defaults (30000 against 2000 and 1000) that holds. Nothing keeps a file from breaking it, and the consequence for a slow primary is not written.
**Suggested fix**: A cross rule, for example a loss time of at least two periods plus the timeout, or a count of failed rounds beside the time. Write the slow primary into Consequences. Owner: the architect.

#### 🟡 r3. Nine wrong changes to the new code pass every test, and five of them have no owner, `crates/pool-core/src/session.rs:445`, `:710`, `apps/pool/src/control.rs:64`, `:203`, `apps/pool/src/daemon.rs:315`, `:333`, `:383`, `crates/pool-core/src/backend.rs:383`

**Problem**: Four are mine. The tasks left after the last limit of shutdown are never dropped (`session.rs:445`). The greeting write is not raced against a cut. The question of step 8 runs with no deadline (`session.rs:710`). The route carries timeline 1 in place of the approved one (`control.rs:64`): every test uses timeline 1. Five are the ones the checker found: the status `I` after the answer of step 8 (`backend.rs:383`) and the last try for a latch (`control.rs:203`), which I ran again and which still pass, and a task that ends is ended with `shutdown` (`daemon.rs:333`), a probe task that ends is not noticed (`daemon.rs:315`), and the listener has no keepalive (`daemon.rs:383`), which I did not run. The record of the checker names the five under "For the tester". No Follow-up item, no decision and no scope owns them, and the claim that the `c9_` test fails without the last try is not true for the call at `control.rs:203`.
**Why it matters**: Each fails closed or was shown to work by a probe, so none is a fault of the product. The first half of the first sentence of `0004 AC-30` (keepalive on every accepted socket) and the whole of c2 are held by no test.
**Suggested fix**: A test for each, or one Follow-up item that names all nine with an owner. For the route, one test with an approved timeline above 1. For the last try, stop the loop through `run` and look for the line. Owner: the tester, and the architect for the record.

#### 🟡 r4. New text that says more than the code, and old text that survived beside the new, `docs/specs/0004-one-real-primary-path/index.md:1094`, `:49`, `:983`, `:1082`, `:44`

**Problem**: The table of who is gone says a client that waits for an answer is noticed by keepalive (`index.md:1094`). While a query is in flight the relay does not read from the client, so such a client is noticed only when the statement gives output or ends. `0004 AC-14` (`index.md:49`) and the row of decision 10 in the rationale still say silence never ends a session, with no pointer to `0004 AC-30`. Step 8 says anything else ends the attempt with the class `identity` (`index.md:983`), and a question that gets no answer in time is logged as `timeout`. "30 seconds plus at most 3" (`index.md:1082`) leaves out the first failed round. The table of SQLSTATEs has no words for a lost node or for the refusal of step 8. `0004 AC-9` (`index.md:44`) does not name the refusal of a CA file or a chain that others can write. The headline of the memory budget still says "at any setting of the limits". Follow-up 14 says two lines of `.cargo/config.toml` where Native code says six. `docs/specs/0004-one-real-primary-path/rationale.md:40` still says fourteen build steps, and `docs/specs/0004-one-real-primary-path/verify.md:1424` still names the hold test in `integration.rs`.
**Suggested fix**: Correct each sentence. For the client that waits, say that a statement in flight hides it until scope 006 (Follow-up 4 already owns it). Owner: the architect.

#### 🟡 r5. Remainders that no Follow-up item holds, and two decisions that should be marked heavy, `docs/specs/0004-one-real-primary-path/rationale.md:276` to `:323`

**Problem**: The triage is complete for what it defers. Six points that it calls done have a part left with no owner: the `Debug` tests of `Lease`, `ServerParts` and `Server` (t2), the second sentence of `0004 AC-28` (t10), the version in `docs/research/technology-evaluation.md:159` (g5), the five behaviors without spec text (d3), the pointer from the security documents to decision 62 (d5), and n15. The note of the checker that a daemon whose stderr is closed serves on with no log is in no item either. Decision 96 changes the shutdown contract of `0004 AC-25` (tasks dropped without a message 2 seconds after the deadline, a new line at exit) and decision 103 changes how a required check ends, and neither is marked heavy. The dated notes on six rows of the triage sit in a cell beyond the header, so a rendered page drops them.
**Suggested fix**: One Follow-up item for the remainders, the mark on the two decisions, and the notes moved into the last column. Owner: the architect.

Nits of this round:

- ⚪ `apps/pool/src/log.rs:82`, the panic hook writes `daemon.panicked` with no field. The file and the line of a panic are constants of the source and hold no value, and without them a panic in the field cannot be found.
- ⚪ `apps/pool/src/control.rs:212`, the last write of a latch at shutdown has no limit, so a disk that stalls holds the exit. The same holds for a write inside a round.
- ⚪ `scripts/integration/run.sh:94` to `:103`, a container that was created and did not start is left under its name, because only an ID that `docker run -d` printed is removed.
- ⚪ `apps/pool/src/daemon.rs:382`, the listener now sets address reuse and a backlog of 1024, and no spec text says so.
- ⚪ `crates/pool-core/src/backend.rs:362`, the question of step 8 runs as the backend login with names that are not qualified by `pg_catalog`, and a parameter status that arrives during it is dropped. Taking the right to run `pg_control_system()` from `PUBLIC` refuses every login, which fails closed and which the checker ran. Say it in the operator text.
- ⚪ `scripts/integration/run.sh:167`, a filter gives a green run with one test. The script says so on stderr, which is enough for a tool of the developer, and a CI job must not set it.
- ⚪ `.cargo/config.toml` is read from the directory `cargo` runs in and its parents. A `cargo` call from outside the repository with `--manifest-path` does not read it.

### What holds the verdict

One finding: r1, major. When the integration script works under `/bin/sh` of Linux with no terminal, and one run there is on record, I have no open blocker and no open major finding, and the verdict becomes "Approved". The minor findings r2 to r5, t8 and n15 do not hold it.

### Addendum, the documents up to `bbcf7c1`

After the second review was written, `main` gained documents only: the range `1492e17..bbcf7c1` changes `index.md`, `rationale.md` and `verify.md` of spec 0004 and no code. It adds decisions 106 to 108 and a checker note for decisions 104 and 105. I read the diff, and `claude-opus-5-5` read it apart from me and checked each decision against the code. Neither of us found a blocker or a major point in it. The verdict does not change: r1 is in a script that this range does not touch.

| Decision | What it says | Judgement |
|---|---|---|
| 106 | A daemon whose stderr has no reader serves on with no log. Accepted for this slice, written under the log events and under Consequences, and scope 008 counts the failed writes (Follow-up 6) | Right, and it matches the code: the log task ignores a failed write (`apps/pool/src/log.rs:90`) and ends only on a panic. This closes the part of r5 that said the note of the checker had no owner. One line is missing (a3). |
| 107 | A primary that the journal did not approve sets the conflict latch, also while the cluster is closed with `authority.config_conflict`. Declared as intended (`docs/specs/0004-one-real-primary-path/index.md:847`) | Right. The rule for a latch never reads `expected_writer` (`crates/pool-control/src/admission.rs:351`), a configuration is not an authority, and a latch can only close. The words around it leave two things out (a2). |
| 108 | The proof that an accepted socket has keepalive on is the pending timer in `/proc/net/tcp`, and the limit of that proof is written (`index.md:1100`) | Right, and the limit is honest: the timer shows that keepalive is on, not that the kernel ends a dead connection. The row that names the evidence is still wrong for a reader of the rendered page (a1). |

New points from this range, all minor or less:

- 🟡 a1. `index.md:1375`: the second cell of the row for `0004 AC-30` still says `ss -tno`. The correction sits in a fourth cell of a table with three columns, so a rendered page drops it. It is the fault of the triage notes in r5 once more. Rewrite the second cell and drop the fourth. Owner: the architect.
- 🟡 a2. `index.md:847`: "the one way to make that node the writer is a new bootstrap" leaves out that `pool bootstrap` refuses while a journal or a witness is present, so the operator first stops the daemon and takes both away, as Consequences says at `index.md:1515`. And "it costs nothing in this slice" is true only against a correct configuration: a latch that was set stays when `expected_writer` is put right again, and the cluster stays closed until scope 009. Say both. Owner: the architect.
- 🟡 a3. `index.md:1251`: a daemon with no log is not in the list of what the slice does not defend, and `docs/security/` does not name it. A latch shows only in the log. With stderr closed, a latched cluster answers `57P03`, `pool show-authority` says `ready`, and nothing tells the operator why. One line in the security model, with a pointer to the paragraph on stderr and to Follow-up 6. Owner: the architect.
- ⚪ A reader of stderr that stalls, and does not go away, is another case than the one decision 106 covers. The write may then hold one of the four blocking threads of the runtime for good. Neither of us ran it, so take it as a question for scope 008.
- ⚪ The section on the integration check now says the watchdog has a process group of its own and that nothing of a run outlives the script (`index.md:1300`). Under `dash` with no terminal that is not true, which is r1. So r1 is now also a place where the spec says more than the script does.

The rationale counts 108 decisions, 40 of them heavy. That fits the table.
