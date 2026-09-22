//! Redaction constants and plaintext-leak test helpers.
//!
//! Secret values render as a fixed constant in every derived representation.
//! The helpers below exist for regression tests that prove no plaintext
//! escapes into logs, errors, events, or exports. They are test utilities,
//! not leak detectors: passing them never proves an arbitrary pipeline safe.

/// Fixed rendering used for every secret value in Debug output.
pub const REDACTED_SECRET_VALUE: &str = "[redacted secret value]";

/// Return true when the haystack visibly contains the plaintext.
///
/// Checks the raw text form when the plaintext is valid UTF-8 and always
/// checks the lowercase hexadecimal form. An empty plaintext never counts as
/// a leak.
pub fn leaks_plaintext(haystack: &str, plaintext: &[u8]) -> bool {
    if plaintext.is_empty() {
        return false;
    }
    if let Ok(text) = core::str::from_utf8(plaintext)
        && haystack.contains(text)
    {
        return true;
    }
    haystack.contains(&hex_lower(plaintext))
}

/// Panic when the haystack visibly contains the plaintext.
///
/// The panic message names only the static caller label, never the haystack
/// and never the plaintext, so failure output cannot become a new leak path.
#[cfg(test)]
pub fn assert_no_plaintext(haystack: &str, plaintext: &[u8], what: &str) {
    assert!(
        !leaks_plaintext(haystack, plaintext),
        "secret plaintext leak detected in {what}"
    );
}

/// Non-test builds expose the same guard for later adversarial suites.
#[cfg(not(test))]
pub fn assert_no_plaintext(haystack: &str, plaintext: &[u8], what: &str) {
    if leaks_plaintext(haystack, plaintext) {
        panic!("secret plaintext leak detected in {what}");
    }
}

fn hex_lower(bytes: &[u8]) -> String {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(DIGITS[(byte >> 4) as usize] as char);
        out.push(DIGITS[(byte & 15) as usize] as char);
    }
    out
}

#[cfg(test)]
mod redaction_tests {
    use super::{assert_no_plaintext, leaks_plaintext};

    #[test]
    fn detects_raw_plaintext() {
        assert!(leaks_plaintext("token=hunter2 here", b"hunter2"));
    }

    #[test]
    fn detects_hex_form_of_binary_plaintext() {
        // 0x00 0xFF is not valid UTF-8, so only the hex form can match.
        assert!(leaks_plaintext("dump 00ff end", &[0x00, 0xFF]));
        assert!(!leaks_plaintext("dump 00FE end", &[0x00, 0xFF]));
    }

    #[test]
    fn empty_plaintext_never_counts() {
        assert!(!leaks_plaintext("anything", &[]));
        assert!(!leaks_plaintext("", &[]));
    }

    #[test]
    fn clean_haystack_passes() {
        assert_no_plaintext("SecretRef(kernux://secret/ref/demo)", b"hunter2", "ref");
    }

    #[test]
    #[should_panic(expected = "secret plaintext leak detected in demo")]
    fn leaking_haystack_panics_without_echoing_material() {
        assert_no_plaintext("token=hunter2 here", b"hunter2", "demo");
    }
}
