// devela::text::egc::string
//
//!
//

use super::Egc;
#[allow(unused_imports)]
use crate::_deps::alloc::{
    str::{self, Chars as CharIterator},
    string::String,
};
#[cfg(feature = "unicode-segmentation")]
use crate::_deps::unicode_segmentation::UnicodeSegmentation;
#[cfg(feature = "unicode-segmentation")]
use crate::text::{Char16, Char24, Char32, Char7, Char8};

/// An <abbr title="Extended Grapheme Cluster">EGC</abbr> backed by a [`String`].
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
pub struct StringEgc(String);

impl StringEgc {
    /// Creates a new empty extended grapheme cluster.
    #[inline]
    #[must_use]
    pub const fn new() -> StringEgc {
        Self(String::new())
    }

    /// Creates a new `StringEgc` from a `Char7`.
    #[inline]
    #[must_use]
    #[cfg(feature = "unicode-segmentation")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unicode-segmentation")))]
    pub fn from_char7(c: Char7) -> StringEgc {
        str::from_utf8(&c.to_utf8_bytes()).unwrap().into()
    }

    /// Creates a new `StringEgc` from a `Char8`.
    #[inline]
    #[must_use]
    #[cfg(feature = "unicode-segmentation")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unicode-segmentation")))]
    pub fn from_char8(c: Char8) -> StringEgc {
        str::from_utf8(&c.to_utf8_bytes()).unwrap().into()
    }

    /// Creates a new `StringEgc` from a `Char16`.
    #[inline]
    #[must_use]
    #[cfg(feature = "unicode-segmentation")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unicode-segmentation")))]
    pub fn from_char16(c: Char16) -> StringEgc {
        str::from_utf8(&c.to_utf8_bytes()).unwrap().into()
    }

    /// Creates a new `StringEgc` from a `Char24`.
    #[inline]
    #[must_use]
    #[cfg(feature = "unicode-segmentation")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unicode-segmentation")))]
    pub fn from_char24(c: Char24) -> StringEgc {
        str::from_utf8(&c.to_utf8_bytes()).unwrap().into()
    }

    /// Creates a new `StringEgc` from a `Char32`.
    /// # Features
    /// Makes use of the `unsafe_str` feature if enabled.
    #[inline]
    #[must_use]
    #[cfg(feature = "unicode-segmentation")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unicode-segmentation")))]
    pub fn from_char32(c: Char32) -> StringEgc {
        #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
        return str::from_utf8(&c.to_utf8_bytes()).unwrap().into();
        #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
        unsafe {
            str::from_utf8_unchecked(&c.to_utf8_bytes()).into()
        }
    }

    /// Creates a new `StringEgc` from a `char`.
    /// # Features
    /// Makes use of the `unsafe_str` feature if enabled.
    #[inline]
    #[must_use]
    #[cfg(feature = "unicode-segmentation")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unicode-segmentation")))]
    pub fn from_char(c: char) -> StringEgc {
        #[cfg(any(feature = "safe_text", not(feature = "unsafe_str")))]
        return str::from_utf8(&crate::text::char_to_utf8_bytes(c))
            .unwrap()
            .into();
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

impl Egc for StringEgc {}

mod core_impls {
    use super::*;
    use core::fmt;

    impl Default for StringEgc {
        /// Returns a new empty extended grapheme cluster.
        #[inline]
        fn default() -> Self {
            Self::new()
        }
    }

    impl fmt::Display for StringEgc {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl fmt::Debug for StringEgc {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }

    #[cfg(feature = "unicode-segmentation")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unicode-segmentation")))]
    impl From<String> for StringEgc {
        #[inline]
        fn from(s: String) -> StringEgc {
            StringEgc(s.graphemes(true).take(1).collect())
        }
    }
    #[cfg(feature = "unicode-segmentation")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unicode-segmentation")))]
    impl From<&str> for StringEgc {
        #[inline]
        #[must_use]
        fn from(s: &str) -> StringEgc {
            StringEgc(s.graphemes(true).take(1).collect())
        }
    }
    impl From<char> for StringEgc {
        #[inline]
        fn from(s: char) -> StringEgc {
            StringEgc(s.into())
        }
    }
}
