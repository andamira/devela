// devela::char::impls
//
//!
//
// TOC
// - common implementations
//   - traits
//   - const fns
// - separate implementations
//   - Char7
//   - Char8
//   - Char16
//   - Char24
//   - Char32
//   - traits for char
// - helper fns

use super::{
    Char16, Char24, Char32, Char7, Char8, CharConversionError, NonMaxU8, NonSurrogateU16, Result,
    UnicodeScalar,
};
use crate::codegen::paste;

/* common implementations */

macro_rules! impls {
    ($name:ident: $( $bits:literal ),+ ) => {
        $( impls![@$name: $bits]; )+
    };
    (@$name:ident: $bits:literal) => { paste! {

        /* impl traits */

        impl UnicodeScalar for [<$name $bits>] {
            const MAX: Self = Self::MAX;

            /* encode */

            #[inline]
            fn byte_len(self) -> usize { self.byte_len() }
            #[inline]
            fn len_utf8(self) -> usize { self.len_utf8() }
            #[inline]
            fn len_utf16(self) -> usize { self.len_utf16() }
            #[inline]
            fn encode_utf8(self, dst: &mut [u8]) -> &mut str {
                self.to_char().encode_utf8(dst)
            }
            #[inline]
            fn to_utf8_bytes(self) -> [u8; 4] {
                let dyn_array = self.to_utf8_bytes();
                let mut array = [0u8; 4];
                for i in 0..dyn_array.len() {
                    array[i] = dyn_array[i];
                }
                array
            }
            #[inline]
            fn encode_utf16(self, dst: &mut [u16]) -> &mut [u16] {
                self.to_char().encode_utf16(dst)
            }
            #[inline]
            fn to_digit(self, radix: u32) -> Option<u32> { self.to_digit(radix) }
            #[inline]
            fn to_ascii_uppercase(self) -> Self { self.to_ascii_uppercase() }
            #[inline]
            fn to_ascii_lowercase(self) -> Self { self.to_ascii_lowercase() }

            /* queries */

            #[inline]
            fn is_noncharacter(self) -> bool { self.is_noncharacter() }
            #[inline]
            fn is_digit(self, radix: u32) -> bool { self.is_digit(radix) }
            //
            #[inline]
            fn is_control(self) -> bool { self.to_char().is_control() }
            #[inline]
            fn is_nul(self) -> bool { self.is_nul() }
            #[inline]
            fn is_alphabetic(self) -> bool { self.to_char().is_alphabetic() }
            #[inline]
            fn is_numeric(self) -> bool { self.to_char().is_numeric() }
            #[inline]
            fn is_alphanumeric(self) -> bool { self.to_char().is_alphanumeric() }
            #[inline]
            fn is_lowercase(self) -> bool { self.to_char().is_lowercase() }
            #[inline]
            fn is_uppercase(self) -> bool { self.to_char().is_uppercase() }
            #[inline]
            fn is_whitespace(self) -> bool { self.to_char().is_whitespace() }
            //
            #[inline]
            fn is_ascii(self) -> bool { self.is_ascii() }
        }

        /* impl const fns */

        impl [<$name $bits>] {

            /* encode */

            /// Returns the number of bytes needed to represent the scalar value.
            pub const fn byte_len(self) -> usize { byte_len(self.to_u32()) }

            /// Returns the number of bytes needed to encode in UTF-8.
            #[inline]
            pub const fn len_utf8(self) -> usize { self.to_char().len_utf8() }

            /// Returns the number of bytes needed to encode in UTF-16.
            #[inline]
            pub const fn len_utf16(self) -> usize { self.to_char().len_utf16() }

            /// Converts the scalar to a digit in the given radix.
            ///
            /// ‘Digit’ is defined to be only the following characters:
            /// `0-9`, `a-z`, `A-Z`.
            ///
            /// # Errors
            /// Returns None if the char does not refer to a digit in the given radix.
            ///
            /// # Panics
            /// Panics if given a radix larger than 36.
            #[inline]
            pub const fn to_digit(self, radix: u32) -> Option<u32> {
                self.to_char().to_digit(radix)
            }

            /* queries */

            /// Returns `true` if this is the nul character (`0x00`).
            #[inline]
            pub const fn is_nul(self) -> bool { self.to_u32() == 0 }

            /// Checks if the unicode scalar is a digit in the given radix.
            ///
            /// See also [`to_digit`][Self#method.to_digit].
            #[inline]
            pub const fn is_digit(self, radix: u32) -> bool {
                if let Some(_) = self.to_digit(radix) { true } else { false }
            }
        }
    }};
}
impls![Char: 7, 8, 16, 24, 32];

/* separate implementations */

impl Char7 {
    /* constants */

    /// The highest unicode scalar a `Char7` can represent, `'\u{7F}'`.
    pub const MAX: Char7 = Char7::new_unchecked(0x7F);

    /* conversions */

    /// Tries to convert a `Char8` to `Char7`.
    #[inline]
    pub const fn try_from_char8(c: Char8) -> Result<Char7> {
        if is_7bit(c.to_u32()) {
            Ok(Char7::new_unchecked(c.to_u32() as u8))
        } else {
            Err(CharConversionError(()))
        }
    }
    /// Tries to convert a `Char16` to `Char7`.
    #[inline]
    pub const fn try_from_char16(c: Char16) -> Result<Char7> {
        if is_7bit(c.to_u32()) {
            Ok(Char7::new_unchecked(c.to_u32() as u8))
        } else {
            Err(CharConversionError(()))
        }
    }
    /// Tries to convert a `Char24` to `Char7`.
    #[inline]
    pub const fn try_from_char24(c: Char24) -> Result<Char7> {
        let c = c.to_u32();
        if is_7bit(c) {
            Ok(Char7::new_unchecked(c as u8))
        } else {
            Err(CharConversionError(()))
        }
    }
    /// Tries to convert a `Char32` to `Char8`.
    #[inline]
    pub const fn try_from_char32(c: Char32) -> Result<Char7> {
        if is_7bit(c.to_u32()) {
            Ok(Char7::new_unchecked(c.to_u32() as u8))
        } else {
            Err(CharConversionError(()))
        }
    }
    /// Tries to convert a `char` to `Char8`.
    #[inline]
    pub const fn try_from_char(c: char) -> Result<Char7> {
        if is_7bit(c as u32) {
            Ok(Char7::new_unchecked(c as u32 as u8))
        } else {
            Err(CharConversionError(()))
        }
    }
    const fn from_char_unchecked(c: char) -> Char7 {
        Char7::new_unchecked(c as u32 as u8)
    }
    // useful because Option::<T>::unwrap is not yet stable as const fn
    const fn new_unchecked(value: u8) -> Char7 {
        #[cfg(not(feature = "unsafe_char"))]
        if let Some(c) = NonMaxU8::new(value) {
            Char7(c)
        } else {
            unreachable![]
        }
        #[cfg(feature = "unsafe_char")]
        unsafe {
            Char7(NonMaxU8::new_unchecked(value))
        }
    }

    //

    /// Converts this `Char7` to `Char8`.
    #[inline]
    pub const fn to_char8(self) -> Char8 {
        Char8::from_char7(self)
    }
    /// Converts this `Char7` to `Char16`.
    #[inline]
    pub const fn to_char16(self) -> Char16 {
        Char16::from_char7(self)
    }
    /// Converts this `Char7` to `Char24`.
    #[inline]
    pub const fn to_char24(self) -> Char24 {
        Char24::from_char7(self)
    }
    /// Converts this `Char7` to `Char32`.
    #[inline]
    pub const fn to_char32(self) -> Char32 {
        Char32::from_char7(self)
    }
    /// Converts this `Char7` to `char`.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_char(self) -> char {
        self.0.get() as char
    }
    /// Converts this `Char7` to `u32`.
    #[inline]
    pub const fn to_u32(self) -> u32 {
        self.0.get() as u32
    }

    /// Converts this `Char7` to an UTF-8 encoded sequence of bytes.
    //
    // https://en.wikipedia.org/wiki/UTF-8#Encoding
    #[inline]
    #[allow(clippy::unusual_byte_groupings)]
    pub const fn to_utf8_bytes(self) -> [u8; 1] {
        // From 0x0000 to 0x007F:
        // the UTF-8 encoding is the same as the scalar value.
        [self.0.get()]
    }

    //

    /* queries */

    /// Returns `true` if this unicode scalar is a [noncharacter][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#noncharacter
    #[inline]
    pub const fn is_noncharacter(self) -> bool {
        false
    }

    /// Returns `true` if this unicode scalar is an [abstract character][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#abstract_character
    #[inline]
    pub const fn is_character(self) -> bool {
        true
    }

    /// Checks if the value is within the ASCII range.
    #[inline]
    pub const fn is_ascii(self) -> bool {
        true
    }

    /// Makes a copy of the value in its ASCII upper case equivalent.
    ///
    /// ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters
    /// are unchanged.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_ascii_uppercase(self) -> Char7 {
        Self::from_char_unchecked(char::to_ascii_uppercase(&self.to_char()))
    }

    /// Makes a copy of the value in its ASCII lower case equivalent.
    ///
    /// ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters
    /// are unchanged.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_ascii_lowercase(self) -> Char7 {
        Self::from_char_unchecked(char::to_ascii_lowercase(&self.to_char()))
    }
}
impl Char8 {
    /* constants */

    /// The highest unicode scalar a `Char8` can represent, `'\u{FF}'`.
    pub const MAX: Char8 = Char8(0xFF);

    /* conversions */

    /// Converts a `Char7` to `Char8`.
    #[inline]
    pub const fn from_char7(c: Char7) -> Char8 {
        Self(c.0.get())
    }
    /// Tries to convert a `Char16` to `Char8`.
    #[inline]
    pub const fn try_from_char16(c: Char16) -> Result<Char8> {
        if byte_len(c.to_u32()) == 1 {
            Ok(Char8(c.to_u32() as u8))
        } else {
            Err(CharConversionError(()))
        }
    }
    /// Tries to convert a `Char24` to `Char8`.
    #[inline]
    pub const fn try_from_char24(c: Char24) -> Result<Char8> {
        let c = c.to_u32();
        if byte_len(c) == 1 {
            Ok(Char8(c as u8))
        } else {
            Err(CharConversionError(()))
        }
    }
    /// Tries to convert a `Char32` to `Char8`.
    #[inline]
    pub const fn try_from_char32(c: Char32) -> Result<Char8> {
        if byte_len(c.to_u32()) == 1 {
            Ok(Char8(c.to_u32() as u8))
        } else {
            Err(CharConversionError(()))
        }
    }
    /// Tries to convert a `char` to `Char8`.
    #[inline]
    pub const fn try_from_char(c: char) -> Result<Char8> {
        if byte_len(c as u32) == 1 {
            Ok(Char8(c as u32 as u8))
        } else {
            Err(CharConversionError(()))
        }
    }
    const fn from_char_unchecked(c: char) -> Char8 {
        Char8(c as u32 as u8)
    }

    //

    /// Tries to convert this `Char8` to `Char7`.
    #[inline]
    pub const fn try_to_char7(self) -> Result<Char7> {
        Char7::try_from_char8(self)
    }
    /// Converts this `Char8` to `Char16`.
    #[inline]
    pub const fn to_char16(self) -> Char16 {
        Char16::from_char8(self)
    }
    /// Converts this `Char8` to `Char24`.
    #[inline]
    pub const fn to_char24(self) -> Char24 {
        Char24::from_char8(self)
    }
    /// Converts this `Char8` to `Char32`.
    #[inline]
    pub const fn to_char32(self) -> Char32 {
        Char32::from_char8(self)
    }
    /// Converts this `Char8` to `char`.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_char(self) -> char {
        self.0 as char
    }
    /// Converts this `Char8` to `u32`.
    #[inline]
    pub const fn to_u32(self) -> u32 {
        self.0 as u32
    }

    /// Converts this `Char8` to an UTF-8 encoded sequence of bytes.
    ///
    /// Note that this function always returns a 2-byte array, but the actual
    /// UTF-8 sequence may be shorter. The unused bytes are set to 0.
    //
    // https://en.wikipedia.org/wiki/UTF-8#Encoding
    #[inline]
    #[allow(clippy::unusual_byte_groupings)]
    pub const fn to_utf8_bytes(self) -> [u8; 2] {
        let c = self.0;
        match c {
            // From 0x0000 to 0x007F:
            // the UTF-8 encoding is the same as the scalar value.
            0x0000..=0x007F => [c, 0],

            // from 0x0080 to 0x00FF:
            // the UTF-8 encoding is 110xxxxx 10xxxxxx,
            // where xxxxx and xxxxxx are the bits of the scalar value.
            0x0080.. => {
                let y = 0b10_000000 | (0b0011_1111 & c);
                let x = 0b110_00000 | (c >> 6);
                [x, y]
            }
        }
    }

    //

    /* queries */

    /// Returns `true` if this unicode scalar is a [noncharacter][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#noncharacter
    #[inline]
    pub const fn is_noncharacter(self) -> bool {
        is_noncharacter(self.0 as u32)
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
        self.0 <= 0x7F
    }

    /// Makes a copy of the value in its ASCII upper case equivalent.
    ///
    /// ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters
    /// are unchanged.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_ascii_uppercase(self) -> Char8 {
        Self::from_char_unchecked(char::to_ascii_uppercase(&self.to_char()))
    }

    /// Makes a copy of the value in its ASCII lower case equivalent.
    ///
    /// ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters
    /// are unchanged.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_ascii_lowercase(self) -> Char8 {
        Self::from_char_unchecked(char::to_ascii_lowercase(&self.to_char()))
    }
}

impl Char16 {
    /* constants */

    /// The highest unicode scalar a `Char16` can represent, `'\u{FFFF}'`.
    ///
    /// Note that `'\u{FFFF}'` is a *noncharacter*.
    pub const MAX: Char16 = Char16::new_unchecked(0xFFFF);

    /// `U+FFFD REPLACEMENT CHARACTER (�)` is used in Unicode to represent a decoding error.
    pub const REPLACEMENT_CHARACTER: Char16 =
        Char16::new_unchecked(char::REPLACEMENT_CHARACTER as u32 as u16);

    /* conversions */

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
        if byte_len(c) == 1 {
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
        if byte_len(c as u32) <= 2 {
            Ok(Char16::new_unchecked(c as u32 as u16))
        } else {
            Err(CharConversionError(()))
        }
    }
    const fn from_char_unchecked(c: char) -> Char16 {
        Char16::new_unchecked(c as u32 as u16)
    }
    // useful because Option::<T>::unwrap is not yet stable as const fn
    const fn new_unchecked(value: u16) -> Char16 {
        #[cfg(not(feature = "unsafe_char"))]
        if let Some(c) = NonSurrogateU16::new(value) {
            Char16(c)
        } else {
            unreachable![]
        }
        #[cfg(feature = "unsafe_char")]
        unsafe {
            Char16(NonSurrogateU16::new_unchecked(value))
        }
    }
    //

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
        is_noncharacter(self.0.get() as u32)
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

impl Char24 {
    /* constants */

    /// The highest unicode scalar a `Char24` can represent, `'\u{10FFFF}'`.
    pub const MAX: Char24 = Char24::from_char('\u{10ffff}');

    /// `U+FFFD REPLACEMENT CHARACTER (�)` is used in Unicode to represent a decoding error.
    pub const REPLACEMENT_CHARACTER: Char24 = Char24::from_char(char::REPLACEMENT_CHARACTER);

    /* conversions */

    /// Converts a `Char7` to `Char24`.
    #[inline]
    pub const fn from_char7(c: Char7) -> Char24 {
        Char24 {
            hi: Self::new_unchecked_hi(0),
            mi: 0,
            lo: c.0.get(),
        }
    }
    /// Converts a `Char8` to `Char24`.
    #[inline]
    pub const fn from_char8(c: Char8) -> Char24 {
        Char24 {
            hi: Self::new_unchecked_hi(0),
            mi: 0,
            lo: c.0,
        }
    }
    /// Converts a `Char16` to `Char24`.
    #[inline]
    pub const fn from_char16(c: Char16) -> Char24 {
        let mi = ((c.0.get() & 0xFF00) >> 8) as u8;
        let lo = (c.0.get() & 0x00FF) as u8;
        Char24 {
            hi: Self::new_unchecked_hi(0),
            mi,
            lo,
        }
    }
    /// Converts a `Char32` to `Char24`.
    #[inline]
    pub const fn from_char32(c: Char32) -> Char24 {
        Char24::from_char(c.0)
    }
    /// Converts a `char` to `Char24`.
    #[inline]
    pub const fn from_char(c: char) -> Char24 {
        let hi = ((c as u32 & 0x001F0000) >> 16) as u8;
        let mi = ((c as u32 & 0x0000FF00) >> 8) as u8;
        let lo = (c as u32 & 0x000000FF) as u8;
        Char24 {
            hi: Self::new_unchecked_hi(hi),
            mi,
            lo,
        }
    }
    // useful because Option::<T>::unwrap is not yet stable as const fn
    const fn new_unchecked_hi(value: u8) -> NonMaxU8 {
        #[cfg(not(feature = "unsafe_char"))]
        if let Some(c) = NonMaxU8::new(value) {
            c
        } else {
            unreachable![]
        }
        #[cfg(feature = "unsafe_char")]
        unsafe {
            NonMaxU8::new_unchecked(value)
        }
    }

    //

    /// Tries to convert this `Char24` to `Char7`.
    #[inline]
    pub const fn try_to_char7(self) -> Result<Char7> {
        Char7::try_from_char24(self)
    }
    /// Tries to convert this `Char24` to `Char8`.
    #[inline]
    pub const fn try_to_char8(self) -> Result<Char8> {
        Char8::try_from_char24(self)
    }
    /// Tries to convert this `Char24` to `Char16`.
    #[inline]
    pub const fn try_to_char16(self) -> Result<Char16> {
        Char16::try_from_char24(self)
    }
    /// Converts this `Char24` to `Char32`.
    #[inline]
    pub const fn to_char32(self) -> Char32 {
        Char32(self.to_char())
    }
    /// Converts this `Char24` to `u32`.
    #[inline]
    pub const fn to_u32(self) -> u32 {
        (self.hi.get() as u32) << 16 | (self.mi as u32) << 8 | (self.lo as u32)
    }
    /// Converts this `Char24` to `char`.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_char(self) -> char {
        // #[cfg(not(feature = "unsafe_char"))]
        if let Some(c) = char::from_u32(self.to_u32()) { c } else { unreachable![] }

        // WAITING for stable const: https://github.com/rust-lang/rust/issues/89259
        // SAFETY: we've already checked we contain a valid char.
        // #[cfg(feature = "unsafe_char")]
        // return unsafe { char::from_u32_unchecked(code_point) };
    }

    /// Converts this `Char24` to an UTF-8 encoded sequence of bytes.
    ///
    /// Note that this function always returns a 4-byte array, but the actual
    /// UTF-8 sequence may be shorter. The unused bytes are set to 0.
    //
    // https://en.wikipedia.org/wiki/UTF-8#Encoding
    #[inline]
    #[allow(clippy::unusual_byte_groupings)]
    pub const fn to_utf8_bytes(self) -> [u8; 4] {
        let c = self.to_u32();
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

    //

    /// Makes a copy of the value in its ASCII upper case equivalent.
    ///
    /// ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters
    /// are unchanged.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_ascii_uppercase(self) -> Char24 {
        Self::from_char(char::to_ascii_uppercase(&self.to_char()))
    }

    /// Makes a copy of the value in its ASCII lower case equivalent.
    ///
    /// ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters
    /// are unchanged.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_ascii_lowercase(self) -> Char24 {
        Self::from_char(char::to_ascii_lowercase(&self.to_char()))
    }

    /* queries */

    /// Returns `true` if this unicode scalar is a [noncharacter][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#noncharacter
    #[inline]
    pub const fn is_noncharacter(self) -> bool {
        is_noncharacter(self.to_u32())
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
        self.to_u32() <= 0x7F
    }
}

impl Char32 {
    /* constants */

    /// The highest unicode scalar a `Char32` can represent, `'\u{10FFFF}'`.
    pub const MAX: Char32 = Char32(char::MAX);

    /// `U+FFFD REPLACEMENT CHARACTER (�)` is used in Unicode to represent a decoding error.
    pub const REPLACEMENT_CHARACTER: Char32 = Char32(char::REPLACEMENT_CHARACTER);

    /* conversions */

    /// Converts a `Char7` to `Char32`.
    #[inline]
    pub const fn from_char7(c: Char7) -> Char32 {
        Char32(c.to_char())
    }
    /// Converts a `Char8` to `Char32`.
    #[inline]
    pub const fn from_char8(c: Char8) -> Char32 {
        Char32(c.to_char())
    }
    /// Converts a `Char16` to `Char32`.
    #[inline]
    pub const fn from_char16(c: Char16) -> Char32 {
        Char32(c.to_char())
    }
    /// Converts a `Char24` to `Char32`.
    #[inline]
    pub const fn from_char24(c: Char24) -> Char32 {
        Char32(c.to_char())
    }
    /// Converts a `char` to `Char32`.
    #[inline]
    pub const fn from_char(c: char) -> Char32 {
        Char32(c)
    }

    //

    /// Tries to convert this `Char32` to `Char7`.
    #[inline]
    pub const fn try_to_char7(self) -> Result<Char7> {
        Char7::try_from_char32(self)
    }
    /// Tries to convert this `Char32` to `Char8`.
    #[inline]
    pub const fn try_to_char8(self) -> Result<Char8> {
        Char8::try_from_char32(self)
    }
    /// Tries to convert this `Char32` to `Char16`.
    #[inline]
    pub const fn try_to_char16(self) -> Result<Char16> {
        Char16::try_from_char32(self)
    }
    /// Converts this `Char32` to `Char24`.
    #[inline]
    pub const fn to_char24(self) -> Char24 {
        Char24::from_char32(self)
    }
    /// Converts this `Char32` to `char`.
    #[inline]
    pub const fn to_char(self) -> char {
        self.0
    }
    /// Converts this `Char32` to `u32`.
    #[inline]
    pub const fn to_u32(self) -> u32 {
        self.0 as u32
    }

    /// Converts this `Char32` to an UTF-8 encoded sequence of bytes.
    ///
    /// Note that this function always returns a 4-byte array, but the actual
    /// UTF-8 sequence may be shorter. The unused bytes are set to 0.
    //
    // https://en.wikipedia.org/wiki/UTF-8#Encoding
    #[inline]
    #[allow(clippy::unusual_byte_groupings)]
    pub const fn to_utf8_bytes(self) -> [u8; 4] {
        let c = self.0 as u32;
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

    //

    /// Makes a copy of the value in its ASCII upper case equivalent.
    ///
    /// ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters
    /// are unchanged.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_ascii_uppercase(self) -> Char32 {
        Char32(char::to_ascii_uppercase(&self.0))
    }
    /// Makes a copy of the value in its ASCII lower case equivalent.
    ///
    /// ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters
    /// are unchanged.
    #[inline]
    #[rustfmt::skip]
    pub const fn to_ascii_lowercase(self) -> Char32 {
        Char32(char::to_ascii_lowercase(&self.0))
    }

    /* queries */

    /// Returns `true` if this unicode scalar is a [noncharacter][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#noncharacter
    #[inline]
    pub const fn is_noncharacter(self) -> bool {
        is_noncharacter(self.0 as u32)
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
        char::is_ascii(&self.0)
    }
}

/* traits for char */

impl UnicodeScalar for char {
    const MAX: Self = Self::MAX;

    /* encode */

    #[inline]
    fn byte_len(self) -> usize {
        byte_len(self as u32)
    }
    #[inline]
    fn len_utf8(self) -> usize {
        self.len_utf8()
    }
    #[inline]
    fn len_utf16(self) -> usize {
        self.len_utf16()
    }
    #[inline]
    fn encode_utf8(self, dst: &mut [u8]) -> &mut str {
        self.encode_utf8(dst)
    }
    #[inline]
    fn to_utf8_bytes(self) -> [u8; 4] {
        Char32(self).to_utf8_bytes()
    }
    #[inline]
    fn encode_utf16(self, dst: &mut [u16]) -> &mut [u16] {
        self.encode_utf16(dst)
    }
    #[inline]
    fn to_digit(self, radix: u32) -> Option<u32> {
        self.to_digit(radix)
    }
    #[inline]
    fn to_ascii_uppercase(self) -> char {
        char::to_ascii_uppercase(&self)
    }
    #[inline]
    fn to_ascii_lowercase(self) -> char {
        char::to_ascii_lowercase(&self)
    }

    /* queries */

    #[inline]
    fn is_noncharacter(self) -> bool {
        is_noncharacter(self as u32)
    }
    #[inline]
    fn is_digit(self, radix: u32) -> bool {
        self.is_digit(radix)
    }
    #[inline]
    fn is_control(self) -> bool {
        self.is_control()
    }
    #[inline]
    fn is_nul(self) -> bool {
        self as u32 == 0
    }
    #[inline]
    fn is_alphabetic(self) -> bool {
        self.is_alphabetic()
    }
    #[inline]
    fn is_numeric(self) -> bool {
        self.is_numeric()
    }
    #[inline]
    fn is_alphanumeric(self) -> bool {
        self.is_alphanumeric()
    }
    #[inline]
    fn is_lowercase(self) -> bool {
        self.is_lowercase()
    }
    #[inline]
    fn is_uppercase(self) -> bool {
        self.is_uppercase()
    }
    #[inline]
    fn is_whitespace(self) -> bool {
        self.is_whitespace()
    }

    /* ascii queries*/

    #[inline]
    fn is_ascii(self) -> bool {
        (self as u32) <= 0x7F
    }
}

/* helper fns */

/// Returns `true` if the given unicode scalar code is a 7bit ASCII code.
#[inline]
const fn is_noncharacter(code: u32) -> bool {
    // sub-block of 32 non-characters:
    (code >= 0xFDD0 && code <= 0xFDEF)
        // 2× non-characters at the end of each plane:
        || (code >= 0xFFFE && (code & 0xFF) == 0xFE)
        || (code >= 0xFFFE && (code & 0xFF) == 0xFF)
        // unallocated range (16 potential non-characters):
        || (code >= 0x2FE0 && code <= 0x2FEF)
    // surrogates (0xD800..=0xDFFF) are already filtered out in `char`.
}

/// Returns `true` if the given unicode scalar code is a 7bit ASCII code.
#[inline]
const fn is_7bit(code: u32) -> bool {
    code <= 0x7F
}

/// Returns the number of bytes necessary to store the given unicode scalar code.
#[inline]
const fn byte_len(code: u32) -> usize {
    match code {
        0..=0xFF => 1,
        0x100..=0xFFFF => 2,
        _ => 3,
    }
}
