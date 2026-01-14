// devela::text::grapheme::family_kind
//
//! Defines [`GraphemeKind`].
//

#[doc = crate::_tags!(text)]
/// The kinds of supported graphemes.
#[doc = crate::_doc_location!("text/grapheme")]
#[derive(Clone, Debug, PartialEq, Eq)]
#[allow(non_camel_case_types, missing_docs)]
pub enum GraphemeKind {
    /* graphemes */
    #[cfg(feature = "grapheme")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "grapheme")))]
    Nonul,

    #[cfg(feature = "grapheme")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "grapheme")))]
    U8,

    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    String,

    /* scalars */
    char,
    char7,
    char8,
    char16,
    charu,
    charu_niche,
}
