use kernux_contracts::{
    CapabilityRequest, Event, Evidence, Grant, KRP_SCHEMA_SHA256, OperationObservation,
    OperationStart, RuntimeContactObservation, RuntimeError,
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
        8,
        "expected the canonical eight core fixture cases"
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
            other => panic!("unsupported shared fixture definition: {other}"),
        }
    }
}
