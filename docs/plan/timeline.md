# Timeline and estimation model

These are order of magnitude planning ranges. No calendar delivery dates are promised. An engineer week means five focused working days. Two experienced full time engineers are your confirmed planning baseline, with periodic independent protocol, security and SRE review. Estimates include implementation, tests and documentation; they are not estimates of this documentation session.

| Phase | Engineer weeks | Best elapsed weeks at two engineers | Realistic elapsed weeks at 65 percent effective capacity | Principal uncertainty |
|---|---|---|---|---|
| 0 Research and architecture | 4 to 6 | 2 to 3 | 4 to 5 | Complete baseline and safety contracts |
| 1 Foundation | 4 to 6 | 2 to 3 | 4 to 5 | CI/lab/toolchain integration |
| 2 Core proxy | 12 to 18 | 6 to 9 | 10 to 14 | Authentication, resets, protocol and drivers |
| 3 SQL routing | 10 to 16 | 5 to 8 | 8 to 13 | Semantic contract and replica consistency |
| 4 Production reliability | 14 to 22 | 7 to 11 | 11 to 17 | Durable operations, fencing and fault injection |
| 5 HA and recovery | 20 to 32 | 10 to 16 | 16 to 25 | Distributed authority and independent failure domains |
| 6 Selected advanced work | 24 to 48 | 12 to 24 | 19 to 37 | Cache correctness and mode feasibility |

Elapsed ranges are rounded up from effort divided by capacity. The realistic column uses `engineer_weeks / (2 * 0.65)`. It is a planning allowance for review, coordination, interruptions and discovery, not a measured team velocity. Summing rounded phase values differs slightly from rounding total effort.

| Cumulative milestone | Total engineer weeks | Best elapsed weeks | Realistic elapsed weeks |
|---|---|---|---|
| Core MVP, through Phase 2 | 20 to 30 | 10 to 15 | 16 to 24 |
| Routing preview, through Phase 3 | 30 to 46 | 15 to 23 | 24 to 36 |
| Operational V1, through Phase 4 | 44 to 68 | 22 to 34 | 34 to 53 |
| V1.5, through Phase 5 | 64 to 100 | 32 to 50 | 50 to 77 |
| Selected V2, through Phase 6 | 88 to 148 | 44 to 74 | 68 to 114 |

The totals above include the original research and planning allowance. They are not a forecast of remaining effort from this review. Refresh the remaining estimate from the actual team, lab availability and completed gates when implementation is authorized.

The first walking slice is a proposed 1 to 2 engineer week experiment after prerequisites. It is included in Phase 2 and is not added again. A failed experiment may change the range. Native/snapshot replication and complete parity are not bounded by the selected V2 estimate; they may be declined after feasibility review.

One engineer roughly doubles capacity based elapsed time and requires external review rather than eliminating it. Three or four engineers can parallelize harnesses, operations and documentation, but protocol, auth and routing contracts stay on the critical path. Do not divide the entire schedule by four. Reestimate after the first two completed slices using measured effort and defect discovery.
