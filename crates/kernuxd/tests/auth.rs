use kernuxd::{
    AUTH_PREAMBLE_BYTES, AuthError, ExpectedPeerIdentity, LaunchNonce, ObservedPeerIdentity,
    SessionAuthConfig, authorize_session, encode_auth_preamble, parse_auth_preamble,
};

fn nonce(byte: u8) -> LaunchNonce {
    LaunchNonce::from_bytes([byte; 32])
}

#[test]
fn launch_nonce_external_form_is_exact_lowercase_hex() {
    let launch_nonce = LaunchNonce::from_bytes([
        0x00, 0x01, 0x0f, 0x10, 0x7f, 0x80, 0xab, 0xcd, 0xef, 0x55, 0xaa, 0x11, 0x22, 0x33, 0x44,
        0x66, 0x77, 0x88, 0x99, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x12, 0x23, 0x34, 0x45, 0x56, 0x67,
        0x78, 0x89,
    ]);
    let frame = encode_auth_preamble(&launch_nonce);
    assert_eq!(frame.len(), AUTH_PREAMBLE_BYTES);
    assert_eq!(&frame[..6], b"kxa/1 ");
    assert_eq!(frame.last(), Some(&b'\n'));
    let hex = std::str::from_utf8(&frame[6..70]).expect("wire form is ascii");
    assert_eq!(hex.len(), 64);
    assert!(
        hex.bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    );
    let parsed = parse_auth_preamble(&frame).expect("canonical frame parses");
    assert!(launch_nonce.matches(&parsed));
}

#[test]
fn nonce_parser_rejects_wrong_length_uppercase_and_non_hex() {
    assert!(matches!(
        LaunchNonce::parse_lower_hex("00"),
        Err(AuthError::InvalidNonceEncoding)
    ));
    assert!(matches!(
        LaunchNonce::parse_lower_hex(&"A".repeat(64)),
        Err(AuthError::InvalidNonceEncoding)
    ));
    assert!(matches!(
        LaunchNonce::parse_lower_hex(&"g".repeat(64)),
        Err(AuthError::InvalidNonceEncoding)
    ));
}

#[test]
fn preamble_parser_is_versioned_exact_and_closed() {
    let canonical = encode_auth_preamble(&nonce(0x42));
    for malformed in [
        canonical[..canonical.len() - 1].to_vec(),
        [b"kxa/2 ".as_slice(), &canonical[6..]].concat(),
        [b" kxa/1".as_slice(), &canonical[6..]].concat(),
        [canonical.as_slice(), b"x"].concat(),
    ] {
        assert!(matches!(
            parse_auth_preamble(&malformed),
            Err(AuthError::InvalidPreamble)
        ));
    }
}

#[cfg(unix)]
#[test]
fn unix_policy_requires_exact_euid_and_configured_pid() {
    let config = SessionAuthConfig::new(nonce(0x11), ExpectedPeerIdentity::unix(501, Some(7001)));
    let allowed = ObservedPeerIdentity {
        euid: Some(501),
        pid: Some(7001),
    };
    assert_eq!(authorize_session(&config, allowed, &nonce(0x11)), Ok(()));

    for denied in [
        ObservedPeerIdentity {
            euid: Some(502),
            pid: Some(7001),
        },
        ObservedPeerIdentity {
            euid: Some(501),
            pid: Some(7002),
        },
    ] {
        assert_eq!(
            authorize_session(&config, denied, &nonce(0x11)),
            Err(AuthError::Unauthorized)
        );
    }
    assert_eq!(
        authorize_session(
            &config,
            ObservedPeerIdentity {
                euid: Some(501),
                pid: None,
            },
            &nonce(0x11),
        ),
        Err(AuthError::PeerCredentialsUnavailable)
    );
}

#[cfg(unix)]
#[test]
fn unix_policy_allows_pid_to_be_unrequired_but_never_euid() {
    let config = SessionAuthConfig::new(nonce(0x22), ExpectedPeerIdentity::unix(501, None));
    assert_eq!(
        authorize_session(
            &config,
            ObservedPeerIdentity {
                euid: Some(501),
                pid: None,
            },
            &nonce(0x22),
        ),
        Ok(())
    );
    assert_eq!(
        authorize_session(
            &config,
            ObservedPeerIdentity {
                euid: None,
                pid: Some(99),
            },
            &nonce(0x22),
        ),
        Err(AuthError::PeerCredentialsUnavailable)
    );
}

#[cfg(windows)]
#[test]
fn windows_policy_requires_exact_peer_pid() {
    let config = SessionAuthConfig::new(nonce(0x33), ExpectedPeerIdentity::windows(7001));
    assert_eq!(
        authorize_session(
            &config,
            ObservedPeerIdentity {
                euid: None,
                pid: Some(7001),
            },
            &nonce(0x33),
        ),
        Ok(())
    );
    assert_eq!(
        authorize_session(
            &config,
            ObservedPeerIdentity {
                euid: None,
                pid: Some(7002),
            },
            &nonce(0x33),
        ),
        Err(AuthError::Unauthorized)
    );
    assert_eq!(
        authorize_session(
            &config,
            ObservedPeerIdentity {
                euid: None,
                pid: None
            },
            &nonce(0x33),
        ),
        Err(AuthError::PeerCredentialsUnavailable)
    );
}

#[test]
fn wrong_well_formed_nonce_is_rejected_without_secret_in_error() {
    #[cfg(unix)]
    let (config, observed) = (
        SessionAuthConfig::new(nonce(0x44), ExpectedPeerIdentity::unix(501, None)),
        ObservedPeerIdentity {
            euid: Some(501),
            pid: None,
        },
    );
    #[cfg(windows)]
    let (config, observed) = (
        SessionAuthConfig::new(nonce(0x44), ExpectedPeerIdentity::windows(7001)),
        ObservedPeerIdentity {
            euid: None,
            pid: Some(7001),
        },
    );

    let error =
        authorize_session(&config, observed, &nonce(0x45)).expect_err("wrong nonce rejects");
    assert_eq!(error, AuthError::Unauthorized);
    let message = error.to_string();
    assert!(!message.contains("44"));
    assert!(!message.contains("45"));
}

#[test]
fn source_keeps_nonce_non_debuggable_and_comparison_full_length() {
    let source = include_str!("../src/auth.rs");
    assert!(!source.contains("impl fmt::Debug for LaunchNonce"));
    assert!(!source.contains("impl fmt::Display for LaunchNonce"));
    assert!(!source.contains("Serialize for LaunchNonce"));
    assert!(source.contains("difference |= left ^ right;"));
    assert!(!source.contains("if left != right {\n                return false;"));
}
