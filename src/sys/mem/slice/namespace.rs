// devela::sys::mem::slice::wrapper
//
//! `Slice` namespace.
//

#[cfg(feature = "_cmp_usize")]
use crate::Compare;
use core::slice::{from_mut, from_ref};
#[allow(unused_imports, reason = "unsafe feature-gated")]
use core::slice::{from_raw_parts, from_raw_parts_mut};

/// Slice-related operations.
///
/// It is designed as a utility namespace and does not hold or wrap data itself.
/// Instead, it operates on slices provided directly as arguments to its static methods.
///
// TODO: # Methods
// - namespaced from core::slice.
// - shared subslicing.
// - exclusive subslicing.
// - splitting.
/// See also: [`ExtSlice`][crate::ExtSlice], [`Mem`][crate::Mem], [`Ptr`][crate::Ptr].
pub struct Slice<T>(crate::PhantomData<T>);

/// # `core::slice` namespaced methods
impl<T> Slice<T> {
    /// Converts a reference to `T` into a slice of length 1 (without copying).
    ///
    /// See `core::slice::`[`from_ref`].
    #[must_use]
    pub const fn from_ref(s: &T) -> &[T] {
        from_ref(s)
    }

    /// Converts a reference to `T` into a slice of length 1 (without copying).
    ///
    /// See `core::slice::`[`from_mut`].
    #[must_use]
    pub fn from_mut(s: &mut T) -> &mut [T] {
        from_mut(s)
    }

    /// Forms a shared slice from a pointer and a length.
    ///
    /// # Safety
    /// See `core::slice::`[`from_raw_parts`]
    ///
    /// See also `Ptr::`[`slice_from_raw_parts`][crate::Ptr::slice_from_raw_parts].
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_slice")))]
    #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_slice"))]
    pub const unsafe fn from_raw_parts<'a>(data: *const T, len: usize) -> &'a [T] {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { from_raw_parts(data, len) }
    }

    /// Forms an exclusive slice from a pointer and a length.
    ///
    /// # Safety
    /// See `core::slice::`[`from_raw_parts_mut`].
    ///
    /// See also `Ptr::`[`slice_from_raw_parts_mut`][crate::Ptr::slice_from_raw_parts_mut].
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_slice")))]
    #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_slice"))]
    pub const unsafe fn from_raw_parts_mut<'a>(data: *mut T, len: usize) -> &'a mut [T] {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { from_raw_parts_mut(data, len) }
    }
}

/// # Methods for shared subslicing using index-based splitting.
impl<T> Slice<T> {
    /* panicking */

    /// Returns a subslice up to the given `end` index.
    ///
    /// Equivalent to `&slice[..end]`.
    ///
    /// # Panics
    /// Panics if `end` > `slice.len()`.
    #[must_use]
    pub const fn range_to(slice: &[T], end: usize) -> &[T] {
        slice.split_at(end).0
    }
    /// Returns a subslice starting from the given `start` index.
    ///
    /// Equivalent to `&slice[start..]`.
    ///
    /// # Panics
    /// Panics if `start` > `slice.len()`.
    #[must_use]
    pub const fn range_from(slice: &[T], start: usize) -> &[T] {
        slice.split_at(start).1
    }
    /// Returns a subslice from `start` (inclusive) to `end` (exclusive).
    ///
    /// Equivalent to `&slice[start..end]`.
    ///
    /// # Panics
    /// Panics if `start` > len or `end` > `slice.len()`.
    #[must_use]
    pub const fn range(slice: &[T], start: usize, end: usize) -> &[T] {
        slice.split_at(start).1.split_at(end - start).0
    }

    /// Returns the first `n` elements of the slice.
    ///
    /// Equivalent to `&slice[..n]`.
    ///
    /// # Panics
    /// Panics if `n` > `slice.len()`.
    #[must_use]
    pub const fn take_first(slice: &[T], n: usize) -> &[T] {
        slice.split_at(n).0
    }
    /// Returns the last `n` elements of the slice.
    ///
    /// Equivalent to `&slice[slice.len() - n..]`.
    ///
    /// # Panics
    /// Panics if `n` > `slice.len()`.
    #[must_use]
    pub const fn take_last(slice: &[T], n: usize) -> &[T] {
        slice.split_at(slice.len() - n).1
    }
    /// Returns the slice omitting the last `n` elements.
    ///
    /// Equivalent to `&slice[..slice.len() - n]`.
    ///
    /// # Panics
    /// Panics if `n` > `slice.len()`.
    #[must_use]
    pub const fn take_omit_last(slice: &[T], n: usize) -> &[T] {
        slice.split_at(slice.len() - n).0
    }

    /* Checked */

    /// Returns a subslice up to the given `end` index.
    ///
    /// Equivalent to `&slice[..end]`.
    ///
    /// Returns `None` if `end` > `slice.len()`.
    #[must_use]
    pub const fn range_to_checked(slice: &[T], end: usize) -> Option<&[T]> {
        match slice.split_at_checked(end) {
            Some((subslice, _)) => Some(subslice),
            None => None,
        }
    }
    /// Returns a subslice starting from the given `start` index.
    ///
    /// Equivalent to `&slice[start..]`.
    ///
    /// Returns `None` if `start` > `slice.len()`.
    #[must_use]
    pub const fn range_from_checked(slice: &[T], start: usize) -> Option<&[T]> {
        match slice.split_at_checked(start) {
            Some((_, subslice)) => Some(subslice),
            None => None,
        }
    }
    /// Returns a subslice from `start` (inclusive) to `end` (exclusive).
    ///
    /// Equivalent to `&slice[start..end]`.
    ///
    /// Returns `None` if `start` > `slice.len()` or `end` > `slice.len()`.
    ///
    /// # Features
    /// This method makes use of of the `unsafe_slice` feature is enabled.
    #[must_use]
    pub const fn range_checked(slice: &[T], start: usize, end: usize) -> Option<&[T]> {
        if start <= end && end <= slice.len() {
            #[cfg(any(feature = "safe_mem", not(feature = "unsafe_slice")))]
            return Some(slice.split_at(start).1.split_at(end - start).0);
            #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_slice"))]
            // SAFETY: `start` and `end` are checked to be within bounds and valid
            Some(unsafe { slice.split_at_unchecked(start).1.split_at_unchecked(end - start).0 })
        } else {
            None
        }
    }

    /// Returns the first `n` elements of the slice.
    ///
    /// Equivalent to `&slice[..n]`.
    ///
    /// Returns `None` if `n` > `slice.len()`.
    #[must_use]
    pub const fn take_first_checked(slice: &[T], n: usize) -> Option<&[T]> {
        match slice.split_at_checked(n) {
            Some((subslice, _)) => Some(subslice),
            None => None,
        }
    }
    /// Returns the last `n` elements of the slice.
    ///
    /// Equivalent to `&slice[slice.len() - n..]`.
    ///
    /// Returns `None` if `n` > `slice.len()`.
    ///
    /// # Features
    /// This method makes use of of the `unsafe_slice` feature is enabled.
    #[must_use]
    pub const fn take_last_checked(slice: &[T], n: usize) -> Option<&[T]> {
        match slice.len().checked_sub(n) {
            Some(index) => {
                #[cfg(any(feature = "safe_mem", not(feature = "unsafe_slice")))]
                return Some(slice.split_at(index).1);
                #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_slice"))]
                // SAFETY: `n` is checked to be within bounds and valid
                return Some(unsafe { slice.split_at_unchecked(index).1 });
            }
            None => None,
        }
    }
    /// Returns the slice omitting the last `n` elements.
    ///
    /// Equivalent to `&slice[..slice.len() - n]`.
    ///
    /// Returns `None` if `n` > `slice.len()`.
    ///
    /// # Features
    /// This method makes use of of the `unsafe_slice` feature is enabled.
    #[must_use]
    pub const fn take_omit_last_checked(slice: &[T], n: usize) -> Option<&[T]> {
        match slice.len().checked_sub(n) {
            Some(index) => {
                #[cfg(any(feature = "safe_mem", not(feature = "unsafe_slice")))]
                return Some(slice.split_at(index).0);
                #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_slice"))]
                // SAFETY: `n` is checked to be within bounds and valid
                return Some(unsafe { slice.split_at_unchecked(index).0 });
            }
            None => None,
        }
    }

    /* TODO Unchecked */

    /* TODO Saturating */
}

/// # Methods for exclusive subslicing using index-based splitting.
impl<T> Slice<T> {
    /* panicking */

    /// Returns a subslice up to the given `end` index.
    ///
    /// Equivalent to `&mut slice[..end]`.
    ///
    /// # Panics
    /// Panics if `end` > `slice.len()`.
    #[must_use]
    pub const fn range_to_mut(slice: &mut [T], end: usize) -> &mut [T] {
        slice.split_at_mut(end).0
    }
    /// Returns a subslice starting from the given `start` index.
    ///
    /// Equivalent to `&mut slice[start..]`.
    ///
    /// # Panics
    /// Panics if `start` > `slice.len()`.
    #[must_use]
    pub const fn range_from_mut(slice: &mut [T], start: usize) -> &mut [T] {
        slice.split_at_mut(start).1
    }
    /// Returns a subslice from `start` (inclusive) to `end` (exclusive).
    ///
    /// Equivalent to `&mut slice[start..end]`.
    ///
    /// # Panics
    /// Panics if `start` > `slice.len()` or `end` > `slice.len()`.
    #[must_use]
    pub const fn range_mut(slice: &mut [T], start: usize, end: usize) -> &mut [T] {
        slice.split_at_mut(start).1.split_at_mut(end - start).0
    }

    /// Returns the first `n` elements of the slice.
    ///
    /// Equivalent to `&mut slice[..n]`.
    ///
    /// # Panics
    /// Panics if `n` > `slice.len()`.
    #[must_use]
    pub const fn take_first_mut(slice: &mut [T], n: usize) -> &mut [T] {
        slice.split_at_mut(n).0
    }
    /// Returns the last `n` elements of the slice.
    ///
    /// Equivalent to `&mut slice[slice.len() - n..]`.
    ///
    /// # Panics
    /// Panics if `n` > `slice.len()`.
    #[must_use]
    pub const fn take_last_mut(slice: &mut [T], n: usize) -> &mut [T] {
        slice.split_at_mut(slice.len() - n).1
    }
    /// Returns the slice omitting the last `n` elements.
    ///
    /// Equivalent to `&mut slice[..slice.len() - n]`.
    ///
    /// # Panics
    /// Panics if `n` > `slice.len()`.
    #[must_use]
    pub const fn take_omit_last_mut(slice: &mut [T], n: usize) -> &[T] {
        slice.split_at_mut(slice.len() - n).0
    }

    /* Checked */

    /// Returns a subslice up to the given `end` index.
    ///
    /// Equivalent to `&mut slice[..end]`.
    ///
    /// Returns `None` if `end` > `slice.len()`.
    #[must_use]
    pub const fn range_to_mut_checked(slice: &mut [T], end: usize) -> Option<&mut [T]> {
        match slice.split_at_mut_checked(end) {
            Some((subslice, _)) => Some(subslice),
            None => None,
        }
    }
    /// Returns a subslice starting from the given `start` index.
    ///
    /// Equivalent to `&mut slice[start..]`.
    ///
    /// Returns `None` if `start` > `slice.len()`.
    #[must_use]
    pub const fn range_from_mut_checked(slice: &mut [T], start: usize) -> Option<&mut [T]> {
        match slice.split_at_mut_checked(start) {
            Some((_, subslice)) => Some(subslice),
            None => None,
        }
    }
    /// Returns a subslice from `start` (inclusive) to `end` (exclusive).
    ///
    /// Equivalent to `&mut slice[start..end]`.
    ///
    /// Returns `None` if `start` > `slice.len()` or `end` > `slice.len()`.
    ///
    /// # Features
    /// This method makes use of of the `unsafe_slice` feature is enabled.
    #[must_use]
    pub const fn range_mut_checked(slice: &mut [T], start: usize, end: usize) -> Option<&mut [T]> {
        if start <= end && end <= slice.len() {
            #[cfg(any(feature = "safe_mem", not(feature = "unsafe_slice")))]
            return Some(slice.split_at_mut(start).1.split_at_mut(end - start).0);
            #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_slice"))]
            // SAFETY: `start` and `end` are checked to be within bounds and valid
            Some(unsafe {
                slice.split_at_mut_unchecked(start).1.split_at_mut_unchecked(end - start).0
            })
        } else {
            None
        }
    }

    /// Returns the first `n` elements of the slice.
    ///
    /// Equivalent to `&mut slice[..n]`.
    ///
    /// Returns `None` if `n` > `slice.len()`.
    #[must_use]
    pub const fn take_first_mut_checked(slice: &mut [T], n: usize) -> Option<&mut [T]> {
        match slice.split_at_mut_checked(n) {
            Some((subslice, _)) => Some(subslice),
            None => None,
        }
    }
    /// Returns the last `n` elements of the slice.
    ///
    /// Equivalent to `&mut slice[slice.len() - n..]`.
    ///
    /// Returns `None` if `n` > `slice.len()`.
    ///
    /// # Features
    /// This method makes use of of the `unsafe_slice` feature is enabled.
    #[must_use]
    pub const fn take_last_mut_checked(slice: &mut [T], n: usize) -> Option<&mut [T]> {
        match slice.len().checked_sub(n) {
            Some(index) => {
                #[cfg(any(feature = "safe_mem", not(feature = "unsafe_slice")))]
                return Some(slice.split_at_mut(index).1);
                #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_slice"))]
                // SAFETY: `n` is checked to be within bounds and valid
                return Some(unsafe { slice.split_at_mut_unchecked(index).1 });
            }
            None => None,
        }
    }
    /// Returns the slice omitting the last `n` elements.
    ///
    /// Equivalent to `&mut slice[..slice.len() - n]`.
    ///
    /// Returns `None` if `n` > `slice.len()`.
    ///
    /// # Features
    /// This method makes use of of the `unsafe_slice` feature is enabled.
    #[must_use]
    pub const fn take_omit_last_mut_checked(slice: &mut [T], n: usize) -> Option<&mut [T]> {
        match slice.len().checked_sub(n) {
            Some(index) => {
                #[cfg(any(feature = "safe_mem", not(feature = "unsafe_slice")))]
                return Some(slice.split_at_mut(index).0);
                #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_slice"))]
                // SAFETY: `n` is checked to be within bounds and valid
                return Some(unsafe { slice.split_at_mut_unchecked(index).0 });
            }
            None => None,
        }
    }
}

/// # Methods for splitting.
impl<T> Slice<T> {
    /* left split */

    /// Returns the leftmost sub-`slice` with the given maximum `len`.
    ///
    /// If `len > self.len()` it returns the full slice.
    ///
    /// # Example
    /// ```
    /// # use devela::Slice;
    /// let v = [1, 2, 3, 4, 5, 6];
    /// assert_eq!(Slice::lsplit(&v, 3), &[1, 2, 3]);
    /// assert_eq!(Slice::lsplit(&v, 0), &[] as &[i32]);
    /// assert_eq!(Slice::lsplit(&v, 10), &[1, 2, 3, 4, 5, 6]);
    /// ```
    /// # Features
    /// This method will only be const if the `_cmp_usize` feature is enabled.
    // WAIT: [const_cmp](https://github.com/rust-lang/rust/issues/92391)
    #[must_use] #[cfg(feature = "_cmp_usize")] #[rustfmt::skip]
    pub const fn lsplit(slice: &[T], len: usize) -> &[T] {
        let end_idx = Compare(len).clamp(0, slice.len());
        let (left, _) = slice.split_at(end_idx);
        left
    }
    #[allow(missing_docs)]
    #[must_use] #[cfg(not(feature = "_cmp_usize"))] #[rustfmt::skip]
    pub fn lsplit(slice: &[T], len: usize) -> &[T] {
        let end_idx = len.clamp(0, slice.len());
        let (left, _) = slice.split_at(end_idx);
        left
    }

    /// Returns the leftmost exclusive sub-`slice` with the given maximum `len`.
    ///
    /// If `left_len > slice.len()` it returns the full slice.
    ///
    /// # Example
    /// ```
    /// # use devela::Slice;
    /// let mut v = [1, 2, 3, 4, 5, 6];
    /// assert_eq!(Slice::lsplit_mut(&mut v, 3), &mut [1, 2, 3]);
    /// assert_eq!(Slice::lsplit_mut(&mut v, 0), &mut [] as &mut [i32]);
    /// assert_eq!(Slice::lsplit_mut(&mut v, 10), &mut [1, 2, 3, 4, 5, 6]);
    /// ```
    /// See also [`Slice::lsplit_mut`].
    ///
    /// # Features
    /// This method will only be const if the `_cmp_usize` feature is enabled.
    #[must_use] #[cfg(feature = "_cmp_usize")] #[rustfmt::skip]
    pub const fn lsplit_mut(slice: &mut [T], len: usize) -> &mut [T] {
        let end_idx = Compare(len).clamp(0, slice.len());
        let (left, _) = slice.split_at_mut(end_idx);
        left
    }
    #[allow(missing_docs)]
    #[must_use] #[cfg(not(feature = "_cmp_usize"))] #[rustfmt::skip]
    pub fn lsplit_mut(slice: &mut [T], len: usize) -> &mut [T] {
        let end_idx = len.clamp(0, slice.len());
        let (left, _) = slice.split_at_mut(end_idx);
        left
    }

    /* right split */

    /// Returns the rightmost sub-`slice` with the given maximum `len`.
    ///
    /// If `left_len > slice.len()` it returns the full slice.
    ///
    /// # Example
    /// ```
    /// # use devela::Slice;
    /// let v = [1, 2, 3, 4, 5, 6];
    /// assert_eq!(Slice::rsplit(&v, 3), &[4, 5, 6]);
    /// assert_eq!(Slice::rsplit(&v, 0), &[] as &[i32]);
    /// assert_eq!(Slice::rsplit(&v, 10), &[1, 2, 3, 4, 5, 6]);
    /// ```
    #[must_use]
    pub const fn rsplit(slice: &[T], len: usize) -> &[T] {
        let start_idx = slice.len().saturating_sub(len);
        let (_, right) = slice.split_at(start_idx);
        right
    }

    /// Returns the rightmost mutable sub-`slice` with the given maximum `len`.
    ///
    /// If `left_len > slice.len()` it returns the full slice.
    ///
    /// # Example
    /// ```
    /// # use devela::Slice;
    /// let mut v = [1, 2, 3, 4, 5, 6];
    /// assert_eq!(Slice::rsplit_mut(&mut v, 3), &mut [4, 5, 6]);
    /// assert_eq!(Slice::rsplit_mut(&mut v, 0), &mut [] as &mut [i32]);
    /// assert_eq!(Slice::rsplit_mut(&mut v, 10), &mut [1, 2, 3, 4, 5, 6]);
    /// ```
    /// See also [`Slice::lsplit_mut`].
    ///
    /// # Features
    /// This method will only be const if the `_cmp_usize` feature is enabled.
    #[must_use] #[cfg(feature = "_cmp_usize")] #[rustfmt::skip]
    // WAIT:? [const_cmp](https://github.com/rust-lang/rust/issues/92391)
    pub const fn rsplit_mut(slice: &mut [T], len: usize) -> &mut [T] {
        let start_idx = slice.len().saturating_sub(len);
        let (_, right) = slice.split_at_mut(start_idx);
        right
    }
    #[allow(missing_docs)]
    #[must_use] #[cfg(not(feature = "_cmp_usize"))] #[rustfmt::skip]
    pub const fn rsplit_mut(slice: &mut [T], len: usize) -> &mut [T] {
        let start_idx = slice.len().saturating_sub(len);
        let (_, right) = slice.split_at_mut(start_idx);
        right
    }

    /* middle split left biased */

    /// Returns the middle sub-`slice` with the given maximum `len` and a left bias.
    ///
    /// In case of a non-perfect middle split, it will have one character more
    /// on the left.
    ///
    /// If `len > slice.len()` returns the full `slice`.
    ///
    /// # Example
    /// ```
    /// # use devela::Slice;
    /// let v = [1, 2, 3, 4, 5, 6];
    /// assert_eq!(Slice::msplit_left(&v, 0), &[] as &[i32]);
    /// assert_eq!(Slice::msplit_left(&v, 1), &[3]);
    /// assert_eq!(Slice::msplit_left(&v, 2), &[3, 4]);
    /// assert_eq!(Slice::msplit_left(&v, 3), &[2, 3, 4]);
    /// assert_eq!(Slice::msplit_left(&v, 4), &[2, 3, 4, 5]);
    /// assert_eq!(Slice::msplit_left(&v, 5), &[1, 2, 3, 4, 5]);
    /// assert_eq!(Slice::msplit_left(&v, 10), &[1, 2, 3, 4, 5, 6]);
    /// ```
    /// See also [`Slice::msplit_right`].
    ///
    /// # Features
    /// This method will only be const if the `_cmp_usize` feature is enabled.
    // WAIT:? [const_cmp](https://github.com/rust-lang/rust/issues/92391)
    #[must_use] #[cfg(feature = "_cmp_usize")] #[rustfmt::skip]
    pub const fn msplit_left(slice: &[T], len: usize) -> &[T] {
        let mid_idx = slice.len() / 2;
        let half_len = len / 2;
        let start_idx = mid_idx.saturating_sub(half_len + (len % 2));
        let end_idx = Compare(mid_idx + half_len).min(slice.len());
        let (_, right) = slice.split_at(start_idx);
        let (middle, _) = right.split_at(end_idx - start_idx);
        middle
    }
    #[allow(missing_docs)]
    #[must_use] #[cfg(not(feature = "_cmp_usize"))] #[rustfmt::skip]
    pub fn msplit_left(slice: &[T], len: usize) -> &[T] {
        let mid_idx = slice.len() / 2;
        let half_len = len / 2;
        let start_idx = mid_idx.saturating_sub(half_len + (len % 2));
        let end_idx = (mid_idx + half_len).min(slice.len());
        let (_, right) = slice.split_at(start_idx);
        let (middle, _) = right.split_at(end_idx - start_idx);
        middle
    }

    /// Returns the middle exclusive sub-`slice` with the given maximum `len` and a
    /// left bias.
    ///
    /// In case of a non-perfect middle split, it will have one character more
    /// on the left.
    ///
    /// If `len > slice.len()` returns the full `slice`.
    ///
    /// # Example
    /// ```
    /// # use devela::Slice;
    /// let mut v = [1, 2, 3, 4, 5, 6];
    /// assert_eq!(Slice::msplit_left_mut(&mut v, 0), &mut [] as &mut [i32]);
    /// assert_eq!(Slice::msplit_left_mut(&mut v, 1), &mut [3]);
    /// assert_eq!(Slice::msplit_left_mut(&mut v, 2), &mut [3, 4]);
    /// assert_eq!(Slice::msplit_left_mut(&mut v, 3), &mut [2, 3, 4]);
    /// assert_eq!(Slice::msplit_left_mut(&mut v, 4), &mut [2, 3, 4, 5]);
    /// assert_eq!(Slice::msplit_left_mut(&mut v, 5), &mut [1, 2, 3, 4, 5]);
    /// assert_eq!(Slice::msplit_left_mut(&mut v, 10), &mut [1, 2, 3, 4, 5, 6]);
    /// ```
    /// See also [`Slice::msplit_right_mut`].
    ///
    /// # Features
    /// This method will only be const if the `_cmp_usize` feature is enabled.
    // WAIT:? [const_cmp](https://github.com/rust-lang/rust/issues/92391)
    #[must_use] #[cfg(feature = "_cmp_usize")] #[rustfmt::skip]
    pub const fn msplit_left_mut(slice: &mut [T], len: usize) -> &mut [T] {
        let mid_idx = slice.len() / 2;
        let half_len = len / 2;
        let start_idx = mid_idx.saturating_sub(half_len + (len % 2));
        let end_idx = Compare(mid_idx + half_len).min(slice.len());
        let (_, right) = slice.split_at_mut(start_idx);
        let (middle, _) = right.split_at_mut(end_idx - start_idx);
        middle
    }
    #[allow(missing_docs)]
    #[must_use] #[cfg(not(feature = "_cmp_usize"))] #[rustfmt::skip]
    pub fn msplit_left_mut(slice: &mut [T], len: usize) -> &mut [T] {
        let mid_idx = slice.len() / 2;
        let half_len = len / 2;
        let start_idx = mid_idx.saturating_sub(half_len + (len % 2));
        let end_idx = (mid_idx + half_len).min(slice.len());
        let (_, right) = slice.split_at_mut(start_idx);
        let (middle, _) = right.split_at_mut(end_idx - start_idx);
        middle
    }

    /* middle split right biased */

    /// Returns the middle sub-`slice` with the given maximum `len` and a right bias.
    ///
    /// In case of a non-perfect middle split, it will have one character more
    /// on the right.
    ///
    /// If `len > slice.len()` returns the full `slice`.
    ///
    /// # Example
    /// ```
    /// # use devela::Slice;
    /// let v = [1, 2, 3, 4, 5, 6];
    /// assert_eq!(Slice::msplit_right(&v, 0), &[] as &[i32]);
    /// assert_eq!(Slice::msplit_right(&v, 1), &[4]);
    /// assert_eq!(Slice::msplit_right(&v, 2), &[3, 4]);
    /// assert_eq!(Slice::msplit_right(&v, 3), &[3, 4, 5]);
    /// assert_eq!(Slice::msplit_right(&v, 4), &[2, 3, 4, 5]);
    /// assert_eq!(Slice::msplit_right(&v, 5), &[2, 3, 4, 5, 6]);
    /// assert_eq!(Slice::msplit_right(&v, 10), &[1, 2, 3, 4, 5, 6]);
    /// ```
    /// See also [`Slice::msplit_left`].
    ///
    /// # Features
    /// This method will only be const if the `_cmp_usize` feature is enabled.
    #[must_use] #[cfg(feature = "_cmp_usize")] #[rustfmt::skip]
    // WAIT: [const_cmp](https://github.com/rust-lang/rust/issues/92391)
    pub const fn msplit_right(slice: &[T], len: usize) -> &[T] {
        let mid_idx = slice.len() / 2;
        let half_len = len / 2;
        let start_idx = mid_idx.saturating_sub(half_len);
        let end_idx = Compare(mid_idx + half_len + (len % 2)).min(slice.len());
        let (_, right) = slice.split_at(start_idx);
        let (middle, _) = right.split_at(end_idx - start_idx);
        middle
    }
    #[allow(missing_docs)]
    #[must_use] #[cfg(not(feature = "_cmp_usize"))] #[rustfmt::skip]
    pub fn msplit_right(slice: &[T], len: usize) -> &[T] {
        let mid_idx = slice.len() / 2;
        let half_len = len / 2;
        let start_idx = mid_idx.saturating_sub(half_len);
        let end_idx = (mid_idx + half_len + (len % 2)).min(slice.len());
        let (_, right) = slice.split_at(start_idx);
        let (middle, _) = right.split_at(end_idx - start_idx);
        middle
    }

    /// Returns the middle exclusive sub-`slice` with the given maximum `len` and a
    /// right bias.
    ///
    /// In case of a non-perfect middle split, it will have one character more
    /// on the right.
    ///
    /// If `len > slice.len()` returns the full `slice`.
    /// # Example
    /// ```
    /// # use devela::Slice;
    /// let mut v = [1, 2, 3, 4, 5, 6];
    /// assert_eq!(Slice::msplit_right_mut(&mut v, 0), &mut [] as &mut[i32]);
    /// assert_eq!(Slice::msplit_right_mut(&mut v, 1), &mut [4]);
    /// assert_eq!(Slice::msplit_right_mut(&mut v, 2), &mut [3, 4]);
    /// assert_eq!(Slice::msplit_right_mut(&mut v, 3), &mut [3, 4, 5]);
    /// assert_eq!(Slice::msplit_right_mut(&mut v, 4), &mut [2, 3, 4, 5]);
    /// assert_eq!(Slice::msplit_right_mut(&mut v, 5), &mut [2, 3, 4, 5, 6]);
    /// assert_eq!(Slice::msplit_right_mut(&mut v, 10), &mut [1, 2, 3, 4, 5, 6]);
    /// ```
    /// See also [`Slice::msplit_left_mut`].
    ///
    /// # Features
    /// This method will only be const if the `_cmp_usize` feature is enabled.
    // WAIT:? [const_cmp](https://github.com/rust-lang/rust/issues/92391)
    #[must_use] #[cfg(feature = "_cmp_usize")] #[rustfmt::skip]
    pub const fn msplit_right_mut(slice: &mut [T], len: usize) -> &mut [T] {
        let mid_idx = slice.len() / 2;
        let half_len = len / 2;
        let start_idx = mid_idx.saturating_sub(half_len);
        let end_idx = Compare(mid_idx + half_len + (len % 2)).min(slice.len());
        let (_, right) = slice.split_at_mut(start_idx);
        let (middle, _) = right.split_at_mut(end_idx - start_idx);
        middle
    }
    #[allow(missing_docs)]
    #[must_use] #[cfg(not(feature = "_cmp_usize"))] #[rustfmt::skip]
    pub fn msplit_right_mut(slice: &mut [T], len: usize) -> &mut [T] {
        let mid_idx = slice.len() / 2;
        let half_len = len / 2;
        let start_idx = mid_idx.saturating_sub(half_len);
        let end_idx = (mid_idx + half_len + (len % 2)).min(slice.len());
        let (_, right) = slice.split_at_mut(start_idx);
        let (middle, _) = right.split_at_mut(end_idx - start_idx);
        middle
    }
}

/// # Methods for slices of bytes.
impl Slice<u8> {
    /// Returns a subslice without the given leading `byte`s.
    #[must_use]
    pub const fn trim_leading_bytes(slice: &[u8], byte: u8) -> &[u8] {
        let mut start = 0;
        while start < slice.len() && slice[start] == byte {
            start += 1;
        }
        slice.split_at(start).1 // == &slice[start..]
    }

    /// Replaces the `old` leading byte with a `new` byte.
    pub const fn replace_leading_bytes(slice: &mut [u8], old: u8, new: u8) {
        let mut start = 0;
        while start < slice.len() && slice[start] == old {
            slice[start] = new;
            start += 1;
        }
    }
}
