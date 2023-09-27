// devela::ascii::char::char16

use super::*;
use crate::ascii::AsciiChar;

impl Char16 {
    /* private helper fns */

    // SAFETY: this is not marked as unsafe because it's only used privately
    // by this module for a few selected operations.
    #[inline]
    const fn from_char_unchecked(c: char) -> Char16 {
        Char16::new_unchecked(c as u32 as u16)
    }

    // useful because Option::<T>::unwrap is not yet stable as const fn
    #[inline]
    const fn new_unchecked(value: u16) -> Char16 {
        #[cfg(not(all(feature = "unsafe_char", feature = "unsafe_num")))]
        if let Some(c) = NonSurrogateU16::new(value) {
            Char16(c)
        } else {
            unreachable![]
        }
        #[cfg(all(feature = "unsafe_char", feature = "unsafe_num"))]
        unsafe {
            Char16(NonSurrogateU16::new_unchecked(value))
        }
    }

    /* constants */

    /// The highest unicode scalar a `Char16` can represent, `'\u{FFFF}'`.
    ///
    /// Note that `'\u{FFFF}'` is a *noncharacter*.
    pub const MAX: Char16 = Char16::new_unchecked(0xFFFF);

    /// `U+FFFD REPLACEMENT CHARACTER (�)` is used in Unicode to represent a decoding error.
    pub const REPLACEMENT_CHARACTER: Char16 =
        Char16::new_unchecked(char::REPLACEMENT_CHARACTER as u32 as u16);

    /* conversions */

    /// Converts an `AsciiChar` to `Char16`.
    #[inline]
    pub const fn from_ascii_char(c: AsciiChar) -> Char16 {
        Char16::new_unchecked(c as u8 as u16)
    }

    /// Converts a `Char7` to `Char16`.
    #[inline]
    pub const fn from_char7(c: Char7) -> Char16 {
        Char16::new_unchecked(c.0.get() as u16)
    }
    /// Converts a `Char8` to `Char16`.
    #[inline]
    pub const fn from_char8(c: Char8) -> Char16 {
        Char16::new_unchecked(c.0 as u16)
    }
    /// Tries to convert a `Char24` to `Char16`.
    #[inline]
    pub const fn try_from_char24(c: Char24) -> Result<Char16> {
        let c = c.to_u32();
        if char_byte_len(c) == 1 {
            Ok(Char16::new_unchecked(c as u16))
        } else {
            Err(CharConversionError(()))
        }
    }
    /// Tries to convert a `Char32` to `Char16`.
    #[inline]
    pub const fn try_from_char32(c: Char32) -> Result<Char16> {
        Self::try_from_char(c.to_char())
    }
    /// Tries to convert a `char` to `Char16`.
    #[inline]
    pub const fn try_from_char(c: char) -> Result<Char16> {
        if char_byte_len(c as u32) <= 2 {
            Ok(Char16::new_unchecked(c as u32 as u16))
        } else {
            Err(CharConversionError(()))
        }
    }

    //

    /// Tries to convert this `Char16` to `AsciiChar`.
    #[inline]
    pub const fn try_to_ascii_char(self) -> Result<AsciiChar> {
        if char_is_7bit(self.to_u32()) {
            #[cfg(not(feature = "unsafe_char"))]
            if let Some(c) = AsciiChar::from_u8(self.0.get() as u8) {
                Ok(c)
            } else {
                unreachable![]
            }

            #[cfg(feature = "unsafe_char")]
            // SAFETY: we've already checked it's in range.
            return Ok(unsafe { AsciiChar::from_u8_unchecked(self.0.get() as u8) });
        } else {
            Err(CharConversionError(()))
        }
    }

    /// Tries to convert this `Char16` to `Char7`.
    #[inline]
    pub const fn try_to_char7(self) -> Result<Char7> {
        Char7::try_from_char16(self)
    }
    /// Tries to convert this `Char16` to `Char8`.
    #[inline]
    pub const fn try_to_char8(self) -> Result<Char8> {
        Char8::try_from_char16(self)
    }
    /// Converts this `Char16` to `Char24`.
    #[inline]
    pub const fn to_char24(self) -> Char24 {
        Char24::from_char16(self)
    }
    /// Converts this `Char16` to `Char32`.
    #[inline]
    pub const fn to_char32(self) -> Char32 {
        Char32::from_char16(self)
    }
    /// Converts this `Char16` to `char`.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_char(self) -> char {
        // #[cfg(not(feature = "unsafe_char"))]
        if let Some(c) = char::from_u32(self.0.get() as u32) { c } else { unreachable![] }

        // WAITING for stable const: https://github.com/rust-lang/rust/issues/89259
        // SAFETY: we've already checked we contain a valid char.
        // #[cfg(feature = "unsafe_char")]
        // return unsafe { char::from_u32_unchecked(self.0 as u32) };
    }
    /// Converts this `Char16` to `u32`.
    #[inline]
    pub const fn to_u32(self) -> u32 {
        self.0.get() as u32
    }

    /// Converts this `Char16` to an UTF-8 encoded sequence of bytes.
    ///
    /// Note that this function always returns a 3-byte array, but the actual
    /// UTF-8 sequence may be shorter. The unused bytes are set to 0.
    //
    // https://en.wikipedia.org/wiki/UTF-8#Encoding
    #[inline]
    #[allow(clippy::unusual_byte_groupings)]
    pub const fn to_utf8_bytes(self) -> [u8; 3] {
        let c = self.0.get();
        match c {
            // From 0x0000 to 0x007F:
            // the UTF-8 encoding is the same as the scalar value.
            0x0000..=0x007F => [c as u8, 0, 0],

            // from 0x0080 to 0x07FF:
            // the UTF-8 encoding is 110xxxxx 10xxxxxx,
            // where xxxxx and xxxxxx are the bits of the scalar value.
            0x0080..=0x07FF => {
                let y = 0b10_000000 | (0b0011_1111 & (c as u8));
                let x = 0b110_00000 | ((c >> 6) as u8);
                [x, y, 0]
            }

            // From from 0x0800 to 0xFFFF:
            // the UTF-8 encoding is 1110xxxx 10xxxxxx 10xxxxxx.
            0x0800..=0xFFFF => {
                let z = 0b10_000000 | (0b0011_1111 & (c as u8));
                let y = 0b10_000000 | ((c >> 6) & 0b0011_1111) as u8;
                let x = 0b1110_0000 | ((c >> 12) as u8);
                [x, y, z]
            }
        }
    }

    //

    /// Makes a copy of the value in its ASCII upper case equivalent.
    ///
    /// ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters
    /// are unchanged.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_ascii_uppercase(self) -> Char16 {
        Self::from_char_unchecked(char::to_ascii_uppercase(&self.to_char()))
    }

    /// Makes a copy of the value in its ASCII lower case equivalent.
    ///
    /// ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters
    /// are unchanged.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_ascii_lowercase(self) -> Char16 {
        Self::from_char_unchecked(char::to_ascii_lowercase(&self.to_char()))
    }

    /* queries */

    /// Returns `true` if this unicode scalar is a [noncharacter][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#noncharacter
    #[inline]
    pub const fn is_noncharacter(self) -> bool {
        char_is_noncharacter(self.0.get() as u32)
    }

    /// Returns `true` if this unicode scalar is an [abstract character][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#abstract_character
    #[inline]
    pub const fn is_character(self) -> bool {
        !self.is_noncharacter()
    }

    /// Checks if the value is within the ASCII range.
    #[inline]
    pub const fn is_ascii(self) -> bool {
        self.0.get() <= 0x7F
    }
}
