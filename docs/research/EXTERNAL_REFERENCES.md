# External References

This file separates **founding code donors** from standards and projects that are currently **reference-only** unless a future provenance record authorizes source reuse.

## Founding donors

See `docs/canonical/DONOR_PROVENANCE.md` for exact planning revisions and import rules.

- `https://github.com/stablyai/orca`
- `https://github.com/tinyfish-io/agentql` plus founder-authorized TinyFish source
- `https://github.com/wonderwhy-er/DesktopCommanderMCP`

## Delivery methodology

- `https://github.com/TheHalfMoon/SpecGrain` — bounded specs/Grains, WorkPackets, evidence discipline.
- `https://github.com/TheHalfMoon/Diffcipline` — proof-before-done, exact-diff/risk verification.

## Agent interoperability

### Agent Client Protocol (ACP)

- `https://agentclientprotocol.com/`
- `https://zed.dev/acp`
- `https://github.com/agentclientprotocol/`

Why relevant: standardized editor/client ↔ coding-agent communication. Kernux should implement stable ACP v1 before considering experimental protocol generations.

### Agent2Agent (A2A)

- `https://a2a-protocol.org/`

Why relevant: open protocol for capability discovery and collaboration between independent/opaque agents. Current planning targets A2A 1.0 semantics through an adapter rather than adopting A2A as Kernux's internal data model.

## Tool and context interoperability

### Model Context Protocol (MCP)

- `https://modelcontextprotocol.io/`
- `https://blog.modelcontextprotocol.io/posts/2026-07-28/`

Planning baseline: final `2026-07-28` generation, including stateless protocol core, extension framework, Tasks extension, cacheable discovery/list results, and authorization hardening.

Kernux should track MCP's published deprecation window and maintain version fixtures for any older protocol generation it intentionally supports.

## Agentic web

### WebMCP

- `https://developer.chrome.com/docs/ai/webmcp`
- `https://developer.chrome.com/docs/agents/security`

Why relevant: proposed web standard that lets pages expose structured tools to browser agents. It is still an evolving/experimental platform surface, so support must remain negotiated and never become an assumption that all sites expose trusted tools.

Security lesson: WebMCP manifests and tool outputs can themselves be malicious or prompt-injected; page-exposed tools are not authorization authority.

### Chrome DevTools Protocol

- `https://chromedevtools.github.io/devtools-protocol/`

Why relevant: low-level Chromium control/inspection boundary.

### Playwright

- `https://github.com/microsoft/playwright`

Why relevant: deterministic browser automation, contexts, downloads/uploads, tracing, cross-browser testing patterns.

## Terminal / editor / desktop references

### xterm.js

- `https://github.com/xtermjs/xterm.js`

Why relevant: terminal rendering and web/Electron terminal ecosystem.

### Ghostty

- `https://github.com/ghostty-org/ghostty`

Why relevant: modern terminal performance/rendering/reference architecture. Reference-only unless separately authorized/licensed for a particular reuse.

### Zed

- `https://github.com/zed-industries/zed`

Why relevant: high-performance editor UX and ACP origin/reference implementation.

## Isolation/runtime references

### OCI specifications

- `https://opencontainers.org/`

Why relevant: image/runtime artifact conventions for container-backed execution.

### OpenSandbox

- `https://github.com/opensandbox-group/OpenSandbox`

Why relevant: sandbox orchestration and isolated agent execution research/reference.

### Docker

- `https://docs.docker.com/`

Why relevant: practical local container runtime for a first isolation mode.

Kernux must not equate containers with universal strong isolation. VM/microVM runtimes remain a separate trust mode.

## Observability and supply chain

### OpenTelemetry

- `https://opentelemetry.io/`

Why relevant: common trace/metric/log concepts for runs, agents, runtimes, capabilities, and verification.

### Sigstore

- `https://www.sigstore.dev/`

Why relevant: release/artifact signing and provenance patterns.

### SLSA

- `https://slsa.dev/`

Why relevant: build provenance and supply-chain hardening framework.

### SPDX / CycloneDX

- `https://spdx.dev/`
- `https://cyclonedx.org/`

Why relevant: SBOM/license inventory formats.

## Research questions to keep active

These are deliberately not architecture decisions until implementation evidence answers them:

1. Which sandbox backend provides the best cross-platform default without bloating local installation?
2. How much of Orca's Electron runtime should be imported versus rebuilt around `kernuxd` contracts?
3. Which terminal rendering/runtime path best balances donor reuse, performance, licensing, and maintenance?
4. What is the narrowest KRP schema that supports local + SSH + remote-device runtimes without duplicating MCP/A2A?
5. Can the same browser evidence model represent WebMCP, CDP, semantic extraction, and vision actions cleanly?
6. What permission granularity minimizes prompt fatigue while preserving meaningful least privilege?
7. What state belongs in durable events versus mutable projections?
8. Which ACP agents provide reliable resume/status/usage metadata versus requiring PTY fallback?
9. What local secret-broker primitives are portable enough across macOS/Windows/Linux?
10. What remote relay architecture preserves local-first behavior and supports NAT/mobile access without becoming a mandatory hosted dependency?

## Reference policy

- A reference is not automatically an authorized code donor.
- Before copying code, create/update provenance and review the exact source license/terms.
- Prefer learning architectural lessons over accumulating dependencies.
- Pin protocol/spec versions in implementation evidence when behavior depends on them.
- Re-verify fast-moving standards before implementing a phase that depends on them.