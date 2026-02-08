// devela_base_core::text::grapheme::u8
//
//!
//
// TOC
// - definitions
// - trait impls

use crate::{
    CharIter, GraphemeMachine, GraphemeScanner, MismatchedCapacity, StringU8, char7, char8, char16,
    charu, unwrap,
};

/* definitions */

#[doc = crate::_tags!(text)]
#[doc = concat!["An ", crate::_ABBR_EGC!(), " backed by a [`StringU8`]."]]
#[doc = crate::_doc_location!("text/grapheme")]
///
/// ## Methods
///
/// - [Constructors](#constructors):
///   [`new`][Self::new],
///     *([_checked][Self::new_checked])*.
///   [`from_str`][Self::from_str],
///     *([_truncate][Self::from_str_truncate],
///       [_unchecked][Self::from_str_unchecked])*,
///   [`from_char`][Self::from_char]
///     *([7][Self::from_char7],
///       [8][Self::from_char8],
///       [16](Self::from_char16),
///       [utf8](Self::from_charu))*.
#[must_use]
#[repr(transparent)]
#[derive(Clone, Copy, Eq, PartialOrd, Ord)]
pub struct GraphemeU8<const CAP: usize>(pub(crate) StringU8<CAP>);

#[rustfmt::skip]
impl<const CAP: usize> GraphemeU8<CAP> {
    /* constructors */

    /// Creates a new empty `GraphemeU8` with a capacity of `CAP` bytes.
    ///
    /// # Panics
    /// Panics if `CAP > 255.
    #[inline(always)]
    pub const fn new() -> Self {
        Self(StringU8::new())
    }

    /// Creates a new empty `GraphemeU8` with a capacity of `CAP` bytes.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `CAP > 255.
    pub const fn new_checked() -> Result<Self, MismatchedCapacity> {
        Ok(Self(unwrap![ok? StringU8::new_checked()]))
    }

    /* from_str* conversions */

    /// Creates a new `GraphemeU8` from the first grapheme of a `string` slice.
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
        if let Some(g) = scanner.next_grapheme_u8::<CAP>() {
            Ok(g)
        } else {
            Ok(Self::new())
        }
    }
    // TODO make another version exact non-truncating.
    // MAYBE return err if the string is empty.

    /* from_char* conversions */

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
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `CAP` > 255
    /// or < `c.`[`len_utf8()`][UnicodeScalar#method.len_utf8].
    ///
    /// Will always succeed if `CAP` >= 4 and <= 255.
    pub const fn from_char(c: char) -> Result<Self, MismatchedCapacity> {
        Ok(Self(unwrap![ok? StringU8::from_char(c)]))
    }

    /// Creates a new `GraphemeU8` from a `charu`.
    ///
    /// # Errors
    /// Returns [`MismatchedCapacity`] if `CAP` > 255
    /// or < `c.`[`len_utf8()`][charu#method.len_utf8].
    ///
    /// Will always succeed if `CAP` >= 4 and <= 255.
    pub const fn from_charu(c: charu) -> Result<Self, MismatchedCapacity> {
        Ok(Self(unwrap![ok? StringU8::from_charu(c)]))
    }
    /// Creates a new `GraphemeU8` from a `charu`.
    ///
    /// # Panics
    /// Panics if `CAP` > 255 or < `c.`[`len_utf8()`][charu#method.len_utf8].
    ///
    /// Will always succeed if `CAP` >= 4 and <= 255.
    pub const fn from_charu_unchecked(c: charu) -> Self {
        Self(StringU8::from_charu_unchecked(c))
    }

    /* queries */

    /// Returns the length in bytes.
    #[must_use] #[inline(always)]
    pub const fn len(&self) -> usize { self.0.len() }

    /// Returns `true` if the current length is 0.
    #[must_use] #[inline(always)]
    pub const fn is_empty(&self) -> bool { self.0.len() == 0 }

    /// Returns the total capacity in bytes.
    #[must_use] #[inline(always)]
    pub const fn capacity() -> usize { CAP }

    /// Returns the remaining capacity.
    #[must_use] #[inline(always)]
    pub const fn remaining_capacity(&self) -> usize { CAP - self.len() }

    /// Returns `true` if the current remaining capacity is 0.
    #[must_use] #[inline(always)]
    pub const fn is_full(&self) -> bool { self.len() == CAP }

    /// Sets the length to 0, by resetting all bytes to 0.
    #[inline(always)]
    pub const fn clear(&mut self) { self.0.clear(); }

    /// Const-compatible `Eq`.
    #[must_use] #[inline(always)]
    pub const fn eq(self, other: &Self) -> bool { self.0.eq(&other.0) }

    //

    /// Returns a byte slice of the inner string slice.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] { self.0.as_bytes() }

    /// Returns a mutable byte slice of the inner string slice.
    ///
    /// # Safety
    /// The caller must ensure that the content of the slice is valid UTF-8
    /// and that it contains exactly one extended grapheme character,
    /// before the borrow ends and the underlying `str` is used.
    #[must_use]
    #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_str")))]
    pub const unsafe fn as_bytes_mut(&mut self) -> &mut [u8] {
        // SAFETY: caller must ensure safety
        unsafe { self.0.as_bytes_mut() }
    }

    /// Returns a copy of the inner array with the full contents.
    ///
    /// The array contains all the bytes, including those outside the current length.
    #[must_use]
    pub const fn as_array(&self) -> &[u8; CAP] { self.0.as_array() }

    /// Returns the inner array with the full contents.
    ///
    /// The array contains all the bytes, including those outside the current length.
    #[must_use]
    pub const fn into_array(self) -> [u8; CAP] { self.0.into_array() }

    /// Returns the inner string slice.
    #[must_use]
    pub const fn as_str(&self) -> &str { self.0.as_str() }

    /// Returns the inner string type.
    #[inline(always)]
    pub const fn as_string_u8(&self) -> &StringU8::<CAP> { &self.0 }

    /// Returns the inner string type.
    #[inline(always)]
    pub const fn into_string_u8(self) -> StringU8::<CAP> { self.0 }

    /// Returns the mutable inner string slice.
    ///
    /// # Safety
    /// The caller must ensure that the content of the slice is valid UTF-8
    /// and that it contains exactly one extended grapheme character,
    /// before the borrow ends and the underlying `str` is used.
    #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_str")))]
    pub const unsafe fn as_mut_str(&mut self) -> &mut str {
        // SAFETY: caller must ensure safety
        unsafe { self.0.as_mut_str() }
    }

    /// Returns an iterator over the `chars` of this grapheme cluster.
    #[inline(always)]
    pub const fn chars(&self) -> CharIter<'_, &str> { self.0.chars() }
}

/* trait impls */

#[rustfmt::skip]
mod trait_impls {
    use crate::{
        Debug, ConstInitCore, Display, Formatter, FmtResult, GraphemeU8, Hash, Hasher, StringU8,
    };

    impl<const CAP: usize> Default for GraphemeU8<CAP> {
        /// Returns an empty extended grapheme character.
        ///
        /// # Panics
        /// Panics if `CAP` > 255.
        #[inline(always)]
        fn default() -> Self { Self::new() }
    }
    impl<const CAP: usize> ConstInitCore for GraphemeU8<CAP> {
        /// An empty extended grapheme character.
        /// # Panics
        /// Panics if `CAP` > 255.
        const INIT: Self = Self::new();
    }

    impl<const CAP: usize> PartialEq for GraphemeU8<CAP> {
        fn eq(&self, other: &Self) -> bool { self.0.eq(&other.0) }
    }
    impl<const CAP: usize> PartialEq<StringU8<CAP>> for GraphemeU8<CAP> {
        fn eq(&self, other: &StringU8<CAP>) -> bool { self.0.eq(other) }
    }
    impl<const CAP: usize> PartialEq<GraphemeU8<CAP>> for StringU8<CAP> {
        fn eq(&self, other: &GraphemeU8<CAP>) -> bool { self.eq(&other.0) }
    }

    impl<const CAP: usize> Hash for GraphemeU8<CAP> {
        fn hash<H: Hasher>(&self, state: &mut H) { self.0.hash(state); }
    }

    impl<const CAP: usize> Display for GraphemeU8<CAP> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> { write!(f, "{}", self.0) }
    }
    impl<const CAP: usize> Debug for GraphemeU8<CAP> {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult<()> { write!(f, "{:?}", self.0) }
    }
}
