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

## Current agent-work and orchestration references

These sources are **reference-only** unless a future provenance record separately authorizes source reuse. They are tracked for product expectations, architecture lessons and comparative evaluation design.

### OpenAI ChatGPT Work

- `https://openai.com/index/chatgpt-for-your-most-ambitious-work/`
- `https://help.openai.com/en/articles/20001280-using-cloud-browser-in-chatgpt`
- `https://help.openai.com/en/articles/20001278-creating-and-editing-documents-spreadsheets-and-presentations-with-chatgpt-work`

Observed design lessons as of September 2026:

- long-running work across connected apps/files;
- separate cloud-browser execution and local desktop browser paths;
- finished editable documents, spreadsheets, presentations and sites/apps;
- approval/steering during long-running work;
- integration/plugin discovery as part of the work surface.

Kernux should learn from the expectation of finished work without making one hosted model/cloud/browser mandatory.

### Microsoft Copilot Studio computer use

- `https://learn.microsoft.com/en-us/microsoft-copilot-studio/computer-use`
- `https://learn.microsoft.com/en-us/microsoft-copilot-studio/configure-where-computer-use-runs`
- `https://learn.microsoft.com/en-us/microsoft-copilot-studio/monitor-computer-use`

Observed design lessons:

- visual computer use remains necessary when structured APIs do not exist;
- hosted browser/cloud-PC and bring-your-own-machine paths can coexist;
- step-level monitoring, screenshots and audit improve supportability;
- metered computer use reinforces the need for explicit cost ownership and local/BYOC alternatives.

### Composio

- `https://docs.composio.dev/docs`
- `https://docs.composio.dev/docs/composio-connect`

Observed design lessons:

- very broad app coverage requires managed auth/account lifecycle;
- tool discovery/router patterns are preferable to exposing thousands of tools to every agent context;
- MCP can be a transport for integration breadth while Kernux still owns policy and evidence.

### n8n

- `https://n8n.io/ai-agents/`

Observed design lessons:

- traditional deterministic workflow control and agentic behavior need to coexist;
- self-hosting remains valuable;
- human-in-the-loop and multi-agent flows should remain explicit rather than hidden in prompt conventions.

### Temporal

- `https://temporal.io/`

Observed design lesson: long-running stateful agent systems benefit from durable execution semantics. Kernux should preserve its own operation identity, event/evidence and recovery model rather than importing a workflow engine as public architecture.

### LangGraph

- `https://www.langchain.com/langgraph`

Observed design lessons:

- human-in-the-loop controls, persisted state/memory and customizable single/multi-agent control flows are baseline orchestration expectations;
- Kernux should expose these outcomes through its provider-neutral Task/WorkUnit/Run model.

### OpenHands

- `https://www.openhands.dev/`
- `https://github.com/OpenHands/docs`

Observed design lessons:

- model-agnostic deployment and self-hosting matter for agent platforms;
- executing generated/untrusted code needs an isolated runtime rather than relying on prompt-level safety.

### Browser Use / BrowserCode

- `https://github.com/browser-use/browser-use`
- `https://github.com/browser-use/browsercode`

Observed design lessons:

- browser agents benefit from reusable deterministic code/scripts after exploratory action;
- hosted and local/headless browser execution can share higher-level contracts;
- browser-native coding/control remains a fast-moving comparative area for P06/P20 evaluation.

### Genspark AI Slides

- `https://www.genspark.ai/helpcenter/ai-slides`

Observed design lesson: presentation/artifact generation is moving toward an editable workspace with templates, charts, notes, data computation and conversational targeted edits. Kernux P16 should treat structured artifact fidelity as a first-class engineering problem rather than a rendering demo.

### Manus

- `https://help.manus.im/en/articles/11960169-what-is-wide-research`
- `https://help.manus.im/en/articles/14178443-what-is-the-my-computer-feature-capable-of`

Observed design lessons:

- wide/parallel research is a user-visible fleet pattern;
- local-machine command execution and cloud agent work are converging;
- Kernux must bind fan-out, local execution and handoff to explicit evidence/budget/policy semantics.

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