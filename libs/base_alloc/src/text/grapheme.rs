// devela_base_alloc::text::grapheme
//
//! Defines [`GraphemeString`].
//

// #[cfg(feature = "dep_unicode_segmentation")]
// use crate::{_dep::unicode_segmentation::UnicodeSegmentation, text::*};
use crate::{IterChars, String};
// crate::_use! {basic::from_utf8}

// #[allow(unused, reason = "feature-gated")]
// use {crate::Char, ::core::str::from_utf8_unchecked};

#[doc = crate::_TAG_TEXT!()]
/// An <abbr title="Extended Grapheme Cluster">EGC</abbr> backed by a [`String`].
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct GraphemeString(String);

#[rustfmt::skip]
impl GraphemeString {
    /// Creates a new empty extended grapheme cluster.
    #[must_use]
    pub const fn new() -> GraphemeString { Self(String::new()) }

    // /// Creates a new `GraphemeString` from a `char7`.
    // #[must_use]
    // pub fn from_char7(c: char7) -> GraphemeString {
    //     from_utf8(&c.to_utf8_bytes()).unwrap().into()
    // }
    //
    // /// Creates a new `GraphemeString` from a `char8`.
    // #[must_use]
    // pub fn from_char8(c: char8) -> GraphemeString {
    //     from_utf8(&c.to_utf8_bytes()).unwrap().into()
    // }
    //
    // /// Creates a new `GraphemeString` from a `char16`.
    // #[must_use]
    // pub fn from_char16(c: char16) -> GraphemeString {
    //     from_utf8(&c.to_utf8_bytes()).unwrap().into()
    // }
    //
    // /// Creates a new `GraphemeString` from a `char`.
    // /// # Features
    // /// Makes use of the `unsafe_str` feature if enabled.
    // #[must_use]
    // pub fn from_char(c: char) -> GraphemeString {
    //     #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
    //     return from_utf8(&crate::Char(c).to_utf8_bytes()).unwrap().into();
    //     #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
    //     unsafe {
    //         from_utf8_unchecked(&crate::Char(c).to_utf8_bytes()).into()
    //     }
    // }

    //

    /// Returns the length in bytes.
    #[must_use]
    #[inline(always)]
    pub const fn len(&self) -> usize { self.0.len() }

    /// Returns `true` if the current length is 0.
    #[must_use]
    #[inline(always)]
    pub const fn is_empty(&self) -> bool { self.0.len() == 0 }

    /// Sets the length to 0, by resetting all bytes to 0.
    #[inline(always)]
    pub fn clear(&mut self) { self.0.clear(); }

    /// Returns an iterator over the `chars` of this grapheme cluster.
    pub const fn chars(&self) -> IterChars<'_, &str> { IterChars::<&str>::new(self.0.as_str()) }
}

/* traits */

#[rustfmt::skip]
mod core_impls {
    use super::*;
    use crate::{Formatter, FmtResult, Debug, Display};

    impl Default for GraphemeString {
        /// Returns a new empty extended grapheme cluster.
        fn default() -> Self { Self::new() }
    }

    impl Display for GraphemeString {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> { write!(f, "{}", self.0) }
    }

    impl Debug for GraphemeString {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> { write!(f, "{:?}", self.0) }
    }

    /* conversions */

    impl From<char> for GraphemeString {
        fn from(s: char) -> GraphemeString {
            GraphemeString(s.into())
        }
    }

    // #[cfg(feature = "dep_unicode_segmentation")]
    // #[cfg_attr(nightly_doc, doc(cfg(feature = "dep_unicode_segmentation")))]
    // impl From<String> for GraphemeString {
    //     fn from(s: String) -> GraphemeString {
    //         GraphemeString(s.graphemes(true).take(1).collect())
    //     }
    // }
    // #[cfg(feature = "dep_unicode_segmentation")]
    // #[cfg_attr(nightly_doc, doc(cfg(feature = "dep_unicode_segmentation")))]
    // impl From<&str> for GraphemeString {
    //     fn from(s: &str) -> GraphemeString {
    //         GraphemeString(s.graphemes(true).take(1).collect())
    //     }
    // }
}
