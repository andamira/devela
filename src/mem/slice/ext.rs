// devela::mem::slice::ext
//
//!
//

use super::fns::*;
use crate::data::convert::slice_into_array;
#[cfg(feature = "alloc")]
use {
    crate::data::convert::{slice_into_vec, try_slice_into_vec},
    _alloc::vec::Vec,
};

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
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
pub trait SliceExt<T>: private::Sealed {
    /* split */

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

    /* convert */

    /// Converts `&[T]` to `[U; N]` when `U` implements `From<T>`.
    ///
    /// # Panics
    /// Panics if the length of the slice is less than the length of the array.
    /// # Examples
    /// ```
    /// # use devela::mem::SliceExt;
    /// assert_eq![[1_u16, 2, 3], [1_u8, 2, 3].into_array()];
    /// assert_eq![[1_u16, 2, 3], [1_u8, 2, 3].into_array::<u16, 3>()];
    /// ```
    #[must_use]
    fn into_array<U, const N: usize>(&self) -> [U; N]
    where
        T: Clone,
        U: From<T>;

    /// Converts `&[T]` to `Vec<U>` when `U` implements `From<T>`.
    /// # Examples
    /// ```
    /// # use devela::mem::SliceExt;
    /// assert_eq![vec![1_i16, 2, 3], [1_u8, 2, 3].into_vec()];
    /// assert_eq![vec![1_i16, 2, 3], [1_u8, 2, 3].into_vec::<i16>()];
    /// ```
    #[must_use]
    #[cfg(feature = "alloc")]
    fn into_vec<U>(&self) -> Vec<U>
    where
        T: Clone,
        U: From<T>;

    /// Tries to convert `&[T]` to `Vec<U>` when `U` implements `TryFrom<T>`.
    /// # Examples
    /// ```
    /// # use devela::mem::SliceExt;
    /// assert_eq![Ok(vec![1_i32, 2, 3]), [1_i64, 2, 3].try_into_vec()];
    /// assert_eq![Ok(vec![1_i32, 2, 3]), [1_i64, 2, 3].try_into_vec::<_, i32>()];
    /// ```
    #[must_use]
    #[cfg(feature = "alloc")]
    fn try_into_vec<E, U>(&self) -> Result<Vec<U>, E>
    where
        T: Clone,
        U: TryFrom<T, Error = E>;
}

/// Extension trait providing additional methods for [`&mut [T]`][slice].
///
/// This trait is sealed and cannot be implemented for any other type.
#[cfg_attr(feature = "nightly", doc(cfg(feature = "mem")))]
pub trait SliceExtMut<T>: private::Sealed + SliceExt<T> {
    /* split */

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
            /* split */

            #[inline]
            fn lsplit(&self, len: usize) -> &[T] { slice_lsplit(self, len) }
            #[inline]
            fn rsplit(&self, len: usize) -> &[T] { slice_rsplit(self, len) }
            #[inline]
            fn msplit_left(&self, len: usize) -> &[T] { slice_msplit_left(self, len) }
            #[inline]
            fn msplit_right(&self, len: usize) -> &[T] { slice_msplit_right(self, len) }

            /* collection */

            #[inline]
            fn into_array<U, const N: usize>(&self) -> [U; N] where T: Clone, U: From<T> {
                slice_into_array(self)
            }
            #[inline] #[cfg(feature = "alloc")]
            fn into_vec<U>(&self) -> Vec<U> where T: Clone, U: From<T> { slice_into_vec(self) }
            #[inline] #[cfg(feature = "alloc")]
            fn try_into_vec<E, U>(&self) -> Result<Vec<U>, E>
                where T: Clone, U: TryFrom<T, Error = E> { try_slice_into_vec(self) }
        }
    };
    (mut: $t:ty, for $for:ty, impl: $($impl:tt)*) => {
        slice_ext_impl![$t, for $for, impl: $($impl)*];

        impl<$($impl)*> SliceExtMut<$t> for $for {
            /* split */

            #[inline]
            fn lsplit_mut(&mut self, len: usize) -> &mut [T] { slice_lsplit_mut(self, len) }
            #[inline]
            fn rsplit_mut(&mut self, len: usize) -> &mut [T] { slice_rsplit_mut(self, len) }
            #[inline]
            fn msplit_left_mut(&mut self, len: usize) -> &mut [T] {
                slice_msplit_left_mut(self, len) }
            #[inline]
            fn msplit_right_mut(&mut self, len: usize) -> &mut [T] {
                slice_msplit_right_mut(self, len) }
        }
    };
}
slice_ext_impl![mut: T, for [T], impl: T];
slice_ext_impl![T, for &[T], impl: T];
slice_ext_impl![mut: T, for &mut [T], impl: T];
slice_ext_impl![mut: T, for [T; LEN], impl: T, const LEN: usize];
#[cfg(feature = "alloc")]
slice_ext_impl![mut: T, for Vec<T>, impl: T];
