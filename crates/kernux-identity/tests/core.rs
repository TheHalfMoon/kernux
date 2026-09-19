use kernux_identity::{
    DeviceVerifyingKey, IdentityError, InstallationId, InstanceId, KeyGeneration, LocalIdentity,
    SessionChallenge,
};

#[test]
fn installation_id_has_exact_strict_lowercase_hex_form() {
    let canonical = "00112233445566778899aabbccddeeff";
    let id = InstallationId::from_hex(canonical).expect("canonical id must parse");
    assert_eq!(id.to_string(), canonical);
    assert_eq!(id.to_hex(), canonical);
    assert_eq!(id.as_bytes().len(), 16);

    for invalid in [
        "00112233445566778899AABBCCDDEEFF",
        "00112233445566778899aabbccddeef",
        "00112233445566778899aabbccddeeff00",
        "00112233445566778899aabbccddeefg",
        " 00112233445566778899aabbccddeeff",
        "00112233445566778899aabbccddeeff\n",
    ] {
        assert_eq!(
            InstallationId::from_hex(invalid),
            Err(IdentityError::InvalidEncoding)
        );
    }
}

#[test]
fn instance_id_has_exact_strict_lowercase_hex_form() {
    let canonical = "ffeeddccbbaa99887766554433221100";
    let id = InstanceId::from_hex(canonical).expect("canonical id must parse");
    assert_eq!(id.to_string(), canonical);
    assert_eq!(id.as_bytes().len(), 16);
    assert_eq!(
        InstanceId::from_hex("FFEEDDCCBBAA99887766554433221100"),
        Err(IdentityError::InvalidEncoding)
    );
}

#[test]
fn key_generation_is_positive_checked_and_non_wrapping() {
    assert_eq!(KeyGeneration::INITIAL.get(), 1);
    assert_eq!(
        KeyGeneration::try_from(0),
        Err(IdentityError::InvalidGeneration)
    );
    assert_eq!(KeyGeneration::try_from(7).expect("positive").get(), 7);
    assert_eq!(
        KeyGeneration::try_from(u32::MAX)
            .expect("max is a valid current generation")
            .checked_next(),
        Err(IdentityError::GenerationOverflow)
    );
    assert_eq!(
        KeyGeneration::INITIAL
            .checked_next()
            .expect("generation 2")
            .get(),
        2
    );
}

#[test]
fn generated_ids_use_exact_canonical_external_forms() {
    let installation = InstallationId::generate().expect("OS entropy must be available");
    let instance = InstanceId::generate().expect("OS entropy must be available");

    let installation_text = installation.to_string();
    let instance_text = instance.to_string();
    assert_eq!(installation_text.len(), 32);
    assert_eq!(instance_text.len(), 32);
    assert!(
        installation_text
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    );
    assert!(
        instance_text
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    );
    assert_eq!(
        InstallationId::from_hex(&installation_text).unwrap(),
        installation
    );
    assert_eq!(InstanceId::from_hex(&instance_text).unwrap(), instance);
}

#[test]
fn generated_local_identity_exposes_only_public_descriptor_material() {
    let identity = LocalIdentity::generate().expect("OS entropy must be available");
    let descriptor = identity.descriptor();

    assert_eq!(descriptor.generation(), KeyGeneration::INITIAL);
    assert_eq!(descriptor.installation_id().to_string().len(), 32);
    let verifying_hex = descriptor.verifying_key().to_string();
    assert_eq!(verifying_hex.len(), 64);
    assert!(
        verifying_hex
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    );
    assert_eq!(
        DeviceVerifyingKey::from_hex(&verifying_hex).expect("generated key must be valid"),
        descriptor.verifying_key()
    );
}

#[test]
fn verifying_key_parser_rejects_noncanonical_or_invalid_input() {
    assert_eq!(
        DeviceVerifyingKey::from_hex("00"),
        Err(IdentityError::InvalidEncoding)
    );
    assert_eq!(
        DeviceVerifyingKey::from_hex(
            "FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF"
        ),
        Err(IdentityError::InvalidEncoding)
    );
    assert_eq!(
        DeviceVerifyingKey::from_hex(
            "0000000000000000000000000000000000000000000000000000000000000001"
        ),
        Err(IdentityError::InvalidVerifyingKey)
    );
}

#[test]
fn rotation_preserves_installation_id_and_advances_public_generation_once() {
    let mut identity = LocalIdentity::generate().expect("OS entropy must be available");
    let before = identity.descriptor();
    let rotation = identity.rotate().expect("rotation must succeed");

    assert_eq!(rotation.previous(), before);
    assert_eq!(rotation.current(), identity.descriptor());
    assert_eq!(
        rotation.current().installation_id(),
        before.installation_id()
    );
    assert_eq!(
        rotation.current().generation().get(),
        before.generation().get() + 1
    );
    assert_ne!(rotation.current().verifying_key(), before.verifying_key());
}

#[test]
fn source_keeps_recovery_seed_secret_zeroizing_and_signing_narrow() {
    let source = include_str!("../src/lib.rs");

    assert!(source.contains("impl Zeroize for RecoverySeed"));
    assert!(source.contains("impl Drop for RecoverySeed"));
    assert!(source.contains("self.zeroize();"));
    assert!(!source.contains("impl fmt::Debug for RecoverySeed"));
    assert!(!source.contains("impl fmt::Display for RecoverySeed"));
    assert!(!source.contains("pub fn secret"));
    assert!(!source.contains("pub fn signing_key"));
    assert!(!source.contains("pub fn private_key"));
    assert!(!source.contains("pub fn sign("));
    assert!(!source.contains("signing_key.to_bytes()"));
    assert!(!source.contains("signing_key.as_bytes()"));
    assert!(!source.contains("serde"));
}

#[test]
fn exact_session_binding_round_trips_and_exposes_only_public_material() {
    let identity = LocalIdentity::generate().expect("OS entropy must be available");
    let instance_id = InstanceId::generate().expect("OS entropy must be available");
    let challenge = SessionChallenge::new([0xa5; 32]);
    let binding = identity.bind_session(instance_id, challenge);

    binding.verify().expect("exact binding must verify");
    assert_eq!(binding.descriptor(), identity.descriptor());
    assert_eq!(binding.instance_id(), instance_id);
    assert_eq!(binding.challenge(), challenge);
    assert_eq!(binding.signature_bytes().len(), 64);
    assert_eq!(challenge.as_bytes(), &[0xa5; 32]);
}

#[test]
fn source_keeps_session_signing_domain_separated_and_narrow() {
    let source = include_str!("../src/lib.rs");

    assert!(source.contains("kernux.identity.session/v1\\0"));
    assert!(source.contains("pub fn bind_session("));
    assert!(source.contains("pub fn verify(&self) -> Result<(), IdentityError>"));
    assert!(!source.contains("pub fn sign("));
    assert!(!source.contains("pub fn signing_key"));
    assert!(!source.contains("pub fn private_key"));
    assert!(!source.contains("pub fn verify_message"));
}
