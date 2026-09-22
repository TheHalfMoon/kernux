//! Opaque secret references, opaque secret handles, and redacted values.
//!
//! A [`SecretRef`] names secret material through the canonical Kernux
//! resource grammar without containing any plaintext. A [`SecretHandle`] is
//! a short opaque identifier minted by the broker for one resolved use; the
//! handle itself is non-secret metadata and possession of it authorizes
//! nothing. A [`SecretValue`] carries live plaintext in memory only: it is
//! zeroized on drop, renders as a fixed redaction constant, and deliberately
//! implements neither Display nor Serialize so values cannot drift into logs,
//! prompts, process arguments, environments, events, or artifact exports.

use core::hash::{Hash, Hasher};

use kernux_policy::{CanonicalResource, ResourceAuthority, ResourceScope};
use zeroize::Zeroizing;

use crate::{REDACTED_SECRET_VALUE, SecretError};

/// Maximum decoded opaque token length inside a secret reference, in bytes.
pub const MAX_SECRET_REF_TOKEN_BYTES: usize = 256;
/// Maximum retained secret value length, in bytes.
pub const MAX_SECRET_VALUE_BYTES: usize = 65536;
/// Required handle prefix. Handles are never bearer tokens.
pub const SECRET_HANDLE_PREFIX: &str = "hnd_";
/// Required lowercase hexadecimal payload length after the handle prefix.
pub const SECRET_HANDLE_HEX_LEN: usize = 32;

/// Canonical opaque secret reference.
///
/// Wraps the validated Kernux resource grammar instead of duplicating it, so
/// reference shape can never drift from policy truth. The stored canonical
/// spelling is the exact validated input; strict equality compares canonical
/// spellings, while [`SecretRef::matches_exact`] compares the decoded
/// resource identity the broker enforces at admission time.
#[derive(Clone)]
pub struct SecretRef {
    canonical: String,
    resource: CanonicalResource,
}

impl SecretRef {
    /// Parse one canonical `kernux://secret/ref/<opaque>` reference.
    ///
    /// Fails closed on any malformed input, on any non-secret authority, and
    /// on any opaque token outside the length bound.
    pub fn parse(input: &str) -> Result<Self, SecretError> {
        let resource =
            CanonicalResource::parse(input).map_err(|_| SecretError::InvalidReference)?;
        if resource.authority() != ResourceAuthority::Secret {
            return Err(SecretError::InvalidReference);
        }
        let segments = resource.segments();
        if segments.len() != 2 || segments[0] != "ref" {
            return Err(SecretError::InvalidReference);
        }
        let token = &segments[1];
        if token.is_empty() || token.len() > MAX_SECRET_REF_TOKEN_BYTES {
            return Err(SecretError::InvalidReference);
        }
        Ok(Self {
            canonical: input.to_owned(),
            resource,
        })
    }

    /// The exact validated canonical spelling. Non-secret metadata.
    pub fn canonical(&self) -> &str {
        &self.canonical
    }

    /// The decoded opaque token. Non-secret metadata, never plaintext.
    pub fn token(&self) -> &str {
        &self.resource.segments()[1]
    }

    /// The underlying validated policy resource.
    pub fn resource(&self) -> &CanonicalResource {
        &self.resource
    }

    /// Semantic identity used at broker admission: decoded resources must be
    /// exactly equal. Percent-encoding spellings of the same token match.
    pub fn matches_exact(&self, other: &Self) -> bool {
        self.resource
            .matches(&other.resource, ResourceScope::Exact, false)
            .unwrap_or(false)
    }
}

impl PartialEq for SecretRef {
    fn eq(&self, other: &Self) -> bool {
        self.canonical == other.canonical
    }
}

impl Eq for SecretRef {}

impl Hash for SecretRef {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.canonical.hash(state);
    }
}

impl core::fmt::Debug for SecretRef {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "SecretRef({})", self.canonical)
    }
}

impl core::fmt::Display for SecretRef {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.canonical)
    }
}

/// Opaque non-secret handle minted by the broker for one resolved use.
///
/// Handles are short, closed-format identifiers. They carry no authority: a
/// caller that presents a handle without an exact Grant, an exact secret
/// reference, and an exact approved destination is denied.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SecretHandle {
    inner: String,
}

impl SecretHandle {
    /// Validate one handle of the form `hnd_` plus 32 lowercase hex digits.
    pub fn parse(input: &str) -> Result<Self, SecretError> {
        let payload = input
            .strip_prefix(SECRET_HANDLE_PREFIX)
            .ok_or(SecretError::InvalidHandle)?;
        if payload.len() != SECRET_HANDLE_HEX_LEN || !payload.bytes().all(is_lower_hex) {
            return Err(SecretError::InvalidHandle);
        }
        Ok(Self {
            inner: input.to_owned(),
        })
    }

    /// The handle text. Non-secret metadata.
    pub fn as_str(&self) -> &str {
        &self.inner
    }
}

fn is_lower_hex(byte: u8) -> bool {
    matches!(byte, 48..=57 | 97..=102)
}

impl core::fmt::Debug for SecretHandle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "SecretHandle({})", self.inner)
    }
}

impl core::fmt::Display for SecretHandle {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

/// Live secret plaintext held in memory only.
///
/// Values move, never duplicate: there is no Clone. Values never render as
/// text: there is no Display and no Serialize. Debug output is a fixed
/// redaction constant. Memory is zeroized on drop. Only the broker may call
/// [`SecretValue::expose`], and only while resolving one admitted use.
pub struct SecretValue {
    inner: Zeroizing<Vec<u8>>,
}

impl SecretValue {
    /// Wrap plaintext bytes. Rejects empty input and input above
    /// [`MAX_SECRET_VALUE_BYTES`] without inspecting the content.
    pub fn from_bytes(bytes: Vec<u8>) -> Result<Self, SecretError> {
        if bytes.is_empty() {
            return Err(SecretError::EmptyValue);
        }
        if bytes.len() > MAX_SECRET_VALUE_BYTES {
            return Err(SecretError::ValueTooLarge);
        }
        Ok(Self {
            inner: Zeroizing::new(bytes),
        })
    }

    /// Borrow the live plaintext for one admitted resolution.
    pub fn expose(&self) -> &[u8] {
        &self.inner
    }

    /// Plaintext length in bytes.
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Always false: construction rejects empty values.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

impl core::fmt::Debug for SecretValue {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{REDACTED_SECRET_VALUE}")
    }
}

#[cfg(test)]
mod handle_tests {
    use super::{
        MAX_SECRET_REF_TOKEN_BYTES, MAX_SECRET_VALUE_BYTES, SecretHandle, SecretRef, SecretValue,
    };
    use crate::{assert_no_plaintext, leaks_plaintext};

    const DEMO_REF: &str = "kernux://secret/ref/demo-token-01";

    #[test]
    fn reference_round_trip() {
        let parsed = SecretRef::parse(DEMO_REF).expect("valid reference");
        assert_eq!(parsed.canonical(), DEMO_REF);
        assert_eq!(parsed.token(), "demo-token-01");
        assert_eq!(format!("{parsed}"), DEMO_REF);
        assert_eq!(
            format!("{parsed:?}"),
            "SecretRef(kernux://secret/ref/demo-token-01)"
        );
    }

    #[test]
    fn reference_rejects_malformed_input() {
        let bad = [
            "",
            "kernux://secret/ref/",
            "kernux://secret/ref",
            "kernux://secret/ref/a/b",
            "kernux://project/p/fs/src",
            "kernux://runtime/r",
            "https://secret/ref/x",
            "kernux://secret/ref/with space",
            "kernux://secret/ref/with?query",
            "kernux://secret/ref/lower%2fhex",
            "not a uri at all",
        ];
        for input in bad {
            assert!(SecretRef::parse(input).is_err(), "must reject {input:?}");
        }
        let long = "kernux://secret/ref/".to_owned() + &"t".repeat(MAX_SECRET_REF_TOKEN_BYTES + 1);
        assert!(SecretRef::parse(&long).is_err());
    }

    #[test]
    fn reference_strict_and_semantic_equality() {
        let first = SecretRef::parse("kernux://secret/ref/a+b").expect("valid");
        let same = SecretRef::parse("kernux://secret/ref/a+b").expect("valid");
        let encoded = SecretRef::parse("kernux://secret/ref/a%2Bb").expect("valid");
        let other = SecretRef::parse("kernux://secret/ref/a+c").expect("valid");
        let upper = SecretRef::parse("kernux://secret/ref/A+B").expect("valid");
        assert_eq!(first, same);
        assert_ne!(first, encoded);
        assert!(first.matches_exact(&same));
        assert!(first.matches_exact(&encoded));
        assert!(!first.matches_exact(&other));
        assert!(!first.matches_exact(&upper));
    }

    #[test]
    fn handle_round_trip() {
        let text = "hnd_0123456789abcdef0123456789abcdef";
        let handle = SecretHandle::parse(text).expect("valid handle");
        assert_eq!(handle.as_str(), text);
        assert_eq!(format!("{handle}"), text);
    }

    #[test]
    fn handle_rejects_malformed_input() {
        let bad = [
            "",
            "hnd_",
            "hnd_0123456789abcdef0123456789abcde",
            "hnd_0123456789abcdef0123456789abcdef0",
            "hnd_0123456789ABCDEF0123456789ABCDEF",
            "hnd_0123456789abcdef0123456789abcdeg",
            "sk_0123456789abcdef0123456789abcdef",
            "0123456789abcdef0123456789abcdef",
        ];
        for input in bad {
            assert!(SecretHandle::parse(input).is_err(), "must reject {input:?}");
        }
    }

    #[test]
    fn value_round_trip_and_redaction() {
        let plaintext = b"correct horse battery staple".to_vec();
        let value = SecretValue::from_bytes(plaintext.clone()).expect("valid value");
        assert_eq!(value.len(), plaintext.len());
        assert_eq!(value.expose(), plaintext.as_slice());
        let debug = format!("{value:?}");
        assert_eq!(debug, "[redacted secret value]");
        assert!(!leaks_plaintext(&debug, &plaintext));
        assert_no_plaintext(&debug, &plaintext, "value debug");
    }

    #[test]
    fn value_rejects_empty_and_oversize() {
        assert!(SecretValue::from_bytes(Vec::new()).is_err());
        let big = vec![7u8; MAX_SECRET_VALUE_BYTES + 1];
        assert!(SecretValue::from_bytes(big).is_err());
        let max = vec![7u8; MAX_SECRET_VALUE_BYTES];
        assert!(SecretValue::from_bytes(max).is_ok());
    }
}
