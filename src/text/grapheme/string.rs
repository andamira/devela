// devela::text::grapheme::string
//
//!
//

use super::Grapheme;
#[allow(unused_imports)]
use crate::_dep::_alloc::{
    str::{self, Chars as CharIterator},
    string::String,
};
#[cfg(feature = "unicode-segmentation")]
use crate::{_dep::unicode_segmentation::UnicodeSegmentation, text::*};

/// An <abbr title="Extended Grapheme Cluster">EGC</abbr> backed by a [`String`].
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
pub struct GraphemeString(String);

impl GraphemeString {
    /// Creates a new empty extended grapheme cluster.
    #[inline]
    #[must_use]
    pub const fn new() -> GraphemeString {
        Self(String::new())
    }

    /// Creates a new `GraphemeString` from a `Char7`.
    #[inline]
    #[must_use]
    #[cfg(feature = "_char7")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_7")))]
    #[cfg(feature = "unicode-segmentation")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unicode-segmentation")))]
    pub fn from_char7(c: Char7) -> GraphemeString {
        str::from_utf8(&c.to_utf8_bytes()).unwrap().into()
    }

    /// Creates a new `GraphemeString` from a `Char8`.
    #[inline]
    #[must_use]
    #[cfg(feature = "_char8")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_8")))]
    #[cfg(feature = "unicode-segmentation")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unicode-segmentation")))]
    pub fn from_char8(c: Char8) -> GraphemeString {
        str::from_utf8(&c.to_utf8_bytes()).unwrap().into()
    }

    /// Creates a new `GraphemeString` from a `Char16`.
    #[inline]
    #[must_use]
    #[cfg(feature = "_char16")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_16")))]
    #[cfg(feature = "unicode-segmentation")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unicode-segmentation")))]
    pub fn from_char16(c: Char16) -> GraphemeString {
        str::from_utf8(&c.to_utf8_bytes()).unwrap().into()
    }

    /// Creates a new `GraphemeString` from a `Char24`.
    #[inline]
    #[must_use]
    #[cfg(feature = "_char24")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_24")))]
    #[cfg(feature = "unicode-segmentation")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unicode-segmentation")))]
    pub fn from_char24(c: Char24) -> GraphemeString {
        str::from_utf8(&c.to_utf8_bytes()).unwrap().into()
    }

    /// Creates a new `GraphemeString` from a `Char32`.
    /// # Features
    /// Makes use of the `unsafe_str` feature if enabled.
    #[inline]
    #[must_use]
    #[cfg(feature = "_char32")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_32")))]
    #[cfg(feature = "unicode-segmentation")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unicode-segmentation")))]
    pub fn from_char32(c: Char32) -> GraphemeString {
        #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
        return str::from_utf8(&c.to_utf8_bytes()).unwrap().into();
        #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
        unsafe {
            str::from_utf8_unchecked(&c.to_utf8_bytes()).into()
        }
    }

    /// Creates a new `GraphemeString` from a `char`.
    /// # Features
    /// Makes use of the `unsafe_str` feature if enabled.
    #[inline]
    #[must_use]
    #[cfg(feature = "unicode-segmentation")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unicode-segmentation")))]
    pub fn from_char(c: char) -> GraphemeString {
        #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
        return str::from_utf8(&crate::text::char_to_utf8_bytes(c)).unwrap().into();
        #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
        unsafe {
            str::from_utf8_unchecked(&crate::text::char_to_utf8_bytes(c)).into()
        }
    }

    //

    /// Returns the length in bytes.
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the current length is 0.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    /// Sets the length to 0, by resetting all bytes to 0.
    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Returns an iterator over the `chars` of this grapheme cluster.
    #[inline]
    #[cfg(feature = "alloc")]
    pub fn chars(&self) -> CharIterator {
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
        #[inline]
        fn default() -> Self {
            Self::new()
        }
    }

    impl fmt::Display for GraphemeString {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl fmt::Debug for GraphemeString {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }

    #[cfg(feature = "unicode-segmentation")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unicode-segmentation")))]
    impl From<String> for GraphemeString {
        #[inline]
        fn from(s: String) -> GraphemeString {
            GraphemeString(s.graphemes(true).take(1).collect())
        }
    }
    #[cfg(feature = "unicode-segmentation")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unicode-segmentation")))]
    impl From<&str> for GraphemeString {
        #[inline]
        #[must_use]
        fn from(s: &str) -> GraphemeString {
            GraphemeString(s.graphemes(true).take(1).collect())
        }
    }
    impl From<char> for GraphemeString {
        #[inline]
        fn from(s: char) -> GraphemeString {
            GraphemeString(s.into())
        }
    }
}
