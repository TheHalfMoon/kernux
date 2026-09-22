# Platform Fabrics and Source Integration Plan

## Status

This is the canonical entry point for integrating donor projects, models, tools, and founder-owned repositories into Kernux.

The permanent rule is:

> **Adopt capabilities behind Kernux contracts; never adopt donor architecture as product authority.**

The active implementation frontier remains governed by `specs/CURRENT.md`, SpecGrain, Diffcipline, exact-head evidence, and the dependency order in `EXECUTION_MASTER_PLAN.md`.

## Normative modules

1. [Platform Fabrics Architecture](platform-fabrics/ARCHITECTURE.md)
   - exact source identities;
   - architecture tiers;
   - Decision Fabric;
   - Tool and Integration Fabric;
   - Automation and Event Fabric.

2. [Platform Fabrics Runtime, Reliability, and Delivery](platform-fabrics/DELIVERY.md)
   - Web and Live-Data Fabric;
   - Computer Fabric;
   - Context and Memory Fabric;
   - Evidence and Verification Fabric;
   - Capability Packs;
   - reliability and failure semantics;
   - privacy, cost, provenance, observability, UX and evaluation;
   - dependency-correct delivery mapping;
   - anti-gap admission checklist;
   - planning exit criteria.

## Local privacy and economics

All fabrics and providers inherit:
- [Local Privacy Implementation Plan](LOCAL_PRIVACY_IMPLEMENTATION_PLAN.md);
- [Local Subscription and Zero Runtime COGS](LOCAL_SUBSCRIPTION_ZERO_RUNTIME_COGS.md).

No provider, Capability Pack, hosted service, or donor import may introduce hidden cloud processing, alternate authority, or founder-funded core runtime cost.
