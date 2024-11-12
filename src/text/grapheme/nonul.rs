// devela::text::grapheme::nonul
//
//!
//
// TOC
// - definitions
// - trait impls
// - conversions

use super::Grapheme;
#[cfg(_char_·)]
use crate::text::char::*;
#[cfg(feature = "alloc")]
use crate::text::CString;
#[cfg(doc)]
use crate::text::TextError::{NotEnoughCapacity, OutOfBounds};
use crate::{
    code::ConstDefault,
    error::unwrap,
    text::{helpers::impl_sized_alias, StringNonul, TextResult as Result},
};
use core::str::Chars;
// use unicode_segmentation::UnicodeSegmentation;

/* definitions */

/// An <abbr title="Extended Grapheme Cluster">EGC</abbr> backed by a [`StringNonul`].
#[must_use]
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GraphemeNonul<const CAP: usize>(StringNonul<CAP>);

impl_sized_alias![
    GraphemeNonul, GraphemeNonul,
    "<abbr title='Extended Grapheme Cluster'>EGC</abbr>, backed by an array of ",
    ". Can't contain nul chars.":
    "An" 8, 1 "";
    "A" 16, 2 "s";
    "A" 32, 4 "s";
    "A" 64, 8 "s";
    "A" 128, 16 "s"
];

/* impls */

impl<const CAP: usize> GraphemeNonul<CAP> {
    /// Creates a new empty `StringNonul`.
    ///
    /// # Errors
    /// Returns [`OutOfBounds`] if `CAP` > 255.
    #[inline]
    pub const fn new() -> Result<Self> {
        Ok(Self(unwrap![ok? StringNonul::new()]))
    }

    /// Creates a new `GraphemeNonul` from a `CharU7`.
    ///
    /// If `c`.[`is_nul()`][CharU7#method.is_nul] an empty grapheme will be returned.
    ///
    /// # Errors
    /// Returns [`OutOfBounds`] if `CAP` > 255,
    /// or [`NotEnoughCapacity`] if `!c.is_nul()` and `CAP` < 1.
    ///
    /// Will always succeed if `CAP` >= 1.
    #[inline]
    #[cfg(feature = "_char_u7")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_u7")))]
    pub const fn from_char_u7(c: CharU7) -> Result<Self> {
        Ok(Self(unwrap![ok? StringNonul::from_char_u7(c)]))
    }

    /// Creates a new `GraphemeNonul` from a `CharU8`.
    ///
    /// If `c`.[`is_nul()`][CharU8#method.is_nul] an empty grapheme will be returned.
    ///
    /// # Errors
    /// Returns [`OutOfBounds`] if `CAP` > 255,
    /// or [`NotEnoughCapacity`] if `!c.is_nul()`
    /// and `CAP` < `c.`[`len_utf8()`][CharU8#method.len_utf8].
    ///
    /// Will always succeed if `CAP` >= 2.
    #[inline]
    #[cfg(feature = "_char_u8")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_u8")))]
    pub const fn from_char_u8(c: CharU8) -> Result<Self> {
        Ok(Self(unwrap![ok? StringNonul::from_char_u8(c)]))
    }

    /// Creates a new `GraphemeNonul` from a `CharU16`.
    ///
    /// If `c`.[`is_nul()`][CharU16#method.is_nul] an empty grapheme will be returned.
    ///
    /// # Errors
    /// Returns [`OutOfBounds`] if `CAP` > 255,
    /// or [`NotEnoughCapacity`] if `!c.is_nul()`
    /// and `CAP` < `c.`[`len_utf8()`][CharU16#method.len_utf8].
    ///
    /// Will always succeed if `CAP` >= 3.
    #[inline]
    #[cfg(feature = "_char_u16")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_u16")))]
    pub const fn from_char_u16(c: CharU16) -> Result<Self> {
        Ok(Self(unwrap![ok? StringNonul::from_char_u16(c)]))
    }

    /// Creates a new `GraphemeNonul` from a `CharU24`.
    ///
    /// If `c`.[`is_nul()`][CharU24#method.is_nul] an empty grapheme will be returned.
    ///
    /// # Errors
    /// Returns [`OutOfBounds`] if `CAP` > 255,
    /// or [`NotEnoughCapacity`] if `!c.is_nul()`
    /// and `CAP` < `c.`[`len_utf8()`][CharU24#method.len_utf8].
    ///
    /// Will always succeed if `CAP` >= 4.
    #[inline]
    #[cfg(feature = "_char_u24")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_u24")))]
    pub const fn from_char_u24(c: CharU24) -> Result<Self> {
        Ok(Self(unwrap![ok? StringNonul::from_char_u24(c)]))
    }

    /// Creates a new `GraphemeNonul` from a `CharU32`.
    ///
    /// If `c`.[`is_nul()`][CharU32#method.is_nul] an empty grapheme will be returned.
    ///
    /// # Errors
    /// Returns [`OutOfBounds`] if `CAP` > 255,
    /// or [`NotEnoughCapacity`] if `!c.is_nul()`
    /// and `CAP` < `c.`[`len_utf8()`][CharU32#method.len_utf8].
    ///
    /// Will always succeed if `CAP` >= 4.
    #[inline]
    #[cfg(feature = "_char_u32")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_u32")))]
    pub const fn from_char_u32(c: CharU32) -> Result<Self> {
        Ok(Self(unwrap![ok? StringNonul::from_char_u32(c)]))
    }

    /// Creates a new `GraphemeNonul` from a `char`.
    ///
    /// If `c`.[`is_nul()`][UnicodeScalar#method.is_nul] an empty grapheme will be returned.
    ///
    /// # Errors
    /// Returns [`OutOfBounds`] if `CAP` > 255,
    /// or [`NotEnoughCapacity`] if `!c.is_nul()`
    /// and `CAP` < `c.`[`len_utf8()`][UnicodeScalar#method.len_utf8].
    ///
    /// Will always succeed if `CAP` >= 4.
    #[inline]
    pub const fn from_char(c: char) -> Result<Self> {
        Ok(Self(unwrap![ok? StringNonul::from_char(c)]))
    }

    //

    /// Returns the length in bytes.
    #[inline] #[must_use] #[rustfmt::skip]
    pub const fn len(&self) -> usize { self.0.len() }

    /// Returns `true` if the current length is 0.
    #[inline] #[must_use] #[rustfmt::skip]
    pub const fn is_empty(&self) -> bool { self.0.len() == 0 }

    /// Returns the total capacity in bytes.
    #[inline] #[must_use] #[rustfmt::skip]
    pub const fn capacity() -> usize { CAP }

    /// Returns the remaining capacity.
    #[inline] #[must_use] #[rustfmt::skip]
    pub const fn remaining_capacity(&self) -> usize { CAP - self.len() }

    /// Returns `true` if the current remaining capacity is 0.
    #[inline] #[must_use] #[rustfmt::skip]
    pub const fn is_full(&self) -> bool { self.len() == CAP }

    /// Sets the length to 0, by resetting all bytes to 0.
    #[inline] #[rustfmt::skip]
    pub fn clear(&mut self) { self.0.clear(); }

    //

    /// Returns a byte slice of the inner string slice.
    #[inline] #[must_use] #[rustfmt::skip]
    pub const fn as_bytes(&self) -> &[u8] { self.0.as_bytes() }

    /// Returns a mutable byte slice of the inner string slice.
    /// # Safety
    /// The content must be valid UTF-8.
    #[inline]
    #[cfg(all(not(feature = "safe_text"), feature = "unsafe_slice"))]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_slice")))]
    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        // SAFETY: unsafe fn
        unsafe { self.0.as_bytes_mut() }
    }

    /// Returns a copy of the inner array with the full contents.
    ///
    /// The array contains all the bytes, including those outside the current length.
    #[inline] #[must_use] #[rustfmt::skip]
    pub const fn as_array(&self) -> [u8; CAP] { self.0.as_array() }

    /// Returns the inner array with the full contents.
    ///
    /// The array contains all the bytes, including those outside the current length.
    #[inline] #[must_use] #[rustfmt::skip]
    pub const fn into_array(self) -> [u8; CAP] { self.0.into_array() }

    /// Returns the inner string slice.
    #[inline] #[must_use] #[rustfmt::skip]
    pub const fn as_str(&self) -> &str { self.0.as_str() }

    /// Returns the mutable inner string slice.
    /// # Safety
    /// The content must be valid UTF-8.
    #[inline] #[must_use] #[rustfmt::skip]
    #[cfg(all(not(feature = "safe_text"), feature = "unsafe_slice"))]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_slice")))]
    pub unsafe fn as_mut_str(&mut self) -> &mut str {
        // SAFETY: caller must ensure safety
        unsafe { self.0.as_mut_str() }
    }

    /// Returns an iterator over the `chars` of this grapheme cluster.
    #[inline] #[rustfmt::skip]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
    pub fn chars(&self) -> Chars { self.0.chars() }

    /// Returns a new allocated C-compatible, nul-terminanted string.
    #[inline] #[must_use] #[rustfmt::skip]
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
    pub fn to_cstring(&self) -> CString { self.0.to_cstring() }
}

/* traits */

impl<const CAP: usize> Grapheme for GraphemeNonul<CAP> {}

mod core_impls {
    use super::*;
    use core::fmt;

    impl<const CAP: usize> Default for GraphemeNonul<CAP> {
        /// Returns an empty extended grapheme character.
        #[inline] #[rustfmt::skip]
        fn default() -> Self { Self::new().unwrap() }
    }
    impl<const CAP: usize> ConstDefault for GraphemeNonul<CAP> {
        /// Returns an empty string.
        ///
        /// # Panics
        /// Panics if `CAP > 255`.
        const DEFAULT: Self = unwrap![ok Self::new()];
    }

    impl<const CAP: usize> fmt::Display for GraphemeNonul<CAP> {
        #[inline] #[rustfmt::skip]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
    }
    impl<const CAP: usize> fmt::Debug for GraphemeNonul<CAP> {
        #[inline] #[rustfmt::skip]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:?}", self.0) }
    }

    // TODO
    // impl<const CAP: usize> From<String> for GraphemeNonul<CAP> {
    //     fn from(s: String) -> GraphemeNonul<CAP> {
    //         GraphemeNonul(s.graphemes(true).take(1).collect())
    //     }
    // }
    // impl From<&str> for GraphemeNonul {
    //     fn from(s: &str) -> GraphemeNonul {
    //         GraphemeNonul(s.graphemes(true).take(1).collect())
    //     }
    // }
    // impl From<char> for GraphemeNonul {
    //     fn from(s: char) -> GraphemeNonul {
    //         GraphemeNonul(s.into())
    //     }
    // }
}
