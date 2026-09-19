use kernux_contracts::{OperationStart, RetryGuidance, RuntimeError, SideEffectCertainty};
use serde_json::Value;

fn fixture_value(definition: &str) -> Value {
    let bytes = include_bytes!("../../../protocol/fixtures/v1/core.json");
    let fixture: Value = serde_json::from_slice(bytes).expect("shared fixture must be valid JSON");
    fixture
        .get("cases")
        .and_then(Value::as_array)
        .expect("shared fixture must contain cases")
        .iter()
        .find(|case| case.get("definition").and_then(Value::as_str) == Some(definition))
        .and_then(|case| case.get("value"))
        .cloned()
        .unwrap_or_else(|| panic!("shared fixture must contain {definition}"))
}

fn operation_start_fixture() -> Value {
    fixture_value("OperationStart")
}

#[test]
fn generated_rust_rejects_unknown_operation_start_fields() {
    let mut value = operation_start_fixture();
    value
        .as_object_mut()
        .expect("OperationStart fixture must be an object")
        .insert(
            "future_hint".to_owned(),
            Value::String("unnegotiated".to_owned()),
        );

    let error = serde_json::from_value::<OperationStart>(value)
        .expect_err("unknown fields must fail closed in generated Rust contracts");

    assert!(
        error.to_string().contains("unknown field"),
        "unexpected serde error: {error}"
    );
}

#[test]
fn generated_rust_rejects_unsupported_operation_versions() {
    let baseline = operation_start_fixture();
    serde_json::from_value::<OperationStart>(baseline.clone())
        .expect("krp/1 with capability version 1 must deserialize");

    for (field, unsupported) in [("protocol_contract", "krp/2"), ("capability_version", "2")] {
        let mut value = baseline.clone();
        value
            .as_object_mut()
            .expect("OperationStart fixture must be an object")
            .insert(field.to_owned(), Value::String(unsupported.to_owned()));

        let error = serde_json::from_value::<OperationStart>(value)
            .expect_err("unsupported version token must fail closed");

        assert!(
            error.to_string().contains("unknown variant"),
            "unexpected serde error for {field}: {error}"
        );
    }
}

#[test]
fn optional_diagnostic_omission_preserves_side_effect_truth() {
    let mut value = fixture_value("RuntimeError");
    value
        .as_object_mut()
        .expect("RuntimeError fixture must be an object")
        .remove("provider_diagnostic");

    let typed: RuntimeError =
        serde_json::from_value(value).expect("known optional field omission must remain valid");

    assert!(typed.provider_diagnostic.is_none());
    assert_eq!(
        typed.side_effect_certainty,
        SideEffectCertainty::MayHaveStarted
    );
    assert_eq!(typed.retry_guidance, RetryGuidance::ReconcileOperation);
}
