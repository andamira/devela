// devela/src/text/ascii/namespace.rs
//
//! Defines the [`Ascii`] namespace.
//

use crate::is;

#[doc = crate::_tags!(code text namespace)]
/// ASCII code-space constants, digit conversions, and lookup data.
#[doc = crate::_doc_meta!{location("text")}]
#[derive(Debug)]
pub struct Ascii;

impl Ascii {
    /// Number of characters in the ASCII space.
    pub const LEN: usize = 128;

    /// The minimum ASCII byte.
    pub const MIN_BYTE: u8 = 0x00;

    /// The maximum ASCII byte.
    pub const MAX_BYTE: u8 = 0x7F;

    /// Returns whether `byte` is an ASCII byte.
    #[must_use]
    pub const fn is_ascii_byte(byte: u8) -> bool {
        byte <= Ascii::MAX_BYTE
    }
    /// Returns whether `ch` is an ASCII byte.
    #[must_use]
    pub const fn is_ascii_char(ch: char) -> bool {
        (ch as u32) <= Ascii::MAX_BYTE as u32
    }

    /// Returns the digit value represented by an ASCII alphanumeric byte.
    ///
    /// Maps `0`–`9` to `0`–`9`, and `A`–`Z` or `a`–`z` to `10`–`35`.
    /// Returns `None` for any other byte.
    pub const fn digit_value(byte: u8) -> Option<u8> {
        match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'A'..=b'Z' => Some(byte - b'A' + 10),
            b'a'..=b'z' => Some(byte - b'a' + 10),
            _ => None,
        }
    }
    /// Returns the hexadecimal digit value represented by an ASCII byte.
    ///
    /// Accepts `0`–`9`, `A`–`F`, and `a`–`f`, returning values `0`–`15`.
    /// Returns `None` for any other byte.
    pub const fn hex_digit_value(byte: u8) -> Option<u8> {
        match byte {
            b'0'..=b'9' => Some(byte - b'0'),
            b'A'..=b'F' => Some(byte - b'A' + 10),
            b'a'..=b'f' => Some(byte - b'a' + 10),
            _ => None,
        }
    }

    /// Returns the lowercase ASCII symbol for a digit value.
    ///
    /// # Panics
    /// Panics if `value >= 36`.
    pub const fn digit_lower(value: u8) -> u8 {
        Self::DIGITS_BASE36_LOWER[value as usize]
    }
    /// Returns the lowercase ASCII symbol for a digit value from `0` through `35`.
    ///
    /// Returns `None` if `value >= 36`.
    pub const fn digit_lower_checked(value: u8) -> Option<u8> {
        is! { value < 36, Some(Self::digit_lower(value)), None }
    }

    /// Returns the uppercase ASCII symbol for a digit value.
    ///
    /// # Panics
    /// Panics if `value >= 36`.
    #[must_use]
    pub const fn digit_upper(value: u8) -> u8 {
        Self::DIGITS_BASE36_UPPER[value as usize]
    }
    /// Returns the uppercase ASCII symbol for a digit value from `0` through `35`.
    ///
    /// Returns `None` if `value >= 36`.
    pub const fn digit_upper_checked(value: u8) -> Option<u8> {
        is! { value < 36, Some(Self::digit_upper(value)), None }
    }
}

#[doc = concat!["# Unicode scalar related ", crate::_ABBR_LUT!(), "s."]]
impl Ascii {
    #[rustfmt::skip]
    /// Lookup table for fast ASCII code point to UTF-8 encoding.
    ///
    /// Used internally in [`Char`][crate::Char] and [`char7`][crate::char7].
    pub const ASCII_CHARS: [&str; 128] = [
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

    /// Lookup table for ASCII offsets in base36 encoding.
    ///
    /// Maps digits 0-36 to their ASCII offset from '0':
    /// - Digits 0-9: offset 0 (yields '0'-'9')
    /// - Digits 10-36: offset 7 (yields 'A'-'Z')
    ///
    /// Used internally in [`FontArt`].
    #[doc = crate::doclink!(custom devela "[`FontArt`]" "media/font/struct.FontArt.html")]
    pub const ASCII_BASE36_OFFSET: [u8; 37] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, // 0-9: offset 0
        7, 7, 7, 7, 7, 7, 7, 7, 7, 7, // 10-19: offset 7
        7, 7, 7, 7, 7, 7, 7, 7, 7, 7, // 20-29: offset 7
        7, 7, 7, 7, 7, 7, 7, // 30-36: offset 7
    ];

    /// Precomputed two-digit decimal number strings (00-99).
    ///
    /// Used internally in [`Digits`][crate::Digits].
    pub const DECIMAL_PAIRS: &[u8; 200] = b"\
        0001020304050607080910111213141516171819\
        2021222324252627282930313233343536373839\
        4041424344454647484950515253545556575859\
        6061626364656667686970717273747576777879\
        8081828384858687888990919293949596979899";

    /// Lookup table for uppercase digit characters in bases 2 through 36.
    ///
    /// Used internally in [`Digits`][crate::Digits].
    pub const DIGITS_BASE36_UPPER: [u8; 36] = *b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    /// Lookup table for lowercase digit characters in bases 2 through 36.
    pub const DIGITS_BASE36_LOWER: [u8; 36] = *b"0123456789abcdefghijklmnopqrstuvwxyz";
}

#[cfg(test)]
mod _test {
    use super::Ascii;

    #[test]
    fn digit_conversion() {
        assert_eq!(Ascii::digit_value(b'0'), Some(0));
        assert_eq!(Ascii::digit_value(b'9'), Some(9));
        assert_eq!(Ascii::digit_value(b'A'), Some(10));
        assert_eq!(Ascii::digit_value(b'z'), Some(35));
        assert_eq!(Ascii::digit_value(b'@'), None);
        assert_eq!(Ascii::hex_digit_value(b'0'), Some(0));
        assert_eq!(Ascii::hex_digit_value(b'F'), Some(15));
        assert_eq!(Ascii::hex_digit_value(b'f'), Some(15));
        assert_eq!(Ascii::hex_digit_value(b'G'), None);
        assert_eq!(Ascii::digit_upper(10), b'A');
        assert_eq!(Ascii::digit_upper(35), b'Z');
        assert_eq!(Ascii::digit_lower(10), b'a');
        assert_eq!(Ascii::digit_lower(35), b'z');
        assert_eq!(Ascii::digit_upper_checked(36), None);
        assert_eq!(Ascii::digit_lower_checked(36), None);
    }
}
