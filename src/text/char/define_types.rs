// devela::text::char::definitions
//
//! Char* struct definitions (unicode scalars)
//

#![allow(non_camel_case_types)]

#[cfg(any(feature = "_char7", feature = "_char24"))]
pub(super) use crate::num::NonExtremeU8;
#[cfg(feature = "_char16")]
pub(super) use crate::num::NonValueU16;

// This is a surrogate UTF-16 code point that can't ever be a unicode scalar.
#[cfg(feature = "_char16")]
pub(super) type NonSurrogateU16 = NonValueU16<0xDFFF>;

/* public types */

/// A 7-bit [unicode scalar value][scalar], limited to [basic latin][0w] subset
/// (ASCII).
///
/// `Option<char7>` is the same size as `char7` or `char8` (1 byte).
///
/// See also: [`char8`], [`char16`], [`char24`], [`char`].
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
/// See also: [`char7`], [`char16`], [`char24`], [`char`].
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
/// See also: [`char7`], [`char8`], [`char24`], [`char`].
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
/// It can represent each and every scalar the same as [`char`],
/// since the maximum value (`\u{10FFFF}`) needs only 21 bits.
///
/// `Option<char24>` is the same size as `char24` (3 bytes).
///
/// See also: [`char7`], [`char8`], [`char16`], [`char`].
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
