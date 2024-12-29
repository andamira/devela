// devela::sys::path::ext
//
//! An extension trait for [`Path`] and [`PathBuf`].
//

use crate::{IoResult, Path, PathBuf};
use std::path::{absolute, is_separator, MAIN_SEPARATOR, MAIN_SEPARATOR_STR};

/// Marker trait to prevent downstream implementations of the [`ExtPath`] trait.
trait Sealed {}
impl Sealed for Path {}
impl Sealed for PathBuf {}

/// Extension trait providing additional methods for paths.
#[rustfmt::skip]
#[cfg_attr(feature = "nightly_doc", doc(notable_trait))]
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
