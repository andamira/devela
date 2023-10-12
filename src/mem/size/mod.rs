// devela::mem::size
//
//! Traits related to memory size.
//

use super::mem_size_of;
use crate::codegen::iif;

mod bit;
mod heap;
mod size;
pub use {bit::*, heap::*, size::*};

/// Returns the ratio of a `usize` in respect to `other_size`.
///
/// For example: the ratio will be `(1, 1)` if both sizes are equal, `(2, 1)`
/// if the pointer size is double the other size, and `(1, 2)` if is is half
/// the other byte size.
///
/// # Examples
/// ```
/// use devela::mem::{mem_ptr_ratio, mem_size_of};
///
/// assert_eq![mem_ptr_ratio(0), (1, 0)];
/// assert_eq![mem_ptr_ratio(mem_size_of::<usize>()), (1, 1)];
/// assert_eq![mem_ptr_ratio(mem_size_of::<&str>()), (1, 2)];
/// assert_eq![mem_ptr_ratio(mem_size_of::<String>()), (1, 3)];
///
/// #[cfg(target_pointer_width = "64")]
/// assert_eq![mem_ptr_ratio(mem_size_of::<char>()), (2,1)];
/// ```
///
/// Note that when `other_size == 0` it returns `(1, 0)` which is an invalid ratio.
///
#[inline]
pub const fn mem_ptr_ratio(other_size: usize) -> (usize, usize) {
    #[inline]
    const fn gcd(m: usize, n: usize) -> usize {
        iif![n == 0; m; gcd(n, m % n)]
    }
    let g = gcd(mem_size_of::<usize>(), other_size);
    (mem_size_of::<usize>() / g, other_size / g)
}
