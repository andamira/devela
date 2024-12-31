// devela::text::char::reexports

use crate::reexport;

#[doc = crate::TAG_PRIMITIVE!()]
/// <span class="stab portability" title="re-exported from rust's `core`">`core`</span>
/// A 32-bit [unicode scalar][scalar].
///
/// It can represent each and every scalar.
///
/// See also: [`char7`], [`char8`], [`char16`].
///
/// [scalar]: https://www.unicode.org/glossary/#unicode_scalar_value
/// [`char7`]: crate::char7
/// [`char8`]: crate::char8
/// [`char16`]: crate::char16
// NOTE: this reexport type is not recognized implicity by rustdoc, is it a BUG?
// TODO: minimal example and the search/make an ISSUE in rust-repo
#[allow(non_camel_case_types)]
pub type char = crate::_core::primitive::char;

reexport! { rust: core::str,
    tag: crate::TAG_ITERATOR!(),
    doc: "An iterator over the [`char`]s of a string slice.",
    @Chars as IterChars
}
