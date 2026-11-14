# Starting development

The next queued unit of work is [001. Stack and workspace](scope/001-stack-and-workspace.md). The design in [spec 0001](specs/0001-initial-architecture/index.md) is ratified; the workspace does not exist yet. Implementation starts only on explicit request. The [scope index](scope/index.md) owns the queue and each numbered feature links to the requirements it delivers. The [dependency plan](plan/dependencies.md) explains what each stage needs.

## First handoff: the workspace

| Question | Handoff for scope 001 |
|---|---|
| What may be built once authorized? | A minimal Linux build of the daemon and offline configuration validation. Create a crate or add a dependency only if that path uses it. The proposed workspace layout is a boundary, not a list to scaffold. |
| What must be settled before editing? | Use spec 0001 and the actual checkout. In its build notes record the Linux target and toolchain pin, the smallest supported config, and the expected output and exit status of `pool check-config` for valid and invalid fixtures. Include [SEC-10](security/security-requirements.md): omitted or disabled cache and automatic promotion are safe; requesting either unsupported capability is rejected. Review only dependencies being adopted. |
| What demonstrates completion? | Reproduce the build from a clean checkout. Run the valid and invalid fixtures through the built binary without contacting PostgreSQL, including separate attempts to enable each unsupported capability. Check that the invalid fixture fails and that a seeded secret is absent from the output. Link the revision, commands, results and independent review from scope 001. |
| Where does work stop? | No listener, live query, session pool or administration API in this handoff. Standards/CI are tracked in [002](scope/002-standards-and-tooling.md); the first database write belongs to [004](scope/004-one-real-primary-path.md) after the [003 authority and identity contract](scope/003-control-and-session-data-model.md). |

## Handoff for later slices

1. **Before code:** The implementer and a reviewer use the numbered scope file to select one observable outcome. In the owning spec, record the PRD criteria by document and ID (for example, `PRD-001 AC-1`), exclusions, input/output and failure behavior, applicable budgets, and the fixture or environment that can demonstrate them. Name the implementer, reviewer and owner of any external prerequisite. If a needed contract or fixture is missing, record who will resolve it and which work must wait. Unrelated later decisions do not block the slice.
2. **At review:** Link the change and its [acceptance evidence](testing/strategy.md#acceptance-evidence) to the spec. For a public interface or persisted record, include its compatibility, migration and recovery behavior. For an irreversible action, state the last reversible step and how an unknown outcome is reconciled. Apply these only where the slice introduces the boundary.
3. **On completion or change:** Update the numbered scope file and its index status together when the evidence supports the change. If behavior changes, amend the owning PRD and obtain a decision in the feature spec before dependent work proceeds. Update affected milestones, tests or operating instructions by reference. A failed or unrun criterion stays visible; it does not become a release claim.

Use the existing scope file, spec and verification record for this handoff. A second backlog or a PRD per crate would separate the requirement from the work that proves it.
