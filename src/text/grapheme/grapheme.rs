// devela::text::grapheme::grapheme
//
//! Defines the [`Grapheme`] trait.
//
// TOC
// - trait Grapheme
// - impls
// - tests

use crate::GraphemeKind;

#[doc = crate::_TAG_TEXT!()]
/// Common trait for <abbr title="Extended Grapheme Cluster">EGC</abbr> types.
#[doc = crate::_doc_location!("text/grapheme")]
pub trait Grapheme {
    /// Returns an iterator over Unicode scalars.
    // NOTE: this method makes the trait not dyn compatible
    fn grapheme_chars(&self) -> impl Iterator<Item = char>;

    /// Returns the kind discriminant.
    fn grapheme_kind(&self) -> GraphemeKind;

    /// Returns the length in number of bytes.
    #[must_use]
    fn grapheme_len_bytes(&self) -> usize;

    /// Returns the length in number of UTF-8 bytes.
    #[must_use]
    fn grapheme_len_utf8(&self) -> usize;

    /* non-required */

    /// Returns the length in number of Unicode scalars.
    #[must_use]
    fn grapheme_len_chars(&self) -> usize {
        self.grapheme_chars().count()
    }

    /// Returns the kind discriminant.
    #[must_use]
    fn grapheme_is_kind(&self, kind: GraphemeKind) -> bool {
        self.grapheme_kind() == kind
    }
}

#[rustfmt::skip]
mod impls {
    use crate::{Char,Grapheme, GraphemeKind, GraphemeNonul, GraphemeU8, Iter,
    char, charu, charu_niche, char7, char8, char16};

    /* for graphemes */

    crate::items! {
        impl<const CAP: usize> Grapheme for GraphemeNonul<CAP> {
            fn grapheme_chars(&self) -> impl Iterator<Item = char> { self.chars() }
            fn grapheme_kind(&self) -> GraphemeKind { GraphemeKind::Nonul }
            fn grapheme_len_bytes(&self) -> usize { self.len() }
            fn grapheme_len_utf8(&self) -> usize { self.len() }
        }
        impl<const CAP: usize> Grapheme for GraphemeU8<CAP> {
            fn grapheme_chars(&self) -> impl Iterator<Item = char> { self.chars() }
            // fn grapheme_family(self) -> GraphemeFamily<CAP> { GraphemeFamily::U8(self) }
            fn grapheme_kind(&self) -> GraphemeKind { GraphemeKind::U8 }
            fn grapheme_len_bytes(&self) -> usize { self.len() }
            fn grapheme_len_utf8(&self) -> usize { self.len() }
        }
    }
    #[cfg(feature = "alloc")]
    impl Grapheme for crate::GraphemeString {
        fn grapheme_chars(&self) -> impl Iterator<Item = char> { self.chars() }
        fn grapheme_kind(&self) -> GraphemeKind { GraphemeKind::String }
        fn grapheme_len_bytes(&self) -> usize { self.len() }
        fn grapheme_len_utf8(&self) -> usize { self.len() }
    }

    /* for scalars */

    impl Grapheme for char {
        fn grapheme_chars(&self) -> impl Iterator<Item = char> { Iter::once(*self) }
        fn grapheme_kind(&self) -> GraphemeKind { GraphemeKind::char }
        fn grapheme_len_bytes(&self) -> usize { Char(*self as u32).len_bytes() }
        fn grapheme_len_utf8(&self) -> usize { Char(*self).len_utf8() }
        fn grapheme_len_chars(&self) -> usize { 1 }
    }
    impl Grapheme for char7 {
        fn grapheme_chars(&self) -> impl Iterator<Item = char> { Iter::once(self.to_char()) }
        fn grapheme_kind(&self) -> GraphemeKind { GraphemeKind::char7 }
        fn grapheme_len_bytes(&self) -> usize { Char(self.to_scalar()).len_bytes() }
        fn grapheme_len_utf8(&self) -> usize { Char(self.to_scalar()).len_utf8_unchecked() }
        fn grapheme_len_chars(&self) -> usize { 1 }
    }
    impl Grapheme for char8 {
        fn grapheme_chars(&self) -> impl Iterator<Item = char> { Iter::once(self.to_char()) }
        fn grapheme_kind(&self) -> GraphemeKind { GraphemeKind::char8 }
        fn grapheme_len_bytes(&self) -> usize { Char(self.to_scalar()).len_bytes() }
        fn grapheme_len_utf8(&self) -> usize { Char(self.to_scalar()).len_utf8_unchecked() }
        fn grapheme_len_chars(&self) -> usize { 1 }
    }
    impl Grapheme for char16 {
        fn grapheme_chars(&self) -> impl Iterator<Item = char> { Iter::once(self.to_char()) }
        fn grapheme_kind(&self) -> GraphemeKind { GraphemeKind::char16 }
        fn grapheme_len_bytes(&self) -> usize { Char(self.to_scalar()).len_bytes() }
        fn grapheme_len_utf8(&self) -> usize { Char(self.to_scalar()).len_utf8_unchecked() }
        fn grapheme_len_chars(&self) -> usize { 1 }
    }
    impl Grapheme for charu {
        fn grapheme_chars(&self) -> impl Iterator<Item = char> { Iter::once(self.to_char()) }
        fn grapheme_kind(&self) -> GraphemeKind { GraphemeKind::charu }
        fn grapheme_len_bytes(&self) -> usize { self.len_bytes() }
        fn grapheme_len_utf8(&self) -> usize { Char(self.to_scalar()).len_utf8_unchecked() }
        fn grapheme_len_chars(&self) -> usize { 1 }
    }
    impl Grapheme for charu_niche {
        fn grapheme_chars(&self) -> impl Iterator<Item = char> { Iter::once(self.to_char()) }
        fn grapheme_kind(&self) -> GraphemeKind { GraphemeKind::charu_niche }
        fn grapheme_len_bytes(&self) -> usize { self.len_bytes() }
        fn grapheme_len_utf8(&self) -> usize { Char(self.to_scalar()).len_utf8_unchecked() }
        fn grapheme_len_chars(&self) -> usize { 1 }
    }

}
