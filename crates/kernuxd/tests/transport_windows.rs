#![cfg(windows)]

use interprocess::local_socket::{GenericNamespaced, Stream, prelude::*};
use kernux_contracts::{
    DaemonHealthState, DaemonLifecycleState, DaemonProbeKind, DaemonProbeRequest, ProtocolContract,
};
use kernuxd::{DaemonEndpoint, DaemonServer, ProbeMetadata, ServerConfig, probe_once};
use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        mpsc,
    },
    thread,
    time::{Duration, Instant},
};

static NEXT_PIPE_ID: AtomicU64 = AtomicU64::new(0);

fn pipe_name(label: &str) -> String {
    let id = NEXT_PIPE_ID.fetch_add(1, Ordering::Relaxed);
    format!("kernuxd-{label}-{}-{id}", std::process::id())
}

fn metadata() -> ProbeMetadata {
    ProbeMetadata::new(
        env!("CARGO_PKG_VERSION"),
        "3333333333333333333333333333333333333333",
    )
    .expect("fixture metadata must be valid")
}

fn request(id: &str) -> DaemonProbeRequest {
    DaemonProbeRequest {
        contract_version: ProtocolContract::Krp1,
        probe: DaemonProbeKind::HealthVersion,
        request_id: id.to_owned(),
    }
}

fn short_config() -> ServerConfig {
    ServerConfig {
        max_frame_bytes: 512,
        io_timeout: Duration::from_millis(250),
        accept_poll_interval: Duration::from_millis(5),
    }
}

#[test]
fn local_named_pipe_probe_shutdown_and_restart_are_deterministic() {
    let token = pipe_name("restart");
    let endpoint = DaemonEndpoint::windows_pipe(token.clone()).expect("local pipe endpoint");
    assert_eq!(endpoint.pipe_name(), token);

    let config = short_config();
    let (server, shutdown) =
        DaemonServer::bind(endpoint.clone(), metadata(), config).expect("bind daemon");
    let worker = thread::spawn(move || server.serve());

    let response = probe_once(
        &endpoint,
        &request("01890f00-0000-7000-8000-000000000201"),
        config,
    )
    .expect("Windows named-pipe probe succeeds");
    assert_eq!(response.health, DaemonHealthState::Healthy);
    assert_eq!(response.lifecycle_state, DaemonLifecycleState::Serving);
    assert_eq!(response.contract_version, ProtocolContract::Krp1);
    assert_eq!(
        response.implementation_revision,
        "3333333333333333333333333333333333333333"
    );

    assert!(shutdown.request_shutdown());
    worker.join().expect("worker join").expect("clean shutdown");

    let (server, shutdown) =
        DaemonServer::bind(endpoint.clone(), metadata(), config).expect("restart bind");
    let worker = thread::spawn(move || server.serve());
    let response = probe_once(
        &endpoint,
        &request("01890f00-0000-7000-8000-000000000202"),
        config,
    )
    .expect("probe after restart");
    assert_eq!(response.health, DaemonHealthState::Healthy);
    assert!(shutdown.request_shutdown());
    worker
        .join()
        .expect("worker join")
        .expect("clean restart shutdown");
}

#[test]
fn second_bind_conflicts_and_listener_release_allows_rebind() {
    let endpoint = DaemonEndpoint::windows_pipe(pipe_name("collision")).expect("endpoint");
    let (first, _) =
        DaemonServer::bind(endpoint.clone(), metadata(), short_config()).expect("first bind");

    let second = DaemonServer::bind(endpoint.clone(), metadata(), short_config());
    assert!(second.is_err(), "second named-pipe listener must conflict");

    drop(first);

    let third = DaemonServer::bind(endpoint, metadata(), short_config());
    assert!(
        third.is_ok(),
        "dropping the listener must release the pipe name"
    );
}

#[test]
fn remote_or_path_named_pipe_forms_are_rejected() {
    for invalid in [
        "",
        r"\\server\pipe\kernuxd",
        r"\\.\pipe\kernuxd",
        r"nested\pipe",
        "nested/pipe",
        "contains space",
    ] {
        assert!(
            DaemonEndpoint::windows_pipe(invalid).is_err(),
            "unsafe Windows pipe token was accepted: {invalid:?}"
        );
    }

    assert!(DaemonEndpoint::windows_pipe("kernuxd.local_1-test").is_ok());
}

#[test]
fn idle_named_pipe_client_cannot_block_owner_shutdown_past_io_deadline() {
    let endpoint = DaemonEndpoint::windows_pipe(pipe_name("idle")).expect("endpoint");
    let config = short_config();
    let (server, shutdown) =
        DaemonServer::bind(endpoint.clone(), metadata(), config).expect("bind daemon");

    let (done_tx, done_rx) = mpsc::channel();
    thread::spawn(move || {
        let _ = done_tx.send(server.serve());
    });

    let name = endpoint
        .pipe_name()
        .to_ns_name::<GenericNamespaced>()
        .expect("local namespaced pipe");
    let idle_client = Stream::connect(name).expect("connect idle client");
    thread::sleep(Duration::from_millis(25));

    let shutdown_started = Instant::now();
    assert!(shutdown.request_shutdown());
    let result = done_rx
        .recv_timeout(Duration::from_secs(2))
        .expect("owner shutdown must remain bounded with an idle client");
    let shutdown_elapsed = shutdown_started.elapsed();
    assert!(
        shutdown_elapsed >= Duration::from_millis(100),
        "idle client must exercise the finite I/O deadline instead of causing an immediate close: {shutdown_elapsed:?}"
    );
    result.expect("daemon must stop cleanly after idle-client timeout");

    drop(idle_client);
}
