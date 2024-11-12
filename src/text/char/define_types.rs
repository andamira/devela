// devela::text::char::definitions
//
//! Char* struct definitions (unicode scalars)
//

#[cfg(any(feature = "_char_u7", feature = "_char_u24"))]
pub(super) use crate::num::NonExtremeU8;
#[cfg(feature = "_char_u16")]
pub(super) use crate::num::NonValueU16;

// This is a surrogate UTF-16 code point that can't ever be a unicode scalar.
#[cfg(feature = "_char_u16")]
pub(super) type NonSurrogateU16 = NonValueU16<0xDFFF>;

/* public types */

/// A 7-bit [unicode scalar value][scalar], limited to [basic latin][0w] subset
/// (ASCII).
///
/// `Option<CharU7>` is the same size as `CharU7` or `CharU8` (1 byte).
///
/// See also: [`CharU8`], [`CharU16`], [`CharU24`], [`CharU32`], [`char`].
///
/// [scalar]: https://www.unicode.org/glossary/#unicode_scalar_value
/// [0w]: https://en.wikipedia.org/wiki/Basic_Latin_(Unicode_block)
#[repr(transparent)]
#[cfg(feature = "_char_u7")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_7")))]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CharU7(pub(super) NonExtremeU8);

/// An 8-bit [unicode scalar value][scalar], limited to [basic latin][0w]
/// and [latin-1][1w] subsets.
///
/// This is the only scalar type without memory layout optimization
/// because each possible value is a valid unicode scalar. Therefore
/// `Option<CharU8>` is the same size as `CharU16` or `Option<CharU16>` (2 bytes).
///
/// See also: [`CharU7`], [`CharU16`], [`CharU24`], [`CharU32`], [`char`].
///
/// [scalar]: https://www.unicode.org/glossary/#unicode_scalar_value
/// [0w]: https://en.wikipedia.org/wiki/Basic_Latin_(Unicode_block)
/// [1w]: https://en.wikipedia.org/wiki/Latin-1_Supplement
#[repr(transparent)]
#[cfg(feature = "_char_u8")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_8")))]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CharU8(pub(super) u8);

/// A 16-bit [unicode scalar value][scalar], limited to the
/// [Basic Multilingual Plane][0w] subset.
///
/// It can represent every scalar from the [Basic Multilingual Plane][0w] (BMP),
/// the first and most important plane in the Unicode standard (also known as
/// plane 0), containing nearly all commonly used writing systems and symbols.
///
/// `Option<CharU16>` is the same size as `CharU16` (2 bytes).
///
/// See also: [`CharU7`], [`CharU8`], [`CharU24`], [`CharU32`], [`char`].
///
/// [scalar]: https://www.unicode.org/glossary/#unicode_scalar_value
/// [0w]: https://en.wikipedia.org/wiki/Plane_(Unicode)#Basic_Multilingual_Plane
#[repr(transparent)]
#[cfg(feature = "_char_u16")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_16")))]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CharU16(pub(super) NonSurrogateU16);

/// A 24-bit [unicode scalar value][scalar], unlimited value representation.
///
/// It can represent each and every scalar the same as [`CharU32`],
/// since the maximum value (`\u{10FFFF}`) needs only 21 bits.
///
/// `Option<CharU24>` is the same size as `CharU24` (3 bytes).
///
/// See also: [`CharU7`], [`CharU8`], [`CharU16`], [`CharU32`], [`char`].
///
/// [scalar]: https://www.unicode.org/glossary/#unicode_scalar_value
#[cfg(feature = "_char_u24")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_24")))]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CharU24 {
    pub(super) hi: NonExtremeU8, // highest byte
    pub(super) mi: u8,           // middle byte
    pub(super) lo: u8,           // lowest byte
}

/// A 32-bit [unicode scalar value][scalar], unlimited value representation,
/// wraps a [`char`].
///
/// This type wraps the default unicode scalar type in Rust.
/// It can represent the same range of unicode scalars as [`CharU24`].
///
/// `Option<CharU32>` is the same size as `CharU32` or `char` (4 bytes).
///
/// See also: [`CharU7`], [`CharU8`], [`CharU16`], [`CharU24`], [`char`].
///
/// [scalar]: https://www.unicode.org/glossary/#unicode_scalar_value
#[repr(transparent)]
#[cfg(feature = "_char_u32")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_32")))]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct CharU32(pub char);
