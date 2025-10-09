// devela::text::grapheme::trait
//
//! Defines the [`Grapheme`] trait.
//

#[doc = crate::_TAG_TEXT!()]
/// Common trait for <abbr title="Extended Grapheme Cluster">EGC</abbr> types.
pub trait Grapheme {
    /// Returns an iterator over Unicode scalars.
    fn grapheme_chars(&self) -> impl Iterator<Item = char> + '_;

    /// Returns the length in number of bytes.
    fn grapheme_len_bytes(&self) -> usize;

    /// Returns the length in number of Unicode scalars.
    fn grapheme_len_chars(&self) -> usize {
        self.grapheme_chars().count()
    }
}

#[rustfmt::skip]
mod impls {
    use crate::{Grapheme, Iter, char, char_utf8, char7, char8, char16};

    /* for graphemes */

    #[cfg(feature = "grapheme")]
    crate::items! {
        impl<const CAP: usize> Grapheme for crate::GraphemeNonul<CAP> {
            fn grapheme_chars(&self) -> impl Iterator<Item = char> { self.chars() }
            fn grapheme_len_bytes(&self) -> usize { self.len() }
        }
        impl<const CAP: usize> Grapheme for crate::GraphemeU8<CAP> {
            fn grapheme_chars(&self) -> impl Iterator<Item = char> { self.chars() }
            fn grapheme_len_bytes(&self) -> usize { self.len() }
        }
    }
    #[cfg(feature = "alloc")]
    impl Grapheme for crate::GraphemeString {
        fn grapheme_chars(&self) -> impl Iterator<Item = char> { self.chars() }
        fn grapheme_len_bytes(&self) -> usize { self.len() }
    }

    /* for scalars */

    impl Grapheme for char {
        fn grapheme_chars(&self) -> impl Iterator<Item = char> { Iter::once(*self) }
        fn grapheme_len_bytes(&self) -> usize { 4 }
        fn grapheme_len_chars(&self) -> usize { 1 }
    }
    impl Grapheme for char7 {
        fn grapheme_chars(&self) -> impl Iterator<Item = char> { Iter::once(self.to_char()) }
        fn grapheme_len_bytes(&self) -> usize { 1 }
        fn grapheme_len_chars(&self) -> usize { 1 }
    }
    impl Grapheme for char8 {
        fn grapheme_chars(&self) -> impl Iterator<Item = char> { Iter::once(self.to_char()) }
        fn grapheme_len_bytes(&self) -> usize { 1 }
        fn grapheme_len_chars(&self) -> usize { 1 }
    }
    impl Grapheme for char16 {
        fn grapheme_chars(&self) -> impl Iterator<Item = char> { Iter::once(self.to_char()) }
        fn grapheme_len_bytes(&self) -> usize { 2 }
        fn grapheme_len_chars(&self) -> usize { 1 }
    }
    impl Grapheme for char_utf8 {
        fn grapheme_chars(&self) -> impl Iterator<Item = char> { Iter::once(self.to_char()) }
        fn grapheme_len_bytes(&self) -> usize { self.len_bytes() }
        fn grapheme_len_chars(&self) -> usize { 1 }
    }
}
