// devela::text::grapheme::string_u8
//
//!
//
// TOC
// - definitions
// - trait impls

use super::Grapheme;
#[cfg(feature = "alloc")]
use crate::CString;
#[cfg(_char··)]
use crate::text::char::*;
use crate::{ConstDefault, IterChars, MismatchedCapacity, StringU8, unwrap};
// use unicode_segmentation::UnicodeSegmentation;

/* definitions */

/// An <abbr title="Extended Grapheme Cluster">EGC</abbr> backed by an
/// [`StringU8`].
#[derive(Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
#[must_use]
pub struct GraphemeU8<const CAP: usize>(StringU8<CAP>);

impl<const CAP: usize> GraphemeU8<CAP> {
    /// Creates a new empty `GraphemeU8`.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `CAP > 255.
    pub const fn new() -> Result<Self, MismatchedCapacity> {
        Ok(Self(unwrap![ok? StringU8::new()]))
    }

    /// Creates a new `GraphemeU8` from a `char7`.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `CAP > 255.
    ///
    /// Will always succeed if `CAP` >= 1 and <= 255.
    #[cfg(feature = "_char7")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "_char7")))]
    pub const fn from_char7(c: char7) -> Result<Self, MismatchedCapacity> {
        Ok(Self(unwrap![ok? StringU8::from_char7(c)]))
    }

    /// Creates a new `GraphemeU8` from a `char8`.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `CAP` > 255
    /// or < `c.`[`len_utf8()`][char8#method.len_utf8].
    ///
    /// Will always succeed if `CAP` >= 2 and <= 255.
    #[cfg(feature = "_char8")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "_char8")))]
    pub const fn from_char8(c: char8) -> Result<Self, MismatchedCapacity> {
        Ok(Self(unwrap![ok? StringU8::from_char8(c)]))
    }

    /// Creates a new `GraphemeU8` from a `char16`.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `CAP` > 255
    /// or < `c.`[`len_utf8()`][char16#method.len_utf8].
    ///
    /// Will always succeed if `CAP` >= 3 and <= 255.
    #[cfg(feature = "_char16")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "_char16")))]
    pub const fn from_char16(c: char16) -> Result<Self, MismatchedCapacity> {
        Ok(Self(unwrap![ok? StringU8::from_char16(c)]))
    }

    /// Creates a new `GraphemeU8` from a `char`.
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `CAP` > 255
    /// or < `c.`[`len_utf8()`][UnicodeScalar#method.len_utf8].
    ///
    /// Will never panic if `CAP` >= 4 and <= 255.
    pub const fn from_char(c: char) -> Result<Self, MismatchedCapacity> {
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
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_slice")))]
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
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_slice")))]
    pub unsafe fn as_mut_str(&mut self) -> &mut str {
        self.0.as_mut_str()
    }

    /// Returns an iterator over the `chars` of this grapheme cluster.
    #[rustfmt::skip]
    pub fn chars(&self) -> IterChars<'_> { self.0.chars() }

    /// Returns a new allocated C-compatible, nul-terminanted string.
    #[rustfmt::skip]
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
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
