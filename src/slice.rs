// revela::slice
//
//! Slice-related abstractions.
//

/// Returns a left subslice of `slice`, with the given maximum `len`.
///
/// If `left_len > slice.len()` it returns the full slice.
#[inline]
pub fn subslice_left<T>(slice: &[T], len: usize) -> &[T] {
    let start_idx = 0;
    let end_idx = len.clamp(start_idx, slice.len());
    &slice[start_idx..end_idx]
}

/// Returns a right subslice of `slice`, with the given maximum `len`.
///
/// If `left_len > slice.len()` it returns the full slice.
#[inline]
pub fn subslice_right<T>(slice: &[T], len: usize) -> &[T] {
    let start_idx = slice.len().saturating_sub(len);
    let end_idx = slice.len();
    &slice[start_idx..end_idx]
}

/// Returns a middle subslice of `slice`, with the given maximum `len`.
///
/// If `len > slice.len()` returns the full `slice`.
///
/// In case of a non-perfect middle split, it leaves an extra left character.
#[inline]
pub fn subslice_middle<T>(slice: &[T], len: usize) -> &[T] {
    let mid_idx = slice.len() / 2;
    let half_len = len / 2;
    let start_idx = mid_idx.saturating_sub(half_len);
    let end_idx = (mid_idx + half_len + (len % 2)).min(slice.len());
    &slice[start_idx..end_idx]
}
