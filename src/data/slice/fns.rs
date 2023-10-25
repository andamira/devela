// revela::data::fns
//
//!
//

use crate::ops::{clamp_usize, min_usize};

/// Returns the leftmost sub-`slice` with the given maximum `len`.
///
/// If `left_len > slice.len()` it returns the full slice.
///
/// # Examples
/// ```
/// use devela::data::slice_lsplit;
///
/// let v = [1, 2, 3, 4, 5, 6];
///
/// assert_eq!(slice_lsplit(&v, 3), &[1, 2, 3]);
/// assert_eq!(slice_lsplit(&v, 0), &[]);
/// assert_eq!(slice_lsplit(&v, 10), &[1, 2, 3, 4, 5, 6]);
/// ```
#[inline]
#[must_use]
pub const fn slice_lsplit<T>(slice: &[T], len: usize) -> &[T] {
    let end_idx = clamp_usize(len, 0, slice.len());
    let (left, _) = slice.split_at(end_idx);
    left
}

/// Returns the leftmost exclusive subslice with the given maximum `len`.
///
/// If `left_len > slice.len()` it returns the full slice.
///
/// # Examples
/// ```
/// use devela::data::slice_lsplit_mut;
///
/// let mut v = [1, 2, 3, 4, 5, 6];
///
/// assert_eq!(slice_lsplit_mut(&mut v, 3), &mut [1, 2, 3]);
/// assert_eq!(slice_lsplit_mut(&mut v, 0), &mut []);
/// assert_eq!(slice_lsplit_mut(&mut v, 10), &mut [1, 2, 3, 4, 5, 6]);
/// ```
#[inline]
#[must_use]
pub fn slice_lsplit_mut<T>(slice: &mut [T], len: usize) -> &mut [T] {
    let end_idx = clamp_usize(len, 0, slice.len());
    let (left, _) = slice.split_at_mut(end_idx);
    left
}

/// Returns the rightmost sub-`slice` with the given maximum `len`.
///
/// If `left_len > slice.len()` it returns the full slice.
///
/// # Examples
/// ```
/// use devela::data::slice_rsplit;
///
/// let v = [1, 2, 3, 4, 5, 6];
///
/// assert_eq!(slice_rsplit(&v, 3), &[4, 5, 6]);
/// assert_eq!(slice_rsplit(&v, 0), &[]);
/// assert_eq!(slice_rsplit(&v, 10), &[1, 2, 3, 4, 5, 6]);
/// ```
#[inline]
#[must_use]
pub const fn slice_rsplit<T>(slice: &[T], len: usize) -> &[T] {
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
/// use devela::data::slice_rsplit_mut;
///
/// let mut v = [1, 2, 3, 4, 5, 6];
///
/// assert_eq!(slice_rsplit_mut(&mut v, 3), &mut [4, 5, 6]);
/// assert_eq!(slice_rsplit_mut(&mut v, 0), &mut []);
/// assert_eq!(slice_rsplit_mut(&mut v, 10), &mut [1, 2, 3, 4, 5, 6]);
/// ```
#[inline]
#[must_use]
pub fn slice_rsplit_mut<T>(slice: &mut [T], len: usize) -> &mut [T] {
    let start_idx = slice.len().saturating_sub(len);
    let (_, right) = slice.split_at_mut(start_idx);
    right
}

/// Returns the middle sub-`slice` with the given maximum `len` and a right bias.
///
/// In case of a non-perfect middle split, it will have one character more
/// on the right.
///
/// If `len > slice.len()` returns the full `slice`.
///
/// # Examples
/// ```
/// use devela::data::slice_msplit_right;
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
#[must_use]
pub const fn slice_msplit_right<T>(slice: &[T], len: usize) -> &[T] {
    let mid_idx = slice.len() / 2;
    let half_len = len / 2;
    let start_idx = mid_idx.saturating_sub(half_len);
    let end_idx = min_usize(mid_idx + half_len + (len % 2), slice.len());
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
///
/// # Examples
/// ```
/// use devela::data::slice_msplit_right_mut;
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
#[must_use]
pub fn slice_msplit_right_mut<T>(slice: &mut [T], len: usize) -> &mut [T] {
    let mid_idx = slice.len() / 2;
    let half_len = len / 2;
    let start_idx = mid_idx.saturating_sub(half_len);
    let end_idx = min_usize(mid_idx + half_len + (len % 2), slice.len());
    let (_, right) = slice.split_at_mut(start_idx);
    let (middle, _) = right.split_at_mut(end_idx - start_idx);
    middle
}

/// Returns the middle sub-`slice` with the given maximum `len` and a left bias.
///
/// In case of a non-perfect middle split, it will have one character more
/// on the left.
///
/// If `len > slice.len()` returns the full `slice`.
///
/// # Examples
/// ```
/// use devela::data::slice_msplit_left;
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
#[must_use]
pub const fn slice_msplit_left<T>(slice: &[T], len: usize) -> &[T] {
    let mid_idx = slice.len() / 2;
    let half_len = len / 2;
    let start_idx = mid_idx.saturating_sub(half_len + (len % 2));
    let end_idx = min_usize(mid_idx + half_len, slice.len());
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
/// use devela::data::slice_msplit_left_mut;
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
#[must_use]
pub fn slice_msplit_left_mut<T>(slice: &mut [T], len: usize) -> &mut [T] {
    let mid_idx = slice.len() / 2;
    let half_len = len / 2;
    let start_idx = mid_idx.saturating_sub(half_len + (len % 2));
    let end_idx = min_usize(mid_idx + half_len, slice.len());
    let (_, right) = slice.split_at_mut(start_idx);
    let (middle, _) = right.split_at_mut(end_idx - start_idx);
    middle
}

/// Replaces the `old` leading byte with a `new` byte.
#[inline]
pub fn slice_replace_leading_bytes(slice: &mut [u8], old: u8, new: u8) {
    let mut start = 0;
    while start < slice.len() && slice[start] == old {
        slice[start] = new;
        start += 1;
    }
}
