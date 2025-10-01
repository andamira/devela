// devela_base_core::text::char::definitions
//
//! Define [`char7`], [`char8`], [`char16`], [`char_utf8`].
//
// TOC
// - struct char7
// - struct char8
// - struct char16
// - struct char_utf8

#![allow(non_camel_case_types)]

pub(super) use crate::{NonExtremeU8, NonExtremeU32, NonValueU16};

// This is a surrogate UTF-16 code point that can't ever be a Unicode scalar.
pub(super) type NonSurrogateU16 = NonValueU16<0xDFFF>;

/* public types */

#[doc = crate::_TAG_TEXT!()]
/// A 7-bit [Unicode scalar][scalar], limited to [basic latin][0w] subset
/// (ASCII).
///
#[doc = crate::_doc!(location: "text/char")]
///
/// `Option<char7>` is the same size as `char7` or `char8` (1 byte).
///
/// See also: [`char8`], [`char16`], [`char`][crate::char].
///
/// [scalar]: https://www.unicode.org/glossary/#unicode_scalar_value
/// [0w]: https://en.wikipedia.org/wiki/Basic_Latin_(Unicode_block)
#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct char7(pub(super) NonExtremeU8);

#[doc = crate::_TAG_TEXT!()]
/// An 8-bit [Unicode scalar][scalar], limited to [basic latin][0w]
/// and [latin-1][1w] subsets.
///
#[doc = crate::_doc!(location: "text/char")]
///
/// This is the only scalar type without memory layout optimization
/// because each possible value is a valid Unicode scalar. Therefore
/// `Option<char8>` is the same size as `char16` or `Option<char16>` (2 bytes).
///
/// See also: [`char7`], [`char16`], [`char`][crate::char].
///
/// [scalar]: https://www.unicode.org/glossary/#unicode_scalar_value
/// [0w]: https://en.wikipedia.org/wiki/Basic_Latin_(Unicode_block)
/// [1w]: https://en.wikipedia.org/wiki/Latin-1_Supplement
#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct char8(pub(crate) u8);

#[doc = crate::_TAG_TEXT!()]
/// A 16-bit [Unicode scalar][scalar], limited to the
/// [Basic Multilingual Plane][0w] subset.
///
#[doc = crate::_doc!(location: "text/char")]
///
/// It can represent every scalar from the [Basic Multilingual Plane][0w] (BMP),
/// the first and most important plane in the Unicode standard (also known as
/// plane 0), containing nearly all commonly used writing systems and symbols.
///
/// `Option<char16>` is the same size as `char16` (2 bytes).
///
/// See also: [`char7`], [`char8`], [`char`][crate::char].
///
/// [scalar]: https://www.unicode.org/glossary/#unicode_scalar_value
/// [0w]: https://en.wikipedia.org/wiki/Plane_(Unicode)#Basic_Multilingual_Plane
#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct char16(pub(super) NonSurrogateU16);

/// A 32-bit [Unicode scalar][scalar], with UTF-8 representation.
///
#[doc = crate::_doc!(location: "text/char")]
///
/// It stores the UTF-8 bytes in big-endian order, similarly as a [`str`].
///
/// [scalar]: https://www.unicode.org/glossary/#unicode_scalar_value
#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct char_utf8(pub(super) NonExtremeU32);
