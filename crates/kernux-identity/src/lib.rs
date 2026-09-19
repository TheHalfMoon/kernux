//! Local installation identity primitives.
//!
//! This crate owns only in-memory local identity/key material for the active
//! SG-000020 frontier. Durable persistence, OS credential-store integration,
//! policy/Grant authority, and remote enrollment remain out of scope.

#![forbid(unsafe_code)]

use core::fmt;
use core::str::FromStr;

use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use zeroize::Zeroize;

const LOCAL_ID_BYTES: usize = 16;
const VERIFYING_KEY_BYTES: usize = 32;
const SECRET_KEY_BYTES: usize = 32;
const RECOVERY_SIGNATURE_BYTES: usize = 64;
const RECOVERY_DOMAIN: &[u8] = b"kernux.identity.recovery/v1\0";

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum IdentityError {
    EntropyUnavailable,
    InvalidEncoding,
    InvalidGeneration,
    GenerationOverflow,
    InvalidVerifyingKey,
    RecoveryMismatch,
}

impl fmt::Debug for IdentityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::EntropyUnavailable => "EntropyUnavailable",
            Self::InvalidEncoding => "InvalidEncoding",
            Self::InvalidGeneration => "InvalidGeneration",
            Self::GenerationOverflow => "GenerationOverflow",
            Self::InvalidVerifyingKey => "InvalidVerifyingKey",
            Self::RecoveryMismatch => "RecoveryMismatch",
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
            Self::RecoveryMismatch => "identity recovery material does not match",
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

pub struct RecoverySeed([u8; SECRET_KEY_BYTES]);

impl RecoverySeed {
    pub const fn new(bytes: [u8; SECRET_KEY_BYTES]) -> Self {
        Self(bytes)
    }
}

impl Zeroize for RecoverySeed {
    fn zeroize(&mut self) {
        self.0.zeroize();
    }
}

impl Drop for RecoverySeed {
    fn drop(&mut self) {
        self.zeroize();
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RecoveryReference {
    descriptor: IdentityPublicDescriptor,
    signature: [u8; RECOVERY_SIGNATURE_BYTES],
}

impl RecoveryReference {
    pub const fn descriptor(&self) -> IdentityPublicDescriptor {
        self.descriptor
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyRotation {
    previous: IdentityPublicDescriptor,
    current: IdentityPublicDescriptor,
}

impl KeyRotation {
    pub const fn previous(&self) -> IdentityPublicDescriptor {
        self.previous
    }

    pub const fn current(&self) -> IdentityPublicDescriptor {
        self.current
    }
}

pub struct LocalIdentity {
    installation_id: InstallationId,
    generation: KeyGeneration,
    signing_key: SigningKey,
    recovery_reference: RecoveryReference,
}

impl LocalIdentity {
    pub fn generate() -> Result<Self, IdentityError> {
        let installation_id = InstallationId::generate()?;
        let generation = KeyGeneration::INITIAL;
        let signing_key = generate_signing_key()?;
        let descriptor = public_descriptor(installation_id, generation, &signing_key);
        let recovery_reference = build_recovery_reference(descriptor, &signing_key);
        Ok(Self {
            installation_id,
            generation,
            signing_key,
            recovery_reference,
        })
    }

    pub fn descriptor(&self) -> IdentityPublicDescriptor {
        public_descriptor(self.installation_id, self.generation, &self.signing_key)
    }

    pub const fn recovery_reference(&self) -> RecoveryReference {
        self.recovery_reference
    }

    pub fn recover(
        recovery_seed: RecoverySeed,
        reference: RecoveryReference,
    ) -> Result<Self, IdentityError> {
        let signing_key = SigningKey::from_bytes(&recovery_seed.0);
        let descriptor = public_descriptor(
            reference.descriptor.installation_id,
            reference.descriptor.generation,
            &signing_key,
        );
        if descriptor.verifying_key != reference.descriptor.verifying_key {
            return Err(IdentityError::RecoveryMismatch);
        }

        let signature = Signature::from_bytes(&reference.signature);
        signing_key
            .verifying_key()
            .verify_strict(&recovery_message(reference.descriptor), &signature)
            .map_err(|_| IdentityError::RecoveryMismatch)?;

        Ok(Self {
            installation_id: reference.descriptor.installation_id,
            generation: reference.descriptor.generation,
            signing_key,
            recovery_reference: reference,
        })
    }

    pub fn rotate(&mut self) -> Result<KeyRotation, IdentityError> {
        let next_generation = self.generation.checked_next()?;
        let replacement = generate_signing_key()?;
        let previous = self.descriptor();
        let current = public_descriptor(self.installation_id, next_generation, &replacement);
        let recovery_reference = build_recovery_reference(current, &replacement);

        self.signing_key = replacement;
        self.generation = next_generation;
        self.recovery_reference = recovery_reference;

        Ok(KeyRotation { previous, current })
    }
}

fn public_descriptor(
    installation_id: InstallationId,
    generation: KeyGeneration,
    signing_key: &SigningKey,
) -> IdentityPublicDescriptor {
    IdentityPublicDescriptor {
        installation_id,
        generation,
        verifying_key: DeviceVerifyingKey::from_verifying_key(signing_key.verifying_key()),
    }
}

fn build_recovery_reference(
    descriptor: IdentityPublicDescriptor,
    signing_key: &SigningKey,
) -> RecoveryReference {
    let signature = signing_key.sign(&recovery_message(descriptor)).to_bytes();
    RecoveryReference {
        descriptor,
        signature,
    }
}

fn recovery_message(descriptor: IdentityPublicDescriptor) -> Vec<u8> {
    let mut message = Vec::with_capacity(
        RECOVERY_DOMAIN.len() + LOCAL_ID_BYTES + core::mem::size_of::<u32>() + VERIFYING_KEY_BYTES,
    );
    message.extend_from_slice(RECOVERY_DOMAIN);
    message.extend_from_slice(descriptor.installation_id.as_bytes());
    message.extend_from_slice(&descriptor.generation.get().to_be_bytes());
    message.extend_from_slice(descriptor.verifying_key.as_bytes());
    message
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
        let descriptor = public_descriptor(installation_id, KeyGeneration::INITIAL, &signing_key);
        let recovery_reference = build_recovery_reference(descriptor, &signing_key);
        let identity = LocalIdentity {
            installation_id,
            generation: KeyGeneration::INITIAL,
            signing_key,
            recovery_reference,
        };

        let descriptor = identity.descriptor();
        assert_eq!(descriptor.installation_id(), installation_id);
        assert_eq!(descriptor.generation(), KeyGeneration::INITIAL);
        assert_eq!(descriptor.verifying_key(), expected);
    }

    #[test]
    fn rotation_replaces_secret_key_and_new_key_signs_and_verifies() {
        use ed25519_dalek::{Signer, Verifier};

        let mut identity = LocalIdentity::generate().expect("OS entropy");
        let previous_key = identity.signing_key.verifying_key();
        let previous_descriptor = identity.descriptor();

        let rotation = identity.rotate().expect("rotation succeeds");
        let current_key = identity.signing_key.verifying_key();
        let message = b"kernux/identity/rotation-test/v1";
        let signature = identity.signing_key.sign(message);

        assert_eq!(rotation.previous(), previous_descriptor);
        assert_eq!(rotation.current(), identity.descriptor());
        assert_eq!(rotation.current().generation().get(), 2);
        assert_eq!(
            rotation.previous().installation_id(),
            rotation.current().installation_id()
        );
        assert_ne!(previous_key, current_key);
        current_key
            .verify(message, &signature)
            .expect("new key verifies");
        assert!(previous_key.verify(message, &signature).is_err());
    }

    #[test]
    fn rotation_overflow_fails_before_mutating_identity() {
        let installation_id = InstallationId::from_hex("00112233445566778899aabbccddeeff")
            .expect("valid installation id");
        let identity_key = SigningKey::from_bytes(&[0x24; SECRET_KEY_BYTES]);
        let generation = KeyGeneration::try_from(u32::MAX).expect("max generation is valid");
        let descriptor = public_descriptor(installation_id, generation, &identity_key);
        let recovery_reference = build_recovery_reference(descriptor, &identity_key);
        let mut identity = LocalIdentity {
            installation_id,
            generation,
            signing_key: identity_key,
            recovery_reference,
        };
        let before = identity.descriptor();

        assert_eq!(identity.rotate(), Err(IdentityError::GenerationOverflow));
        assert_eq!(identity.descriptor(), before);
    }

    #[test]
    fn recovery_seed_zeroizes_without_exposing_formatting_surface() {
        let mut seed = RecoverySeed::new([0x5a; SECRET_KEY_BYTES]);
        seed.zeroize();
        assert_eq!(seed.0, [0u8; SECRET_KEY_BYTES]);
    }

    #[test]
    fn exact_recovery_seed_restores_bound_identity() {
        let seed_bytes = [0x37; SECRET_KEY_BYTES];
        let installation_id = InstallationId::from_hex("102132435465768798a9bacbdcedfe0f")
            .expect("valid installation id");
        let generation = KeyGeneration::try_from(9).expect("positive generation");
        let signing_key = SigningKey::from_bytes(&seed_bytes);
        let descriptor = public_descriptor(installation_id, generation, &signing_key);
        let reference = build_recovery_reference(descriptor, &signing_key);

        let recovered = LocalIdentity::recover(RecoverySeed::new(seed_bytes), reference)
            .expect("matching recovery material");
        assert_eq!(recovered.descriptor(), descriptor);
        assert_eq!(recovered.recovery_reference(), reference);
    }

    #[test]
    fn recovery_rejects_wrong_seed_and_tampered_bound_metadata() {
        let seed_bytes = [0x37; SECRET_KEY_BYTES];
        let installation_id = InstallationId::from_hex("102132435465768798a9bacbdcedfe0f")
            .expect("valid installation id");
        let generation = KeyGeneration::try_from(9).expect("positive generation");
        let signing_key = SigningKey::from_bytes(&seed_bytes);
        let descriptor = public_descriptor(installation_id, generation, &signing_key);
        let reference = build_recovery_reference(descriptor, &signing_key);

        assert_eq!(
            LocalIdentity::recover(RecoverySeed::new([0x38; SECRET_KEY_BYTES]), reference)
                .map(|_| ()),
            Err(IdentityError::RecoveryMismatch)
        );

        let mut wrong_installation = reference;
        wrong_installation.descriptor.installation_id =
            InstallationId::from_hex("202132435465768798a9bacbdcedfe0f").expect("valid id");
        assert_eq!(
            LocalIdentity::recover(RecoverySeed::new(seed_bytes), wrong_installation).map(|_| ()),
            Err(IdentityError::RecoveryMismatch)
        );

        let mut wrong_generation = reference;
        wrong_generation.descriptor.generation = KeyGeneration::try_from(10).expect("positive");
        assert_eq!(
            LocalIdentity::recover(RecoverySeed::new(seed_bytes), wrong_generation).map(|_| ()),
            Err(IdentityError::RecoveryMismatch)
        );

        let other_key = SigningKey::from_bytes(&[0x39; SECRET_KEY_BYTES]);
        let mut wrong_key = reference;
        wrong_key.descriptor.verifying_key =
            DeviceVerifyingKey::from_verifying_key(other_key.verifying_key());
        assert_eq!(
            LocalIdentity::recover(RecoverySeed::new(seed_bytes), wrong_key).map(|_| ()),
            Err(IdentityError::RecoveryMismatch)
        );

        let mut wrong_signature = reference;
        wrong_signature.signature[0] ^= 0x01;
        assert_eq!(
            LocalIdentity::recover(RecoverySeed::new(seed_bytes), wrong_signature).map(|_| ()),
            Err(IdentityError::RecoveryMismatch)
        );
    }
}
