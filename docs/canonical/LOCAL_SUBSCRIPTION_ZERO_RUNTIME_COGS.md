# Local Subscription and Zero Runtime COGS

## 1. Business invariant

Kernux is sold as a **subscription to locally executed software**, not as a bundled cloud-compute service.

The target economic invariant is:

> **Recurring subscription revenue must not create recurring founder-funded runtime cost.**

For ordinary paid users, Kernux must not require the project owner to pay for:

- model inference;
- embeddings or reranking;
- browser-agent minutes;
- search/fetch APIs;
- vector databases;
- object storage;
- user files or memory storage;
- remote desktops;
- containers/VMs;
- GPU/CPU runtime;
- agent execution;
- email/SMS/WhatsApp delivery;
- connector/API usage;
- relay bandwidth;
- background automation compute.

The intended runtime unit economics are therefore:

```text
subscription revenue
        -
payment / merchant fees
        -
taxes / unavoidable commerce costs
        -
small fixed business/distribution costs
        =
gross margin

runtime variable COGS per user/task ~= 0
```

"Zero cost" in this document means **zero Kernux-funded variable runtime COGS**, not a claim that payment processing, taxation, code signing, domains, legal/compliance, customer support, or every possible business expense is literally free.

## 2. User-compute ownership

The default execution model is:

```text
User buys Kernux subscription
        |
        v
Installs official desktop application
        |
        +--> CPU/GPU/RAM -> user's device
        +--> files/storage -> user's device
        +--> memory/indexes -> user's device
        +--> browser -> user's device
        +--> models -> local model OR user's provider account
        +--> APIs/tools -> user's account / BYOK / existing subscription
        +--> remote compute -> user's enrolled machine/server
        +--> automation -> user's device/runtime
```

Kernux coordinates and governs the work. It does not proxy ordinary user workloads through founder-funded infrastructure.

## 3. Cost-owner rule

Every metered capability has exactly one explicit cost owner.

Allowed cost owners:

- `LOCAL_DEVICE` — user's own hardware;
- `USER_BYOK` — user's API key/account;
- `USER_SUBSCRIPTION` — provider-native subscription already owned by user;
- `USER_REMOTE_RUNTIME` — user's own SSH/server/device;
- `ORGANIZATION` — employer/team-owned account or infrastructure;
- `SEPARATELY_PRICED_MANAGED_SERVICE` — optional future Kernux-managed service with explicit price and positive unit economics.

Forbidden for core paid-product operation:

- hidden founder API keys;
- founder-funded shared inference;
- founder-funded browser/search pool;
- founder-funded storage/sync required for correctness;
- unlimited managed usage bundled into a fixed local-software subscription;
- promotional credits used as the only proof that a feature works.

If no valid cost owner exists, the capability fails closed or offers a local/BYOK alternative.

## 4. Subscription is entitlement, not execution

The monthly subscription controls **commercial entitlement**, not user task execution.

An entitlement may authorize:

- official signed Kernux Desktop distribution;
- commercial UI/product modules;
- premium Capability Packs;
- premium local workflows/templates;
- official update channel;
- compatibility packs;
- organization/policy features;
- priority support or other separately defined commercial benefits.

An entitlement must never:

- become a capability Grant;
- bypass `kernuxd` policy;
- grant file/network/secret authority;
- upload user content for license validation;
- make cloud processing mandatory;
- prevent export/deletion of user-owned data after expiry.

Security authority and commercial entitlement are separate systems.

## 5. Open-source / commercial boundary

Kernux Core remains governed by its existing Apache-2.0 policy.

The sustainable commercial architecture is:

```text
Apache-2.0 Kernux Core
  - contracts
  - kernuxd capability/security kernel
  - local runtime fundamentals
  - protocol adapters
  - evidence/provenance fundamentals
  - extension SDK/contracts

Paid official product layer
  - signed official distributions
  - commercial product modules
  - selected premium Capability Packs
  - premium workflow/UX packs
  - organization/enterprise convenience features
  - official compatibility/update/support channel
```

Any non-Apache commercial module must have an explicit repository/package/license boundary. Apache-covered source must never be silently relicensed by bundling.

If a future founder decision makes the entire paid layer Apache-2.0 as well, subscription enforcement must be treated as a convenience/support/distribution model rather than exclusive technical access, because users can legally self-build Apache-licensed code.

## 6. Entitlement architecture

The product must avoid an always-online licensing dependency.

### 6.1 Signed local entitlement

The official product accepts a signed entitlement document with at least:

- opaque customer/account id;
- subscription/plan id;
- entitlement version;
- enabled commercial feature set;
- issue time;
- expiration/renewal horizon;
- device/seat constraints where applicable;
- issuer/key id;
- signature.

The application embeds only the public verification key.

Private signing keys never ship in the app.

Recommended implementation class:

- Ed25519 or another independently reviewed modern signature scheme;
- canonical serialization;
- rollback/replay protection where state is persisted;
- key rotation;
- deterministic validation tests.

Exact cryptographic dependencies remain subject to security review.

### 6.2 Offline behavior

A valid cached entitlement is checked locally.

Kernux must remain usable offline for a bounded grace period defined by the commercial policy.

License validation must not require:

- user project contents;
- prompts;
- files;
- memory;
- browser history;
- model requests;
- task history.

Only minimal commerce identity/entitlement metadata may leave the machine during refresh.

### 6.3 Renewal

Monthly renewal should refresh entitlement infrequently rather than on every launch/task.

Preferred flow:

```text
payment / merchant provider
        |
        v
minimal entitlement issuer
        |
 signed entitlement
        v
user device
        |
local verification
```

The entitlement issuer is not a workload backend.

It should be stateless or near-stateless where practical and hold no user work content.

Where a merchant/payment provider can natively issue or host the required entitlement, prefer that over a custom always-on service.

## 7. No per-task metering by Kernux

The default paid plans should not meter:

- tokens;
- tasks;
- browser minutes;
- terminal minutes;
- local model calls;
- local storage;
- local memory;
- local agents;
- local automations.

Those resources are supplied by the user.

Kernux may display usage/cost from third-party providers, but the charge relationship remains between the user and that provider unless the user separately purchases an explicitly priced managed Kernux service.

## 8. Model and agent economics

Order of preference:

1. local models on user hardware;
2. provider-native subscriptions already owned by user;
3. BYOK API providers;
4. user/organization-owned remote inference;
5. separately priced managed service only if intentionally offered later.

Kernux must never route through a founder API account merely to make onboarding look easier.

The same rule applies to DecisionProvider, embeddings, OCR, speech, vision, reranking and code agents.

## 9. Web/browser/search economics

Default:

- local Chromium/Playwright/CDP;
- direct destination web access from the user's device;
- local AgentQL/compatible local primitives where qualified;
- user-owned/BYOK hosted search/browser providers when selected.

Hosted TinyFish or another hosted browser/search product may be an optional provider, but the user's/provider's billing relationship must own the variable cost unless sold as a separately priced managed add-on with positive margin.

## 10. Storage, memory and sync economics

Default storage is local:

- SQLite;
- local artifact CAS;
- local browser profiles;
- local memory;
- local search/indexes;
- user filesystem;
- user-owned databases where explicitly connected.

Kernux must not require project-owner object storage, vector DB, database, backup, or sync service.

Cross-device options, in preferred order:

1. user-controlled file synchronization;
2. user-owned Git/repository/storage;
3. user-owned SSH/device/runtime;
4. self-hosted Kernux relay/sync if later built;
5. separately priced managed sync/relay only with explicit positive unit economics.

## 11. Notification economics

Local desktop notifications are free of hosted delivery.

External delivery such as SMS, WhatsApp, email gateway or push provider must use:

- user's own provider/account;
- organization-owned provider;
- platform-native free mechanism where qualified;
- or a separately priced add-on that covers its cost.

Never hide message-delivery cost inside an unlimited base subscription.

## 12. Distribution economics

Prefer distribution channels that do not create significant founder-paid bandwidth:

- GitHub Releases;
- package managers;
- platform-native release channels where economics are acceptable;
- direct model downloads from original approved sources rather than mirroring large model weights.

Do not become the CDN for third-party models or donor assets without a deliberate funded reason.

## 13. Optional managed services rule

Kernux Cloud is not required for the base subscription.

A future managed service is allowed only when all are true:

- optional;
- separable from local core;
- explicit price;
- identified cost owner;
- usage limits or pricing preserve positive unit economics;
- local/self-hosted path remains available where the feature is a core product capability;
- user data boundary is explicit;
- loss of the managed provider does not destroy local task truth.

No "unlimited AI/browser/compute" promise may be bundled without evidence that the economics remain sustainable.

## 14. Commercial plan shape

The intended base commercial product is simple.

### Kernux Personal / Pro

Monthly or annual subscription.

Includes the official local application and the commercial feature set defined for that plan.

Compute, models, APIs and storage are supplied by the user.

### Kernux Team

Per-seat subscription.

Default deployment remains local/user- or organization-owned.

Shared/team infrastructure should be self-hostable or use organization-owned services by default.

### Managed add-ons — future only

Examples:

- managed relay/sync;
- managed remote runtime;
- managed team service;
- hosted model/browser/search bundles.

These are separate SKUs with explicit economics, never hidden costs of the base local subscription.

## 15. Expiry and cancellation

Subscription expiry must preserve user sovereignty.

After entitlement expiration:

- user data remains readable/exportable according to product policy;
- secrets are not destroyed;
- local project files remain untouched;
- no data is held hostage;
- Kernux does not upload private data to resolve licensing;
- paid commercial modules may become unavailable or read-only as defined;
- Apache-2.0 core rights remain governed by the open-source license.

Renewal restores entitlement without migrating user work through a cloud service.

## 16. Anti-abuse without surveillance

Commercial enforcement should avoid invasive anti-piracy systems.

Do not require:

- continuous device fingerprint streaming;
- project-content inspection;
- process spying unrelated to Kernux;
- hidden telemetry;
- always-online DRM.

Allowed controls may include:

- bounded device/seat identities;
- signed entitlements;
- rate-limited entitlement issuance;
- revocation for fraud/chargeback;
- privacy-preserving account/device metadata.

Anti-abuse controls must be proportionate and separately threat-modeled.

## 17. Required metrics

Track business economics without collecting user work content.

Required:

- paid subscriptions;
- renewals/churn;
- plan/seat counts;
- entitlement refresh success;
- commerce/payment failure;
- managed-add-on COGS if any;
- support burden;
- release/download health.

Critical invariant metric:

```text
FOUNDER_FUNDED_RUNTIME_COST
  inference = 0
  browser/search = 0
  user storage = 0
  agent execution = 0
  local automation = 0
  ordinary connector usage = 0
```

Any nonzero value requires a documented exception, cost owner, price and margin.

## 18. Implementation ownership

### P03

Surface plan/entitlement status without exposing commerce internals to task authority.

### P05/P06/P11/P12/P17

Every provider/tool/model/browser/automation/connector declares cost owner. Founder-funded runtime is ineligible for core proof.

### P15

Qualify:

- signed entitlement validation;
- offline grace;
- renewal/expiry/cancellation;
- no user-content transmission during entitlement checks;
- telemetry-off operation;
- official binary/update entitlement behavior;
- user data/export behavior after expiry.

### P20

Optional managed services require separate unit-economics qualification and may not weaken the local product.

## 19. Commercial release gate

The official paid product is not commercially ready until:

1. one subscription can activate the official app without provisioning any founder-funded compute;
2. the app completes the local-only privacy golden journey while commercially entitled;
3. entitlement validation works locally from a signed cached token;
4. renewal transmits only minimal commerce metadata;
5. expired entitlement cannot destroy or trap user data;
6. every metered runtime provider has a non-founder cost owner;
7. no core golden journey requires a founder API key;
8. no subscription plan promises hidden unlimited managed compute;
9. payment/merchant costs are modeled separately from runtime COGS;
10. an audit shows ordinary paid-user runtime variable COGS attributable to Kernux infrastructure is zero.

## 20. Final rule

> **Kernux sells software capability, not subsidized compute. The user pays for Kernux; the user's machine, accounts and infrastructure perform the work.**
