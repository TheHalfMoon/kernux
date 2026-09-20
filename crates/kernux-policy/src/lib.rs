//! Pure, provider-neutral Kernux authorization primitives.
//!
//! This crate validates authority vocabulary. It does not persist Grants,
//! execute host operations, read secrets, or infer authority from providers.

mod semantics;
pub use semantics::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PolicyValidationError {
    InvalidAction,
    UnknownAction,
    UnknownExtensionAction,
    InvalidResource,
    UnknownResourceAuthority,
    ActionResourceMismatch,
    NonHierarchicalSubtree,
    UnknownExtensionConsequence,
    InvalidTimestamp,
    InvalidConstraint,
    ConstraintConflict,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Action(String);

impl Action {
    pub fn parse(input: &str, trusted_extensions: &[&str]) -> Result<Self, PolicyValidationError> {
        if !valid_dotted_identifier(input) {
            return Err(PolicyValidationError::InvalidAction);
        }
        if CORE_ACTIONS.contains(&input) {
            return Ok(Self(input.to_owned()));
        }
        if input.starts_with("ext.") {
            return trusted_extensions
                .contains(&input)
                .then(|| Self(input.to_owned()))
                .ok_or(PolicyValidationError::UnknownExtensionAction);
        }
        Err(PolicyValidationError::UnknownAction)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn family(&self) -> &str {
        self.0.split('.').next().expect("validated action family")
    }

    pub fn permits_authority(&self, authority: ResourceAuthority) -> bool {
        match self.family() {
            "files" => matches!(
                authority,
                ResourceAuthority::Project | ResourceAuthority::Runtime
            ),
            "process" | "pty" => {
                matches!(
                    authority,
                    ResourceAuthority::Runtime | ResourceAuthority::Sandbox
                )
            }
            "git" => authority == ResourceAuthority::Git,
            "browser" => authority == ResourceAuthority::Browser,
            "computer" | "clipboard" | "runtime" => authority == ResourceAuthority::Runtime,
            "network" => authority == ResourceAuthority::Network,
            "secret" => authority == ResourceAuthority::Secret,
            "artifact" => authority == ResourceAuthority::Artifact,
            "sandbox" => authority == ResourceAuthority::Sandbox,
            "tool" => matches!(authority, ResourceAuthority::Tool | ResourceAuthority::Ext),
            "policy" => authority == ResourceAuthority::Policy,
            "grant" => matches!(
                authority,
                ResourceAuthority::Grant | ResourceAuthority::Policy
            ),
            "identity" => authority == ResourceAuthority::Identity,
            "ext" => authority == ResourceAuthority::Ext,
            _ => false,
        }
    }
}

const CORE_ACTIONS: &[&str] = &[
    "files.read",
    "files.metadata",
    "files.create",
    "files.write",
    "files.move",
    "files.delete",
    "process.inspect",
    "process.spawn",
    "process.signal",
    "pty.open",
    "pty.write",
    "pty.resize",
    "git.read",
    "git.modify",
    "git.publish",
    "git.admin",
    "browser.observe",
    "browser.navigate",
    "browser.interact",
    "browser.download",
    "browser.upload",
    "browser.commit",
    "computer.observe",
    "computer.input",
    "clipboard.read",
    "clipboard.write",
    "network.connect",
    "network.send",
    "secret.use",
    "secret.reveal",
    "artifact.read",
    "artifact.create",
    "artifact.export",
    "artifact.delete",
    "runtime.inspect",
    "runtime.manage",
    "sandbox.create",
    "sandbox.manage",
    "sandbox.destroy",
    "tool.invoke",
    "policy.inspect",
    "policy.modify",
    "grant.issue",
    "grant.revoke",
    "grant.delegate",
    "identity.inspect",
    "identity.manage",
];

fn valid_dotted_identifier(value: &str) -> bool {
    let mut count = 0;
    for token in value.split('.') {
        count += 1;
        let mut bytes = token.bytes();
        if !matches!(bytes.next(), Some(b'a'..=b'z'))
            || !bytes.all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
        {
            return false;
        }
    }
    count >= 2
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceAuthority {
    Project,
    Runtime,
    Artifact,
    Network,
    Secret,
    Browser,
    Git,
    System,
    Policy,
    Grant,
    Tool,
    Sandbox,
    Identity,
    Ext,
}

impl ResourceAuthority {
    fn parse(value: &str) -> Result<Self, PolicyValidationError> {
        match value {
            "project" => Ok(Self::Project),
            "runtime" => Ok(Self::Runtime),
            "artifact" => Ok(Self::Artifact),
            "network" => Ok(Self::Network),
            "secret" => Ok(Self::Secret),
            "browser" => Ok(Self::Browser),
            "git" => Ok(Self::Git),
            "system" => Ok(Self::System),
            "policy" => Ok(Self::Policy),
            "grant" => Ok(Self::Grant),
            "tool" => Ok(Self::Tool),
            "sandbox" => Ok(Self::Sandbox),
            "identity" => Ok(Self::Identity),
            "ext" => Ok(Self::Ext),
            _ => Err(PolicyValidationError::UnknownResourceAuthority),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CanonicalResource {
    authority: ResourceAuthority,
    segments: Vec<String>,
}

impl CanonicalResource {
    pub fn parse(input: &str) -> Result<Self, PolicyValidationError> {
        if !input.is_ascii() || input.contains(['?', '#']) {
            return Err(PolicyValidationError::InvalidResource);
        }
        let tail = input
            .strip_prefix("kernux://")
            .ok_or(PolicyValidationError::InvalidResource)?;
        let (authority, path) = tail
            .split_once('/')
            .ok_or(PolicyValidationError::InvalidResource)?;
        if authority.is_empty() || authority.contains(['@', ':']) || path.is_empty() {
            return Err(PolicyValidationError::InvalidResource);
        }
        let authority = ResourceAuthority::parse(authority)?;
        let segments = path
            .split('/')
            .map(parse_segment)
            .collect::<Result<Vec<_>, _>>()?;
        validate_shape(authority, &segments)?;
        Ok(Self {
            authority,
            segments,
        })
    }

    pub fn authority(&self) -> ResourceAuthority {
        self.authority
    }

    pub fn segments(&self) -> &[String] {
        &self.segments
    }

    pub fn matches(
        &self,
        candidate: &Self,
        scope: ResourceScope,
        hierarchical: bool,
    ) -> Result<bool, PolicyValidationError> {
        if self.authority != candidate.authority {
            return Ok(false);
        }
        match scope {
            ResourceScope::Exact => Ok(self.segments == candidate.segments),
            ResourceScope::Subtree if hierarchical => {
                Ok(candidate.segments.starts_with(&self.segments))
            }
            ResourceScope::Subtree => Err(PolicyValidationError::NonHierarchicalSubtree),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceScope {
    Exact,
    Subtree,
}

pub fn validate_action_resource(
    action: &Action,
    resource: &CanonicalResource,
) -> Result<(), PolicyValidationError> {
    action
        .permits_authority(resource.authority())
        .then_some(())
        .ok_or(PolicyValidationError::ActionResourceMismatch)
}

fn parse_segment(raw: &str) -> Result<String, PolicyValidationError> {
    if raw.is_empty() {
        return Err(PolicyValidationError::InvalidResource);
    }
    let bytes = raw.as_bytes();
    let mut decoded = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        let byte = bytes[index];
        if byte == b'%' {
            if index + 2 >= bytes.len()
                || !is_upper_hex(bytes[index + 1])
                || !is_upper_hex(bytes[index + 2])
            {
                return Err(PolicyValidationError::InvalidResource);
            }
            let value = (hex_value(bytes[index + 1]) << 4) | hex_value(bytes[index + 2]);
            if is_unreserved(value) || matches!(value, 0 | b'/' | b'?' | b'#') {
                return Err(PolicyValidationError::InvalidResource);
            }
            decoded.push(value);
            index += 3;
        } else {
            if !is_pchar(byte) {
                return Err(PolicyValidationError::InvalidResource);
            }
            decoded.push(byte);
            index += 1;
        }
    }
    let decoded = String::from_utf8(decoded).map_err(|_| PolicyValidationError::InvalidResource)?;
    if matches!(decoded.as_str(), "." | "..") {
        return Err(PolicyValidationError::InvalidResource);
    }
    Ok(decoded)
}

fn is_unreserved(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~')
}

fn is_pchar(byte: u8) -> bool {
    is_unreserved(byte)
        || matches!(
            byte,
            b'!' | b'$'
                | b'&'
                | b'\''
                | b'('
                | b')'
                | b'*'
                | b'+'
                | b','
                | b';'
                | b'='
                | b':'
                | b'@'
        )
}

fn is_upper_hex(byte: u8) -> bool {
    byte.is_ascii_digit() || matches!(byte, b'A'..=b'F')
}

fn hex_value(byte: u8) -> u8 {
    match byte {
        b'0'..=b'9' => byte - b'0',
        b'A'..=b'F' => byte - b'A' + 10,
        _ => unreachable!("validated uppercase hex"),
    }
}

fn validate_shape(
    authority: ResourceAuthority,
    segments: &[String],
) -> Result<(), PolicyValidationError> {
    let uuid_at = |index: usize| {
        segments
            .get(index)
            .is_some_and(|value| canonical_uuid_v7(value))
    };
    let valid = match authority {
        ResourceAuthority::Project | ResourceAuthority::Runtime => uuid_at(0),
        ResourceAuthority::Artifact | ResourceAuthority::Grant => segments.len() == 1 && uuid_at(0),
        ResourceAuthority::Git | ResourceAuthority::Policy => {
            segments.len() == 2 && segments[0] == "project" && uuid_at(1)
        }
        ResourceAuthority::Identity => {
            segments.len() == 2
                && matches!(segments[0].as_str(), "runtime" | "project")
                && uuid_at(1)
        }
        ResourceAuthority::Network => valid_origin_shape(segments),
        ResourceAuthority::Browser => {
            segments.len() == 2 && segments[0] == "profile"
                || segments.len() == 6
                    && segments[0] == "profile"
                    && segments[2] == "origin"
                    && valid_origin_tail(&segments[3..])
        }
        ResourceAuthority::Secret => segments.len() == 2 && segments[0] == "ref",
        ResourceAuthority::System => segments.len() == 2,
        ResourceAuthority::Tool | ResourceAuthority::Sandbox => segments.len() == 1,
        ResourceAuthority::Ext => segments.len() >= 2 && valid_registry_token(&segments[0]),
    };
    valid
        .then_some(())
        .ok_or(PolicyValidationError::InvalidResource)
}

fn valid_origin_shape(segments: &[String]) -> bool {
    segments.len() == 4 && segments[0] == "origin" && valid_origin_tail(&segments[1..])
}

fn valid_origin_tail(segments: &[String]) -> bool {
    if segments.len() != 3 || !matches!(segments[0].as_str(), "http" | "https") {
        return false;
    }
    let host = &segments[1];
    let port = &segments[2];
    !host.is_empty()
        && host.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || matches!(byte, b'.' | b'-' | b':' | b'[' | b']')
        })
        && port.parse::<u16>().is_ok()
        && (port == "0" || !port.starts_with('0'))
}

fn valid_registry_token(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

fn canonical_uuid_v7(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 36
        || ![8, 13, 18, 23]
            .into_iter()
            .all(|index| bytes[index] == b'-')
        || bytes[14] != b'7'
        || !matches!(bytes[19], b'8' | b'9' | b'a' | b'b')
    {
        return false;
    }
    bytes.iter().enumerate().all(|(index, byte)| {
        [8, 13, 18, 23].contains(&index) || byte.is_ascii_digit() || matches!(byte, b'a'..=b'f')
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROJECT: &str = "01890f3a-7b2c-7d45-8a61-3c4e5f607182";
    const RUNTIME: &str = "01890f3a-7b2f-7e55-aa66-6f708192a3b5";

    #[test]
    fn core_actions_are_exact_and_extensions_require_trust() {
        assert_eq!(Action::parse("files.write", &[]).unwrap().family(), "files");
        for invalid in [
            "files.*",
            "Files.write",
            "files..write",
            "files.write ",
            "provider.write",
        ] {
            assert!(Action::parse(invalid, &[]).is_err(), "{invalid}");
        }
        assert_eq!(
            Action::parse("ext.acme.observe", &["ext.acme.observe"])
                .unwrap()
                .as_str(),
            "ext.acme.observe"
        );
        assert_eq!(
            Action::parse("ext.acme.observe", &[]),
            Err(PolicyValidationError::UnknownExtensionAction)
        );
    }

    #[test]
    fn action_resource_matrix_is_closed() {
        let project =
            CanonicalResource::parse(&format!("kernux://project/{PROJECT}/fs/src")).unwrap();
        let runtime = CanonicalResource::parse(&format!("kernux://runtime/{RUNTIME}")).unwrap();
        assert!(
            validate_action_resource(&Action::parse("files.read", &[]).unwrap(), &project).is_ok()
        );
        assert!(
            validate_action_resource(&Action::parse("computer.input", &[]).unwrap(), &runtime)
                .is_ok()
        );
        assert_eq!(
            validate_action_resource(&Action::parse("git.publish", &[]).unwrap(), &project),
            Err(PolicyValidationError::ActionResourceMismatch)
        );
    }

    #[test]
    fn every_core_action_family_has_only_its_frozen_primary_authority() {
        let cases = [
            ("files.read", ResourceAuthority::Project),
            ("process.spawn", ResourceAuthority::Runtime),
            ("pty.open", ResourceAuthority::Sandbox),
            ("git.read", ResourceAuthority::Git),
            ("browser.observe", ResourceAuthority::Browser),
            ("computer.observe", ResourceAuthority::Runtime),
            ("clipboard.read", ResourceAuthority::Runtime),
            ("network.connect", ResourceAuthority::Network),
            ("secret.use", ResourceAuthority::Secret),
            ("artifact.read", ResourceAuthority::Artifact),
            ("runtime.inspect", ResourceAuthority::Runtime),
            ("sandbox.create", ResourceAuthority::Sandbox),
            ("tool.invoke", ResourceAuthority::Tool),
            ("policy.inspect", ResourceAuthority::Policy),
            ("grant.issue", ResourceAuthority::Grant),
            ("identity.inspect", ResourceAuthority::Identity),
        ];
        for (value, authority) in cases {
            assert!(
                Action::parse(value, &[])
                    .unwrap()
                    .permits_authority(authority),
                "{value}"
            );
        }
        for action in CORE_ACTIONS {
            assert!(Action::parse(action, &[]).is_ok(), "{action}");
        }
    }

    #[test]
    fn canonical_resource_rejects_ambiguous_or_noncanonical_forms() {
        let valid = format!("kernux://project/{PROJECT}/fs/src");
        assert!(CanonicalResource::parse(&valid).is_ok());
        let encoded_utf8 = format!("kernux://project/{PROJECT}/fs/caf%C3%A9");
        assert_eq!(
            CanonicalResource::parse(&encoded_utf8)
                .unwrap()
                .segments()
                .last()
                .unwrap(),
            "café"
        );
        for invalid in [
            format!("Kernux://project/{PROJECT}/fs/src"),
            format!("kernux://project/{PROJECT}//src"),
            format!("kernux://project/{PROJECT}/fs/../src"),
            format!("kernux://project/{PROJECT}/fs/%2Fetc"),
            format!("kernux://project/{PROJECT}/fs/%41"),
            format!("kernux://project/{PROJECT}/fs/%c3%A9"),
            format!("kernux://project/{PROJECT}/fs/café"),
            format!("kernux://project/{PROJECT}/fs/src?x=1"),
            format!("kernux://project/{PROJECT}/fs/src#frag"),
        ] {
            assert_eq!(
                CanonicalResource::parse(&invalid),
                Err(PolicyValidationError::InvalidResource),
                "{invalid}"
            );
        }
    }

    #[test]
    fn uuid_bound_resources_require_canonical_v7_identity() {
        let v4 = "550e8400-e29b-41d4-a716-446655440000";
        assert!(CanonicalResource::parse(&format!("kernux://artifact/{PROJECT}")).is_ok());
        assert!(CanonicalResource::parse(&format!("kernux://git/project/{PROJECT}")).is_ok());
        assert!(CanonicalResource::parse(&format!("kernux://identity/runtime/{RUNTIME}")).is_ok());
        assert_eq!(
            CanonicalResource::parse(&format!("kernux://grant/{v4}")),
            Err(PolicyValidationError::InvalidResource)
        );
    }

    #[test]
    fn subtree_matching_is_segment_aware_and_explicitly_hierarchical() {
        let base = CanonicalResource::parse(&format!("kernux://project/{PROJECT}/fs/src")).unwrap();
        let child = CanonicalResource::parse(&format!("kernux://project/{PROJECT}/fs/src/auth.rs"))
            .unwrap();
        let sibling =
            CanonicalResource::parse(&format!("kernux://project/{PROJECT}/fs/src-old/auth.rs"))
                .unwrap();
        assert_eq!(base.matches(&child, ResourceScope::Subtree, true), Ok(true));
        assert_eq!(
            base.matches(&sibling, ResourceScope::Subtree, true),
            Ok(false)
        );
        assert_eq!(
            base.matches(&child, ResourceScope::Subtree, false),
            Err(PolicyValidationError::NonHierarchicalSubtree)
        );
        assert_eq!(base.matches(&base, ResourceScope::Exact, false), Ok(true));
    }

    #[test]
    fn network_and_extension_resources_are_closed() {
        assert!(CanonicalResource::parse("kernux://network/origin/https/example.com/443").is_ok());
        assert!(CanonicalResource::parse("kernux://browser/profile/default").is_ok());
        assert!(
            CanonicalResource::parse(
                "kernux://browser/profile/default/origin/https/example.com/443"
            )
            .is_ok()
        );
        assert!(
            CanonicalResource::parse(
                "kernux://browser/profile/default/origin/https/example.com/0443"
            )
            .is_err()
        );
        assert!(CanonicalResource::parse("kernux://browser/profile").is_err());
        assert!(CanonicalResource::parse("kernux://ext/acme/resource").is_ok());
        assert!(
            CanonicalResource::parse("kernux://network/origin/https/example.com/0443").is_err()
        );
        assert_eq!(
            CanonicalResource::parse("kernux://other/value"),
            Err(PolicyValidationError::UnknownResourceAuthority)
        );
    }
}
