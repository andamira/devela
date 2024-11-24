// devela::text::grapheme::string
//
//!
//

use crate::Grapheme;
#[allow(unused)]
use crate::{Char, Str};
#[cfg(feature = "alloc")]
use crate::{IterChars, String};
#[cfg(feature = "dep_unicode_segmentation")]
use crate::{_dep::unicode_segmentation::UnicodeSegmentation, text::*};

/// An <abbr title="Extended Grapheme Cluster">EGC</abbr> backed by a [`String`].
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
pub struct GraphemeString(String);

impl GraphemeString {
    /// Creates a new empty extended grapheme cluster.
    #[must_use]
    pub const fn new() -> GraphemeString {
        Self(String::new())
    }

    /// Creates a new `GraphemeString` from a `char7`.
    #[must_use]
    #[cfg(feature = "_char7")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char7")))]
    #[cfg(feature = "dep_unicode_segmentation")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "dep_unicode_segmentation")))]
    pub fn from_char7(c: char7) -> GraphemeString {
        Str::from_utf8(&c.to_utf8_bytes()).unwrap().into()
    }

    /// Creates a new `GraphemeString` from a `char8`.
    #[must_use]
    #[cfg(feature = "_char8")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char8")))]
    #[cfg(feature = "dep_unicode_segmentation")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "dep_unicode_segmentation")))]
    pub fn from_char8(c: char8) -> GraphemeString {
        Str::from_utf8(&c.to_utf8_bytes()).unwrap().into()
    }

    /// Creates a new `GraphemeString` from a `char16`.
    #[must_use]
    #[cfg(feature = "_char16")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char16")))]
    #[cfg(feature = "dep_unicode_segmentation")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "dep_unicode_segmentation")))]
    pub fn from_char16(c: char16) -> GraphemeString {
        Str::from_utf8(&c.to_utf8_bytes()).unwrap().into()
    }

    /// Creates a new `GraphemeString` from a `char24`.
    #[must_use]
    #[cfg(feature = "_char24")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char24")))]
    #[cfg(feature = "dep_unicode_segmentation")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "dep_unicode_segmentation")))]
    pub fn from_char24(c: char24) -> GraphemeString {
        Str::from_utf8(&c.to_utf8_bytes()).unwrap().into()
    }

    /// Creates a new `GraphemeString` from a `char`.
    /// # Features
    /// Makes use of the `unsafe_str` feature if enabled.
    #[must_use]
    #[cfg(feature = "dep_unicode_segmentation")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "dep_unicode_segmentation")))]
    pub fn from_char(c: char) -> GraphemeString {
        #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
        return Str::from_utf8(&crate::Char::to_utf8_bytes(c)).unwrap().into();
        #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
        unsafe {
            Str::from_utf8_unchecked(&crate::Char::to_utf8_bytes(c)).into()
        }
    }

    //

    /// Returns the length in bytes.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the current length is 0.
    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    /// Sets the length to 0, by resetting all bytes to 0.
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Returns an iterator over the `chars` of this grapheme cluster.
    #[cfg(feature = "alloc")]
    pub fn chars(&self) -> IterChars {
        self.0.chars()
    }
}

/* traits */

impl Grapheme for GraphemeString {}

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

    #[cfg(feature = "dep_unicode_segmentation")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "dep_unicode_segmentation")))]
    impl From<String> for GraphemeString {
        fn from(s: String) -> GraphemeString {
            GraphemeString(s.graphemes(true).take(1).collect())
        }
    }
    #[cfg(feature = "dep_unicode_segmentation")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "dep_unicode_segmentation")))]
    impl From<&str> for GraphemeString {
        #[must_use]
        fn from(s: &str) -> GraphemeString {
            GraphemeString(s.graphemes(true).take(1).collect())
        }
    }
    impl From<char> for GraphemeString {
        fn from(s: char) -> GraphemeString {
            GraphemeString(s.into())
        }
    }
}
