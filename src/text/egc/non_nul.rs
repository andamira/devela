// devela::text::egc::non_nul
//
//!
//
// TOC
// - definitions
// - trait impls
// - conversions

use super::Egc;
use crate::text::{
    char::*,
    {helpers::impl_sized_alias, ArrayU8NonNulString},
};
#[cfg(feature = "alloc")]
use _alloc::{ffi::CString, str::Chars};
// use unicode_segmentation::UnicodeSegmentation;

/* definitions */

/// An <abbr title="Extended Grapheme Cluster">EGC</abbr> backed by an
/// [`ArrayU8NonNulString`].
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(transparent)]
pub struct ArrayU8NonNulEgc<const CAP: usize>(ArrayU8NonNulString<CAP>);

impl_sized_alias![
    NonNulEgc, ArrayU8NonNulEgc,
    "<abbr title='Extended Grapheme Cluster'>EGC</abbr>, backed by an array of ",
    ". Can't contain nul chars.":
    "An" 8, 1 "";
    "A" 16, 2 "s";
    "A" 32, 4 "s";
    "A" 64, 8 "s";
    "A" 128, 16 "s"
];

/* impls */

impl<const CAP: usize> ArrayU8NonNulEgc<CAP> {
    /// Creates a new empty `ArrayU8NonNulString`.
    #[inline]
    pub const fn new() -> Self {
        Self(ArrayU8NonNulString::new())
    }

    /// Creates a new `ArrayU8NonNulEgc` from a `Char7`.
    ///
    /// If `c`.[`is_nul()`][Char7#method.is_nul] an empty egc will be returned.
    ///
    /// # Panics
    /// Panics if `!c.is_nul()` and `CAP` < 1.
    ///
    /// Will never panic if `CAP` >= 1.
    #[inline]
    #[must_use]
    pub const fn from_char7(c: Char7) -> Self {
        Self(ArrayU8NonNulString::from_char7(c))
    }

    /// Creates a new `ArrayU8NonNulEgc` from a `Char8`.
    ///
    /// If `c`.[`is_nul()`][Char8#method.is_nul] an empty egc will be returned.
    ///
    /// # Panics
    /// Panics if `!c.is_nul()` and `CAP` < `c.`[`len_utf8()`][Chars#method.len_utf8].
    ///
    /// Will never panic if `CAP` >= 2.
    #[inline]
    #[must_use]
    pub const fn from_char8(c: Char8) -> Self {
        Self(ArrayU8NonNulString::from_char8(c))
    }

    /// Creates a new `ArrayU8NonNulEgc` from a `Char16`.
    ///
    /// If `c`.[`is_nul()`][Char16#method.is_nul] an empty egc will be returned.
    ///
    /// # Panics
    /// Panics if `!c.is_nul()` and `CAP` < `c.`[`len_utf8()`][Chars#method.len_utf8].
    ///
    /// Will never panic if `CAP` >= 3
    #[inline]
    #[must_use]
    pub const fn from_char16(c: Char16) -> Self {
        Self(ArrayU8NonNulString::from_char16(c))
    }

    /// Creates a new `ArrayU8NonNulEgc` from a `Char24`.
    ///
    /// If `c`.[`is_nul()`][Char24#method.is_nul] an empty egc will be returned.
    ///
    /// # Panics
    /// Panics if `!c.is_nul()` and `CAP` < `c.`[`len_utf8()`][Chars#method.len_utf8].
    ///
    /// Will never panic if `CAP` >= 4.
    #[inline]
    #[must_use]
    pub const fn from_char24(c: Char24) -> Self {
        Self(ArrayU8NonNulString::from_char24(c))
    }

    /// Creates a new `ArrayU8NonNulEgc` from a `Char32`.
    ///
    /// If `c`.[`is_nul()`][Char32#method.is_nul] an empty egc will be returned.
    ///
    /// # Panics
    /// Panics if `!c.is_nul()` and `CAP` < `c.`[`len_utf8()`][Chars#method.len_utf8].
    ///
    /// Will never panic if `CAP` >= 4.
    #[inline]
    #[must_use]
    pub const fn from_char32(c: Char32) -> Self {
        Self(ArrayU8NonNulString::from_char32(c))
    }

    /// Creates a new `ArrayU8NonNulEgc` from a `char`.
    ///
    /// If `c`.[`is_nul()`][Chars#method.is_nul] an empty egc will be returned.
    ///
    /// # Panics
    /// Panics if `!c.is_nul()` and `CAP` < `c.`[`len_utf8()`][Chars#method.len_utf8].
    ///
    /// Will never panic if `CAP` >= 4.
    #[inline]
    #[must_use]
    pub const fn from_char(c: char) -> Self {
        Self(ArrayU8NonNulString::from_char(c))
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

    /// Returns the total capacity in bytes.
    #[inline]
    pub const fn capacity() -> usize {
        CAP
    }

    /// Returns the remaining capacity.
    #[inline]
    pub fn remaining_capacity(&self) -> usize {
        CAP - self.len()
    }

    /// Returns `true` if the current remaining capacity is 0.
    #[inline]
    pub fn is_full(&self) -> bool {
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
    ///
    /// # Safety
    /// TODO
    #[inline]
    #[cfg(feature = "unsafe")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        self.0.as_bytes_mut()
    }

    /// Returns a copy of the inner array with the full contents.
    ///
    /// The array contains all the bytes, including those outside the current length.
    #[inline]
    pub fn as_array(&self) -> [u8; CAP] {
        self.0.as_array()
    }

    /// Returns the inner array with the full contents.
    ///
    /// The array contains all the bytes, including those outside the current length.
    #[inline]
    pub fn into_array(self) -> [u8; CAP] {
        self.0.into_array()
    }

    /// Returns the inner string slice.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    /// Returns the mutable inner string slice.
    ///
    /// # Safety
    /// TODO
    #[cfg(feature = "unsafe")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "unsafe")))]
    pub unsafe fn as_str_mut(&mut self) -> &mut str {
        self.0.as_str_mut()
    }

    /// Returns an iterator over the `chars` of this grapheme cluster.
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
    pub fn chars(&self) -> Chars {
        self.0.chars()
    }

    /// Returns a new allocated C-compatible, nul-terminanted string.
    #[inline]
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
    pub fn to_cstring(&self) -> CString {
        self.0.to_cstring()
    }

    //
}

/* traits */

impl<const CAP: usize> Egc for ArrayU8NonNulEgc<CAP> {}

mod core_impls {
    use super::*;
    use core::fmt;

    impl<const CAP: usize> Default for ArrayU8NonNulEgc<CAP> {
        /// Returns an empty extended grapheme character.
        #[inline]
        fn default() -> Self {
            Self::new()
        }
    }

    impl<const CAP: usize> fmt::Display for ArrayU8NonNulEgc<CAP> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
    impl<const CAP: usize> fmt::Debug for ArrayU8NonNulEgc<CAP> {
        #[inline]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }

    // TODO
    // impl<const CAP: usize> From<String> for ArrayU8NonNulEgc<CAP> {
    //     fn from(s: String) -> ArrayU8NonNulEgc<CAP> {
    //         ArrayU8NonNulEgc(s.graphemes(true).take(1).collect())
    //     }
    // }
    // impl From<&str> for ArrayU8NonNulEgc {
    //     fn from(s: &str) -> ArrayU8NonNulEgc {
    //         ArrayU8NonNulEgc(s.graphemes(true).take(1).collect())
    //     }
    // }
    // impl From<char> for ArrayU8NonNulEgc {
    //     fn from(s: char) -> ArrayU8NonNulEgc {
    //         ArrayU8NonNulEgc(s.into())
    //     }
    // }
}
