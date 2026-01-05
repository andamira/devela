// devela_base_core::text::char::_reexport

#[doc = crate::_TAG_PRIMITIVE!()]
#[doc = crate::_TAG_TEXT!()]
/// <span class="stab portability" title="re-exported from rust's `core`">`core`</span>
/// A 32-bit [Unicode scalar][scalar].
#[doc = crate::_doc_location!("text/char")]
///
/// It can represent each and every scalar.
///
/// See also: [`char7`], [`char8`], [`char16`].
///
/// [scalar]: https://www.unicode.org/glossary/#unicode_scalar_value
/// [`char7`]: crate::char7
/// [`char8`]: crate::char8
/// [`char16`]: crate::char16
///
/// ---
/// ---
// NOTE: this reexport type is not recognized implicity by rustdoc, is it a BUG?
// TODO: minimal example and the search/make an ISSUE in rust-repo
#[allow(non_camel_case_types)]
pub type char = ::core::primitive::char;

// NOTE: replaced by `CharIter`
// _reexport! { rust: core::str,
//     tag: _TAG_TEXT!() crate::_TAG_ITERATOR!(),
//     doc: "An iterator over the [`char`][prim@char]s of a string slice.",
//     @Chars as IterChars
// }
