# Security requirements

These are release requirements for implementation, not results already achieved.

| ID | Requirement | Evidence |
|---|---|---|
| SEC-1 | Require TLS for network application traffic and verify backend CA plus server name. Plain local development exceptions must be explicit. | Positive and negative certificate corpus |
| SEC-2 | Use independently authenticated frontend and backend identities. SCRAM verifiers are not backend passwords. | Identity map and SCRAM interoperability tests |
| SEC-3 | Advertise SCRAM PLUS only with valid channel binding data for that connection. Bound work factors and message sizes. | PLUS/downgrade/CPU budget tests |
| SEC-4 | Keep secrets in protected files or provider references, never committed configuration or CLI argument values. Redact Debug, errors and diagnostics. | Canary secret redaction checks and permission validation |
| SEC-5 | Preserve least privilege database logins. Monitor and promotion capabilities use separate restricted roles. | Database grant manifest and denial tests |
| SEC-6 | Authenticate all administration. Local TCP is not identity. Prefer protected Unix transport locally; remote exposure requires mTLS and explicit role mapping. | Transport and role/action matrix |
| SEC-7 | Journal administrative intent and audit result before acknowledging mutations; fail safely on journal storage failure. | Crash, full disk and idempotency tests |
| SEC-8 | Apply global and per principal connection, byte, queue, parser and authentication CPU budgets. | Saturation and slow client tests |
| SEC-9 | Use allowlisted structured fencing/recovery adapters with scoped credentials and verified completion. | Injection and wrong-target negative tests |
| SEC-10 | Disable automatic promotion and result caching by default. Reject unsupported enabled features. | Default configuration tests |
| SEC-11 | Keep logs/metrics/traces free of credentials, cancellation keys, SQL parameters and raw SQL by default. Database error text may also contain sensitive values. | Export snapshots and bounded label review |
| SEC-12 | Validate a whole configuration generation before publication. Rotation drains retired identities and handles explicit revocation. | Reload/rotation and concurrent lease tests |
| SEC-13 | Inventory dependencies and native components with license notices and advisories before release. | SBOM, lockfile and reviewed update evidence |
| SEC-14 | Audit security sensitive, protocol, parser and HA changes with independent review. | PR approval and test artifacts |

Use one deliberately selected rustls crypto provider, initially the compatible aws-lc-rs feature family, and avoid enabling a second provider through defaults. This is an implementation recommendation subject to the locked dependency review. No FIPS compliance is claimed. The frontend authentication feature must audit reused pgwire components and override inappropriate server parameter defaults.

## Operational defaults

Use an unprivileged service account, restrictive secret/journal permissions, no core dumps containing credentials by default, an isolated management network, and no broad Proxmox administrator credential in the data plane. Document certificate expiry alerts, credential rotation, lost credentials, emergency drain, compromised node quarantine and backup restoration. SQL diagnostics require an explicit restricted opt in and retention policy.

## Disclosure and support

The repository must publish a real private reporting channel before production releases. Until one is established, do not invent an email address or advertise a staffed response SLA. See [SECURITY.md](../../SECURITY.md).
