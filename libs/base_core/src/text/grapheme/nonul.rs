// devela_base_core::text::grapheme::nonul
//
//!
//
// TOC
// - definitions
// - trait impls
// - conversions

use crate::{CharIter, MismatchedCapacity, StringNonul, char7, char8, char16, doclink, unwrap};
// use unicode_segmentation::UnicodeSegmentation;

/* definitions */

#[doc = crate::_TAG_TEXT!()]
/// An <abbr title="Extended Grapheme Cluster">EGC</abbr> backed by a [`StringNonul`].
///
#[doc = crate::_doc!(location: "text")]
#[must_use]
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GraphemeNonul<const CAP: usize>(StringNonul<CAP>);

/* impls */

impl<const CAP: usize> GraphemeNonul<CAP> {
    /// Creates a new empty `GraphemeNonul`.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `CAP` > 255.
    pub const fn new() -> Result<Self, MismatchedCapacity> {
        Ok(Self(unwrap![ok? StringNonul::new_checked()]))
    }

    /// Creates a new `GraphemeNonul` from a `char7`.
    ///
    /// If `c`.[`is_nul()`][char7#method.is_nul] an empty grapheme will be returned.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `CAP` > 255,
    /// or if `!c.is_nul()` and `CAP` < 1.
    ///
    /// Will always succeed if `CAP` >= 1.
    pub const fn from_char7(c: char7) -> Result<Self, MismatchedCapacity> {
        Ok(Self(unwrap![ok? StringNonul::from_char7(c)]))
    }

    /// Creates a new `GraphemeNonul` from a `char8`.
    ///
    /// If `c`.[`is_nul()`][char8#method.is_nul] an empty grapheme will be returned.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `CAP` > 255,
    /// or if `!c.is_nul()` and `CAP` < `c.`[`len_utf8()`][char8#method.len_utf8].
    ///
    /// Will always succeed if `CAP` >= 2.
    pub const fn from_char8(c: char8) -> Result<Self, MismatchedCapacity> {
        Ok(Self(unwrap![ok? StringNonul::from_char8(c)]))
    }

    /// Creates a new `GraphemeNonul` from a `char16`.
    ///
    /// If `c`.[`is_nul()`][char16#method.is_nul] an empty grapheme will be returned.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `CAP` > 255,
    /// or if `!c.is_nul()` and `CAP` < `c.`[`len_utf8()`][char16#method.len_utf8].
    ///
    /// Will always succeed if `CAP` >= 3.
    pub const fn from_char16(c: char16) -> Result<Self, MismatchedCapacity> {
        Ok(Self(unwrap![ok? StringNonul::from_char16(c)]))
    }

    /// Creates a new `GraphemeNonul` from a `char`.
    ///
    /// If `c`.[`is_nul()`] an empty grapheme will be returned.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `CAP` > 255,
    /// or if `!c.is_nul()` and `CAP` < `c.`[`len_utf8()`].
    ///
    /// Will always succeed if `CAP` >= 4.
    #[doc = doclink!(devela "[`is_nul()`]" "text/trait.UnicodeScalar.html#method.is_nul")]
    #[doc = doclink!(devela "[`len_utf8()`]" "text/trait.UnicodeScalar.html#method.len_utf8")]
    pub const fn from_char(c: char) -> Result<Self, MismatchedCapacity> {
        Ok(Self(unwrap![ok? StringNonul::from_char(c)]))
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
    pub const fn clear(&mut self) { self.0.clear(); }

    //

    /// Returns a byte slice of the inner string slice.
    #[must_use] #[rustfmt::skip]
    pub const fn as_bytes(&self) -> &[u8] { self.0.as_bytes() }

    /// Returns a mutable byte slice of the inner string slice.
    pub const fn as_bytes_mut(&mut self) -> &mut [u8] {
        self.0.as_bytes_mut()
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
    #[must_use] #[rustfmt::skip]
    #[cfg(all(not(base_safe_text), feature = "unsafe_slice"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_slice")))]
    pub const unsafe fn as_mut_str(&mut self) -> &mut str {
        // SAFETY: caller must ensure safety
        unsafe { self.0.as_mut_str() }
    }

    /// Returns an iterator over the `chars` of this grapheme cluster.
    #[rustfmt::skip]
    pub const fn chars(&self) -> CharIter<'_, &str> { self.0.chars() }
}

/* traits */

mod core_impls {
    use super::*;
    use core::fmt;

    impl<const CAP: usize> Default for GraphemeNonul<CAP> {
        /// Returns an empty extended grapheme character.
        #[rustfmt::skip]
        fn default() -> Self { Self::new().unwrap() }
    }
    impl<const CAP: usize> fmt::Display for GraphemeNonul<CAP> {
        #[rustfmt::skip]
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{}", self.0) }
    }
    impl<const CAP: usize> fmt::Debug for GraphemeNonul<CAP> {
        #[rustfmt::skip]
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
