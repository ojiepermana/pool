# ADR 0003. Control and session data model

The canonical decision and lifecycle status are in [spec 0003](../specs/0003-control-and-session-data-model/index.md). Its [rationale](../specs/0003-control-and-session-data-model/rationale.md) records alternatives, tradeoffs, and the decisions taken.

The decision is that every record about identity, sessions, configuration, observations and durable authority has one owner, one source and one restart rule, and that writes are admitted only when the configuration, the journal and a fresh observation name the same writer. A journal is created only by an explicit bootstrap, and a missing, foreign or restored journal keeps admission closed. Scope 003 builds the configuration keys for clusters, nodes and the identity map, and a third crate, `pool-control`, that holds the authority rules as plain tested functions. The journal file and the bootstrap command are built in scope 004. This ADR is an index entry, not a second decision record.
