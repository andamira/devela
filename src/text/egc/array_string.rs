// devela::text::egc::u8string
//
//!
//
// TOC
// - definitions
// - trait impls

use super::Egc;
#[cfg(feature = "alloc")]
use crate::_deps::alloc::{ffi::CString, str::Chars};
use crate::text::{
    char::*,
    {helpers::impl_sized_alias, ArrayU8String},
};
// use unicode_segmentation::UnicodeSegmentation;

/* definitions */

/// An <abbr title="Extended Grapheme Cluster">EGC</abbr> backed by an
/// [`ArrayU8String`].
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct ArrayU8Egc<const CAP: usize>(ArrayU8String<CAP>);

impl_sized_alias![
    Egc, ArrayU8Egc,
    "<abbr title='Extended Grapheme Cluster'>EGC</abbr>, backed by an array of ",
    ".":
    "A" 16, 1 "";
    "A" 32, 3 "s";
    "A" 64, 7 "s";
    "A" 128, 15 "s"
];

impl<const CAP: usize> ArrayU8Egc<CAP> {
    /// Creates a new empty `ArrayU8Egc`.
    /// # Panics
    /// Panics if `CAP` > 255.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self(ArrayU8String::new())
    }

    /// Creates a new `ArrayU8Egc` from a `Char7`.
    /// # Panics
    /// Panics if `CAP` > 255 or < 1.
    ///
    /// Will never panic if `CAP` >= 1 and <= 255.
    #[inline]
    #[must_use]
    pub const fn from_char7(c: Char7) -> Self {
        Self(ArrayU8String::from_char7(c))
    }

    /// Creates a new `ArrayU8Egc` from a `Char8`.
    /// # Panics
    /// Panics if `CAP` > 255 or < `c.`[`len_utf8()`][Char8#method.len_utf8].
    ///
    /// Will never panic if `CAP` >= 2 and <= 255.
    #[inline]
    #[must_use]
    pub const fn from_char8(c: Char8) -> Self {
        Self(ArrayU8String::from_char8(c))
    }

    /// Creates a new `ArrayU8Egc` from a `Char16`.
    /// # Panics
    /// Panics if `CAP` > 255 or < `c.`[`len_utf8()`][Char16#method.len_utf8].
    ///
    /// Will never panic if `CAP` >= 3 and <= 255.
    #[inline]
    #[must_use]
    pub const fn from_char16(c: Char16) -> Self {
        Self(ArrayU8String::from_char16(c))
    }

    /// Creates a new `ArrayU8Egc` from a `Char24`.
    /// # Panics
    /// Panics if `CAP` > 255 or < `c.`[`len_utf8()`][Char24#method.len_utf8].
    ///
    /// Will never panic if `CAP` >= 4 and <= 255.
    #[inline]
    #[must_use]
    pub const fn from_char24(c: Char24) -> Self {
        Self(ArrayU8String::from_char24(c))
    }

    /// Creates a new `ArrayU8Egc` from a `Char32`.
    /// # Panics
    /// Panics if `CAP` > 255 or < `c.`[`len_utf8()`][Char32#method.len_utf8].
    ///
    /// Will never panic if `CAP` >= 4 and <= 255.
    #[inline]
    #[must_use]
    pub const fn from_char32(c: Char32) -> Self {
        Self(ArrayU8String::from_char32(c))
    }

    /// Creates a new `ArrayU8Egc` from a `char`.
    /// # Panics
    /// Panics if `CAP` > 255 or < `c.`[`len_utf8()`][Chars#method.len_utf8].
    ///
    /// Will never panic if `CAP` >= 4 and <= 255.
    #[inline]
    #[must_use]
    pub const fn from_char(c: char) -> Self {
        Self(ArrayU8String::from_char(c))
    }

    //

    /// Returns the length in bytes.
    pub const fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the current length is 0.
    pub const fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    /// Returns the total capacity in bytes.
    #[inline]
    pub const fn capacity() -> usize {
        CAP
    }

    /// Returns the remaining capacity.
    #[inline]
    pub const fn remaining_capacity(&self) -> usize {
        CAP - self.len()
    }

    /// Returns `true` if the current remaining capacity is 0.
    #[inline]
    pub const fn is_full(&self) -> bool {
        self.len() == CAP
    }

    /// Sets the length to 0, by resetting all bytes to 0.
    #[inline]
    pub fn clear(&mut self) {
        self.0.clear();
    }

    //

    /// Returns a byte slice of the inner string slice.
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }

    /// Returns a mutable byte slice of the inner string slice.
    /// # Safety
    /// TODO
    #[inline]
    #[cfg(all(not(feature = "safe_text"), feature = "unsafe_slice"))]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_slice")))]
    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        self.0.as_bytes_mut()
    }

    /// Returns a copy of the inner array with the full contents.
    ///
    /// The array contains all the bytes, including those outside the current length.
    #[inline]
    pub const fn as_array(&self) -> [u8; CAP] {
        self.0.as_array()
    }

    /// Returns the inner array with the full contents.
    ///
    /// The array contains all the bytes, including those outside the current length.
    #[inline]
    pub const fn into_array(self) -> [u8; CAP] {
        self.0.into_array()
    }

    /// Returns the inner string slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Returns the mutable inner string slice.
    /// # Safety
    /// TODO
    #[cfg(all(not(feature = "safe_text"), feature = "unsafe_slice"))]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_slice")))]
    pub unsafe fn as_str_mut(&mut self) -> &mut str {
        self.0.as_str_mut()
    }

    /// Returns an iterator over the `chars` of this grapheme cluster.
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
    pub fn chars(&self) -> Chars {
        self.0.chars()
    }

    /// Returns a new allocated C-compatible, nul-terminanted string.
    #[inline]
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
    pub fn to_cstring(&self) -> CString {
        self.0.to_cstring()
    }
}

/* traits */

impl<const CAP: usize> Egc for ArrayU8Egc<CAP> {}

mod core_impls {
    use super::*;
    use core::fmt;

    impl<const CAP: usize> Default for ArrayU8Egc<CAP> {
        /// Returns an empty extended grapheme character.
        #[inline]
        fn default() -> Self {
            Self::new()
        }
    }

    impl<const CAP: usize> fmt::Display for ArrayU8Egc<CAP> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl<const CAP: usize> fmt::Debug for ArrayU8Egc<CAP> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }

    // impl From<String> for ArrayU8Egc {
    //     fn from(s: String) -> ArrayU8Egc {
    //         ArrayU8Egc(s.graphemes(true).take(1).collect())
    //     }
    // }
    // impl From<&str> for ArrayU8Egc {
    //     fn from(s: &str) -> ArrayU8Egc {
    //         ArrayU8Egc(s.graphemes(true).take(1).collect())
    //     }
    // }
    // impl From<char> for ArrayU8Egc {
    //     fn from(s: char) -> ArrayU8Egc {
    //         ArrayU8Egc(s.into())
    //     }
    // }
}
