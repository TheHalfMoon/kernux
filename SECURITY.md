# Security Policy

Kernux treats security boundaries as product contracts, especially around local privilege, secrets, browser/tool content, plugins, remote runtimes, updates, destructive operations, and provenance.

## Supported state

Kernux is pre-release. There is no stable supported release line yet. Security fixes are evaluated against current canonical `main` and any explicitly qualified release candidate; this does not create a response-time or support-lifetime guarantee.

## Reporting a vulnerability

Do **not** publish vulnerability details, exploit steps, credentials, tokens, personal data, PHI, or other sensitive material in a public issue or pull request.

1. Check the repository Security page. If GitHub shows **Report a vulnerability**, use that private reporting flow.
2. If no private reporting flow is available, open a minimal public issue asking the maintainers for a private security contact channel. Include no vulnerability details in that issue.
3. Once a private channel is established, provide the smallest evidence needed to reproduce and assess the problem.

A useful private report includes the affected commit/version, affected platform/runtime, impact, preconditions, reproduction steps or proof of concept, and any proposed mitigation. Redact unrelated secrets and sensitive data.
## Handling and disclosure

Maintainers may reproduce the report, request clarification, create a private fix branch/advisory, and coordinate disclosure when appropriate. No bounty, confidentiality guarantee, response SLA, or embargo duration is implied unless explicitly agreed for a specific report.

Do not test against systems, accounts, or data you are not authorized to use. Use synthetic or self-owned data wherever possible.

A security fix is not complete merely because a patch exists. Applicable tests, adversarial checks, provenance, exact-head qualification, and post-merge/release gates still apply under canonical repository governance.

For non-security bugs or support questions, use the channels described in `SUPPORT.md`.
