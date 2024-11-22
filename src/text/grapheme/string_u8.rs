// devela::text::grapheme::string_u8
//
//!
//
// TOC
// - definitions
// - trait impls

use super::Grapheme;
#[cfg(_char_·)]
use crate::text::char::*;
#[cfg(feature = "alloc")]
use crate::CString;
#[cfg(doc)]
use crate::TextError::OutOfBounds;
use crate::{
    text::helpers::impl_sized_alias, unwrap, ConstDefault, IterChars, StringU8,
    TextResult as Result,
};
// use unicode_segmentation::UnicodeSegmentation;

/* definitions */

/// An <abbr title="Extended Grapheme Cluster">EGC</abbr> backed by an
/// [`StringU8`].
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[must_use]
pub struct GraphemeU8<const CAP: usize>(StringU8<CAP>);

impl_sized_alias![
    Grapheme, GraphemeU8,
    "<abbr title='Extended Grapheme Cluster'>EGC</abbr>, backed by an array of ",
    ".":
    "A" 16, 1 "";
    "A" 32, 3 "s";
    "A" 64, 7 "s";
    "A" 128, 15 "s"
];

impl<const CAP: usize> GraphemeU8<CAP> {
    /// Creates a new empty `GraphemeU8`.
    ///
    /// # Errors
    /// Returns [`OutOfBounds`] if `CAP > 255.
    pub const fn new() -> Result<Self> {
        Ok(Self(unwrap![ok? StringU8::new()]))
    }

    /// Creates a new `GraphemeU8` from a `CharU7`.
    ///
    /// # Errors
    /// Returns [`OutOfBounds`] if `CAP > 255.
    ///
    /// Will always succeed if `CAP` >= 1 and <= 255.
    #[cfg(feature = "_char_u7")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_7")))]
    pub const fn from_char_u7(c: CharU7) -> Result<Self> {
        Ok(Self(unwrap![ok? StringU8::from_char_u7(c)]))
    }

    /// Creates a new `GraphemeU8` from a `CharU8`.
    ///
    /// # Errors
    /// Returns [`OutOfBounds`] if `CAP` > 255 or < `c.`[`len_utf8()`][CharU8#method.len_utf8].
    ///
    /// Will always succeed if `CAP` >= 2 and <= 255.
    #[cfg(feature = "_char_u8")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_8")))]
    pub const fn from_char_u8(c: CharU8) -> Result<Self> {
        Ok(Self(unwrap![ok? StringU8::from_char_u8(c)]))
    }

    /// Creates a new `GraphemeU8` from a `CharU16`.
    ///
    /// # Errors
    /// Returns [`OutOfBounds`] if `CAP` > 255 or < `c.`[`len_utf8()`][CharU16#method.len_utf8].
    ///
    /// Will always succeed if `CAP` >= 3 and <= 255.
    #[cfg(feature = "_char_u16")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_16")))]
    pub const fn from_char_u16(c: CharU16) -> Result<Self> {
        Ok(Self(unwrap![ok? StringU8::from_char_u16(c)]))
    }

    /// Creates a new `GraphemeU8` from a `CharU24`.
    ///
    /// # Errors
    /// Returns [`OutOfBounds`] if `CAP` > 255 or < `c.`[`len_utf8()`][CharU24#method.len_utf8].
    ///
    /// Will always succeed if `CAP` >= 4 and <= 255.
    #[cfg(feature = "_char_u24")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_24")))]
    pub const fn from_char_u24(c: CharU24) -> Result<Self> {
        Ok(Self(unwrap![ok? StringU8::from_char_u24(c)]))
    }

    /// Creates a new `GraphemeU8` from a `CharU32`.
    ///
    /// # Errors
    /// Returns [`OutOfBounds`] if `CAP` > 255 or < `c.`[`len_utf8()`][CharU32#method.len_utf8].
    ///
    /// Will always succeed if `CAP` >= 4 and <= 255.
    #[cfg(feature = "_char_u32")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "_char_32")))]
    pub const fn from_char_u32(c: CharU32) -> Result<Self> {
        Ok(Self(unwrap![ok? StringU8::from_char_u32(c)]))
    }

    /// Creates a new `GraphemeU8` from a `char`.
    /// # Panics
    /// Panics if `CAP` > 255 or < `c.`[`len_utf8()`][UnicodeScalar#method.len_utf8].
    ///
    /// Will never panic if `CAP` >= 4 and <= 255.
    pub const fn from_char(c: char) -> Result<Self> {
        Ok(Self(unwrap![ok? StringU8::from_char(c)]))
    }

    //

    /// Returns the length in bytes.
    #[must_use] #[rustfmt::skip]
    pub const fn len(&self) -> usize { self.0.len() }

    /// Returns `true` if the current length is 0.
    #[must_use] #[rustfmt::skip]
    pub const fn is_empty(&self) -> bool { self.0.len() == 0 }

    /// Returns the total capacity in bytes.
    #[must_use] #[rustfmt::skip]
    pub const fn capacity() -> usize { CAP }

    /// Returns the remaining capacity.
    #[must_use] #[rustfmt::skip]
    pub const fn remaining_capacity(&self) -> usize { CAP - self.len() }

    /// Returns `true` if the current remaining capacity is 0.
    #[must_use] #[rustfmt::skip]
    pub const fn is_full(&self) -> bool { self.len() == CAP }

    /// Sets the length to 0, by resetting all bytes to 0.
    #[rustfmt::skip]
    pub fn clear(&mut self) { self.0.clear(); }

    //

    /// Returns a byte slice of the inner string slice.
    #[must_use] #[rustfmt::skip]
    pub const fn as_bytes(&self) -> &[u8] { self.0.as_bytes() }

    /// Returns a mutable byte slice of the inner string slice.
    /// # Safety
    /// The content must be valid UTF-8.
    #[must_use]
    #[cfg(all(not(feature = "safe_text"), feature = "unsafe_slice"))]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_slice")))]
    pub unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        // SAFETY: caller must ensure safety
        unsafe { self.0.as_bytes_mut() }
    }

    /// Returns a copy of the inner array with the full contents.
    ///
    /// The array contains all the bytes, including those outside the current length.
    #[must_use] #[rustfmt::skip]
    pub const fn as_array(&self) -> [u8; CAP] { self.0.as_array() }

    /// Returns the inner array with the full contents.
    ///
    /// The array contains all the bytes, including those outside the current length.
    #[must_use] #[rustfmt::skip]
    pub const fn into_array(self) -> [u8; CAP] { self.0.into_array() }

    /// Returns the inner string slice.
    #[must_use] #[rustfmt::skip]
    pub const fn as_str(&self) -> &str { self.0.as_str() }

    /// Returns the mutable inner string slice.
    /// # Safety
    /// The content must be valid UTF-8.
    #[cfg(all(not(feature = "safe_text"), feature = "unsafe_slice"))]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_slice")))]
    pub unsafe fn as_mut_str(&mut self) -> &mut str {
        self.0.as_mut_str()
    }

    /// Returns an iterator over the `chars` of this grapheme cluster.
    #[rustfmt::skip]
    pub fn chars(&self) -> IterChars { self.0.chars() }

    /// Returns a new allocated C-compatible, nul-terminanted string.
    #[rustfmt::skip]
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
    pub fn to_cstring(&self) -> CString { self.0.to_cstring() }
}

/* traits */

impl<const CAP: usize> Grapheme for GraphemeU8<CAP> {}

mod core_impls {
    use super::*;
    use core::fmt;

    impl<const CAP: usize> Default for GraphemeU8<CAP> {
        /// Returns an empty extended grapheme character.
        ///
        /// # Panics
        /// Panics if `CAP` > 255.
        #[rustfmt::skip]
        fn default() -> Self { unwrap![ok Self::new()] }
    }
    impl<const CAP: usize> ConstDefault for GraphemeU8<CAP> {
        /// Returns an empty string.
        ///
        /// # Panics
        /// Panics if `CAP > 255`.
        const DEFAULT: Self = unwrap![ok Self::new()];
    }

    impl<const CAP: usize> fmt::Display for GraphemeU8<CAP> {
        #[rustfmt::skip]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
    }
    impl<const CAP: usize> fmt::Debug for GraphemeU8<CAP> {
        #[rustfmt::skip]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:?}", self.0) }
    }

    // impl From<String> for GraphemeU8 {
    //     fn from(s: String) -> GraphemeU8 {
    //         GraphemeU8(s.graphemes(true).take(1).collect())
    //     }
    // }
    // impl From<&str> for GraphemeU8 {
    //     fn from(s: &str) -> GraphemeU8 {
    //         GraphemeU8(s.graphemes(true).take(1).collect())
    //     }
    // }
    // impl From<char> for GraphemeU8 {
    //     fn from(s: char) -> GraphemeU8 {
    //         GraphemeU8(s.into())
    //     }
    // }
}
