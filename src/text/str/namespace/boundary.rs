// devela/src/text/str/namespace/boundary.rs

use crate::{Char, Slice, Str, is, whilst};

/// # Equality and boundary methods
#[rustfmt::skip]
impl Str {
    /// Checks the equality of two string slices in compile-time.
    #[must_use]
    pub const fn eq(a: &str, b: &str) -> bool {
        Slice::<&str>::eq(a, b)
    }

    /// Returns `true` if `string` starts with `prefix`.
    #[must_use]
    pub const fn starts_with(string: &str, prefix: &str) -> bool {
        Self::bytes_start_with(string.as_bytes(), prefix.as_bytes())
    }
    /// Returns `true` if `string` ends with `suffix`.
    #[must_use]
    pub const fn ends_with(string: &str, suffix: &str) -> bool {
        Self::bytes_end_with(string.as_bytes(), suffix.as_bytes())
    }
    /// Returns `string` with `prefix` removed, if present.
    #[must_use]
    pub const fn strip_prefix<'a>(string: &'a str, prefix: &str) -> Option<&'a str> {
        if Self::starts_with(string, prefix) {
            let (_, rest) = string.split_at(prefix.len());
            Some(rest)
        } else {
            None
        }
    }
    /// Returns `string` with `suffix` removed, if present.
    #[must_use]
    pub const fn strip_suffix<'a>(string: &'a str, suffix: &str) -> Option<&'a str> {
        if Self::ends_with(string, suffix) {
            let (rest, _) = string.split_at(string.len() - suffix.len());
            Some(rest)
        } else {
            None
        }
    }
    /// Returns `string` with both `prefix` and `suffix` removed, if both are present.
    ///
    /// The prefix and suffix must not overlap.
    #[must_use]
    pub const fn strip_circumfix<'a>(
        string: &'a str,
        prefix: &str,
        suffix: &str,
    ) -> Option<&'a str> {
        let Some(rest) = Self::strip_prefix(string, prefix) else { return None };
        Self::strip_suffix(rest, suffix)
    }
}

/// Const equivalents for exact `char` prefix/suffix stripping.
impl Str {
    /// Returns `true` if `string` starts with `prefix`.
    #[must_use]
    pub const fn starts_with_char(string: &str, prefix: char) -> bool {
        let bytes = Char(prefix).to_utf8_bytes();
        let len = Char(prefix).len_utf8();
        Self::bytes_start_with_array(string.as_bytes(), bytes, len)
    }
    /// Returns `true` if `string` ends with `suffix`.
    #[must_use]
    pub const fn ends_with_char(string: &str, suffix: char) -> bool {
        let bytes = Char(suffix).to_utf8_bytes();
        let len = Char(suffix).len_utf8();
        Self::bytes_end_with_array(string.as_bytes(), bytes, len)
    }
    /// Returns `string` with `prefix` removed, if present.
    #[must_use]
    pub const fn strip_prefix_char(string: &str, prefix: char) -> Option<&str> {
        if Self::starts_with_char(string, prefix) {
            let (_, rest) = string.split_at(Char(prefix).len_utf8());
            Some(rest)
        } else {
            None
        }
    }
    /// Returns `string` with `suffix` removed, if present.
    #[must_use]
    pub const fn strip_suffix_char(string: &str, suffix: char) -> Option<&str> {
        if Self::ends_with_char(string, suffix) {
            let (rest, _) = string.split_at(string.len() - Char(suffix).len_utf8());
            Some(rest)
        } else {
            None
        }
    }
    /// Returns `string` with both character delimiters removed, if both are present.
    #[must_use]
    pub const fn strip_circumfix_chars(string: &str, prefix: char, suffix: char) -> Option<&str> {
        let Some(rest) = Self::strip_prefix_char(string, prefix) else { return None };
        Self::strip_suffix_char(rest, suffix)
    }
}

// Private byte helpers
impl Str {
    const fn bytes_start_with(bytes: &[u8], prefix: &[u8]) -> bool {
        is! { prefix.len() > bytes.len(), return false }
        whilst! { i in 0..prefix.len(); {
            is! { bytes[i] != prefix[i], return false }
        }}
        true
    }
    const fn bytes_end_with(bytes: &[u8], suffix: &[u8]) -> bool {
        is! { suffix.len() > bytes.len(), return false }
        let start = bytes.len() - suffix.len();
        whilst! { i in 0..suffix.len(); {
            is! { bytes[start + i] != suffix[i], return false }
        }}
        true
    }
    const fn bytes_start_with_array(bytes: &[u8], prefix: [u8; 4], len: usize) -> bool {
        is! { len > prefix.len() || len > bytes.len(), return false }
        whilst! { i in 0..len; {
            is! { bytes[i] != prefix[i], return false }
        }}
        true
    }
    const fn bytes_end_with_array(bytes: &[u8], suffix: [u8; 4], len: usize) -> bool {
        is! { len > suffix.len() || len > bytes.len(), return false }
        let start = bytes.len() - len;
        whilst! { i in 0..len; {
            is! { bytes[start + i] != suffix[i], return false }
        }}
        true
    }
}
