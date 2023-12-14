// devela::mem::slice::ext
//
//!
//

#[cfg(feature = "alloc")]
use _alloc::vec::Vec;

use super::fns::*;
use crate::data::cmp::*;

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
}

/// Extension trait providing additional methods for [`&mut [T]`][slice].
///
/// This trait is sealed and cannot be implemented for any other type.
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

    /* sort */

    /// Sorts `self` using bubble sort.
    fn sort_bubble(&mut self)
    where
        T: Ord;

    /// Sorts a `slice` using counting sort, and returns the ordered frequencies.
    ///
    /// Counting sort is particularly efficient when the range of input values is
    /// small compared to the number of elements to be sorted.
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
    fn sort_counting(&mut self) -> Vec<usize>
    where
        T: Ord + Clone;

    /// Sorts `self` using counting sort, and writes the frequencies, without allocating.
    ///
    /// Counting sort is particularly efficient when the range of input values is
    /// small compared to the number of elements to be sorted.
    ///
    /// This implementation makes the following assumptions:
    /// - `values` contains all distinct values present in `self`.
    /// - `freq` and `values` are of the same length.
    /// - `freq` only contains zeros.
    ///
    /// Returns `None` if `values` does not contain a value present in `self`,
    /// or if `self` has more elements than `freq` can accommodate.
    ///
    /// Note that the frequencies in `freq` will be in the order of the sorted
    /// distinct elements in `values`.
    #[must_use]
    fn sort_counting_buf(&mut self, freq: &mut [T], values: &[T]) -> Option<()>
    where
        T: Ord + Clone + TryInto<usize> + TryFrom<usize>;

    /// Sorts `self` using insertion sort.
    fn sort_insertion(&mut self)
    where
        T: Ord;

    /// Sorts `self` using merge sort.
    ///
    /// It allocates one vector for the entire sort operation.
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
    fn sort_merge(&mut self)
    where
        T: Ord + Copy;

    /// Sorts `self` using quick sort with the Lomuto partition scheme.
    ///
    /// It performs more swaps compared to the Hoare partition scheme.
    fn sort_quick_lomuto(&mut self)
    where
        T: Ord;

    /// Sorts `self` using quick sort with the Three way partition scheme.
    ///
    /// It is more efficient when dealing with duplicate elements.
    fn sort_quick_3way(&mut self)
    where
        T: Ord + Clone;

    /// Sorts `self` using quick sort with the Hoare partition scheme.
    ///
    /// It performs fewer swaps compared to the Lomuto partition scheme.
    fn sort_quick_hoare(&mut self)
    where
        T: Ord + Clone;

    /// Sorts `self` using selection sort.
    fn sort_selection(&mut self)
    where
        T: Ord;

    /// Sorts `self` using shaker sort.
    ///
    /// Also known as cocktail sort and double quicksort.
    fn sort_shaker(&mut self)
    where
        T: Ord + Clone;
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
            /* split */

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
            /* sort */

            fn sort_bubble(&mut self) where T: Ord {
                sort_bubble(self)
            }
            #[cfg(feature = "alloc")]
            fn sort_counting(&mut self) -> Vec<usize> where T: Ord + Clone {
                sort_counting(self)
            }
            fn sort_counting_buf(&mut self, freq: &mut[T], values: &[T]) -> Option<()>
                where T: Ord + Clone + TryInto<usize> + TryFrom<usize> {
                sort_counting_buf(self, freq, values)
            }
            fn sort_insertion(&mut self) where T: Ord {
                sort_insertion(self)
            }
            #[cfg(feature = "alloc")]
            fn sort_merge(&mut self) where T: Ord + Copy {
                sort_merge(self)
            }
            fn sort_quick_lomuto(&mut self) where T: Ord {
                sort_quick_lomuto(self)
            }
            fn sort_quick_3way(&mut self) where T: Ord + Clone {
                sort_quick_3way(self)
            }
            fn sort_quick_hoare(&mut self) where T: Ord + Clone {
                sort_quick_hoare(self)
            }
            fn sort_selection(&mut self) where T: Ord {
                sort_selection(self)
            }
            fn sort_shaker(&mut self) where T: Ord + Clone {
                sort_shaker(self)
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
