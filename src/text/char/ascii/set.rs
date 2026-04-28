// devela::text::char::ascii::set
//
//! Defines [`AsciiSet`].
//

use crate::{CharAscii, char7, is};

crate::set! {
    #[doc = crate::_tags!(text data_structure)]
    /// A finite set of ASCII characters.
    #[doc = crate::_doc_location!("text/char")]
    ///
    /// Represents membership over the 128-byte ASCII space, one bit per byte.
    #[must_use]
    pub struct AsciiSet(u128) {
        /// All ASCII characters.
        ASCII = 0x00..=0x7F;

        /// Control characters.
        CONTROL = 0x00..=0x1F, 0x7F;

        /// Printable characters, including space.
        PRINTABLE = 0x20..=0x7E;

        /// Graphic printable characters, excluding space.
        GRAPHIC = 0x21..=0x7E;

        /// The ASCII space character.
        SPACE = b' ';

        /// Horizontal whitespace: space and tab.
        HWS = b' ', b'\t';

        /// ASCII whitespace: space, tab, line-feed, carriage-return, form-feed, vertical-tab.
        WS = b' ', b'\t', b'\n', b'\r', 0x0C, 0x0B;

        /// ASCII decimal digits.
        DIGIT = b'0'..=b'9';

        /// ASCII hexadecimal digits.
        HEX_DIGIT = b'0'..=b'9', b'A'..=b'F', b'a'..=b'f';

        /// ASCII alphabetic characters.
        ALPHA = b'A'..=b'Z', b'a'..=b'z';

        /// ASCII alphanumeric characters.
        ALNUM = b'0'..=b'9', b'A'..=b'Z', b'a'..=b'z';

        /// ASCII lowercase alphabetic characters.
        LOWER = b'a'..=b'z';

        /// ASCII uppercase alphabetic characters.
        UPPER = b'A'..=b'Z';

        /// ASCII identifier head: alphabetic or `_`.
        IDENT_HEAD = b'A'..=b'Z', b'a'..=b'z', b'_';

        /// ASCII identifier tail: alphanumeric or `_`.
        IDENT_TAIL = b'0'..=b'9', b'A'..=b'Z', b'a'..=b'z', b'_';

        /// ASCII punctuation.
        PUNCT = 0x21..=0x2F, 0x3A..=0x40, 0x5B..=0x60, 0x7B..=0x7E;
    }

    impl {
        /// Number of characters in the ASCII space.
        pub const LEN: usize = 128;

        /// The minimum ASCII byte.
        pub const MIN_BYTE: u8 = 0x00;

        /// The maximum ASCII byte.
        pub const MAX_BYTE: u8 = 0x7F;

        /// Returns `true` if `byte` is ASCII.
        #[must_use]
        pub const fn is_ascii_byte(byte: u8) -> bool { byte <= Self::MAX_BYTE }

        /// Returns `true` if `ch` is ASCII.
        #[must_use]
        pub const fn is_ascii_char(ch: char) -> bool { (ch as u32) <= Self::MAX_BYTE as u32 }

        /// Returns a singleton set containing `byte`, if it is ASCII.
        #[must_use]
        pub const fn from_byte(byte: u8) -> Option<Self> {
            is![Self::is_ascii_byte(byte), Some(Self::from_ascii_byte_unchecked(byte)), None]
        }

        /// Returns a singleton set containing `ch`, if it is ASCII.
        #[must_use]
        pub const fn from_char(ch: char) -> Option<Self> {
            is![Self::is_ascii_char(ch), Some(Self::from_ascii_byte_unchecked(ch as u8)), None]
        }
        /// Returns a singleton set containing `ch`.
        #[must_use]
        pub const fn from_char7(ch: char7) -> Self {
            Self::from_ascii_byte_unchecked(ch.to_utf8_bytes()[0])
        }
        /// Returns a singleton set containing `ch`.
        #[must_use]
        pub const fn from_ascii(ch: CharAscii) -> Self {
            Self::from_ascii_byte_unchecked(ch as u8)
        }

        /// Returns `true` if this set contains `byte`.
        #[must_use]
        pub const fn contains_byte(self, byte: u8) -> bool {
            Self::is_ascii_byte(byte) && (self.bits & Self::bit(byte)) != 0
        }
        /// Returns `true` if this set contains `ch`.
        #[must_use]
        pub const fn contains_char(self, ch: char) -> bool {
            Self::is_ascii_char(ch) && self.contains_byte(ch as u8)
        }
        /// Returns `true` if this set contains `ch`.
        #[must_use]
        pub const fn contains_char7(self, ch: char7) -> bool {
            self.contains_byte(ch.to_utf8_bytes()[0])
        }
        /// Returns `true` if this set contains `ch`.
        #[must_use]
        pub const fn contains_ascii(self, ch: CharAscii) -> bool {
            self.contains_byte(ch as u8)
        }

        /// Returns this set with `byte` inserted, if it is ASCII.
        #[must_use]
        pub const fn with_byte(self, byte: u8) -> Option<Self> {
            is![Self::is_ascii_byte(byte),
                Some(Self::from_bits(self.bits | Self::bit(byte))), None]
        }
        /// Returns this set without `byte`, if it is ASCII.
        #[must_use]
        pub const fn without_byte(self, byte: u8) -> Option<Self> {
            is![Self::is_ascii_byte(byte),
                Some(Self::from_bits(self.bits & !Self::bit(byte))), None]
        }

        /// Returns this set with `ch` inserted.
        #[must_use]
        pub const fn with_char7(self, ch: char7) -> Self {
            Self::from_bits(self.bits | Self::bit(ch.to_utf8_bytes()[0]))
        }
        /// Returns this set without `ch`.
        #[must_use]
        pub const fn without_char7(self, ch: char7) -> Self {
            Self::from_bits(self.bits & !Self::bit(ch.to_utf8_bytes()[0]))
        }
        /// Returns this set with `ch` inserted.
        #[must_use]
        pub const fn with_ascii(self, ch: CharAscii) -> Self {
            Self::from_bits(self.bits | Self::bit(ch as u8))
        }
        /// Returns this set without `ch`.
        #[must_use]
        pub const fn without_ascii(self, ch: CharAscii) -> Self {
            Self::from_bits(self.bits & !Self::bit(ch as u8))
        }

        #[must_use]
        const fn bit(byte: u8) -> u128 { 1_u128 << byte }
        #[must_use]
        const fn from_ascii_byte_unchecked(byte: u8) -> Self { Self::from_bits(Self::bit(byte)) }
    }
}

#[cfg(test)]
mod tests {
    use crate::AsciiSet as A;

    #[test]
    fn ascii_set_classes() {
        assert!(A::DIGIT.contains_byte(b'7'));
        assert!(!A::DIGIT.contains_byte(b'a'));

        assert!(A::HEX_DIGIT.contains_byte(b'f'));
        assert!(A::HEX_DIGIT.contains_byte(b'F'));
        assert!(!A::HEX_DIGIT.contains_byte(b'g'));

        assert!(A::IDENT_HEAD.contains_byte(b'_'));
        assert!(!A::IDENT_HEAD.contains_byte(b'0'));

        assert!(A::IDENT_TAIL.contains_byte(b'0'));
        assert!(A::IDENT_TAIL.contains_byte(b'_'));

        assert!(A::PUNCT.contains_byte(b'!'));
        assert!(A::PUNCT.contains_byte(b'~'));
        assert!(!A::PUNCT.contains_byte(b'A'));
        assert!(!A::PUNCT.contains_byte(b' '));
    }

    #[test]
    fn ascii_set_non_ascii() {
        assert_eq!(A::from_byte(0x80), None);
        assert_eq!(A::from_char('ñ'), None);
        assert!(!A::ASCII.contains_byte(0x80));
        assert!(!A::ASCII.contains_char('ñ'));
    }
}
