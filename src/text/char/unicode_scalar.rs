// devela_base_core::text::char::char
//
//! Define the [`UnicodeScalar`] trait.
//
// TOC
// - trait UnicodeScalar
// - impl for char
// - impl for char[7|8|16]
// - impl for charu[_niche]

use crate::{Char, char7, char8, char16, charu, charu_niche};

#[rustfmt::skip]
#[doc = crate::_tags!(text)]
/// Common trait for Unicode scalar types.
///
#[doc = crate::_doc_location!("text/char")]
///
/// It's implemented for: [`char7`], [`char8`], [`char16`],
/// and [`char`][crate::char].
pub trait UnicodeScalar {
    /// The lowest Unicode scalar that can be represented.
    const MIN: Self;

    /// The highest Unicode scalar that can be represented.
    const MAX: Self;

    /* encode */

    /// Returns itself as a `char`.
    #[must_use]
    fn to_char(self) -> char;

    /// Returns itself as a `u32` scalar.
    fn to_scalar(self) -> u32;

    // // MAYBE
    // fn to_charu(self) -> char_utf;

    #[must_use]
    /// Returns the number of bytes needed to represent the scalar value.
    fn len_bytes(self) -> usize;

    #[must_use]
    /// Returns the number of bytes needed to encode in UTF-8.
    fn len_utf8(self) -> usize;

    #[must_use]
    /// Returns the number of bytes needed to encode in UTF-16.
    fn len_utf16(self) -> usize;

    #[must_use]
    /// Encodes this scalar as UTF-8 into the provided byte buffer,
    /// and then returns the subslice of the buffer that contains the encoded scalar.
    ///
    /// # Panics
    /// Panics if the buffer is not large enough.
    /// A buffer of length four is large enough to encode any char.
    fn encode_utf8(self, dst: &mut [u8]) -> &mut str;

    #[must_use]
    /// Converts this `scalar` to an UTF-8 encoded sequence of bytes.
    ///
    /// Note that this function always returns a 4-byte array, but the actual
    /// UTF-8 sequence may be shorter. The unused bytes are set to 0.
    fn to_utf8_bytes(self) -> [u8; 4];

    #[must_use]
    /// Encodes this scalar as UTF-16 into the provided byte buffer,
    /// and then returns the subslice of the buffer that contains the encoded scalar.
    ///
    /// # Panics
    /// Panics if the buffer is not large enough.
    /// A buffer of length 2 is large enough to encode any char.
    fn encode_utf16(self, dst: &mut [u16]) -> &mut [u16];

    #[must_use]
    /// Converts the scalar to a digit in the given radix.
    ///
    /// 'Digit' is defined to be only the following characters:
    /// `0-9`, `a-z`, `A-Z`.
    ///
    /// # Errors
    /// Returns None if the char does not refer to a digit in the given radix.
    ///
    /// # Panics
    /// Panics if given a radix larger than 36.
    fn to_digit(self, radix: u32) -> Option<u32>;

    #[must_use]
    /// Makes a copy of the value in its ASCII upper case equivalent.
    ///
    /// ASCII letters 'a' to 'z' are mapped to 'A' to 'Z', but non-ASCII letters
    /// are unchanged.
    fn to_ascii_uppercase(self) -> Self where Self: Sized;

    #[must_use]
    /// Makes a copy of the value in its ASCII lower case equivalent.
    ///
    /// ASCII letters 'A' to 'Z' are mapped to 'a' to 'z', but non-ASCII letters
    /// are unchanged.
    fn to_ascii_lowercase(self) -> Self where Self: Sized;

    #[must_use]
    /// Returns the ASCII transliteration of the value.
    #[cfg(feature = "translit")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "translit")))]
    fn as_ascii_translit(self) -> &'static str where Self: Sized {
        crate::scalar_as_ascii_translit(self.to_scalar())
    }

    /* escape */

    /* queries */

    // basic ASCII checks

    #[must_use]
    /// Checks if the value is within the ASCII range.
    fn is_ascii(self) -> bool;

    #[must_use]
    /// Returns `true` if this Unicode scalar is the nul character (`0x00`).
    fn is_nul(self) -> bool;

    // character type categories (general classification)

    #[must_use]
    /// Returns `true` if this Unicode scalar has the `Alphabetic` property.
    fn is_alphabetic(self) -> bool;

    #[must_use]
    /// Returns `true` if this Unicode scalar has one of the general categories for numbers.
    ///
    /// If you want to parse ASCII decimal digits (0-9) or ASCII base-N,
    /// use [`is_ascii_digit`][Self#method.is_ascii_digit] or
    /// [`is_digit`][Self#method.is_digit] instead.
    fn is_numeric(self) -> bool;

    #[must_use]
    /// Returns `true` if this Unicode scalar satisfies either
    /// [`is_alphabetic()`][Self#method.is_alphabetic] or
    /// [`is_numeric()`][Self#method.is_numeric].
    fn is_alphanumeric(self) -> bool;

    #[must_use]
    /// Checks if the Unicode scalar is a digit in the given radix.
    ///
    /// See also [`to_digit`][Self#method.to_digit].
    fn is_digit(self, radix: u32) -> bool;

    // casing (related to alphabetic)

    #[must_use]
    /// Returns `true` if this Unicode scalar has the `Lowercase` property.
    fn is_lowercase(self) -> bool;

    #[must_use]
    /// Returns `true` if this Unicode scalar has the `Uppercase` property.
    fn is_uppercase(self) -> bool;

    // whitespace and controls (structural characters)

    #[must_use]
    /// Returns `true` if this Unicode scalar has the `White_Space` property.
    fn is_whitespace(self) -> bool;

    #[must_use]
    /// Returns `true` if this Unicode scalar has the general category for control codes.
    ///
    /// This crate's implementations leverage [`char::is_control()`],
    /// instead of [`Char::is_control()`][crate::Char::is_control].
    fn is_control(self) -> bool;

    /// Optional faster control mark check for common text.
    fn is_control_common(self) -> bool where Self: Sized { self.is_control() }

    // special Unicode categories (edge cases, least common)

    #[must_use]
    /// Returns `true` if this Unicode scalar is a [noncharacter][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#noncharacter
    fn is_noncharacter(self) -> bool;

    #[must_use]
    /// Returns `true` if this Unicode scalar is an [abstract character][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#abstract_character
    fn is_character(self) -> bool where Self: Sized { !self.is_noncharacter() }

    #[must_use]
    /// Returns `true` if this Unicode scalar is a [combining character][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#combining_character
    fn is_combining(self) -> bool;

    #[must_use]
    /// Optional faster combining mark check for common text.
    ///
    /// By default, this calls `is_combining()`. Implementations may override
    /// this with a faster version that only checks common ranges, but may
    /// return false negatives for rare [combining characters][0].
    ///
    /// For complete Unicode compliance, use `is_combining()` instead.
    ///
    /// [0]: https://www.unicode.org/glossary/#combining_character
    fn is_combining_common(self) -> bool where Self: Sized { self.is_combining() }

    #[must_use]
    /// Returns `true` if this Unicode scalar is a [fullwidth][0] character.
    ///
    /// [0]: https://www.unicode.org/glossary/#fullwidth
    fn is_fullwidth(self) -> bool;

    #[must_use]
    /// Optional faster fullwidth check for common text.
    ///
    /// By default, this calls `is_fullwidth()`. Implementations may override
    /// this with a faster version that only checks common ranges, but may
    /// return false negatives for rare [fullwidth][0] characters.
    ///
    /// For complete Unicode compliance, use `is_fullwidth()` instead.
    ///
    /// [0]: https://www.unicode.org/glossary/#fullwidth
    fn is_fullwidth_common(self) -> bool where Self: Sized { self.is_fullwidth() }
}

/* impls */

#[rustfmt::skip]
impl UnicodeScalar for char {
    const MIN: Self = char::MIN;
    const MAX: Self = char::MAX;

    /* encode */

    fn to_char(self) -> char { self }
    fn to_scalar(self) -> u32 { self as u32 }
    fn len_bytes(self) -> usize { Char(self as u32).len_bytes() }
    fn len_utf8(self) -> usize { self.len_utf8() }
    fn len_utf16(self) -> usize { self.len_utf16() }
    fn encode_utf8(self, dst: &mut [u8]) -> &mut str { self.encode_utf8(dst) }
    fn to_utf8_bytes(self) -> [u8; 4] { Char(self).to_utf8_bytes() }
    fn encode_utf16(self, dst: &mut [u16]) -> &mut [u16] { self.encode_utf16(dst) }
    fn to_digit(self, radix: u32) -> Option<u32> { self.to_digit(radix) }
    fn to_ascii_uppercase(self) -> char { char::to_ascii_uppercase(&self) }
    fn to_ascii_lowercase(self) -> char { char::to_ascii_lowercase(&self) }

    /* queries */

    fn is_ascii(self) -> bool { (self as u32) <= 0x7F }
    fn is_nul(self) -> bool { self as u32 == 0 }
    fn is_alphabetic(self) -> bool { self.is_alphabetic() }
    fn is_numeric(self) -> bool { self.is_numeric() }
    fn is_alphanumeric(self) -> bool { self.is_alphanumeric() }
    fn is_digit(self, radix: u32) -> bool { self.is_digit(radix) }
    fn is_lowercase(self) -> bool { self.is_lowercase() }
    fn is_uppercase(self) -> bool { self.is_uppercase() }
    fn is_whitespace(self) -> bool { self.is_whitespace() }
    fn is_control(self) -> bool { self.is_control() }
    fn is_control_common(self) -> bool { Char(self as u32).is_control_common() }
    fn is_noncharacter(self) -> bool { Char(self as u32).is_noncharacter() }
    fn is_combining(self) -> bool { Char(self as u32).is_combining() }
    fn is_combining_common(self) -> bool { Char(self as u32).is_combining_common() }
    fn is_fullwidth(self) -> bool { Char(self as u32).is_fullwidth() }
    fn is_fullwidth_common(self) -> bool { Char(self as u32).is_fullwidth_common() }
}

/// Implements `UnicodeScalar` for custom char types.
macro_rules! impl_char {
    () => {
        impl_char![7, 8, 16];
    };
    ($( $bits:literal),+ ) => { $crate::paste! {
        $( impl_char!(@[<char $bits>]); )+
    }};
    (@$name:ident) => {

        /* impl traits */

        impl UnicodeScalar for $name { // TODO:IMPROVE avoid converting to char
            const MIN: Self = Self::MIN;
            const MAX: Self = Self::MAX;

            /* encode */

            fn to_char(self) -> char { self.to_char() }
            fn to_scalar(self) -> u32 { self.to_scalar() }
            fn len_bytes(self) -> usize { self.len_bytes() }
            fn len_utf8(self) -> usize { self.len_utf8() }
            fn len_utf16(self) -> usize { self.len_utf16() }
            fn encode_utf8(self, dst: &mut [u8]) -> &mut str {
                self.to_char().encode_utf8(dst)
            }
            fn to_utf8_bytes(self) -> [u8; 4] {
                let dyn_array = self.to_utf8_bytes();
                let mut array = [0u8; 4];
                for i in 0..dyn_array.len() {
                    array[i] = dyn_array[i];
                }
                array
            }
            fn encode_utf16(self, dst: &mut [u16]) -> &mut [u16] {
                self.to_char().encode_utf16(dst)
            }
            fn to_digit(self, radix: u32) -> Option<u32> { self.to_digit(radix) }
            fn to_ascii_uppercase(self) -> Self { self.to_ascii_uppercase() }
            fn to_ascii_lowercase(self) -> Self { self.to_ascii_lowercase() }

            /* queries */

            fn is_ascii(self) -> bool { self.is_ascii() }
            fn is_nul(self) -> bool { self.is_nul() }
            fn is_alphabetic(self) -> bool { self.to_char().is_alphabetic() }
            fn is_numeric(self) -> bool { self.to_char().is_numeric() }
            fn is_alphanumeric(self) -> bool { self.to_char().is_alphanumeric() }
            fn is_digit(self, radix: u32) -> bool { self.is_digit(radix) }
            fn is_lowercase(self) -> bool { self.to_char().is_lowercase() }
            fn is_uppercase(self) -> bool { self.to_char().is_uppercase() }
            fn is_whitespace(self) -> bool { self.to_char().is_whitespace() }
            fn is_control(self) -> bool { self.to_char().is_control() }
            fn is_control_common(self) -> bool { Char(self.to_scalar()).is_control_common() }
            fn is_noncharacter(self) -> bool { self.is_noncharacter() }
            fn is_combining(self) -> bool { Char(self.to_scalar()).is_combining() }
            fn is_combining_common(self) -> bool { Char(self.to_scalar()).is_combining_common() }
            fn is_fullwidth(self) -> bool { Char(self.to_scalar()).is_fullwidth() }
            fn is_fullwidth_common(self) -> bool { Char(self.to_scalar()).is_fullwidth_common() }
        }
    };
}
impl_char!();

macro_rules! impl_charu {
    () => { impl_charu![charu, charu_niche]; };
    ($( $name:ident),+ ) => { $( impl_charu!(@$name); )+ };
    (@$name:ident) => {

        impl UnicodeScalar for $name { // TODO:IMPROVE avoid converting to char
            const MIN: Self = Self::MIN;
            const MAX: Self = Self::MAX;

            /* encode */

            fn to_char(self) -> char { self.to_char() }
            fn to_scalar(self) -> u32 { self.to_scalar() }
            fn len_bytes(self) -> usize { self.len_bytes() }
            fn len_utf8(self) -> usize { self.len_utf8() }
            fn len_utf16(self) -> usize { self.to_char().len_utf16() }
            fn encode_utf8(self, dst: &mut [u8]) -> &mut str { self.to_char().encode_utf8(dst) }
            fn to_utf8_bytes(self) -> [u8; 4] { self.to_utf8_bytes() }
            fn encode_utf16(self, dst: &mut [u16]) -> &mut [u16] { self.to_char().encode_utf16(dst) }
            fn to_digit(self, radix: u32) -> Option<u32> { self.to_char().to_digit(radix) }
            fn to_ascii_uppercase(self) -> Self {
                Self::from_char(char::to_ascii_uppercase(&self.to_char()))
            }
            fn to_ascii_lowercase(self) -> Self {
                Self::from_char(char::to_ascii_lowercase(&self.to_char()))
            }

            /* queries */

            fn is_ascii(self) -> bool { self.is_ascii() }
            fn is_nul(self) -> bool { self.is_nul() }
            fn is_alphabetic(self) -> bool { self.to_char().is_alphabetic() }
            fn is_numeric(self) -> bool { self.to_char().is_numeric() }
            fn is_alphanumeric(self) -> bool { self.to_char().is_alphanumeric() }
            fn is_digit(self, radix: u32) -> bool { self.to_char().is_digit(radix) }
            fn is_lowercase(self) -> bool { self.to_char().is_lowercase() }
            fn is_uppercase(self) -> bool { self.to_char().is_uppercase() }
            fn is_whitespace(self) -> bool { self.to_char().is_whitespace() }
            fn is_control(self) -> bool { self.to_char().is_control() }
            fn is_noncharacter(self) -> bool { Char(self.to_scalar()).is_noncharacter() }
            fn is_combining(self) -> bool { Char(self.to_scalar()).is_combining() }
            fn is_combining_common(self) -> bool { Char(self.to_scalar()).is_combining_common() }
            fn is_fullwidth(self) -> bool { Char(self.to_scalar()).is_fullwidth() }
            fn is_fullwidth_common(self) -> bool { Char(self.to_scalar()).is_fullwidth_common() }
        }
    };
}
impl_charu!();
