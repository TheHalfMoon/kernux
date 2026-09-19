use kernux_contracts::{
    CapabilityRequest, DaemonProbeRequest, DaemonProbeResponse, Event, Evidence, Grant,
    KRP_SCHEMA_SHA256, OperationObservation, OperationStart, RuntimeContactObservation,
    RuntimeError,
};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;

fn round_trip<T>(value: &Value)
where
    T: DeserializeOwned + Serialize,
{
    let typed: T = serde_json::from_value(value.clone()).expect("fixture must deserialize");
    let encoded = serde_json::to_value(typed).expect("fixture must serialize");
    assert_eq!(
        &encoded, value,
        "Rust round trip changed semantic JSON value"
    );
}

#[test]
fn shared_fixture_bytes_round_trip_through_generated_rust_contracts() {
    let bytes = include_bytes!("../../../protocol/fixtures/v1/core.json");
    let fixture: Value = serde_json::from_slice(bytes).expect("shared fixture must be valid JSON");
    assert_eq!(
        fixture.get("schema_sha256").and_then(Value::as_str),
        Some(KRP_SCHEMA_SHA256),
        "fixture must bind the generated schema digest",
    );

    let cases = fixture
        .get("cases")
        .and_then(Value::as_array)
        .expect("shared fixture must contain cases");
    assert_eq!(
        cases.len(),
        10,
        "expected the canonical ten core fixture cases"
    );

    for case in cases {
        let definition = case
            .get("definition")
            .and_then(Value::as_str)
            .expect("fixture definition must be a string");
        let value = case.get("value").expect("fixture case must contain value");
        match definition {
            "CapabilityRequest" => round_trip::<CapabilityRequest>(value),
            "Grant" => round_trip::<Grant>(value),
            "RuntimeContactObservation" => round_trip::<RuntimeContactObservation>(value),
            "OperationStart" => round_trip::<OperationStart>(value),
            "OperationObservation" => round_trip::<OperationObservation>(value),
            "RuntimeError" => round_trip::<RuntimeError>(value),
            "Event" => round_trip::<Event>(value),
            "Evidence" => round_trip::<Evidence>(value),
            "DaemonProbeRequest" => round_trip::<DaemonProbeRequest>(value),
            "DaemonProbeResponse" => round_trip::<DaemonProbeResponse>(value),
            other => panic!("unsupported shared fixture definition: {other}"),
        }
    }
}

#[test]
fn daemon_probe_contract_rejects_unknown_fields_and_unsupported_versions() {
    let request_id = "01890f00-0000-7000-8000-000000000013";

    let unknown_request = serde_json::json!({
        "request_id": request_id,
        "contract_version": "krp/1",
        "probe": "health_version",
        "authority": "admin"
    });
    assert!(
        serde_json::from_value::<DaemonProbeRequest>(unknown_request).is_err(),
        "daemon probe request must reject unknown fields"
    );

    let unsupported_request = serde_json::json!({
        "request_id": request_id,
        "contract_version": "krp/2",
        "probe": "health_version"
    });
    assert!(
        serde_json::from_value::<DaemonProbeRequest>(unsupported_request).is_err(),
        "daemon probe request must reject unsupported protocol versions"
    );

    let unsupported_response = serde_json::json!({
        "request_id": request_id,
        "contract_version": "krp/2",
        "lifecycle_state": "serving",
        "health": "healthy",
        "daemon_version": "0.0.0",
        "implementation_revision": "1111111111111111111111111111111111111111",
        "schema_sha256": KRP_SCHEMA_SHA256
    });
    assert!(
        serde_json::from_value::<DaemonProbeResponse>(unsupported_response).is_err(),
        "daemon probe response must reject unsupported protocol versions"
    );
}

#[test]
fn daemon_probe_fixture_binds_request_response_and_schema_provenance() {
    let bytes = include_bytes!("../../../protocol/fixtures/v1/core.json");
    let fixture: Value = serde_json::from_slice(bytes).expect("shared fixture must be valid JSON");
    let cases = fixture
        .get("cases")
        .and_then(Value::as_array)
        .expect("shared fixture must contain cases");

    let request = cases
        .iter()
        .find(|case| case.get("name").and_then(Value::as_str) == Some("daemon_probe_request"))
        .and_then(|case| case.get("value"))
        .expect("daemon probe request fixture must exist");
    let response = cases
        .iter()
        .find(|case| case.get("name").and_then(Value::as_str) == Some("daemon_probe_response"))
        .and_then(|case| case.get("value"))
        .expect("daemon probe response fixture must exist");

    assert_eq!(request.get("request_id"), response.get("request_id"));
    assert_eq!(
        request.get("contract_version").and_then(Value::as_str),
        Some("krp/1")
    );
    assert_eq!(
        response.get("contract_version").and_then(Value::as_str),
        Some("krp/1")
    );
    assert_eq!(
        response.get("schema_sha256").and_then(Value::as_str),
        Some(KRP_SCHEMA_SHA256)
    );
}
