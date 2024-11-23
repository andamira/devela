// devela::mem::slice::wrapper
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
/// See also: [`ExtSlice`][crate::ExtSlice], [`Mem`][crate::Mem], [`Ptr`][crate::Ptr].
#[repr(transparent)]
pub struct Slice<T>(crate::PhantomData<T>);

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
    /// See also `Ptr::`[slice_from_raw_parts`][crate::Ptr::slice_from_raw_parts].
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
    /// See also `Ptr::`[slice_from_raw_parts_mut`][crate::Ptr::slice_from_raw_parts_mut].
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_slice")))]
    #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_slice"))]
    // WAIT:1.83 [const_slice_from_raw_parts_mut](https://github.com/rust-lang/rust/issues/67456)
    pub unsafe fn from_raw_parts_mut<'a>(data: *mut T, len: usize) -> &'a mut [T] {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { from_raw_parts_mut(data, len) }
    }

    /* left split */

    /// Returns the leftmost sub-`slice` with the given maximum `len`.
    ///
    /// If `len > self.len()` it returns the full slice.
    ///
    /// # Examples
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
    /// # Examples
    /// ```
    /// # use devela::Slice;
    /// let mut v = [1, 2, 3, 4, 5, 6];
    /// assert_eq!(Slice::lsplit_mut(&mut v, 3), &mut [1, 2, 3]);
    /// assert_eq!(Slice::lsplit_mut(&mut v, 0), &mut [] as &mut [i32]);
    /// assert_eq!(Slice::lsplit_mut(&mut v, 10), &mut [1, 2, 3, 4, 5, 6]);
    /// ```
    #[must_use]
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
    /// # Examples
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
    /// # Examples
    /// ```
    /// # use devela::Slice;
    /// let mut v = [1, 2, 3, 4, 5, 6];
    /// assert_eq!(Slice::rsplit_mut(&mut v, 3), &mut [4, 5, 6]);
    /// assert_eq!(Slice::rsplit_mut(&mut v, 0), &mut [] as &mut [i32]);
    /// assert_eq!(Slice::rsplit_mut(&mut v, 10), &mut [1, 2, 3, 4, 5, 6]);
    /// ```
    #[must_use]
    pub fn rsplit_mut(slice: &mut [T], len: usize) -> &mut [T] {
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
    /// # Examples
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
    /// # Examples
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
    /// See also [`Slice::msplit_right`].
    #[must_use]
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
    /// # Examples
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
    /// # Examples
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
    #[must_use]
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
    pub fn replace_leading_bytes(slice: &mut [u8], old: u8, new: u8) {
        let mut start = 0;
        while start < slice.len() && slice[start] == old {
            slice[start] = new;
            start += 1;
        }
    }
}
