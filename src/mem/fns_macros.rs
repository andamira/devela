// devela::mem::fns_macros
//
//!
//
// TOC
// - fns
//   - mem_copy
//   - mem_as_bytes
//   - mem_as_bytes_mut
//   - mem_as_bytes_sized
// - macros
//   - cswap

#[cfg(not(miri))]
use crate::code::iif;

/* fns */

/// Returns `true` if it's probable the given `address` is in the stack, for a
/// given `stack_size`.
///
/// # Stack size
/// - <https://doc.rust-lang.org/std/thread/#stack-size>.
///
/// The default stack size is platform-dependent and subject to change.
/// Currently, it is 2 MiB on all Tier-1 platforms.
/// Note that the stack size of the main thread is *not* determined by Rust.
///
/// If the address is close to a stack variable address it might be stack allocated.
///
/// # Examples
/// ```
/// # use devela::mem::ptr_in_stack;
/// const STACK_SIZE: usize = 2 << 20; // assume a 2 MB stack size
///
/// let in_stack: [i32; 10] = [0; 10];
/// let in_heap = vec![0; 10];
///
/// assert_eq!(true, ptr_in_stack(in_stack.as_ptr(), STACK_SIZE));
/// assert_eq!(false, ptr_in_stack(in_heap.as_ptr(), STACK_SIZE));
/// ```
#[cfg(not(miri))] // The addresses in Miri are not real addresses
#[must_use]
#[inline]
pub fn ptr_in_stack<T>(address: *const T, stack_size: usize) -> bool {
    let local_var = 0;
    let local_addr = &local_var as *const _ as usize;
    let obj_addr = address as *const _ as usize;
    let addr_diff = iif![local_addr > obj_addr; local_addr - obj_addr; obj_addr - local_addr];
    addr_diff < stack_size
}

/* macros */

/// Swaps two mutable variables in a *compile-time* friendly manner.
///
/// For that it uses either a temporary variable or the [xor swap method].
///
/// [xor swap method]: https://en.wikipedia.org/wiki/XOR_swap_algorithm
//
// WAIT: [const_swap](https://github.com/rust-lang/rust/issues/83163)
#[macro_export]
#[cfg_attr(cargo_primary_package, doc(hidden))]
macro_rules! cswap {
    (
        // swaps two values using a temporary variable
        $a:expr, $b:expr) => {{
        let tmp = $a;
        $a = $b;
        $b = tmp;
    }};
    (
        // swaps two `T: PartialEq + BitXorAssign` values without a temporary variable
        xor $a:expr, $b:expr) => {{
        if $a != $b {
            $a ^= $b;
            $b ^= $a;
            $a ^= $b;
        }
    }};
}
#[doc(inline)]
pub use cswap;
