// devela::mem::fns
//
//!
//

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
pub fn ptr_in_stack<T>(address: *const T, stack_size: usize) -> bool {
    let stack_var = 0;
    let stack_var_address = &stack_var as *const _ as usize;
    let object_address = address as *const _ as usize;

    let address_difference = if stack_var_address > object_address {
        stack_var_address - object_address
    } else {
        object_address - stack_var_address
    };

    address_difference < stack_size
}
