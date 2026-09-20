use kernux_contracts::ConstraintSet;

use crate::{Action, PolicyValidationError};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ConsequenceClass {
    C0,
    C1,
    C2,
    C3,
    C4,
}

impl ConsequenceClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::C0 => "C0",
            Self::C1 => "C1",
            Self::C2 => "C2",
            Self::C3 => "C3",
            Self::C4 => "C4",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct TrustedConsequenceFacts {
    pub sensitive_data: bool,
    pub external_side_effect: bool,
    pub unrecoverable: bool,
    pub privileged_host_change: bool,
    pub generated_or_untrusted_host_code: bool,
    pub authority_or_trust_change: bool,
    pub financial_commitment: bool,
    pub production_or_release_mutation: bool,
    pub extension_floor: Option<ConsequenceClass>,
}

pub fn classify_consequence(
    action: &Action,
    facts: TrustedConsequenceFacts,
) -> Result<ConsequenceClass, PolicyValidationError> {
    let mut floor = match action.as_str() {
        "secret.use" | "grant.revoke" | "network.send" | "browser.commit" | "browser.upload"
        | "git.publish" | "artifact.export" | "git.admin" | "runtime.manage" | "tool.invoke" => {
            ConsequenceClass::C2
        }
        "secret.reveal" => ConsequenceClass::C3,
        "grant.issue" | "grant.delegate" | "policy.modify" | "identity.manage" => {
            ConsequenceClass::C4
        }
        value if value.starts_with("ext.") => facts
            .extension_floor
            .ok_or(PolicyValidationError::UnknownExtensionConsequence)?,
        "files.read" | "files.metadata" | "process.inspect" | "git.read" | "browser.observe"
        | "computer.observe" | "clipboard.read" | "artifact.read" | "runtime.inspect"
        | "policy.inspect" | "identity.inspect" => ConsequenceClass::C0,
        _ => ConsequenceClass::C1,
    };
    if facts.sensitive_data || facts.external_side_effect {
        floor = floor.max(ConsequenceClass::C2);
    }
    if facts.unrecoverable || facts.privileged_host_change || facts.generated_or_untrusted_host_code
    {
        floor = floor.max(ConsequenceClass::C3);
    }
    if facts.authority_or_trust_change
        || facts.financial_commitment
        || facts.production_or_release_mutation
    {
        floor = ConsequenceClass::C4;
    }
    Ok(floor)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct CanonicalUtcSecond {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
}

impl CanonicalUtcSecond {
    pub fn parse(value: &str) -> Result<Self, PolicyValidationError> {
        let bytes = value.as_bytes();
        if bytes.len() != 20
            || bytes[4] != b'-'
            || bytes[7] != b'-'
            || bytes[10] != b'T'
            || bytes[13] != b':'
            || bytes[16] != b':'
            || bytes[19] != b'Z'
        {
            return Err(PolicyValidationError::InvalidTimestamp);
        }
        let year = decimal_u16(&bytes[0..4])?;
        let month = decimal_u8(&bytes[5..7])?;
        let day = decimal_u8(&bytes[8..10])?;
        let hour = decimal_u8(&bytes[11..13])?;
        let minute = decimal_u8(&bytes[14..16])?;
        let second = decimal_u8(&bytes[17..19])?;
        if year == 0
            || !(1..=12).contains(&month)
            || day == 0
            || day > days_in_month(year, month)
            || hour > 23
            || minute > 59
            || second > 59
        {
            return Err(PolicyValidationError::InvalidTimestamp);
        }
        Ok(Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
        })
    }

    pub fn to_canonical_text(self) -> String {
        format!(
            "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
            self.year, self.month, self.day, self.hour, self.minute, self.second
        )
    }
}

fn decimal_u16(bytes: &[u8]) -> Result<u16, PolicyValidationError> {
    if !bytes.iter().all(u8::is_ascii_digit) {
        return Err(PolicyValidationError::InvalidTimestamp);
    }
    Ok(bytes
        .iter()
        .fold(0u16, |value, byte| value * 10 + u16::from(byte - b'0')))
}

fn decimal_u8(bytes: &[u8]) -> Result<u8, PolicyValidationError> {
    if !bytes.iter().all(u8::is_ascii_digit) {
        return Err(PolicyValidationError::InvalidTimestamp);
    }
    Ok(bytes
        .iter()
        .fold(0u8, |value, byte| value * 10 + (byte - b'0')))
}

fn days_in_month(year: u16, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if year.is_multiple_of(400) || (year.is_multiple_of(4) && !year.is_multiple_of(100)) => {
            29
        }
        2 => 28,
        _ => 0,
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ValidatedConstraints {
    pub allowed_roots: Option<Vec<String>>,
    pub max_bytes: Option<u64>,
    pub max_duration_ms: Option<u64>,
    pub max_uses: Option<u64>,
    pub network_hosts: Option<Vec<String>>,
}

impl ValidatedConstraints {
    pub fn from_wire(value: &ConstraintSet) -> Result<Self, PolicyValidationError> {
        Self::from_parts(
            value.allowed_roots.clone(),
            value.max_bytes,
            value.max_duration_ms,
            value.max_uses,
            value.network_hosts.clone(),
        )
    }

    pub fn from_parts(
        allowed_roots: Option<Vec<String>>,
        max_bytes: Option<u64>,
        max_duration_ms: Option<u64>,
        max_uses: Option<u64>,
        network_hosts: Option<Vec<String>>,
    ) -> Result<Self, PolicyValidationError> {
        if max_uses == Some(0) {
            return Err(PolicyValidationError::InvalidConstraint);
        }
        Ok(Self {
            allowed_roots: validate_set(allowed_roots.as_deref(), valid_root)?,
            max_bytes,
            max_duration_ms,
            max_uses,
            network_hosts: validate_set(network_hosts.as_deref(), valid_host)?,
        })
    }

    pub fn intersect(&self, other: &Self) -> Result<Self, PolicyValidationError> {
        Ok(Self {
            allowed_roots: intersect_set(
                self.allowed_roots.as_deref(),
                other.allowed_roots.as_deref(),
            )?,
            max_bytes: min_optional(self.max_bytes, other.max_bytes),
            max_duration_ms: min_optional(self.max_duration_ms, other.max_duration_ms),
            max_uses: min_optional(self.max_uses, other.max_uses),
            network_hosts: intersect_set(
                self.network_hosts.as_deref(),
                other.network_hosts.as_deref(),
            )?,
        })
    }

    pub fn is_subset_of(&self, parent: &Self) -> bool {
        numeric_subset(self.max_bytes, parent.max_bytes)
            && numeric_subset(self.max_duration_ms, parent.max_duration_ms)
            && numeric_subset(self.max_uses, parent.max_uses)
            && set_subset(
                self.allowed_roots.as_deref(),
                parent.allowed_roots.as_deref(),
            )
            && set_subset(
                self.network_hosts.as_deref(),
                parent.network_hosts.as_deref(),
            )
    }
}

fn validate_set(
    values: Option<&[String]>,
    validator: fn(&str) -> bool,
) -> Result<Option<Vec<String>>, PolicyValidationError> {
    let Some(values) = values else {
        return Ok(None);
    };
    if values.is_empty() || values.iter().any(|value| !validator(value)) {
        return Err(PolicyValidationError::InvalidConstraint);
    }
    let mut normalized = values.to_vec();
    normalized.sort();
    if normalized.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(PolicyValidationError::InvalidConstraint);
    }
    Ok(Some(normalized))
}

fn valid_root(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 1024
        && !value.starts_with('/')
        && !value.ends_with('/')
        && !value.contains(['\\', '\0'])
        && !value
            .split('/')
            .any(|segment| segment.is_empty() || matches!(segment, "." | ".."))
        && !value.chars().any(char::is_control)
}

fn valid_host(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 253
        && !value.contains('*')
        && value.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || matches!(byte, b'.' | b'-' | b':' | b'[' | b']')
        })
}

fn min_optional(left: Option<u64>, right: Option<u64>) -> Option<u64> {
    match (left, right) {
        (Some(left), Some(right)) => Some(left.min(right)),
        (Some(value), None) | (None, Some(value)) => Some(value),
        (None, None) => None,
    }
}

fn intersect_set(
    left: Option<&[String]>,
    right: Option<&[String]>,
) -> Result<Option<Vec<String>>, PolicyValidationError> {
    match (left, right) {
        (Some(left), Some(right)) => {
            let values: Vec<_> = left
                .iter()
                .filter(|value| right.contains(value))
                .cloned()
                .collect();
            if values.is_empty() {
                Err(PolicyValidationError::ConstraintConflict)
            } else {
                Ok(Some(values))
            }
        }
        (Some(values), None) | (None, Some(values)) => Ok(Some(values.to_vec())),
        (None, None) => Ok(None),
    }
}

fn numeric_subset(child: Option<u64>, parent: Option<u64>) -> bool {
    match (child, parent) {
        (_, None) => true,
        (Some(child), Some(parent)) => child <= parent,
        (None, Some(_)) => false,
    }
}

fn set_subset(child: Option<&[String]>, parent: Option<&[String]>) -> bool {
    match (child, parent) {
        (_, None) => true,
        (Some(child), Some(parent)) => child.iter().all(|value| parent.contains(value)),
        (None, Some(_)) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consequence_floors_can_only_escalate() {
        let read = Action::parse("files.read", &[]).unwrap();
        assert_eq!(
            classify_consequence(&read, TrustedConsequenceFacts::default()),
            Ok(ConsequenceClass::C0)
        );
        assert_eq!(
            classify_consequence(
                &read,
                TrustedConsequenceFacts {
                    sensitive_data: true,
                    ..Default::default()
                }
            ),
            Ok(ConsequenceClass::C2)
        );
        assert_eq!(
            classify_consequence(
                &read,
                TrustedConsequenceFacts {
                    unrecoverable: true,
                    ..Default::default()
                }
            ),
            Ok(ConsequenceClass::C3)
        );
        assert_eq!(
            classify_consequence(
                &read,
                TrustedConsequenceFacts {
                    authority_or_trust_change: true,
                    ..Default::default()
                }
            ),
            Ok(ConsequenceClass::C4)
        );
        assert_eq!(
            classify_consequence(
                &Action::parse("grant.delegate", &[]).unwrap(),
                TrustedConsequenceFacts::default()
            ),
            Ok(ConsequenceClass::C4)
        );
    }

    #[test]
    fn extension_consequence_requires_registered_floor() {
        let action = Action::parse("ext.acme.observe", &["ext.acme.observe"]).unwrap();
        assert_eq!(
            classify_consequence(&action, TrustedConsequenceFacts::default()),
            Err(PolicyValidationError::UnknownExtensionConsequence)
        );
        assert_eq!(
            classify_consequence(
                &action,
                TrustedConsequenceFacts {
                    extension_floor: Some(ConsequenceClass::C1),
                    ..Default::default()
                }
            ),
            Ok(ConsequenceClass::C1)
        );
    }

    #[test]
    fn canonical_utc_seconds_are_exact_and_calendar_valid() {
        let a = CanonicalUtcSecond::parse("2026-09-19T01:00:00Z").unwrap();
        let b = CanonicalUtcSecond::parse("2026-09-19T01:05:00Z").unwrap();
        assert!(a < b);
        assert!(CanonicalUtcSecond::parse("2024-02-29T23:59:59Z").is_ok());
        for invalid in [
            "2026-02-29T00:00:00Z",
            "2026-01-01T00:00:60Z",
            "2026-01-01t00:00:00Z",
            "2026-01-01T00:00:00+00:00",
            "2026-01-01T00:00:00.0Z",
        ] {
            assert_eq!(
                CanonicalUtcSecond::parse(invalid),
                Err(PolicyValidationError::InvalidTimestamp),
                "{invalid}"
            );
        }
    }

    #[test]
    fn constraints_are_typed_conjunctive_and_narrowing_only() {
        let grant = ValidatedConstraints::from_wire(&ConstraintSet {
            allowed_roots: Some(vec!["src".into(), "tests".into()]),
            max_bytes: Some(4096),
            max_duration_ms: Some(10_000),
            max_uses: Some(4),
            network_hosts: None,
        })
        .unwrap();
        let request = ValidatedConstraints::from_wire(&ConstraintSet {
            allowed_roots: Some(vec!["src".into()]),
            max_bytes: Some(1024),
            max_duration_ms: None,
            max_uses: Some(1),
            network_hosts: None,
        })
        .unwrap();
        let effective = grant.intersect(&request).unwrap();
        assert_eq!(effective.allowed_roots, Some(vec!["src".into()]));
        assert_eq!(effective.max_bytes, Some(1024));
        assert_eq!(effective.max_duration_ms, Some(10_000));
        assert!(effective.is_subset_of(&grant));
        assert!(!request.is_subset_of(&grant));
        assert!(!grant.is_subset_of(&request));
    }

    #[test]
    fn invalid_or_disjoint_constraints_fail_closed() {
        assert!(
            ValidatedConstraints::from_wire(&ConstraintSet {
                allowed_roots: None,
                max_bytes: None,
                max_duration_ms: None,
                max_uses: Some(0),
                network_hosts: None,
            })
            .is_err()
        );
        assert!(
            ValidatedConstraints::from_wire(&ConstraintSet {
                allowed_roots: Some(vec!["../etc".into()]),
                max_bytes: None,
                max_duration_ms: None,
                max_uses: None,
                network_hosts: None,
            })
            .is_err()
        );
        let left = ValidatedConstraints::from_wire(&ConstraintSet {
            allowed_roots: Some(vec!["src".into()]),
            max_bytes: None,
            max_duration_ms: None,
            max_uses: None,
            network_hosts: None,
        })
        .unwrap();
        let right = ValidatedConstraints::from_wire(&ConstraintSet {
            allowed_roots: Some(vec!["tests".into()]),
            max_bytes: None,
            max_duration_ms: None,
            max_uses: None,
            network_hosts: None,
        })
        .unwrap();
        assert_eq!(
            left.intersect(&right),
            Err(PolicyValidationError::ConstraintConflict)
        );
    }
}
