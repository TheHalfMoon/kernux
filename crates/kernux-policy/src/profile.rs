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
    ValidatedGrant, ValidatedSubjectScope,
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

#[cfg(test)]
mod tests {
    use super::*;

    const RUN: &str = "01890f00-0000-7000-8000-000000000002";
    const SESSION: &str = "01890f00-0000-7000-8000-000000000003";
    const RUNTIME: &str = "01890f00-0000-7000-8000-000000000005";
    const GRANT_ID: &str = "01890f00-0000-7000-8000-000000000007";
    const PROJECT_URI: &str = "kernux://project/01890f00-0000-7000-8000-000000000004/fs/src";

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
}
