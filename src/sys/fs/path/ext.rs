// devela::sys::fs::path::ext
//
//! An extension trait for [`Path`] and [`PathBuf`].
//

use crate::{IoResult, Path, PathBuf};
use std::path::{MAIN_SEPARATOR, MAIN_SEPARATOR_STR, absolute, is_separator};

/// Marker trait to prevent downstream implementations of the [`ExtPath`] trait.
trait Sealed {}
impl Sealed for Path {}
impl Sealed for PathBuf {}

#[doc = crate::_TAG_NAMESPACE!()]
/// Extension trait providing additional methods for [`Path`] and [`PathBuf`].
#[rustfmt::skip]
#[cfg_attr(nightly_doc, doc(notable_trait))]
#[expect(private_bounds, reason = "Sealed")]
pub trait ExtPath: Sealed {
    /// The primary separator string of path components for the current platform.
    ///
    /// See `std::path::`[MAIN_SEPARATOR_STR].
    const SEPARATOR: &str = MAIN_SEPARATOR_STR;

    /// The primary separator char of path components for the current platform.
    ///
    /// See `std::path::`[MAIN_SEPARATOR].
    const SEPARATOR_CHAR: char = MAIN_SEPARATOR;

    /// Makes the given path absolute without accessing the filesystem.
    ///
    /// See `std::path::`[`absolute`] and [`Self::to_absolute`].
    fn absolute<P: AsRef<Path>>(path: P) -> IoResult<PathBuf> {
        absolute(path)
    }

    /// Makes the current path absolute without accessing the filesystem.
    ///
    /// See `std::path::`[`absolute`] and [`Self::absolute`].
    fn to_absolute(&self) -> IoResult<PathBuf>;

    /// Determines whether the character is one of the permitted path separators
    /// for the current platform.
    ///
    /// See `std::path::`[`is_separator`].
    #[must_use]
    fn is_separator(c: char) -> bool {
        is_separator(c)
    }

}
impl ExtPath for Path {
    fn to_absolute(&self) -> IoResult<PathBuf> {
        absolute(self)
    }
}
impl ExtPath for PathBuf {
    fn to_absolute(&self) -> IoResult<PathBuf> {
        absolute(self)
    }
}
