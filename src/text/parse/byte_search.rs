// devela::text::parse::byte_search
//
//! Naive versions of `memchr` fns
//

#[cfg(not(feature = "dep_memchr"))]
use crate::iif;

/// A utility struct for searching bytes in slices.
///
/// # Features
/// - Supports efficient search for bytes with `dep_memchr`.
/// - Provides fallback naive implementations for environments without `dep_memchr`.
pub struct ByteSearch;

impl ByteSearch {
    /// Search for the first occurrence of a byte in a slice.
    ///
    /// Equivalent to [`memchr`][crate::memchr].
    ///
    /// # Features
    /// Makes use of the `dep_memchr` dependency if enabled.
    #[must_use] #[rustfmt::skip]
    pub fn first1(needle: u8, haystack: &[u8]) -> Option<usize> {
        #[cfg(feature = "dep_memchr")]
        return memchr::memchr(needle, haystack);
        #[cfg(not(feature = "dep_memchr"))]
        haystack.iter().enumerate()
            .find_map(|(index, &byte)| iif![byte == needle; Some(index); None])
    }

    /// Search for the first occurrence of two possible bytes in a haystack.
    ///
    /// Equivalent to [`memchr2`][crate::memchr2].
    ///
    /// # Features
    /// Makes use of the `dep_memchr` dependency if enabled.
    #[must_use] #[rustfmt::skip]
    pub fn first2(needle1: u8, needle2: u8, haystack: &[u8]) -> Option<usize> {
        #[cfg(feature = "dep_memchr")]
        return memchr::memchr2(needle1, needle2, haystack);
        #[cfg(not(feature = "dep_memchr"))]
        haystack.iter().enumerate()
            .find_map(|(index, &byte)| iif![byte == needle1 || byte == needle2; Some(index); None])
    }

    /// Search for the first occurrence of three possible bytes in a haystack.
    ///
    /// Equivalent to [`memchr3`][crate::memchr3].
    ///
    /// # Features
    /// Makes use of the `dep_memchr` dependency if enabled.
    #[must_use]
    pub fn first3(needle1: u8, needle2: u8, needle3: u8, haystack: &[u8]) -> Option<usize> {
        #[cfg(feature = "dep_memchr")]
        return memchr::memchr3(needle1, needle2, needle3, haystack);
        #[cfg(not(feature = "dep_memchr"))]
        haystack.iter().enumerate().find_map(|(index, &byte)| {
            iif![byte == needle1 || byte == needle2 || byte == needle3; Some(index); None]
        })
    }

    /// Search for the last occurrence of a byte in a slice.
    ///
    /// Equivalent to [`memrchr`][crate::memrchr].
    ///
    /// # Features
    /// Makes use of the `dep_memchr` dependency if enabled.
    #[must_use] #[rustfmt::skip]
    pub fn last1(needle: u8, haystack: &[u8]) -> Option<usize> {
        #[cfg(feature = "dep_memchr")]
        return memchr::memrchr(needle, haystack);
        #[cfg(not(feature = "dep_memchr"))]
        haystack.iter().enumerate().rev()
            .find_map(|(index, &byte)| iif![byte == needle; Some(index); None])
    }

    /// Search for the last occurrence of two possible bytes in a haystack.
    ///
    /// Equivalent to [memrchr2][crate::memrchr2].
    ///
    /// # Features
    /// Makes use of the `dep_memchr` dependency if enabled.
    #[must_use] #[rustfmt::skip]
    pub fn last2(needle1: u8, needle2: u8, haystack: &[u8]) -> Option<usize> {
        #[cfg(feature = "dep_memchr")]
        return memchr::memrchr2(needle1, needle2, haystack);
        #[cfg(not(feature = "dep_memchr"))]
        haystack.iter().enumerate().rev()
            .find_map(|(index, &byte)| iif![byte == needle1 || byte == needle2; Some(index); None])
    }

    /// Search for the last occurrence of three possible bytes in a haystack.
    ///
    /// Equivalent to [memrchr3][crate::memrchr3].
    ///
    /// # Features
    /// Makes use of the `dep_memchr` dependency if enabled.
    #[must_use]
    pub fn last3(needle1: u8, needle2: u8, needle3: u8, haystack: &[u8]) -> Option<usize> {
        #[cfg(feature = "dep_memchr")]
        return memchr::memrchr3(needle1, needle2, needle3, haystack);
        #[cfg(not(feature = "dep_memchr"))]
        haystack.iter().enumerate().rev()
            .find_map(|(index, &byte)|
                iif![byte == needle1 || byte == needle2 || byte == needle3; Some(index); None])
    }
}
