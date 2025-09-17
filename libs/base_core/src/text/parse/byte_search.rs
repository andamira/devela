// devela_base_core::text::parse::byte_search
//
//! Naive versions of `memchr` fns
//

#[cfg(feature = "dep_memchr")]
use crate::_dep::memchr::*;

#[cfg(not(feature = "dep_memchr"))]
use crate::is;

#[doc = crate::_TAG_TEXT!()]
#[doc = crate::_TAG_NAMESPACE!()]
// #[doc = crate::_TAG_SEARCH!()] // TODO
/// A utility struct for searching bytes in slices.
///
/// # Features
/// - Supports efficient search for bytes with `dep_memchr` enabled.
/// - Provides fallback naive implementations otherwise.
#[derive(Debug)]
pub struct ByteSearch;

impl ByteSearch {
    /// Search for the first occurrence of a byte in a slice.
    ///
    /// # Features
    /// Makes use of the `dep_memchr` dependency if enabled. See [memchr],
    ///
    /// [memchr]: fn@memchr
    #[must_use] #[rustfmt::skip]
    pub fn first1(needle: u8, haystack: &[u8]) -> Option<usize> {
        #[cfg(feature = "dep_memchr")]
        return memchr(needle, haystack);
        #[cfg(not(feature = "dep_memchr"))]
        haystack.iter().enumerate()
            .find_map(|(index, &byte)| is![byte == needle; Some(index); None])
    }

    /// Search for the first occurrence of two possible bytes in a haystack.
    ///
    /// # Features
    /// Makes use of the `dep_memchr` dependency if enabled. See [memchr2],
    #[must_use] #[rustfmt::skip]
    pub fn first2(needle1: u8, needle2: u8, haystack: &[u8]) -> Option<usize> {
        #[cfg(feature = "dep_memchr")]
        return memchr2(needle1, needle2, haystack);
        #[cfg(not(feature = "dep_memchr"))]
        haystack.iter().enumerate()
            .find_map(|(index, &byte)| is![byte == needle1 || byte == needle2; Some(index); None])
    }

    /// Search for the first occurrence of three possible bytes in a haystack.
    ///
    /// # Features
    /// Makes use of the `dep_memchr` dependency if enabled. See [memchr3],
    #[must_use]
    pub fn first3(needle1: u8, needle2: u8, needle3: u8, haystack: &[u8]) -> Option<usize> {
        #[cfg(feature = "dep_memchr")]
        return memchr3(needle1, needle2, needle3, haystack);
        #[cfg(not(feature = "dep_memchr"))]
        haystack.iter().enumerate().find_map(|(index, &byte)| {
            is![byte == needle1 || byte == needle2 || byte == needle3; Some(index); None]
        })
    }

    /// Search for the last occurrence of a byte in a slice.
    ///
    /// # Features
    /// Makes use of the `dep_memchr` dependency if enabled. See [memrchr],
    #[must_use] #[rustfmt::skip]
    pub fn last1(needle: u8, haystack: &[u8]) -> Option<usize> {
        #[cfg(feature = "dep_memchr")]
        return memrchr(needle, haystack);
        #[cfg(not(feature = "dep_memchr"))]
        haystack.iter().enumerate().rev()
            .find_map(|(index, &byte)| is![byte == needle; Some(index); None])
    }

    /// Search for the last occurrence of two possible bytes in a haystack.
    ///
    /// # Features
    /// Makes use of the `dep_memchr` dependency if enabled. See [memrchr2],
    #[must_use] #[rustfmt::skip]
    pub fn last2(needle1: u8, needle2: u8, haystack: &[u8]) -> Option<usize> {
        #[cfg(feature = "dep_memchr")]
        return memrchr2(needle1, needle2, haystack);
        #[cfg(not(feature = "dep_memchr"))]
        haystack.iter().enumerate().rev()
            .find_map(|(index, &byte)| is![byte == needle1 || byte == needle2; Some(index); None])
    }

    /// Search for the last occurrence of three possible bytes in a haystack.
    ///
    /// # Features
    /// Makes use of the `dep_memchr` dependency if enabled. See [memrchr3],
    #[must_use]
    pub fn last3(needle1: u8, needle2: u8, needle3: u8, haystack: &[u8]) -> Option<usize> {
        #[cfg(feature = "dep_memchr")]
        return memrchr3(needle1, needle2, needle3, haystack);
        #[cfg(not(feature = "dep_memchr"))]
        haystack.iter().enumerate().rev()
            .find_map(|(index, &byte)|
                is![byte == needle1 || byte == needle2 || byte == needle3; Some(index); None])
    }
}
