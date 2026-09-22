//! Provider-neutral secret vocabulary for the Kernux secret broker.
//!
//! This crate defines opaque secret references, opaque secret handles,
//! redacted secret values, and fail-closed vocabulary errors. References and
//! handles are non-secret metadata: they identify secret material without
//! containing it and may appear in logs, events, prompts, and diagnostics.
//! Secret values are redacted in every derived rendering, are zeroized on
//! drop, and never implement Display or Serialize.
//!
//! This crate performs no operating system credential-store access, persists
//! nothing, logs nothing, spawns no processes, and grants no authority.
//! Possession of a handle authorizes nothing. Only broker admission under an
//! exact Grant can produce a bounded resolved use. Provider identities, destination classes, broker admission, and OS adapter templates arrive across SG-000026 slices; provider execution and audit events arrive in later SG-000026 slices.

#![forbid(unsafe_code)]

mod broker;
mod handle;
mod os_adapter;
mod provider;
mod redaction;

pub use broker::{
    AdmittedUse, BrokerDestination, BrokerError, BrokerRequest, EgressClass, MAX_ACCOUNT_ID_BYTES,
    MAX_OPERATION_ID_BYTES, MAX_PROJECT_ID_BYTES, admit,
};
use core::fmt;
pub use handle::{
    MAX_SECRET_REF_TOKEN_BYTES, MAX_SECRET_VALUE_BYTES, SECRET_HANDLE_HEX_LEN, SECRET_HANDLE_PREFIX,
};
pub use handle::{SecretHandle, SecretRef, SecretValue};
pub use os_adapter::{
    AdapterCommand, LINUX_SECRET_SERVICE_PROGRAM, MACOS_KEYCHAIN_PROGRAM, MAX_OS_ACCOUNT_BYTES,
    MAX_OS_SERVICE_BYTES, WINDOWS_CREDENTIAL_PROGRAM, command_template, program_for,
};
pub use provider::{
    DESTINATION_EGRESS_SUBSTITUTION, DESTINATION_HOST_COMMAND, DESTINATION_PROCESS_ENV,
    DestinationClass, EnvName, HostName, MAX_ENV_NAME_BYTES, MAX_HOST_LABEL_BYTES,
    MAX_HOST_NAME_BYTES, OS_LINUX_SECRET_SERVICE, OS_MACOS_KEYCHAIN, OS_WINDOWS_CREDENTIAL_MANAGER,
    ProviderCapabilities, ProviderId,
};
pub use redaction::{REDACTED_SECRET_VALUE, assert_no_plaintext, leaks_plaintext};

/// Fail-closed vocabulary errors for secret references, handles, values,
/// providers, and destinations.
///
/// Every message is a static string. Error values never echo untrusted input,
/// never contain secret material, and never carry a redaction burden.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecretError {
    /// A secret reference was malformed or denoted the wrong authority.
    InvalidReference,
    /// A secret handle was malformed.
    InvalidHandle,
    /// A destination class or destination name was malformed.
    InvalidDestination,
    /// A secret value was empty.
    EmptyValue,
    /// A secret value exceeded the maximum retained length.
    ValueTooLarge,
    /// A provider identity was unknown.
    UnknownProvider,
}

impl fmt::Display for SecretError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            SecretError::InvalidReference => "invalid secret reference",
            SecretError::InvalidHandle => "invalid secret handle",
            SecretError::InvalidDestination => "invalid secret destination",
            SecretError::EmptyValue => "empty secret value",
            SecretError::ValueTooLarge => "secret value exceeds maximum length",
            SecretError::UnknownProvider => "unknown secret provider",
        };
        write!(f, "{message}")
    }
}

impl std::error::Error for SecretError {}

#[cfg(test)]
mod error_tests {
    use super::SecretError;

    #[test]
    fn messages_are_static_and_never_echo_input() {
        let hostile = "hnd_evil-ref";
        let cases = [
            SecretError::InvalidReference,
            SecretError::InvalidHandle,
            SecretError::InvalidDestination,
            SecretError::EmptyValue,
            SecretError::ValueTooLarge,
            SecretError::UnknownProvider,
        ];
        for error in cases {
            let rendered = format!("{error}");
            assert!(!rendered.is_empty());
            assert!(!rendered.contains(hostile));
            assert!(!rendered.contains("evil"));
        }
        assert_eq!(
            format!("{}", SecretError::UnknownProvider),
            "unknown secret provider"
        );
    }
}
