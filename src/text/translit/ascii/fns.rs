// devela/src/text/translit/ascii/fns.rs

use crate::{Translit, UnicodeScalar, is, whilst};

impl Translit {
    /// Returns the ASCII transliteration of a Unicode scalar value.
    ///
    /// This is a lossy transliteration, not an encoding.
    /// The result may contain zero, one, or more ASCII bytes.
    ///
    /// Returns an empty string if the scalar is not mapped.
    ///
    /// Converts Unicode to readable ASCII approximations:
    /// - `'°'` → `"deg"`
    /// - `'α'` → `"a"`
    /// - `'©'` → `"(c)"`
    /// - `'中'` → `"Zhong "`
    /// - `'🚀'` → `""`
    /// - ...
    #[must_use]
    pub const fn ascii(char: char) -> &'static str {
        Self::ascii_scalar(char as u32)
    }
    /// Returns the ASCII transliteration of a Unicode scalar code point.
    ///
    /// This accepts a raw `u32` for table-oriented use.
    /// Invalid or unmapped scalar values simply return an empty string.
    #[doc = crate::_doc_vendor!("transliteration")]
    #[must_use]
    pub const fn ascii_scalar(scalar: u32) -> &'static str {
        let block = (scalar >> 8) as usize;
        let offset = (scalar & 0xFF) as usize;
        if block < Translit::ASCII_BLOCKS.len() {
            let block_data = Translit::ASCII_BLOCKS[block];
            if offset < block_data.len() {
                return block_data[offset];
            }
        }
        ""
    }
    /// Returns an ASCII approximation of any Unicode scalar type.
    #[must_use]
    pub fn ascii_of<S: UnicodeScalar>(scalar: S) -> &'static str {
        Self::ascii_scalar(scalar.to_scalar())
    }

    /// Returns whether the scalar has a non-empty ASCII approximation.
    #[must_use]
    pub const fn has_ascii(c: char) -> bool {
        !Self::ascii(c).is_empty()
    }
    /// Returns whether the scalar code point has a non-empty ASCII approximation.
    #[must_use]
    pub const fn has_ascii_scalar(scalar: u32) -> bool {
        !Self::ascii_scalar(scalar).is_empty()
    }

    /// Returns an ASCII approximation, or `fallback` if the scalar is unmapped.
    #[must_use]
    pub const fn ascii_or(c: char, fallback: &'static str) -> &'static str {
        let out = Self::ascii(c);
        if out.is_empty() { fallback } else { out }
    }
    /// Returns an ASCII approximation, or `fallback` if the scalar is unmapped.
    #[must_use]
    pub const fn ascii_scalar_or(scalar: u32, fallback: &'static str) -> &'static str {
        let out = Self::ascii_scalar(scalar);
        if out.is_empty() { fallback } else { out }
    }

    /// Returns the single ASCII byte approximation, if it is exactly one byte.
    #[must_use]
    pub const fn ascii_byte(c: char) -> Option<u8> {
        Self::ascii_scalar_byte(c as u32)
    }
    /// Returns the single ASCII byte approximation, if it is exactly one byte.
    #[must_use]
    pub const fn ascii_scalar_byte(scalar: u32) -> Option<u8> {
        let out = Self::ascii_scalar(scalar);
        let bytes = out.as_bytes();
        if bytes.len() == 1 { Some(bytes[0]) } else { None }
    }

    /// Returns the length of the ASCII approximation.
    #[must_use]
    pub const fn ascii_len(c: char) -> usize {
        Self::ascii(c).len()
    }
    /// Returns the length of the ASCII approximation.
    #[must_use]
    pub const fn ascii_scalar_len(scalar: u32) -> usize {
        Self::ascii_scalar(scalar).len()
    }
}
impl Translit {
    /// Writes the ASCII approximation of `src` into `dst`.
    ///
    /// Unmapped scalars are omitted.
    ///
    /// Returns the number of bytes written, or `None` if `dst` is too small.
    pub fn write_ascii(src: &str, dst: &mut [u8]) -> Option<usize> {
        let mut written = 0;
        for c in src.chars() {
            let out = Self::ascii(c).as_bytes();
            is! { written + out.len() > dst.len(), return None }
            whilst! { i in 0..out.len(); { dst[written + i] = out[i]; }}
            written += out.len();
        }
        Some(written)
    }
    /// Writes the ASCII approximation of `src` into `dst`, using `fallback`
    /// for unmapped scalars.
    ///
    /// Returns the number of bytes written, or `None` if `dst` is too small.
    pub fn write_ascii_or(src: &str, dst: &mut [u8], fallback: &str) -> Option<usize> {
        let mut written = 0;
        for c in src.chars() {
            let out = Self::ascii(c);
            let out = if out.is_empty() { fallback } else { out };
            let bytes = out.as_bytes();
            is! { written + bytes.len() > dst.len(), return None }
            whilst! { i in 0..bytes.len(); { dst[written + i] = bytes[i]; }}
            written += bytes.len();
        }
        Some(written)
    }
}

#[cfg(test)]
mod tests {
    use super::Translit;

    #[test]
    fn ascii() {
        assert_eq![Translit::ascii_scalar('°' as u32), "deg"];
        assert_eq![Translit::ascii_scalar('α' as u32), "a"];
        assert_eq![Translit::ascii_scalar('©' as u32), "(c)"];
        assert_eq![Translit::ascii_scalar('㍱' as u32), "HPA"];
        assert_eq![Translit::ascii_scalar('㎮' as u32), "rad/s"];
        assert_eq![Translit::ascii_scalar('中' as u32), "Zhong "];
        assert_eq![Translit::ascii_scalar('ﬀ' as u32), "ff"];
        assert_eq![Translit::ascii_scalar('ﬤ' as u32), "k"];
        assert_eq![Translit::ascii_scalar('�' as u32), ""];
        assert_eq![Translit::ascii_scalar('🚀' as u32), ""];
    }
}
