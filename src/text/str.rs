// devela::text::str
//
//! `Str` namespace.
//

#[cfg(feature = "alloc")]
use crate::_dep::_alloc::str::from_boxed_utf8_unchecked;
#[allow(unused_imports)]
use crate::{
    Utf8Error,
    _core::str::{from_utf8, from_utf8_mut, from_utf8_unchecked, from_utf8_unchecked_mut},
};

/// A string slice namespace.
///
/// See also the [`std::str`] module.
pub struct Str;

impl Str {
    /// Converts a slice of bytes to a string slice.
    ///
    /// See [`from_utf8`].
    pub const fn from_utf8(v: &[u8]) -> Result<&str, Utf8Error> {
        from_utf8(v)
    }

    /// Converts a mutable slice of bytes to a mutable string slice.
    ///
    /// See [`from_utf8_mut`].
    pub fn from_utf8_mut(v: &mut [u8]) -> Result<&mut str, Utf8Error> {
        from_utf8_mut(v)
    }

    /// Converts a slice of bytes to a string slice without checking valid UTF-8.
    ///
    /// See [`from_utf8_unchecked`].
    ///
    /// # Safety
    /// The bytes passed in must be valid UTF-8.
    #[must_use]
    #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_str")))]
    pub const unsafe fn from_utf8_unchecked(v: &[u8]) -> &str {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { from_utf8_unchecked(v) }
    }

    /// Converts a mutable slice of bytes to a mutable string slice without checking valid UTF-8.
    ///
    /// See [`from_utf8_unchecked_mut`].
    ///
    /// # Safety
    /// The bytes passed in must be valid UTF-8.
    #[must_use]
    #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_str")))]
    pub unsafe fn from_utf8_unchecked_mut(v: &mut [u8]) -> &mut str {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { from_utf8_unchecked_mut(v) }
    }

    /// Converts a boxed slice of bytes to a boxed string slice without checking valid UTF-8.
    ///
    /// See [`from_boxed_utf8_unchecked`].
    ///
    /// # Safety
    /// The bytes passed in must be valid UTF-8.
    #[must_use]
    #[cfg(feature = "alloc")]
    #[cfg(all(not(feature = "safe_text"), feature = "unsafe_str"))]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(all(feature = "alloc", feature = "unsafe_str"))))]
    pub unsafe fn from_boxed_utf8_unchecked(v: Box<[u8]>) -> Box<str> {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { from_boxed_utf8_unchecked(v) }
    }
}
