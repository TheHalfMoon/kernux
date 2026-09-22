//! Closed secret-provider identities, destination classes, and validated destination names.
//!
//! A [`ProviderId`] names one exact OS credential-store family the broker may
//! serve through a declared adapter. A [`DestinationClass`] names one exact
//! approved use shape. [`HostName`] and [`EnvName`] carry validated destination names.
//! All four are non-secret metadata and may appear in logs, events, prompts,
//! and diagnostics.
//!
//! Provider identity is metadata, not authority. Parsing or holding a
//! [`ProviderId`], [`DestinationClass`], [`HostName`], or [`EnvName`] grants
//! nothing and resolves nothing. Only broker admission under an exact Grant
//! can produce a bounded resolved use in a later SG-000026 slice. This module
//! performs no credential-store access, environment reads, spawns, network,
//! persistence, or logging.

use crate::SecretError;

/// Canonical identity for the macOS platform keychain adapter family.
pub const OS_MACOS_KEYCHAIN: &str = "os-macos-keychain";
/// Canonical identity for the Windows credential-manager adapter family.
pub const OS_WINDOWS_CREDENTIAL_MANAGER: &str = "os-windows-credential-manager";
/// Canonical identity for the Linux secret-service adapter family.
pub const OS_LINUX_SECRET_SERVICE: &str = "os-linux-secret-service";

/// Canonical destination class for bounded injection into one exact process environment.
pub const DESTINATION_PROCESS_ENV: &str = "process-env";
/// Canonical destination class for bounded use by one exact host command.
pub const DESTINATION_HOST_COMMAND: &str = "host-command";
/// Canonical destination class for brokered substitution on one exact egress request.
pub const DESTINATION_EGRESS_SUBSTITUTION: &str = "egress-substitution";

/// Maximum [`HostName`] length in bytes, including dots.
pub const MAX_HOST_NAME_BYTES: usize = 253;
/// Maximum single DNS label length in bytes.
pub const MAX_HOST_LABEL_BYTES: usize = 63;
/// Maximum [`EnvName`] length in bytes.
pub const MAX_ENV_NAME_BYTES: usize = 128;

/// Closed set of secret-provider identities.
///
/// Each variant names one exact OS credential-store family for a later broker
/// adapter slice. The set is closed: any other string fails closed with
/// [`SecretError::UnknownProvider`]. Possession of a parsed value authorizes
/// nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ProviderId {
    /// macOS platform keychain adapter family.
    OsMacosKeychain,
    /// Windows credential-manager adapter family.
    OsWindowsCredentialManager,
    /// Linux secret-service adapter family.
    OsLinuxSecretService,
}

impl ProviderId {
    /// Parse one exact canonical provider identity.
    ///
    /// Matching is exact and case-sensitive. Never echoes the input.
    pub fn parse(input: &str) -> Result<Self, SecretError> {
        match input {
            OS_MACOS_KEYCHAIN => Ok(Self::OsMacosKeychain),
            OS_WINDOWS_CREDENTIAL_MANAGER => Ok(Self::OsWindowsCredentialManager),
            OS_LINUX_SECRET_SERVICE => Ok(Self::OsLinuxSecretService),
            _ => Err(SecretError::UnknownProvider),
        }
    }

    /// The canonical identity string. Non-secret metadata.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::OsMacosKeychain => OS_MACOS_KEYCHAIN,
            Self::OsWindowsCredentialManager => OS_WINDOWS_CREDENTIAL_MANAGER,
            Self::OsLinuxSecretService => OS_LINUX_SECRET_SERVICE,
        }
    }

    /// Declared adapter capabilities for this provider identity.
    ///
    /// Declared willingness, not an observed operating system property. Store
    /// reach happens only in a later adapter slice through daemon-brokered authority.
    pub fn capabilities(&self) -> ProviderCapabilities {
        ProviderCapabilities::declared()
    }
}

impl core::fmt::Display for ProviderId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Declared willingness of one provider adapter to serve each destination class.
///
/// Pure metadata for exact capability matching at broker admission; grants nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ProviderCapabilities {
    /// May serve [`DestinationClass::ProcessEnv`] uses.
    pub supports_process_env: bool,
    /// May serve [`DestinationClass::HostCommand`] uses.
    pub supports_host_command: bool,
    /// May serve [`DestinationClass::EgressSubstitution`] uses.
    pub supports_egress_substitution: bool,
}

impl ProviderCapabilities {
    /// The single declared capability set for first-generation OS adapters.
    pub const fn declared() -> Self {
        Self {
            supports_process_env: true,
            supports_host_command: true,
            supports_egress_substitution: true,
        }
    }

    /// Whether this set serves one destination class. Pure: no I/O, no ambient state.
    pub const fn can_serve(&self, destination: DestinationClass) -> bool {
        match destination {
            DestinationClass::ProcessEnv => self.supports_process_env,
            DestinationClass::HostCommand => self.supports_host_command,
            DestinationClass::EgressSubstitution => self.supports_egress_substitution,
        }
    }
}

/// Closed set of approved secret-use destination shapes.
///
/// Each variant names one exact use shape the broker may admit. Any other
/// string fails closed with [`SecretError::InvalidDestination`]; possession
/// authorizes nothing.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DestinationClass {
    /// Bounded injection into one exact process environment.
    ProcessEnv,
    /// Bounded use by one exact host command.
    HostCommand,
    /// Brokered substitution on one exact egress request.
    EgressSubstitution,
}

impl DestinationClass {
    /// Parse one exact canonical destination class.
    ///
    /// Matching is exact and case-sensitive. Never echoes the input.
    pub fn parse(input: &str) -> Result<Self, SecretError> {
        match input {
            DESTINATION_PROCESS_ENV => Ok(Self::ProcessEnv),
            DESTINATION_HOST_COMMAND => Ok(Self::HostCommand),
            DESTINATION_EGRESS_SUBSTITUTION => Ok(Self::EgressSubstitution),
            _ => Err(SecretError::InvalidDestination),
        }
    }

    /// The canonical destination class string. Non-secret metadata.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ProcessEnv => DESTINATION_PROCESS_ENV,
            Self::HostCommand => DESTINATION_HOST_COMMAND,
            Self::EgressSubstitution => DESTINATION_EGRESS_SUBSTITUTION,
        }
    }
}

impl core::fmt::Display for DestinationClass {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// One validated DNS hostname for egress-shaped destinations.
///
/// Validation is syntactic and fail-closed: bare DNS names only (ASCII
/// letters/digits/hyphen, dot separators, at most 63 bytes per label and 253
/// total, no empty labels, no leading/trailing dots or hyphens, no ports,
/// schemes, userinfo, paths, underscores, whitespace, non-ASCII, or escapes).
/// IPv4-style numeric literals are rejected. Uppercase ASCII is normalized to
/// lowercase. The stored canonical spelling is non-secret metadata.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct HostName {
    canonical: String,
}

impl HostName {
    /// Parse and validate one bare DNS hostname.
    ///
    /// Never echoes the input. Performs no network access.
    pub fn parse(input: &str) -> Result<Self, SecretError> {
        if input.is_empty() || input.len() > MAX_HOST_NAME_BYTES {
            return Err(SecretError::InvalidDestination);
        }
        if !input
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'.')
        {
            return Err(SecretError::InvalidDestination);
        }
        if input.starts_with('.')
            || input.ends_with('.')
            || input.starts_with('-')
            || input.ends_with('-')
        {
            return Err(SecretError::InvalidDestination);
        }
        if input.contains("..") {
            return Err(SecretError::InvalidDestination);
        }
        let mut labels = 0;
        let mut all_numeric = true;
        for label in input.split('.') {
            labels += 1;
            if label.is_empty() || label.len() > MAX_HOST_LABEL_BYTES {
                return Err(SecretError::InvalidDestination);
            }
            let bytes = label.as_bytes();
            if !bytes[0].is_ascii_alphanumeric() || !bytes[bytes.len() - 1].is_ascii_alphanumeric()
            {
                return Err(SecretError::InvalidDestination);
            }
            if !label
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-')
            {
                return Err(SecretError::InvalidDestination);
            }
            if !label.bytes().all(|b| b.is_ascii_digit()) {
                all_numeric = false;
            }
        }
        if labels == 0 || all_numeric {
            return Err(SecretError::InvalidDestination);
        }
        Ok(Self {
            canonical: input.to_ascii_lowercase(),
        })
    }

    /// The validated canonical hostname. Non-secret metadata.
    pub fn as_str(&self) -> &str {
        &self.canonical
    }
}

impl core::fmt::Debug for HostName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "HostName({})", self.canonical)
    }
}

impl core::fmt::Display for HostName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.canonical)
    }
}

/// One validated process-environment variable name.
///
/// Validation is syntactic and fail-closed: one to 128 bytes, ASCII letters,
/// digits, and underscore only, starting with a letter or underscore; matching
/// is case-sensitive. The stored spelling is non-secret metadata, never a value.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct EnvName {
    canonical: String,
}

impl EnvName {
    /// Parse and validate one environment-variable name.
    ///
    /// Never echoes the input. Reads no process environment.
    pub fn parse(input: &str) -> Result<Self, SecretError> {
        if input.is_empty() || input.len() > MAX_ENV_NAME_BYTES {
            return Err(SecretError::InvalidDestination);
        }
        let mut bytes = input.bytes();
        match bytes.next() {
            Some(first) if first.is_ascii_alphabetic() || first == b'_' => {}
            _ => return Err(SecretError::InvalidDestination),
        }
        if !input
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_')
        {
            return Err(SecretError::InvalidDestination);
        }
        Ok(Self {
            canonical: input.to_owned(),
        })
    }

    /// The validated variable name. Non-secret metadata.
    pub fn as_str(&self) -> &str {
        &self.canonical
    }
}

impl core::fmt::Debug for EnvName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "EnvName({})", self.canonical)
    }
}

impl core::fmt::Display for EnvName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.canonical)
    }
}

#[cfg(test)]
mod provider_tests {
    use super::{
        DESTINATION_EGRESS_SUBSTITUTION, DESTINATION_HOST_COMMAND, DESTINATION_PROCESS_ENV,
        DestinationClass, EnvName, HostName, MAX_ENV_NAME_BYTES, MAX_HOST_LABEL_BYTES,
        MAX_HOST_NAME_BYTES, OS_LINUX_SECRET_SERVICE, OS_MACOS_KEYCHAIN,
        OS_WINDOWS_CREDENTIAL_MANAGER, ProviderCapabilities, ProviderId,
    };
    use crate::{SecretError, SecretValue, assert_no_plaintext, leaks_plaintext};

    #[test]
    fn provider_identities_round_trip() {
        let cases = [
            (OS_MACOS_KEYCHAIN, ProviderId::OsMacosKeychain),
            (
                OS_WINDOWS_CREDENTIAL_MANAGER,
                ProviderId::OsWindowsCredentialManager,
            ),
            (OS_LINUX_SECRET_SERVICE, ProviderId::OsLinuxSecretService),
        ];
        for (text, expected) in cases {
            let parsed = ProviderId::parse(text).expect("valid provider");
            assert_eq!(parsed, expected);
            assert_eq!(parsed.as_str(), text);
            assert_eq!(format!("{parsed}"), text);
            assert_eq!(ProviderId::parse(parsed.as_str()), Ok(expected));
        }
    }

    #[test]
    fn provider_rejects_unknown_identities() {
        let bad = [
            "",
            " ",
            "os-keychain",
            "os-credential-store",
            "os-macos-keychain ",
            " os-macos-keychain",
            "OS-MACOS-KEYCHAIN",
            "Os-Macos-Keychain",
            "os-macos-Keychain",
            "aws-secrets-manager",
            "hashicorp-vault",
            "env",
            "plaintext",
            "kernux://secret/ref/demo-token-01",
            "hnd_0123456789abcdef0123456789abcdef",
            "os-macos-keychain\0",
        ];
        for input in bad {
            let error = ProviderId::parse(input).expect_err("must reject");
            assert_eq!(error, SecretError::UnknownProvider);
            assert_eq!(format!("{error}"), "unknown secret provider");
            assert!(!format!("{error}").contains(input));
        }
    }

    #[test]
    fn destination_classes_round_trip() {
        let cases = [
            (DESTINATION_PROCESS_ENV, DestinationClass::ProcessEnv),
            (DESTINATION_HOST_COMMAND, DestinationClass::HostCommand),
            (
                DESTINATION_EGRESS_SUBSTITUTION,
                DestinationClass::EgressSubstitution,
            ),
        ];
        for (text, expected) in cases {
            let parsed = DestinationClass::parse(text).expect("valid class");
            assert_eq!(parsed, expected);
            assert_eq!(parsed.as_str(), text);
            assert_eq!(format!("{parsed}"), text);
            assert_eq!(DestinationClass::parse(parsed.as_str()), Ok(expected));
        }
    }

    #[test]
    fn destination_class_is_closed_and_case_sensitive() {
        let bad = [
            "",
            " ",
            "PROCESS-ENV",
            "Process-Env",
            "process_env",
            "host command",
            "none",
            "direct-destination",
            "secret.use",
            "process-env ",
            " process-env",
            "process-env\0",
        ];
        for input in bad {
            let error = DestinationClass::parse(input).expect_err("must reject");
            assert_eq!(error, SecretError::InvalidDestination);
            assert_eq!(format!("{error}"), "invalid secret destination");
            assert!(!format!("{error}").contains(input));
        }
    }

    #[test]
    fn capabilities_cover_every_destination_class() {
        let capabilities = ProviderCapabilities::declared();
        assert!(capabilities.can_serve(DestinationClass::ProcessEnv));
        assert!(capabilities.can_serve(DestinationClass::HostCommand));
        assert!(capabilities.can_serve(DestinationClass::EgressSubstitution));
        assert_eq!(capabilities, ProviderCapabilities::declared());
        for provider in [
            ProviderId::OsMacosKeychain,
            ProviderId::OsWindowsCredentialManager,
            ProviderId::OsLinuxSecretService,
        ] {
            let set = provider.capabilities();
            assert_eq!(set, capabilities);
            assert!(set.can_serve(DestinationClass::ProcessEnv));
            assert!(set.can_serve(DestinationClass::HostCommand));
            assert!(set.can_serve(DestinationClass::EgressSubstitution));
        }
    }

    #[test]
    fn hostname_accepts_canonical_names() {
        let cases = [
            ("example.com", "example.com"),
            ("Example.COM", "example.com"),
            ("LOCALHOST", "localhost"),
            ("a", "a"),
            ("host-01.example-domain.com", "host-01.example-domain.com"),
            ("xn--nxasmq6b.example", "xn--nxasmq6b.example"),
        ];
        for (input, canonical) in cases {
            let parsed = HostName::parse(input).expect("valid hostname");
            assert_eq!(parsed.as_str(), canonical);
            assert_eq!(format!("{parsed}"), canonical);
            assert_eq!(HostName::parse(input).expect("valid hostname"), parsed);
        }
    }

    #[test]
    fn hostname_rejects_malformed_input() {
        let long_label = "l".repeat(MAX_HOST_LABEL_BYTES + 1) + ".example";
        let too_long = "a.".to_owned() + &"b".repeat(MAX_HOST_NAME_BYTES);
        let bad = [
            "",
            ".example.com",
            "example.com.",
            "-bad.com",
            "bad-.com",
            "bad..com",
            "bad_host.com",
            "bad host.com",
            "host:8080",
            "https://example.com",
            "user@example.com",
            "example.com/path",
            "192.168.1.1",
            "123",
            "m\\u00fcnchen.de",
            "exam%70le.com",
            "host_name",
            "host!",
            "host;rm -rf",
            "$(hostname)",
            "a.",
        ];
        for input in bad {
            assert!(HostName::parse(input).is_err(), "must reject {input:?}");
        }
        assert!(HostName::parse(&long_label).is_err());
        assert!(HostName::parse(&too_long).is_err());
        let error = HostName::parse("evil host!").expect_err("must reject");
        assert_eq!(format!("{error}"), "invalid secret destination");
        assert!(!format!("{error}").contains("evil"));
        assert!(HostName::parse("m\u{00fc}nchen.de").is_err());
    }

    #[test]
    fn env_name_accepts_valid_names() {
        let cases = [
            "FOO",
            "_PRIVATE",
            "AWS_SECRET_ACCESS_KEY",
            "a",
            "lowercase_ok",
            "Mixed_Case_123",
        ];
        for input in cases {
            let parsed = EnvName::parse(input).expect("valid env name");
            assert_eq!(parsed.as_str(), input);
            assert_eq!(format!("{parsed}"), input);
            assert_eq!(format!("{parsed:?}"), format!("EnvName({input})"));
        }
        let upper = EnvName::parse("TOKEN").expect("valid");
        let lower = EnvName::parse("token").expect("valid");
        assert_ne!(upper, lower);
        assert_eq!(EnvName::parse("FOO").expect("valid").as_str(), "FOO");
    }

    #[test]
    fn env_name_rejects_malformed_input() {
        let too_long = "E".repeat(MAX_ENV_NAME_BYTES + 1);
        let bad = [
            "",
            "1FOO",
            "FOO-BAR",
            "FOO.BAR",
            "FOO BAR",
            "FOO$BAR",
            "FOO\0BAR",
            "FOO\nBAR",
            "$FOO",
            "-FOO",
            "caf\\u00e9",
        ];
        for input in bad {
            assert!(EnvName::parse(input).is_err(), "must reject {input:?}");
        }
        assert!(EnvName::parse(&too_long).is_err());
        let error = EnvName::parse("1EVIL-CRED").expect_err("must reject");
        assert_eq!(format!("{error}"), "invalid secret destination");
        assert!(!format!("{error}").contains("EVIL"));
        assert!(EnvName::parse("caf\u{00e9}").is_err());
    }

    #[test]
    fn provider_and_destination_rendering_never_carries_plaintext() {
        let plaintext = b"correct horse battery staple".to_vec();
        let value = SecretValue::from_bytes(plaintext.clone()).expect("valid value");
        let haystacks = [
            format!("{}", ProviderId::OsMacosKeychain),
            format!("{:?}", ProviderId::OsLinuxSecretService),
            format!("{}", DestinationClass::ProcessEnv),
            format!("{:?}", DestinationClass::EgressSubstitution),
            format!("{}", HostName::parse("example.com").expect("valid")),
            format!("{:?}", HostName::parse("example.com").expect("valid")),
            format!("{}", EnvName::parse("TOKEN_A").expect("valid")),
            format!("{:?}", EnvName::parse("TOKEN_A").expect("valid")),
            format!("{:?}", value),
        ];
        for haystack in &haystacks {
            assert!(!leaks_plaintext(haystack, &plaintext));
            assert_no_plaintext(haystack, &plaintext, "provider rendering");
        }
    }

    #[test]
    fn hostile_untrusted_input_is_rejected_closed() {
        let hostile = [
            "../etc/shadow",
            "..\\..\\windows\\system32",
            "$(rm -rf /)",
            "; DROP TABLE secrets; --",
            "{{constructor.constructor(\"return process\")()}}",
            "\u{202e}evil",
        ];
        for input in hostile {
            assert!(ProviderId::parse(input).is_err(), "provider must reject");
            assert!(
                DestinationClass::parse(input).is_err(),
                "destination must reject"
            );
            assert!(HostName::parse(input).is_err(), "hostname must reject");
            assert!(EnvName::parse(input).is_err(), "env must reject");
        }
        let long = "a".repeat(1024);
        assert!(ProviderId::parse(&long).is_err());
        assert!(DestinationClass::parse(&long).is_err());
        assert!(HostName::parse(&long).is_err());
        assert!(EnvName::parse(&long).is_err());
    }
}
