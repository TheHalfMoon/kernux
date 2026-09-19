use kernux_contracts::{Event, OperationStart, RuntimeCapability, RuntimeDescriptor, RuntimeError};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

const REQUIRED_POLICY_RULES: [(&str, &str); 20] = [
    ("ADV-01", "contact_loss_preserves_execution"),
    ("ADV-02", "timeout_preserves_execution"),
    ("ADV-03", "cancellation_acceptance_preserves_execution"),
    ("ADV-04", "cancellation_transport_loss_is_unverifiable"),
    ("ADV-05", "stale_observation_is_rejected"),
    ("ADV-06", "same_fingerprint_deduplicates"),
    ("ADV-07", "fingerprint_conflict_rejects"),
    ("ADV-08", "ambiguous_start_preserves_operation_id"),
    ("ADV-09", "may_have_started_forbids_fresh_operation"),
    ("ADV-10", "unknown_retry_guidance_reconciles"),
    ("ADV-11", "authority_requires_intersection"),
    ("ADV-12", "authority_requires_intersection"),
    ("ADV-13", "runtime_type_does_not_imply_capability"),
    ("ADV-14", "reconnect_does_not_restore_grant"),
    ("ADV-15", "trust_identity_change_is_new_runtime"),
    ("ADV-16", "provider_diagnostic_cannot_lower_uncertainty"),
    ("ADV-17", "request_timeout_is_not_operation_failure"),
    ("ADV-18", "controller_cache_loss_is_not_exit"),
    ("ADV-19", "equal_generation_conflict_fails"),
    ("ADV-20", "unknown_operation_preserves_uncertainty"),
];

const REQUIRED_WIRE_CASES: [(&str, &str); 13] = [
    ("WIRE-01", "unknown_operation_start_field"),
    ("WIRE-02", "missing_operation_start_protocol_contract"),
    ("WIRE-03", "malformed_operation_uuid"),
    ("WIRE-04", "malformed_payload_digest"),
    ("WIRE-05", "malformed_resource_uri"),
    ("WIRE-06", "unknown_error_category"),
    ("WIRE-07", "unknown_retry_guidance"),
    ("WIRE-08", "malformed_nested_runtime_revision"),
    ("WIRE-09", "runtime_descriptor_protocol_mismatch"),
    ("WIRE-10", "runtime_capability_version_mismatch"),
    ("WIRE-11", "operation_protocol_mismatch"),
    ("WIRE-12", "operation_capability_version_mismatch"),
    ("WIRE-13", "event_contract_version_mismatch"),
];

fn adversarial_fixture() -> Value {
    serde_json::from_slice(include_bytes!(
        "../../../protocol/fixtures/v1/adversarial.json"
    ))
    .expect("shared adversarial fixture must be valid JSON")
}

fn core_fixture() -> Value {
    serde_json::from_slice(include_bytes!("../../../protocol/fixtures/v1/core.json"))
        .expect("shared core fixture must be valid JSON")
}

fn object<'a>(value: &'a Value, path: &str) -> &'a serde_json::Map<String, Value> {
    value
        .as_object()
        .unwrap_or_else(|| panic!("{path} must be an object"))
}

fn array<'a>(value: &'a Value, path: &str) -> &'a Vec<Value> {
    value
        .as_array()
        .unwrap_or_else(|| panic!("{path} must be an array"))
}

fn string<'a>(value: &'a Value, path: &str) -> &'a str {
    value
        .as_str()
        .unwrap_or_else(|| panic!("{path} must be a string"))
}

fn bool_value(value: &Value, path: &str) -> bool {
    value
        .as_bool()
        .unwrap_or_else(|| panic!("{path} must be a boolean"))
}

fn u64_value(value: &Value, path: &str) -> u64 {
    value
        .as_u64()
        .unwrap_or_else(|| panic!("{path} must be an unsigned integer"))
}

fn field<'a>(value: &'a Value, name: &str, path: &str) -> &'a Value {
    object(value, path)
        .get(name)
        .unwrap_or_else(|| panic!("{path}.{name} is required"))
}

fn source_cases() -> BTreeMap<String, (String, Value)> {
    let fixture = core_fixture();
    let mut result = BTreeMap::new();
    for entry in array(
        field(&fixture, "cases", "core fixture"),
        "core fixture.cases",
    ) {
        let name = string(field(entry, "name", "core case"), "core case.name").to_owned();
        let definition = string(
            field(entry, "definition", "core case"),
            "core case.definition",
        )
        .to_owned();
        let value = field(entry, "value", "core case").clone();
        assert!(
            result.insert(name.clone(), (definition, value)).is_none(),
            "duplicate core fixture case {name}"
        );
    }
    result
}

fn policy_cases() -> BTreeMap<String, Value> {
    let fixture = adversarial_fixture();
    let mut result = BTreeMap::new();
    for entry in array(
        field(&fixture, "policy_cases", "adversarial fixture"),
        "adversarial fixture.policy_cases",
    ) {
        let id = string(field(entry, "id", "policy case"), "policy case.id").to_owned();
        assert!(
            result.insert(id.clone(), entry.clone()).is_none(),
            "duplicate policy case {id}"
        );
    }
    result
}

fn expected_policy_case<'a>(cases: &'a BTreeMap<String, Value>, id: &str, rule: &str) -> &'a Value {
    let entry = cases
        .get(id)
        .unwrap_or_else(|| panic!("missing policy case {id}"));
    assert_eq!(
        string(field(entry, "rule", id), &format!("{id}.rule")),
        rule,
        "{id} must remain bound to its canonical rule"
    );
    entry
}

fn value_at<'a>(entry: &'a Value, section: &str, key: &str, id: &str) -> &'a Value {
    field(field(entry, section, id), key, &format!("{id}.{section}"))
}

fn set_at_path(value: &mut Value, path: &[Value], replacement: Value) {
    assert!(!path.is_empty(), "set mutation path must be non-empty");
    let mut target = value;
    for segment in &path[..path.len() - 1] {
        let name = segment
            .as_str()
            .expect("set mutation path segments must be strings");
        target = target
            .as_object_mut()
            .and_then(|object| object.get_mut(name))
            .unwrap_or_else(|| panic!("set mutation cannot traverse {name}"));
    }
    let name = path
        .last()
        .and_then(Value::as_str)
        .expect("set mutation final path segment must be a string");
    target
        .as_object_mut()
        .unwrap_or_else(|| panic!("set mutation target for {name} must be an object"))
        .insert(name.to_owned(), replacement);
}

fn remove_at_path(value: &mut Value, path: &[Value]) {
    assert!(!path.is_empty(), "remove mutation path must be non-empty");
    let mut target = value;
    for segment in &path[..path.len() - 1] {
        let name = segment
            .as_str()
            .expect("remove mutation path segments must be strings");
        target = target
            .as_object_mut()
            .and_then(|object| object.get_mut(name))
            .unwrap_or_else(|| panic!("remove mutation cannot traverse {name}"));
    }
    let name = path
        .last()
        .and_then(Value::as_str)
        .expect("remove mutation final path segment must be a string");
    assert!(
        target
            .as_object_mut()
            .unwrap_or_else(|| panic!("remove mutation target for {name} must be an object"))
            .remove(name)
            .is_some(),
        "remove mutation target {name} must exist"
    );
}

fn apply_mutation(value: &mut Value, mutation: &Value, id: &str) {
    let op = string(field(mutation, "op", id), &format!("{id}.mutation.op"));
    let path = array(field(mutation, "path", id), &format!("{id}.mutation.path"));

    match op {
        "none" => assert!(path.is_empty(), "{id} none mutation path must be empty"),
        "set" => {
            let replacement = field(mutation, "value", id).clone();
            set_at_path(value, path, replacement);
        }
        "remove" => remove_at_path(value, path),
        other => panic!("{id} has unsupported mutation operation {other}"),
    }
}

fn rust_deserializes<T: DeserializeOwned>(value: Value) -> Result<(), String> {
    serde_json::from_value::<T>(value)
        .map(|_| ())
        .map_err(|error| error.to_string())
}

fn observed_rust_result(definition: &str, value: Value) -> Result<(), String> {
    match definition {
        "Event" => rust_deserializes::<Event>(value),
        "OperationStart" => rust_deserializes::<OperationStart>(value),
        "RuntimeCapability" => rust_deserializes::<RuntimeCapability>(value),
        "RuntimeDescriptor" => rust_deserializes::<RuntimeDescriptor>(value),
        "RuntimeError" => rust_deserializes::<RuntimeError>(value),
        other => panic!("unsupported adversarial Rust definition {other}"),
    }
}

fn policy_outcome(entry: &Value, id: &str, rule: &str) -> Value {
    let input = field(entry, "input", id);
    match rule {
        "contact_loss_preserves_execution"
        | "timeout_preserves_execution"
        | "cancellation_acceptance_preserves_execution" => {
            json!({"execution_state": value_at(entry, "input", "execution_state", id)})
        }
        "cancellation_transport_loss_is_unverifiable" => {
            json!({"cancellation_state": "unverifiable"})
        }
        "stale_observation_is_rejected" => {
            assert!(
                u64_value(
                    value_at(entry, "input", "incoming_generation", id),
                    "incoming_generation"
                ) < u64_value(
                    value_at(entry, "input", "current_generation", id),
                    "current_generation"
                )
            );
            json!({
                "disposition": "stale_rejected",
                "execution_state": value_at(entry, "input", "current_state", id)
            })
        }
        "same_fingerprint_deduplicates" => {
            assert_eq!(
                value_at(entry, "input", "stored_fingerprint", id),
                value_at(entry, "input", "incoming_fingerprint", id)
            );
            json!({"disposition": "reconcile_existing", "execute": false})
        }
        "fingerprint_conflict_rejects" => {
            assert_ne!(
                value_at(entry, "input", "stored_fingerprint", id),
                value_at(entry, "input", "incoming_fingerprint", id)
            );
            json!({"disposition": "operation_conflict", "execute": false})
        }
        "ambiguous_start_preserves_operation_id" => json!({
            "operation_id": value_at(entry, "input", "operation_id", id),
            "retry_guidance": "reconcile_operation"
        }),
        "may_have_started_forbids_fresh_operation" => {
            assert_eq!(
                string(
                    value_at(entry, "input", "side_effect_certainty", id),
                    "side_effect_certainty"
                ),
                "may_have_started"
            );
            json!({
                "create_fresh_operation": false,
                "operation_id": value_at(entry, "input", "operation_id", id)
            })
        }
        "unknown_retry_guidance_reconciles" => {
            assert_eq!(
                string(
                    value_at(entry, "input", "side_effect_certainty", id),
                    "side_effect_certainty"
                ),
                "may_have_started"
            );
            json!({"blind_retry": false, "retry_guidance": "reconcile_operation"})
        }
        "authority_requires_intersection" => json!({
            "authorized":
                bool_value(field(input, "advertised_capability", id), "advertised_capability")
                && bool_value(field(input, "grant", id), "grant")
                && bool_value(field(input, "policy_allows", id), "policy_allows")
                && bool_value(
                    field(input, "negotiated_capability", id),
                    "negotiated_capability"
                )
        }),
        "runtime_type_does_not_imply_capability" => {
            let requested = string(field(input, "requested_action", id), "requested_action");
            let advertised = array(field(input, "advertised_actions", id), "advertised_actions");
            json!({"authorized": advertised.iter().any(|value| value.as_str() == Some(requested))})
        }
        "reconnect_does_not_restore_grant" => json!({
            "authorized": string(field(input, "grant_status", id), "grant_status") == "active"
        }),
        "trust_identity_change_is_new_runtime" => json!({
            "same_runtime":
                field(input, "enrolled_identity", id) == field(input, "observed_identity", id)
        }),
        "provider_diagnostic_cannot_lower_uncertainty" => json!({
            "side_effect_certainty":
                field(input, "canonical_side_effect_certainty", id)
        }),
        "request_timeout_is_not_operation_failure" => {
            let observed = field(input, "authoritative_execution_observation", id);
            json!({
                "execution_state": if observed.is_null() {
                    Value::String("unverifiable".to_owned())
                } else {
                    observed.clone()
                }
            })
        }
        "controller_cache_loss_is_not_exit" => json!({
            "execution_state": if bool_value(
                field(input, "authoritative_exit_observation", id),
                "authoritative_exit_observation"
            ) {
                "exited"
            } else {
                "unverifiable"
            }
        }),
        "equal_generation_conflict_fails" => {
            assert_eq!(
                field(input, "current_generation", id),
                field(input, "incoming_generation", id)
            );
            assert_ne!(
                field(input, "current_state", id),
                field(input, "incoming_state", id)
            );
            json!({"disposition": "invariant_conflict"})
        }
        "unknown_operation_preserves_uncertainty" => {
            assert_eq!(
                string(field(input, "operation_lookup", id), "operation_lookup"),
                "unknown"
            );
            if bool_value(
                field(input, "persistence_guarantee_definitely_not_started", id),
                "persistence_guarantee_definitely_not_started",
            ) {
                json!({
                    "retry_guidance": "retry_same_operation",
                    "side_effect_certainty": "definitely_not_started"
                })
            } else {
                json!({
                    "retry_guidance": "reconcile_operation",
                    "side_effect_certainty": "may_have_started"
                })
            }
        }
        other => panic!("unsupported adversarial policy rule {other}"),
    }
}

#[test]
fn shared_adversarial_policy_fixture_preserves_all_twenty_invariants() {
    let fixture = adversarial_fixture();
    assert_eq!(
        string(
            field(&fixture, "schema", "adversarial fixture"),
            "adversarial fixture.schema"
        ),
        "kernux.krp.adversarial/v1"
    );

    let required_ids: Vec<_> = array(
        field(&fixture, "required_invariant_ids", "adversarial fixture"),
        "adversarial fixture.required_invariant_ids",
    )
    .iter()
    .map(|value| {
        value
            .as_str()
            .expect("required invariant id must be a string")
    })
    .collect();
    let expected_ids: Vec<_> = REQUIRED_POLICY_RULES.iter().map(|(id, _)| *id).collect();
    assert_eq!(required_ids, expected_ids);

    let cases = policy_cases();
    assert_eq!(cases.len(), REQUIRED_POLICY_RULES.len());

    for (id, rule) in REQUIRED_POLICY_RULES {
        let entry = expected_policy_case(&cases, id, rule);
        let expected = field(entry, "expected", id);
        assert_eq!(
            policy_outcome(entry, id, rule),
            *expected,
            "{id} policy outcome drifted from the canonical fixture"
        );
    }
}

#[test]
fn shared_adversarial_wire_fixture_matches_generated_rust_rejection_boundary() {
    let fixture = adversarial_fixture();
    let core = source_cases();
    let wire_cases = array(
        field(&fixture, "wire_cases", "adversarial fixture"),
        "adversarial fixture.wire_cases",
    );

    assert_eq!(wire_cases.len(), REQUIRED_WIRE_CASES.len());

    let required: BTreeMap<_, _> = REQUIRED_WIRE_CASES.into_iter().collect();
    let mut observed_ids = BTreeSet::new();
    let mut version_definitions = BTreeSet::new();
    let mut reject_count = 0usize;
    let mut schema_only_count = 0usize;

    for entry in wire_cases {
        let id = string(field(entry, "id", "wire case"), "wire case.id");
        let name = string(field(entry, "name", id), &format!("{id}.name"));
        let expected_name = required
            .get(id)
            .unwrap_or_else(|| panic!("unexpected wire case id {id}"));
        assert_eq!(
            name, *expected_name,
            "{id} must remain bound to its scenario"
        );
        assert!(observed_ids.insert(id), "duplicate wire case {id}");

        let definition = string(field(entry, "definition", id), &format!("{id}.definition"));
        let expected_rust = string(
            field(entry, "expected_rust", id),
            &format!("{id}.expected_rust"),
        );

        let mut value = if let Some(source_case) = object(entry, id).get("source_case") {
            let source_name = string(source_case, &format!("{id}.source_case"));
            let (source_definition, source_value) = core
                .get(source_name)
                .unwrap_or_else(|| panic!("{id} references unknown source case {source_name}"));
            assert_eq!(
                source_definition, definition,
                "{id} source definition must match wire definition"
            );
            source_value.clone()
        } else {
            field(entry, "inline_value", id).clone()
        };

        apply_mutation(&mut value, field(entry, "mutation", id), id);
        let observed = observed_rust_result(definition, value);

        match expected_rust {
            "reject" => {
                reject_count += 1;
                assert!(
                    observed.is_err(),
                    "{id} {name} must be rejected by generated Rust serde"
                );
            }
            "schema_only" => {
                schema_only_count += 1;
                assert!(
                    observed.is_ok(),
                    "{id} {name} is intentionally a JSON-Schema pattern boundary; generated Rust serde should not pretend to enforce it"
                );
            }
            other => panic!("{id} has unsupported expected_rust value {other}"),
        }

        if name.contains("mismatch") {
            version_definitions.insert(definition);
        }
    }

    assert_eq!(observed_ids.len(), REQUIRED_WIRE_CASES.len());
    assert_eq!(reject_count, 10);
    assert_eq!(schema_only_count, 3);
    assert_eq!(
        version_definitions,
        BTreeSet::from([
            "Event",
            "OperationStart",
            "RuntimeCapability",
            "RuntimeDescriptor"
        ])
    );
}

#[test]
fn adversarial_fixture_keeps_runtime_evidence_claims_out_of_scope() {
    let fixture = adversarial_fixture();
    let boundary = field(&fixture, "evidence_boundary", "adversarial fixture");

    assert_eq!(
        string(
            field(boundary, "proves", "evidence_boundary"),
            "evidence_boundary.proves"
        ),
        "contract_and_policy_fixture_conformance"
    );

    let exclusions: Vec<_> = array(
        field(boundary, "does_not_prove", "evidence_boundary"),
        "evidence_boundary.does_not_prove",
    )
    .iter()
    .map(|value| {
        value
            .as_str()
            .expect("evidence-boundary exclusion must be a string")
    })
    .collect();

    assert_eq!(
        exclusions,
        vec![
            "exactly_once_execution",
            "durable_replay_protection",
            "cancellation_delivery",
            "reconnect_persistence",
            "runtime_transport_behavior",
        ]
    );
}
