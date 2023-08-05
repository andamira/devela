// revela::slice
//
//! Slice, extends [`alloc::slice`].
//

use crate::cmp::{clamp_usize, min_usize};

/// Returns a left subslice of `slice`, with the given maximum `len`.
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

/// Returns a right subslice of `slice`, with the given maximum `len`.
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

/// Returns a middle subslice of `slice`, with the given maximum `len`.
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

/// Returns a middle subslice of `slice`, with the given maximum `len`.
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
