//! Privileged Kernux daemon lifecycle foundation.
//!
//! This slice defines deterministic daemon lifecycle state and an owner-controlled
//! in-process shutdown handle. It intentionally does not expose IPC shutdown,
//! caller authentication, privileged operations, persistence, or remote transport.

#![forbid(unsafe_code)]

use kernux_contracts::{
    DaemonHealthState, DaemonLifecycleState, DaemonProbeResponse, KRP_SCHEMA_SHA256,
    ProtocolContract,
};
use std::fmt;
use std::sync::{
    Arc,
    atomic::{AtomicU8, Ordering},
};

mod transport;

pub use transport::{
    DEFAULT_ACCEPT_POLL_INTERVAL, DEFAULT_IO_TIMEOUT, DEFAULT_MAX_FRAME_BYTES, DaemonEndpoint,
    DaemonServer, ServerConfig, TransportError, probe_once,
};

const STARTING: u8 = 0;
const SERVING: u8 = 1;
const SHUTTING_DOWN: u8 = 2;
const STOPPED: u8 = 3;

/// Immutable daemon build/provenance metadata returned by the read-only probe.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProbeMetadata {
    daemon_version: String,
    implementation_revision: String,
}

impl ProbeMetadata {
    /// Create validated probe metadata.
    pub fn new(
        daemon_version: impl Into<String>,
        implementation_revision: impl Into<String>,
    ) -> Result<Self, LifecycleError> {
        let daemon_version = daemon_version.into();
        let implementation_revision = implementation_revision.into();
        if daemon_version.trim().is_empty() {
            return Err(LifecycleError::EmptyDaemonVersion);
        }
        if implementation_revision.trim().is_empty() {
            return Err(LifecycleError::EmptyImplementationRevision);
        }
        Ok(Self {
            daemon_version,
            implementation_revision,
        })
    }
}

/// Deterministic daemon lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleState {
    Starting,
    Serving,
    ShuttingDown,
    Stopped,
}

impl LifecycleState {
    fn from_raw(value: u8) -> Self {
        match value {
            STARTING => Self::Starting,
            SERVING => Self::Serving,
            SHUTTING_DOWN => Self::ShuttingDown,
            STOPPED => Self::Stopped,
            _ => unreachable!("lifecycle state is written only by this module"),
        }
    }

    fn raw(self) -> u8 {
        match self {
            Self::Starting => STARTING,
            Self::Serving => SERVING,
            Self::ShuttingDown => SHUTTING_DOWN,
            Self::Stopped => STOPPED,
        }
    }

    fn wire(self) -> DaemonLifecycleState {
        match self {
            Self::Starting => DaemonLifecycleState::Starting,
            Self::Serving => DaemonLifecycleState::Serving,
            Self::ShuttingDown => DaemonLifecycleState::ShuttingDown,
            Self::Stopped => DaemonLifecycleState::Stopped,
        }
    }

    fn health(self) -> DaemonHealthState {
        match self {
            Self::Serving => DaemonHealthState::Healthy,
            Self::Starting | Self::ShuttingDown | Self::Stopped => DaemonHealthState::Unavailable,
        }
    }
}

/// Lifecycle transition failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LifecycleError {
    EmptyDaemonVersion,
    EmptyImplementationRevision,
    InvalidTransition {
        from: LifecycleState,
        to: LifecycleState,
    },
}

impl fmt::Display for LifecycleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDaemonVersion => f.write_str("daemon version must not be empty"),
            Self::EmptyImplementationRevision => {
                f.write_str("implementation revision must not be empty")
            }
            Self::InvalidTransition { from, to } => {
                write!(f, "invalid daemon lifecycle transition: {from:?} -> {to:?}")
            }
        }
    }
}

impl std::error::Error for LifecycleError {}

#[derive(Debug)]
struct SharedState {
    state: AtomicU8,
}

/// Owner side of the daemon lifecycle.
///
/// The worker owns state-completion transitions; callers receive only the
/// separate ShutdownHandle for requesting shutdown.
#[derive(Debug)]
pub struct DaemonLifecycle {
    shared: Arc<SharedState>,
    metadata: ProbeMetadata,
}

/// In-process owner-controlled shutdown request handle.
///
/// This is intentionally not serializable and is not exposed over IPC.
#[derive(Debug, Clone)]
pub struct ShutdownHandle {
    shared: Arc<SharedState>,
}

impl DaemonLifecycle {
    /// Create a lifecycle in the Starting state and its owner shutdown handle.
    pub fn new(metadata: ProbeMetadata) -> (Self, ShutdownHandle) {
        let shared = Arc::new(SharedState {
            state: AtomicU8::new(STARTING),
        });
        (
            Self {
                shared: Arc::clone(&shared),
                metadata,
            },
            ShutdownHandle { shared },
        )
    }

    /// Return the current lifecycle state.
    pub fn state(&self) -> LifecycleState {
        LifecycleState::from_raw(self.shared.state.load(Ordering::Acquire))
    }

    /// Mark a successfully initialized daemon as accepting local work.
    pub fn mark_serving(&self) -> Result<(), LifecycleError> {
        self.transition(LifecycleState::Starting, LifecycleState::Serving)
    }

    /// Complete owner-controlled shutdown.
    pub fn mark_stopped(&self) -> Result<(), LifecycleError> {
        self.transition(LifecycleState::ShuttingDown, LifecycleState::Stopped)
    }

    /// Build a read-only health/version probe response from current state.
    pub fn probe_response(&self, request_id: impl Into<String>) -> DaemonProbeResponse {
        let state = self.state();
        DaemonProbeResponse {
            contract_version: ProtocolContract::Krp1,
            daemon_version: self.metadata.daemon_version.clone(),
            health: state.health(),
            implementation_revision: self.metadata.implementation_revision.clone(),
            lifecycle_state: state.wire(),
            request_id: request_id.into(),
            schema_sha256: KRP_SCHEMA_SHA256.to_owned(),
        }
    }

    fn transition(
        &self,
        expected: LifecycleState,
        next: LifecycleState,
    ) -> Result<(), LifecycleError> {
        self.shared
            .state
            .compare_exchange(
                expected.raw(),
                next.raw(),
                Ordering::AcqRel,
                Ordering::Acquire,
            )
            .map(|_| ())
            .map_err(|observed| LifecycleError::InvalidTransition {
                from: LifecycleState::from_raw(observed),
                to: next,
            })
    }
}

impl ShutdownHandle {
    /// Request shutdown without granting any IPC-visible authority.
    ///
    /// Returns true exactly once when this call transitions a starting or
    /// serving daemon into ShuttingDown. Later calls are idempotent.
    pub fn request_shutdown(&self) -> bool {
        loop {
            let current = self.shared.state.load(Ordering::Acquire);
            match current {
                STARTING | SERVING => {
                    if self
                        .shared
                        .state
                        .compare_exchange(
                            current,
                            SHUTTING_DOWN,
                            Ordering::AcqRel,
                            Ordering::Acquire,
                        )
                        .is_ok()
                    {
                        return true;
                    }
                }
                SHUTTING_DOWN | STOPPED => return false,
                _ => unreachable!("lifecycle state is written only by this module"),
            }
        }
    }
}
