# Data plane

You can model each frontend as one owner task with bounded read and write state. It holds a session permit, an authenticated identity, a routing profile, a pinned backend lease, protocol state, deadlines, and a cancellation token. Avoid a task per message and avoid unbounded result buffering.

| Boundary | Input | Output and ownership |
|---|---|---|
| Listener | Socket, listener configuration generation | Connection permit or explicit overload rejection |
| Startup | Framed bytes and TLS policy | Validated startup identity, negotiated protocol |
| Auth | Identity and verifier/certificate reference | Verified principal, then operator map to allowed cluster/database/backend login/secret reference/profile |
| Initial admission | Mapped principal/database/profile, pool key and fresh topology | One eligible endpoint and initial lease, or bounded wait/refusal, before ReadyForQuery |
| Operation admission | Later syntax classification, pinned session state and policy | Allow or refuse on the existing lease, with reason and policy generation, never a new endpoint |
| Relay | Frontend/backend frame streams | Ordered messages, counters, updated protocol state |
| Release | Terminal frontend state and outstanding work | Clean idle backend or socket closure |

## Resource proposal

Provisional lab defaults are 1,000 frontend sessions, 100 total application backends per configured node, 100 queued acquisitions per pool, 5 second acquisition timeout, 5 second startup/auth timeout, and a 30 second graceful drain deadline. These are configurable starting points, not capacity claims. Reserve separate monitor and admin connection budgets and subtract them from PostgreSQL's connection budget. Per identity limits nest within global limits; pool partitioning must not multiply a global cap silently.

Use fair queues, per principal quotas, byte budgets, idle/lifetime controls, and a bounded number of new backend handshakes. Long transactions and COPY can hold leases, so include their occupancy in overload tests. Socket backpressure should stop producers rather than accumulating unlimited frames. Classification runs in a bounded worker lane with input and time limits.

## Failure rules

Once any query bytes may have been sent, a backend failure produces connection loss or the real server error. Never retry automatically. Only initial acquisition before the lease and ReadyForQuery are exposed may choose another eligible endpoint within the same profile and deadline. An established session never replaces a lost backend, even between statements. The session manager serializes cancellation dispatch with retirement, revokes virtual key mappings before release and retires physical backends after forwarded cancellation. Drain stops admission first and lets existing work finish to a deadline, then closes affected sockets visibly.
