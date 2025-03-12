// devela::text::char::namespace
//
//! Defines the [`Char`] namespace.
//
// MAYBE: Defines the [`Char`] and [`Utf8`] namespaces.

#[doc = crate::TAG_NAMESPACE!()]
/// Unicode scalars-related operations.
///
/// See also [`Str`][crate::Str], [`ExtMem`][crate::ExtMem],
pub struct Char;

/// Global ASCII lookup table for fast UTF-8 encoding.
#[rustfmt::skip]
pub(crate) static ASCII_TABLE: [&str; 128] = [
    "\0", "\x01", "\x02", "\x03", "\x04", "\x05", "\x06", "\x07", "\x08", "\t", "\n",
    "\x0B", "\x0C", "\r", "\x0E", "\x0F", "\x10", "\x11", "\x12", "\x13", "\x14", "\x15",
    "\x16", "\x17", "\x18", "\x19", "\x1A", "\x1B", "\x1C", "\x1D", "\x1E", "\x1F",
    " ", "!", "\"", "#", "$", "%", "&", "'", "(", ")", "*", "+", ",", "-", ".", "/",
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", ":", ";", "<", "=", ">", "?",
    "@", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L", "M", "N", "O",
    "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z", "[", "\\", "]", "^", "_",
    "`", "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o",
    "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "{", "|", "}", "~", "\x7F",
];

/// # Methods over `u32`.
#[rustfmt::skip]
impl Char {
    /// Returns the number of bytes necessary to store the given unicode scalar `code`.
    #[must_use]
    pub const fn byte_len(code: u32) -> usize {
        match code {
            0..=0xFF => 1,
            0x100..=0xFFFF => 2,
            _ => 3,
        }
    }

    /// Returns `true` if the given unicode scalar `code` is a 7bit ASCII code.
    #[must_use]
    pub const fn is_7bit(code: u32) -> bool {
        code <= 0x7F
    }

    /// Returns `true` if the given unicode scalar `code` is a [noncharacter][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#noncharacter
    #[must_use]
    // FIXME: make a version that checks for surrogates
    pub const fn is_noncharacter(code: u32) -> bool {
        // sub-block of 32 non-characters:
        (code >= 0xFDD0 && code <= 0xFDEF)
            // 2× non-characters at the end of each plane:
            || (code >= 0xFFFE && (code & 0xFF) == 0xFE)
            || (code >= 0xFFFE && (code & 0xFF) == 0xFF)
            // unallocated range (16 potential non-characters):
            || (code >= 0x2FE0 && code <= 0x2FEF)
    }

    /// Returns the ASCII representation as a `&'static str`, or `""` if non-ASCII.
    #[must_use]
    pub const fn code_to_ascii_str(c: u32) -> &'static str {
        if Char::is_7bit(c) { ASCII_TABLE[c as usize] } else { "" }
    }
    /// Returns the ASCII representation as a `&'static str`, or panics if non-ASCII.
    ///
    /// # Panics
    /// Panics if the character is not ASCII.
    #[must_use]
    pub const fn code_to_ascii_str_unchecked(c: u32) -> &'static str { ASCII_TABLE[c as usize] }
}

/// # Methods over bytes.
impl Char {
    /// Returns the expected UTF-8 byte length based on the first byte.
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
    pub const fn utf8_len(first_byte: u8) -> u8 {
        match first_byte {
            0x00..=0x7F => 1, // ASCII (1 byte)
            0xC0..=0xDF => 2, // 2-byte sequence
            0xE0..=0xEF => 3, // 3-byte sequence
            0xF0..=0xF7 => 4, // 4-byte sequence
            _ => 0,           // Invalid leading byte
        }
    }

    /// Returns the UTF-8 byte length or `None` if the first byte is invalid.
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
    pub const fn utf8_len_checked(first_byte: u8) -> Option<u8> {
        match first_byte {
            0x00..=0x7F => Some(1),
            0xC2..=0xDF => Some(2),
            0xE0..=0xEF => Some(3),
            0xF0..=0xF4 => Some(4),
            _ => None,
        }
    }

    /// Returns the number of bytes needed to store the given unicode scalar `code`,
    /// already UTF-8 encoded in 2 bytes.
    #[must_use]
    #[deprecated(since = "0.23.0", note = "Use `utf8_len` instead")]
    pub const fn utf8_2bytes_len(code: [u8; 2]) -> u8 {
        1 + ((code[1] > 0) & (code[1] & 0b1100_0000 != 0b1000_0000)) as u8
    }

    /// Returns the number of bytes needed to store the given unicode scalar `code`,
    /// already UTF-8 encoded in 3 bytes.
    #[must_use]
    #[deprecated(since = "0.23.0", note = "Use `utf8_len` instead")]
    pub const fn utf8_3bytes_len(code: [u8; 3]) -> u8 {
        1 + ((code[1] > 0) & (code[1] & 0b1100_0000 != 0b1000_0000)) as u8
            + ((code[2] > 0) & (code[2] & 0b1100_0000 != 0b1000_0000)) as u8
    }

    /// Returns the number of bytes needed to store the given unicode scalar `code`,
    /// already UTF-8 encoded in 4 bytes.
    #[must_use]
    #[deprecated(since = "0.23.0", note = "Use `utf8_len` instead")]
    pub const fn utf8_4bytes_len(code: [u8; 4]) -> u8 {
        1 + ((code[1] > 0) & (code[1] & 0b1100_0000 != 0b1000_0000)) as u8
            + ((code[2] > 0) & (code[2] & 0b1100_0000 != 0b1000_0000)) as u8
            + ((code[3] > 0) & (code[3] & 0b1100_0000 != 0b1000_0000)) as u8
    }
}

/// # Methods over `char`
#[rustfmt::skip]
impl Char {
    /// Returns the number of bytes needed to encode the given unicode scalar `code` as UTF-8.
    #[must_use] #[rustfmt::skip]
    pub const fn len_utf8(code: char) -> usize {
        let code = code as u32;
        if code < 0x80 { 1 } else if code < 0x800 { 2 } else if code < 0x10_000 { 3 } else { 4 }
    }
    /// Returns the number of bytes needed to encode the given unicode scalar `code` as UTF-8.
    #[must_use]
    #[deprecated(since = "0.23.0", note = "Use `len_utf8` instead")]
    pub const fn len_to_utf8(code: char) -> usize { Self::len_utf8(code) }

    /// Converts this `char` to an UTF-8 encoded sequence of bytes.
    ///
    /// Note that this function always returns a 4-byte array, but the actual
    /// UTF-8 sequence may be shorter. The unused bytes are set to 0.
    ///
    /// See also [`char::encode_utf8`].
    #[must_use]
    #[allow(clippy::unusual_byte_groupings)]
    pub const fn to_utf8_bytes(c: char) -> [u8; 4] {
        let c = c as u32;
        match c {
            // From 0x0000 to 0x007F:
            // the UTF-8 encoding is the same as the scalar value.
            0x0000..=0x007F => [c as u8, 0, 0, 0],

            // from 0x0080 to 0x07FF:
            // the UTF-8 encoding is 110xxxxx 10xxxxxx,
            // where xxxxx and xxxxxx are the bits of the scalar value.
            0x0080..=0x07FF => {
                let y = 0b10_000000 | (0b0011_1111 & (c as u8));
                let x = 0b110_00000 | ((c >> 6) as u8);
                [x, y, 0, 0]
            }

            // From from 0x0800 to 0xFFFF:
            // the UTF-8 encoding is 1110xxxx 10xxxxxx 10xxxxxx.
            0x0800..=0xFFFF => {
                let z = 0b10_000000 | (0b0011_1111 & (c as u8));
                let y = 0b10_000000 | ((c >> 6) & 0b0011_1111) as u8;
                let x = 0b1110_0000 | ((c >> 12) as u8);
                [x, y, z, 0]
            }

            // From 0x10000 to 0x10FFFF:
            // the UTF-8 encoding is 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx.
            _ => {
                let w = 0b10_000000 | (0b0011_1111 & (c as u8));
                let z = 0b10_000000 | ((c >> 6) & 0b0011_1111) as u8;
                let y = 0b10_000000 | ((c >> 12) & 0b0011_1111) as u8;
                let x = 0b11110_000 | ((c >> 18) as u8);
                [x, y, z, w]
            }
        }
    }

    /// Returns the ASCII representation as a `&'static str`, or `""` if non-ASCII.
    #[must_use]
    pub const fn to_ascii_str(c: char) -> &'static str {
        if c.is_ascii() { ASCII_TABLE[c as usize] } else { "" }
    }
    /// Returns the ASCII representation as a `&'static str`, or panics if non-ASCII.
    ///
    /// # Panics
    /// Panics if the character is not ASCII.
    #[must_use]
    pub const fn to_ascii_str_unchecked(c: char) -> &'static str { ASCII_TABLE[c as usize] }

    /// Converts a character to its closest ASCII equivalent, if possible.
    ///
    /// This function attempts to replace accented or special characters with
    /// their ASCII counterparts. If a mapping exists, it returns `Some(char)`,
    /// otherwise, it returns `None`.
    #[must_use]
    pub const fn to_ascii_fold(c: char) -> Option<char> {
        match c {
            // ASCII already, return as-is
            _ if c.is_ascii() => Some(c),
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
    /// This function is similar to [`to_ascii_fold`], but **never returns `None`**.
    /// If no ASCII equivalent exists, the input character is returned unchanged.
    #[must_use]
    pub const fn to_ascii_fold_unchecked(c: char) -> char {
        if let Some(m) = Self::to_ascii_fold(c) { m } else { c }
    }
}
