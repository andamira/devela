// devela::data::slice::ext
//
//!
//

#[cfg(feature = "alloc")]
use _alloc::vec::Vec;

use super::fns::*;

// Marker trait to prevent downstream implementations of the `SliceExt` trait.
mod private {
    pub trait Sealed {}
}
impl<T> private::Sealed for [T] {}
impl<T> private::Sealed for &[T] {}
impl<T> private::Sealed for &mut [T] {}
impl<T, const LEN: usize> private::Sealed for [T; LEN] {}
#[cfg(feature = "alloc")]
impl<T> private::Sealed for Vec<T> {}

/// Extension trait providing additional methods for [`&[T]`][slice].
///
/// This trait is sealed and cannot be implemented for any other type.
pub trait SliceExt<T>: private::Sealed {
    /// Returns a left subslice of `slice` with the given maximum `len`.
    ///
    /// If `left_len > slice.len()` it returns the full slice.
    ///
    /// See also [`slice_lsplit`] for the standalone `const` version.
    #[must_use]
    fn lsplit(&self, len: usize) -> &[T];

    /// Returns a right subslice of `slice` with the given maximum `len`.
    ///
    /// If `left_len > slice.len()` it returns the full slice.
    ///
    /// See also [`slice_rsplit`] for the standalone `const` version.
    #[must_use]
    fn rsplit(&self, len: usize) -> &[T];

    /// Returns a middle subslice of `slice` with the given maximum `len`
    /// and a left bias.
    ///
    /// In case of a non-perfect middle split, it will have one character more
    /// on the left.
    ///
    /// If `len > slice.len()` returns the full `slice`.
    ///
    /// See also [`slice_msplit_left`] for the standalone `const` version.
    #[must_use]
    fn msplit_left(&self, len: usize) -> &[T];

    /// Returns a middle subslice of `slice` with the given maximum `len`
    /// and a right bias.
    ///
    /// In case of a non-perfect middle split, it will have one character more
    /// on the right.
    ///
    /// If `len > slice.len()` returns the full `slice`.
    ///
    /// See also [`slice_msplit_right`] for the standalone `const` version.
    #[must_use]
    fn msplit_right(&self, len: usize) -> &[T];
}

/// Extension trait providing additional methods for [`&mut [T]`][slice].
///
/// This trait is sealed and cannot be implemented for any other type.
pub trait SliceExtMut<T>: private::Sealed + SliceExt<T> {
    /// Returns a mutable left subslice of `slice` with the given maximum `len`.
    ///
    /// If `left_len > slice.len()` it returns the full slice.
    #[must_use]
    fn lsplit_mut(&mut self, len: usize) -> &mut [T];

    /// Returns a mutable right subslice of `slice` with the given maximum `len`.
    ///
    /// If `left_len > slice.len()` it returns the full slice.
    #[must_use]
    fn rsplit_mut(&mut self, len: usize) -> &mut [T];

    /// Returns a mutable middle subslice of `slice` with the given maximum `len`
    /// and a left bias.
    ///
    /// In case of a non-perfect middle split, it will have one character more
    /// on the left.
    ///
    /// If `len > slice.len()` returns the full `slice`.
    #[must_use]
    fn msplit_left_mut(&mut self, len: usize) -> &mut [T];

    /// Returns a mutable middle subslice of `slice` with the given maximum `len`
    /// and a right bias.
    ///
    /// In case of a non-perfect middle split, it will have one character more
    /// on the right.
    ///
    /// If `len > slice.len()` returns the full `slice`.
    #[must_use]
    fn msplit_right_mut(&mut self, len: usize) -> &mut [T];
}

macro_rules! slice_ext_impl {
    ($t:ty, for $for:ty, impl: $($impl:tt)*) => {
        impl<$($impl)*> SliceExt<$t> for $for {
            #[inline]
            fn lsplit(&self, len: usize) -> &[T] {
                slice_lsplit(self, len)
            }
            #[inline]
            fn rsplit(&self, len: usize) -> &[T] {
                slice_rsplit(self, len)
            }
            #[inline]
            fn msplit_left(&self, len: usize) -> &[T] {
                slice_msplit_left(self, len)
            }
            #[inline]
            fn msplit_right(&self, len: usize) -> &[T] {
                slice_msplit_right(self, len)
            }
        }
    };
    (mut: $t:ty, for $for:ty, impl: $($impl:tt)*) => {
        slice_ext_impl![$t, for $for, impl: $($impl)*];

        impl<$($impl)*> SliceExtMut<$t> for $for {
            #[inline]
            fn lsplit_mut(&mut self, len: usize) -> &mut [T] {
                slice_lsplit_mut(self, len)
            }
            #[inline]
            fn rsplit_mut(&mut self, len: usize) -> &mut [T] {
                slice_rsplit_mut(self, len)
            }
            #[inline]
            fn msplit_left_mut(&mut self, len: usize) -> &mut [T] {
                slice_msplit_left_mut(self, len)
            }
            #[inline]
            fn msplit_right_mut(&mut self, len: usize) -> &mut [T] {
                slice_msplit_right_mut(self, len)
            }
        }
    };
}
slice_ext_impl![mut: T, for [T], impl: T];
slice_ext_impl![T, for &[T], impl: T];
slice_ext_impl![mut: T, for &mut [T], impl: T];
slice_ext_impl![mut: T, for [T; LEN], impl: T, const LEN: usize];
#[cfg(feature = "alloc")]
slice_ext_impl![mut: T, for Vec<T>, impl: T];
