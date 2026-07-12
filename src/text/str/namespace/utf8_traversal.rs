// devela/src/text/str/namespace/utf8_traversal.rs
//
// IMPROVE:
// - one default, (simd == api if possible)
// - other faster-simdversion if possible (no care about api, errors)
// can't import either or, has to be both, for this module…
use ::core::str::from_utf8_mut;
// crate::_use! {basic::from_utf8} // MAYBE not needed

use crate::{CharIter, InvalidUtf8, Str, slice};
#[cfg(feature = "grapheme")]
use crate::{GraphemeBoundary, GraphemeIter, GraphemeMachine, GraphemeScanner, charu, is};
#[allow(unused_imports, reason = "±unsafe")]
use {
    crate::unwrap,
    ::core::str::{from_utf8_unchecked, from_utf8_unchecked_mut},
};

/// # UTF-8 conversion methods.
impl Str {
    /// Converts a slice of bytes to a string slice.
    ///
    /// See `core::str::`[`from_utf8`][::core::str::from_utf8].
    //
    // WAIT: [const_methods](https://github.com/rusticstuff/simdutf8/pull/111)
    // /// # Features
    // /// if the `dep_simdutf8` is enabled
    // /// then `simdutf8::compat::`[`from_utf8`] is called instead.
    #[allow(rustdoc::broken_intra_doc_links, reason = "±unsafe")]
    pub const fn from_utf8(v: &[u8]) -> Result<&str, InvalidUtf8> {
        // #[cfg(not(feature = "dep_simdutf8"))]
        match ::core::str::from_utf8(v) {
            Ok(v) => Ok(v),
            Err(e) => Err(InvalidUtf8::from_utf8_error(e)),
        }
        // #[cfg(feature = "dep_simdutf8")]
        // match ::simdutf8::compat::from_utf8(v) {
        //     Ok(v) => Ok(v),
        //     Err(e) => Err(InvalidUtf8::from_compat_utf8_error(e)),
        // }
    }

    /// Converts an exclusive slice of bytes to an exclusive string slice.
    ///
    /// See [`from_utf8_mut`].
    pub const fn from_utf8_mut(v: &mut [u8]) -> Result<&mut str, InvalidUtf8> {
        match from_utf8_mut(v) {
            Ok(v) => Ok(v),
            Err(e) => Err(InvalidUtf8::from_utf8_error(e)),
        }
    }

    /// Converts a slice of bytes to a string slice without checking valid UTF-8.
    ///
    /// See [`from_utf8_unchecked`].
    ///
    /// # Safety
    /// The bytes passed in must be valid UTF-8.
    #[must_use]
    #[cfg(not(feature = "safe_text"))]
    pub const unsafe fn from_utf8_unchecked(v: &[u8]) -> &str {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { from_utf8_unchecked(v) }
    }

    /// Converts an exclusive slice of bytes to an exclusive string slice
    /// without checking valid UTF-8.
    ///
    /// See [`from_utf8_unchecked_mut`].
    ///
    /// # Safety
    /// The bytes passed in must be valid UTF-8.
    #[must_use]
    #[cfg(not(feature = "safe_text"))]
    pub const unsafe fn from_utf8_unchecked_mut(v: &mut [u8]) -> &mut str {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { from_utf8_unchecked_mut(v) }
    }

    /// Converts `bytes` to the longest complete UTF-8 prefix.
    ///
    /// An incomplete final code point is discarded.
    /// Any other invalid UTF-8 sequence is returned as an error.
    pub const fn from_utf8_complete_prefix(bytes: &[u8]) -> Result<&str, InvalidUtf8> {
        match Self::from_utf8(bytes) {
            Ok(string) => Ok(string),
            Err(error) if error.error_len.is_none() => {
                Self::from_utf8(slice![bytes, ..error.valid_up_to])
            }
            Err(error) => Err(error),
        }
    }

    /// Converts a byte slice known to be valid UTF-8 to a string.
    ///
    /// # Features
    /// It can use unsafe internally to skip checks.
    ///
    /// # Safety
    /// `bytes` must contain valid utf-8.
    ///
    /// This is intended to be called by macros, where the feature-bounds
    /// would be tested against the user code.
    ///
    /// This is why it is not marked as unsafe; the macro is the one to enforce safety.
    ///
    /// # Used by
    /// [`join!`][crate::join].
    #[doc(hidden)] #[rustfmt::skip]
    pub const fn __utf8_bytes_to_str(bytes: &[u8]) -> &str {
        cfg_select! { not(feature = "safe_text") => {
            unsafe { ::core::str::from_utf8_unchecked(bytes) }
        } _ => { unwrap![ok ::core::str::from_utf8(bytes)] }}
    }
}

/// # Text traversal methods.
impl Str {
    /* chars */

    /// Returns an iterator over the Unicode scalars.
    #[inline(always)]
    pub const fn chars(string: &str) -> CharIter<'_, &str> {
        CharIter::<&str>::new(string)
    }
    /// Returns the total number of Unicode scalars.
    #[must_use]
    #[inline(always)]
    pub const fn char_count(string: &str) -> usize {
        CharIter::<&str>::new(string).count()
    }

    /* graphemes */

    /// Returns an iterator over grapheme-boundary actions and Unicode scalars.
    #[inline(always)]
    #[cfg(feature = "grapheme")]
    pub const fn graphemes(string: &str) -> GraphemeIter<'_, char> {
        GraphemeIter::<char>::new(string)
    }
    /// Returns an iterator over grapheme-boundary actions and [`charu`] scalars.
    #[inline(always)]
    #[cfg(feature = "grapheme")]
    pub const fn graphemes_charu(string: &str) -> GraphemeIter<'_, charu> {
        GraphemeIter::<charu>::new_charu(string)
    }

    /// Returns a grapheme-boundary scanner over `string`.
    ///
    /// The caller provides the machine so the scan can continue across buffers.
    #[inline(always)]
    #[cfg(feature = "grapheme")]
    pub const fn graphemes_in<'a>(
        machine: &'a mut GraphemeMachine,
        string: &'a str,
    ) -> GraphemeScanner<'a, char> {
        machine.next_char_from_str(string)
    }
    /// Returns a grapheme-boundary scanner over `string`.
    ///
    /// The caller provides the machine so the scan can continue across buffers.
    #[inline(always)]
    #[cfg(feature = "grapheme")]
    pub const fn graphemes_charu_in<'a>(
        machine: &'a mut GraphemeMachine,
        string: &'a str,
    ) -> GraphemeScanner<'a, charu> {
        machine.next_charu_from_str(string)
    }
    /// Returns the number of grapheme clusters in `string`.
    #[must_use]
    #[cfg(feature = "grapheme")]
    pub const fn grapheme_count(string: &str) -> usize {
        let mut iter = Self::graphemes_charu(string);
        let mut count = 0;
        while let Some((boundary, _)) = iter.next() {
            is! { boundary.eq(GraphemeBoundary::Split), count += 1 }
        }
        count
    }
}
