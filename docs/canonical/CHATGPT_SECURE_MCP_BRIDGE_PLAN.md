# Kernux ChatGPT Secure MCP Bridge Plan

## Status

- Planning only.
- Research snapshot: 2026-09-23.
- Canonical base observed before this planning branch: `main@a55832a5e5f6b2cf2abd6ac72c756c8ab2a01235`.
- This plan does not advance `specs/CURRENT.md`, change the P02 frontier, import donor code, or authorize a merge.
- Detailed source ledger: `docs/research/CHATGPT_SECURE_MCP_BRIDGE_SOURCE_LEDGER.md`.
- Open Computer Use integration remains separately owned by draft PR #200.
- Working component name: **Kernux ChatGPT Bridge**.
- Proposed binary/package name: `kernux-mcp-bridge`.

## 1. Architecture decision

### Decision

Build a **thin ChatGPT/OpenAI MCP bridge backed by the existing Kernux privileged runtime**.

Do not create a second standalone privileged gateway and do not make MCP itself the host-security boundary.

```text
ChatGPT / supported OpenAI surface
  |
  v
OpenAI Secure MCP Tunnel
  |
  v
openai/tunnel-client
  |  local stdio or loopback-only transport
  v
kernux-mcp-bridge                    UNPRIVILEGED EDGE
  |  authenticated local IPC / Kernux Runtime Protocol
  v
kernuxd                              PRIVILEGED AUTHORITY
  |
  +--> Grant / policy engine
  +--> workspace + resource identity
  +--> local approval broker
  +--> secret broker
  +--> audit / event / evidence authority
  +--> egress policy
  |
  +--> Filesystem provider
  +--> Process / PTY provider
  +--> Git provider
  +--> System provider
  +--> ComputerUse provider
  +--> Browser provider
  +--> Screenshot / input fallback providers
```

### Why this is the preferred option

Kernux already owns the required security primitives: scoped Grants, runtime/resource identity, secret brokering, policy, durable events/evidence, egress classes, and a Rust privileged daemon. Duplicating those inside a new MCP project would create two policy engines, two audit systems, two secret paths, and two definitions of workspace scope.

The bridge may later be distributed as a small independently installable component, but its authority remains Kernux-owned.

### Explicit rejection

Do not build:

```text
ChatGPT -> tunnel -> Node MCP server -> arbitrary shell / UI authority
```

That recreates the class of design this project is intended to replace.

## 2. Transport is not authorization

OpenAI Secure MCP Tunnel is the default remote transport because it preserves an outbound-only network posture.

Required topology:

```text
NO public inbound listener
NO router port forwarding
NO public relay
NO firewall hole

Windows PC
  -> outbound HTTPS
  -> OpenAI Secure MCP Tunnel
  -> supported OpenAI product
```

The tunnel credential authenticates `tunnel-client` to the OpenAI tunnel control plane. It does not authorize filesystem, process, browser, desktop, secret, or network actions on the PC.

Every external MCP request is re-authorized by `kernuxd`.

## 3. ChatGPT product compatibility boundary

The local architecture must remain correct even when ChatGPT product behavior changes.

Current constraints to design around:

- full MCP write/modify support is plan-dependent and beta;
- read/fetch-only operation must remain useful;
- ChatGPT confirmation is defense-in-depth, not local authority;
- custom-app tool snapshots may be frozen until an admin refreshes them;
- agent mode and deep research do not provide the same write-capable custom-app path.

Therefore:

- tool schemas are versioned;
- incompatible schema changes fail explicitly;
- the bridge provides read-only and mutation-capable profiles;
- local approval remains independent of ChatGPT confirmation;
- no security invariant depends on a specific ChatGPT UI.

## 4. Trust boundaries

| Zone | Trusted for | Not trusted for |
| --- | --- | --- |
| Human owner / organization policy | Granting authority | Runtime correctness |
| OpenAI product | Sending MCP calls under product policy | Local host authorization |
| Secure MCP Tunnel | Private transport | Host authorization |
| `tunnel-client` | Tunnel transport | Kernux policy decisions |
| `kernux-mcp-bridge` | Schema validation, request translation | Granting privilege |
| `kernuxd` | Local authority, policy, approvals, audit | External content truth |
| Capability provider | Executing one bounded authorized operation | Expanding its own authority |
| Web/document/repository content | Data | Authorization or instruction authority |
| Agent/model output | Proposal/request | Completion or permission proof |

## 5. MCP tool-surface rule

Do not mirror an internal API and do not expose one `run_anything` tool.

Tools are split when permission or consequence differs.

Representative external surface:

```text
system.describe
workspace.describe

files.list
files.read
files.search
files.write
files.edit
files.move
files.delete

process.start
process.status
process.read_output
process.cancel

shell.powershell
shell.session_start
shell.session_write

git.status
git.diff
git.log
git.run_declared_operation

apps.list
windows.list
windows.observe
windows.activate

browser.observe
browser.navigate
browser.act

screen.capture
computer.act
clipboard.read
clipboard.write
```

This is a product-facing vocabulary, not permission authority. Each tool compiles into one or more Kernux `CapabilityRequest` values.

MCP annotations such as read-only, destructive, open-world, and idempotency hints must be accurate but never substitute for local authorization.

## 6. Capability separation

Minimum capability families:

```text
files.observe
files.read
files.write
files.delete

process.observe
process.spawn
process.signal
pty.open
pty.write

git.observe
git.mutate

system.observe
app.launch
window.observe
window.activate

computer.observe
computer.semantic_act
computer.coordinate_act
computer.global_input

screen.capture
clipboard.read
clipboard.write

browser.observe
browser.navigate
browser.mutate
browser.download
browser.upload

network.direct
secret.use
secret.disclose

system.privileged
```

A Grant is scoped by subject, action, resource, runtime, constraints, expiry, and policy revision.

## 7. Permission profiles

Profiles compile to Grants; they are not alternate authorization logic.

| Profile | Behavior |
| --- | --- |
| `READ_ONLY` | Observation only inside selected workspaces. No writes, process starts, browser mutations, clipboard reads, or input. |
| `DEVELOPMENT` | Read/write and declared build/test/Git operations inside approved repositories. Raw shell, external network, destructive operations, secret disclosure, and desktop input remain gated. |
| `BALANCED` | Broader workspace/app control with local approval for material mutations and sensitive observation. |
| `FULL` | Owner-only broad host mode. Audit, protected targets, privilege boundaries, and explicit destructive/system controls remain active. |

`FULL` is never the default.

## 8. Workspace isolation

Every bridge session binds to a Kernux workspace identity.

Required binding:

```text
workspace_id
runtime_id
repository_identity
read_roots
write_roots
delete_roots
browser_context_id
network_policy_id
secret_policy_id
grant_revision
```

A call for workspace A cannot silently resolve a resource from workspace B.

Cross-workspace access requires an explicit new Grant and must be visible in approval/audit evidence.

## 9. Windows filesystem authorization

String prefix checks are forbidden as the only path boundary.

The Windows provider must account for:

- path normalization and case-insensitive identity;
- symlinks, junctions, mount points, and other reparse points;
- hard links and file identity aliases;
- UNC/network paths;
- device namespaces;
- short/alternate path representations;
- `..` and environment expansion;
- alternate data streams;
- WSL path translation;
- removable media;
- time-of-check/time-of-use races.

Preferred authorization pattern for existing paths:

```text
parse requested resource
-> reject unsupported namespace
-> open target/ancestor with safe flags
-> resolve final path from handle
-> capture volume + file identity
-> evaluate root/resource policy
-> perform operation through the validated identity
```

For creates, authorize the nearest existing parent identity, reject unexpected reparse traversal, create with bounded semantics, then revalidate the resulting identity before returning success.

Read, write, and delete roots may differ.

## 10. Process and shell model

Direct argv execution is preferred over shell interpolation.

Separate:

```text
declared project command
argv process spawn
PowerShell script
interactive PowerShell / shell
background job
long-running PTY
system mutation
privileged execution
```

Requirements:

- executable identity and argv recorded separately;
- cwd is a scoped resource;
- child environment is an explicit projection, not ambient inheritance;
- tunnel credentials and unrelated secrets are removed;
- process tree is supervised;
- Windows Job Objects are used where appropriate for owned process-tree lifecycle and kill-on-close behavior;
- output is bounded and streamable;
- process ID plus runtime-owned identity prevents PID-substitution assumptions;
- cancellation request is distinct from verified termination;
- ambiguous retry of side-effecting commands fails closed;
- shell command blocklists are not a security boundary.

PowerShell constrained modes may be used as defense-in-depth only.

## 11. Tunnel credential and secret design

Use a dedicated tunnel credential with only the minimum tunnel permissions supported.

Secret requirements:

- plaintext tunnel credentials never enter ordinary SQLite rows, logs, approval history, child environments, crash reports, or model context;
- secret-at-rest protection uses the Kernux secret broker and Windows-native protection such as DPAPI / approved credential facilities;
- `tunnel-client` receives the credential through a narrowly scoped launch/handoff path;
- the bridge process itself does not need the plaintext runtime key after tunnel-client starts;
- child process environment is allowlisted;
- rotation/revocation is independent of workspace data;
- known key/token patterns are redacted before audit persistence;
- `.env`, SSH keys, browser cookies, Git credentials, and cloud credentials are classified as sensitive resources;
- secret use and secret disclosure are separate capabilities.

## 12. Local approval: non-self-approvable by construction

The approval surface is independent of ChatGPT and must not be controllable by the agent.

Required invariants:

1. Approval is bound to an immutable request digest containing normalized target, action, argv/URL, workspace, runtime, data/secret references, policy revision, nonce, expiry, and consequence class.
2. Any target or argument drift invalidates approval.
3. The approval process/window is a protected resource denied to ComputerUse, Window, Browser, and coordinate-input providers.
4. While approval is pending, the agent's interactive input lease for that Windows session is suspended.
5. Background agent processes do not gain approval authority merely because they are still running.
6. One-time approval cannot be replayed after expiry, target change, or policy revision.
7. Highly destructive/privileged operations require fresh authority.
8. Denial is a typed result, not a generic failure.

Input-injection detection may be used as defense-in-depth, but it is not the sole invariant.

## 13. Desktop / computer-use hierarchy

Preferred order:

```text
typed application/tool API
> Windows UI Automation / accessibility semantic action
> bounded Win32/window-targeted action
> browser DOM when the target is browser content
> vision-assisted action
> coordinate input
> raw global keyboard/mouse
```

Windows UI Automation is the first native structured path.

Element references are session-local and observation-revision-bound. A stale or substituted target returns `STALE_UI_STATE` or `ELEMENT_NOT_FOUND`; it never silently clicks the old coordinate.

Every result records the actual mechanism:

```text
typed_api
uia_pattern
win32_targeted
browser_dom
vision
coordinate_input
global_input
```

Failure at a safer level never silently grants a stronger method.

PR #200 owns the detailed Open Computer Use provider planning. The ChatGPT bridge consumes the qualified Kernux ComputerUse abstraction only.

## 14. Human-input safety

The agent must not fight the human for the machine.

Required state:

```text
input_lease = none | background | foreground
human_override = available
emergency_stop = always_available
active_control_indicator = visible_when_foreground
lease_expiry = bounded
```

Human takeover invalidates stale UI observations and suspends agent input until a fresh observation and eligible Grant exist.

Background/window-targeted interaction is preferred when reliable.

## 15. Browser boundary

Browser control is separate from desktop control.

Preferred order:

```text
WebMCP / typed page capability
> Playwright / DOM / accessibility
> semantic browser abstraction
> qualified native ComputerUse
> vision
> coordinates
```

Default browser automation uses an isolated profile/context.

Personal authenticated profiles require explicit selection and project/session scope.

Mutation requests bind to expected origin. Unexpected navigation or origin drift fails with `BROWSER_ORIGIN_CHANGED` and requires re-observation/re-authorization.

Page content is hostile data. Prompt injection cannot create a new Grant.

Downloads, uploads, clipboard, cookies/storage, and network destinations are separately governed.

## 16. Network and egress

Shell authority does not imply network authority.

Supported policy shapes:

```text
NONE
DIRECT_DESTINATION
CONNECTED_ACCOUNT
EXTERNAL_MODEL
EXTERNAL_TOOL
REMOTE_RUNTIME
UPDATE
TELEMETRY
```

The bridge must preserve the P02 egress decision instead of creating a new network policy language.

Material outbound destinations are logged without leaking secret payloads.

## 17. Audit and evidence

Each material operation produces an append-oriented record with:

```text
timestamp
external_call_id
session_id
workspace_id
runtime_id
tool
capability_request_id
grant_id / policy_revision
risk_or_consequence_class
normalized_resource
approval_id / decision
actual_execution_method
result_class
duration
artifact/evidence references
redaction metadata
```

Never persist raw secrets by default.

Audit storage is bounded, exportable, rotation-aware, and integrity-addressable through the existing Kernux event/evidence model.

## 18. Typed failures

The MCP adapter preserves typed local failures.

Required baseline:

```text
PERMISSION_DENIED
LOCAL_APPROVAL_REQUIRED
LOCAL_APPROVAL_DENIED
PATH_OUTSIDE_SCOPE
RESOURCE_IDENTITY_CHANGED
SECRET_READ_BLOCKED
PROCESS_FAILED
PROCESS_TIMEOUT
PROCESS_CANCELLED
PROCESS_STATE_UNKNOWN
WINDOW_NOT_FOUND
ELEMENT_NOT_FOUND
ELEMENT_AMBIGUOUS
STALE_UI_STATE
SESSION_LOCKED
UAC_REQUIRED
MFA_REQUIRED
CAPTCHA_REQUIRED
NETWORK_DENIED
NETWORK_FAILURE
BROWSER_ORIGIN_CHANGED
HUMAN_TAKEOVER
CAPABILITY_UNAVAILABLE
PROTOCOL_VERSION_UNSUPPORTED
TUNNEL_UNAVAILABLE
```

Unknown state is not success.

## 19. Threat model

| Threat | Invariant/control | Required test/evidence | Residual risk |
| --- | --- | --- | --- |
| Malicious page asks agent to steal secrets | Web content is untrusted provenance; secret + egress Grants are separate | Injection corpus cannot expand Grant or destination | Approved user action can still intentionally disclose data |
| Malicious repository instruction | Repository text cannot authorize shell/network/secret use | Source-file injection fixtures | Human may deliberately approve risky work |
| Shell output instructs agent | Tool output is data, not authority | Output-injection fixtures | Model reasoning remains probabilistic |
| Project A reaches project B | Workspace/resource binding and separate roots | Cross-workspace denial fixtures | User can explicitly grant both |
| Junction/reparse escape | Handle/final-path identity authorization | Junction/symlink/mount adversarial suite | Compromised kernel/filesystem driver out of scope |
| Hard-link alias escape | File identity/volume checks and conservative write/delete policy | Cross-root hard-link fixtures | Filesystem features unsupported by policy are denied |
| Agent clicks approval | Protected approval surface + suspended input lease | Attempt via UIA, Win32, coordinates, global input | Fully compromised same-user OS is out of scope |
| Approval reused after drift | Request digest + nonce + expiry + policy revision | Modify target/argv after approval and prove denial | Human can approve a new request |
| Command semantics change | argv-first execution; raw shell separately gated | quoting/substitution/alternate-interpreter corpus | Approved arbitrary shell remains powerful |
| Environment leaks credential | allowlisted environment projection | child process asserts no tunnel/ambient secret vars | Child can access secrets explicitly granted to it |
| Screenshot exposes secret | capture classification + protected target/redaction + approval | secure-field/protected-window fixtures | Pixels can contain unknown sensitive data |
| Clipboard exposes password | clipboard.read separate and sensitive | denied/approved clipboard tests | User can explicitly approve disclosure |
| Personal browser leaks sessions | isolated profile default; explicit authenticated profile scope | cross-workspace cookie/storage isolation | Explicit profile reuse carries intended risk |
| Tunnel credential stolen | native protected storage + narrow handoff + no child inheritance | DPAPI roundtrip + log/env scans | Same-user host compromise can access user secrets |
| Audit leaks secrets | structured redaction before persistence | seeded secret corpus absent from logs/evidence | Unknown secret formats require conservative handling |
| Updater compromised | signed/provenanced artifacts, checksums, rollback | release provenance verification | Trust in signing keys/build infrastructure remains |
| Duplicate action after reconnect | operation IDs + idempotency/ambiguity rules | disconnect/retry side-effect fixtures | Non-idempotent external systems may require manual reconciliation |
| Locked/UAC/MFA/CAPTCHA surface | explicit typed failure; no bypass | native Windows fixtures/manual qualification | Workflow requires human completion |

## 20. Capability comparison

This table is factual/descriptive; it is not a score.

| Dimension | Desktop Commander | Unlimited Agent | LocalAnt | Open Computer Use | Kernux ChatGPT Bridge target |
| --- | --- | --- | --- | --- | --- |
| Files | Yes | Yes | Yes | No / not primary | Yes, Kernux-scoped |
| Shell/processes | Yes | Yes | Yes | No / not primary | Yes, separated capabilities |
| Git/dev workflows | Via shell/files | Via shell/files | Documented Git/coding tools | No / not primary | Yes |
| Browser | Limited/general through shell/docs; not core structured path | Chrome control | Documented browser capability | Native/browser UI automation | Playwright/DOM first |
| Native desktop | Not primary | Screenshot/input/window tools | Not primary control focus | Core capability | Qualified ComputerUse provider |
| Accessibility-first | No | Screenshot/input oriented | Not primary | Yes | Required |
| Local approval | Guardrails, not sandbox authority | Yes | Yes | Not policy kernel | Kernux approval broker |
| Policy granularity | Advisory guardrails | Central tool/path policy | Risk/security modes | Capability execution, not full gateway policy | Scoped Grants |
| Workspace isolation | Filesystem setting does not constrain arbitrary shell | Allowed roots | Gateway policy concepts | Not project authority | Required identity binding |
| Secret boundary | Not primary security boundary | DPAPI tunnel credential + env sanitization | Gateway secret/policy concepts | Not gateway focus | Kernux secret broker |
| Audit | Local tool history | Audit log | Audit/dashboard | Not gateway focus | Durable Kernux events/evidence |
| Network control | Shell can be broad | HTTP tool + policy/approval | Gateway controls | Not primary | P02 egress classes |
| Secure MCP Tunnel | Not canonical baseline | Yes | Not primary | No | Official transport |
| Extensibility | MCP tools | MCP tools | Gateway/catalog/skills | Computer-use API/MCP | Provider-neutral Kernux contracts |
| Privileged boundary | Connected AI is trusted; guardrails are not sandbox | Node policy + approvals | Gateway runtime | Native automation service | Rust `kernuxd` |

## 21. Service lifecycle

Required owner operations:

```text
install
pair_tunnel
start
stop
restart
status
rotate_tunnel_credential
revoke_chatgpt_access
delete_tunnel_binding
repair
update
rollback
uninstall
```

Visible local status must expose:

```text
CONNECTED / DISCONNECTED
ACTIVE_SESSION
PENDING_APPROVAL
CURRENT_WORKSPACE
CURRENT_PROFILE
ACTIVE_INPUT_LEASE
LAST_ACTION
TUNNEL_HEALTH
```

Immediate revocation must stop new bridge requests without requiring ChatGPT cooperation.

## 22. First usable release

The first safe usable release is intentionally narrower than full desktop automation.

Required capabilities:

```text
system.describe
workspace.describe
files.list/read/search/write/edit/move/delete
process.start/status/read_output/cancel
gated PowerShell
Git status/diff/basic operations
local approval
audit/evidence
Secure MCP Tunnel
basic app/window inspection
revocation/status
```

Not required for the first release:

```text
vision-first automation
global keyboard/mouse
personal browser-profile attachment
arbitrary network access
privileged Windows mutation
service/registry mutation
```

## 23. Delivery sequence

The bridge is a cross-cutting consumer of existing Kernux phases. It must not bypass the active dependency frontier.

| Slice | Outcome | Dependencies | Acceptance |
| --- | --- | --- | --- |
| `CGB-000` | Research/source ledger + architecture freeze | Current canonical docs | Exact pins, threat model, no canonical task-state change |
| `CGB-001` | Bridge protocol contract | P01 contracts + P02 IPC | Versioned MCP-to-Capability mapping; no privilege in adapter |
| `CGB-002` | Tunnel lifecycle + secret handoff | P02 secret broker + egress | Restricted credential protected; bridge/children cannot read it |
| `CGB-003` | Read-only system/workspace bridge | CGB-001/002 | Real tunnel E2E returns system/workspace state without mutation |
| `CGB-004` | Files read/search | P04 filesystem | Path escape corpus fails closed |
| `CGB-005` | Files mutation + approval | P04 filesystem + approval UI | Request-bound approval; atomic write semantics; audit evidence |
| `CGB-006` | Process/PowerShell lifecycle | P04 process/PTY | argv-first path; env isolation; Job Object qualification; typed cancellation |
| `CGB-007` | Git development workflow | P04 Git | status/diff/test workflow scoped to workspace |
| `CGB-008` | Basic Windows app/window observation | P04 ComputerUse baseline | UIA/Win32 read-only observation; stale identity tests |
| `CGB-009` | Installer/status/revocation | CGB-002..008 | Install/pair/connect/revoke/uninstall journey |
| `CGB-010` | First-release E2E security qualification | All first-release slices | ChatGPT -> tunnel -> bridge -> Kernux -> real repo workflow, negative/adversarial suite |
| `CGB-011` | Structured browser capability | P06 | Isolated Playwright context, origin drift, injection, download/upload scope |
| `CGB-012` | Accessibility-first desktop actions | qualified P04 ComputerUse | Semantic/background path before coordinate/global input |
| `CGB-013` | Visual/coordinate fallback | P18/P06 policy | Explicit method ceiling + input lease + evidence |
| `CGB-014` | Clipboard and sensitive observation | policy + ComputerUse | Separate Grants, protected-content fixtures |
| `CGB-015` | Release hardening | P15 | Signed/provenanced release, upgrade/rollback, Windows qualification |

Each slice becomes a SpecGrain Grain with bounded paths, explicit dependencies, negative cases, exact acceptance evidence, and Diffcipline verification.

## 24. Required test layers

### Unit

Policy compilation, capability mapping, path identity, risk/consequence class, redaction, operation IDs, tool schema generation, audit record construction.

### Integration

Real Windows filesystem, process tree, PowerShell, Git, local IPC, secret broker, tunnel-client lifecycle, UIA observation, Playwright, clipboard when enabled.

### Security

Traversal/reparse/hard-link, device/UNC/ADS, shell injection, environment leakage, approval self-click, stale approval, stale UI refs, cross-workspace access, browser-session leakage, secret exfiltration, tunnel-key leakage, prompt injection, output injection, duplicate action after reconnect.

### Failure injection

Tunnel disconnect, PC sleep, process crash, bridge crash, daemon restart, Chrome crash, locked desktop, revoked permission, app closes during action, partial file write, approval denial/expiry.

### E2E

```text
ChatGPT
-> Secure MCP Tunnel
-> kernux-mcp-bridge
-> kernuxd
-> inspect repository
-> edit bounded file
-> run tests
-> inspect failure
-> repair
-> rerun
-> inspect Git diff
-> produce local audit/evidence
```

and later:

```text
observe app
-> select semantic element
-> act
-> verify fresh observation
-> capture approved evidence
```

## 25. Independent review and evidence

Every high-risk implementation slice requires:

- exact-head CI;
- native Windows qualification for Windows-specific behavior;
- applicable Linux/macOS compatibility checks without pretending parity;
- SpecGrain readiness and WorkPacket evidence;
- Diffcipline exact-diff verification;
- genuine Jev review;
- Alibaba Open Code Review accounting;
- semantic security review;
- zero unresolved blocking threads;
- post-merge verification before canonical effectiveness is claimed.

No review tool may replace deterministic tests or policy evidence.

## 26. Installer and UX target

The intended owner experience:

```text
Install Kernux / bridge
-> create or select Secure MCP Tunnel
-> enter restricted tunnel credential into local protected setup
-> choose workspaces/folders
-> choose permission profile
-> connect
-> create/select the private ChatGPT app using the tunnel
-> verify read-only system call
-> optionally enable development mutations
```

Do not require hand-editing multiple configuration files.

## 27. Zero-cost-to-founder rule

The base local bridge must require no Kernux-hosted relay, database, browser farm, observability service, or managed execution infrastructure.

User hardware and user-owned OpenAI/product access provide runtime resources.

Optional hosted services must never become a hidden dependency for the local path.

## 28. Rollback and recovery

Every release must support:

- stop bridge without stopping unrelated Kernux work;
- revoke tunnel credential;
- disable the external MCP surface while retaining local Kernux data;
- roll back bridge version within the supported protocol window;
- reject protocol mismatch rather than silently downgrade security;
- restore config without restoring revoked secrets;
- uninstall bridge/tunnel integration without deleting user project data.

## 29. Implementation-readiness gate

The first implementation Grain is not ready until all of these are true:

1. `kernuxd` remains the only privileged policy authority.
2. MCP tool -> CapabilityRequest mapping is frozen and versioned.
3. Tunnel-client credential ownership/handoff is specified.
4. Child environment projection excludes tunnel and unrelated secrets.
5. Read/write/delete workspace resources are separately scoped.
6. Windows path identity and reparse/hard-link policy are explicit.
7. Process argv/cwd/env/lifecycle semantics are explicit.
8. Approval cannot be actuated through any agent input capability.
9. Approval reuse/drift/expiry rules are explicit.
10. Browser and desktop authorities remain separate.
11. UIA/DOM structured paths precede vision/coordinates.
12. Global input is separately gated.
13. Personal browser-profile reuse is explicit.
14. P02 egress remains authoritative.
15. Audit/redaction schema is mapped to Kernux events/evidence.
16. Typed failures survive the MCP adapter.
17. Immediate revocation path exists.
18. Threat fixtures cover the listed adversarial cases.
19. Source provenance is pinned.
20. No paid infrastructure is required for the local path.
21. ChatGPT plan/product limitations are documented without weakening local security.
22. PR #200 ComputerUse work, if merged, is consumed through its Kernux contract rather than duplicated.

## 30. Final product rule

The bridge is complete only when a real supported OpenAI surface can request:

```text
Open my Kernux repository, inspect the latest state, continue a bounded implementation,
run the tests, inspect failures, repair them, verify the diff, and report evidence.
```

while the local runtime independently enforces:

```text
least privilege
workspace isolation
local approval
secret safety
network/egress policy
auditability
revocation
human override
typed failure semantics
no ambient shell authority
```

The bridge is not considered successful merely because ChatGPT can execute commands. It is successful when useful computer work is possible without moving host authority into the model or MCP edge.
