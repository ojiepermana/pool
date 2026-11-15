# Control and session data model rationale

## Context

POOL is about to gain shared state. Scope 004 opens the first real connection, logs in to PostgreSQL with a backend login and sends a write. Before that can be safe, three things have to be true. The login must come from an operator's map and never from what the client typed. The node that receives the write must be the one an operator approved, and a probe must agree right now. And a restart must not quietly change either answer.

The planning package says all of this in sentences. Safety contracts 2, 3 and 8 of spec 0001 name the parts of a backend identity, the identity map and the durable writer. The control plane page lists proposed records and ends with three questions it leaves open: how an operator initializes a new cluster once, how a start handles missing or unreadable authority, and how a deliberate restore enters reconciliation. The glossary lists seven kinds of generation and says this scope settles what advances each one and what survives a restart. The risk register names the failure this is all about, `R-19`: a restart adopts the wrong writable node.

Several forces shape the answer. The workstation has no listener, no PostgreSQL connection and no administration API yet, and scope 003 may not build any of them. The project builds by tracer bullet, so a slice should leave something real that the next slice extends. The product has sixteen third party packages and no native code; SQLite would be the first, and a new license in its dependency tree is a decision only a person may take. Every later slice (reuse, replicas, administration, promotion) reads these records, so a second source for one fact would be expensive to remove. And a mistake here does not show as a failed test. It shows as a write on the wrong node after a bad day.

The cost of not deciding is that scope 004 decides it while also building TLS, authentication and a wire state machine, or that each slice brings its own idea of what a restart may trust.

## Options considered

### Option 1: Configuration keys and a plain, tested authority model now; the journal file in 004 (chosen)

Scope 003 builds what can be proven without any input or output: the configuration keys with their findings, the identifier and generation types, the identity map lookup, and a library of plain functions that classify a start, plan a bootstrap and decide writer admission. The journal format is fixed in the spec and built with the first slice that has something to write.

**Pros**:

- The four walks the scope asks for are tests today, not prose.
- The rule against `R-19` is reviewed and tested alone, before it is tangled with sockets and SQL.
- No dependency, no native code and no license decision is pulled into a slice that cannot use them.
- Scope 004 gets smaller.

**Cons**:

- A crate is created before anything in the binary calls it, so its interface is a guess about its caller.
- The journal schema is written down but not run by the product until 004.
- A third workspace member changes three baseline files and several documents.

### Option 2: Also build the SQLite journal now, with offline commands to create and inspect it

Adopt `rusqlite`, create the journal file with a bootstrap command, and show its state with a status command, all without a database connection.

**Pros**:

- A real path through the binary: an operator could create and inspect a journal today.
- The schema is exercised by the product at once.

**Cons**:

- Bootstrap must check the identity and role of every node first, and that needs a PostgreSQL connection that this scope may not open. A bootstrap without the check would have to be rebuilt in 004, and for one slice the tree would hold a command that creates authority without evidence.
- SQLite is native code with its own dependency tree. It needs a boundary review, the `NOTICE` file, and very likely a license that is not on the allowed list, which only a person may add. That would hold this whole scope for a decision that has nothing to do with the data model.
- A dedicated writer needs a thread or a blocking lane, which rule W3 forbids today.

### Option 3: A spec only, with no build

Write the records, the rules and the walks, and let scope 004 build all of it.

**Pros**:

- Nothing is built before its caller exists.
- No workspace change.

**Cons**:

- The walks stay sentences. Nothing fails when a later slice gets one wrong.
- Scope 004 grows by the configuration schema, the identity map and the authority rules, on top of the largest protocol work in the plan.
- The shared convention for identifier and generation types would still be pending when the first slice needs it.

### Option 4: Put the authority model into `pool-config`

Keep two workspace members and add the model as a module of the configuration crate.

**Pros**:

- No third crate and no baseline change.

**Cons**:

- It breaks the boundary the repository structure sets: configuration types "should not become a generic dumping ground for every shared runtime record". Observations and authority belong to control.
- It would have to move in 004 anyway, with every import.

## Rationale

Option 1 follows from the forces. What this scope can prove without input or output is exactly the part that carries the safety argument: which combinations of configuration, journal and observation may open writes. Building that as functions over values makes the three way agreement a property that a loop over 8192 cases can check, and it makes "closed" the only result that code can produce by accident, because an open admission can only be built from a ready authority.

Option 2 fails on its own terms. The one command it would add, bootstrap, is the one command that must not exist without evidence from PostgreSQL. And it would trade a data model decision for a dependency and license decision that a person has to take.

Option 3 leaves the scope's own "Done when" without evidence, and the lead asked for a build whose steps can be run. Option 4 saves three baseline lines and spends a boundary.

The price of Option 1 is an interface built ahead of its caller. That is acceptable here because the functions are small and have no state: changing a signature in 004 costs minutes, while the rules behind them are the part that must not move.

## Decisions taken

The session ran in the `auto` decision mode: each choice below is the recommended option, taken without asking, with the runner up and the way back. "Heavy" marks a public contract, a format on disk or a part of the security model.

| # | Question | Taken | Runner up | Heavy | What changes if the runner up is chosen |
|---|---|---|---|---|---|
| 1 | What does scope 003 build? | Configuration keys and a plain authority model; the journal file in 004 | Build the SQLite journal now | yes | Build plan gains the adoption of `rusqlite`, a license decision by a person, the `NOTICE` file and two commands. AC-13 changes |
| 2 | Where does the authority model live? | A third crate, `crates/pool-control` | Wait for 004, or a module in `pool-config` | yes | Steps 1 and 5 and AC-13 go away or move. The walks stay prose until 004 |
| 3 | What is a configured server called? | Node, table `[[node]]` | Backend, table `[[backend]]`, as the control plane page and an example in spec 0001 say | yes | Every `node.*` key, `NodeId` and the fixtures are renamed. The glossary meaning of backend (a connection) would then have two senses |
| 4 | How does the configuration name the writer? | One key per cluster, `expected_writer`, that must keep agreeing with the journal | A role key on every node; or a key used only at bootstrap and ignored afterwards | yes | With a bootstrap only key, `authority.config_conflict` no longer covers the writer, walk 3 row b opens, and a stale file is never noticed |
| 5 | How is a first start told from a lost journal? | An explicit bootstrap command, plus the authority ID repeated in the configuration as a witness | The explicit command alone | yes | `Unconfirmed`, `Missing` and `Mismatch` go away. A lost journal looks like a first start, and a journal of another daemon is accepted. AC-8 falls |
| 6 | How does a restore enter reconciliation? | A supported backup carries the hold. Reconcile gives the journal a new authority ID. The timeline rule catches part of a raw copy | A witness file outside the journal, or comparing the inode of the file | yes | Another file to protect and to explain, and it still does not survive a rollback of the whole VM |
| 7 | What is the journal on disk? | One SQLite file, `application_id` and `user_version`, three strict tables, forward migrations applied at start | An explicit migration command | yes | A start with an older journal stays closed until the operator runs the command. More steps in every upgrade |
| 8 | Which generations survive a restart? | Topology epoch, authority revision and start sequence, in the journal. All others live in memory and are scoped by the start sequence | A durable configuration generation | yes | Every configuration load writes the journal. A reload needs storage even when nothing durable changed |
| 9 | What does a finding inside a repeated table carry? | Key and position, a fourth location | Position only | yes | A missing key in the third identity would say where and not which key |
| 10 | Does the valid report show the new tables? | No. It stays as spec 0001 defines it | Counts of clusters, nodes and identities | no | The output contract of spec 0001 and its byte for byte tests change. The report would hold numbers derived from the file |
| 11 | Where is the credential reference? | A key in the identity entry, `credential_file`, added now and read in 004 | A separate `[[credential]]` table that identities name | yes | One more table and reference check. Useful if many identities share one secret |
| 12 | What may default telemetry name? | Identifiers, generations and reason names. Never principal, logins, database names, hosts or paths | Allow principal and login names | yes | Logs show who logs in as what. The identity ID loses its purpose |
| 13 | How exact is the identity map? | Exact pair, byte for byte, no wildcard, no default | A wildcard for the database | no | `resolve_identity` gains an order of precedence, and a typo in a database name can match the wildcard |
| 14 | What does `routing_profile` accept today? | `"primary"`, default. `"replica"` is refused with its own code | Leave the key out until 007 | no | No `config.unsupported_value` yet. The key arrives with 007 |
| 15 | Does missing evidence about a replica close writes? | No. Only a positive conflict closes. Bootstrap needs every node once | Every node must be fresh for a write | yes | One dead replica stops all writes. Walk 3 row j closes |
| 16 | How do W4 and W5 learn the packages? | From the manifests, as today, plus a check of path packages in the lockfile | Read the dependency edges from the lockfile | no | The manifest reader of W5 is replaced and proven again |
| 17 | Does a quarantined node that says it is a primary close writes? | No | Yes | yes | A former primary that comes back unfenced can stop the cluster after a correct promotion. Walk 3 row k closes |
| 18 | Does a conflict stay closed when its evidence disappears? | Yes. A durable conflict latch per cluster, set by the daemon, cleared only by an operation | A latch in memory only, or none | yes | Without it, writes reopen to the approved writer when the second primary drops out of sight, or after a restart. The `conflict_latched` column, reason and walk 3 row o go away, and the citation of `PRD-006 AC-2` has to go too |
| 19 | Does the journal record the nodes of a cluster? | Yes, as members. A member missing from the configuration is a conflict | Record the writer only | yes | The `member` table and walk 3 row p go away. Taking a node out of the file then hides what it would report |
| 20 | Is the time of validation a criterion? | Yes, a new `AC-15`, with Build plan step 8 | Widen the rule under Keys and add no criterion; or make it a part of `AC-3` | no | Without a criterion nothing fails when the counting goes back to once per entry. Inside `AC-3`, the evidence that milestone 2 recorded for `AC-3` would have to be taken again |
| 21 | Does the bound cover an invalid file, and a file with every entry on one line? | Yes, both | Valid files only | no | Made input 3 goes away and a table of line starts is enough. A generated file with one mistake per entry on a long line can then take minutes |
| 22 | How is the time checked? | An upper limit of 10 seconds on one call, in the debug build of `cargo test`, on three inputs at the file limit, plus one small input whose findings come against file order | A ratio between a half size and a full size input; or a release build timed by hand in the verification record | no | A ratio needs two short timings and fails by noise. A run by hand is not repeated by CI |
| 23 | Does a cluster with a failed `id` stop check 3 for all clusters? | No. That cluster alone is passed over. This is what the opening sentence of step 6 already said, now written into check 3 | Skip the whole check | no | A bad cluster `id` would hide a wrong `expected_writer` in another cluster until the next run |
| 24 | May the system identifier and the authority ID appear in default telemetry? | Yes, in log fields and default diagnostics, never as metric labels. The timeline ID too. The observed role may also be a label | Never by default for the system identifier: `[REDACTED]` in `Debug` | yes | `ClusterConfig` and the observation records need a hand written `Debug`, a Build plan step is added, and a log of an identity mismatch cannot show the two numbers |
| 25 | May a generation be a metric label? | No. A log field or the value of a metric only. This narrows what decision 12 allowed | Leave generations in the label column | yes | Label values without a bound, against `T-12`. No code changes either way, because nothing writes a metric yet |
| 26 | Is a `Ready` state tied to the configuration that confirmed it? | Yes, in code now: `writer_admission` compares the witness again. New `AC-16` and Build plan step 9 | Text only: one sentence under writer admission and a rule for the reload of 009 | yes | `AC-16` and step 9 go away. The sentence for the caller and the line in Follow-up 6 stay. The safety then rests on scope 009 remembering to classify again |
| 27 | May `TimelineId` and `MonotonicTime` derive `Hash`? | Yes. The spec now lists it | Remove the derive from the code | no | A Build plan step that deletes two words and buys nothing |

### Notes on the heavy ones

**Decision 3, node.** The glossary owns the word backend and gives it one meaning: a connection that POOL opens. The data plane page already counts "application backends per configured node", and the administration PRD has a `/v1/nodes` route. Calling the configured server a backend too would give one word two meanings in the pool key, where "which backend" and "which server" are different questions. The example `[[backend]]` in the configuration rules of spec 0001 illustrates the syntax of a repeated table; no key was fixed by it.

**Decision 4, three way agreement.** Safety contract 8 says configuration conflicts fail closed, and the control plane page says that a configuration which contradicts durable authority keeps admission closed until an explicit reconciliation. A key that is read once and then ignored cannot contradict anything, so it could not meet either sentence. The price is real: after a promotion the file must be updated before writes open. That is one line in a reviewed procedure, against a file that silently drifts from the truth until the day someone bootstraps from it.

**Decision 5, the witness.** A daemon that finds an empty directory has no way to know whether a journal ever existed. Something outside the journal has to remember. The configuration is the only other durable thing an operator already controls and backs up. The same value also binds a configuration to its journal in both directions, which catches a state directory that points at another daemon's journal.

**Decision 6, restore.** A restored journal is correct about the past and silent about everything since. The safe rule is that it may not speak until an operator has compared it with the present. Putting the hold into the backup itself means the rule holds even if the restore procedure is forgotten. Giving the journal a new identity at reconcile solves a second problem with the same mechanism: a request that named an old revision or epoch can never match a number that the restored journal happens to reach again. What stays unsolved is stated plainly in the consequences: a raw copy, or a snapshot rollback of the whole VM, with the old writer up and nothing newer reachable.

**Decision 8, start sequence.** Without it, a configuration generation of 2 before a restart and a configuration generation of 2 after it are indistinguishable, and a reload request written against the first would be accepted against the second. One durable counter that scopes every memory generation is smaller than making each of them durable.

**Decisions 15 and 17, what closes writes.** Fail closed is the rule for uncertainty about the writer. Applying it to every node would let the least reliable replica decide the availability of the primary, and would let a fenced former primary, which is exactly the node most likely to misbehave, hold the cluster after an operation that was done right. Both exceptions are narrow: silence about a replica, and a node that an operator already excluded.

**Decisions 18 and 19, added after the cross check.** Both close the same kind of hole: evidence that can disappear. Admission is a function of what is observed now, so a second primary that drops off the network would have reopened writes by itself, and a node deleted from the file would have stopped being asked. `PRD-006 AC-2` says two observed primaries close writes until explicit reconciliation, and the control plane page says a configuration that removes durable authority keeps admission closed. The latch is the one case where an observation writes to the journal. That is acceptable because it can only restrict: the rule that an observation never grants authority still holds. The cost is that until scope 009 brings reconcile and detach, the only way out of either state is to put things back or to bootstrap anew.

**Decisions 20 to 22, added after the verification of milestone 2.** The checker measured that `validate` counted line feeds from the top of the file for every entry and every value: 12 seconds in a release build and 241 seconds in a debug build for a valid file of 16000 nodes. The old rule named only the uniqueness and reference checks, so its letter was met. A rule that no test holds is how this happened, so the rule gets a criterion. A new ID is used because `AC-1` to `AC-6` already have recorded evidence, and a criterion that grows after its evidence was taken makes that evidence say less than it seems to. The bound covers invalid files because the daemon of scope 004 will run the same function at start, and a start that hangs for minutes on a generated file with a repeated mistake is a failure of the bound that the 1 MiB limit exists to give. It covers the one line shape because the checker measured it (12000 inline tables, 16 seconds in a release build), and because it is the shape where the obvious repair, a table of line starts, is still quadratic. The limit is an absolute number and not a ratio. A ratio looks more exact, but it divides two timings of a fraction of a second, and one pause of the machine moves it past any threshold. The absolute limit sits about 25 times above the correct code and about 16 times below the broken code, so it has to be wrong by a large factor before it gives a false answer in either direction. A test cannot interrupt the call, because rule W3 allows no thread, so a regression shows as a test that runs for minutes and then fails. That is slow, and it is still red.

**Decision 24, the system identifier.** Three things speak for showing it. PostgreSQL does not treat it as a secret: `pg_control_system()` returns it, and a replication client gets it in answer to `IDENTIFY_SYSTEM`. It is no credential, so reading it in a log lets nobody do anything. And it is the one value that explains an identity mismatch: the reason says that a node reported another system identifier, and without the numbers you have to open a session on each node to learn which one. Two things speak against. It names a cluster for its whole life, so a log that leaves your hands can be matched with a backup or a replica of the same cluster. And its upper bits hold the time the cluster was initialized. The first is just as true of the cluster ID that every log line already carries. The second tells a reader how old the cluster is and nothing more. Neither opens a door. The runner up, redaction, would cost a hand written `Debug` on every type that carries the number and would take the most useful field out of the event that most needs it. Metric labels are held back for a different reason: a label is copied onto every sample and often leaves the host for a hosted monitoring service, and the cluster ID already names the cluster there.

**Decision 26, the witness at every admission.** The checker of milestone 3 passed a state classified `Ready` under witness `X` together with a configuration with no witness, and with one with witness `Y`, and got `Open` both times. The spec text asked for exactly that, and scope 004 cannot reach it, because it classifies once with the one configuration it has. The case arrives with the reload of scope 009. Writing a rule for 009 would be enough if every later slice read it. This spec does not rely on that anywhere else: admission compares the writer, the system identifier and the members of the journal with the configuration on every call, and the consequences promise that a later slice cannot open writes by forgetting a check. The identity of the journal was the one agreement that was checked only once. Comparing it again costs one comparison of 16 bytes, changes no signature and no reason name, and gives the answer a fresh classification would give. The price is a second small build step after the feature was verified, and one more criterion.

## Evidence gathered for this spec

- **Positions for findings in repeated tables.** A small program with `toml` 1.2.1 and `serde` 1.0.229 (the locked versions) read each fixture through `toml::Spanned`. The span of an entry written with a `[[...]]` header starts at the first bracket of the header; for an inline table it starts at the brace; the span of a value starts at the value. The lines and columns in the fixture table of the spec are the output of that program, run on 2026-11-15.
- **Journal schema.** The four `CREATE TABLE` statements and the pragmas were run with the `sqlite3` command line tool, version 3.54.0, on 2026-11-15. The tables were created and rows were inserted. A second `store` row was refused by the `singleton` check, a member or quarantine row for an unknown cluster by the foreign key, a repeated member by the primary key, and a latch value of 2 by its check.
- **Validation time.** On 2026-11-15, on an Apple M1 Max, a copy of the workspace outside the repository built the three made inputs of `AC-15` by the rule in the spec and ran them through `validate` in a debug build with `CARGO_PROFILE_DEV_DEBUG=0`. With the code of step 3, inputs cut to 20000 bytes already took 0.17, 0.51 and 0.72 seconds, and made input 3 gave 345 findings, the first at line 2, column 60. With the position counting replaced by a constant, the full inputs (13217, 16471 and 17268 entries; 1048511, 1048558 and 1048559 bytes) took 0.34, 0.37 and 0.39 seconds, and made input 3 gave 17268 findings. A release build of the same work on a file of 800 KB took 0.03 seconds. These numbers are the floor that correct code adds a little to. CI does not set `CARGO_PROFILE_DEV_DEBUG=0`; that setting changes what the compiler writes for a debugger and not the speed of the code in any way that matters here. The base of the fixtures with a second node `pg1` whose port is 0 (made input 4) gave `config.invalid_value` at line 21, column 8, then `config.duplicate_id` at line 18, column 6. The copy was not kept.
- **Guards.** `scripts/prove-checks.sh` case 9 uses `crates/pool-core`, so removing `crates/pool-control` from the forbidden paths leaves it valid. Case 17 edits the first line of the scope 003 file and depends on the word `planned` there.

None of these experiments is product evidence. They exist so that the Build plan does not ask the developer to discover them.

## Cross check findings

An independent read by another model (Claude Opus, read only, no access to the author's conversation) ran on 2026-11-15 on the first draft. It checked every fixture position, every row of the four walks against the rule tables, the mapping of criteria to Build plan steps, and the guard changes, and found those consistent. It raised 14 gaps and 7 soundness findings. All were taken, under the `auto` decision mode. None is left open.

| Finding | What was done |
|---|---|
| Major: a conflict did not stay closed once the second primary was out of sight, against `PRD-006 AC-2` | Decision 18: the durable conflict latch, the reason `authority.conflict_latched`, walk 3 row o |
| Major: the journal did not record the nodes, so editing the file could hide evidence | Decision 19: the `member` table, the wider `authority.config_conflict`, walk 3 row p |
| Major: the start wrote to the journal before classifying it, so it could change a journal of another daemon | The start now reads and classifies first and writes only in `Ready` |
| The lookup index of the identity map could print names through the derived `Debug` | The index is its own private type with a hand written `Debug` |
| "Such a node" in the timeline check was unclear; reasons could repeat; a tie of two sample times was undefined; bootstrap did not say which nodes take part in its later checks | Each is written out under writer admission, bootstrap and the freshness rule |
| The new W4 check would fail the existing test tree; the evidence of step 1 could not run before its commit | The test helper gets a `source` line; the proving script runs after the commit |
| Two criteria promised things no test showed; the documentation comments had no check | `compile_fail` documentation tests are named for AC-6 and AC-12; AC-14 says its comment half is checked by reading |
| Derives and `Display` of the new types were not given | Stated for both crates |
| One fixture description could be read two ways | Reworded |
| The pool key named a `pool-control` type that `pool-core` cannot see | The source is now the route view of `pool-core`, filled by the daemon |
| Scope 004 needs a lock on the state directory, a way to read the authority ID again, and a current timeline source | Added to Follow-up 4 |
| The claim that only a ready authority can open was too strong while `StoredAuthority` has public fields | The claim now names its condition: the journal reader is the only code that builds a `StoreProbe` |
| The glossary rows "Observation" and "Approved writer" still say backend for a server | Document correction 4 |
| The reason given for the duplicate endpoint check was overstated | Reworded: an alias that slips through closes writes, it never opens them |

### Cross check of the update of 2026-11-15

The update that added `AC-15`, the sentence in cross check 3 and the telemetry columns was read by another model (Claude Opus, read only, no access to the author's conversation) on 2026-11-15. It built the three large inputs again from the text and got the same counts and lengths, confirmed column 60, traced in the code that made input 3 gives one finding per entry and no other, and found the new sentence in check 3 in agreement with step 6 and with the code. It agreed with an absolute limit over a ratio. It raised 10 gaps and 5 soundness points. All were taken, under the `auto` decision mode.

| Finding | What was done |
|---|---|
| The telemetry table listed the system identifier and the authority ID under a header that allows metric labels, against its own paragraph. The timeline ID and the observed role had no clear answer for labels. Generations, which only grow, were allowed as labels | The table has three columns. Decision 25 moves generations out of the label column |
| The list of `Debug` texts left out `ValidatedConfig` and the identifier types | The paragraph now names every type that holds one of the values |
| "A valid file needs no line and no column" was written as a consequence of the rule, and it ruled out a forward cursor that the next sentence allowed | It is now part of the suggested way |
| No test showed that findings keep their order after the positions are filled in | Made input 4 |
| `AC-15` compared only the first and the last column, and "one finding per entry" for the binary named no check | Every column is compared, and the expected output of the binary is written out |
| "Within the same 10 seconds in both formats" could be read two ways | Each run by itself, from start to exit |
| A test cannot stop a slow call, because rule W3 allows no thread | Said under the limit and in the consequences |
| "Step 6" could mean step 6 of spec 0001 | Now "after the cross checks (item 6 of step 5)" |
| The rule covered the parser and the standard maps, which the developer does not control | The rule is about the code of `pool-config` |
| The bound was stated in two ways | The bullet under Keys points to the one rule |
| The margin below the broken code was overstated | Scaled to the entries of input 1: about 16 times |
| One slow shape passes the test | Named under "What the test does not catch" |
| Two statements about PostgreSQL and one argument in the rationale went too far, and the quote of spec 0001 was out of context | Reworded |

The addition of `AC-16` (decision 26) was read the same way, by the same kind of reader, on the same day. It found the order against check 1 clear, no changed answer in any walk, in the freshness test or in the model of `AC-12`, the sentence about a fresh classification true for every configuration that can be valid, and no other function with the same gap. It raised 3 gaps and 3 soundness points, all taken.

| Finding | What was done |
|---|---|
| The two cases of `AC-16` could not tell the right build from one that adds the reason and still runs the other checks | Each case also observes a second primary, and one call names a cluster the configuration does not have |
| Step 9 did not name the documentation comment of `writer_admission`, which becomes wrong | Added to step 9 |
| The new consequence claimed that any swapped configuration closes | It now says: another witness or none |
| Follow-up 6 named only a changed or removed witness | It now names adding one, and `control.state_directory` |
| Classifying again at a reload was not tied to the start sequence | Follow-up 6 leaves the reading of the journal at a reload to 009 and says the start sequence does not advance |
| Two wordings were loose | "With the same journal", and "identifier and generation types" |

## Review of 2026-11-15

The independent [review](../../reviews/2026-11-15-scope-003-control-and-session-data-model.md) of the range `ca9a897..be06393` ended with approve with nits: no blocker, no major finding, 15 minor findings and 18 nits. Scope 003 closes on the tree that was verified. This update of the spec, written on the same day, records what the review found in the places that scope 004 and scope 009 read. It adds no criterion, no Build plan step and no decision, and it changes no code. There was no cross check of this update, because nothing was decided in it.

**Why nothing changes now.** The three findings about `writer_admission` (m1, m2, m3) are limits of what the caller passes in, and no caller exists yet. Closing them in code now would mean guessing the shape of the journal reader and of the probe list before scope 004 designs either. The spec already says that the interface of `pool-control` may change when real inputs arrive and that the rules should not. What scope 004 must not do is call the function before it has read the three points, so each one is named where the rule lives and again in Follow-up 4.

| Finding | Sorted as | Where it is recorded in `index.md` |
|---|---|---|
| m1, a `Ready` state cannot learn of a later latch | A duty of scope 004, with a recommended shape and a test it owes | Follow-up 4. Notes under writer admission, after walk 3 and in the first consequence about open admission |
| m2, any crate can build a journal value | A duty of scope 004, when the journal reader is built | Follow-up 4. The same consequence |
| m3, a sample after `now` or a repeated sample hides a second primary | Two duties of the caller, and one question for scope 004 | Follow-up 4. Note under the rule "Fresh" |
| m4, the lockfile check of W4 has one way around it | Work for the next slice that touches `tools/repo-check/` | Follow-up 10. Note under Workspace and guards. One sentence in Follow-up 10 of spec 0002 |
| m5, two million findings from one file | A decision for the user, with three options and a recommendation | Follow-up 11 |
| m6, the cost of one admission | A question for scope 004, or scope 008 | Follow-up 4 |
| m7, the unused workspace dependency | A known exception that ends with scope 004 | Follow-up 4. Note under Workspace and guards |
| m8, control characters above U+007F | A gap, not a choice. Closing it changes which files are valid | Follow-up 12. Note at the name syntax |
| d1, row k against the letter of `PRD-006 AC-2` | The exception is now stated where the PRD is cited. The note in the PRD is for the user | Follow-up 13 and Follow-up 7. Notes under the criteria, under writer admission and after walk 3 |
| t6, no guard keeps a clock or a file out of `pool-control` | A proposal for scope 004 | Follow-up 4 |
| n2, the expiry has no range | A duty of scope 004 | Follow-up 4 |
| n4, case 17 fails when scope 004 leaves `planned` | With the slice of m4 | Follow-up 10 |
| n12 and n15, text of the spec behind the code | With the next update that changes the interface | Follow-up 14 |
| n16, the open case of `R-19` is not in the risk register | The "Document it" box | Follow-up 8 |

Not recorded in this spec, because another file or another session owns each one: t1, t2, t3, t4 and t5 (tests, with the tester), n5, n6, n11 and n18 (code comments and the verification record), n1, n3, n7, n8, n9, n10 and n17 (code and tests), n13 (rows of spec 0001 and spec 0002), n14 (the glossary), and the first half of n16 (`AGENTS.md` and `CLAUDE.md`, which item 6 of the document corrections already gives to the "Document it" box).

**m1, why a function in `pool-control` is the recommended shape.** Reading the journal again after every write also works, and scope 009 may need it anyway for a reload. It leaves one case to the memory of the daemon: the write that failed. A function that turns a `Ready` state and a closed answer into a `Ready` state with the latch set needs no journal, so it covers that case too, and the latch can then not be forgotten by a caller. That is the argument of decision 26. It stays a recommendation: the architect of scope 004 sees the journal reader and decides.

**m5, why the recommendation is both, in two steps.** The buffer removes one system call per finding and changes nothing a user can see, so it needs no decision. It does not bound memory or the size of the output. A limit does, and it changes a public contract: `AC-3` says the whole configuration is reported in one run, and a last line that says findings were left out is new output under spec 0001. That is why the user decides it, and why this update does not pick the limit. The moment that matters is the first start of the daemon in scope 004, when "bounded resources" stops being about a command at a desk.

**m8, why it is a gap.** No decision in the list above covers the control character list. It was written as the ASCII set and never weighed against the rest of Unicode. Decision 12 keeps names out of default telemetry, which is why nothing is exposed today. Refusing more characters makes files invalid that are valid now, so it is a change of behavior and not part of this update.

**d1, why the exception stands.** Decision 17 and its note above give the reason. The review agreed with the decision and asked for one thing: say it where the requirement is cited. The spec now does. It cannot say it in the PRD.

## Update of 2026-11-15 with spec 0004

[Spec 0004](../0004-one-real-primary-path/index.md) designs the first caller of the authority functions. Follow-up 4 and Follow-up 14 of this spec said that three findings of the review would come back as an update of this spec, with criteria, when that caller is designed. This is that update. It was written by the architect session of scope 004 under the `auto` decision mode, in the same change as spec 0004. Because this spec is accepted, every decision below is marked heavy, no existing text was reworded, and the criteria `AC-1` to `AC-16` keep their text and their evidence.

| # | Question | Taken | Runner up | Heavy | What changes if the runner up is chosen |
|---|---|---|---|---|---|
| 28 | How does a `Ready` state learn of a conflict latch that was set after the start? (m1) | Two functions of the state: `latch_conflict`, which sets the latch in memory from a closed answer, and `latch_recorded`, which takes the revision from a receipt that only the journal can make. New `AC-17` | Read and classify the journal again after every write, and keep a memory in the daemon for the write that failed | yes | `AC-17` describes a reread. The case of the failed write rests on the daemon again |
| 29 | Does the rule "Fresh" get stricter for a sample that cannot be used? (m3) | No. One caller, `build_snapshot`, a board with one sample per node, a clamp of now, and a guard on the name `writer_admission`. New `AC-18` | A new reason, `evidence.sample_unusable`, that closes when the latest sample of a node other than the writer is in the future or tied | yes | A new reason name. Two assertions in the accepted tests of `AC-11` (`ac11_the_latest_sample_decides_even_when_it_is_from_the_future` and `ac11_a_tie_hides_evidence_only_at_the_latest_time`) turn from open to closed |
| 30 | Who may build a journal value? (m2) | Everyone inside `pool-control`, nobody outside: other crates get an opaque `JournalRead`, and a source guard keeps `StoredAuthority`, `StoreProbe`, `classify_startup` and `plan_bootstrap` out of them. New `AC-19` | Private fields and one constructor that runs the checks of the reader | yes | The walk tests are rewritten to the constructor. Any crate can still call it, so the gap narrows and does not close |

**Decision 28.** This is the shape that Follow-up 4 recommended, for the reason of decision 26: the rule should not rest on a caller remembering to do something. The receipt answers the second half of the finding, the authority revision of an open answer. The latch is set in memory first and the write follows, so that a slow disk cannot delay closing. That order came from the cross check of spec 0004: the first draft passed the receipt into the same call, which made the daemon wait for the write before it could publish a closed snapshot. When the journal recorded the latch, the revision in memory follows the journal. When the write failed, the latch still closes the cluster in memory, the revision stays, and the daemon tries the write again. A restart after a failed write forgets the latch, as the text under "A conflict stays until someone looks at it" already implies by "for the rest of the run". If the second primary is still there, it is latched again at once.

**Decision 29.** The first draft of the update took the stricter rule. Reading the tests showed that `AC-11` is pinned by assertions that expect an open answer when a node other than the writer has a latest sample from the future, or two samples at the same time. A stricter rule would have changed accepted evidence to defend against inputs that the one real caller cannot produce. Fencing the caller reaches the same safety and changes nothing that was accepted: the board cannot hold two samples of a node, `build_snapshot` cannot see a sample in the future, and no other code may call the function. The cost is that the safety now has three parts where a stricter rule would have had one, and that the guard matches text, so a comment that holds the name `writer_admission` in another crate is refused too.

**Decision 30.** Private fields look stronger than a guard on a name, and they are not: the tests need a way to build a journal value, and whatever they use, other crates can use. A constructor that checks consistency does not check truth. Only a rule about who may touch the value keeps a made up journal out of the daemon. The first draft guarded the one name `StoredAuthority`. The cross check of spec 0004 showed that this is not enough: the daemon would still hold a `StoreProbe`, could change the writer inside it without naming the type, and pass it on. So the daemon no longer holds one. The reader returns a `JournalRead` that can only be asked three things, and the guard refuses the four names that would let code go around it.

**The three texts of Follow-up 14** were brought in line by notes. For the rule "Fresh" the note says that the code is the statement that holds, because the difference is only in which further reasons a closed answer lists.

**Point n2** needed no change here. The range belongs to the configuration key, which spec 0004 defines, and `build_snapshot` reads the expiry from the configuration.

**Decision 31, added on 2026-11-15, the name of the method.** `AC-19` as first written named the method `JournalRead::plan_bootstrap` and forbade the text `plan_bootstrap` to the crates that have to call it. The guard was built as written, and the first real caller, `pool bootstrap`, was refused by it. Taken: the method is named `bootstrap_plan`, and the guard does not change. Runner up: an `allow` line for the calling file, which would also have let that file call the free function with a journal value of its own. It is marked heavy, as every change to this accepted spec is. The criterion keeps its ID and its text, with a dated note that says which half holds.
