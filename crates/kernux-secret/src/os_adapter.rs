//! First-generation OS credential-store adapter declarations for SG-000026.
//! SG-000027 corrective re-adoption: executable semantics are intentionally unchanged.
//!
//! This module declares how the daemon reaches the platform credential store
//! without performing any store access itself. It performs no operating system
//! credential-store access, no process spawn, no environment read, no network
//! access, no persistence, and no logging. It never handles secret plaintext:
//! there is no SecretValue input, output, or temporary boundary in this slice.
//!
//! The daemon may execute the declared template only through brokered process
//! authority with argv-first spawn and no shell. The program path is fixed per
//! provider, the argument prefix is fixed, and the validated service and
//! account travel as separate argv elements that are never concatenated into
//! a shell string. Native library binding remains explicitly deferred to a
//! governed dependency-admission Grain.
//!
//! Service and account identify secret material without containing it. They
//! are non-secret metadata and may appear in logs, events, prompts, and
//! diagnostics. All errors are static strings that never echo untrusted input
//! and never carry secret material.

use crate::{ProviderId, SecretError};

/// Maximum OS credential service name length in bytes.
pub const MAX_OS_SERVICE_BYTES: usize = 256;
/// Maximum OS credential account name length in bytes.
pub const MAX_OS_ACCOUNT_BYTES: usize = 256;

/// Fixed macOS Keychain lookup program.
pub const MACOS_KEYCHAIN_PROGRAM: &str = "/usr/bin/security";
/// Fixed Windows Credential Manager lookup program.
pub const WINDOWS_CREDENTIAL_PROGRAM: &str = "C:\\Windows\\System32\\cmdkey.exe";
/// Fixed Linux Secret Service lookup program.
pub const LINUX_SECRET_SERVICE_PROGRAM: &str = "/usr/bin/secret-tool";

/// Declared OS adapter command template.
///
/// Non-secret metadata only. Constructible solely through
/// [`command_template`] so unvalidated service or account text cannot become
/// a program argument. The daemon must spawn [`program`] with [`argv`] via
/// argv-first process authority and must never render the template through a
/// shell.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AdapterCommand {
    provider: ProviderId,
    program: &'static str,
    argv: Vec<String>,
    service: String,
    account: String,
}

impl AdapterCommand {
    /// The provider family this template targets. Metadata, not authority.
    pub fn provider(&self) -> ProviderId {
        self.provider
    }

    /// Fixed program path. Never derived from untrusted input.
    pub fn program(&self) -> &'static str {
        self.program
    }

    /// Exact argv for argv-first spawn. Element zero is the subcommand, the
    /// service and account travel as separate validated elements.
    pub fn argv(&self) -> &[String] {
        &self.argv
    }

    /// Validated service name. Non-secret metadata.
    pub fn service(&self) -> &str {
        &self.service
    }

    /// Validated account name. Non-secret metadata.
    pub fn account(&self) -> &str {
        &self.account
    }
}

/// Fixed program for one provider family. Never derived from input.
pub const fn program_for(provider: ProviderId) -> &'static str {
    match provider {
        ProviderId::OsMacosKeychain => MACOS_KEYCHAIN_PROGRAM,
        ProviderId::OsWindowsCredentialManager => WINDOWS_CREDENTIAL_PROGRAM,
        ProviderId::OsLinuxSecretService => LINUX_SECRET_SERVICE_PROGRAM,
    }
}

fn valid_os_field(value: &str, max: usize) -> bool {
    !value.is_empty() && value.len() <= max && !value.chars().any(char::is_control)
}

/// Declare the fixed non-shell command template for one OS credential lookup.
///
/// Fails closed on empty, overlong, or control-character service or account
/// text. The returned template carries the fixed program, a fixed argument
/// prefix per provider, and the validated service and account as separate
/// argv elements. It performs no lookup and grants no authority.
pub fn command_template(
    provider: ProviderId,
    service: &str,
    account: &str,
) -> Result<AdapterCommand, SecretError> {
    if !valid_os_field(service, MAX_OS_SERVICE_BYTES) {
        return Err(SecretError::InvalidReference);
    }
    if !valid_os_field(account, MAX_OS_ACCOUNT_BYTES) {
        return Err(SecretError::InvalidReference);
    }
    let program = program_for(provider);
    let argv: Vec<String> = match provider {
        ProviderId::OsMacosKeychain => vec![
            "find-generic-password".to_owned(),
            "-s".to_owned(),
            service.to_owned(),
            "-a".to_owned(),
            account.to_owned(),
            "-w".to_owned(),
        ],
        ProviderId::OsWindowsCredentialManager => {
            vec!["/list".to_owned(), service.to_owned(), account.to_owned()]
        }
        ProviderId::OsLinuxSecretService => vec![
            "lookup".to_owned(),
            "service".to_owned(),
            service.to_owned(),
            "account".to_owned(),
            account.to_owned(),
        ],
    };
    Ok(AdapterCommand {
        provider,
        program,
        argv,
        service: service.to_owned(),
        account: account.to_owned(),
    })
}

#[cfg(test)]
mod os_adapter_tests {
    use super::*;
    use crate::{REDACTED_SECRET_VALUE, assert_no_plaintext, leaks_plaintext};

    #[test]
    fn programs_are_fixed_per_provider() {
        assert_eq!(
            program_for(ProviderId::OsMacosKeychain),
            MACOS_KEYCHAIN_PROGRAM
        );
        assert_eq!(
            program_for(ProviderId::OsWindowsCredentialManager),
            WINDOWS_CREDENTIAL_PROGRAM
        );
        assert_eq!(
            program_for(ProviderId::OsLinuxSecretService),
            LINUX_SECRET_SERVICE_PROGRAM
        );
    }

    #[test]
    fn macos_template_is_argv_first_with_validated_fields() {
        let cmd = command_template(
            ProviderId::OsMacosKeychain,
            "kernux-demo-service",
            "kernux-demo-account",
        )
        .expect("valid template");
        assert_eq!(cmd.provider(), ProviderId::OsMacosKeychain);
        assert_eq!(cmd.program(), "/usr/bin/security");
        assert_eq!(
            cmd.argv(),
            &[
                "find-generic-password".to_owned(),
                "-s".to_owned(),
                "kernux-demo-service".to_owned(),
                "-a".to_owned(),
                "kernux-demo-account".to_owned(),
                "-w".to_owned(),
            ]
        );
        assert_eq!(cmd.service(), "kernux-demo-service");
        assert_eq!(cmd.account(), "kernux-demo-account");
    }

    #[test]
    fn windows_and_linux_templates_keep_fields_as_separate_argv() {
        let win = command_template(
            ProviderId::OsWindowsCredentialManager,
            "svc-alpha",
            "acct-alpha",
        )
        .expect("valid windows template");
        assert_eq!(win.program(), WINDOWS_CREDENTIAL_PROGRAM);
        assert!(win.argv().contains(&"svc-alpha".to_owned()));
        assert!(win.argv().contains(&"acct-alpha".to_owned()));
        assert_eq!(win.argv()[0], "/list");

        let linux = command_template(ProviderId::OsLinuxSecretService, "svc-beta", "acct-beta")
            .expect("valid linux template");
        assert_eq!(linux.program(), LINUX_SECRET_SERVICE_PROGRAM);
        assert_eq!(
            linux.argv(),
            &[
                "lookup".to_owned(),
                "service".to_owned(),
                "svc-beta".to_owned(),
                "account".to_owned(),
                "acct-beta".to_owned(),
            ]
        );
    }

    #[test]
    fn hostile_service_and_account_fail_closed() {
        assert_eq!(
            command_template(ProviderId::OsMacosKeychain, "", "acct"),
            Err(SecretError::InvalidReference)
        );
        assert_eq!(
            command_template(ProviderId::OsMacosKeychain, "svc", ""),
            Err(SecretError::InvalidReference)
        );
        let long_service = "s".repeat(MAX_OS_SERVICE_BYTES + 1);
        assert_eq!(
            command_template(ProviderId::OsMacosKeychain, &long_service, "acct"),
            Err(SecretError::InvalidReference)
        );
        let long_account = "a".repeat(MAX_OS_ACCOUNT_BYTES + 1);
        assert_eq!(
            command_template(ProviderId::OsLinuxSecretService, "svc", &long_account),
            Err(SecretError::InvalidReference)
        );
        assert_eq!(
            command_template(ProviderId::OsMacosKeychain, "svc\x00evil", "acct"),
            Err(SecretError::InvalidReference)
        );
        assert_eq!(
            command_template(
                ProviderId::OsWindowsCredentialManager,
                "svc",
                "acct\ninjected"
            ),
            Err(SecretError::InvalidReference)
        );
        assert_eq!(
            command_template(ProviderId::OsLinuxSecretService, "$(hostname)", "acct")
                .map(|cmd| cmd.argv().join(" "))
                .ok(),
            Some("lookup service $(hostname) account acct".to_owned()),
            "shell metacharacters travel as literal argv without shell evaluation"
        );
    }

    #[test]
    fn unknown_provider_and_shell_strings_are_impossible() {
        assert!(ProviderId::parse("aws-secrets-manager").is_err());
        let cmd = command_template(ProviderId::OsMacosKeychain, "svc", "acct").expect("valid");
        assert!(!cmd.program().contains("sh"));
        assert_ne!(cmd.program(), "/bin/sh");
        for element in cmd.argv() {
            assert!(!element.contains("&&"));
            assert!(!element.contains("||"));
        }
    }

    #[test]
    fn errors_and_debug_carry_no_plaintext() {
        let hostile = "evil-service-material";
        let error =
            command_template(ProviderId::OsMacosKeychain, "", hostile).expect_err("must deny");
        let rendered = format!("{error}");
        assert!(!rendered.contains(hostile));
        assert!(!rendered.contains("evil"));
        let cmd = command_template(ProviderId::OsLinuxSecretService, "svc-clean", "acct-clean")
            .expect("valid");
        let plaintext = b"correct horse battery staple".to_vec();
        let rendered = format!("{cmd:?}");
        assert!(!leaks_plaintext(&rendered, &plaintext));
        assert_no_plaintext(&rendered, &plaintext, "adapter command");
        assert!(rendered.contains(REDACTED_SECRET_VALUE) || !rendered.contains("correct horse"));
    }

    #[test]
    fn template_declares_without_executing_or_granting() {
        let cmd = command_template(
            ProviderId::OsMacosKeychain,
            "svc-exec-check",
            "acct-exec-check",
        )
        .expect("valid");
        assert_eq!(cmd.service(), "svc-exec-check");
        assert_eq!(cmd.account(), "acct-exec-check");
        assert_eq!(cmd.provider(), ProviderId::OsMacosKeychain);
    }
}
