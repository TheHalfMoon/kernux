#![doc = "Provider-neutral kernel egress-class and destination primitives (SG-000029 PR-A)."]
#![doc = ""]
#![doc = "PR-A scope: closed EgressClass vocabulary with exact parsing and stable"]
#![doc = "strings, validated destination identity shapes, and exact class-destination"]
#![doc = "structural binding. No Grant evaluation, no authority minting, no provider"]
#![doc = "coupling, no plaintext. Evaluation arrives in PR-B; adversarial corpus in PR-C."]

use crate::{PolicyValidationError, canonical_uuid_v7};

/// Canonical string for NONE: no non-loopback network.
pub const EGRESS_NONE: &str = "NONE";
/// Canonical string for direct user-selected origin.
pub const EGRESS_DIRECT_DESTINATION: &str = "DIRECT_DESTINATION";
/// Canonical string for explicit third-party account.
pub const EGRESS_CONNECTED_ACCOUNT: &str = "CONNECTED_ACCOUNT";
/// Canonical string for model provider.
pub const EGRESS_EXTERNAL_MODEL: &str = "EXTERNAL_MODEL";
/// Canonical string for hosted tool provider.
pub const EGRESS_EXTERNAL_TOOL: &str = "EXTERNAL_TOOL";
/// Canonical string for enrolled remote runtime.
pub const EGRESS_REMOTE_RUNTIME: &str = "REMOTE_RUNTIME";
/// Canonical string for software update.
pub const EGRESS_UPDATE: &str = "UPDATE";
/// Canonical string for telemetry.
pub const EGRESS_TELEMETRY: &str = "TELEMETRY";

/// Maximum validated host length in bytes, including dots.
pub const MAX_EGRESS_HOST_BYTES: usize = 253;
/// Maximum single DNS label length in bytes.
pub const MAX_EGRESS_HOST_LABEL_BYTES: usize = 63;
/// Maximum validated account identifier length in bytes.
pub const MAX_EGRESS_ACCOUNT_BYTES: usize = 128;

/// Closed kernel egress classification.
///
/// Exact and case-sensitive. Unknown classes fail closed. Possession of a
/// parsed value authorizes nothing; PR-B binds it as a grant constraint only.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EgressClass {
    None,
    DirectDestination,
    ConnectedAccount,
    ExternalModel,
    ExternalTool,
    RemoteRuntime,
    Update,
    Telemetry,
}

impl EgressClass {
    /// Parse one exact canonical egress class.
    ///
    /// Matching is exact and case-sensitive. Never echoes input in errors
    /// beyond the typed error variant.
    pub fn parse(input: &str) -> Result<Self, PolicyValidationError> {
        match input {
            EGRESS_NONE => Ok(Self::None),
            EGRESS_DIRECT_DESTINATION => Ok(Self::DirectDestination),
            EGRESS_CONNECTED_ACCOUNT => Ok(Self::ConnectedAccount),
            EGRESS_EXTERNAL_MODEL => Ok(Self::ExternalModel),
            EGRESS_EXTERNAL_TOOL => Ok(Self::ExternalTool),
            EGRESS_REMOTE_RUNTIME => Ok(Self::RemoteRuntime),
            EGRESS_UPDATE => Ok(Self::Update),
            EGRESS_TELEMETRY => Ok(Self::Telemetry),
            _ => Err(PolicyValidationError::UnknownEgressClass),
        }
    }

    /// The canonical stable string. Non-secret metadata.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => EGRESS_NONE,
            Self::DirectDestination => EGRESS_DIRECT_DESTINATION,
            Self::ConnectedAccount => EGRESS_CONNECTED_ACCOUNT,
            Self::ExternalModel => EGRESS_EXTERNAL_MODEL,
            Self::ExternalTool => EGRESS_EXTERNAL_TOOL,
            Self::RemoteRuntime => EGRESS_REMOTE_RUNTIME,
            Self::Update => EGRESS_UPDATE,
            Self::Telemetry => EGRESS_TELEMETRY,
        }
    }

    /// Whether this class requires an explicit destination.
    ///
    /// NONE requires no destination; every other class requires one.
    pub const fn requires_destination(self) -> bool {
        !matches!(self, Self::None)
    }
}

impl core::fmt::Display for EgressClass {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// One validated DNS hostname for host-shaped egress destinations.
///
/// Syntactic and fail-closed: bare DNS names only (ASCII letters/digits/hyphen,
/// dot separators, at most 63 bytes per label and 253 total, no empty labels,
/// no leading/trailing dots or hyphens, no ports, schemes, userinfo, paths,
/// underscores, whitespace, non-ASCII, or escapes). Numeric-only names are
/// rejected. Uppercase ASCII is normalized to lowercase. Stored spelling is
/// non-secret metadata. No network access is performed.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct EgressHost {
    canonical: String,
}

impl EgressHost {
    /// Parse and validate one bare DNS hostname. No I/O.
    pub fn parse(input: &str) -> Result<Self, PolicyValidationError> {
        if input.is_empty() || input.len() > MAX_EGRESS_HOST_BYTES {
            return Err(PolicyValidationError::InvalidEgressDestination);
        }
        if !input
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'.')
        {
            return Err(PolicyValidationError::InvalidEgressDestination);
        }
        if input.starts_with('.')
            || input.ends_with('.')
            || input.starts_with('-')
            || input.ends_with('-')
        {
            return Err(PolicyValidationError::InvalidEgressDestination);
        }
        if input.contains("..") {
            return Err(PolicyValidationError::InvalidEgressDestination);
        }
        let mut labels = 0;
        let mut all_numeric = true;
        for label in input.split('.') {
            labels += 1;
            if label.is_empty() || label.len() > MAX_EGRESS_HOST_LABEL_BYTES {
                return Err(PolicyValidationError::InvalidEgressDestination);
            }
            let bytes = label.as_bytes();
            if !bytes[0].is_ascii_alphanumeric() || !bytes[bytes.len() - 1].is_ascii_alphanumeric()
            {
                return Err(PolicyValidationError::InvalidEgressDestination);
            }
            if !label
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-')
            {
                return Err(PolicyValidationError::InvalidEgressDestination);
            }
            if !label.bytes().all(|b| b.is_ascii_digit()) {
                all_numeric = false;
            }
        }
        if labels == 0 || all_numeric {
            return Err(PolicyValidationError::InvalidEgressDestination);
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

impl core::fmt::Debug for EgressHost {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "EgressHost({})", self.canonical)
    }
}

impl core::fmt::Display for EgressHost {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.canonical)
    }
}

/// One validated connected-account identifier.
///
/// Syntactic and fail-closed: 1 to 128 bytes, ASCII letters/digits plus
/// '-', '_', '.' only, starting and ending with an alphanumeric. No spaces,
/// no userinfo, no secret material. Stored spelling is non-secret metadata.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct EgressAccount {
    canonical: String,
}

impl EgressAccount {
    /// Parse and validate one account identifier. No I/O.
    pub fn parse(input: &str) -> Result<Self, PolicyValidationError> {
        if input.is_empty() || input.len() > MAX_EGRESS_ACCOUNT_BYTES {
            return Err(PolicyValidationError::InvalidEgressDestination);
        }
        if !input
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
        {
            return Err(PolicyValidationError::InvalidEgressDestination);
        }
        let bytes = input.as_bytes();
        if !bytes[0].is_ascii_alphanumeric() || !bytes[bytes.len() - 1].is_ascii_alphanumeric() {
            return Err(PolicyValidationError::InvalidEgressDestination);
        }
        if input.contains("..") {
            return Err(PolicyValidationError::InvalidEgressDestination);
        }
        Ok(Self {
            canonical: input.to_owned(),
        })
    }

    /// The validated canonical account identifier. Non-secret metadata.
    pub fn as_str(&self) -> &str {
        &self.canonical
    }
}

impl core::fmt::Debug for EgressAccount {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "EgressAccount({})", self.canonical)
    }
}

impl core::fmt::Display for EgressAccount {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.canonical)
    }
}

/// One validated remote-runtime identifier (canonical UUIDv7).
///
/// Distinct shape from hosts and accounts so REMOTE_RUNTIME cannot be confused
/// with local execution or with host-shaped classes. Stored spelling is
/// non-secret metadata.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct EgressRuntimeId {
    canonical: String,
}

impl EgressRuntimeId {
    /// Parse one canonical UUIDv7 runtime identifier. No I/O.
    pub fn parse(input: &str) -> Result<Self, PolicyValidationError> {
        if !canonical_uuid_v7(input) {
            return Err(PolicyValidationError::InvalidEgressDestination);
        }
        Ok(Self {
            canonical: input.to_owned(),
        })
    }

    /// The validated canonical runtime identifier. Non-secret metadata.
    pub fn as_str(&self) -> &str {
        &self.canonical
    }
}

impl core::fmt::Debug for EgressRuntimeId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "EgressRuntimeId({})", self.canonical)
    }
}

impl core::fmt::Display for EgressRuntimeId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.canonical)
    }
}

/// Explicit egress destination identity and scope.
///
/// NONE carries no destination. Every other class carries an explicit valid
/// destination. Destinations are never inferred.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EgressDestination {
    /// No destination. Only valid with EgressClass::None.
    None,
    /// Validated host for direct, model, tool, update, and telemetry classes.
    Host(EgressHost),
    /// Validated account for the connected-account class.
    Account(EgressAccount),
    /// Validated runtime for the remote-runtime class.
    Runtime(EgressRuntimeId),
}

impl EgressDestination {
    /// Whether this destination carries an explicit identity.
    pub const fn has_identity(&self) -> bool {
        !matches!(self, Self::None)
    }
}

/// Exact class-destination structural binding.
///
/// Returns Ok only when the destination shape matches the class:
/// NONE requires None; DIRECT_DESTINATION, EXTERNAL_MODEL, EXTERNAL_TOOL,
/// UPDATE, and TELEMETRY require Host; CONNECTED_ACCOUNT requires Account;
/// REMOTE_RUNTIME requires Runtime. Anything else fails closed. No inference,
/// no reuse across classes, no wildcard.
pub fn validate_egress_binding(
    class: EgressClass,
    destination: &EgressDestination,
) -> Result<(), PolicyValidationError> {
    let valid = matches!(
        (class, destination),
        (EgressClass::None, EgressDestination::None)
            | (EgressClass::DirectDestination, EgressDestination::Host(_))
            | (EgressClass::ConnectedAccount, EgressDestination::Account(_))
            | (EgressClass::ExternalModel, EgressDestination::Host(_))
            | (EgressClass::ExternalTool, EgressDestination::Host(_))
            | (EgressClass::RemoteRuntime, EgressDestination::Runtime(_))
            | (EgressClass::Update, EgressDestination::Host(_))
            | (EgressClass::Telemetry, EgressDestination::Host(_))
    );
    valid
        .then_some(())
        .ok_or(PolicyValidationError::EgressClassDestinationMismatch)
}

#[cfg(test)]
mod tests {
    use super::*;

    const RUNTIME_V7: &str = "01890f3a-7b2f-7e55-aa66-6f708192a3b5";

    #[test]
    fn all_eight_classes_parse_exactly_and_render_stably() {
        let cases = [
            ("NONE", EgressClass::None),
            ("DIRECT_DESTINATION", EgressClass::DirectDestination),
            ("CONNECTED_ACCOUNT", EgressClass::ConnectedAccount),
            ("EXTERNAL_MODEL", EgressClass::ExternalModel),
            ("EXTERNAL_TOOL", EgressClass::ExternalTool),
            ("REMOTE_RUNTIME", EgressClass::RemoteRuntime),
            ("UPDATE", EgressClass::Update),
            ("TELEMETRY", EgressClass::Telemetry),
        ];
        for (text, class) in cases {
            assert_eq!(EgressClass::parse(text).unwrap(), class);
            assert_eq!(class.as_str(), text);
            assert_eq!(class.to_string(), text);
        }
        assert_eq!(EgressClass::None.requires_destination(), false);
        for class in [
            EgressClass::DirectDestination,
            EgressClass::ConnectedAccount,
            EgressClass::ExternalModel,
            EgressClass::ExternalTool,
            EgressClass::RemoteRuntime,
            EgressClass::Update,
            EgressClass::Telemetry,
        ] {
            assert!(class.requires_destination(), "{:?}", class);
        }
    }

    #[test]
    fn unknown_and_permissive_forms_fail_closed() {
        for invalid in [
            "",
            "none",
            "None",
            "direct_destination",
            "DIRECT-DESTINATION",
            "DIRECT DESTINATION",
            "EXTERNAL",
            "external",
            "*",
            "ANY",
            "ALL",
            "DIRECT_DESTINATION ",
            " DIRECT_DESTINATION",
            "EXTERNAL_MODEL\n",
            "CONNECTED-ACCOUNT",
            "REMOTE RUNTIME",
            "UPDATE\x00",
        ] {
            assert_eq!(
                EgressClass::parse(invalid),
                Err(PolicyValidationError::UnknownEgressClass),
                "{:?}",
                invalid,
            );
        }
    }

    #[test]
    fn host_validation_is_closed_and_normalizing() {
        assert_eq!(
            EgressHost::parse("Example.COM").unwrap().as_str(),
            "example.com"
        );
        assert!(EgressHost::parse("docs.rs").is_ok());
        for invalid in [
            "",
            ".example.com",
            "example.com.",
            "-example.com",
            "example.com-",
            "example..com",
            "exam ple.com",
            "example_com",
            "https://example.com",
            "example.com:443",
            "user@example.com",
            "example.com/path",
            "127.0.0.1",
            "12345",
            "café.example",
        ] {
            assert!(EgressHost::parse(invalid).is_err(), "{:?}", invalid);
        }
        let long_host = format!("{}.example.com", "a".repeat(250));
        assert!(EgressHost::parse(&long_host).is_err());
    }

    #[test]
    fn account_and_runtime_validation_are_closed() {
        assert!(EgressAccount::parse("user-01.work").is_ok());
        for invalid in [
            "",
            ".leading",
            "trailing.",
            "-leading",
            "trailing-",
            "has space",
            "has/slash",
            "has@at",
            "double..dot",
            &"a".repeat(129),
        ] {
            assert!(EgressAccount::parse(invalid).is_err(), "{:?}", invalid);
        }
        assert!(EgressRuntimeId::parse(RUNTIME_V7).is_ok());
        assert!(EgressRuntimeId::parse("550e8400-e29b-41d4-a716-446655440000").is_err());
        assert!(EgressRuntimeId::parse("not-a-uuid").is_err());
    }

    #[test]
    fn none_requires_no_destination_and_others_require_explicit() {
        let host = EgressDestination::Host(EgressHost::parse("example.com").unwrap());
        let account = EgressDestination::Account(EgressAccount::parse("user-01").unwrap());
        let runtime = EgressDestination::Runtime(EgressRuntimeId::parse(RUNTIME_V7).unwrap());
        assert!(validate_egress_binding(EgressClass::None, &EgressDestination::None).is_ok());
        assert!(validate_egress_binding(EgressClass::None, &host).is_err());
        assert!(validate_egress_binding(EgressClass::None, &account).is_err());
        assert!(validate_egress_binding(EgressClass::None, &runtime).is_err());
        assert!(validate_egress_binding(EgressClass::DirectDestination, &host).is_ok());
        assert!(
            validate_egress_binding(EgressClass::DirectDestination, &EgressDestination::None)
                .is_err()
        );
        assert!(validate_egress_binding(EgressClass::ConnectedAccount, &account).is_ok());
        assert!(validate_egress_binding(EgressClass::ConnectedAccount, &host).is_err());
        assert!(validate_egress_binding(EgressClass::RemoteRuntime, &runtime).is_ok());
        assert!(validate_egress_binding(EgressClass::RemoteRuntime, &host).is_err());
        assert!(validate_egress_binding(EgressClass::ExternalModel, &host).is_ok());
        assert!(validate_egress_binding(EgressClass::ExternalModel, &account).is_err());
        assert!(validate_egress_binding(EgressClass::ExternalTool, &host).is_ok());
        assert!(validate_egress_binding(EgressClass::Update, &host).is_ok());
        assert!(validate_egress_binding(EgressClass::Telemetry, &host).is_ok());
        assert!(validate_egress_binding(EgressClass::Telemetry, &account).is_err());
    }

    #[test]
    fn class_separation_is_structural() {
        // Same host string in different classes stays a different class.
        assert_ne!(EgressClass::DirectDestination, EgressClass::ExternalModel);
        assert_ne!(EgressClass::ConnectedAccount, EgressClass::ExternalTool);
        assert_ne!(EgressClass::None, EgressClass::Telemetry);
        assert_ne!(EgressClass::RemoteRuntime, EgressClass::DirectDestination);
        assert_ne!(EgressClass::Update, EgressClass::ExternalTool);
        assert_ne!(EgressClass::Telemetry, EgressClass::ExternalModel);
        // Cross-shape substitution fails even when the inner string is valid.
        let host = EgressDestination::Host(EgressHost::parse("example.com").unwrap());
        assert!(validate_egress_binding(EgressClass::ConnectedAccount, &host).is_err());
        assert!(validate_egress_binding(EgressClass::RemoteRuntime, &host).is_err());
    }
}
