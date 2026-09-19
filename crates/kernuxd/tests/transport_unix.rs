#![cfg(unix)]

use kernux_contracts::{
    DaemonHealthState, DaemonLifecycleState, DaemonProbeKind, DaemonProbeRequest, ProtocolContract,
};
use kernuxd::{
    DaemonEndpoint, DaemonServer, ExpectedPeerIdentity, LaunchNonce, MAX_AUTH_BOOTSTRAP_BYTES,
    ProbeMetadata, ServerConfig, SessionAuthConfig, TransportError, encode_auth_preamble,
    probe_once,
};
use std::{
    fs,
    io::{BufRead, BufReader, Read, Write},
    os::unix::{
        fs::{MetadataExt, PermissionsExt, symlink},
        net::{UnixListener, UnixStream},
    },
    path::{Path, PathBuf},
    process::{Command, Stdio},
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

fn launch_nonce() -> LaunchNonce {
    LaunchNonce::from_bytes([0x5a; 32])
}

fn current_euid() -> u32 {
    let probe_path = temp_runtime_dir("euid-probe");
    fs::write(&probe_path, b"").expect("create euid probe");
    let euid = fs::metadata(&probe_path)
        .expect("euid probe metadata")
        .uid();
    fs::remove_file(&probe_path).expect("remove euid probe");
    euid
}

fn auth_config() -> SessionAuthConfig {
    SessionAuthConfig::new(
        launch_nonce(),
        ExpectedPeerIdentity::unix(current_euid(), None),
    )
}

fn authenticate_raw(stream: &mut UnixStream) {
    stream
        .write_all(&encode_auth_preamble(&launch_nonce()))
        .expect("write auth preamble");
}

fn assert_connection_closes_without_response(mut stream: UnixStream) {
    let mut byte = [0_u8; 1];
    match stream.read(&mut byte) {
        Ok(0) => {}
        Ok(read) => panic!("unauthorized connection received {read} response bytes"),
        Err(error) => assert!(
            matches!(
                error.kind(),
                std::io::ErrorKind::ConnectionReset
                    | std::io::ErrorKind::BrokenPipe
                    | std::io::ErrorKind::NotConnected
                    | std::io::ErrorKind::ConnectionAborted
            ),
            "unauthorized connection failed for unexpected reason: {error}"
        ),
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
fn binary_rejects_invalid_bootstrap_before_listener_creation_without_secret_echo() {
    let secret = "9d".repeat(32);
    let malformed = format!("kxb/1 unix {secret} {} - extra\n", current_euid());
    let oversized = vec![b'x'; MAX_AUTH_BOOTSTRAP_BYTES + 1];
    let cases: Vec<(&str, Vec<u8>)> = vec![
        ("missing", Vec::new()),
        ("malformed", malformed.into_bytes()),
        ("oversized", oversized),
    ];

    for (label, input) in cases {
        let runtime_dir = temp_runtime_dir(&format!("bootstrap-{label}"));
        let mut child = Command::new(env!("CARGO_BIN_EXE_kernuxd"))
            .arg(&runtime_dir)
            .stdin(Stdio::piped())
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn kernuxd bootstrap probe");
        if !input.is_empty() {
            child
                .stdin
                .as_mut()
                .expect("bootstrap stdin")
                .write_all(&input)
                .expect("write bootstrap input");
        }
        drop(child.stdin.take());
        let output = child.wait_with_output().expect("collect kernuxd result");
        assert!(
            !output.status.success(),
            "invalid bootstrap must fail: {label}"
        );
        assert!(
            !runtime_dir.exists(),
            "listener runtime directory must not be created before bootstrap validates: {label}"
        );
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(
            !stderr.contains(&secret),
            "bootstrap error leaked launch nonce: {label}"
        );
    }
}

#[test]
fn startup_probe_shutdown_cleanup_and_restart_are_deterministic() {
    let runtime_dir = temp_runtime_dir("restart");
    let endpoint = DaemonEndpoint::unix_runtime_dir(&runtime_dir).expect("endpoint");
    let socket_path = endpoint.socket_path().to_owned();
    let config = short_config();

    let (server, shutdown) =
        DaemonServer::bind(endpoint.clone(), metadata(), auth_config(), config)
            .expect("bind daemon");
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
        &launch_nonce(),
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
        DaemonServer::bind(endpoint.clone(), metadata(), auth_config(), config)
            .expect("restart bind");
    let worker = thread::spawn(move || server.serve());
    let response = probe_once(
        &endpoint,
        &launch_nonce(),
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
        DaemonServer::bind(endpoint.clone(), metadata(), auth_config(), short_config())
            .expect("first bind");

    let second = DaemonServer::bind(endpoint, metadata(), auth_config(), short_config());
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
    let (server, _) = DaemonServer::bind(endpoint, metadata(), auth_config(), short_config())
        .expect("recover stale socket");
    drop(server);
    assert!(!socket_path.exists());

    fs::write(&socket_path, b"not a socket").expect("create unsafe file");
    let endpoint = DaemonEndpoint::unix_runtime_dir(&runtime_dir).expect("endpoint");
    let result = DaemonServer::bind(endpoint, metadata(), auth_config(), short_config());
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
    let result = DaemonServer::bind(endpoint, metadata(), auth_config(), short_config());
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
    let result = DaemonServer::bind(endpoint, metadata(), auth_config(), short_config());
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
        DaemonServer::bind(endpoint.clone(), metadata(), auth_config(), config)
            .expect("bind daemon");
    let worker = thread::spawn(move || server.serve());

    let mut malformed = connect_raw(&socket_path);
    authenticate_raw(&mut malformed);
    malformed.write_all(b"{\n").expect("send malformed");
    drop(malformed);

    let mut unknown = connect_raw(&socket_path);
    authenticate_raw(&mut unknown);
    unknown
        .write_all(
            br#"{"request_id":"01890f00-0000-7000-8000-000000000103","contract_version":"krp/1","probe":"health_version","extra":true}
"#,
        )
        .expect("send unknown field");
    drop(unknown);

    let mut version = connect_raw(&socket_path);
    authenticate_raw(&mut version);
    version
        .write_all(
            br#"{"request_id":"01890f00-0000-7000-8000-000000000104","contract_version":"krp/2","probe":"health_version"}
"#,
        )
        .expect("send unsupported version");
    drop(version);

    let mut oversized = connect_raw(&socket_path);
    authenticate_raw(&mut oversized);
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
        &launch_nonce(),
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
fn unauthenticated_and_wrong_nonce_callers_get_no_krp_response_and_daemon_survives() {
    let runtime_dir = temp_runtime_dir("unauthorized");
    let endpoint = DaemonEndpoint::unix_runtime_dir(&runtime_dir).expect("endpoint");
    let socket_path = endpoint.socket_path().to_owned();
    let config = short_config();
    let (server, shutdown) =
        DaemonServer::bind(endpoint.clone(), metadata(), auth_config(), config)
            .expect("bind daemon");
    let worker = thread::spawn(move || server.serve());
    wait_until(&socket_path, true);

    let payload =
        serde_json::to_vec(&request("01890f00-0000-7000-8000-000000000108")).expect("encode");
    let mut unauthenticated = connect_raw(&socket_path);
    unauthenticated
        .write_all(&payload)
        .expect("write unauthenticated KRP");
    unauthenticated.write_all(b"\n").expect("terminate KRP");
    unauthenticated.flush().expect("flush unauthenticated KRP");
    assert_connection_closes_without_response(unauthenticated);

    let mut wrong_nonce = connect_raw(&socket_path);
    wrong_nonce
        .write_all(&encode_auth_preamble(&LaunchNonce::from_bytes([0x6a; 32])))
        .expect("write wrong nonce");
    wrong_nonce
        .write_all(&payload)
        .expect("write KRP after wrong nonce");
    wrong_nonce.write_all(b"\n").expect("terminate KRP");
    wrong_nonce.flush().expect("flush wrong nonce KRP");
    assert_connection_closes_without_response(wrong_nonce);

    let response = probe_once(
        &endpoint,
        &launch_nonce(),
        &request("01890f00-0000-7000-8000-000000000109"),
        config,
    )
    .expect("daemon survives unauthorized callers");
    assert_eq!(response.health, DaemonHealthState::Healthy);

    assert!(shutdown.request_shutdown());
    worker.join().expect("worker join").expect("clean shutdown");
    fs::remove_dir(&runtime_dir).expect("remove runtime");
}

#[test]
fn actual_unix_peer_euid_mismatch_fails_closed() {
    let runtime_dir = temp_runtime_dir("wrong-euid");
    let endpoint = DaemonEndpoint::unix_runtime_dir(&runtime_dir).expect("endpoint");
    let socket_path = endpoint.socket_path().to_owned();
    let config = short_config();
    let actual_euid = current_euid();
    let wrong_euid = if actual_euid == u32::MAX {
        actual_euid - 1
    } else {
        actual_euid + 1
    };
    let auth = SessionAuthConfig::new(launch_nonce(), ExpectedPeerIdentity::unix(wrong_euid, None));
    let (server, shutdown) =
        DaemonServer::bind(endpoint.clone(), metadata(), auth, config).expect("bind daemon");
    let worker = thread::spawn(move || server.serve());
    wait_until(&socket_path, true);

    assert!(
        probe_once(
            &endpoint,
            &launch_nonce(),
            &request("01890f00-0000-7000-8000-000000000110"),
            config,
        )
        .is_err(),
        "peer euid mismatch must reject the session"
    );

    assert!(shutdown.request_shutdown());
    worker.join().expect("worker join").expect("clean shutdown");
    fs::remove_dir(&runtime_dir).expect("remove runtime");
}

#[cfg(target_os = "linux")]
#[test]
fn linux_peer_pid_binding_accepts_exact_pid_and_rejects_wrong_pid() {
    let runtime_dir = temp_runtime_dir("peer-pid");
    let endpoint = DaemonEndpoint::unix_runtime_dir(&runtime_dir).expect("endpoint");
    let socket_path = endpoint.socket_path().to_owned();
    let config = short_config();
    let euid = current_euid();
    let exact = SessionAuthConfig::new(
        launch_nonce(),
        ExpectedPeerIdentity::unix(euid, Some(std::process::id())),
    );
    let (server, shutdown) =
        DaemonServer::bind(endpoint.clone(), metadata(), exact, config).expect("bind daemon");
    let worker = thread::spawn(move || server.serve());
    wait_until(&socket_path, true);
    probe_once(
        &endpoint,
        &launch_nonce(),
        &request("01890f00-0000-7000-8000-000000000111"),
        config,
    )
    .expect("exact Linux peer pid authenticates");
    assert!(shutdown.request_shutdown());
    worker.join().expect("worker join").expect("clean shutdown");
    wait_until(&socket_path, false);

    let wrong_pid = std::process::id()
        .checked_add(1)
        .expect("test pid has headroom");
    let wrong = SessionAuthConfig::new(
        launch_nonce(),
        ExpectedPeerIdentity::unix(euid, Some(wrong_pid)),
    );
    let (server, shutdown) =
        DaemonServer::bind(endpoint.clone(), metadata(), wrong, config).expect("rebind daemon");
    let worker = thread::spawn(move || server.serve());
    assert!(
        probe_once(
            &endpoint,
            &launch_nonce(),
            &request("01890f00-0000-7000-8000-000000000112"),
            config,
        )
        .is_err(),
        "wrong Linux peer pid must reject the session"
    );
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
    let (server, shutdown) =
        DaemonServer::bind(endpoint, metadata(), auth_config(), config).expect("bind daemon");
    let worker = thread::spawn(move || server.serve());

    let first =
        serde_json::to_vec(&request("01890f00-0000-7000-8000-000000000106")).expect("encode first");
    let second = serde_json::to_vec(&request("01890f00-0000-7000-8000-000000000107"))
        .expect("encode second");
    let mut stream = connect_raw(&socket_path);
    authenticate_raw(&mut stream);
    let reader_stream = stream.try_clone().expect("clone client stream");
    let mut reader = BufReader::new(reader_stream);

    stream.write_all(&first).expect("write first");
    stream.write_all(b"\n").expect("newline first");
    stream.flush().expect("flush first request");

    let mut first_response = String::new();
    let first_bytes = reader
        .read_line(&mut first_response)
        .expect("read first response");
    assert!(first_bytes > 0, "first probe must receive one response");
    let response: kernux_contracts::DaemonProbeResponse =
        serde_json::from_str(first_response.trim_end()).expect("decode first response");
    assert_eq!(response.request_id, "01890f00-0000-7000-8000-000000000106");

    let second_write = (|| -> std::io::Result<()> {
        stream.write_all(&second)?;
        stream.write_all(b"\n")?;
        stream.flush()
    })();

    match second_write {
        Err(error) => assert!(
            matches!(
                error.kind(),
                std::io::ErrorKind::BrokenPipe
                    | std::io::ErrorKind::ConnectionReset
                    | std::io::ErrorKind::NotConnected
            ),
            "second request failed for unexpected reason: {error}"
        ),
        Ok(()) => {
            let mut second_response = String::new();
            match reader.read_line(&mut second_response) {
                Ok(0) => {}
                Ok(_) => panic!("connection produced a second response: {second_response}"),
                Err(error) => assert!(
                    matches!(
                        error.kind(),
                        std::io::ErrorKind::ConnectionReset
                            | std::io::ErrorKind::BrokenPipe
                            | std::io::ErrorKind::NotConnected
                            | std::io::ErrorKind::UnexpectedEof
                    ),
                    "connection failed for unexpected reason after first response: {error}"
                ),
            }
        }
    }

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
