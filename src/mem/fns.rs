// devela::mem::fns
//
//!
//

use crate::code::iif;

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
/// use devela::mem::ptr_in_stack;
///
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
