// devela_base_core::text::grapheme::u8
//
//!
//
// TOC
// - definitions
// - trait impls

use crate::{IterChars, MismatchedCapacity, StringU8, char7, char8, char16, unwrap};
// use unicode_segmentation::UnicodeSegmentation;

/* definitions */

#[doc = crate::_TAG_TEXT!()]
/// An <abbr title="Extended Grapheme Cluster">EGC</abbr> backed by a [`StringU8`].
#[must_use]
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GraphemeU8<const CAP: usize>(StringU8<CAP>);

#[rustfmt::skip]
impl<const CAP: usize> GraphemeU8<CAP> {
    /// Creates a new empty `GraphemeU8`.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `CAP > 255.
    pub const fn new() -> Result<Self, MismatchedCapacity> {
        Ok(Self(unwrap![ok? StringU8::new_checked()]))
    }

    /// Creates a new `GraphemeU8` from a `char7`.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `CAP > 255.
    ///
    /// Will always succeed if `CAP` >= 1 and <= 255.
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
    #[must_use]
    pub const fn len(&self) -> usize { self.0.len() }

    /// Returns `true` if the current length is 0.
    #[must_use]
    pub const fn is_empty(&self) -> bool { self.0.len() == 0 }

    /// Returns the total capacity in bytes.
    #[must_use]
    pub const fn capacity() -> usize { CAP }

    /// Returns the remaining capacity.
    #[must_use]
    pub const fn remaining_capacity(&self) -> usize { CAP - self.len() }

    /// Returns `true` if the current remaining capacity is 0.
    #[must_use]
    pub const fn is_full(&self) -> bool { self.len() == CAP }

    /// Sets the length to 0, by resetting all bytes to 0.
    pub const fn clear(&mut self) { self.0.clear(); }

    //

    /// Returns a byte slice of the inner string slice.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] { self.0.as_bytes() }

    /// Returns a mutable byte slice of the inner string slice.
    #[must_use]
    pub const fn as_bytes_mut(&mut self) -> &mut [u8] {
        self.0.as_bytes_mut()
    }

    /// Returns a copy of the inner array with the full contents.
    ///
    /// The array contains all the bytes, including those outside the current length.
    #[must_use]
    pub const fn as_array(&self) -> [u8; CAP] { self.0.as_array() }

    /// Returns the inner array with the full contents.
    ///
    /// The array contains all the bytes, including those outside the current length.
    #[must_use]
    pub const fn into_array(self) -> [u8; CAP] { self.0.into_array() }

    /// Returns the inner string slice.
    #[must_use]
    pub const fn as_str(&self) -> &str { self.0.as_str() }

    /// Returns the mutable inner string slice.
    /// # Safety
    /// The content must be valid UTF-8.
    #[cfg(all(not(base_safe_text), feature = "unsafe_slice"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_slice")))]
    pub unsafe fn as_mut_str(&mut self) -> &mut str {
        self.0.as_mut_str()
    }

    /// Returns an iterator over the `chars` of this grapheme cluster.
    pub fn chars(&self) -> IterChars<'_> { self.0.chars() }
}

/* traits */

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
    impl<const CAP: usize> fmt::Display for GraphemeU8<CAP> {
        #[rustfmt::skip]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
    }
    impl<const CAP: usize> fmt::Debug for GraphemeU8<CAP> {
        #[rustfmt::skip]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:?}", self.0) }
    }
}
