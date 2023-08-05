// revela::slice
//
//! Slice, extends [`alloc::slice`].
//

use crate::cmp::{clamp_usize, min_usize};

/// Extension trait providing additional methods for slices [`[T]`][slice].
pub trait SliceExt<T> {
    /// Returns a left subslice of `slice` with the given maximum `len`.
    ///
    /// If `left_len > slice.len()` it returns the full slice.
    ///
    /// See also [`slice_lsplit`] for the standalone `const` version.
    fn lsplit(&self, len: usize) -> &[T];

    /// Returns a mutable left subslice of `slice` with the given maximum `len`.
    ///
    /// If `left_len > slice.len()` it returns the full slice.
    fn lsplit_mut(&mut self, len: usize) -> &mut [T];

    /// Returns a right subslice of `slice` with the given maximum `len`.
    ///
    /// If `left_len > slice.len()` it returns the full slice.
    ///
    /// See also [`slice_rsplit`] for the standalone `const` version.
    fn rsplit(&self, len: usize) -> &[T];

    /// Returns a mutable right subslice of `slice` with the given maximum `len`.
    ///
    /// If `left_len > slice.len()` it returns the full slice.
    fn rsplit_mut(&mut self, len: usize) -> &mut [T];

    /// Returns a middle subslice of `slice` with the given maximum `len` and a left bias.
    ///
    /// In case of a non-perfect middle split, it will have one character more on the left.
    ///
    /// If `len > slice.len()` returns the full `slice`.
    ///
    /// See also [`slice_msplit_left`] for the standalone `const` version.
    fn msplit_left(&self, len: usize) -> &[T];

    /// Returns a mutable middle subslice of `slice` with the given maximum `len` and a left bias.
    ///
    /// In case of a non-perfect middle split, it will have one character more on the left.
    ///
    /// If `len > slice.len()` returns the full `slice`.
    fn msplit_left_mut(&mut self, len: usize) -> &mut [T];

    /// Returns a middle subslice of `slice` with the given maximum `len` and a right bias.
    ///
    /// In case of a non-perfect middle split, it will have one character more on the right.
    ///
    /// If `len > slice.len()` returns the full `slice`.
    ///
    /// See also [`slice_msplit_right`] for the standalone `const` version.
    fn msplit_right(&self, len: usize) -> &[T];

    /// Returns a mutable middle subslice of `slice` with the given maximum `len` and a right bias.
    ///
    /// In case of a non-perfect middle split, it will have one character more on the right.
    ///
    /// If `len > slice.len()` returns the full `slice`.
    fn msplit_right_mut(&mut self, len: usize) -> &mut [T];
}

impl<T> SliceExt<T> for [T] {
    #[inline]
    fn lsplit(&self, len: usize) -> &[T] {
        slice_lsplit(self, len)
    }
    #[inline]
    fn lsplit_mut(&mut self, len: usize) -> &mut [T] {
        slice_lsplit_mut(self, len)
    }
    #[inline]
    fn rsplit(&self, len: usize) -> &[T] {
        slice_rsplit(self, len)
    }
    #[inline]
    fn rsplit_mut(&mut self, len: usize) -> &mut [T] {
        slice_rsplit_mut(self, len)
    }
    #[inline]
    fn msplit_left(&self, len: usize) -> &[T] {
        slice_msplit_left(self, len)
    }
    #[inline]
    fn msplit_left_mut(&mut self, len: usize) -> &mut [T] {
        slice_msplit_left_mut(self, len)
    }
    #[inline]
    fn msplit_right(&self, len: usize) -> &[T] {
        slice_msplit_right(self, len)
    }
    #[inline]
    fn msplit_right_mut(&mut self, len: usize) -> &mut [T] {
        slice_msplit_right_mut(self, len)
    }
}

/// Returns a left subslice of `slice` with the given maximum `len`.
///
/// If `left_len > slice.len()` it returns the full slice.
///
/// # Examples
/// ```
/// use devela::slice::slice_lsplit;
///
/// let v = [1, 2, 3, 4, 5, 6];
///
/// assert_eq!(slice_lsplit(&v, 3), &[1, 2, 3]);
/// assert_eq!(slice_lsplit(&v, 0), &[]);
/// assert_eq!(slice_lsplit(&v, 10), &[1, 2, 3, 4, 5, 6]);
/// ```
#[inline]
pub const fn slice_lsplit<T>(slice: &[T], len: usize) -> &[T] {
    let end_idx = clamp_usize(len, 0, slice.len());
    let (left, _) = slice.split_at(end_idx);
    left
}

/// Returns a mutable left subslice of `slice` with the given maximum `len`.
///
/// If `left_len > slice.len()` it returns the full slice.
///
/// # Examples
/// ```
/// use devela::slice::slice_lsplit_mut;
///
/// let mut v = [1, 2, 3, 4, 5, 6];
///
/// assert_eq!(slice_lsplit_mut(&mut v, 3), &mut [1, 2, 3]);
/// assert_eq!(slice_lsplit_mut(&mut v, 0), &mut []);
/// assert_eq!(slice_lsplit_mut(&mut v, 10), &mut [1, 2, 3, 4, 5, 6]);
/// ```
#[inline]
pub fn slice_lsplit_mut<T>(slice: &mut [T], len: usize) -> &mut [T] {
    let end_idx = clamp_usize(len, 0, slice.len());
    let (left, _) = slice.split_at_mut(end_idx);
    left
}

/// Returns a right subslice of `slice` with the given maximum `len`.
///
/// If `left_len > slice.len()` it returns the full slice.
///
/// # Examples
/// ```
/// use devela::slice::slice_rsplit;
///
/// let v = [1, 2, 3, 4, 5, 6];
///
/// assert_eq!(slice_rsplit(&v, 3), &[4, 5, 6]);
/// assert_eq!(slice_rsplit(&v, 0), &[]);
/// assert_eq!(slice_rsplit(&v, 10), &[1, 2, 3, 4, 5, 6]);
/// ```
#[inline]
pub const fn slice_rsplit<T>(slice: &[T], len: usize) -> &[T] {
    let start_idx = slice.len().saturating_sub(len);
    let (_, right) = slice.split_at(start_idx);
    right
}

/// Returns a mutable right subslice of `slice` with the given maximum `len`.
///
/// If `left_len > slice.len()` it returns the full slice.
///
/// # Examples
/// ```
/// use devela::slice::slice_rsplit_mut;
///
/// let mut v = [1, 2, 3, 4, 5, 6];
///
/// assert_eq!(slice_rsplit_mut(&mut v, 3), &mut [4, 5, 6]);
/// assert_eq!(slice_rsplit_mut(&mut v, 0), &mut []);
/// assert_eq!(slice_rsplit_mut(&mut v, 10), &mut [1, 2, 3, 4, 5, 6]);
/// ```
#[inline]
pub fn slice_rsplit_mut<T>(slice: &mut [T], len: usize) -> &mut [T] {
    let start_idx = slice.len().saturating_sub(len);
    let (_, right) = slice.split_at_mut(start_idx);
    right
}

/// Returns a middle subslice of `slice` with the given maximum `len` and a right bias.
///
/// In case of a non-perfect middle split, it will have one character more on the right.
///
/// If `len > slice.len()` returns the full `slice`.
///
/// # Examples
/// ```
/// use devela::slice::slice_msplit_right;
///
/// let v = [1, 2, 3, 4, 5, 6];
///
/// assert_eq!(slice_msplit_right(&v, 0), &[]);
/// assert_eq!(slice_msplit_right(&v, 1), &[4]);
/// assert_eq!(slice_msplit_right(&v, 2), &[3, 4]);
/// assert_eq!(slice_msplit_right(&v, 3), &[3, 4, 5]);
/// assert_eq!(slice_msplit_right(&v, 4), &[2, 3, 4, 5]);
/// assert_eq!(slice_msplit_right(&v, 5), &[2, 3, 4, 5, 6]);
/// assert_eq!(slice_msplit_right(&v, 10), &[1, 2, 3, 4, 5, 6]);
/// ```
///
/// See also [`slice_msplit_left`].
#[inline]
pub const fn slice_msplit_right<T>(slice: &[T], len: usize) -> &[T] {
    let mid_idx = slice.len() / 2;
    let half_len = len / 2;
    let start_idx = mid_idx.saturating_sub(half_len);
    let end_idx = min_usize(mid_idx + half_len + (len % 2), slice.len());
    let (_, right) = slice.split_at(start_idx);
    let (middle, _) = right.split_at(end_idx - start_idx);
    middle
}

/// Returns a mutable middle subslice of `slice` with the given maximum `len` and a right bias.
///
/// In case of a non-perfect middle split, it will have one character more on the right.
///
/// If `len > slice.len()` returns the full `slice`.
///
/// # Examples
/// ```
/// use devela::slice::slice_msplit_right_mut;
///
/// let mut v = [1, 2, 3, 4, 5, 6];
///
/// assert_eq!(slice_msplit_right_mut(&mut v, 0), &mut []);
/// assert_eq!(slice_msplit_right_mut(&mut v, 1), &mut [4]);
/// assert_eq!(slice_msplit_right_mut(&mut v, 2), &mut [3, 4]);
/// assert_eq!(slice_msplit_right_mut(&mut v, 3), &mut [3, 4, 5]);
/// assert_eq!(slice_msplit_right_mut(&mut v, 4), &mut [2, 3, 4, 5]);
/// assert_eq!(slice_msplit_right_mut(&mut v, 5), &mut [2, 3, 4, 5, 6]);
/// assert_eq!(slice_msplit_right_mut(&mut v, 10), &mut [1, 2, 3, 4, 5, 6]);
/// ```
///
/// See also [`slice_msplit_left_mut`].
#[inline]
pub fn slice_msplit_right_mut<T>(slice: &mut [T], len: usize) -> &mut [T] {
    let mid_idx = slice.len() / 2;
    let half_len = len / 2;
    let start_idx = mid_idx.saturating_sub(half_len);
    let end_idx = min_usize(mid_idx + half_len + (len % 2), slice.len());
    let (_, right) = slice.split_at_mut(start_idx);
    let (middle, _) = right.split_at_mut(end_idx - start_idx);
    middle
}

/// Returns a middle subslice of `slice` with the given maximum `len` and a left bias.
///
/// In case of a non-perfect middle split, it will have one character more on the left.
///
/// If `len > slice.len()` returns the full `slice`.
///
/// # Examples
/// ```
/// use devela::slice::slice_msplit_left;
///
/// let v = [1, 2, 3, 4, 5, 6];
///
/// assert_eq!(slice_msplit_left(&v, 0), &[]);
/// assert_eq!(slice_msplit_left(&v, 1), &[3]);
/// assert_eq!(slice_msplit_left(&v, 2), &[3, 4]);
/// assert_eq!(slice_msplit_left(&v, 3), &[2, 3, 4]);
/// assert_eq!(slice_msplit_left(&v, 4), &[2, 3, 4, 5]);
/// assert_eq!(slice_msplit_left(&v, 5), &[1, 2, 3, 4, 5]);
/// assert_eq!(slice_msplit_left(&v, 10), &[1, 2, 3, 4, 5, 6]);
/// ```
///
/// See also [`slice_msplit_right`].
pub const fn slice_msplit_left<T>(slice: &[T], len: usize) -> &[T] {
    let mid_idx = slice.len() / 2;
    let half_len = len / 2;
    let start_idx = mid_idx.saturating_sub(half_len + (len % 2));
    let end_idx = min_usize(mid_idx + half_len, slice.len());
    let (_, right) = slice.split_at(start_idx);
    let (middle, _) = right.split_at(end_idx - start_idx);
    middle
}

/// Returns a mutable middle subslice of `slice` with the given maximum `len` and a left bias.
///
/// In case of a non-perfect middle split, it will have one character more on the left.
///
/// If `len > slice.len()` returns the full `slice`.
///
/// # Examples
/// ```
/// use devela::slice::slice_msplit_left_mut;
///
/// let mut v = [1, 2, 3, 4, 5, 6];
///
/// assert_eq!(slice_msplit_left_mut(&mut v, 0), &mut []);
/// assert_eq!(slice_msplit_left_mut(&mut v, 1), &mut [3]);
/// assert_eq!(slice_msplit_left_mut(&mut v, 2), &mut [3, 4]);
/// assert_eq!(slice_msplit_left_mut(&mut v, 3), &mut [2, 3, 4]);
/// assert_eq!(slice_msplit_left_mut(&mut v, 4), &mut [2, 3, 4, 5]);
/// assert_eq!(slice_msplit_left_mut(&mut v, 5), &mut [1, 2, 3, 4, 5]);
/// assert_eq!(slice_msplit_left_mut(&mut v, 10), &mut [1, 2, 3, 4, 5, 6]);
/// ```
///
/// See also [`slice_msplit_right`].
pub fn slice_msplit_left_mut<T>(slice: &mut [T], len: usize) -> &mut [T] {
    let mid_idx = slice.len() / 2;
    let half_len = len / 2;
    let start_idx = mid_idx.saturating_sub(half_len + (len % 2));
    let end_idx = min_usize(mid_idx + half_len, slice.len());
    let (_, right) = slice.split_at_mut(start_idx);
    let (middle, _) = right.split_at_mut(end_idx - start_idx);
    middle
}
