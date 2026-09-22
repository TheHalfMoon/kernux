# Local Privacy Implementation Plan

## Status

This is the canonical entry point for Kernux local-privacy implementation planning.

Local privacy is a product invariant, not a deployment preference:

> **User data stays local by default. External data movement is explicit, destination-bound, inspectable, revocable, and never required for core correctness.**

The active implementation frontier remains governed by `specs/CURRENT.md`, SpecGrain, Diffcipline, repository policy, and exact-head evidence. This plan does not authorize skipping dependency order.

## Normative modules

1. [Local Privacy Foundations](local-privacy/FOUNDATIONS.md)
   - threat model;
   - privacy modes;
   - egress classification;
   - provider data-boundary contract;
   - local-first capability baseline;
   - no-silent-cloud-fallback;
   - network enforcement;
   - secret privacy;
   - protection at rest;
   - privacy firewall;
   - context minimization;
   - browser/account isolation;
   - telemetry, updates, deletion and backup.

2. [Local Privacy Qualification and Delivery](local-privacy/QUALIFICATION.md)
   - Privacy Inspector;
   - privacy evidence;
   - adversarial corpus;
   - SLOs and release metrics;
   - dependency-correct phase mapping;
   - implementation-ready work packages LP-01 through LP-12;
   - alpha privacy gate;
   - definition of implementation-ready.

## Permanent rule

> **Local is the default trust boundary. Network access is a capability. Cloud processing is an explicit data-boundary decision. Failure never silently weakens privacy.**
