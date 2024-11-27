// devela::text::char::definitions
//
//! Define Char* structs and UnicodeScalar trait.
//
// TOC
// - struct char7
// - struct char8
// - struct char16
// - struct char24
// - trait UnicodeScalar

#![allow(non_camel_case_types)]

#[cfg(any(feature = "_char7", feature = "_char24"))]
pub(super) use crate::NonExtremeU8;
#[cfg(feature = "_char16")]
pub(super) use crate::NonValueU16;

// This is a surrogate UTF-16 code point that can't ever be a unicode scalar.
#[cfg(feature = "_char16")]
pub(super) type NonSurrogateU16 = NonValueU16<0xDFFF>;

/* primitive alias */

#[doc = crate::doc_primitive!()]
/// A 32-bit [unicode scalar value][scalar].
///
/// It can represent each and every scalar.
///
/// See also: [`char7`], [`char8`], [`char16`], [`char24`].
///
/// [scalar]: https://www.unicode.org/glossary/#unicode_scalar_value
// NOTE: this reexport type is not recognized implicity by rustdoc, is it a BUG?
// TODO: minimal example and the search/make an ISSUE in rust-repo
#[allow(non_camel_case_types)]
pub type char = crate::_core::primitive::char;

/* public types */

/// A 7-bit [unicode scalar value][scalar], limited to [basic latin][0w] subset
/// (ASCII).
///
/// `Option<char7>` is the same size as `char7` or `char8` (1 byte).
///
/// See also: [`char8`], [`char16`], [`char24`], [`char`][crate::char].
///
/// [scalar]: https://www.unicode.org/glossary/#unicode_scalar_value
/// [0w]: https://en.wikipedia.org/wiki/Basic_Latin_(Unicode_block)
#[repr(transparent)]
#[cfg(feature = "_char7")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char7")))]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct char7(pub(super) NonExtremeU8);

/// An 8-bit [unicode scalar value][scalar], limited to [basic latin][0w]
/// and [latin-1][1w] subsets.
///
/// This is the only scalar type without memory layout optimization
/// because each possible value is a valid unicode scalar. Therefore
/// `Option<char8>` is the same size as `char16` or `Option<char16>` (2 bytes).
///
/// See also: [`char7`], [`char16`], [`char24`], [`char`][crate::char].
///
/// [scalar]: https://www.unicode.org/glossary/#unicode_scalar_value
/// [0w]: https://en.wikipedia.org/wiki/Basic_Latin_(Unicode_block)
/// [1w]: https://en.wikipedia.org/wiki/Latin-1_Supplement
#[repr(transparent)]
#[cfg(feature = "_char8")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char8")))]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct char8(pub(super) u8);

/// A 16-bit [unicode scalar value][scalar], limited to the
/// [Basic Multilingual Plane][0w] subset.
///
/// It can represent every scalar from the [Basic Multilingual Plane][0w] (BMP),
/// the first and most important plane in the Unicode standard (also known as
/// plane 0), containing nearly all commonly used writing systems and symbols.
///
/// `Option<char16>` is the same size as `char16` (2 bytes).
///
/// See also: [`char7`], [`char8`], [`char24`], [`char`][crate::char].
///
/// [scalar]: https://www.unicode.org/glossary/#unicode_scalar_value
/// [0w]: https://en.wikipedia.org/wiki/Plane_(Unicode)#Basic_Multilingual_Plane
#[repr(transparent)]
#[cfg(feature = "_char16")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char16")))]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct char16(pub(super) NonSurrogateU16);

/// A 24-bit [unicode scalar value][scalar], unlimited value representation.
///
/// It can represent each and every scalar the same as [`char`][crate::char],
/// since the maximum value (`\u{10FFFF}`) needs only 21 bits.
///
/// `Option<char24>` is the same size as `char24` (3 bytes).
///
/// See also: [`char7`], [`char8`], [`char16`], [`char`][crate::char].
///
/// [scalar]: https://www.unicode.org/glossary/#unicode_scalar_value
#[cfg(feature = "_char24")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char24")))]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct char24 {
    pub(super) hi: NonExtremeU8, // highest byte
    pub(super) mi: u8,           // middle byte
    pub(super) lo: u8,           // lowest byte
}

/// Common trait for unicode scalar types.
///
/// It's implemented for: [`char7`], [`char8`], [`char16`], [`char24`],
/// and [`char`][crate::char].
pub trait UnicodeScalar {
    /// The highest unicode scalar that can be represented by this type.
    const MAX: Self;

    /* encode */

    /// Returns the number of bytes needed to represent the scalar value.
    #[must_use]
    fn byte_len(self) -> usize;

    /// Returns the number of bytes needed to encode in UTF-8.
    #[must_use]
    fn len_utf8(self) -> usize;

    /// Returns the number of bytes needed to encode in UTF-16.
    #[must_use]
    fn len_utf16(self) -> usize;

    /// Encodes this scalar as UTF-8 into the provided byte buffer,
    /// and then returns the subslice of the buffer that contains the encoded scalar.
    ///
    /// # Panics
    /// Panics if the buffer is not large enough.
    /// A buffer of length four is large enough to encode any char.
    #[must_use]
    fn encode_utf8(self, dst: &mut [u8]) -> &mut str;

    /// Converts this `scalar` to an UTF-8 encoded sequence of bytes.
    ///
    /// Note that this function always returns a 4-byte array, but the actual
    /// UTF-8 sequence may be shorter. The unused bytes are set to 0.
    #[must_use]
    fn to_utf8_bytes(self) -> [u8; 4];

    /// Encodes this scalar as UTF-16 into the provided byte buffer,
    /// and then returns the subslice of the buffer that contains the encoded scalar.
    ///
    /// # Panics
    /// Panics if the buffer is not large enough.
    /// A buffer of length 2 is large enough to encode any char.
    #[must_use]
    fn encode_utf16(self, dst: &mut [u16]) -> &mut [u16];

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
    #[must_use]
    fn to_digit(self, radix: u32) -> Option<u32>;

    /// Makes a copy of the value in its ASCII upper case equivalent.
    ///
    /// ASCII letters ‘a’ to ‘z’ are mapped to ‘A’ to ‘Z’, but non-ASCII letters
    /// are unchanged.
    #[must_use]
    fn to_ascii_uppercase(self) -> Self
    where
        Self: Sized;

    /// Makes a copy of the value in its ASCII lower case equivalent.
    ///
    /// ASCII letters ‘A’ to ‘Z’ are mapped to ‘a’ to ‘z’, but non-ASCII letters
    /// are unchanged.
    #[must_use]
    fn to_ascii_lowercase(self) -> Self
    where
        Self: Sized;

    /* escape */

    /* queries */

    /// Returns `true` if this unicode scalar is a [noncharacter][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#noncharacter
    #[must_use]
    fn is_noncharacter(self) -> bool;

    /// Returns `true` if this unicode scalar is an [abstract character][0].
    ///
    /// [0]: https://www.unicode.org/glossary/#abstract_character
    #[must_use]
    fn is_character(self) -> bool
    where
        Self: Sized,
    {
        !self.is_noncharacter()
    }

    /// Checks if the unicode scalar is a digit in the given radix.
    ///
    /// See also [`to_digit`][Self#method.to_digit].
    #[must_use]
    fn is_digit(self, radix: u32) -> bool;

    /// Returns `true` if this unicode scalar has the general category for
    /// control codes.
    #[must_use]
    fn is_control(self) -> bool;

    /// Returns `true` if this unicode scalar is the nul character (`0x00`).
    #[must_use]
    fn is_nul(self) -> bool;

    /// Returns `true` if this unicode scalar has the `Alphabetic` property.
    #[must_use]
    fn is_alphabetic(self) -> bool;

    /// Returns `true` if this unicode scalar has one of the general categories
    /// for numbers.
    ///
    /// If you want to parse ASCII decimal digits (0-9) or ASCII base-N,
    /// use [`is_ascii_digit`][Self#method.is_ascii_digit] or
    /// [`is_digit`][Self#method.is_digit] instead.
    #[must_use]
    fn is_numeric(self) -> bool;

    /// Returns `true` if this unicode scalar satisfies either
    /// [`is_alphabetic()`][Self#method.is_alphabetic] or
    /// [`is_numeric()`][Self#method.is_numeric].
    #[must_use]
    fn is_alphanumeric(self) -> bool;

    /// Returns `true` if this unicode scalar has the `Lowercase` property.
    #[must_use]
    fn is_lowercase(self) -> bool;

    /// Returns `true` if this unicode scalar has the `Lowercase` property.
    #[must_use]
    fn is_uppercase(self) -> bool;

    /// Returns `true` if this unicode scalar has the `White_Space` property.
    #[must_use]
    fn is_whitespace(self) -> bool;

    /* ascii */

    /// Checks if the value is within the ASCII range.
    #[must_use]
    fn is_ascii(self) -> bool;
}
