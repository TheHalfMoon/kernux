# ChatGPT Secure MCP Bridge — Research Source Ledger

## Status

- Research snapshot date: 2026-09-23.
- Scope: evidence for a private ChatGPT-to-PC bridge backed by Kernux.
- This ledger is planning evidence only. It does not authorize source import.
- Any source adaptation requires a fresh immutable revision check, license/notice inspection, source-to-destination mapping, characterization tests, and the normal Kernux provenance workflow.

## Architecture conclusion supported by this research

The bridge should not become a second privileged runtime. It should be a thin OpenAI-facing MCP adapter that terminates into the existing Kernux Runtime Protocol / authenticated local IPC, with `kernuxd` retaining authorization, approval, workspace isolation, secret brokering, audit/evidence, and host capability ownership.

OpenAI Secure MCP Tunnel solves private reachability. It is not the local authorization boundary.

## Primary source ledger

| Source | Pinned revision | Observed license | Role | Planned disposition |
| --- | --- | --- | --- | --- |
| `openai/tunnel-client` | `cce7a8226654c432ce53c7e05e17a9825a34b6e3` | Apache-2.0 | Official Secure MCP Tunnel client | External dependency / transport reference; do not fork unless required |
| `gunwoo55/unlimited-agent` | `44c350c7c94963006c6f9d48d8e730e8626f9f7b` | MIT | Windows ChatGPT + Secure MCP Tunnel reference | Reference-first; selectively port credential isolation, installer, approval, and tunnel lifecycle lessons |
| `yuga-hashimoto/localant` | `217b53bc80ea6c98847f0fed162408b03ebb3a9b` | MIT | Permissioned local MCP gateway/control-plane reference | Reference-first; adapt catalog/risk/approval/audit concepts only behind Kernux contracts |
| `opensymph/open-computer-use` | `5b433b98019c18201a15d11e8c3cb0010879a3d8` | MIT | Accessibility-first native computer-use donor/reference | Governed separately by draft PR #200; no raw donor MCP authority |
| `microsoft/playwright-mcp` | `f1257a5a67aff872f947fae274759f7d54853862` | Apache-2.0 | Structured browser-control reference | Reference for snapshot/ref semantics and MCP surface; Kernux BrowserCapability should use Playwright-class APIs behind policy |
| `trycua/cua` | `8446459f93d0f7ebaa1b233bbf066a4b0f78931e` | MIT | Background computer-use / driver reference | Reference-only until a bounded provider decision exists |
| `opensandbox-group/OpenSandbox` | `2dd6e027bc3ee5b4f906c162d611ce9aa684aab7` | Apache-2.0 | Sandbox/egress/credential-broker reference | Architecture reference only; do not import its Kubernetes/control-plane architecture into the local bridge |
| `wonderwhy-er/DesktopCommanderMCP` | `2434718a0a8993d882cb05dc648c669f09bb4399` | MIT | Behavior baseline and existing Kernux donor | Preserve useful file/process/document behavior behind Kernux authority; do not inherit its guardrail-as-sandbox model |
| `browser-use/browser-use` | `d8110c5ff87ccba887aaa726cdb780f2f84bef8d` | MIT | Browser-agent reference | Reference-only for browser-agent workflow/evaluation ideas |
| `browserbase/stagehand` | `2c098f44857665045dbed31168ac36af920774d9` | MIT | Hybrid deterministic/agent browser reference | Reference-only for structured-first/self-healing browser patterns |
| `modelcontextprotocol/servers` | `f46d9578190b476b3501923ea8977d899e8db2cb` | Repository metadata: NOASSERTION | MCP server ecosystem reference | Reference-only; inspect per-package licensing before any reuse |
| `modelcontextprotocol/typescript-sdk` | `7f7a94c22017e121a960e071bb50ec75e34450bd` | Repository metadata: NOASSERTION | MCP TypeScript implementation reference | Dependency candidate only after package-level license/version review |

## Official OpenAI findings

Current OpenAI documentation establishes these constraints:

1. Secure MCP Tunnel is outbound-only. The local host initiates HTTPS to OpenAI and the private MCP server does not need a public listener, router forwarding, or inbound firewall rule.
2. `tunnel-client` can forward to a local MCP server over stdio or HTTP.
3. Runtime operation requires a `tunnel_id` and a runtime API key. Running `tunnel-client` or selecting a tunnel uses the Tunnels Read + Use permission; create/edit uses Tunnels Read + Manage.
4. The tunnel is transport, not application authorization. The local MCP server still owns validation/authorization for every tool call.
5. OpenAI MCP tool annotations inform host confirmation/safety behavior but do not replace server-side authorization.
6. Full ChatGPT MCP write/modify support is currently plan-dependent and beta. The bridge architecture must therefore keep a read-only mode and must not couple correctness to a particular ChatGPT confirmation UX.
7. Agent mode does not currently use custom apps, and deep research is read/fetch only. Product documentation must distinguish those surfaces from ordinary chats using the private app.

## Unlimited Agent findings

High-value mechanisms observed at the pinned revision:

- outbound Secure MCP Tunnel topology with a stdio MCP server;
- dedicated restricted tunnel credential;
- Windows DPAPI protection for the tunnel credential;
- child-process environment sanitization;
- centralized policy wrapper around tool registration;
- local approval flow;
- Windows UI and Chrome primitives;
- installer/status/settings lifecycle;
- audit logging and bounded diagnostics.

Do not copy its security boundary wholesale. Kernux already has stronger typed Grant/resource/runtime contracts, durable event/evidence authority, explicit egress classes, and a Rust privileged core.

The self-approval invariant must be stronger than serialized tool calls alone. Kernux must deny the approval surface as an automation target and suspend the agent's interactive input lease while approval is pending.

## LocalAnt findings

High-value concepts:

- capability catalog and discovery;
- risk classification;
- approval queue;
- audit dashboard;
- shell/filesystem/Git/browser/custom-skill/downstream-MCP aggregation;
- permission wrapping.

Do not copy permissive policy defaults. The ChatGPT bridge inherits Kernux DEFAULT_DENY / scoped-Grant behavior.

## Open Computer Use and Cua findings

Both strengthen the structured-before-visual direction:

- Windows UI Automation / accessibility structures can provide semantic state and actions;
- background/window-targeted interaction may avoid stealing physical mouse/keyboard control;
- screenshots and coordinate input should remain fallback mechanisms;
- target identity and stale-state detection must be explicit.

PR #200 owns the detailed Open Computer Use integration plan. This bridge consumes the future qualified Kernux ComputerUse contract rather than embedding a second desktop-control stack.

## Playwright findings

Use browser DOM/accessibility state before native desktop or vision fallback. The bridge should never expose a raw browser super-tool that bypasses Kernux origin/profile/workspace policy.

Required browser invariants:

- isolated automation profile by default;
- authenticated personal profile reuse is explicit;
- expected origin is bound to mutating operations;
- navigation/origin drift invalidates stale action authority;
- page content remains untrusted provenance and cannot expand capabilities;
- upload/download roots are workspace scoped;
- clipboard is a separate capability.

## Desktop Commander baseline and security lesson

Desktop Commander remains useful as a functional baseline for files, terminal/processes, repository work, documents, and system information. Its own published security model states that allowed directories, command blocklists, and symlink protections are guardrails rather than a sandbox, and that arbitrary terminal execution can escape those advisory restrictions.

Kernux therefore must not model shell authority as a side effect of filesystem access. Process execution is a separate capability with explicit cwd/environment/network/system-mutation semantics.

## Windows platform sources

Use Microsoft-supported primitives as the native baseline:

- UI Automation for structured UI discovery and programmatic manipulation;
- handle-resolved paths (for example, final-path-by-handle semantics) as part of Windows path authorization;
- Job Objects for supervised process-tree ownership and kill-on-close behavior where appropriate;
- DPAPI / approved Windows credential facilities for local secret-at-rest protection.

PowerShell language restrictions can be defense-in-depth but are not the primary authority boundary.

## Source-reuse gate

Before any copied or recognizably adapted code lands:

```text
PIN_EXACT_REVISION
-> INSPECT_LICENSE_AND_NOTICES
-> IDENTIFY_EXACT_SOURCE_PATHS
-> RECORD_PERMISSION_BASIS
-> CHARACTERIZE_BEHAVIOR
-> DEFINE_KERNUX_OWNED_CONTRACT
-> WRITE_NEGATIVE_AND_ADVERSARIAL_TESTS
-> IMPORT_BOUNDED_DIFF
-> RUN_JEV_REVIEW
-> RUN_ALIBABA_OPEN_CODE_REVIEW
-> RUN_DIFFCIPLINE_EXACT_DIFF EVIDENCE
-> RECORD_PROVENANCE
```

No source in this ledger is permission to wholesale-copy a repository.
