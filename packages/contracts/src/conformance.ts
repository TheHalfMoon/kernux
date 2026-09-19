// @generated from protocol/fixtures/v1/core.json
// Fixture SHA-256: 1bc115ec247d0480b456fbbcfd8e6eb5daf82a712e4169c7c32adcb2e6c7e6b5
// Schema SHA-256: 0ec273c387cf8f344b178c66ae046f75af88945dc428f4a16601156f4e538500
// DO NOT EDIT. Change the shared fixture source and regenerate.

import type { CapabilityRequest, Event, Evidence, Grant, OperationObservation, OperationStart, RuntimeContactObservation, RuntimeError } from "./generated";

export const capabilityRequestFixture = {
    "request_id": "01890f00-0000-7000-8000-000000000001",
    "subject_scope": {
      "run": {
        "kind": "run",
        "id": "01890f00-0000-7000-8000-000000000002"
      },
      "agent_session": {
        "kind": "agent_session",
        "id": "01890f00-0000-7000-8000-000000000003"
      }
    },
    "action": "files.write",
    "resource_uri": "kernux://project/01890f00-0000-7000-8000-000000000004/fs/src/main.rs",
    "runtime": {
      "kind": "runtime",
      "id": "01890f00-0000-7000-8000-000000000005",
      "revision": 2
    },
    "requested_constraints": {
      "allowed_roots": [
        "src"
      ],
      "max_bytes": 4096
    },
    "provenance_event_ids": [
      "01890f00-0000-7000-8000-000000000006"
    ],
    "reason": "Update one bounded source file."
  } satisfies CapabilityRequest;

export const boundedGrantFixture = {
    "grant_id": "01890f00-0000-7000-8000-000000000007",
    "subject_scope": {
      "run": {
        "kind": "run",
        "id": "01890f00-0000-7000-8000-000000000002"
      }
    },
    "action": "files.write",
    "resource_uri": "kernux://project/01890f00-0000-7000-8000-000000000004/fs/src",
    "resource_match": "subtree",
    "runtime": {
      "kind": "runtime",
      "id": "01890f00-0000-7000-8000-000000000005",
      "revision": 2
    },
    "constraints": {
      "allowed_roots": [
        "src"
      ],
      "max_bytes": 4096,
      "max_uses": 1
    },
    "consequence_ceiling": "C2",
    "issuer_authority": "local-user",
    "policy_revision": 4,
    "issued_at": "2026-09-19T01:00:00Z",
    "not_before": "2026-09-19T01:00:00Z",
    "expires_at": "2026-09-19T01:05:00Z",
    "max_uses": 1,
    "delegation_depth": 0
  } satisfies Grant;

export const runtimeContactLostFixture = {
    "runtime": {
      "kind": "runtime",
      "id": "01890f00-0000-7000-8000-000000000005",
      "revision": 2
    },
    "contact_state": "disconnected",
    "observation_generation": 7,
    "observed_at": "2026-09-19T01:01:00Z"
  } satisfies RuntimeContactObservation;

export const operationStartFixture = {
    "request_id": "01890f00-0000-7000-8000-000000000008",
    "operation_id": "01890f00-0000-7000-8000-000000000009",
    "subject_scope": {
      "run": {
        "kind": "run",
        "id": "01890f00-0000-7000-8000-000000000002"
      }
    },
    "action": "files.write",
    "resource_uri": "kernux://project/01890f00-0000-7000-8000-000000000004/fs/src/main.rs",
    "runtime": {
      "kind": "runtime",
      "id": "01890f00-0000-7000-8000-000000000005",
      "revision": 2
    },
    "constraints": {
      "allowed_roots": [
        "src"
      ],
      "max_bytes": 4096
    },
    "payload_sha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
    "protocol_contract": "krp/1",
    "capability_version": "1"
  } satisfies OperationStart;

export const operationUnverifiableFixture = {
    "operation_id": "01890f00-0000-7000-8000-000000000009",
    "runtime": {
      "kind": "runtime",
      "id": "01890f00-0000-7000-8000-000000000005",
      "revision": 2
    },
    "observation_generation": 8,
    "execution_state": "unverifiable",
    "observed_at": "2026-09-19T01:01:01Z"
  } satisfies OperationObservation;

export const runtimeErrorAmbiguousSideEffectFixture = {
    "code": "transport_lost_after_start",
    "category": "operation_unverifiable",
    "message": "Runtime contact was lost after start admission.",
    "request_id": "01890f00-0000-7000-8000-000000000008",
    "operation_id": "01890f00-0000-7000-8000-000000000009",
    "retry_guidance": "reconcile_operation",
    "side_effect_certainty": "may_have_started",
    "details": [
      {
        "key": "observation_generation",
        "value": "8"
      }
    ],
    "provider_diagnostic": [
      {
        "key": "foreign_code",
        "value": "transport-reset"
      }
    ]
  } satisfies RuntimeError;

export const operationEventFixture = {
    "event_id": "01890f00-0000-7000-8000-000000000010",
    "event_type": "operation.observed",
    "contract_version": "krp/1",
    "stream": {
      "owner_kind": "run",
      "owner": {
        "kind": "run",
        "id": "01890f00-0000-7000-8000-000000000002"
      }
    },
    "stream_seq": 12,
    "previous_event_id": "01890f00-0000-7000-8000-000000000011",
    "producer": "kernux-runtime-controller",
    "recorded_at": "2026-09-19T01:01:02Z",
    "correlations": {
      "run": {
        "kind": "run",
        "id": "01890f00-0000-7000-8000-000000000002"
      },
      "runtime": {
        "kind": "runtime",
        "id": "01890f00-0000-7000-8000-000000000005",
        "revision": 2
      },
      "protocol_request_id": "01890f00-0000-7000-8000-000000000008",
      "operation_id": "01890f00-0000-7000-8000-000000000009"
    },
    "lineage": {
      "caused_by": [
        "01890f00-0000-7000-8000-000000000006"
      ],
      "derived_from": []
    },
    "artifact_refs": [],
    "sensitivity": "internal",
    "redaction": {
      "state": "none",
      "affected_roles": []
    }
  } satisfies Event;

export const machineEvidenceFixture = {
    "evidence_id": "01890f00-0000-7000-8000-000000000012",
    "claim": "The operation remains unverifiable after runtime contact loss.",
    "subject_bindings": [
      {
        "kind": "run",
        "id": "01890f00-0000-7000-8000-000000000002"
      }
    ],
    "source_class": "machine",
    "observer": "kernux-runtime-controller",
    "observed_event_ids": [
      "01890f00-0000-7000-8000-000000000010"
    ],
    "artifact_refs": [],
    "runtime_context": {
      "kind": "runtime",
      "id": "01890f00-0000-7000-8000-000000000005",
      "revision": 2
    },
    "operation_id": "01890f00-0000-7000-8000-000000000009",
    "implementation_revision": "44e8ec21f7a3b402cc78a9a6c8e7cb176dc39b46",
    "result": "unverifiable",
    "limitations": [
      "No authoritative exit observation was available."
    ],
    "recorded_by_event_id": "01890f00-0000-7000-8000-000000000010",
    "sensitivity": "internal",
    "redaction": {
      "state": "none",
      "affected_roles": []
    }
  } satisfies Evidence;
