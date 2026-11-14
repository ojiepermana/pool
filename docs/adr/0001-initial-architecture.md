# ADR 0001. Initial architecture

The canonical decision and lifecycle status are in [spec 0001](../specs/0001-initial-architecture/index.md). Its [rationale](../specs/0001-initial-architecture/rationale.md) records alternatives, tradeoffs, and sources.

The recommendation is a modular Rust daemon and separate CLI, session pooling first, PostgreSQL owned replication, and manual failover gated by verified fencing. This ADR is an index entry, not a second decision record. Feature decisions can receive their own ADR links when their specs are ratified.
