// devela/src/text/unicode/grapheme/nonul.rs
//
//!
//
// TOC
// - definitions
// - trait impls

use crate::{
    CharIter, GraphemeMachine, GraphemeScanner, MismatchedCapacity, StringNonul, char7, char8,
    char16, charu, doclink, unwrap,
};

/* definitions */

#[must_use]
#[doc = crate::_tags!(text)]
#[doc = concat!["An ", crate::_ABBR_EGC!(), " backed by a [`StringNonul`]."]]
#[doc = crate::_doc_meta!{location("text/unicode/grapheme")}]
///
/// ## Methods
///
/// - [Constructors](#constructors):
///   [`new`][Self::new],
///     *([_checked][Self::new_checked])*.
///   [`from_str`][Self::from_str],
//     *([_truncate][Self::from_str_truncate],
//       [_unchecked][Self::from_str_unchecked])*,
///   [`from_char`][Self::from_char]
///     *([7][Self::from_char7],
///       [8][Self::from_char8],
///       [16](Self::from_char16),
///       [utf8](Self::from_charu))*.
#[repr(transparent)]
#[derive(Clone, Eq, PartialOrd, Ord)]
pub struct GraphemeNonul<const CAP: usize>(pub(crate) StringNonul<CAP>);

#[rustfmt::skip]
impl<const CAP: usize> GraphemeNonul<CAP> {
    /* constructors */

    /// Creates a new empty `GraphemeNonul` with a capacity of `CAP` bytes.
    ///
    /// # Panics
    /// Panics if `CAP > 255.
    pub const fn new() -> Self {
        Self(StringNonul::new())
    }

    /// Creates a new empty `GraphemeNonul` with a capacity of `CAP` bytes.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `CAP > 255.
    pub const fn new_checked() -> Result<Self, MismatchedCapacity> {
        Ok(Self(unwrap![ok? StringNonul::new_checked()]))
    }

    /* from_str* conversions */

    /// Creates a new `GraphemeNonul` from the first grapheme of a `string` slice.
    ///
    /// The grapheme will be truncated if it exceeds the capacity `CAP`.
    ///
    /// # Panics
    /// Panics if `CAP > 255.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `CAP > 255.
    pub const fn from_str(string: &str) -> Result<Self, MismatchedCapacity> {
        let mut machine = GraphemeMachine::new();
        let mut scanner = GraphemeScanner::<charu>::new(&mut machine, string);
        if let Some(g) = scanner.next_grapheme_nonul::<CAP>() {
            Ok(g)
        } else {
            Ok(Self::new())
        }
    }
    // TODO make another version exact non-truncating.
    // MAYBE return err if the string is empty.

    /* from_char* conversions */

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

    /// Creates a new `GraphemeNonul` from a `charu`.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `CAP` > 255
    /// or < `c.`[`len_utf8()`][charu#method.len_utf8].
    ///
    /// Will always succeed if `CAP` >= 4 and <= 255.
    pub const fn from_charu(c: charu) -> Result<Self, MismatchedCapacity> {
        Ok(Self(unwrap![ok? StringNonul::from_charu(c)]))
    }
    /// Creates a new `GraphemeNonul` from a `charu`.
    ///
    /// # Panics
    /// Panics if `CAP` > 255 or < `c.`[`len_utf8()`][charu#method.len_utf8].
    ///
    /// Will always succeed if `CAP` >= 4 and <= 255.
    pub const fn from_charu_unchecked(c: charu) -> Self {
        Self(StringNonul::from_charu_unchecked(c))
    }

    /* queries */

    #[must_use]
    /// Returns the length in bytes.
    pub const fn len(&self) -> usize { self.0.len() }

    #[must_use]
    /// Returns `true` if the current length is 0.
    pub const fn is_empty(&self) -> bool { self.0.len() == 0 }

    #[must_use]
    /// Returns the total capacity in bytes.
    pub const fn capacity() -> usize { CAP }

    #[must_use]
    /// Returns the remaining capacity.
    pub const fn remaining_capacity(&self) -> usize { CAP - self.len() }

    #[must_use]
    /// Returns `true` if the current remaining capacity is 0.
    pub const fn is_full(&self) -> bool { self.len() == CAP }

    /// Sets the length to 0, by resetting all bytes to 0.
    pub const fn clear(&mut self) { self.0.clear(); }

    /// Const-compatible `Eq`.
    pub const fn eq(self, other: &Self) -> bool { self.0.eq(&other.0) }

    //

    #[must_use]
    /// Returns a byte slice of the inner string slice.
    pub const fn as_bytes(&self) -> &[u8] { self.0.as_bytes() }

    #[must_use]
    /// Returns a mutable byte slice of the inner string slice.
    ///
    /// # Safety
    /// The caller must ensure that the content of the slice is valid UTF-8
    /// and that it contains exactly one extended grapheme character other
    /// than `NUL`, before the borrow ends and the underlying `str` is used.
    ///
    /// # Features
    /// Makes use of the `unsafe_slice` feature if enabled.
    #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_str")))]
    pub const unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        // SAFETY: caller must ensure safety
        unsafe { self.0.as_bytes_mut() }
    }

    #[must_use]
    /// Returns a copy of the inner array with the full contents.
    ///
    /// The array contains all the bytes, including those outside the current length.
    pub const fn as_array(&self) -> &[u8; CAP] { self.0.as_array() }

    #[must_use]
    /// Returns the inner array with the full contents.
    ///
    /// The array contains all the bytes, including those outside the current length.
    pub const fn into_array(self) -> [u8; CAP] { self.0.into_array() }

    #[must_use]
    /// Returns the inner string slice.
    pub const fn as_str(&self) -> &str { self.0.as_str() }

    /// Returns the inner string type.
    pub const fn as_string_nonul(&self) -> &StringNonul::<CAP> { &self.0 }

    /// Returns the inner string type.
    pub const fn into_string_nonul(self) -> StringNonul::<CAP> { self.0 }

    #[must_use]
    /// Returns the mutable inner string slice.
    ///
    /// # Safety
    /// The caller must ensure that the content of the slice is valid UTF-8
    /// and that it contains exactly one extended grapheme character other
    /// than `NUL`, before the borrow ends and the underlying `str` is used.
    #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_str")))]
    pub const unsafe fn as_mut_str(&mut self) -> &mut str {
        // SAFETY: caller must ensure safety
        unsafe { self.0.as_mut_str() }
    }

    /// Returns an iterator over the `chars` of this grapheme cluster.
    pub const fn chars(&self) -> CharIter<'_, &str> { self.0.chars() }
}

/* trait impls */

#[rustfmt::skip]
mod trait_impls {
    use crate::{
        ConstInit, Debug, Display, Formatter, FmtResult, GraphemeNonul, Hash, Hasher,
        StringNonul,
    };

    impl<const CAP: usize> Default for GraphemeNonul<CAP> {
        /// Returns an empty extended grapheme character.
        fn default() -> Self { Self::new() }
    }
    impl<const CAP: usize> ConstInit for GraphemeNonul<CAP> {
        /// An empty extended grapheme character.
        const INIT: Self = Self::new();
    }

    impl<const CAP: usize> PartialEq for GraphemeNonul<CAP> {
        fn eq(&self, other: &Self) -> bool { self.0.eq(&other.0) }
    }

    impl<const CAP: usize> PartialEq<StringNonul<CAP>> for GraphemeNonul<CAP> {
        fn eq(&self, other: &StringNonul<CAP>) -> bool { self.0.eq(other) }
    }
    impl<const CAP: usize> PartialEq<GraphemeNonul<CAP>> for StringNonul<CAP> {
        fn eq(&self, other: &GraphemeNonul<CAP>) -> bool { self.eq(&other.0) }
    }

    impl<const CAP: usize> Hash for GraphemeNonul<CAP> {
        fn hash<H: Hasher>(&self, state: &mut H) { self.0.hash(state); }
    }

    impl<const CAP: usize> Display for GraphemeNonul<CAP> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> { write!(f, "{}", self.0) }
    }
    impl<const CAP: usize> Debug for GraphemeNonul<CAP> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> { write!(f, "{:?}", self.0) }
    }
}
