// devela::mem::size
//
//! Traits related to memory size.
//

use super::size_of;
use crate::codegen::iif;

mod bit;
mod size;
pub use {bit::*, size::*};

/// Returns the size ratio between the byte length of `usize` and `byte_size`.
///
/// For example: the ratio will be (1, 1) if both sizes are the same,
/// (2, 1) if a pointer is double the byte size, and (1, 2) if a pointer is
/// half the byte size.
///
/// # Examples
/// ```
/// use devela::mem::{ptr_ratio, size_of};
///
/// assert_eq![ptr_ratio(0), (1, 0)];
/// assert_eq![ptr_ratio(size_of::<usize>()), (1, 1)];
/// assert_eq![ptr_ratio(size_of::<&str>()), (1, 2)];
/// assert_eq![ptr_ratio(size_of::<String>()), (1, 3)];
///
/// #[cfg(target_pointer_width = "64")]
/// assert_eq![ptr_ratio(size_of::<char>()), (2,1)];
/// ```
///
/// Note that when `byte_size == 0` it returns `(1, 0)` which is an invalid ratio.
///
#[inline]
pub const fn ptr_ratio(byte_size: usize) -> (usize, usize) {
    #[inline]
    const fn gcd(m: usize, n: usize) -> usize {
        iif![n == 0; m; gcd(n, m % n)]
    }
    let g = gcd(size_of::<usize>(), byte_size);
    (size_of::<usize>() / g, byte_size / g)
}
