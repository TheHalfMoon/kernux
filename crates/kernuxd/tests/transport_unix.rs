#![cfg(unix)]

use kernux_contracts::{
    DaemonHealthState, DaemonLifecycleState, DaemonProbeKind, DaemonProbeRequest, ProtocolContract,
};
use kernuxd::{
    DaemonEndpoint, DaemonServer, ProbeMetadata, ServerConfig, TransportError, probe_once,
};
use std::{
    fs,
    io::{Read, Write},
    os::unix::{
        fs::{PermissionsExt, symlink},
        net::{UnixListener, UnixStream},
    },
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    thread,
    time::{Duration, Instant},
};

static NEXT_TEMP_ID: AtomicU64 = AtomicU64::new(0);

fn temp_runtime_dir(label: &str) -> PathBuf {
    let id = NEXT_TEMP_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("kernuxd-{label}-{}-{id}", std::process::id()))
}

fn metadata() -> ProbeMetadata {
    ProbeMetadata::new(
        env!("CARGO_PKG_VERSION"),
        "2222222222222222222222222222222222222222",
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
        io_timeout: Duration::from_millis(150),
        accept_poll_interval: Duration::from_millis(5),
    }
}

fn wait_until(path: &Path, exists: bool) {
    let deadline = Instant::now() + Duration::from_secs(2);
    while path.exists() != exists {
        assert!(Instant::now() < deadline, "timed out waiting for {path:?}");
        thread::sleep(Duration::from_millis(5));
    }
}

fn connect_raw(path: &Path) -> UnixStream {
    let stream = UnixStream::connect(path).expect("connect to test daemon");
    stream
        .set_read_timeout(Some(Duration::from_secs(1)))
        .expect("set read timeout");
    stream
        .set_write_timeout(Some(Duration::from_secs(1)))
        .expect("set write timeout");
    stream
}

#[test]
fn startup_probe_shutdown_cleanup_and_restart_are_deterministic() {
    let runtime_dir = temp_runtime_dir("restart");
    let endpoint = DaemonEndpoint::unix_runtime_dir(&runtime_dir).expect("endpoint");
    let socket_path = endpoint.socket_path().to_owned();
    let config = short_config();

    let (server, shutdown) =
        DaemonServer::bind(endpoint.clone(), metadata(), config).expect("bind daemon");
    assert_eq!(
        fs::metadata(&runtime_dir)
            .expect("runtime metadata")
            .permissions()
            .mode()
            & 0o777,
        0o700
    );
    assert_eq!(
        fs::metadata(&socket_path)
            .expect("socket metadata")
            .permissions()
            .mode()
            & 0o777,
        0o600
    );

    let worker = thread::spawn(move || server.serve());
    wait_until(&socket_path, true);

    let response = probe_once(
        &endpoint,
        &request("01890f00-0000-7000-8000-000000000101"),
        config,
    )
    .expect("probe succeeds");
    assert_eq!(response.health, DaemonHealthState::Healthy);
    assert_eq!(response.lifecycle_state, DaemonLifecycleState::Serving);
    assert_eq!(response.contract_version, ProtocolContract::Krp1);
    assert_eq!(
        response.implementation_revision,
        "2222222222222222222222222222222222222222"
    );

    assert!(shutdown.request_shutdown());
    worker.join().expect("worker join").expect("clean shutdown");
    wait_until(&socket_path, false);

    let (server, shutdown) =
        DaemonServer::bind(endpoint.clone(), metadata(), config).expect("restart bind");
    let worker = thread::spawn(move || server.serve());
    let response = probe_once(
        &endpoint,
        &request("01890f00-0000-7000-8000-000000000102"),
        config,
    )
    .expect("probe after restart");
    assert_eq!(response.health, DaemonHealthState::Healthy);
    assert!(shutdown.request_shutdown());
    worker
        .join()
        .expect("worker join")
        .expect("clean restart shutdown");

    fs::remove_dir(&runtime_dir).expect("remove private runtime directory");
}

#[test]
fn second_bind_conflicts_without_displacing_live_listener() {
    let runtime_dir = temp_runtime_dir("collision");
    let endpoint = DaemonEndpoint::unix_runtime_dir(&runtime_dir).expect("endpoint");
    let (server, _) =
        DaemonServer::bind(endpoint.clone(), metadata(), short_config()).expect("first bind");

    let second = DaemonServer::bind(endpoint, metadata(), short_config());
    assert!(matches!(second, Err(TransportError::EndpointInUse)));

    drop(server);
    fs::remove_dir(&runtime_dir).expect("cleanup runtime directory");
}

#[test]
fn verified_stale_socket_is_recovered_but_non_socket_path_is_rejected() {
    let runtime_dir = temp_runtime_dir("stale");
    fs::create_dir(&runtime_dir).expect("create runtime");
    fs::set_permissions(&runtime_dir, fs::Permissions::from_mode(0o700)).expect("private mode");
    let socket_path = runtime_dir.join("kernuxd.sock");

    let stale = UnixListener::bind(&socket_path).expect("create stale socket");
    drop(stale);
    assert!(socket_path.exists());

    let endpoint = DaemonEndpoint::unix_runtime_dir(&runtime_dir).expect("endpoint");
    let (server, _) =
        DaemonServer::bind(endpoint, metadata(), short_config()).expect("recover stale socket");
    drop(server);
    assert!(!socket_path.exists());

    fs::write(&socket_path, b"not a socket").expect("create unsafe file");
    let endpoint = DaemonEndpoint::unix_runtime_dir(&runtime_dir).expect("endpoint");
    let result = DaemonServer::bind(endpoint, metadata(), short_config());
    assert!(matches!(result, Err(TransportError::UnsafeEndpointPath(_))));

    fs::remove_file(&socket_path).expect("remove unsafe path");
    fs::remove_dir(&runtime_dir).expect("remove runtime");
}

#[test]
fn non_private_or_symlink_runtime_directory_fails_closed() {
    let runtime_dir = temp_runtime_dir("mode");
    fs::create_dir(&runtime_dir).expect("create runtime");
    fs::set_permissions(&runtime_dir, fs::Permissions::from_mode(0o755)).expect("public mode");

    let endpoint = DaemonEndpoint::unix_runtime_dir(&runtime_dir).expect("endpoint");
    let result = DaemonServer::bind(endpoint, metadata(), short_config());
    assert!(matches!(
        result,
        Err(TransportError::UnsafeRuntimeDirectory(_))
    ));

    fs::set_permissions(&runtime_dir, fs::Permissions::from_mode(0o700)).expect("private mode");
    fs::remove_dir(&runtime_dir).expect("remove runtime");

    let real_dir = temp_runtime_dir("symlink-target");
    let link_dir = temp_runtime_dir("symlink");
    fs::create_dir(&real_dir).expect("create symlink target");
    fs::set_permissions(&real_dir, fs::Permissions::from_mode(0o700)).expect("private target");
    symlink(&real_dir, &link_dir).expect("create runtime symlink");

    let endpoint = DaemonEndpoint::unix_runtime_dir(&link_dir).expect("endpoint");
    let result = DaemonServer::bind(endpoint, metadata(), short_config());
    assert!(matches!(
        result,
        Err(TransportError::UnsafeRuntimeDirectory(_))
    ));

    fs::remove_file(&link_dir).expect("remove runtime symlink");
    fs::remove_dir(&real_dir).expect("remove symlink target");
}

#[test]
fn malformed_oversized_unknown_version_and_idle_clients_do_not_kill_server() {
    let runtime_dir = temp_runtime_dir("adversarial");
    let endpoint = DaemonEndpoint::unix_runtime_dir(&runtime_dir).expect("endpoint");
    let socket_path = endpoint.socket_path().to_owned();
    let config = short_config();
    let (server, shutdown) =
        DaemonServer::bind(endpoint.clone(), metadata(), config).expect("bind daemon");
    let worker = thread::spawn(move || server.serve());

    let mut malformed = connect_raw(&socket_path);
    malformed.write_all(b"{\n").expect("send malformed");
    drop(malformed);

    let mut unknown = connect_raw(&socket_path);
    unknown
        .write_all(
            br#"{"request_id":"01890f00-0000-7000-8000-000000000103","contract_version":"krp/1","probe":"health_version","extra":true}
"#,
        )
        .expect("send unknown field");
    drop(unknown);

    let mut version = connect_raw(&socket_path);
    version
        .write_all(
            br#"{"request_id":"01890f00-0000-7000-8000-000000000104","contract_version":"krp/2","probe":"health_version"}
"#,
        )
        .expect("send unsupported version");
    drop(version);

    let mut oversized = connect_raw(&socket_path);
    oversized
        .write_all(&vec![b'x'; config.max_frame_bytes + 1])
        .expect("send oversized frame");
    oversized
        .write_all(b"\n")
        .expect("terminate oversized frame");
    drop(oversized);

    let idle = connect_raw(&socket_path);
    thread::sleep(config.io_timeout + Duration::from_millis(75));
    drop(idle);

    let response = probe_once(
        &endpoint,
        &request("01890f00-0000-7000-8000-000000000105"),
        config,
    )
    .expect("server survives invalid clients");
    assert_eq!(response.health, DaemonHealthState::Healthy);

    assert!(shutdown.request_shutdown());
    worker.join().expect("worker join").expect("clean shutdown");
    fs::remove_dir(&runtime_dir).expect("remove runtime");
}

#[test]
fn connection_processes_only_one_probe_then_closes() {
    let runtime_dir = temp_runtime_dir("one-request");
    let endpoint = DaemonEndpoint::unix_runtime_dir(&runtime_dir).expect("endpoint");
    let socket_path = endpoint.socket_path().to_owned();
    let config = short_config();
    let (server, shutdown) = DaemonServer::bind(endpoint, metadata(), config).expect("bind daemon");
    let worker = thread::spawn(move || server.serve());

    let first =
        serde_json::to_vec(&request("01890f00-0000-7000-8000-000000000106")).expect("encode first");
    let second = serde_json::to_vec(&request("01890f00-0000-7000-8000-000000000107"))
        .expect("encode second");
    let mut stream = connect_raw(&socket_path);
    stream.write_all(&first).expect("write first");
    stream.write_all(b"\n").expect("newline first");
    stream.write_all(&second).expect("write second");
    stream.write_all(b"\n").expect("newline second");

    let mut received = String::new();
    stream
        .read_to_string(&mut received)
        .expect("read response and EOF");
    let lines: Vec<_> = received.lines().collect();
    assert_eq!(
        lines.len(),
        1,
        "only one request may be processed per connection"
    );

    assert!(shutdown.request_shutdown());
    worker.join().expect("worker join").expect("clean shutdown");
    fs::remove_dir(&runtime_dir).expect("remove runtime");
}

#[test]
fn transport_source_contains_no_network_listener_primitives() {
    let transport = include_str!("../src/transport.rs");
    let binary = include_str!("../src/main.rs");
    for forbidden in [
        "TcpListener",
        "TcpStream",
        "UdpSocket",
        "std::net",
        "http://",
        "https://",
    ] {
        assert!(
            !transport.contains(forbidden) && !binary.contains(forbidden),
            "network primitive leaked into kernuxd transport: {forbidden}"
        );
    }
}
