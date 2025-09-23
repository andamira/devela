// devela_base_core::text::char::namespace
//
//! Defines the [`Char`] namespace.
//
// TOC
// - methods over char
// - methods over u32
// - methods over u16
// - methods over u8
// - methods over &[u8]

use crate::{ASCII_TABLE, unwrap};

#[doc = crate::_TAG_TEXT!()]
#[doc = crate::_TAG_NAMESPACE!()]
/// Unicode scalars-related *const* operations.
///
/// # Methods
/// - [over `char`](#methods-over-char)
/// - [over `u32`](#methods-over-u32)
/// - [over `u16`](#methods-over-u16)
/// - [over `u8`](#methods-over-u8)
/// - [over `&[u8]`](#methods-over-u8-slice)
///
/// See also [`Str`][crate::Str], [`ExtMem`][crate::ExtMem],
#[derive(Clone, Copy, Debug)]
pub struct Char<T>(pub T);

/// # Methods over `char`
#[rustfmt::skip]
impl Char<char> {
    /// Returns the number of bytes needed to encode the given `char` as UTF-8.
    ///
    /// See also [`Char::code_len_utf8`].
    #[must_use] #[rustfmt::skip]
    pub const fn len_utf8(self) -> usize {
        let code = self.0 as u32;
        if code < 0x80 { 1 } else if code < 0x800 { 2 } else if code < 0x10_000 { 3 } else { 4 }
    }

    /// Converts the given `char` to a UTF-8 encoded byte sequence.
    ///
    /// Always returns a `[u8; 4]` array, with unused bytes set to `0`.
    ///
    /// See also [`char::encode_utf8`].
    #[must_use]
    pub const fn to_utf8_bytes(self) -> [u8; 4] {
        Char(self.0 as u32).code_to_utf8_bytes_unchecked()
    }

    /// Returns the ASCII representation as a `&'static str`, or `""` if non-ASCII.
    #[must_use]
    pub const fn to_ascii_str(self) -> &'static str {
        if self.0.is_ascii() { ASCII_TABLE[self.0 as usize] } else { "" }
    }
    /// Returns the ASCII representation as a `&'static str`.
    ///
    /// # Panics
    /// Panics if the character is not ASCII.
    #[must_use]
    pub const fn to_ascii_str_unchecked(self) -> &'static str { ASCII_TABLE[self.0 as usize] }

    /// Converts a character to its closest ASCII equivalent, if possible.
    ///
    /// This function attempts to replace accented or special characters with
    /// their ASCII counterparts. If a mapping exists, it returns `Some(char)`,
    /// otherwise, it returns `None`.
    #[must_use]
    pub const fn to_ascii_fold(self) -> Option<char> {
        match self.0 {
            // ASCII already, return as-is
            _ if self.0.is_ascii() => Some(self.0),
            // Latin-1 Supplement
            'À' | 'Á' | 'Â' | 'Ã' | 'Ä' | 'Å' => Some('A'),
            'à' | 'á' | 'â' | 'ã' | 'ä' | 'å' => Some('a'),
            'È' | 'É' | 'Ê' | 'Ë' => Some('E'),
            'è' | 'é' | 'ê' | 'ë' => Some('e'),
            'Ì' | 'Í' | 'Î' | 'Ï' => Some('I'),
            'ì' | 'í' | 'î' | 'ï' => Some('i'),
            'Ò' | 'Ó' | 'Ô' | 'Õ' | 'Ö' => Some('O'),
            'ò' | 'ó' | 'ô' | 'õ' | 'ö' => Some('o'),
            'Ù' | 'Ú' | 'Û' | 'Ü' => Some('U'),
            'ù' | 'ú' | 'û' | 'ü' => Some('u'),
            'Ý' | 'Ÿ' => Some('Y'),
            'ý' | 'ÿ' => Some('y'),
            'Ç' => Some('C'),
            'ç' => Some('c'),
            'Ñ' => Some('N'),
            'ñ' => Some('n'),
            // Ligatures & Special Cases
            'Æ' => Some('A'), 'æ' => Some('a'),
            'Œ' => Some('O'), 'œ' => Some('o'),
            // Symbols that could be mapped
            'ß' => Some('s'), // German sharp S → s
            'Ð' => Some('D'), 'ð' => Some('d'),
            'Þ' => Some('P'), 'þ' => Some('p'),
            _ => None, // No reasonable ASCII mapping
        }
    }
    /// Converts a character to its closest ASCII equivalent,
    /// or returns the input character if no mapping exists.
    ///
    /// This function is similar to
    /// [`to_ascii_fold`][Self::to_ascii_fold], but **never returns `None`**.
    /// If no ASCII equivalent exists, the input character is returned unchanged.
    #[must_use]
    pub const fn to_ascii_fold_unchecked(self) -> char {
        if let Some(m) = self.to_ascii_fold() { m } else { self.0 }
    }
}
/// # Methods over `u32`.
#[rustfmt::skip]
impl Char<u32> {
    /// Returns the bytes required to store the given Unicode scalar code in a non-UTF encoding.
    ///
    /// This function does **not** determine the UTF-8 byte length.
    /// It assumes a simple encoding where values up to `0xFF` use 1 byte,
    /// `0x100..=0xFFFF` use 2 bytes, and anything larger uses 3 bytes.
    #[must_use]
    pub const fn byte_len(self) -> usize {
        match self.0 {
            0x0000..=0x00FF => 1,
            0x0100..=0xFFFF => 2,
            _ => 3,
        }
    }

    /// Returns the number of bytes required to encode the given code as UTF-8.
    ///
    /// Returns `None` if it's not a valid Unicode scalar.
    #[must_use]
    pub const fn code_len_utf8(self) -> Option<usize> {
        if self.is_valid() { Some(self.code_len_utf8_unchecked()) } else { None }
    }
    /// Returns the UTF-8 byte length of the current code **without validation**.
    ///
    /// Assumes the code is a valid Unicode scalar.
    /// Use [`code_len_utf8`][Self::code_len_utf8] for a checked version.
    #[must_use]
    pub const fn code_len_utf8_unchecked(self) -> usize {
        match self.0 {
            0x00_0000..=0x00_007F => 1,
            0x00_0080..=0x00_07FF => 2,
            0x00_0800..=0x00_FFFF => 3,
            _ => 4,
        }
    }

    /// Checks if the given code is a valid Unicode scalar
    /// (`U+0000..=U+10FFFF`, excluding surrogates).
    #[must_use]
    #[inline(always)]
    pub const fn is_valid(self) -> bool {
        (self.0 <= 0xD7FF) || (self.0 >= 0xE000 && self.0 <= 0x10_FFFF)
    }

    /// Checks if the given code is a 7-bit ASCII character (U+0000..=U+007F).
    #[must_use]
    #[inline(always)]
    pub const fn is_7bit(self) -> bool { self.0 <= 0x7F }

    /// Returns `true` if the given unicode scalar code is a [noncharacter][0].
    ///
    /// Note that this also checks against reserved, potential non-characters.
    ///
    /// [0]: https://www.unicode.org/glossary/#noncharacter
    #[must_use]
    // FIXME: make a version that checks for surrogates
    pub const fn is_noncharacter(self) -> bool {
        // sub-block of 32 non-characters:
        (self.0 >= 0xFDD0 && self.0 <= 0xFDEF)
            // 2× non-characters at the end of each plane:
            || (self.0 >= 0xFFFE && (self.0 & 0xFF) == 0xFE)
            || (self.0 >= 0xFFFE && (self.0 & 0xFF) == 0xFF)
            // unallocated range (16 potential non-characters):
            || (self.0 >= 0x2FE0 && self.0 <= 0x2FEF)
    }

    /// Returns `true` if the given unicode scalar code is a [surrogate code point][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#surrogate_code_point
    #[must_use] #[inline(always)]
    pub const fn is_surrogate(self) -> bool { matches!(self.0, 0xD800..=0xDFFF) }

    /// Returns `true` if the given unicode scalar code is a [leading surrogate][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#high_surrogate_code_point
    #[must_use] #[inline(always)]
    pub const fn is_surrogate_high(self) -> bool { matches!(self.0, 0xD800..=0xDBFF) }

    /// Returns `true` if the given unicode scalar code is a [trailing surrogate][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#low_surrogate_code_point
    #[must_use] #[inline(always)]
    pub const fn is_surrogate_low(self) -> bool { matches!(self.0, 0xDC00..=0xDFFF) }

    /// Returns the ASCII representation as a `&'static str`, or `""` if non-ASCII.
    #[must_use]
    pub const fn code_to_ascii_str(self) -> &'static str {
        if self.is_7bit() { ASCII_TABLE[self.0 as usize] } else { "" }
    }
    /// Returns the ASCII representation as a `&'static str`, or panics if non-ASCII.
    ///
    /// # Panics
    /// Panics if the character is not ASCII.
    #[must_use]
    pub const fn code_to_ascii_str_unchecked(self) -> &'static str { ASCII_TABLE[self.0 as usize] }

    /// Converts the Unicode scalar code to a UTF-8 encoded byte sequence.
    ///
    /// Returns `None` if the code is not a valid Unicode scalar.
    /// The result is always a `[u8; 4]` array, with unused bytes set to `0`.
    ///
    /// See also [`char::encode_utf8`].
    pub const fn code_to_utf8_bytes(self) -> Option<[u8; 4]> {
        if self.is_valid() { Some(self.code_to_utf8_bytes_unchecked()) } else { None }
    }

    /// Converts the Unicode scalar code to a UTF-8 encoded byte sequence **without validation**.
    ///
    /// Assumes the code is a valid Unicode scalar.
    /// Always returns a `[u8; 4]` array, with unused bytes set to `0`.
    ///
    /// See also [`Char::code_to_utf8_bytes`] for a checked version.
    #[must_use]
    #[allow(clippy::unusual_byte_groupings)]
    pub const fn code_to_utf8_bytes_unchecked(self) -> [u8; 4] {
        let code = self.0;
        match code {
            // From 0x0000 to 0x007F:
            // the UTF-8 encoding is the same as the scalar value.
            0x0000..=0x007F => [code as u8, 0, 0, 0],

            // from 0x0080 to 0x07FF:
            // the UTF-8 encoding is 110xxxxx 10xxxxxx,
            // where xxxxx and xxxxxx are the bits of the scalar value.
            0x0080..=0x07FF => {
                let y = 0b10_000000 | (0b0011_1111 & (code as u8));
                let x = 0b110_00000 | ((code >> 6) as u8);
                [x, y, 0, 0]
            }

            // From from 0x0800 to 0xFFFF:
            // the UTF-8 encoding is 1110xxxx 10xxxxxx 10xxxxxx.
            0x0800..=0xFFFF => {
                let z = 0b10_000000 | (0b0011_1111 & (code as u8));
                let y = 0b10_000000 | ((code >> 6) & 0b0011_1111) as u8;
                let x = 0b1110_0000 | ((code >> 12) as u8);
                [x, y, z, 0]
            }

            // From 0x10000 to 0x10FFFF:
            // the UTF-8 encoding is 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx.
            _ => {
                let w = 0b10_000000 | (0b0011_1111 & (code as u8));
                let z = 0b10_000000 | ((code >> 6) & 0b0011_1111) as u8;
                let y = 0b10_000000 | ((code >> 12) & 0b0011_1111) as u8;
                let x = 0b11110_000 | ((code >> 18) as u8);
                [x, y, z, w]
            }
        }
    }
}

/// # Methods over `u16`.
#[rustfmt::skip]
impl Char<u16> {
    #[must_use] #[inline(always)]
    /// Returns `true` if the given unicode scalar code is a [surrogate code point][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#surrogate_code_point
    pub const fn is_surrogate(self) -> bool { matches!(self.0, 0xD800..=0xDFFF) }

    #[must_use] #[inline(always)]
    /// Returns `true` if the given unicode scalar code is a [leading surrogate][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#high_surrogate_code_point
    pub const fn is_surrogate_high(self) -> bool { matches!(self.0, 0xD800..=0xDBFF) }

    #[must_use] #[inline(always)]
    /// Returns `true` if the given unicode scalar code is a [trailing surrogate][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#low_surrogate_code_point
    pub const fn is_surrogate_low(self) -> bool { matches!(self.0, 0xDC00..=0xDFFF) }

    #[must_use]
    /// Decodes the given surrogate pair.
    ///
    /// # Features
    /// Makes use of the `unsafe_str` feature if enabled to avoid double validation.
    pub const fn decode_surrogate_pair(high: u16, low: u16) -> Option<char> {
        if Char(high).is_surrogate_high() && Char(low).is_surrogate_low() {
            let code = 0x10000 + (((high as u32 - 0xD800) << 10) | (low as u32 - 0xDC00));
            #[cfg(any(base_safe_text, not(feature = "unsafe_str")))]
            return char::from_u32(code);
            #[cfg(all(not(base_safe_text), feature = "unsafe_str"))]
            Some(unsafe { char::from_u32_unchecked(code) })
        } else {
            None
        }
    }
}

/// # Methods over `u8`.
#[rustfmt::skip]
impl Char<u8> {
    /// Returns the expected UTF-8 byte length based on the given first byte.
    ///
    /// This function does **not** validate UTF-8 but determines how many bytes
    /// a valid sequence **should** occupy based on the leading byte.
    ///
    /// - ASCII (0xxxxxxx) → 1 byte
    /// - 2-byte (110xxxxx) → 2 bytes
    /// - 3-byte (1110xxxx) → 3 bytes
    /// - 4-byte (11110xxx) → 4 bytes
    ///
    /// ### Caveat
    /// - If used on malformed UTF-8, it may suggest a length longer than the actual valid sequence.
    /// - Always use in conjunction with proper UTF-8 validation if handling untrusted input.
    ///
    /// For a stricter check, see [`utf8_len_checked`][Self::utf8_len_checked].
    pub const fn utf8_len(self) -> usize {
        match self.0 {
            0x00..=0x7F => 1, // ASCII (1 byte)
            0xC0..=0xDF => 2, // 2-byte sequence
            0xE0..=0xEF => 3, // 3-byte sequence
            0xF0..=0xF7 => 4, // 4-byte sequence
            _ => 0,           // Invalid leading byte
        }
    }

    /// Returns the expected UTF-8 byte length based on the given first byte, or `None` if invalid.
    ///
    /// This function detects invalid UTF-8 leading bytes and ensures
    /// they fall within valid Unicode scalar boundaries.
    ///
    /// - Returns `Some(len)` for valid leading bytes.
    /// - Returns `None` for invalid first bytes that cannot start a UTF-8 sequence.
    ///
    /// ### Stricter Handling
    /// - Rejects overlong sequences (C0, C1).
    /// - Enforces the valid UTF-8 upper bound (max `F4`).
    /// - Safer for processing untrusted input where malformed UTF-8 must be detected.
    ///
    /// For a simpler length-only function, see [`utf8_len`][Self::utf8_len].
    #[must_use]
    pub const fn utf8_len_checked(self) -> Option<usize> {
        match self.0 {
            0x00..=0x7F => Some(1),
            0xC2..=0xDF => Some(2),
            0xE0..=0xEF => Some(3),
            0xF0..=0xF4 => Some(4),
            _ => None,
        }
    }
}

/// # Methods over `u8` slice.
#[rustfmt::skip]
impl Char<&[u8]> {
    /// Decodes a UTF-8 code point from the given byte slice, starting at `index`.
    ///
    /// Returns `Some((code, len))` if the input is a valid UTF-8 sequence
    /// and the decoded code point is a valid Unicode scalar.
    ///
    /// Returns `None` if:
    /// - The index is out of bounds.
    /// - The bytes do not form a valid UTF-8 sequence.
    /// - The decoded value is not a valid Unicode scalar.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::Char;
    /// assert_eq!(Char("Ħ".as_bytes()).utf8_bytes_to_code(0), Some((u32::from('Ħ'), 2)));
    ///
    /// let invalid = b"\x80"; // Invalid leading byte
    /// assert_eq!(Char(invalid).utf8_bytes_to_code(0), None);
    /// ```
    #[must_use]
    pub const fn utf8_bytes_to_code(self, index: usize) -> Option<(u32, usize)> {
        let bytes = self.0;
        if index >= bytes.len() { return None; } // out of bounds
        let first = bytes[index];
        let len = unwrap![some? Char(first).utf8_len_checked()]; // invalid leading byte?
        if index + len > bytes.len() { return None; } // not enough bytes
        Some(Char(bytes).utf8_bytes_to_code_unchecked(index))
    }

    /// Decodes a UTF-8 code point from `bytes`, starting at `index`.
    ///
    /// Returns `(code, len)`, where `code` is the decoded Unicode scalar,
    /// and `len` is the number of bytes consumed.
    ///
    /// Assumes `bytes[index..]` contains a valid UTF-8 sequence.
    #[must_use]
    pub const fn utf8_bytes_to_code_unchecked(self, index: usize) -> (u32, usize) {
        let bytes = self.0;
        let first = bytes[index];
        match first {
            0x00..=0x7F => (first as u32, 1), // 1-byte ASCII
            0xC2..=0xDF => ( // 2-byte UTF-8
                (((first as u32 & 0b0001_1111) << 6) |
                 (bytes[index + 1] as u32 & 0b0011_1111)),
                2
            ),
            0xE0..=0xEF => ( // 3-byte UTF-8
                (((first as u32 & 0b0000_1111) << 12) |
                 ((bytes[index + 1] as u32 & 0b0011_1111) << 6) |
                 (bytes[index + 2] as u32 & 0b0011_1111)),
                3
            ),
            0xF0..=0xF4 => ( // 4-byte UTF-8
                (((first as u32 & 0b0000_0111) << 18) |
                 ((bytes[index + 1] as u32 & 0b0011_1111) << 12) |
                 ((bytes[index + 2] as u32 & 0b0011_1111) << 6) |
                 (bytes[index + 3] as u32 & 0b0011_1111)),
                4
            ),
            _ => (0xFFFD, 1), // Invalid sequence → Return Unicode replacement char (U+FFFD)
        }
    }
}
/// Methods over a byte array, referring to a byte slice.
impl<const N: usize> Char<&[u8; N]> {
    /// See [utf8_bytes_to_code()][Char::<&[u8]>::utf8_bytes_to_code].
    #[inline(always)] // eliminate the wrapper entirely
    pub const fn utf8_bytes_to_code(self, index: usize) -> Option<(u32, usize)> {
        let bytes: &[u8] = self.0;
        Char(bytes).utf8_bytes_to_code(index)
    }
}

#[cfg(test)]
mod tests {
    use super::Char;

    #[test]
    fn utf8_bytes_to_code() {
        // Single ASCII character
        assert_eq!(Char(b"a").utf8_bytes_to_code(0), Some((97, 1))); // 'a' -> U+0061

        // Multi-byte UTF-8 character
        let bytes = "Ħ".as_bytes(); // 'Ħ' (U+0126) -> [0xC4, 0xA6]
        assert_eq!(Char(bytes).utf8_bytes_to_code(0), Some((0x0126, 2)));
        assert_eq!(char::from_u32(0x0126), Some('Ħ'));

        // 3-byte UTF-8
        let bytes = "✓".as_bytes(); // '✓' (U+2713) -> [0xE2, 0x9C, 0x93]
        assert_eq!(Char(bytes).utf8_bytes_to_code(0), Some((0x2713, 3)));

        // 4-byte UTF-8
        let bytes = "🚀".as_bytes(); // '🚀' (U+1F680) -> [0xF0, 0x9F, 0x9A, 0x80]
        assert_eq!(Char(bytes).utf8_bytes_to_code(0), Some((0x1F680, 4)));

        // Invalid byte sequence
        let invalid = b"\x80"; // Invalid leading byte
        assert_eq!(Char(invalid).utf8_bytes_to_code(0), None);

        let incomplete = b"\xE2\x9C"; // Incomplete 3-byte sequence
        assert_eq!(Char(incomplete).utf8_bytes_to_code(0), None);
    }
}
