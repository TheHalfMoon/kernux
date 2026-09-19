//! Local session authentication primitives.
//!
//! The launch nonce is a short-lived owner-supplied secret. It is deliberately
//! not serializable and does not implement `Debug` or `Display` so ordinary
//! diagnostics cannot reveal it accidentally.

use std::{error::Error, fmt};

/// Exact launch-nonce size in bytes.
pub const LAUNCH_NONCE_BYTES: usize = 32;
/// Exact lowercase hexadecimal launch-nonce size on bootstrap/wire surfaces.
pub const LAUNCH_NONCE_HEX_BYTES: usize = LAUNCH_NONCE_BYTES * 2;
/// Version token for the private pre-KRP authentication preamble.
pub const AUTH_PROTOCOL_VERSION: &str = "kxa/1";
const AUTH_PREAMBLE_PREFIX: &[u8] = b"kxa/1 ";
/// Exact authentication preamble size: version + space + nonce hex + newline.
pub const AUTH_PREAMBLE_BYTES: usize = AUTH_PREAMBLE_PREFIX.len() + LAUNCH_NONCE_HEX_BYTES + 1;

/// A short-lived owner-supplied secret that authorizes one local launch domain.
#[derive(Clone)]
pub struct LaunchNonce([u8; LAUNCH_NONCE_BYTES]);

impl LaunchNonce {
    /// Construct a launch nonce from exactly 32 secret bytes.
    pub const fn from_bytes(bytes: [u8; LAUNCH_NONCE_BYTES]) -> Self {
        Self(bytes)
    }

    /// Parse the strict 64-byte lowercase hexadecimal external form.
    pub fn parse_lower_hex(value: &str) -> Result<Self, AuthError> {
        let raw = value.as_bytes();
        if raw.len() != LAUNCH_NONCE_HEX_BYTES {
            return Err(AuthError::InvalidNonceEncoding);
        }

        let mut decoded = [0_u8; LAUNCH_NONCE_BYTES];
        for (index, pair) in raw.as_chunks::<2>().0.iter().enumerate() {
            let high = decode_lower_hex_nibble(pair[0]).ok_or(AuthError::InvalidNonceEncoding)?;
            let low = decode_lower_hex_nibble(pair[1]).ok_or(AuthError::InvalidNonceEncoding)?;
            decoded[index] = (high << 4) | low;
        }
        Ok(Self(decoded))
    }

    pub(crate) fn encode_lower_hex(&self) -> [u8; LAUNCH_NONCE_HEX_BYTES] {
        const HEX: &[u8; 16] = b"0123456789abcdef";
        let mut encoded = [0_u8; LAUNCH_NONCE_HEX_BYTES];
        for (index, byte) in self.0.iter().copied().enumerate() {
            encoded[index * 2] = HEX[(byte >> 4) as usize];
            encoded[index * 2 + 1] = HEX[(byte & 0x0f) as usize];
        }
        encoded
    }

    /// Compare two well-formed fixed-size secrets without an early mismatch return.
    pub fn matches(&self, other: &Self) -> bool {
        let mut difference = 0_u8;
        for (left, right) in self.0.iter().zip(other.0.iter()) {
            difference |= left ^ right;
        }
        difference == 0
    }
}

fn decode_lower_hex_nibble(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        _ => None,
    }
}

/// Expected operating-system peer identity for one daemon launch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ExpectedPeerIdentity {
    #[cfg(unix)]
    euid: u32,
    #[cfg(unix)]
    pid: Option<u32>,
    #[cfg(windows)]
    pid: u32,
}

impl ExpectedPeerIdentity {
    /// Require the exact effective user ID and optionally the exact process ID on Unix.
    #[cfg(unix)]
    pub const fn unix(euid: u32, pid: Option<u32>) -> Self {
        Self { euid, pid }
    }

    /// Require the exact client process ID on Windows named pipes.
    #[cfg(windows)]
    pub const fn windows(pid: u32) -> Self {
        Self { pid }
    }
}

/// Non-secret normalized peer credentials observed from the local transport.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ObservedPeerIdentity {
    /// Effective user ID when the platform exposes one.
    pub euid: Option<u32>,
    /// Peer process ID when the platform exposes one.
    pub pid: Option<u32>,
}

/// Owner-side authentication policy bound to one daemon launch.
#[derive(Clone)]
pub struct SessionAuthConfig {
    launch_nonce: LaunchNonce,
    expected_peer: ExpectedPeerIdentity,
}

impl SessionAuthConfig {
    /// Bind one launch nonce to the expected local operating-system peer.
    pub const fn new(launch_nonce: LaunchNonce, expected_peer: ExpectedPeerIdentity) -> Self {
        Self {
            launch_nonce,
            expected_peer,
        }
    }
}

/// Stable fail-closed authentication errors that never contain secret material.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthError {
    /// The nonce external form was not exactly 64 lowercase hexadecimal bytes.
    InvalidNonceEncoding,
    /// The pre-KRP authentication frame was malformed or used another version.
    InvalidPreamble,
    /// The observed peer or launch nonce did not match the owner policy.
    Unauthorized,
    /// Required operating-system peer credentials were unavailable.
    PeerCredentialsUnavailable,
}

impl fmt::Display for AuthError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidNonceEncoding => "invalid launch nonce encoding",
            Self::InvalidPreamble => "invalid session authentication preamble",
            Self::Unauthorized => "local session authentication rejected",
            Self::PeerCredentialsUnavailable => "required local peer credentials unavailable",
        })
    }
}

impl Error for AuthError {}

/// Encode the exact private authentication preamble written before any KRP frame.
pub fn encode_auth_preamble(nonce: &LaunchNonce) -> [u8; AUTH_PREAMBLE_BYTES] {
    let mut frame = [0_u8; AUTH_PREAMBLE_BYTES];
    frame[..AUTH_PREAMBLE_PREFIX.len()].copy_from_slice(AUTH_PREAMBLE_PREFIX);
    let hex = nonce.encode_lower_hex();
    let start = AUTH_PREAMBLE_PREFIX.len();
    let end = start + LAUNCH_NONCE_HEX_BYTES;
    frame[start..end].copy_from_slice(&hex);
    frame[end] = b'\n';
    frame
}

/// Parse only the exact supported pre-KRP authentication preamble.
pub fn parse_auth_preamble(frame: &[u8]) -> Result<LaunchNonce, AuthError> {
    if frame.len() != AUTH_PREAMBLE_BYTES
        || !frame.starts_with(AUTH_PREAMBLE_PREFIX)
        || frame.last() != Some(&b'\n')
    {
        return Err(AuthError::InvalidPreamble);
    }
    let start = AUTH_PREAMBLE_PREFIX.len();
    let end = start + LAUNCH_NONCE_HEX_BYTES;
    let nonce_text =
        std::str::from_utf8(&frame[start..end]).map_err(|_| AuthError::InvalidPreamble)?;
    LaunchNonce::parse_lower_hex(nonce_text).map_err(|_| AuthError::InvalidPreamble)
}

/// Authorize one already-parsed authentication preamble against observed OS credentials.
pub fn authorize_session(
    config: &SessionAuthConfig,
    observed: ObservedPeerIdentity,
    presented_nonce: &LaunchNonce,
) -> Result<(), AuthError> {
    authorize_peer(config.expected_peer, observed)?;
    if !config.launch_nonce.matches(presented_nonce) {
        return Err(AuthError::Unauthorized);
    }
    Ok(())
}

#[cfg(unix)]
fn authorize_peer(
    expected: ExpectedPeerIdentity,
    observed: ObservedPeerIdentity,
) -> Result<(), AuthError> {
    let observed_euid = observed.euid.ok_or(AuthError::PeerCredentialsUnavailable)?;
    if observed_euid != expected.euid {
        return Err(AuthError::Unauthorized);
    }
    if let Some(expected_pid) = expected.pid {
        let observed_pid = observed.pid.ok_or(AuthError::PeerCredentialsUnavailable)?;
        if observed_pid != expected_pid {
            return Err(AuthError::Unauthorized);
        }
    }
    Ok(())
}

#[cfg(windows)]
fn authorize_peer(
    expected: ExpectedPeerIdentity,
    observed: ObservedPeerIdentity,
) -> Result<(), AuthError> {
    let observed_pid = observed.pid.ok_or(AuthError::PeerCredentialsUnavailable)?;
    if observed_pid != expected.pid {
        return Err(AuthError::Unauthorized);
    }
    Ok(())
}
