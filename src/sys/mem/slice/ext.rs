// devela::sys::mem::slice::ext
//
//! Defines [`ExtSlice`].
//

use super::Slice;
#[cfg(feature = "alloc")]
use crate::data::Vec;

/// Marker trait to prevent downstream implementations of the [`ExtSlice`] trait.
trait Sealed {}
impl<T> Sealed for [T] {}
impl<T> Sealed for &[T] {}
impl<T> Sealed for &mut [T] {}
impl<T, const LEN: usize> Sealed for [T; LEN] {}
#[cfg(feature = "alloc")]
impl<T> Sealed for Vec<T> {}

#[doc = crate::TAG_NAMESPACE!()]
/// Extension trait providing additional methods for [`&[T]`][slice].
///
/// This trait is sealed and cannot be implemented for any other type.
///
/// See also [`Slice`][crate::Slice] for *const* methods.
#[cfg_attr(nightly_doc, doc(notable_trait))]
#[expect(private_bounds, reason = "Sealed")]
pub trait ExtSlice<T>: Sealed {
    /* split */

    /// Returns a left subslice of `slice` with the given maximum `len`.
    ///
    /// If `left_len > slice.len()` it returns the full slice.
    ///
    /// See also [`Slice::lsplit`] for the standalone `const` version.
    #[must_use]
    fn slice_lsplit(&self, len: usize) -> &[T];

    /// Returns a right subslice of `slice` with the given maximum `len`.
    ///
    /// If `left_len > slice.len()` it returns the full slice.
    ///
    /// See also [`Slice::rsplit`] for the standalone `const` version.
    #[must_use]
    fn slice_rsplit(&self, len: usize) -> &[T];

    /// Returns a middle subslice of `slice` with the given maximum `len`
    /// and a left bias.
    ///
    /// In case of a non-perfect middle split, it will have one character more
    /// on the left.
    ///
    /// If `len > slice.len()` returns the full `slice`.
    ///
    /// See also [`Slice::msplit_left`] for the standalone `const` version.
    #[must_use]
    fn slice_msplit_left(&self, len: usize) -> &[T];

    /// Returns a middle subslice of `slice` with the given maximum `len`
    /// and a right bias.
    ///
    /// In case of a non-perfect middle split, it will have one character more
    /// on the right.
    ///
    /// If `len > slice.len()` returns the full `slice`.
    ///
    /// See also [`Slice::msplit_right`] for the standalone `const` version.
    #[must_use]
    fn slice_msplit_right(&self, len: usize) -> &[T];

    /* convert */

    /// Converts `&[T]` to `[U; N]` when `U` implements `From<T>`.
    ///
    /// # Panics
    /// Panics if the length of the slice is less than the length of the array.
    /// # Examples
    /// ```
    /// # use devela::ExtSlice;
    /// assert_eq![[1_u16, 2, 3], [1_u8, 2, 3].slice_into_array::<u16, 3>()];
    /// assert_eq![[1_u16, 2, 3], [1_u8, 2, 3].slice_into_array::<u16, 3>()];
    /// ```
    /// # Features
    /// If the `unsafe_array` feature is enabled it uses `MaybeUninit` to improve performance.
    #[must_use]
    // IMPROVE make a try_slice_into_array version:
    // WAIT: [try_array_from_fn](https://github.com/rust-lang/rust/issues/89379)
    // - https://doc.rust-lang.org/nightly/core/array/fn.try_from_fn.html
    fn slice_into_array<U, const N: usize>(&self) -> [U; N]
    where
        T: Clone,
        U: From<T>;

    /// Converts `&[T]` to `Vec<U>` when `U` implements `From<T>`.
    /// # Examples
    /// ```
    /// # use devela::ExtSlice;
    /// assert_eq![vec![1_i16, 2, 3], [1_u8, 2, 3].slice_into_vec::<i16>()];
    /// assert_eq![vec![1_i16, 2, 3], [1_u8, 2, 3].slice_into_vec::<i16>()];
    /// ```
    #[must_use]
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    fn slice_into_vec<U>(&self) -> Vec<U>
    where
        T: Clone,
        U: From<T>;

    /// Tries to convert `&[T]` to `Vec<U>` when `U` implements `TryFrom<T>`.
    /// # Examples
    /// ```
    /// # use devela::ExtSlice;
    /// assert_eq![Ok(vec![1_i32, 2, 3]), [1_i64, 2, 3].slice_try_into_vec()];
    /// assert_eq![Ok(vec![1_i32, 2, 3]), [1_i64, 2, 3].slice_try_into_vec::<_, i32>()];
    /// ```
    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    fn slice_try_into_vec<E, U>(&self) -> Result<Vec<U>, E>
    where
        T: Clone,
        U: TryFrom<T, Error = E>;
}

#[doc = crate::TAG_NAMESPACE!()]
/// Extension trait providing additional methods for [`&mut [T]`][slice].
///
/// This trait is sealed and cannot be implemented for any other type.
///
/// See also [`Slice`][crate::Slice] for *const* methods.
#[cfg_attr(nightly_doc, doc(notable_trait))]
pub trait ExtSliceMut<T>: ExtSlice<T> {
    /* split */

    /// Returns a mutable left subslice of `slice` with the given maximum `len`.
    ///
    /// If `left_len > slice.len()` it returns the full slice.
    #[must_use]
    fn slice_lsplit_mut(&mut self, len: usize) -> &mut [T];

    /// Returns a mutable right subslice of `slice` with the given maximum `len`.
    ///
    /// If `left_len > slice.len()` it returns the full slice.
    #[must_use]
    fn slice_rsplit_mut(&mut self, len: usize) -> &mut [T];

    /// Returns a mutable middle subslice of `slice` with the given maximum `len`
    /// and a left bias.
    ///
    /// In case of a non-perfect middle split, it will have one character more
    /// on the left.
    ///
    /// If `len > slice.len()` returns the full `slice`.
    #[must_use]
    fn slice_msplit_left_mut(&mut self, len: usize) -> &mut [T];

    /// Returns a mutable middle subslice of `slice` with the given maximum `len`
    /// and a right bias.
    ///
    /// In case of a non-perfect middle split, it will have one character more
    /// on the right.
    ///
    /// If `len > slice.len()` returns the full `slice`.
    #[must_use]
    fn slice_msplit_right_mut(&mut self, len: usize) -> &mut [T];
}

macro_rules! impl_ext_slice {
    ($t:ty, for $for:ty, impl: $($impl:tt)*) => {
        impl<$($impl)*> ExtSlice<$t> for $for {
            /* split */

            fn slice_lsplit(&self, len: usize) -> &[T] { Slice::lsplit(self, len) }
            fn slice_rsplit(&self, len: usize) -> &[T] { Slice::rsplit(self, len) }
            fn slice_msplit_left(&self, len: usize) -> &[T] { Slice::msplit_left(self, len) }
            fn slice_msplit_right(&self, len: usize) -> &[T] { Slice::msplit_right(self, len) }

            /* collection */

            fn slice_into_array<U, const N: usize>(&self) -> [U; N] where T: Clone, U: From<T> {
                if self.len() >= N {
                    #[cfg(any(feature = "safe_data", not(feature = "unsafe_array")))]
                    {
                        let mut array: [U; N] = crate::array_from_fn(|i| U::from(self[i].clone()));
                        for (i, item) in self.iter().take(N).enumerate() {
                            array[i] = U::from(item.clone());
                        }
                        array
                    }
                    // SAFETY: we make sure of initializing every array element
                    #[cfg(all(not(feature = "safe_data"), feature = "unsafe_array"))]
                    {
                        use crate::MaybeUninit;
                        let mut array: [MaybeUninit<U>; N] =
                            unsafe { MaybeUninit::uninit().assume_init() };
                        for i in 0..N { array[i] = MaybeUninit::new(U::from(self[i].clone())); }
                        array.map(|x| unsafe { x.assume_init() })
                    }
                } else {
                    panic!("Slice length is less than the requested array size")
                }
            }
            #[cfg(feature = "alloc")]
            fn slice_into_vec<U>(&self) -> Vec<U> where T: Clone, U: From<T> {
                self.iter().map(|t| U::from(t.clone())).collect::<Vec<_>>().into_iter().collect()
            }
            #[cfg(feature = "alloc")]
            fn slice_try_into_vec<E, U>(&self) -> Result<Vec<U>, E>
                where T: Clone, U: TryFrom<T, Error = E> {
                    self
                        // 1. Vec<Result<_>>:
                        .iter()
                        .map(|t| U::try_from(t.clone()))
                        .collect::<Vec<_>>()
                        // 2. Result<Vec<_>>:
                        .into_iter()
                        .collect::<Result<Vec<_>, _>>()
            }
        }
    };
    (mut: $t:ty, for $for:ty, impl: $($impl:tt)*) => {
        impl_ext_slice![$t, for $for, impl: $($impl)*];

        impl<$($impl)*> ExtSliceMut<$t> for $for {
            /* split */

            fn slice_lsplit_mut(&mut self, len: usize) -> &mut [T] { Slice::lsplit_mut(self, len) }
            fn slice_rsplit_mut(&mut self, len: usize) -> &mut [T] { Slice::rsplit_mut(self, len) }
            fn slice_msplit_left_mut(&mut self, len: usize) -> &mut [T] {
                Slice::msplit_left_mut(self, len) }
            fn slice_msplit_right_mut(&mut self, len: usize) -> &mut [T] {
                Slice::msplit_right_mut(self, len) }
        }
    };
}
impl_ext_slice![mut: T, for [T], impl: T];
impl_ext_slice![T, for &[T], impl: T];
impl_ext_slice![mut: T, for &mut [T], impl: T];
impl_ext_slice![mut: T, for [T; LEN], impl: T, const LEN: usize];
#[cfg(feature = "alloc")]
impl_ext_slice![mut: T, for Vec<T>, impl: T];
