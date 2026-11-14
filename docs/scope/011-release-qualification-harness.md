# 11. Release qualification harness · planned

[Scope index](index.md)

Run the declared compatibility, overload, security, benchmark, soak and four VM fault qualification.

**Done when:** Evidence supports every claimed V1 capability and unresolved safety failures block release.

The release maintainer indexes [acceptance evidence](../testing/strategy.md#acceptance-evidence) against the [V1 release gate](../plan/milestones.md) and declared [driver coverage](../testing/compatibility-matrix.md). Capture any missing or failing evidence and its owner. Run the remaining integrated drills on the release candidate; evidence collection starts with 001.

- [ ] Qualify the release candidate using the existing slice harnesses; link results and unresolved cases.
