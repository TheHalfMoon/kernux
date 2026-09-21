//! Provider-neutral permission-profile vocabulary and bounded Grant candidates.
//!
//! A user-facing profile is a convenience input, never an authorization path. It
//! names which exact actions may be compiled for which exact resource authority
//! and up to which authoritative consequence class. Compilation itself lives in
//! `compile_profile`; this module owns the closed vocabulary, the explicit preset
//! tables, the effective preset ceiling, and the bounded Grant candidate that
//! every profile must produce.
//!
//! Nothing here issues, persists, or revokes authority, consumes a budget,
//! records an approval, produces [`crate::PolicyDecision`] or
//! [`crate::GrantUseDecision`], or executes a host operation.

use crate::{
    Action, CanonicalResource, CanonicalUtcSecond, ConsequenceClass, PolicyValidationError,
    ResourceAuthority, ResourceScope, TrustedConsequenceFacts, ValidatedConstraints,
    ValidatedGrant, ValidatedSubjectScope, canonical_uuid_v7, classify_consequence,
    validate_action_resource,
};
use kernux_contracts::{
    CanonicalRef, ConsequenceClass as WireConsequenceClass, ConstraintSet, EntityKind, Grant,
    ResourceMatch, SubjectScope,
};

/// Longest Grant lifetime a profile may compile, in seconds.
pub const MAX_PROFILE_WINDOW_SECONDS: i64 = 86_400;

/// Largest use budget a profile may compile.
pub const MAX_PROFILE_USES: u64 = 1_000;

/// Closed vocabulary of user-facing permission profiles.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PermissionProfile {
    Safe,
    Standard,
    Developer,
    Autonomous,
    Custom,
}

impl PermissionProfile {
    /// Every profile name, in canonical display order.
    pub const ALL: [Self; 5] = [
        Self::Safe,
        Self::Standard,
        Self::Developer,
        Self::Autonomous,
        Self::Custom,
    ];

    /// Parse one exact canonical profile name.
    ///
    /// Recognizes exactly `Safe`, `Standard`, `Developer`, `Autonomous`, and
    /// `Custom`. Different case, surrounding whitespace, and any other spelling
    /// are rejected so profile input can never be ambiguous.
    pub fn parse(input: &str) -> Result<Self, ProfileCompileError> {
        match input {
            "Safe" => Ok(Self::Safe),
            "Standard" => Ok(Self::Standard),
            "Developer" => Ok(Self::Developer),
            "Autonomous" => Ok(Self::Autonomous),
            "Custom" => Ok(Self::Custom),
            _ => Err(ProfileCompileError::UnknownProfile),
        }
    }

    /// Canonical profile name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Safe => "Safe",
            Self::Standard => "Standard",
            Self::Developer => "Developer",
            Self::Autonomous => "Autonomous",
            Self::Custom => "Custom",
        }
    }
}

/// Deterministic, redaction-safe reason a profile refused to compile.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProfileCompileError {
    /// Profile input is unknown or ambiguous.
    UnknownProfile,
    /// Action is not a core action and not a trusted registered extension.
    UnknownAction,
    /// Action or resource uses a wildcard instead of an exact value.
    WildcardNotPermitted,
    /// Resource does not satisfy the canonical resource grammar and shape.
    InvalidResource,
    /// Action cannot authorize this resource authority.
    ResourceAuthorityMismatch,
    /// The profile's explicit table does not cover this exact action/authority.
    ActionNotPermittedByProfile,
    /// Authoritative consequence exceeds the profile or rule ceiling.
    ConsequenceExceeded,
    /// Trusted consequence facts are incomplete for the requested action.
    UnknownConsequence,
    /// Intent is missing, malformed, non-canonical, or unbounded.
    InvalidIntent,
    /// Intent originated from content that cannot authorize authority.
    UntrustedProvenance,
    /// `Custom` was requested without a matching trusted granular rule.
    RuleMissing,
    /// A trusted granular rule is not a valid exact rule.
    InvalidRule,
    /// Constraint ceilings cannot be satisfied simultaneously.
    ConstraintConflict,
}

/// Whether an intent comes from trusted authority or from untrusted content.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProvenanceTrust {
    Trusted,
    Untrusted,
}

/// One trusted, exact, finite profile intent.
///
/// Every field is already-trusted caller input. Untrusted web, document, tool,
/// provider, or remote content cannot construct a compiling intent.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProfileIntent<'a> {
    pub subject: ValidatedSubjectScope,
    pub action: &'a str,
    pub resource_uri: &'a str,
    pub resource_scope: ResourceScope,
    pub runtime_id: &'a str,
    pub runtime_revision: u32,
    pub constraints: ValidatedConstraints,
    pub facts: TrustedConsequenceFacts,
    pub issuer_authority: &'a str,
    pub policy_revision: u64,
    pub issued_at: &'a str,
    pub not_before: &'a str,
    pub expires_at: &'a str,
    pub max_uses: u64,
    pub provenance_trust: ProvenanceTrust,
}

/// One explicit trusted granular rule. Only `Custom` consumes rules.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProfileRule<'a> {
    pub action: &'a str,
    pub resource_authorities: &'a [ResourceAuthority],
    pub resource_scope: ResourceScope,
    pub consequence_ceiling: ConsequenceClass,
    pub constraint_ceiling: ValidatedConstraints,
}

/// One bounded Grant candidate compiled from a profile.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProfileGrantCandidate {
    pub profile: PermissionProfile,
    pub subject: ValidatedSubjectScope,
    pub action: Action,
    pub resource: CanonicalResource,
    pub resource_uri: String,
    pub resource_scope: ResourceScope,
    pub runtime_id: String,
    pub runtime_revision: u32,
    pub constraints: ValidatedConstraints,
    pub consequence_ceiling: ConsequenceClass,
    pub issuer_authority: String,
    pub policy_revision: u64,
    pub issued_at: CanonicalUtcSecond,
    pub not_before: CanonicalUtcSecond,
    pub expires_at: CanonicalUtcSecond,
    pub max_uses: u64,
    pub delegation_depth: u64,
    pub parent_grant_id: Option<String>,
}

impl ProfileGrantCandidate {
    /// Render the candidate as the canonical wire Grant.
    ///
    /// Issuance identity is assigned by the durable issuer, never by a profile.
    pub fn to_wire(&self, grant_id: &str) -> Grant {
        Grant {
            action: self.action.as_str().to_owned(),
            consequence_ceiling: wire_consequence(self.consequence_ceiling),
            constraints: ConstraintSet {
                allowed_roots: self.constraints.allowed_roots.clone(),
                max_bytes: self.constraints.max_bytes,
                max_duration_ms: self.constraints.max_duration_ms,
                max_uses: Some(self.max_uses),
                network_hosts: self.constraints.network_hosts.clone(),
            },
            delegation_depth: self.delegation_depth,
            expires_at: self.expires_at.to_canonical_text(),
            grant_id: grant_id.to_owned(),
            issued_at: self.issued_at.to_canonical_text(),
            issuer_authority: self.issuer_authority.clone(),
            max_uses: self.max_uses,
            not_before: self.not_before.to_canonical_text(),
            parent_grant_id: self.parent_grant_id.clone(),
            policy_revision: self.policy_revision,
            resource_match: match self.resource_scope {
                ResourceScope::Exact => ResourceMatch::Exact,
                ResourceScope::Subtree => ResourceMatch::Subtree,
            },
            resource_uri: self.resource_uri.clone(),
            runtime: CanonicalRef {
                id: self.runtime_id.clone(),
                kind: EntityKind::Runtime,
                revision: Some(self.runtime_revision),
            },
            subject_scope: SubjectScope {
                run: CanonicalRef {
                    id: self.subject.run_id.clone(),
                    kind: EntityKind::Run,
                    revision: None,
                },
                agent_session: self
                    .subject
                    .agent_session_id
                    .as_ref()
                    .map(|id| CanonicalRef {
                        id: id.clone(),
                        kind: EntityKind::AgentSession,
                        revision: Some(1),
                    }),
            },
        }
    }

    /// Prove the candidate satisfies the existing Grant envelope contract.
    pub fn validate(
        &self,
        grant_id: &str,
        trusted_extensions: &[&str],
        hierarchical_resource: bool,
    ) -> Result<ValidatedGrant, PolicyValidationError> {
        ValidatedGrant::from_wire(
            &self.to_wire(grant_id),
            trusted_extensions,
            hierarchical_resource,
        )
    }
}

type PresetEntry = (&'static str, &'static [ResourceAuthority]);

/// `Safe`: low-risk project/workspace observation only, ceiling `C0`.
const SAFE_ENTRIES: &[PresetEntry] = &[
    ("files.read", &[ResourceAuthority::Project]),
    ("files.metadata", &[ResourceAuthority::Project]),
    ("git.read", &[ResourceAuthority::Git]),
    ("artifact.read", &[ResourceAuthority::Artifact]),
];

/// `Standard`: project read/write, local artifacts, isolated sandbox process, `C1`.
const STANDARD_ENTRIES: &[PresetEntry] = &[
    ("files.read", &[ResourceAuthority::Project]),
    ("files.metadata", &[ResourceAuthority::Project]),
    ("files.create", &[ResourceAuthority::Project]),
    ("files.write", &[ResourceAuthority::Project]),
    ("files.move", &[ResourceAuthority::Project]),
    ("files.delete", &[ResourceAuthority::Project]),
    ("git.read", &[ResourceAuthority::Git]),
    ("artifact.read", &[ResourceAuthority::Artifact]),
    ("artifact.create", &[ResourceAuthority::Artifact]),
    ("artifact.delete", &[ResourceAuthority::Artifact]),
    ("process.inspect", &[ResourceAuthority::Sandbox]),
    ("process.spawn", &[ResourceAuthority::Sandbox]),
    ("process.signal", &[ResourceAuthority::Sandbox]),
];

/// `Developer`: adds bounded runtime process/PTY, `git.modify`, sandbox control, `C2`.
const DEVELOPER_ENTRIES: &[PresetEntry] = &[
    ("files.read", &[ResourceAuthority::Project]),
    ("files.metadata", &[ResourceAuthority::Project]),
    ("files.create", &[ResourceAuthority::Project]),
    ("files.write", &[ResourceAuthority::Project]),
    ("files.move", &[ResourceAuthority::Project]),
    ("files.delete", &[ResourceAuthority::Project]),
    ("git.read", &[ResourceAuthority::Git]),
    ("git.modify", &[ResourceAuthority::Git]),
    ("artifact.read", &[ResourceAuthority::Artifact]),
    ("artifact.create", &[ResourceAuthority::Artifact]),
    ("artifact.delete", &[ResourceAuthority::Artifact]),
    (
        "process.inspect",
        &[ResourceAuthority::Runtime, ResourceAuthority::Sandbox],
    ),
    (
        "process.spawn",
        &[ResourceAuthority::Runtime, ResourceAuthority::Sandbox],
    ),
    (
        "process.signal",
        &[ResourceAuthority::Runtime, ResourceAuthority::Sandbox],
    ),
    (
        "pty.open",
        &[ResourceAuthority::Runtime, ResourceAuthority::Sandbox],
    ),
    (
        "pty.write",
        &[ResourceAuthority::Runtime, ResourceAuthority::Sandbox],
    ),
    (
        "pty.resize",
        &[ResourceAuthority::Runtime, ResourceAuthority::Sandbox],
    ),
    ("runtime.inspect", &[ResourceAuthority::Runtime]),
    ("sandbox.create", &[ResourceAuthority::Sandbox]),
    ("sandbox.manage", &[ResourceAuthority::Sandbox]),
    ("sandbox.destroy", &[ResourceAuthority::Sandbox]),
    ("clipboard.read", &[ResourceAuthority::Runtime]),
];

/// Authority- or credential-expanding actions no preset may compile.
const PRESET_HARD_DENY_ACTIONS: &[&str] = &[
    "identity.manage",
    "grant.delegate",
    "grant.issue",
    "grant.revoke",
    "policy.modify",
    "secret.reveal",
];

fn table_ceiling(
    entries: &[PresetEntry],
    action: &str,
    authority: ResourceAuthority,
    ceiling: ConsequenceClass,
) -> Option<ConsequenceClass> {
    if PRESET_HARD_DENY_ACTIONS.contains(&action) {
        return None;
    }
    entries
        .iter()
        .any(|(entry_action, authorities)| {
            *entry_action == action && authorities.contains(&authority)
        })
        .then_some(ceiling)
}

/// Effective consequence ceiling a preset may compile for one exact action.
///
/// Returns `None` when the preset does not cover the action/authority pair, when
/// the action is preset hard-denied, or when the profile is `Custom`, which
/// derives every bound from explicit trusted granular rules instead.
pub fn preset_ceiling(
    profile: PermissionProfile,
    action: &str,
    authority: ResourceAuthority,
) -> Option<ConsequenceClass> {
    match profile {
        PermissionProfile::Safe => {
            table_ceiling(SAFE_ENTRIES, action, authority, ConsequenceClass::C0)
        }
        PermissionProfile::Standard => {
            table_ceiling(STANDARD_ENTRIES, action, authority, ConsequenceClass::C1)
        }
        PermissionProfile::Developer => {
            table_ceiling(DEVELOPER_ENTRIES, action, authority, ConsequenceClass::C2)
        }
        PermissionProfile::Autonomous => {
            (!PRESET_HARD_DENY_ACTIONS.contains(&action)).then_some(ConsequenceClass::C2)
        }
        PermissionProfile::Custom => None,
    }
}

fn wire_consequence(value: ConsequenceClass) -> WireConsequenceClass {
    match value {
        ConsequenceClass::C0 => WireConsequenceClass::C0,
        ConsequenceClass::C1 => WireConsequenceClass::C1,
        ConsequenceClass::C2 => WireConsequenceClass::C2,
        ConsequenceClass::C3 => WireConsequenceClass::C3,
        ConsequenceClass::C4 => WireConsequenceClass::C4,
    }
}

fn valid_issuer_authority(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value.trim() == value
        && !value.chars().any(char::is_control)
}

/// Intersect every matching trusted `Custom` rule without ever widening.
///
/// A rule matches only when its exact action and resource authority match the
/// intent and its scope covers the requested scope. Matching rules combine by
/// intersection: the lowest consequence ceiling wins and the constraint ceilings
/// must remain satisfiable together. A malformed rule, a missing match, an
/// unsatisfiable intersection, or a request wider than the resulting ceiling
/// fails closed.
fn custom_ceiling(
    intent: &ProfileIntent<'_>,
    action: &str,
    authority: ResourceAuthority,
    rules: &[ProfileRule<'_>],
    trusted_extensions: &[&str],
) -> Result<ConsequenceClass, ProfileCompileError> {
    let mut ceiling: Option<ConsequenceClass> = None;
    let mut constraints: Option<ValidatedConstraints> = None;
    for rule in rules {
        if rule.action.contains('*') || rule.resource_authorities.is_empty() {
            return Err(ProfileCompileError::InvalidRule);
        }
        Action::parse(rule.action, trusted_extensions)
            .map_err(|_| ProfileCompileError::InvalidRule)?;
        if rule.action != action || !rule.resource_authorities.contains(&authority) {
            continue;
        }
        if rule.resource_scope == ResourceScope::Exact
            && intent.resource_scope == ResourceScope::Subtree
        {
            continue;
        }
        ceiling = Some(match ceiling {
            Some(current) => current.min(rule.consequence_ceiling),
            None => rule.consequence_ceiling,
        });
        constraints = Some(match constraints {
            Some(current) => current
                .intersect(&rule.constraint_ceiling)
                .map_err(|_| ProfileCompileError::ConstraintConflict)?,
            None => rule.constraint_ceiling.clone(),
        });
    }
    let Some(ceiling) = ceiling else {
        return Err(ProfileCompileError::RuleMissing);
    };
    let constraint_ceiling = constraints.ok_or(ProfileCompileError::InvalidRule)?;
    if !intent.constraints.is_subset_of(&constraint_ceiling) {
        return Err(ProfileCompileError::ConstraintConflict);
    }
    Ok(ceiling)
}

/// Compile one trusted profile intent into bounded Grant candidates.
///
/// The intent is bound to one exact action, one canonical resource, one exact
/// run and optional agent session, one exact runtime revision, one explicit
/// resource scope, validated constraints, one finite trusted time window, one
/// positive finite use budget, one issuer authority, and one policy revision.
///
/// Compilation is pure and fails closed. It does not classify risk itself: the
/// authoritative consequence always comes from [`classify_consequence`], so a
/// profile cannot under-declare consequence. The result is one bounded Grant
/// candidate that must still pass [`ValidatedGrant`] validation, durable
/// issuance, mandatory authority intersection, runtime-capability negotiation,
/// and pre-side-effect admission before anything happens.
pub fn compile_profile(
    profile_name: &str,
    intent: &ProfileIntent<'_>,
    rules: &[ProfileRule<'_>],
    trusted_extensions: &[&str],
) -> Result<Vec<ProfileGrantCandidate>, ProfileCompileError> {
    let profile = PermissionProfile::parse(profile_name)?;
    if intent.provenance_trust != ProvenanceTrust::Trusted {
        return Err(ProfileCompileError::UntrustedProvenance);
    }
    if intent.action.contains('*') || intent.resource_uri.contains('*') {
        return Err(ProfileCompileError::WildcardNotPermitted);
    }
    if intent.max_uses == 0 || intent.max_uses > MAX_PROFILE_USES {
        return Err(ProfileCompileError::InvalidIntent);
    }
    if intent.policy_revision == 0 || !valid_issuer_authority(intent.issuer_authority) {
        return Err(ProfileCompileError::InvalidIntent);
    }
    if !canonical_uuid_v7(&intent.subject.run_id)
        || intent
            .subject
            .agent_session_id
            .as_deref()
            .is_some_and(|id| !canonical_uuid_v7(id))
        || !canonical_uuid_v7(intent.runtime_id)
        || intent.runtime_revision == 0
    {
        return Err(ProfileCompileError::InvalidIntent);
    }
    let issued_at = CanonicalUtcSecond::parse(intent.issued_at)
        .map_err(|_| ProfileCompileError::InvalidIntent)?;
    let not_before = CanonicalUtcSecond::parse(intent.not_before)
        .map_err(|_| ProfileCompileError::InvalidIntent)?;
    let expires_at = CanonicalUtcSecond::parse(intent.expires_at)
        .map_err(|_| ProfileCompileError::InvalidIntent)?;
    if issued_at > not_before || not_before >= expires_at {
        return Err(ProfileCompileError::InvalidIntent);
    }
    let window = canonical_second_index(expires_at) - canonical_second_index(not_before);
    if !(1..=MAX_PROFILE_WINDOW_SECONDS).contains(&window) {
        return Err(ProfileCompileError::InvalidIntent);
    }
    if intent
        .constraints
        .max_uses
        .is_some_and(|declared| declared != intent.max_uses)
    {
        return Err(ProfileCompileError::ConstraintConflict);
    }

    let action = Action::parse(intent.action, trusted_extensions)
        .map_err(|_| ProfileCompileError::UnknownAction)?;
    let resource = CanonicalResource::parse(intent.resource_uri)
        .map_err(|_| ProfileCompileError::InvalidResource)?;
    validate_action_resource(&action, &resource)
        .map_err(|_| ProfileCompileError::ResourceAuthorityMismatch)?;
    let authoritative = classify_consequence(&action, intent.facts)
        .map_err(|_| ProfileCompileError::UnknownConsequence)?;
    let ceiling = match profile {
        PermissionProfile::Custom => custom_ceiling(
            intent,
            action.as_str(),
            resource.authority(),
            rules,
            trusted_extensions,
        )?,
        _ => preset_ceiling(profile, action.as_str(), resource.authority())
            .ok_or(ProfileCompileError::ActionNotPermittedByProfile)?,
    };
    if authoritative > ceiling {
        return Err(ProfileCompileError::ConsequenceExceeded);
    }

    let constraints = ValidatedConstraints::from_parts(
        intent.constraints.allowed_roots.clone(),
        intent.constraints.max_bytes,
        intent.constraints.max_duration_ms,
        Some(intent.max_uses),
        intent.constraints.network_hosts.clone(),
    )
    .map_err(|_| ProfileCompileError::InvalidIntent)?;

    Ok(vec![ProfileGrantCandidate {
        profile,
        subject: intent.subject.clone(),
        action,
        resource,
        resource_uri: intent.resource_uri.to_owned(),
        resource_scope: intent.resource_scope,
        runtime_id: intent.runtime_id.to_owned(),
        runtime_revision: intent.runtime_revision,
        constraints,
        consequence_ceiling: authoritative,
        issuer_authority: intent.issuer_authority.to_owned(),
        policy_revision: intent.policy_revision,
        issued_at,
        not_before,
        expires_at,
        max_uses: intent.max_uses,
        delegation_depth: 0,
        parent_grant_id: None,
    }])
}

/// Whole-second index of one canonical UTC second, derived from canonical text.
fn canonical_second_index(value: CanonicalUtcSecond) -> i64 {
    let text = value.to_canonical_text();
    let bytes = text.as_bytes();
    let year = decimal(&bytes[0..4]);
    let month = decimal(&bytes[5..7]);
    let day = decimal(&bytes[8..10]);
    let hour = decimal(&bytes[11..13]);
    let minute = decimal(&bytes[14..16]);
    let second = decimal(&bytes[17..19]);
    days_from_civil(year, month, day) * 86_400 + hour * 3_600 + minute * 60 + second
}

fn decimal(bytes: &[u8]) -> i64 {
    bytes.iter().fold(0, |value, byte| {
        value * 10 + i64::from(byte.saturating_sub(b'0'))
    })
}

fn days_from_civil(year: i64, month: i64, day: i64) -> i64 {
    let adjusted = if month <= 2 { year - 1 } else { year };
    let era = if adjusted >= 0 {
        adjusted
    } else {
        adjusted - 399
    } / 400;
    let year_of_era = adjusted - era * 400;
    let shifted_month = (month + 9) % 12;
    let day_of_year = (153 * shifted_month + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    era * 146_097 + day_of_era - 719_468
}

#[cfg(test)]
mod tests {
    use super::*;

    const RUN: &str = "01890f00-0000-7000-8000-000000000002";
    const SESSION: &str = "01890f00-0000-7000-8000-000000000003";
    const PROJECT: &str = "01890f00-0000-7000-8000-000000000004";
    const RUNTIME: &str = "01890f00-0000-7000-8000-000000000005";
    const GRANT_ID: &str = "01890f00-0000-7000-8000-000000000007";
    const SANDBOX: &str = "01890f00-0000-7000-8000-000000000008";
    const PROJECT_URI: &str = "kernux://project/01890f00-0000-7000-8000-000000000004/fs/src";

    const NOT_COVERED: ProfileCompileError = ProfileCompileError::ActionNotPermittedByProfile;
    const TOO_RISKY: ProfileCompileError = ProfileCompileError::ConsequenceExceeded;

    fn project_uri() -> String {
        format!("kernux://project/{PROJECT}/fs/src/main.rs")
    }

    fn git_uri() -> String {
        format!("kernux://git/project/{PROJECT}")
    }

    fn artifact_uri() -> String {
        format!("kernux://artifact/{PROJECT}")
    }

    fn runtime_uri() -> String {
        format!("kernux://runtime/{RUNTIME}")
    }

    fn sandbox_uri() -> String {
        format!("kernux://sandbox/{SANDBOX}")
    }

    fn secret_uri() -> String {
        "kernux://secret/ref/local-user".to_owned()
    }

    fn policy_uri() -> String {
        format!("kernux://policy/project/{PROJECT}")
    }

    fn grant_uri() -> String {
        format!("kernux://grant/{PROJECT}")
    }

    fn identity_uri() -> String {
        format!("kernux://identity/project/{PROJECT}")
    }

    fn network_uri() -> String {
        "kernux://network/origin/https/example.com/443".to_owned()
    }

    fn browser_uri() -> String {
        "kernux://browser/profile/default/origin/https/example.com/443".to_owned()
    }

    fn intent<'a>(action: &'a str, resource_uri: &'a str) -> ProfileIntent<'a> {
        ProfileIntent {
            subject: ValidatedSubjectScope {
                run_id: RUN.into(),
                agent_session_id: Some(SESSION.into()),
            },
            action,
            resource_uri,
            resource_scope: ResourceScope::Subtree,
            runtime_id: RUNTIME,
            runtime_revision: 2,
            constraints: ValidatedConstraints::from_parts(
                Some(vec!["src".into()]),
                Some(4096),
                Some(30_000),
                Some(4),
                None,
            )
            .unwrap(),
            facts: TrustedConsequenceFacts::default(),
            issuer_authority: "local-user",
            policy_revision: 7,
            issued_at: "2026-09-21T10:00:00Z",
            not_before: "2026-09-21T10:00:00Z",
            expires_at: "2026-09-21T11:00:00Z",
            max_uses: 4,
            provenance_trust: ProvenanceTrust::Trusted,
        }
    }

    fn with_facts<'a>(
        action: &'a str,
        resource_uri: &'a str,
        facts: TrustedConsequenceFacts,
    ) -> ProfileIntent<'a> {
        ProfileIntent {
            facts,
            ..intent(action, resource_uri)
        }
    }

    fn expect_denied(profile: &str, action: &str, resource: &str, expected: ProfileCompileError) {
        assert_eq!(
            compile_profile(profile, &intent(action, resource), &[], &[]),
            Err(expected),
            "{profile} {action}"
        );
    }

    fn candidate() -> ProfileGrantCandidate {
        ProfileGrantCandidate {
            profile: PermissionProfile::Standard,
            subject: ValidatedSubjectScope {
                run_id: RUN.into(),
                agent_session_id: Some(SESSION.into()),
            },
            action: Action::parse("files.write", &[]).unwrap(),
            resource: CanonicalResource::parse(PROJECT_URI).unwrap(),
            resource_uri: PROJECT_URI.into(),
            resource_scope: ResourceScope::Subtree,
            runtime_id: RUNTIME.into(),
            runtime_revision: 2,
            constraints: ValidatedConstraints::from_parts(
                Some(vec!["src".into()]),
                Some(4096),
                Some(30_000),
                Some(4),
                None,
            )
            .unwrap(),
            consequence_ceiling: ConsequenceClass::C1,
            issuer_authority: "local-user".into(),
            policy_revision: 7,
            issued_at: CanonicalUtcSecond::parse("2026-09-21T10:00:00Z").unwrap(),
            not_before: CanonicalUtcSecond::parse("2026-09-21T10:00:00Z").unwrap(),
            expires_at: CanonicalUtcSecond::parse("2026-09-21T11:00:00Z").unwrap(),
            max_uses: 4,
            delegation_depth: 0,
            parent_grant_id: None,
        }
    }

    #[test]
    fn profile_vocabulary_is_closed_and_unambiguous() {
        for profile in PermissionProfile::ALL {
            assert_eq!(PermissionProfile::parse(profile.as_str()), Ok(profile));
        }
        for invalid in [
            "safe",
            "SAFE",
            "Safe ",
            " Safe",
            "Standard\t",
            "",
            "Admin",
            "custom",
            "None",
            "Autononous",
            "Developer2",
            "Developer\r\n",
        ] {
            assert_eq!(
                PermissionProfile::parse(invalid),
                Err(ProfileCompileError::UnknownProfile),
                "{invalid:?}"
            );
        }
    }

    #[test]
    fn preset_ceilings_are_explicit_and_not_inherited() {
        let project = ResourceAuthority::Project;
        assert_eq!(
            preset_ceiling(PermissionProfile::Safe, "files.read", project),
            Some(ConsequenceClass::C0)
        );
        assert_eq!(
            preset_ceiling(PermissionProfile::Safe, "files.write", project),
            None
        );
        assert_eq!(
            preset_ceiling(PermissionProfile::Standard, "files.write", project),
            Some(ConsequenceClass::C1)
        );
        assert_eq!(
            preset_ceiling(
                PermissionProfile::Standard,
                "process.spawn",
                ResourceAuthority::Runtime
            ),
            None
        );
        assert_eq!(
            preset_ceiling(
                PermissionProfile::Standard,
                "process.spawn",
                ResourceAuthority::Sandbox
            ),
            Some(ConsequenceClass::C1)
        );
        assert_eq!(
            preset_ceiling(
                PermissionProfile::Developer,
                "git.modify",
                ResourceAuthority::Git
            ),
            Some(ConsequenceClass::C2)
        );
        assert_eq!(
            preset_ceiling(
                PermissionProfile::Developer,
                "git.publish",
                ResourceAuthority::Git
            ),
            None
        );
        assert_eq!(
            preset_ceiling(
                PermissionProfile::Autonomous,
                "git.publish",
                ResourceAuthority::Git
            ),
            Some(ConsequenceClass::C2)
        );
        assert_eq!(
            preset_ceiling(PermissionProfile::Custom, "files.read", project),
            None
        );
        for action in [
            "files.*",
            "provider.write",
            "ext.acme.observe",
            "policy.inspect.extra",
        ] {
            for profile in [
                PermissionProfile::Safe,
                PermissionProfile::Standard,
                PermissionProfile::Developer,
            ] {
                assert_eq!(preset_ceiling(profile, action, project), None, "{action}");
            }
        }
    }

    #[test]
    fn every_preset_hard_denies_authority_expanding_actions() {
        for action in PRESET_HARD_DENY_ACTIONS {
            for authority in [
                ResourceAuthority::Project,
                ResourceAuthority::Runtime,
                ResourceAuthority::Sandbox,
                ResourceAuthority::Secret,
                ResourceAuthority::Policy,
                ResourceAuthority::Grant,
                ResourceAuthority::Identity,
            ] {
                for profile in [
                    PermissionProfile::Safe,
                    PermissionProfile::Standard,
                    PermissionProfile::Developer,
                    PermissionProfile::Autonomous,
                ] {
                    assert_eq!(
                        preset_ceiling(profile, action, authority),
                        None,
                        "{} {action}",
                        profile.as_str()
                    );
                }
            }
        }
    }

    #[test]
    fn candidates_render_into_the_canonical_grant_envelope() {
        let candidate = candidate();
        let validated = candidate.validate(GRANT_ID, &[], true).unwrap();
        assert_eq!(validated.grant_id, GRANT_ID);
        assert_eq!(validated.delegation_depth, 0);
        assert_eq!(validated.parent_grant_id, None);
        assert_eq!(validated.constraints.max_uses, Some(4));
        assert_eq!(validated.consequence_ceiling, ConsequenceClass::C1);
        assert_eq!(validated.resource_scope, ResourceScope::Subtree);
        assert_eq!(validated.subject.run_id, RUN);
        assert_eq!(validated.subject.agent_session_id.as_deref(), Some(SESSION));

        let wire = candidate.to_wire(GRANT_ID);
        assert_eq!(wire.resource_uri, PROJECT_URI);
        assert_eq!(wire.subject_scope.run.revision, None);
        assert_eq!(wire.constraints.max_uses, Some(4));
        assert_eq!(wire.issued_at, "2026-09-21T10:00:00Z");
        assert!(candidate.validate("not-a-uuid", &[], true).is_err());

        let exact = ProfileGrantCandidate {
            resource_scope: ResourceScope::Exact,
            ..candidate
        };
        assert_eq!(exact.to_wire(GRANT_ID).resource_match, ResourceMatch::Exact);
    }

    #[test]
    fn safe_profile_compiles_only_low_risk_project_observation() {
        let project = project_uri();
        for action in ["files.read", "files.metadata"] {
            let candidates = compile_profile("Safe", &intent(action, &project), &[], &[]).unwrap();
            assert_eq!(candidates.len(), 1);
            assert_eq!(candidates[0].consequence_ceiling, ConsequenceClass::C0);
            assert_eq!(candidates[0].delegation_depth, 0);
            assert_eq!(candidates[0].parent_grant_id, None);
        }
        for (action, resource) in [
            ("files.write", project.clone()),
            ("files.delete", project.clone()),
            ("process.spawn", runtime_uri()),
            ("pty.open", runtime_uri()),
            ("network.connect", network_uri()),
            ("browser.navigate", browser_uri()),
            ("clipboard.write", runtime_uri()),
            ("secret.use", secret_uri()),
            ("policy.modify", policy_uri()),
            ("grant.issue", grant_uri()),
            ("identity.manage", identity_uri()),
            ("artifact.export", artifact_uri()),
        ] {
            expect_denied("Safe", action, &resource, NOT_COVERED);
        }
        let sensitive = with_facts(
            "files.read",
            &project,
            TrustedConsequenceFacts {
                sensitive_data: true,
                ..TrustedConsequenceFacts::default()
            },
        );
        assert_eq!(
            compile_profile("Safe", &sensitive, &[], &[]),
            Err(TOO_RISKY)
        );
    }

    #[test]
    fn standard_profile_adds_project_writes_and_isolated_sandbox_process() {
        let project = project_uri();
        for action in ["files.write", "files.create", "files.move", "files.delete"] {
            let uri = project.as_str();
            assert!(compile_profile("Standard", &intent(action, uri), &[], &[]).is_ok());
        }
        let sandbox = sandbox_uri();
        assert!(compile_profile("Standard", &intent("process.spawn", &sandbox), &[], &[]).is_ok());
        let runtime = runtime_uri();
        expect_denied("Standard", "process.spawn", &runtime, NOT_COVERED);
        for (action, resource) in [
            ("git.modify", git_uri()),
            ("git.publish", git_uri()),
            ("git.admin", git_uri()),
            ("secret.use", secret_uri()),
            ("grant.issue", grant_uri()),
            ("network.connect", network_uri()),
            ("browser.commit", browser_uri()),
            ("computer.input", runtime_uri()),
            ("sandbox.create", sandbox_uri()),
        ] {
            expect_denied("Standard", action, &resource, NOT_COVERED);
        }
    }

    #[test]
    fn developer_profile_adds_bounded_process_and_git_modify_only() {
        let git = git_uri();
        assert!(compile_profile("Developer", &intent("git.modify", &git), &[], &[]).is_ok());
        for action in ["process.spawn", "pty.open", "pty.write"] {
            let runtime = runtime_uri();
            assert!(compile_profile("Developer", &intent(action, &runtime), &[], &[]).is_ok());
        }
        let sandbox = sandbox_uri();
        assert!(
            compile_profile("Developer", &intent("sandbox.manage", &sandbox), &[], &[]).is_ok()
        );
        for (action, resource) in [
            ("git.publish", git_uri()),
            ("git.admin", git_uri()),
            ("secret.use", secret_uri()),
            ("secret.reveal", secret_uri()),
            ("policy.modify", policy_uri()),
            ("grant.delegate", grant_uri()),
            ("identity.manage", identity_uri()),
            ("network.connect", network_uri()),
            ("browser.commit", browser_uri()),
            ("artifact.export", artifact_uri()),
            ("runtime.manage", runtime_uri()),
            ("computer.input", runtime_uri()),
            ("clipboard.write", runtime_uri()),
        ] {
            expect_denied("Developer", action, &resource, NOT_COVERED);
        }
        let privileged = with_facts(
            "git.modify",
            &git,
            TrustedConsequenceFacts {
                privileged_host_change: true,
                ..TrustedConsequenceFacts::default()
            },
        );
        assert_eq!(
            compile_profile("Developer", &privileged, &[], &[]),
            Err(TOO_RISKY)
        );
    }

    #[test]
    fn autonomous_profile_stays_bounded_and_denies_authority_management() {
        let git = git_uri();
        let publish = with_facts(
            "git.publish",
            &git,
            TrustedConsequenceFacts {
                external_side_effect: true,
                ..TrustedConsequenceFacts::default()
            },
        );
        let candidates = compile_profile("Autonomous", &publish, &[], &[]).unwrap();
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].consequence_ceiling, ConsequenceClass::C2);
        for (action, resource) in [
            ("identity.manage", identity_uri()),
            ("grant.delegate", grant_uri()),
            ("grant.issue", grant_uri()),
            ("grant.revoke", grant_uri()),
            ("policy.modify", policy_uri()),
            ("secret.reveal", secret_uri()),
        ] {
            expect_denied("Autonomous", action, &resource, NOT_COVERED);
        }
        let project = project_uri();
        let unrecoverable = with_facts(
            "files.delete",
            &project,
            TrustedConsequenceFacts {
                unrecoverable: true,
                ..TrustedConsequenceFacts::default()
            },
        );
        assert_eq!(
            compile_profile("Autonomous", &unrecoverable, &[], &[]),
            Err(TOO_RISKY)
        );
    }

    #[test]
    fn custom_profile_compiles_only_matching_trusted_rules() {
        let project = project_uri();
        assert_eq!(
            compile_profile("Custom", &intent("files.write", &project), &[], &[]),
            Err(ProfileCompileError::RuleMissing)
        );
        let written = ProfileRule {
            action: "files.write",
            resource_authorities: &[ResourceAuthority::Project],
            resource_scope: ResourceScope::Subtree,
            consequence_ceiling: ConsequenceClass::C1,
            constraint_ceiling: ValidatedConstraints::from_parts(
                Some(vec!["src".into()]),
                Some(8192),
                None,
                None,
                None,
            )
            .unwrap(),
        };
        let candidates =
            compile_profile("Custom", &intent("files.write", &project), &[written], &[]).unwrap();
        assert_eq!(candidates.len(), 1);
        assert_eq!(candidates[0].profile, PermissionProfile::Custom);
        assert_eq!(candidates[0].consequence_ceiling, ConsequenceClass::C1);
        assert_eq!(candidates[0].constraints.max_bytes, Some(4096));
        assert_eq!(
            candidates[0]
                .validate(GRANT_ID, &[], true)
                .unwrap()
                .max_uses,
            4
        );
    }

    #[test]
    fn compiled_candidates_round_trip_through_grant_envelope_validation() {
        let candidates =
            compile_profile("Standard", &intent("files.write", &project_uri()), &[], &[]).unwrap();
        let candidate = &candidates[0];
        let validated = candidate.validate(GRANT_ID, &[], true).unwrap();
        assert_eq!(validated.grant_id, GRANT_ID);
        assert_eq!(validated.max_uses, 4);
        assert_eq!(validated.delegation_depth, 0);
        assert_eq!(validated.parent_grant_id, None);
        assert_eq!(validated.constraints.max_uses, Some(4));
        assert_eq!(validated.consequence_ceiling, ConsequenceClass::C1);
        assert_eq!(validated.resource_scope, ResourceScope::Subtree);
        assert_eq!(validated.policy_revision, 7);
        assert_eq!(validated.issuer_authority, "local-user");
        assert_eq!(validated.runtime_revision, 2);
        assert_eq!(candidate.profile, PermissionProfile::Standard);
        assert!(candidate.validate("not-a-uuid", &[], true).is_err());
    }

    #[test]
    fn compilation_is_deterministic_and_leaves_inputs_untouched() {
        let request = intent("files.write", &project_uri());
        let before = request.clone();
        let first = compile_profile("Developer", &request, &[], &[]).unwrap();
        let second = compile_profile("Developer", &request, &[], &[]).unwrap();
        assert_eq!(first, second);
        assert_eq!(request, before);
        assert_eq!(first[0].delegation_depth, 0);
        assert_eq!(first[0].parent_grant_id, None);
        assert_eq!(first[0].max_uses, request.max_uses);
        assert_eq!(first[0].constraints.max_uses, Some(request.max_uses));
        assert_eq!(first[0].resource_uri, request.resource_uri);
    }
}
