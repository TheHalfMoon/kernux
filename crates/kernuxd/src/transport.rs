use crate::{DaemonLifecycle, LifecycleError, LifecycleState, ProbeMetadata, ShutdownHandle};
#[cfg(unix)]
use interprocess::local_socket::GenericFilePath;
#[cfg(windows)]
use interprocess::local_socket::GenericNamespaced;
use interprocess::local_socket::{
    Listener, ListenerNonblockingMode, ListenerOptions, Stream, prelude::*,
};
use kernux_contracts::{DaemonProbeRequest, DaemonProbeResponse};
use std::{
    fmt,
    io::{self, Read, Write},
    thread,
    time::{Duration, Instant},
};

#[cfg(unix)]
use std::{
    fs,
    os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt},
    path::{Path, PathBuf},
};

/// Maximum encoded JSON bytes accepted for one local control request or response.
pub const DEFAULT_MAX_FRAME_BYTES: usize = 4096;
/// Default per-connection receive/send timeout.
pub const DEFAULT_IO_TIMEOUT: Duration = Duration::from_secs(2);
/// Default delay between nonblocking accept polls.
pub const DEFAULT_ACCEPT_POLL_INTERVAL: Duration = Duration::from_millis(10);

/// Bounded synchronous server settings.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ServerConfig {
    pub max_frame_bytes: usize,
    pub io_timeout: Duration,
    pub accept_poll_interval: Duration,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            max_frame_bytes: DEFAULT_MAX_FRAME_BYTES,
            io_timeout: DEFAULT_IO_TIMEOUT,
            accept_poll_interval: DEFAULT_ACCEPT_POLL_INTERVAL,
        }
    }
}

impl ServerConfig {
    fn validate(self) -> Result<Self, TransportError> {
        if self.max_frame_bytes == 0 {
            return Err(TransportError::InvalidConfiguration(
                "max_frame_bytes must be greater than zero",
            ));
        }
        if self.io_timeout.is_zero() {
            return Err(TransportError::InvalidConfiguration(
                "io_timeout must be greater than zero",
            ));
        }
        if self.accept_poll_interval.is_zero() {
            return Err(TransportError::InvalidConfiguration(
                "accept_poll_interval must be greater than zero",
            ));
        }
        Ok(self)
    }
}

/// Platform-local daemon endpoint.
///
/// Unix always uses a filesystem Unix-domain socket beneath a private runtime
/// directory. Windows always uses a local namespaced pipe token and rejects
/// remote-host or path syntax.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DaemonEndpoint {
    #[cfg(unix)]
    runtime_dir: PathBuf,
    #[cfg(unix)]
    socket_path: PathBuf,
    #[cfg(windows)]
    pipe_name: String,
}

impl DaemonEndpoint {
    /// Build the Unix endpoint `<runtime_dir>/kernuxd.sock`.
    #[cfg(unix)]
    pub fn unix_runtime_dir(runtime_dir: impl Into<PathBuf>) -> Result<Self, TransportError> {
        let runtime_dir = runtime_dir.into();
        if runtime_dir.as_os_str().is_empty() {
            return Err(TransportError::UnsafeRuntimeDirectory(
                "runtime directory must not be empty",
            ));
        }
        let socket_path = runtime_dir.join("kernuxd.sock");
        Ok(Self {
            runtime_dir,
            socket_path,
        })
    }

    /// Build a Windows local named-pipe endpoint from a simple local token.
    #[cfg(windows)]
    pub fn windows_pipe(pipe_name: impl Into<String>) -> Result<Self, TransportError> {
        let pipe_name = pipe_name.into();
        validate_windows_pipe_token(&pipe_name)?;
        Ok(Self { pipe_name })
    }

    /// Return the filesystem socket path on Unix.
    #[cfg(unix)]
    pub fn socket_path(&self) -> &Path {
        &self.socket_path
    }

    /// Return the local named-pipe token on Windows.
    #[cfg(windows)]
    pub fn pipe_name(&self) -> &str {
        &self.pipe_name
    }

    fn bind_listener(&self) -> Result<Listener, TransportError> {
        #[cfg(unix)]
        {
            self.prepare_unix_endpoint()?;
            let name = self
                .socket_path
                .as_os_str()
                .to_fs_name::<GenericFilePath>()?;
            let listener = ListenerOptions::new()
                .name(name)
                .nonblocking(ListenerNonblockingMode::Accept)
                .reclaim_name(true)
                .create_sync()?;

            fs::set_permissions(&self.socket_path, fs::Permissions::from_mode(0o600))?;
            let socket_meta = fs::symlink_metadata(&self.socket_path)?;
            let dir_meta = fs::symlink_metadata(&self.runtime_dir)?;
            if !socket_meta.file_type().is_socket()
                || socket_meta.uid() != dir_meta.uid()
                || socket_meta.permissions().mode() & 0o077 != 0
            {
                return Err(TransportError::UnsafeEndpointPath(
                    "created Unix socket failed ownership or permission verification",
                ));
            }
            Ok(listener)
        }

        #[cfg(windows)]
        {
            let name = self.pipe_name.as_str().to_ns_name::<GenericNamespaced>()?;
            Ok(ListenerOptions::new()
                .name(name)
                .nonblocking(ListenerNonblockingMode::Accept)
                .create_sync()?)
        }
    }

    fn connect_stream(&self) -> Result<Stream, TransportError> {
        #[cfg(unix)]
        {
            let name = self
                .socket_path
                .as_os_str()
                .to_fs_name::<GenericFilePath>()?;
            Ok(Stream::connect(name)?)
        }

        #[cfg(windows)]
        {
            let name = self.pipe_name.as_str().to_ns_name::<GenericNamespaced>()?;
            Ok(Stream::connect(name)?)
        }
    }

    #[cfg(unix)]
    fn prepare_unix_endpoint(&self) -> Result<(), TransportError> {
        ensure_private_runtime_directory(&self.runtime_dir)?;
        let dir_meta = fs::symlink_metadata(&self.runtime_dir)?;

        match fs::symlink_metadata(&self.socket_path) {
            Ok(socket_meta) => {
                if socket_meta.file_type().is_symlink() || !socket_meta.file_type().is_socket() {
                    return Err(TransportError::UnsafeEndpointPath(
                        "pre-existing Unix endpoint is not a socket",
                    ));
                }
                if socket_meta.uid() != dir_meta.uid() {
                    return Err(TransportError::UnsafeEndpointPath(
                        "pre-existing Unix socket owner differs from runtime directory owner",
                    ));
                }

                let name = self
                    .socket_path
                    .as_os_str()
                    .to_fs_name::<GenericFilePath>()?;
                match Stream::connect(name) {
                    Ok(_) => return Err(TransportError::EndpointInUse),
                    Err(error)
                        if matches!(
                            error.kind(),
                            io::ErrorKind::ConnectionRefused | io::ErrorKind::NotFound
                        ) =>
                    {
                        fs::remove_file(&self.socket_path)?;
                    }
                    Err(error) => return Err(TransportError::Io(error)),
                }
            }
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(TransportError::Io(error)),
        }
        Ok(())
    }
}

/// Synchronous local-only daemon server.
///
/// The listener accepts at most one probe request per connection. Malformed
/// client traffic closes that connection without granting authority or
/// terminating the daemon.
#[derive(Debug)]
pub struct DaemonServer {
    listener: Listener,
    endpoint: DaemonEndpoint,
    lifecycle: DaemonLifecycle,
    shutdown: ShutdownHandle,
    config: ServerConfig,
}

impl DaemonServer {
    /// Bind the platform-local endpoint and construct the daemon lifecycle.
    pub fn bind(
        endpoint: DaemonEndpoint,
        metadata: ProbeMetadata,
        config: ServerConfig,
    ) -> Result<(Self, ShutdownHandle), TransportError> {
        let config = config.validate()?;
        let listener = endpoint.bind_listener()?;
        let (lifecycle, shutdown) = DaemonLifecycle::new(metadata);
        Ok((
            Self {
                listener,
                endpoint,
                lifecycle,
                shutdown: shutdown.clone(),
                config,
            },
            shutdown,
        ))
    }

    /// Serve read-only health/version probes until the owner requests shutdown.
    pub fn serve(self) -> Result<(), TransportError> {
        if self.lifecycle.state() == LifecycleState::Starting {
            self.lifecycle.mark_serving()?;
        }

        loop {
            if self.lifecycle.state() == LifecycleState::ShuttingDown {
                break;
            }

            match self.listener.accept() {
                Ok(stream) => {
                    let _ = self.handle_connection(stream);
                }
                Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                    thread::sleep(self.config.accept_poll_interval);
                }
                Err(error) if error.kind() == io::ErrorKind::Interrupted => {}
                Err(error) => {
                    self.shutdown.request_shutdown();
                    let _ = self.lifecycle.mark_stopped();
                    return Err(TransportError::Io(error));
                }
            }
        }

        self.lifecycle.mark_stopped()?;
        let endpoint = self.endpoint.clone();
        drop(self.listener);
        verify_endpoint_released(&endpoint)?;
        Ok(())
    }

    fn handle_connection(&self, mut stream: Stream) -> Result<(), TransportError> {
        configure_stream(&stream, self.config.io_timeout)?;

        let frame = read_frame(
            &mut stream,
            self.config.max_frame_bytes,
            self.config.io_timeout,
        )?;
        let request: DaemonProbeRequest =
            serde_json::from_slice(&frame).map_err(TransportError::InvalidRequest)?;
        let response = self.lifecycle.probe_response(request.request_id);
        let payload = serde_json::to_vec(&response).map_err(TransportError::EncodeResponse)?;
        write_payload_frame(
            &mut stream,
            &payload,
            self.config.max_frame_bytes,
            self.config.io_timeout,
        )
    }
}

#[cfg(unix)]
fn verify_endpoint_released(endpoint: &DaemonEndpoint) -> Result<(), TransportError> {
    match fs::symlink_metadata(&endpoint.socket_path) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
        Ok(_) => Err(TransportError::UnsafeEndpointPath(
            "Unix socket path remains after listener shutdown",
        )),
        Err(error) => Err(TransportError::Io(error)),
    }
}

#[cfg(windows)]
fn verify_endpoint_released(_endpoint: &DaemonEndpoint) -> Result<(), TransportError> {
    // Windows named-pipe instances are kernel objects and have no filesystem
    // path that requires reclamation.
    Ok(())
}

/// Send one bounded read-only probe to a running local daemon.
pub fn probe_once(
    endpoint: &DaemonEndpoint,
    request: &DaemonProbeRequest,
    config: ServerConfig,
) -> Result<DaemonProbeResponse, TransportError> {
    let config = config.validate()?;
    let mut stream = endpoint.connect_stream()?;
    configure_stream(&stream, config.io_timeout)?;
    let payload = serde_json::to_vec(request).map_err(TransportError::EncodeResponse)?;
    write_payload_frame(
        &mut stream,
        &payload,
        config.max_frame_bytes,
        config.io_timeout,
    )?;

    let frame = read_frame(&mut stream, config.max_frame_bytes, config.io_timeout)?;
    serde_json::from_slice(&frame).map_err(TransportError::InvalidResponse)
}

#[cfg(unix)]
fn configure_stream(stream: &Stream, io_timeout: Duration) -> Result<(), TransportError> {
    stream.set_nonblocking(false)?;
    stream.set_recv_timeout(Some(io_timeout))?;
    stream.set_send_timeout(Some(io_timeout))?;
    Ok(())
}

#[cfg(windows)]
fn configure_stream(stream: &Stream, _io_timeout: Duration) -> Result<(), TransportError> {
    // interprocess synchronous Windows named pipes do not expose socket-style
    // I/O timeouts. Keep the handle nonblocking and enforce the same finite
    // deadline in the bounded read/write loops below.
    stream.set_nonblocking(true)?;
    Ok(())
}

fn read_frame(
    reader: &mut impl Read,
    max_frame_bytes: usize,
    io_timeout: Duration,
) -> Result<Vec<u8>, TransportError> {
    let deadline = io_deadline(io_timeout)?;
    let mut frame = Vec::with_capacity(max_frame_bytes.min(1024));
    let mut byte = [0_u8; 1];

    loop {
        match reader.read(&mut byte) {
            Ok(0) => {
                #[cfg(windows)]
                {
                    wait_for_io(deadline, "receiving local control frame")?;
                }
                #[cfg(unix)]
                {
                    return Err(TransportError::IncompleteFrame);
                }
            }
            Ok(_) if byte[0] == b'\n' => return Ok(frame),
            Ok(_) => {
                if frame.len() == max_frame_bytes {
                    return Err(TransportError::FrameTooLarge {
                        limit: max_frame_bytes,
                    });
                }
                frame.push(byte[0]);
            }
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {
                wait_for_io(deadline, "receiving local control frame")?;
            }
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                wait_for_io(deadline, "receiving local control frame")?;
            }
            Err(error) => return Err(TransportError::Io(error)),
        }
    }
}

fn write_payload_frame(
    writer: &mut impl Write,
    payload: &[u8],
    max_frame_bytes: usize,
    io_timeout: Duration,
) -> Result<(), TransportError> {
    if payload.len() > max_frame_bytes {
        return Err(TransportError::FrameTooLarge {
            limit: max_frame_bytes,
        });
    }

    let deadline = io_deadline(io_timeout)?;
    write_all_until(writer, payload, deadline)?;
    write_all_until(writer, b"\n", deadline)
}

fn write_all_until(
    writer: &mut impl Write,
    mut bytes: &[u8],
    deadline: Instant,
) -> Result<(), TransportError> {
    while !bytes.is_empty() {
        match writer.write(bytes) {
            Ok(0) => {
                #[cfg(windows)]
                {
                    wait_for_io(deadline, "sending local control frame")?;
                }
                #[cfg(unix)]
                {
                    return Err(TransportError::Io(io::Error::new(
                        io::ErrorKind::WriteZero,
                        "local control frame write returned zero bytes",
                    )));
                }
            }
            Ok(written) => bytes = &bytes[written..],
            Err(error) if error.kind() == io::ErrorKind::Interrupted => {
                wait_for_io(deadline, "sending local control frame")?;
            }
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {
                wait_for_io(deadline, "sending local control frame")?;
            }
            Err(error) => return Err(TransportError::Io(error)),
        }
    }
    Ok(())
}

fn io_deadline(io_timeout: Duration) -> Result<Instant, TransportError> {
    Instant::now()
        .checked_add(io_timeout)
        .ok_or(TransportError::InvalidConfiguration(
            "io_timeout exceeds the supported deadline range",
        ))
}

fn wait_for_io(deadline: Instant, operation: &'static str) -> Result<(), TransportError> {
    let now = Instant::now();
    if now >= deadline {
        return Err(TransportError::Io(io::Error::new(
            io::ErrorKind::TimedOut,
            format!("timed out while {operation}"),
        )));
    }

    thread::sleep(
        deadline
            .saturating_duration_since(now)
            .min(Duration::from_millis(1)),
    );
    Ok(())
}

#[cfg(unix)]
fn ensure_private_runtime_directory(path: &Path) -> Result<(), TransportError> {
    let metadata = match fs::symlink_metadata(path) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => {
            fs::create_dir_all(path)?;
            fs::set_permissions(path, fs::Permissions::from_mode(0o700))?;
            fs::symlink_metadata(path)?
        }
        Err(error) => return Err(TransportError::Io(error)),
    };

    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        return Err(TransportError::UnsafeRuntimeDirectory(
            "runtime path must be a real directory",
        ));
    }
    if metadata.permissions().mode() & 0o077 != 0 {
        return Err(TransportError::UnsafeRuntimeDirectory(
            "runtime directory must not grant group or other permissions",
        ));
    }

    verify_runtime_directory_owner(path, metadata.uid())
}

#[cfg(unix)]
fn verify_runtime_directory_owner(path: &Path, directory_uid: u32) -> Result<(), TransportError> {
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| TransportError::UnsafeRuntimeDirectory("system clock predates Unix epoch"))?
        .as_nanos();
    let probe_path = path.join(format!(
        ".kernux-owner-probe-{}-{nonce}",
        std::process::id()
    ));
    let probe = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&probe_path)?;
    let probe_uid = probe.metadata()?.uid();
    drop(probe);
    fs::remove_file(&probe_path)?;

    if probe_uid != directory_uid {
        return Err(TransportError::UnsafeRuntimeDirectory(
            "runtime directory owner differs from the daemon process file owner",
        ));
    }
    Ok(())
}

#[cfg(windows)]
fn validate_windows_pipe_token(pipe_name: &str) -> Result<(), TransportError> {
    if pipe_name.is_empty() || pipe_name.len() > 128 {
        return Err(TransportError::InvalidPipeName);
    }
    if !pipe_name
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
    {
        return Err(TransportError::InvalidPipeName);
    }
    Ok(())
}

/// Local transport or framing failure.
#[derive(Debug)]
pub enum TransportError {
    Io(io::Error),
    Lifecycle(LifecycleError),
    InvalidConfiguration(&'static str),
    UnsafeRuntimeDirectory(&'static str),
    UnsafeEndpointPath(&'static str),
    EndpointInUse,
    #[cfg(windows)]
    InvalidPipeName,
    FrameTooLarge {
        limit: usize,
    },
    IncompleteFrame,
    InvalidRequest(serde_json::Error),
    InvalidResponse(serde_json::Error),
    EncodeResponse(serde_json::Error),
}

impl fmt::Display for TransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(error) => write!(f, "local transport I/O failed: {error}"),
            Self::Lifecycle(error) => write!(f, "daemon lifecycle failed: {error}"),
            Self::InvalidConfiguration(message) => {
                write!(f, "invalid server configuration: {message}")
            }
            Self::UnsafeRuntimeDirectory(message) => {
                write!(f, "unsafe runtime directory: {message}")
            }
            Self::UnsafeEndpointPath(message) => write!(f, "unsafe local endpoint: {message}"),
            Self::EndpointInUse => f.write_str("local daemon endpoint is already in use"),
            #[cfg(windows)]
            Self::InvalidPipeName => f.write_str("Windows pipe name must be a simple local token"),
            Self::FrameTooLarge { limit } => {
                write!(f, "local control frame exceeds {limit} bytes")
            }
            Self::IncompleteFrame => {
                f.write_str("local control frame ended before newline terminator")
            }
            Self::InvalidRequest(error) => write!(f, "invalid daemon probe request: {error}"),
            Self::InvalidResponse(error) => write!(f, "invalid daemon probe response: {error}"),
            Self::EncodeResponse(error) => write!(f, "cannot encode local control frame: {error}"),
        }
    }
}

impl std::error::Error for TransportError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(error) => Some(error),
            Self::Lifecycle(error) => Some(error),
            Self::InvalidRequest(error)
            | Self::InvalidResponse(error)
            | Self::EncodeResponse(error) => Some(error),
            Self::InvalidConfiguration(_)
            | Self::UnsafeRuntimeDirectory(_)
            | Self::UnsafeEndpointPath(_)
            | Self::EndpointInUse
            | Self::FrameTooLarge { .. }
            | Self::IncompleteFrame => None,
            #[cfg(windows)]
            Self::InvalidPipeName => None,
        }
    }
}

impl From<io::Error> for TransportError {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<LifecycleError> for TransportError {
    fn from(value: LifecycleError) -> Self {
        Self::Lifecycle(value)
    }
}
