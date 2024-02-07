// devela::num::ops::always_fns
//
//! `num` standalone functions.
//!
//! Always available for internal use.
//

use crate::code::iif;

/// Counts the number of decimal digits in `n`.
/// # Examples
/// ```
/// # use devela::num::count_digits;
/// assert_eq![1, count_digits(0)];
/// assert_eq![4, count_digits(9876)];
/// assert_eq![10, count_digits(u32::MAX as usize)];
/// ```
#[inline]
#[must_use]
pub const fn count_digits(n: usize) -> usize {
    iif![n == 0; 1; n.ilog10() as usize + 1]
}

/// Counts the number of decimal digits in `n`.
/// # Panics
/// Panics if `n == 0`
/// # Examples
/// ```
/// # use devela::num::count_digits_unchecked;
/// assert_eq![4, count_digits_unchecked(9876)];
/// assert_eq![10, count_digits_unchecked(u32::MAX as usize)];
/// ```
#[inline]
#[must_use]
pub const fn count_digits_unchecked(n: usize) -> usize {
    n.ilog10() as usize + 1
}
