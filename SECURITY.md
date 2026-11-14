# Security policy

POOL is a planning stage project. There are no supported production releases and no security support SLA yet. The security model and required controls are in [the threat model](docs/security/threat-model.md) and [security requirements](docs/security/security-requirements.md).

Before the first public release, maintainers need to configure a private vulnerability reporting channel, such as GitHub private vulnerability reporting on the actual repository, name responders and publish supported release lines. No remote repository exists in this workspace, so that channel is not represented as configured. Do not invent or use a guessed project email address.

Until a private channel is published, you can request private contact with the actual maintainer without disclosing exploit details, credentials or user data in a public issue. A useful report includes affected version/configuration, reproduction, impact and the smallest safe test case. Never exercise a report against a production cluster without its owner's authorization.

Protocol/parser/authentication vulnerabilities, cross identity leakage, unsafe cancellation, transaction replay, routing errors, unfenced promotion and dependency issues are in scope for security triage. Availability reports should identify the configured resource limits and workload. A correctness issue that can corrupt or expose data is security relevant even if memory safety is intact.

Release maintainers should coordinate remediation, regression evidence, advisories and supported patch releases. No automatic scan or compliance certification has been completed by writing this policy.
