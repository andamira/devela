// devela_base_core::text::char::namespace::u32

use crate::{CHAR_ASCII, Char};

/// # Methods over `u32`.
#[rustfmt::skip]
impl Char<u32> {
    /* private helpers */

    /// Bitmask for extracting the 6-bit payload from a UTF-8 continuation byte (`10xxxxxx`).
    pub(crate) const CONT_MASK: u32 = 0b0011_1111;

    /* constants */

    /// The maximum value of a Unicode code point.
    const MAX_UNICODE: u32 = 0x10_FFFF;

    /// The value of the first Unicode surrogate code point.
    const SURROGATE_START: u32 = 0xD800;

    /// The value of the last Unicode surrogate code point.
    const SURROGATE_END: u32 = 0xDFFF;

    /* public methods */

    /// Returns the bytes required to store the given Unicode code point in a non-UTF encoding.
    ///
    /// This function does **not** determine the UTF-8 byte length.
    /// It assumes a simple encoding where values up to `0xFF` use 1 byte,
    /// `0x100..=0xFFFF` use 2 bytes, and anything larger uses 3 bytes.
    #[must_use]
    pub const fn len_bytes(self) -> usize {
        match self.0 {
            0x0000..=0x00FF => 1,
            0x0100..=0xFFFF => 2,
            _ => 3,
        }
    }

    /// Returns the number of bytes required to encode the given Unicode scalar as UTF-8.
    ///
    /// Returns `None` if it's not a valid Unicode scalar.
    #[must_use]
    pub const fn len_utf8(self) -> Option<usize> {
        if self.is_valid_scalar() { Some(self.len_utf8_unchecked()) } else { None }
    }

    /// Returns the UTF-8 byte length of the current Unicode scalar **without validation**.
    ///
    /// Assumes the code is a valid Unicode scalar.
    /// Use [`len_utf8`][Self::len_utf8] for a checked version.
    #[must_use]
    pub const fn len_utf8_unchecked(self) -> usize {
        match self.0 {
            0x00_0000..=0x00_007F => 1,
            0x00_0080..=0x00_07FF => 2,
            0x00_0800..=0x00_FFFF => 3,
            _ => 4,
        }
    }

    /// Returns the monospace display width.
    ///
    /// - 0: Non-printing characters (controls, combining marks)
    /// - 1: Regular characters (Latin, Greek, Cyrillic, etc.)
    /// - 2: Wide characters (CJK, emoji, fullwidth forms)
    #[must_use]
    pub const fn width(self) -> usize {
        if self.is_control() || self.is_combining() {
            0
        } else if self.is_fullwidth() {
            2
        } else {
            1
        }
    }

    /// Returns the monospace display width using faster calculation.
    ///
    /// Uses optimized checks that cover common cases but may incorrectly
    /// report some obscure Unicode characters as 1 width instead of 2.
    #[must_use]
    pub const fn width_common(self) -> usize {
        if self.is_control_common() || self.is_combining_common() {
            0
        } else if self.is_fullwidth_common() {
            2
        } else {
            1
        }
    }

    /// Checks if the value is a valid Unicode code point.
    ///
    /// A valid Unicode code point is any integer in the range:
    /// - `U+0000` to `U+10FFFF` (inclusive)
    ///
    /// This includes surrogate code points (`U+D800` to `U+DFFF`), which are
    /// valid code points but cannot be represented as Unicode scalars.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::Char;
    /// assert!(Char('A' as u32).is_valid_code()); // regular character
    /// assert!(Char(0x00).is_valid_code());       // NULL is valid
    /// assert!(Char(0x10FFFF).is_valid_code());   // maximum Unicode code point
    /// // surrogates are valid code points:
    /// assert!(Char(0xD800).is_valid_code());     // high surrogate
    /// assert!(Char(0xDFFF).is_valid_code());     // low surrogate
    /// // invalid:
    /// assert!(!Char(0x110000).is_valid_code());  // above max Unicode
    /// ```
    #[must_use] #[inline(always)]
    pub const fn is_valid_code(self) -> bool {
        self.0 <= Self::MAX_UNICODE
    }

    /// Checks if the value is a valid Unicode scalar (a Rust's [`char`]).
    ///
    /// A valid Unicode scalar value is any integer in the ranges:
    /// - `U+0000` to `U+D7FF` (inclusive), or
    /// - `U+E000` to `U+10FFFF` (inclusive)
    ///
    /// This excludes surrogate code points (`U+D800` to `U+DFFF`), which are
    /// invalid in UTF-8 and cannot be represented as Unicode scalars.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::Char;
    /// assert!(Char('A' as u32).is_valid_scalar()); // regular character
    /// assert!(Char(0x00).is_valid_scalar());       // NULL is valid
    /// assert!(Char(0x10FFFF).is_valid_scalar());   // maximum Unicode scalar
    /// // invalid:
    /// assert!(!Char(0xD800).is_valid_scalar());    // high surrogate
    /// assert!(!Char(0xDFFF).is_valid_scalar());    // low surrogate
    /// assert!(!Char(0x110000).is_valid_scalar());  // above max Unicode
    /// ```
    #[must_use]
    #[inline(always)]
    pub const fn is_valid_scalar(self) -> bool {
        (self.0 < Self::SURROGATE_START)
            || (self.0 > Self::SURROGATE_END && self.is_valid_code())
    }

    /// Checks if the given value is a 7-bit ASCII character (U+0000..=U+007F).
    #[must_use] #[inline(always)]
    pub const fn is_ascii(self) -> bool { self.0 <= 0x7F }

    /// Returns `true` if the given Unicode scalar code is a [noncharacter][0].
    ///
    /// Note that this also checks against reserved, potential non-characters.
    ///
    /// [0]: https://www.unicode.org/glossary/#noncharacter
    #[must_use]
    pub const fn is_noncharacter(self) -> bool {
        // sub-block of 32 non-characters:
        (self.0 >= 0xFDD0 && self.0 <= 0xFDEF)
            // 2× non-characters at the end of each plane:
            || (self.0 >= 0xFFFE && (self.0 & 0xFF) == 0xFE)
            || (self.0 >= 0xFFFE && (self.0 & 0xFF) == 0xFF)
            // unallocated range (16 potential non-characters):
            || (self.0 >= 0x2FE0 && self.0 <= 0x2FEF)
    }

    /// Returns `true` for all Unicode combining characters.
    ///
    /// Includes musical notation, historic scripts, and obscure diacritics.
    /// Comprehensive but slightly slower than `is_combining_common`.
    #[must_use]
    pub const fn is_combining(self) -> bool {
        matches![
            self.0,
            0x0300..=0x036F |   // Combining Diacritical Marks
            0x1AB0..=0x1AFF |   // Combining Diacritical Marks Extended
            0x1DC0..=0x1DFF |   // Combining Diacritical Marks Supplement
            0x20D0..=0x20FF |   // Combining Diacritical Marks for Symbols
            0xFE20..=0xFE2F |   // Combining Half Marks
            0xFE00..=0xFE0F |   // Variation Selectors
            0xE0100..=0xE01EF | // "
            0x1D1A0..=0x1D1CD | // Musical Symbols
            0x1D200..=0x1D245 | // Ancient Greek Musical Notation
            0x1E000..=0x1E006 | // Glagolitic Combining Letters
            0x1E130..=0x1E136 | // Nyiakeng Puachue Hmong
            0x1E2AE..=0x1E2BF | // Toto
            0x1E2EC..=0x1E2EF | // Wancho
            0x1EC71..=0x1ECAB | // Kaktovik Numerals
            0x1ED01..=0x1ED3D   // Ottoman Siyaq Numbers
        ]
    }

    /// Returns `true` for common combining marks used in modern text.
    ///
    /// Covers Latin, Greek, and most European language diacritics.
    /// Fast and suitable for 95% of use cases.
    #[must_use]
    pub const fn is_combining_common(self) -> bool {
        matches![
            self.0,
            0x0300..=0x036F | // Combining Diacritical Marks
            0x1DC0..=0x1DFF | // Combining Diacritical Marks Supplement
            0x20D0..=0x20FF | // Combining Diacritical Marks for Symbols
            0xFE20..=0xFE2F | // Combining Half Marks
            0xFE00..=0xFE0F   // Variation Selectors
        ]
    }

    /// Returns `true` for all Unicode control characters.
    pub const fn is_control(self) -> bool {
        matches![self.0,
            // ASCII and C1 controls
            0x00..=0x1F | 0x7F | 0x80..=0x9F |
            // Unicode control blocks
            0x070F |            // Syriac Abbreviation Mark
            0x180B..=0x180E |   // Mongolian controls
            0x200B..=0x200F |   // Zero-width spaces, bidirectional
            0x202A..=0x202E |   // Bidirectional formatting
            0x2060..=0x206F |   // Word joiners, invisible operators
            0xFEFF |            // Zero Width No-Break Space (BOM)
            0xFFF9..=0xFFFB |   // Interlinear annotation controls
            0x110B9 |           // Kaithi punctuation
            0x1D173..=0x1D17A | // Musical symbols controls
            0xE0000..=0xE007F   // Tags and variation selectors
        ]
    }

    /// Returns `true` for common Unicode control characters.
    ///
    /// Just ASCII, zero-width spaces, bidi formatting, word joiners and invisible operators.
    pub const fn is_control_common(self) -> bool {
        matches![self.0,
            // ASCII and C1 controls
            0x00..=0x1F | 0x7F | 0x80..=0x9F |
            // most common Unicode control blocks
            0x200B..=0x200F | // Zero-width spaces, bidirectional
            0x202A..=0x202E | // Bidirectional formatting
            0x2060..=0x206F   // Word joiners, invisible operators
        ]
    }

    /// Returns `true` for all Unicode fullwidth characters.
    #[must_use]
    pub const fn is_fullwidth(self) -> bool {
        matches![self.0,
            // fullwidth Forms block (FF00-FFEF)
            0xFF01..=0xFF5E | // Fullwidth ASCII
            0xFF5F..=0xFF60 | // Fullwidth brackets
            0xFF61..=0xFF9F | // Halfwidth Katakana (considered fullwidth in context)
            0xFFE0..=0xFFE6 | // Fullwidth symbols
            0xFFE8..=0xFFEE | // Fullwidth halfwidth forms

            // CJK Unified Ideographs and extensions
            0x4E00..=0x9FFF |   // CJK Unified Ideographs
            0x3400..=0x4DBF |   // CJK Extension A
            0x20000..=0x2A6DF | // CJK Extension B
            0x2A700..=0x2B73F | // CJK Extension C
            0x2B740..=0x2B81F | // CJK Extension D
            0x2B820..=0x2CEAF | // CJK Extension E
            0x2CEB0..=0x2EBEF | // CJK Extension F

            // Hangul Syllables
            0xAC00..=0xD7AF |

            // Other East Asian wide characters
            0x3000..=0x303F |   // CJK Symbols and Punctuation
            0x3040..=0x309F |   // Hiragana
            0x30A0..=0x30FF |   // Katakana
            0x3100..=0x312F |   // Bopomofo
            0x3130..=0x318F |   // Hangul Compatibility Jamo
            0x3190..=0x319F |   // Kanbun
            0x31A0..=0x31BF |   // Bopomofo Extended
            0x31C0..=0x31EF |   // CJK Strokes
            0x31F0..=0x31FF |   // Katakana Phonetic Extensions
            0x3200..=0x32FF |   // Enclosed CJK Letters and Months
            0x3300..=0x33FF |   // CJK Compatibility
            0xFE10..=0xFE1F |   // Vertical Forms
            0xFE30..=0xFE4F |   // CJK Compatibility Forms
            0xFE50..=0xFE6F |   // Small Form Variants
            0x1F200..=0x1F2FF   // Enclosed Ideographic Supplement
        ]
    }

    /// Returns `true` for common fullwidth characters (ASCII variants, basic CJK)
    pub const fn is_fullwidth_common(self) -> bool {
        matches![
            self.0,
            // Fullwidth ASCII variants and basic East Asian punctuation
            0xFF01..=0xFF5E |
            0xFF5F..=0xFF60 |
            0xFF61..=0xFF9F |
            0xFFE0..=0xFFE6 |
            0xFFE8..=0xFFEE |
            // Basic CJK ranges
            0x4E00..=0x9FFF |
            0x3000..=0x303F
        ]
    }

    /// Returns `true` if the given value is a Unicode [surrogate][0] code point.
    ///
    /// [0]: https://www.unicode.org/glossary/#surrogate_code_point
    #[must_use] #[inline(always)]
    pub const fn is_surrogate(self) -> bool { matches!(self.0, 0xD800..=0xDFFF) }

    /// Returns `true` if the given value is a Unicode [leading surrogate][0] code point.
    ///
    /// [0]: https://www.unicode.org/glossary/#high_surrogate_code_point
    #[must_use] #[inline(always)]
    pub const fn is_surrogate_high(self) -> bool { matches!(self.0, 0xD800..=0xDBFF) }

    /// Returns `true` if the given value is a Unicode [trailing surrogate][0] code point.
    ///
    /// [0]: https://www.unicode.org/glossary/#low_surrogate_code_point
    #[must_use] #[inline(always)]
    pub const fn is_surrogate_low(self) -> bool { matches!(self.0, 0xDC00..=0xDFFF) }

    //

    /// Returns the ASCII `&'static str` representation of the value, or `""` if non-ASCII.
    #[must_use]
    pub const fn as_ascii(self) -> &'static str {
        if self.is_ascii() { CHAR_ASCII[self.0 as usize] } else { "" }
    }

    /// Returns the ASCII `&'static str` representation of the value, or panics if non-ASCII.
    ///
    /// # Panics
    /// Panics if the character is not ASCII.
    #[must_use]
    pub const fn as_ascii_unchecked(self) -> &'static str { CHAR_ASCII[self.0 as usize] }

    /// Converts the Unicode scalar value to a UTF-8 encoded byte sequence array.
    ///
    /// Returns `None` if the value is not a valid Unicode scalar.
    /// The result is always a `[u8; 4]` array, with unused bytes set to `0`.
    ///
    /// See also [`char::encode_utf8`].
    #[must_use] #[inline(always)]
    pub const fn to_utf8_bytes(self) -> Option<[u8; 4]> {
        if self.is_valid_scalar() { Some(self.to_utf8_bytes_unchecked()) } else { None }
    }

    /// Converts the Unicode scalar value to a UTF-8 encoded byte sequence **without validation**.
    ///
    /// Assumes the value is a valid Unicode scalar.
    /// Always returns a `[u8; 4]` array, with unused bytes set to `0`.
    ///
    /// See also [`Char::to_utf8_byte`] for a checked version.
    #[must_use]
    #[allow(clippy::unusual_byte_groupings)]
    pub const fn to_utf8_bytes_unchecked(self) -> [u8; 4] {
        let value = self.0;
        match value {
            // From 0x0000 to 0x007F:
            // the UTF-8 encoding is the same as the scalar value.
            0x0000..=0x007F => [value as u8, 0, 0, 0],

            // from 0x0080 to 0x07FF:
            // the UTF-8 encoding is 110xxxxx 10xxxxxx,
            // where xxxxx and xxxxxx are the bits of the scalar value.
            0x0080..=0x07FF => {
                let y = 0b10_000000 | (Char::<u8>::CONT_MASK & (value as u8));
                let x = 0b110_00000 | ((value >> 6) as u8);
                [x, y, 0, 0]
            }

            // From from 0x0800 to 0xFFFF:
            // the UTF-8 encoding is 1110xxxx 10xxxxxx 10xxxxxx.
            0x0800..=0xFFFF => {
                let z = 0b10_000000 | (Char::<u8>::CONT_MASK & (value as u8));
                let y = 0b10_000000 | ((value >> 6) & Char::<u32>::CONT_MASK) as u8;
                let x = 0b1110_0000 | ((value >> 12) as u8);
                [x, y, z, 0]
            }

            // From 0x10000 to 0x10FFFF:
            // the UTF-8 encoding is 11110xxx 10xxxxxx 10xxxxxx 10xxxxxx.
            _ => {
                let w = 0b10_000000 | (Char::<u8>::CONT_MASK & (value as u8));
                let z = 0b10_000000 | ((value >> 6) & Char::<u32>::CONT_MASK) as u8;
                let y = 0b10_000000 | ((value >> 12) & Char::<u32>::CONT_MASK) as u8;
                let x = 0b11110_000 | ((value >> 18) as u8);
                [x, y, z, w]
            }
        }
    }
}
