use kernux_contracts::{DaemonHealthState, DaemonLifecycleState, ProtocolContract};
use kernuxd::{DaemonLifecycle, LifecycleError, LifecycleState, ProbeMetadata};

fn metadata() -> ProbeMetadata {
    ProbeMetadata::new(
        env!("CARGO_PKG_VERSION"),
        "1111111111111111111111111111111111111111",
    )
    .expect("fixture metadata must be valid")
}

#[test]
fn lifecycle_is_deterministic_from_start_to_stop() {
    let (lifecycle, shutdown) = DaemonLifecycle::new(metadata());

    assert_eq!(lifecycle.state(), LifecycleState::Starting);
    lifecycle
        .mark_serving()
        .expect("starting daemon must enter serving");
    assert_eq!(lifecycle.state(), LifecycleState::Serving);

    assert!(shutdown.request_shutdown());
    assert_eq!(lifecycle.state(), LifecycleState::ShuttingDown);
    assert!(
        !shutdown.request_shutdown(),
        "shutdown request is idempotent"
    );

    lifecycle
        .mark_stopped()
        .expect("shutting-down daemon must enter stopped");
    assert_eq!(lifecycle.state(), LifecycleState::Stopped);
    assert!(!shutdown.request_shutdown());
}

#[test]
fn shutdown_can_interrupt_starting_without_creating_serving_state() {
    let (lifecycle, shutdown) = DaemonLifecycle::new(metadata());

    assert!(shutdown.request_shutdown());
    assert_eq!(lifecycle.state(), LifecycleState::ShuttingDown);
    assert_eq!(
        lifecycle.mark_serving(),
        Err(LifecycleError::InvalidTransition {
            from: LifecycleState::ShuttingDown,
            to: LifecycleState::Serving,
        })
    );
    lifecycle.mark_stopped().expect("shutdown can complete");
}

#[test]
fn invalid_worker_transitions_fail_closed() {
    let (lifecycle, _) = DaemonLifecycle::new(metadata());

    assert_eq!(
        lifecycle.mark_stopped(),
        Err(LifecycleError::InvalidTransition {
            from: LifecycleState::Starting,
            to: LifecycleState::Stopped,
        })
    );
    lifecycle.mark_serving().expect("first serve transition");
    assert_eq!(
        lifecycle.mark_serving(),
        Err(LifecycleError::InvalidTransition {
            from: LifecycleState::Serving,
            to: LifecycleState::Serving,
        })
    );
}

#[test]
fn probe_reports_health_without_implying_authority() {
    let (lifecycle, shutdown) = DaemonLifecycle::new(metadata());
    let request_id = "01890f00-0000-7000-8000-000000000013";

    let starting = lifecycle.probe_response(request_id);
    assert_eq!(starting.contract_version, ProtocolContract::Krp1);
    assert_eq!(starting.lifecycle_state, DaemonLifecycleState::Starting);
    assert_eq!(starting.health, DaemonHealthState::Unavailable);

    lifecycle.mark_serving().expect("serve");
    let serving = lifecycle.probe_response(request_id);
    assert_eq!(serving.lifecycle_state, DaemonLifecycleState::Serving);
    assert_eq!(serving.health, DaemonHealthState::Healthy);
    assert_eq!(serving.daemon_version, env!("CARGO_PKG_VERSION"));
    assert_eq!(
        serving.implementation_revision,
        "1111111111111111111111111111111111111111"
    );
    assert_eq!(serving.schema_sha256.len(), 64);

    assert!(shutdown.request_shutdown());
    let shutting_down = lifecycle.probe_response(request_id);
    assert_eq!(
        shutting_down.lifecycle_state,
        DaemonLifecycleState::ShuttingDown
    );
    assert_eq!(shutting_down.health, DaemonHealthState::Unavailable);

    lifecycle.mark_stopped().expect("stop");
    let stopped = lifecycle.probe_response(request_id);
    assert_eq!(stopped.lifecycle_state, DaemonLifecycleState::Stopped);
    assert_eq!(stopped.health, DaemonHealthState::Unavailable);
}

#[test]
fn probe_metadata_rejects_empty_fields() {
    assert_eq!(
        ProbeMetadata::new("", "rev"),
        Err(LifecycleError::EmptyDaemonVersion)
    );
    assert_eq!(
        ProbeMetadata::new("0.0.0", " "),
        Err(LifecycleError::EmptyImplementationRevision)
    );
}
