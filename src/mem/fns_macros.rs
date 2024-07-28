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

/// Bitwise-copies a value.
///
/// It is useful when you want to pass a function pointer to a combinator,
/// rather than defining a new closure.
///
/// # Examples
/// ```
/// # use devela::mem_copy;
/// let result_from_ffi_function: Result<(), &i32> = Err(&1);
/// let result_copied: Result<(), i32> = result_from_ffi_function.map_err(mem_copy);
/// ```
// WAIT: [core::mem::copy](https://github.com/rust-lang/rust/issues/98262)
#[inline] #[must_use] #[rustfmt::skip]
pub const fn mem_copy<T: Copy>(x: &T) -> T { *x }

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

#[cfg(all(not(feature = "safe_data"), feature = "unsafe_slice"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_slice")))]
mod unsafe_fns {
    use core::slice;

    /// View any `T: Sync + Unpin + ?Sized` as `&[u8]`.
    ///
    /// This is a safer interface to [`slice::from_raw_parts`].
    /// # Examples
    /// ```
    /// # use devela::mem_as_bytes;
    /// #[repr(C)]
    /// struct Data(u32);
    ///
    /// let data = Data(1234);
    /// let bytes = mem_as_bytes(&data);
    ///
    /// if cfg!(target_endian = "little") {
    ///     assert!(bytes == &[210, 4, 0, 0]);
    /// } else {
    ///     assert!(bytes == &[0, 0, 4, 210]);
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn mem_as_bytes<'t, T: Sync + Unpin + ?Sized + 't>(v: &T) -> &'t [u8] {
        unsafe { slice::from_raw_parts(v as *const _ as *const u8, size_of_val(v)) }
    }

    /// View any `T: Sync + Unpin + ?Sized` as `&mut [u8]`.
    ///
    /// This is a safer interface to [`slice::from_raw_parts_mut`].
    /// # Examples
    /// ```
    /// # use devela::mem_as_bytes_mut;
    /// #[repr(C)]
    /// struct Data(u32);
    ///
    /// let mut data = Data(1234);
    /// let bytes = mem_as_bytes_mut(&mut data);
    ///
    /// if cfg!(target_endian = "little") {
    ///     bytes[1] = 0;
    ///     assert!(bytes == &[210, 0, 0, 0] && data.0 == 210);
    /// } else {
    ///     bytes[1] = 0;
    ///     assert!(bytes == &[0, 0, 0, 210] && data.0 == 210);
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub fn mem_as_bytes_mut<'t, T: Sync + Unpin + ?Sized + 't>(v: &mut T) -> &'t mut [u8] {
        unsafe { slice::from_raw_parts_mut(v as *mut _ as *mut u8, size_of_val(v)) }
    }

    /// View any `T: Sync + Unpin + Sized` as `&[u8]` *compile-time* friendly.
    ///
    /// This is a safer interface to [`slice::from_raw_parts`], for `Sized` types.
    /// # Examples
    /// ```
    /// # use devela::mem_as_bytes_sized;
    /// const DATA: u32 = 1234;
    /// const BYTES: &[u8] = mem_as_bytes_sized(&DATA);
    ///
    /// if cfg!(target_endian = "little") {
    ///     assert_eq!(BYTES, &[210, 4, 0, 0]);
    /// } else {
    ///     assert_eq!(BYTES, &[0, 0, 4, 210]);
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub const fn mem_as_bytes_sized<T: Sync + Unpin>(v: &T) -> &[u8] {
        unsafe { slice::from_raw_parts(v as *const T as *const u8, size_of::<T>()) }
    }
}
#[cfg(all(not(feature = "safe_data"), feature = "unsafe_slice"))]
pub use unsafe_fns::*;

/* macros */

/// Swaps two mutable variables in a *compile-time* friendly manner.
///
/// For that it uses either a temporary variable or the [xor swap method].
///
/// [xor swap method]: https://en.wikipedia.org/wiki/XOR_swap_algorithm
//
// WAIT: [const_swap](https://github.com/rust-lang/rust/issues/83163)
#[macro_export]
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
pub use cswap;
