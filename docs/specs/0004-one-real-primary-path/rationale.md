# One real primary path rationale

This is the decision record of [spec 0004](index.md). The build reads `index.md`. This file says why.

## Context

Scopes 001 to 003 built the ground: a workspace with checks, a configuration format, and the authority rules as plain functions that nothing calls. No byte has gone from a client to PostgreSQL through POOL. Scope 004 is the first slice that has to be right about everything at once: a wire protocol, two TLS legs, two separate authentications, an identity map, durable authority, live evidence, limits and teardown. A mistake in any one of them is either a wrong write, a leaked secret or a proxy that falls over.

Several forces pull on it.

The safety contracts of spec 0001 are fixed. A backend belongs to one frontend for its whole session. Startup strings cannot choose a backend login. Nothing is sent again after bytes may have reached a node. Writes go only to a node that the journal, the configuration and a fresh observation agree on. Scope 004 has to make each of these true on a real connection, not in a test of a function.

The review of scope 003 found three ways in which the authority functions could still open writes that the spec says stay closed, all of them in what a caller passes in. No caller existed then. This slice writes the first caller, so it has to close them first.

The project keeps its dependency tree small on purpose, checks every license by tool, forbids two versions of one package, and has had no native code so far. A real proxy cannot stay there: it needs a runtime, TLS and a durable store. Each of these has to come in through the gate the earlier specs built, and a person has to decide the licenses.

The source guard that says "no socket, no thread, no runtime anywhere" has to change the day a socket appears. If it simply goes away, the next slice can put a socket into the control crate and nothing notices.

The team is two engineers, the lab is one workstation with Docker, and the risk register already names this slice as the one most likely to run over its estimate (`R-24`). So the path has to be narrow, and every piece that can wait has to wait in writing.

If nothing is decided, a developer has to invent the startup rules, the SQLSTATEs, the limits, the file formats and the order of checks while writing the code, and the review then has nothing to check them against.

## Options considered

### Option 1: The whole thin path, wire code in two new crates, a topology snapshot between control and data, no client driver (chosen)

`pool-protocol` holds bytes, `pool-core` holds sessions, `pool-control` gains the journal and builds snapshots, and the binary wires them. The probe uses the same backend connection code as a session.

**Pros**:

- One implementation of the PostgreSQL startup, TLS upgrade and SCRAM, used by both the data path and the probe, tested once.
- The crate boundaries of spec 0001 become real, and each crate gets a list of what it may name.
- The smallest set of new packages that still gives a runtime, TLS and a store.
- The data plane reads a view and never calls into authority code, which is the split spec 0001 asks for.

**Cons**:

- More first party protocol code than a driver would need: a SCRAM client, a result row reader for the probe, base64.
- Two crates are created in one slice, with guard and baseline changes for each.
- The slice is large. It has fourteen build steps.

### Option 2: A client driver for the probe and other control SQL

Adopt `tokio-postgres` (and a TLS adapter for it) in `pool-control`. Keep first party wire code for the data path only.

**Pros**:

- The probe is a few lines. Rows, types and errors are handled by a library that many projects use.
- Later control SQL (lag queries, promotion checks) gets a real client.

**Cons**:

- About 25 more packages, a second SCRAM implementation and a second TLS path to review, for one query.
- `pool-control` would then name the runtime and sockets, so the guard that keeps them out of control would be gone.
- The data path still needs its own SCRAM client, because a driver cannot sit between a frontend and a backend.

### Option 3: Modules in the binary first, crates later

Put the wire code and the session code into `apps/pool` as modules and extract crates when a second user appears.

**Pros**:

- No new members, no new baseline lines, less ceremony now.
- Boundaries can be drawn after the code has shown where they are.

**Cons**:

- The fuzz target has to live in the `fuzz/` directory of the crate it tests, and a binary crate cannot be a dependency of a fuzz package.
- Everything in the binary may name the runtime, so the source guard can no longer say that framing code is pure.
- Spec 0001 already decided the four crates. Putting core code into the binary creates the dependency from library code on binary code that it forbids, as soon as anything is extracted.

### Option 4: Two slices: authority on disk first, the session later

Build the journal, bootstrap and probe as scope 004, and move the session path to a new scope.

**Pros**:

- Each half is small enough to review in one sitting.
- The first native dependency and the first runtime arrive in different pull requests.

**Cons**:

- Scope 004 would end without a client ever connecting, so none of `PRD-001 AC-1`, `AC-3` or `AC-4` would have evidence, and the walking slice milestone would not be met.
- The scope index and the plan would need a new feature and a new number, which is a decision for you and not for this run.
- The same order is available inside one scope: the build plan of option 1 puts authority on disk first and the session second, with a runnable binary after each.

## Rationale

Option 1 is the only one that meets the scope text, the safety contracts and the guards at the same time. The scope asks for one real client path with the writer check in front of it, so the journal, the probe and the session all have to exist. Option 4 gives up the client path. Option 3 gives up the fuzz rule of spec 0002 and the crate boundaries of spec 0001. Option 2 buys a short probe with a second copy of everything this slice is most likely to get wrong, and it puts sockets into the crate that should have none.

What makes option 1 affordable is that the pieces are small when the path is narrow. The protocol subset is two client messages and nine backend messages. SCRAM without channel binding is four short text messages and three hash calls. The probe is one query with one row. The price of doing them first party is paid once, and it is paid in a crate with no dependency, where fuzzing is cheap.

The size of the slice is the real cost, and the build plan is the answer to it: authority on disk comes first and ends in a binary that can bootstrap against a real PostgreSQL (step 8), the session comes second (step 10), and breadth of limits and failure cases comes last (step 11). If the estimate runs over, the cut is after step 10, where the path already works end to end.

## Decisions taken

The session ran in the `auto` decision mode: each choice below is the recommended option, taken without asking, with the runner up and the way back. "Heavy" marks a public contract, a format, a part of the security model, or a change to an accepted spec.

| # | Question | Taken | Runner up | Heavy | What changes if the runner up is chosen |
|---|---|---|---|---|---|
| 1 | Is the number of findings from one file limited? (Follow-up 11 of spec 0003, written there as your decision) | Yes: the first 20000 are kept, the rest are counted, and the report says how many were left out. Plus the output buffer | The buffer only, no limit | yes | `0004 AC-7` loses its first three sentences, `Rejection::omitted`, the omitted line and field go away, and the daemon can hold two million findings at a start |
| 2 | Are control characters above U+007F and direction characters allowed in a name or a path? (Follow-up 12 of spec 0003) | No. A closed list is refused with `config.invalid_value` | Keep the list of spec 0003 and escape names in every later output | yes | `0004 AC-6` and two fixtures go away. Scope 009 has to escape every name it shows |
| 3 | What does the frontend authenticate with? | `SCRAM-SHA-256` against a verifier file per principal | A client certificate first | yes | The `[[principal]]` table becomes a certificate mapping, SCRAM as a server is not built, and `psql` users need a certificate each |
| 4 | Is channel binding offered or used? | No, on neither leg, in this slice | Build `SCRAM-SHA-256-PLUS` now | yes | One more step, a parser for the signature algorithm of a certificate, and more cases in `0004 AC-16` |
| 5 | Where does a verifier live in the configuration? | A `[[principal]]` table with `verifier_file` | A `verifier_file` key in every identity entry | yes | No new table. A principal with ten databases names its verifier ten times, and two entries can disagree |
| 6 | Which messages does the session accept after login? | `Query` and `Terminate`. Everything else ends the session with `0A000` | Pass every message through unchanged | yes | The extended protocol and COPY would work by accident and without a state model. `0004 AC-23` could not say what happens on an error inside a pipeline |
| 7 | What happens when the journal state is not ready at a start? | `pool run` refuses to start | Start, and refuse every login with `57P03` | yes | `0004 AC-13` turns around, and the session tests gain six closed states |
| 8 | Where do bootstrap and reading the authority ID live? | Two commands of `pool`: `bootstrap` and `show-authority` | Create `apps/pool-cli` now | yes | A sixth member, a W2 change, and a second binary before an administration interface exists |
| 9 | Which SQLSTATEs does POOL generate? | The table of the spec: `FATAL` only, constant text, one code per situation | Reuse the texts of PostgreSQL, or `XX000` for everything POOL adds | yes | Clients cannot tell a full proxy from a closed writer, and messages could carry names |
| 10 | Is a running session ended when admission closes later? | On a positive conflict, yes. On silence, no | Never, until scope 008 decides | yes | The second half of `0004 AC-14` goes away. Sessions keep writing to a node that a probe has just shown to be wrong |
| 11 | Where does the probe get the timeline? | Plain SQL: the WAL file name on a primary, the greatest known value on a node in recovery | A replication connection with `IDENTIFY_SYSTEM` | yes | The monitor login needs the `REPLICATION` attribute and a replication line in `pg_hba.conf`. The timeline of a replica becomes exact |
| 12 | How does a `Ready` state learn of a latch set after the start? (point m1) | Two functions in `pool-control`: one sets the latch in memory, one takes the revision from a receipt of the journal | Read and classify the journal again after every write | yes | `0003 AC-17` describes a reread. The failed write still needs the memory of the daemon |
| 13 | Who may build a journal value? (point m2) | Only `pool-control`. Other crates get an opaque `JournalRead`, and a source guard refuses four names outside the crate | Private fields and one checked constructor | yes | Every walk test of spec 0003 is rewritten to the constructor, and any crate can still call it |
| 14 | Does the sample rule get stricter? (point m3) | No. The caller is fenced instead: a board that holds one sample per node, a clamp of now inside `build_snapshot`, and a guard that lets only `pool-control` call `writer_admission` | A new reason, `evidence.sample_unusable`, for a sample of another node that cannot be used | yes | A new reason name in spec 0003, and two assertions of the accepted tests of `0003 AC-11` turn from open to closed |
| 15 | How does rule W3 work once sockets exist? | One file with bare, `deny` and `allow` lines | One baseline file per crate | yes | Five files and a rule for a crate with no file |
| 16 | Which crypto library? | `aws-lc-rs`, which `rustls` uses by default and the security requirements name | `ring` | yes | One exception for the duplicate package `windows-sys`, fewer packages, the same two licenses |
| 17 | Which crates are created? | `pool-protocol` and `pool-core`, both now | Modules in the binary first | no | No new member. The fuzz package has no crate to live in |
| 18 | How does the probe talk to PostgreSQL? | The backend connection of `pool-core` | `tokio-postgres` | no | About 25 more packages, and sockets in `pool-control` |
| 19 | How is SQLite linked? | `rusqlite` with `bundled` | The SQLite of the host | no | The format then depends on the host package, and the Linux image needs a development package |
| 20 | What does POOL do for a user name that is no principal? | The full exchange with a made up verifier | Refuse at once with `28P01` | no | A client can list the principals by trying names |
| 21 | What does POOL do with a startup parameter it does not forward? | Refuse with `0A000` | Ignore it | no | A client believes a setting took effect that never reached PostgreSQL |
| 22 | What does POOL send as `BackendKeyData`? | Nothing | A random key that matches nothing | no | Clients that insist on the message work. The key looks like a promise that cancel exists |
| 23 | What does POOL answer to a request for protocol 3.2? | `NegotiateProtocolVersion` for 3.0 | Refuse | no | A client that would have accepted 3.0 cannot connect |
| 24 | How do sessions get the writer? | They read the route view built from the snapshot (point m6) | Call `writer_admission` per login with an index of the samples | no | The data plane calls control code, and the cost per login grows with the file |
| 25 | What range does the expiry have? (point n2) | 1000 to 60000 ms, at least period plus timeout, read from the configuration by `build_snapshot` | A checked expiry type in `pool-control` | no | The signatures of `writer_admission` and `plan_bootstrap` change, and so does every test of spec 0003 |
| 26 | Does the probe keep its connection? | Yes, one per node, opened again after a failure | A new connection for every sample | no | A login and a TLS handshake every two seconds per node |
| 27 | How does one writer run without a thread? | `&mut self`, the blocking pool of the runtime, and the directory lock | A thread of its own with a channel | no | Rule W3 needs an exception for `std::thread` |
| 28 | How is the state directory locked? | `File::try_lock` on `pool.lock` | The exclusive locking mode of SQLite | no | The lock is taken only when the journal is open, and a reader blocks the daemon |
| 29 | Who is the service account? | The owner of the state directory | The effective user, read through a new dependency | no | One more direct dependency and four more packages. Running as root could be refused |
| 30 | What does a client get when its backend is lost? | `08007` with a query in flight, `08006` without, nothing in the middle of a message | Always close without a message | no | A client cannot tell an unknown outcome from an idle disconnect |
| 31 | Where are the session permits taken? | After authentication. A separate cap bounds handshakes | One connection limit at accept | no | Unauthenticated clients can use up the session limit |
| 32 | Is the relay buffer a key? | No. A constant of 65536 bytes | A key in `[limits]` | no | One more key, and the memory budget becomes a formula |
| 33 | Which methods does POOL accept from a node? | `SCRAM-SHA-256` only | Also a clear text password over TLS, or no proof at all | no | A node with a weaker `pg_hba.conf` works |
| 34 | Which characters may a backend password hold? | Printable ASCII | Any text, with SASLprep | no | A dependency for the Unicode tables, or first party tables |
| 35 | How do the new licenses enter `deny.toml`? | Five exceptions by crate | Two more lines in the general list | no | The fuzz lockfiles fail the check for an unused license |
| 36 | Who writes the log lines? | A first party `tracing::Subscriber` | `tracing-subscriber` with a custom format | no | Seven more packages, and the field order has to be forced |
| 37 | How are refusals logged? | Counted, one line per reason and second | One event per refusal | no | A flood of bad connections fills the log |
| 38 | Where does `psql` run in the integration check? | In the PostgreSQL container, reaching POOL on the host | On the host | no | The check needs a `psql` on every machine, of an unknown version |
| 39 | How is case 17 repaired? | The script makes a feature of its own in its copy | Point the case at the scope 005 file | no | The case fails again when scope 005 starts |
| 40 | When are the dependencies added? | In one step, before the code that uses them | Each step adds its own | no | No warning for a while. Two developers change `Cargo.lock` and the baseline at the same time |
| 41 | How does a command say that a valid file is not enough? | Two views and the code `config.incomplete` | Make the keys required in the schema | no | Fixtures of spec 0003 become invalid, against `0003 AC-1` |
| 42 | Which runtime flavor? | Multi thread, with four blocking threads at most | Current thread | no | One core does all the work |
| 43 | Is the salted password derived at every backend login? | No. The two keys are kept per credential, salt and count | Derive every time | no | A burst of logins fills the four blocking threads, the probe times out, and the cluster closes by itself |
| 44 | Does a log line wait for stderr? | No. A bounded queue and one writing task. A full queue drops and counts | Write in place | no | A slow reader of stderr can stop every worker of the runtime |
| 45 | Which code does a client get when POOL ends its session? | `08007` with a query in flight, `57P01` without | Always `57P01` | no | A client whose COMMIT may have been applied is told only that it was ended |
| 46 | `invalid-unknown-key.toml` of spec 0001 uses `[listener]` as its unknown table, and this slice makes that table known. What gives? (added on 2026-11-15, after acceptance, from a finding of the developer of step 5) | The fixture and two in memory tests name the table `no_table`. Finding, line, column and output stay the same bytes | Keep the fixture text and let its result change to three `config.missing_key` findings, with an exception in `0004 AC-5` | yes | The row of that fixture in spec 0001 and the byte for byte tests of `0001 AC-7` change, and spec 0001 loses its only fixture for an unknown table |

### Notes on the heavy ones

**Decision 1, the limit on findings.** Spec 0003 measured two million findings and 500 MiB from one file of 1 MiB, and said that a buffer alone does not bound it. This slice is the first in which the daemon validates at a start, which is the moment spec 0003 named. Of the two limits it sketched, a fixed count bounds every shape of input, and one finding per empty entry bounds only the shape that was measured. The number has to be above 17268, because `0003 AC-15` promises that many finding lines for made input 3 and that criterion is accepted. 20000 is the next round number. The cut shows itself: a report that was shortened says so and says by how much, and a report that was not is the same bytes as before. The file is rejected as a whole either way, so `SEC-12` is untouched. What changes is that "every finding in one run" now has a ceiling, and that is why this is a decision and not a repair.

**Decision 2, names.** Nothing prints a name today, and this slice does not either. It does send the backend login and the database to PostgreSQL in a startup packet. Bytes in that packet cannot inject anything, so the risk is not there. The risk is in every later output: an escape sequence in a terminal, or two entries that look the same on a screen. There are two ways to deal with that, and one of them has to be remembered by every slice that ever prints a name. Refusing the characters once, in the one place that reads the file, cannot be forgotten. The cost is that a file with such a name stops being valid. No deployment exists, so today that cost is zero, and it only grows.

**Decisions 3 to 5, the frontend proof.** The decision ledger already says terminated SCRAM over TLS, with certificates as an option. `psql` users have passwords. A verifier is what PostgreSQL itself stores, so you can make one with PostgreSQL and POOL never sees the password. It belongs to the principal and not to the pair of principal and database, because one person has one password. Channel binding ties the proof to the TLS channel. It matters when a certificate check can be fooled. Here both legs check certificates fully, and offering it needs the hash of the server certificate under the algorithm of its own signature, which needs a certificate parser. That is real work for a protection this slice does not lean on, so it waits for scope 006, and POOL does not advertise what it cannot do (`SEC-3`).

**Decision 6, the subset.** Passing everything through would make more clients work on day one. It would also mean that POOL forwards messages whose state it does not model: an error in the middle of a pipeline, a COPY that the client abandons, a function call. The protocol engine page says unsupported paths fail explicitly and never fall through as query data. A narrow path that is fully understood is the point of a walking slice.

**Decision 7, refusing to start.** Both answers are closed. The difference is who notices. A daemon that runs and refuses everything looks healthy to a service manager and has no status read to say otherwise until scope 008. A daemon that exits with a named finding is seen at once. When a status read exists, the other answer can be taken up again.

**Decision 9, SQLSTATEs.** Client libraries act on the class of a code: class 08 means the connection, class 53 means resources, class 57 means the operator. Using the codes PostgreSQL itself uses for the nearest situation lets them do the right thing without knowing POOL. `08007` is the standard code for a transaction whose outcome is unknown, and that is exactly the lost COMMIT. Constant text keeps names out of error messages for good.

**Decision 10, ending sessions.** Spec 0001 says a primary profile stays on its lease for its lifetime, and the overview says existing traffic may continue when freshness is lost. Neither says it may continue when a probe has positively shown that the node is no longer the writer. The invariant "keep writes on the verified primary" does. Silence and contradiction are different things in the rules of spec 0003 already (a silent replica does not close writes, a second primary does), and this decision applies the same line to running sessions.

**Decision 11, the timeline.** Spec 0003 asks for the timeline a node writes or replays on now, because the checkpoint value lags after a promotion. On a primary the WAL file name of the write position gives exactly that, and any role may ask for it. On a node in recovery, plain SQL does not show the replay timeline. The replication protocol does, and it needs the `REPLICATION` attribute, which lets a login read every byte of the cluster. Giving that to a monitor is against `SEC-5`. The rules read a replica's timeline for one purpose: to close writes when it is ahead. Taking the greatest value the node shows can only close earlier, never later than the checkpoint value alone, and it never opens anything. Scope 007 has a real replica and qualifies it there.

**Decisions 12 to 14, the three findings of the review.** They are written up in the update of spec 0003, because they change its interface. The common thread is decision 26 of that spec: a rule should not rest on a caller remembering something. A function that latches needs no reread. An opaque value and a guard on four names need no discipline. A board that cannot hold two samples and a clamp that cannot see the future need no duty, and a guard makes sure there is no second caller that goes around them. The stricter rule was the first draft. It was dropped when the existing tests of `0003 AC-11` turned out to assert an open answer for exactly those two inputs: making the rule stricter would have changed accepted evidence, and the guard reaches the same safety without it.

**Decision 15, the guard.** The rule protects two things: that no crate grows a socket by accident, and that control never gets the means to do the work of core. A list of exceptions by path, on top of a list that covers everything, keeps both, and a new crate is covered before anyone thinks about it.

**Decision 16, the crypto library.** Both candidates were resolved and built in the pinned Linux image on 2026-11-15. Both need the same two licenses. `ring` is smaller. `aws-lc-rs` is what the security requirements name, what `rustls` selects by default and tests most, and it brings no duplicate package. With `multiple-versions = "deny"`, `ring` would need the first `skip` entry in `deny.toml`. Either can be swapped for the other later: the calls this slice makes have the same shape in both.

**Decision 46, the fixture.** It is marked heavy because it changes the text of a fixture that an accepted spec quotes. It changes nothing a user can see. The fixture exists to show what an unknown table does, and `listener` was only ever a stand in for "a table this build does not know". Keeping that meaning needs another name. Keeping the name would have changed the meaning, the result and the evidence of `0001 AC-7`. A name of the same length was chosen so that no position can move, and it was checked by running the binary: the report has the same bytes in both formats. The developer had proposed `frontend`. That is a glossary term, so a later slice could make it a table and the same finding would come back. `no_table` cannot.

## Locked packages

Resolved on 2026-11-15 with Cargo 1.100.0, in a scratch project outside the repository that declares the six adopted dependencies and the three existing ones with the features of the spec. The real workspace adds its five members to this list. "Built for the Linux target" says whether the package is compiled for `aarch64-unknown-linux-gnu`. The others are in the lockfile for other platforms or for features that are off.

| Package | Version | Declared license | New in this slice | Built for the Linux target |
|---|---|---|---|---|
| `aws-lc-rs` | 1.19.0 | ISC AND (Apache-2.0 OR ISC) | yes | yes |
| `aws-lc-sys` | 0.47.0 | ISC AND (Apache-2.0 OR ISC) AND Apache-2.0 AND MIT AND BSD-3-Clause AND (Apache-2.0 OR ISC OR MIT) AND (Apache-2.0 OR ISC OR MIT-0) | yes | yes |
| `bitflags` | 2.13.2 | MIT OR Apache-2.0 | yes | yes |
| `bytes` | 1.12.1 | MIT | yes | yes |
| `cc` | 1.11.2 | MIT OR Apache-2.0 | yes | yes |
| `cfg-if` | 1.0.5 | MIT OR Apache-2.0 | yes | no |
| `cmake` | 0.1.58 | MIT OR Apache-2.0 | yes | yes |
| `dunce` | 1.0.5 | CC0-1.0 OR MIT-0 OR Apache-2.0 | yes | yes |
| `errno` | 0.3.15 | MIT OR Apache-2.0 | yes | yes |
| `fallible-iterator` | 0.3.0 | MIT OR Apache-2.0 | yes | yes |
| `fallible-streaming-iterator` | 0.1.9 | MIT OR Apache-2.0 | yes | yes |
| `find-msvc-tools` | 0.1.15 | MIT OR Apache-2.0 | yes | yes |
| `fs_extra` | 1.3.0 | MIT | yes | yes |
| `getrandom` | 0.2.17 | MIT OR Apache-2.0 | yes | no |
| `getrandom` | 0.4.3 | MIT OR Apache-2.0 | yes | no |
| `itoa` | 1.0.19 | MIT OR Apache-2.0 | no | yes |
| `jobserver` | 0.1.35 | MIT OR Apache-2.0 | yes | yes |
| `libc` | 0.2.191 | MIT OR Apache-2.0 | yes | yes |
| `libsqlite3-sys` | 0.40.0 | MIT | yes | yes |
| `memchr` | 2.8.3 | Unlicense OR MIT | no | yes |
| `mio` | 1.2.5 | MIT | yes | yes |
| `once_cell` | 1.21.4 | MIT OR Apache-2.0 | yes | yes |
| `pin-project-lite` | 0.2.17 | Apache-2.0 OR MIT | yes | yes |
| `pkg-config` | 0.3.34 | MIT OR Apache-2.0 | yes | yes |
| `proc-macro2` | 1.0.107 | MIT OR Apache-2.0 | no | yes |
| `quote` | 1.0.47 | MIT OR Apache-2.0 | no | yes |
| `r-efi` | 6.0.0 | MIT OR Apache-2.0 OR LGPL-2.1-or-later | yes | no |
| `ring` | 0.17.14 | Apache-2.0 AND ISC | yes | no |
| `rusqlite` | 0.42.0 | MIT | yes | yes |
| `rustls` | 0.23.46 | Apache-2.0 OR ISC OR MIT | yes | yes |
| `rustls-pki-types` | 1.16.0 | MIT OR Apache-2.0 | yes | yes |
| `rustls-webpki` | 0.103.17 | ISC | yes | yes |
| `serde` | 1.0.229 | MIT OR Apache-2.0 | no | yes |
| `serde_core` | 1.0.229 | MIT OR Apache-2.0 | no | yes |
| `serde_derive` | 1.0.229 | MIT OR Apache-2.0 | no | yes |
| `serde_json` | 1.0.154 | MIT OR Apache-2.0 | no | yes |
| `serde_spanned` | 1.1.3 | MIT OR Apache-2.0 | no | yes |
| `shlex` | 2.0.1 | MIT OR Apache-2.0 | yes | yes |
| `signal-hook-registry` | 1.4.8 | MIT OR Apache-2.0 | yes | yes |
| `smallvec` | 1.16.5 | MIT OR Apache-2.0 | yes | yes |
| `socket2` | 0.6.5 | MIT OR Apache-2.0 | yes | yes |
| `subtle` | 2.6.1 | BSD-3-Clause | yes | yes |
| `syn` | 3.0.10 | MIT OR Apache-2.0 | no | yes |
| `tokio` | 1.54.1 | MIT | yes | yes |
| `tokio-macros` | 2.7.2 | MIT | yes | yes |
| `tokio-rustls` | 0.26.6 | MIT OR Apache-2.0 | yes | yes |
| `toml` | 1.2.1+spec-1.1.0 | MIT OR Apache-2.0 | no | yes |
| `toml_datetime` | 1.1.3+spec-1.1.0 | MIT OR Apache-2.0 | no | yes |
| `toml_parser` | 1.1.5+spec-1.1.0 | MIT OR Apache-2.0 | no | yes |
| `tracing` | 0.1.44 | MIT | yes | yes |
| `tracing-core` | 0.1.36 | MIT | yes | yes |
| `unicode-ident` | 1.0.27 | (MIT OR Apache-2.0) AND Unicode-3.0 | no | yes |
| `untrusted` | 0.9.0 | ISC | yes | yes |
| `vcpkg` | 0.2.15 | MIT OR Apache-2.0 | yes | yes |
| `wasi` | 0.11.1+wasi-snapshot-preview1 | Apache-2.0 WITH LLVM-exception OR Apache-2.0 OR MIT | yes | no |
| `windows-link` | 0.2.1 | MIT OR Apache-2.0 | yes | no |
| `windows-sys` | 0.52.0 | MIT OR Apache-2.0 | yes | no |
| `windows-sys` | 0.61.2 | MIT OR Apache-2.0 | yes | no |
| `windows-targets` | 0.52.6 | MIT OR Apache-2.0 | yes | no |
| `windows_aarch64_gnullvm` | 0.52.6 | MIT OR Apache-2.0 | yes | no |
| `windows_aarch64_msvc` | 0.52.6 | MIT OR Apache-2.0 | yes | no |
| `windows_i686_gnu` | 0.52.6 | MIT OR Apache-2.0 | yes | no |
| `windows_i686_gnullvm` | 0.52.6 | MIT OR Apache-2.0 | yes | no |
| `windows_i686_msvc` | 0.52.6 | MIT OR Apache-2.0 | yes | no |
| `windows_x86_64_gnu` | 0.52.6 | MIT OR Apache-2.0 | yes | no |
| `windows_x86_64_gnullvm` | 0.52.6 | MIT OR Apache-2.0 | yes | no |
| `windows_x86_64_msvc` | 0.52.6 | MIT OR Apache-2.0 | yes | no |
| `winnow` | 1.0.5 | MIT | no | yes |
| `zeroize` | 1.9.1 | Apache-2.0 OR MIT | yes | yes |
| `zmij` | 1.0.23 | MIT | no | yes |

`ring`, `getrandom` 0.2.17, `windows-sys` 0.52.0 and the `windows-targets` family are in the lockfile because `rustls-webpki` can use `ring`. They are not built, and `cargo deny` does not count them as duplicates, because the feature that would use them is off. Rule W1 compares names as a sorted list, so `getrandom` and `windows-sys` are each listed twice in the baseline.

## Evidence gathered for this spec

All on 2026-11-15, on the workstation (macOS, Docker Desktop 29.9.2, `aarch64`). Nothing here is evidence for a criterion. It is what the decisions rest on.

| What | How | Result |
|---|---|---|
| The dependency set resolves | `cargo generate-lockfile` with Cargo 1.100.0 in a scratch project | 70 third party packages, listed above |
| The license policy | `cargo deny --locked check` (cargo-deny 0.20.2) with the `deny.toml` of the repository | Fails on `ISC` (4 packages) and `BSD-3-Clause` (2 packages), and on nothing else |
| The same with the five exceptions of the spec | The same command, on the scratch lockfile and on `crates/pool-config/fuzz/Cargo.lock` | `advisories ok, bans ok, licenses ok, sources ok` on both. An exception that is not met is a warning |
| The same with `ring` in place of `aws-lc-rs` | The same command | Also fails on a duplicate `windows-sys` (0.52.0 and 0.61.2) |
| The Linux build of the native code | A program that opens SQLite in memory, builds the `rustls` provider and a runtime, built and run in `rust:1.100.0-slim-bookworm` at the pinned digest, `linux/arm64` | Builds in 12 seconds with `gcc` only (no `cmake`), prints `sqlite 3.54.1`. The `ring` variant builds in 8 seconds. Both were built before the features `macros` and `io-std` of `tokio` were added after the cross check. That set was resolved and checked by `cargo deny` again, and not built again |
| The image of the fixture | `docker buildx imagetools inspect postgres:18.7-bookworm` | Index digest `sha256:ece55f7868da684e400f82f1030a050718adbdd65c23aa500c486f6e2950dc0f`. Not pulled |
| The probe query and the monitor rights | A disposable container of the local `postgres:18` image (PostgreSQL 18.6), a role with `LOGIN` only, the query of the spec | One row: the system identifier, `f`, `1`. `has_function_privilege` is true for all five functions. The role has no attribute |
| What PostgreSQL 18 reports at startup | A raw startup packet to the same container | `in_hot_standby off` is among the parameter status messages, and `BackendKeyData` follows them |
| `openssl` in the image | `openssl version` in the same container | Present (3.5.6). The local image is the Debian 13 variant. The Debian 12 variant was not pulled |
| A container reaching POOL on the host | A listener on `127.0.0.1` of the host, read from the container through `host.docker.internal` | Works on Docker Desktop |

The container was removed after the checks. Not checked, and left to the build to find: whether `psql` 18 is content without `BackendKeyData` (the runner up of decision 22 is the way out), whether the Debian 12 image holds `openssl`, the gateway address on a Linux engine, and the fuzz build on the hosted runner.

## Cross check findings

The draft was read by a second model (Claude Opus, read only, with no part in writing it) on 2026-11-15. It was asked first for decision completeness: every value or behavior a developer would have to invent. Its verdict was "accept after fixes". Under the `auto` decision mode the recommended fix was taken for each finding. All are applied in `index.md` and, where they touch it, in the update of spec 0003.

| # | Finding | What changed |
|---|---|---|
| 1 | The wrong identity case of AC-14 would have failed at TLS or login, so it never reached the identity rule | The certificates are made once and both containers get the same certificate, roles and passwords. The test waits for `evidence.writer_identity` in the log |
| 2 | Toward a node, the SCRAM header was not said. With `y`, PostgreSQL refuses the login as a downgrade | The backend connection now states the header `n`, the empty name, the nonce, `c=biws`, the nonce check and the `e=` answer |
| 3 | A guard on the name `StoredAuthority` does not stop code that holds a `StoreProbe` from changing it | Other crates get an opaque `JournalRead`. The guard refuses `StoredAuthority`, `StoreProbe`, `classify_startup` and `plan_bootstrap` outside `pool-control`. `0003 AC-19` was rewritten |
| 4 | `select!` needs the feature `macros` of `tokio` | The features `macros` and `io-std` are adopted. The lockfile count is 70, with `tokio-macros` new |
| 5 | No safe way to set the umask aside, and an empty journal file after a crash would block bootstrap for good | The binary creates the file with mode `0o600` and sets the mode again. An empty database counts as absent. AC-11 says "no journal file" |
| 6 | The SQL file of AC-18 would print other bytes on its second run | The file truncates its table first and uses fixed keys and values |
| 7 | The redaction test cannot search for names like `app` | Every fixture name starts with `pz7_` |
| 8 | The principal of the fixture could log in to PostgreSQL, against AC-16 | The verifier is read from a role made with `NOLOGIN` and dropped at once. The operator text says the same |
| 9 | When `NegotiateProtocolVersion` is sent, and a `CancelRequest` after TLS | Both are stated under Startup |
| 10 | `listener.failed` could happen after the log writer was installed | The log writer is installed after the bind. Steps 1 to 7 of `pool run` write findings, later lines are events |
| 11 | How a host name is resolved | The runtime resolves at each attempt and tries the addresses in order, within the deadline |
| 12 | Errors of accept, and file descriptors | An accept error is counted and followed by a wait of 100 ms. The descriptor budget is stated beside the memory budget |
| 13 | Build plan tracing: `run` tested before it exists, m1 to m3 built in two steps, comment lines in the guard file | Fixed in the tests table, the coverage table and the guard format |
| 14 | "Counts the lines that follow" was wrong once an omitted line follows | The line counts the kept findings |
| 15 | Things that happen after the answer were checked without a bound | A poll of 50 ms for at most 2 seconds |
| 16 | How the script tells Docker Desktop from a Linux engine | By the text of `docker info` |
| S1 | PBKDF2 at every backend login shares four blocking threads with the probe, so a burst of logins could close the cluster | The derived keys are kept per credential, salt and count (decision 43) |
| S2 | A round that waits for the journal delays the closed snapshot while the old open one is still fresh | The round works in memory and is published first. The write follows (decision 12, `0003 AC-17`) |
| S3 | A latch cannot be cleared in this slice | Stated under Consequences, with the way out through a new bootstrap |
| S4 | `57P01` with a query in flight hides that the outcome is unknown | `08007` whenever a query is in flight (decision 45) |
| S5 | Log writes on the workers of the runtime | A bounded queue and one writing task (decision 44) |
| S6 | The made up verifier always used the count 4096 | It uses the count of the first principal's verifier |
| S7 | One address can hold every handshake permit | Stated under the security model as not defended |

Facts the cross check corrected: `set_buffer_limit` of `rustls` bounds what is queued for sending and not received plain text; a read only open of a WAL database may create the two side files, so AC-10 speaks of the content of the main file; and `PGOPTIONS` in the environment of a client makes it send `options`, which is refused. One statement of the cross check was not taken over: it held that Cargo prints no warning for an unused dependency of a member. The scratch build of 2026-11-15 with Cargo 1.100.0 printed exactly that warning, so the spec keeps the sentence about it.

Not changed, and why: the cross check proposed guard lines for `ApprovedTopology` and `Quarantine` as well. With `StoreProbe`, `classify_startup` and `plan_bootstrap` fenced, neither type can reach a function that reads it from outside the crate, and `Quarantine` is a word that a comment may start a sentence with.

