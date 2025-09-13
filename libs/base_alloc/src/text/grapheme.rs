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

#[doc = crate::TAG_TEXT!()]
/// An <abbr title="Extended Grapheme Cluster">EGC</abbr> backed by a [`String`].
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct GraphemeString(String);

impl GraphemeString {
    /// Creates a new empty extended grapheme cluster.
    #[must_use]
    pub const fn new() -> GraphemeString {
        Self(String::new())
    }

    // /// Creates a new `GraphemeString` from a `char7`.
    // #[must_use]
    // #[cfg(feature = "dep_unicode_segmentation")]
    // #[cfg_attr(nightly_doc, doc(cfg(feature = "dep_unicode_segmentation")))]
    // pub fn from_char7(c: char7) -> GraphemeString {
    //     from_utf8(&c.to_utf8_bytes()).unwrap().into()
    // }
    //
    // /// Creates a new `GraphemeString` from a `char8`.
    // #[must_use]
    // #[cfg(feature = "dep_unicode_segmentation")]
    // #[cfg_attr(nightly_doc, doc(cfg(feature = "dep_unicode_segmentation")))]
    // pub fn from_char8(c: char8) -> GraphemeString {
    //     from_utf8(&c.to_utf8_bytes()).unwrap().into()
    // }
    //
    // /// Creates a new `GraphemeString` from a `char16`.
    // #[must_use]
    // #[cfg(feature = "dep_unicode_segmentation")]
    // #[cfg_attr(nightly_doc, doc(cfg(feature = "dep_unicode_segmentation")))]
    // pub fn from_char16(c: char16) -> GraphemeString {
    //     from_utf8(&c.to_utf8_bytes()).unwrap().into()
    // }
    //
    // /// Creates a new `GraphemeString` from a `char`.
    // /// # Features
    // /// Makes use of the `unsafe_str` feature if enabled.
    // #[must_use]
    // #[cfg(feature = "dep_unicode_segmentation")]
    // #[cfg_attr(nightly_doc, doc(cfg(feature = "dep_unicode_segmentation")))]
    // pub fn from_char(c: char) -> GraphemeString {
    //     #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
    //     return from_utf8(&crate::Char::to_utf8_bytes(c)).unwrap().into();
    //     #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
    //     unsafe {
    //         from_utf8_unchecked(&crate::Char::to_utf8_bytes(c)).into()
    //     }
    // }

    //

    /// Returns the length in bytes.
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the current length is 0.
    pub const fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    /// Sets the length to 0, by resetting all bytes to 0.
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Returns an iterator over the `chars` of this grapheme cluster.
    pub fn chars(&self) -> IterChars<'_> {
        self.0.chars()
    }
}

/* traits */

mod core_impls {
    use super::*;
    use core::fmt;

    impl Default for GraphemeString {
        /// Returns a new empty extended grapheme cluster.
        fn default() -> Self {
            Self::new()
        }
    }

    impl fmt::Display for GraphemeString {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl fmt::Debug for GraphemeString {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self.0)
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
    impl From<char> for GraphemeString {
        fn from(s: char) -> GraphemeString {
            GraphemeString(s.into())
        }
    }
}
