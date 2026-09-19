//! Local installation identity primitives.
//!
//! This crate owns only in-memory local identity/key material for the active
//! SG-000020 frontier. Durable persistence, OS credential-store integration,
//! policy/Grant authority, and remote enrollment remain out of scope.

#![forbid(unsafe_code)]

use core::fmt;
use core::str::FromStr;

use ed25519_dalek::{SigningKey, VerifyingKey};
use zeroize::Zeroize;

const LOCAL_ID_BYTES: usize = 16;
const VERIFYING_KEY_BYTES: usize = 32;
const SECRET_KEY_BYTES: usize = 32;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IdentityError {
    EntropyUnavailable,
    InvalidEncoding,
    InvalidGeneration,
    GenerationOverflow,
    InvalidVerifyingKey,
}

impl fmt::Debug for IdentityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::EntropyUnavailable => "EntropyUnavailable",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidGeneration => "InvalidGeneration",
            Self::GenerationOverflow => "GenerationOverflow",
            Self::InvalidVerifyingKey => "InvalidVerifyingKey",
        })
    }
}

impl fmt::Display for IdentityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::EntropyUnavailable => "operating-system entropy is unavailable",
            Self::InvalidEncoding => "identity encoding is invalid",
            Self::InvalidGeneration => "identity key generation is invalid",
            Self::GenerationOverflow => "identity key generation cannot advance",
            Self::InvalidVerifyingKey => "identity verifying key is invalid",
        })
    }
}

impl std::error::Error for IdentityError {}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstallationId([u8; LOCAL_ID_BYTES]);

impl InstallationId {
    pub fn generate() -> Result<Self, IdentityError> {
        random_local_id().map(Self)
    }

    pub fn from_hex(value: &str) -> Result<Self, IdentityError> {
        parse_lower_hex::<LOCAL_ID_BYTES>(value).map(Self)
    }

    pub const fn as_bytes(&self) -> &[u8; LOCAL_ID_BYTES] {
        &self.0
    }

    pub fn to_hex(self) -> String {
        encode_lower_hex(&self.0)
    }
}

impl fmt::Display for InstallationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&encode_lower_hex(&self.0))
    }
}

impl fmt::Debug for InstallationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InstallationId({self})")
    }
}

impl FromStr for InstallationId {
    type Err = IdentityError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::from_hex(value)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstanceId([u8; LOCAL_ID_BYTES]);

impl InstanceId {
    pub fn generate() -> Result<Self, IdentityError> {
        random_local_id().map(Self)
    }

    pub fn from_hex(value: &str) -> Result<Self, IdentityError> {
        parse_lower_hex::<LOCAL_ID_BYTES>(value).map(Self)
    }

    pub const fn as_bytes(&self) -> &[u8; LOCAL_ID_BYTES] {
        &self.0
    }

    pub fn to_hex(self) -> String {
        encode_lower_hex(&self.0)
    }
}

impl fmt::Display for InstanceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&encode_lower_hex(&self.0))
    }
}

impl fmt::Debug for InstanceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "InstanceId({self})")
    }
}

impl FromStr for InstanceId {
    type Err = IdentityError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::from_hex(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct KeyGeneration(u32);

impl KeyGeneration {
    pub const INITIAL: Self = Self(1);

    pub const fn get(self) -> u32 {
        self.0
    }

    pub fn checked_next(self) -> Result<Self, IdentityError> {
        self.0
            .checked_add(1)
            .map(Self)
            .ok_or(IdentityError::GenerationOverflow)
    }
}

impl TryFrom<u32> for KeyGeneration {
    type Error = IdentityError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value == 0 {
            Err(IdentityError::InvalidGeneration)
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct DeviceVerifyingKey([u8; VERIFYING_KEY_BYTES]);

impl DeviceVerifyingKey {
    pub fn from_hex(value: &str) -> Result<Self, IdentityError> {
        let bytes = parse_lower_hex::<VERIFYING_KEY_BYTES>(value)?;
        VerifyingKey::from_bytes(&bytes).map_err(|_| IdentityError::InvalidVerifyingKey)?;
        Ok(Self(bytes))
    }

    pub const fn as_bytes(&self) -> &[u8; VERIFYING_KEY_BYTES] {
        &self.0
    }

    pub fn to_hex(self) -> String {
        encode_lower_hex(&self.0)
    }

    fn from_verifying_key(key: VerifyingKey) -> Self {
        Self(key.to_bytes())
    }
}

impl fmt::Display for DeviceVerifyingKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&encode_lower_hex(&self.0))
    }
}

impl fmt::Debug for DeviceVerifyingKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "DeviceVerifyingKey({self})")
    }
}

impl FromStr for DeviceVerifyingKey {
    type Err = IdentityError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::from_hex(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IdentityPublicDescriptor {
    installation_id: InstallationId,
    generation: KeyGeneration,
    verifying_key: DeviceVerifyingKey,
}

impl IdentityPublicDescriptor {
    pub const fn installation_id(&self) -> InstallationId {
        self.installation_id
    }

    pub const fn generation(&self) -> KeyGeneration {
        self.generation
    }

    pub const fn verifying_key(&self) -> DeviceVerifyingKey {
        self.verifying_key
    }
}

pub struct LocalIdentity {
    installation_id: InstallationId,
    generation: KeyGeneration,
    signing_key: SigningKey,
}

impl LocalIdentity {
    pub fn generate() -> Result<Self, IdentityError> {
        let installation_id = InstallationId::generate()?;
        let signing_key = generate_signing_key()?;
        Ok(Self {
            installation_id,
            generation: KeyGeneration::INITIAL,
            signing_key,
        })
    }

    pub fn descriptor(&self) -> IdentityPublicDescriptor {
        IdentityPublicDescriptor {
            installation_id: self.installation_id,
            generation: self.generation,
            verifying_key: DeviceVerifyingKey::from_verifying_key(self.signing_key.verifying_key()),
        }
    }
}

fn random_local_id() -> Result<[u8; LOCAL_ID_BYTES], IdentityError> {
    let mut bytes = [0u8; LOCAL_ID_BYTES];
    getrandom::fill(&mut bytes).map_err(|_| IdentityError::EntropyUnavailable)?;
    Ok(bytes)
}

fn generate_signing_key() -> Result<SigningKey, IdentityError> {
    let mut seed = [0u8; SECRET_KEY_BYTES];
    getrandom::fill(&mut seed).map_err(|_| IdentityError::EntropyUnavailable)?;
    let signing_key = SigningKey::from_bytes(&seed);
    seed.zeroize();
    Ok(signing_key)
}

fn parse_lower_hex<const N: usize>(value: &str) -> Result<[u8; N], IdentityError> {
    if value.len() != N * 2 || !value.as_bytes().iter().all(u8::is_ascii_hexdigit) {
        return Err(IdentityError::InvalidEncoding);
    }
    if value
        .as_bytes()
        .iter()
        .any(|byte| byte.is_ascii_uppercase())
    {
        return Err(IdentityError::InvalidEncoding);
    }

    let bytes = value.as_bytes();
    let mut output = [0u8; N];
    for (index, slot) in output.iter_mut().enumerate() {
        let high = decode_hex_nibble(bytes[index * 2])?;
        let low = decode_hex_nibble(bytes[index * 2 + 1])?;
        *slot = (high << 4) | low;
    }
    Ok(output)
}

fn decode_hex_nibble(byte: u8) -> Result<u8, IdentityError> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        _ => Err(IdentityError::InvalidEncoding),
    }
}

fn encode_lower_hex(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        output.push(HEX[(byte >> 4) as usize] as char);
        output.push(HEX[(byte & 0x0f) as usize] as char);
    }
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deterministic_identity_descriptor_matches_signing_key() {
        let installation_id = InstallationId::from_hex("00112233445566778899aabbccddeeff")
            .expect("valid installation id");
        let seed = [0x42; SECRET_KEY_BYTES];
        let signing_key = SigningKey::from_bytes(&seed);
        let expected = DeviceVerifyingKey::from_verifying_key(signing_key.verifying_key());
        let identity = LocalIdentity {
            installation_id,
            generation: KeyGeneration::INITIAL,
            signing_key,
        };

        let descriptor = identity.descriptor();
        assert_eq!(descriptor.installation_id(), installation_id);
        assert_eq!(descriptor.generation(), KeyGeneration::INITIAL);
        assert_eq!(descriptor.verifying_key(), expected);
    }
}
