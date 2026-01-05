// devela_base_core::text::char::scalar::definitions
//
//! Define [`char7`], [`char8`], [`char16`], [`charu`].
//
// TOC
// - macro ch!
// - struct char7
// - struct char8
// - struct char16
// - struct charu
// - struct charu_niche

#![allow(non_camel_case_types)]

pub(crate) use crate::{NonExtremeU8, NonExtremeU32, NonNiche, NonValueU16};

// This is a surrogate UTF-16 code point that can't ever be a Unicode scalar.
pub(crate) type NonSurrogateU16 = NonValueU16<0xDFFF>;

#[doc = crate::_TAG_TEXT!()]
#[doc = crate::_TAG_CONSTRUCTION!()]
/// Concisely creates any kind of Unicode scalar.
#[doc = crate::_doc_location!("text/char")]
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! ch {
    ( // charu
    u $char:literal) => { $crate::charu::from_char($char) };
    (u str $str:literal) => { $crate::charu::from_str_unchecked($str) };
    (u bytes $str:literal) => { $crate::unwrap![some $crate::charu::from_utf8_bytes($str)] };
    ( // charu_niche
    un $char:literal) => { $crate::charu_niche::from_char($char) };
    (un str $str:literal) => { $crate::charu_niche::from_str_unchecked($str) };
    (un bytes $str:literal) => { $crate::unwrap![some $crate::charu_niche::from_utf8_bytes($str)] };
    (
    c7 $char:literal) => { $crate::unwrap![some $crate::char7::try_from_char($char)] };
    (c8 $char:literal) => { $crate::unwrap![some $crate::char8::try_from_char($char)] };
    (c16 $char:literal) => { $crate::unwrap![some $crate::char16::try_from_char($char)] };
}
pub use ch;

/* public types */

#[doc = crate::_TAG_TEXT!()]
/// A 7-bit [Unicode scalar][scalar], limited to [basic latin][0w] subset
/// (ASCII).
#[doc = crate::_doc_location!("text/char")]
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
#[doc = crate::_doc_location!("text/char")]
///
/// `Option<char8>` is the same size as `char16` or `Option<char16>` (2 bytes),
/// because each possible value is a valid Unicode scalar.
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
#[doc = crate::_doc_location!("text/char")]
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

#[doc = crate::_TAG_TEXT!()]
/// A 32-bit [Unicode scalar][scalar], with UTF-8 representation.
#[doc = crate::_doc_location!("text/char")]
///
/// It stores the UTF-8 bytes in big-endian order, similarly as a [`str`].
///
/// `Option<charu>` occupies extra space (8 bytes),
/// priritizing speed over memory. For the opposite trade-off see [`charu_niche`].
///
/// [scalar]: https://www.unicode.org/glossary/#unicode_scalar_value
#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct charu(pub(super) NonNiche<u32>);

#[doc = crate::_TAG_TEXT!()]
/// A 32-bit [Unicode scalar][scalar], with UTF-8 representation,
/// and niche-memory optimization.
#[doc = crate::_doc_location!("text/char")]
///
/// It stores the UTF-8 bytes in big-endian order, similarly as a [`str`].
///
/// `Option<charu_niche>` is the same size as `charu_niche` (4 bytes),
/// prioritizing memory over speed. For the opposite trade-off see [`charu`].
///
/// [scalar]: https://www.unicode.org/glossary/#unicode_scalar_value
#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct charu_niche(pub(super) NonExtremeU32);

#[cfg(test)]
const _TEST_CHAR_SIZES: () = {
    assert![size_of::<char7>() == 1];
    assert![size_of::<Option<char7>>() == 1];

    assert![size_of::<char8>() == 1];
    assert![size_of::<Option<char8>>() == 2];

    assert![size_of::<char16>() == 2];
    assert![size_of::<Option<char16>>() == 2];

    assert![size_of::<charu>() == 4];
    assert![size_of::<Option<charu>>() == 8];

    assert![size_of::<charu_niche>() == 4];
    assert![size_of::<Option<charu_niche>>() == 4];

    assert![size_of::<char>() == 4];
    assert![size_of::<Option<char>>() == 4];
};
