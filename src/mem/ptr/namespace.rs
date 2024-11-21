// devela::mem::ptr::namespace
//
//! `Ptr` namespace.
//

#[allow(unused_imports, reason = "±unsafe")]
use crate::{
    iif, Hasher,
    _core::ptr::{
        addr_eq, copy, copy_nonoverlapping, drop_in_place, eq, from_mut, from_ref, hash, null,
        null_mut, read, read_unaligned, read_volatile, replace, slice_from_raw_parts,
        slice_from_raw_parts_mut, swap, swap_nonoverlapping, write, write_bytes, write_unaligned,
        write_volatile,
    },
};

/// A pointer-related functionality namespace.
///
/// See also: <https://doc.rust-lang.org/std/ptr/#functions>
pub struct Ptr;

impl Ptr {
    /// Compares raw pointer addresses for equality, ignoring any metadata in fat pointers.
    ///
    /// See std's [`addr_eq`].
    pub fn addr_eq<T: ?Sized, U: ?Sized>(p: *const T, q: *const U) -> bool {
        addr_eq(p, q)
    }

    /// Copies `count * size_of::<T>()` bytes from `src` to `dst`. Can overlap.
    ///
    /// # Safety
    /// See std's [`copy`].
    pub unsafe fn copy<T>(src: *const T, dst: *mut T, count: usize) {
        // SAFETY: Caller must uphold the safety contract.
        unsafe {
            copy(src, dst, count);
        }
    }

    /// Copies `count * size_of::<T>()` bytes from `src` to `dst`. Must *not* overlap.
    ///
    /// # Safety
    /// See std's [`copy_nonoverlapping`].
    pub unsafe fn copy_nonoverlapping<T>(src: *const T, dst: *mut T, count: usize) {
        // SAFETY: Caller must uphold the safety contract.
        unsafe {
            copy_nonoverlapping(src, dst, count);
        }
    }

    /// Executes the destructor (if any) of the pointed-to value.
    ///
    /// # Safety
    /// See std's [`drop_in_place`].
    pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) {
        // SAFETY: Caller must uphold the safety contract.
        unsafe {
            drop_in_place(to_drop);
        }
    }

    /// Compares raw pointers for equality.
    ///
    /// See std's [`eq`].
    pub fn eq<T: ?Sized>(a: *const T, b: *const T) -> bool {
        eq(a, b)
    }

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
    /// # Example
    /// ```
    /// # use devela::Ptr;
    /// const STACK_SIZE: usize = 2 << 20; // assume a 2 MB stack size
    ///
    /// let in_stack: [i32; 10] = [0; 10];
    /// let in_heap = vec![0; 10];
    ///
    /// assert_eq!(true, Ptr::in_stack(in_stack.as_ptr(), STACK_SIZE));
    /// assert_eq!(false, Ptr::in_stack(in_heap.as_ptr(), STACK_SIZE));
    /// ```
    #[cfg(not(miri))] // The addresses in Miri are not real addresses
    #[must_use]
    pub fn in_stack<T>(address: *const T, stack_size: usize) -> bool {
        let local_var = 0;
        let local_addr = &local_var as *const _ as usize;
        let obj_addr = address as *const _ as usize;
        let addr_diff = iif![local_addr > obj_addr; local_addr - obj_addr; obj_addr - local_addr];
        addr_diff < stack_size
    }

    /// Convert an exclusive reference to a raw pointer.
    ///
    /// See std's [`from_mut`].
    // WAIT:1.83: [const_mut_refs|const_refs_to_cell](https://github.com/rust-lang/rust/pull/129195)
    pub fn from_mut<T: ?Sized>(r: &mut T) -> *mut T {
        from_mut(r)
    }

    /// Convert a shared reference to a raw pointer.
    ///
    /// See std's [`from_ref`].
    pub const fn from_ref<T: ?Sized>(r: &T) -> *const T {
        from_ref(r)
    }

    /// Hash a raw pointer.
    ///
    /// See std's [`hash`].
    pub fn hash<T: ?Sized, S: Hasher>(hashee: *const T, into: &mut S) {
        hash(hashee, into);
    }

    /// Creates a null raw pointer.
    ///
    /// See std's [`null`].
    // WAIT: [ptr_metadata](https://github.com/rust-lang/rust/issues/81513)
    // T: Thin + ?Sized https://doc.rust-lang.org/core/ptr/traitalias.Thin.html
    pub const fn null<T>() -> *const T {
        null()
    }

    /// Creates a null mutable raw pointer.
    ///
    /// See std's [`null_mut`].
    // WAIT: [ptr_metadata](https://github.com/rust-lang/rust/issues/81513)
    // T: Thin + ?Sized https://doc.rust-lang.org/core/ptr/traitalias.Thin.html
    pub const fn null_mut<T>() -> *mut T {
        null_mut()
    }

    /// Reads the value from src without moving it.
    ///
    /// # Safety
    /// See std's [`read`].
    pub const unsafe fn read<T>(src: *const T) -> T {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { read(src) }
    }

    /// Reads the value from src without moving it.
    ///
    /// # Safety
    /// See std's [`read_unaligned`].
    pub const unsafe fn read_unaligned<T>(src: *const T) -> T {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { read_unaligned(src) }
    }

    /// Performs a volatile read of the value from src without moving it.
    ///
    /// # Safety
    /// See std's [`read_volatile`].
    pub unsafe fn read_volatile<T>(src: *const T) -> T {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { read_volatile(src) }
    }

    /// Moves src into the pointed dst, returning the previous dst value.
    ///
    /// # Safety
    /// See std's [`replace`].
    pub unsafe fn replace<T>(dst: *mut T, src: T) -> T {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { replace(dst, src) }
    }

    /// Forms a raw slice from a pointer and a length.
    ///
    /// See std's [`slice_from_raw_parts`].
    pub const fn slice_from_raw_parts<T>(data: *const T, len: usize) -> *const [T] {
        slice_from_raw_parts(data, len)
    }

    /// Forms a mutable raw slice from a mutable pointer and a length.
    ///
    /// See std's [`slice_from_raw_parts_mut`].
    // WAIT:1.83: [const_slice_from_raw_parts_mut](https://github.com/rust-lang/rust/pull/130403)
    pub fn slice_from_raw_parts_mut<T>(data: *mut T, len: usize) -> *mut [T] {
        slice_from_raw_parts_mut(data, len)
    }

    /// Swaps the values at two mutable locations of the same type, without deinitializing.
    ///
    /// # Safety
    /// See std's [`swap`].
    pub unsafe fn swap<T>(x: *mut T, y: *mut T) {
        // SAFETY: Caller must uphold the safety contract.
        unsafe {
            swap(x, y);
        }
    }

    /// Swaps the two regions of memory beginning at `x` and `y`. Must *not* overlap.
    ///
    /// # Safety
    /// See std's [`swap_nonoverlapping`].
    pub unsafe fn swap_nonoverlapping<T>(x: *mut T, y: *mut T, count: usize) {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { swap_nonoverlapping(x, y, count) };
    }

    /// Overwrites a memory location with `src` without reading or dropping.
    ///
    /// # Safety
    /// See std's [`write`].
    pub unsafe fn write<T>(dst: *mut T, src: T) {
        // SAFETY: Caller must uphold the safety contract.
        unsafe {
            write(dst, src);
        };
    }

    /// Sets `count * size_of::<T>()` bytes of memory starting at `dst` to `val`.
    ///
    /// # Safety
    /// See std's [`write_bytes`].
    pub unsafe fn write_bytes<T>(dst: *mut T, val: u8, count: usize) {
        // SAFETY: Caller must uphold the safety contract.
        unsafe {
            write_bytes(dst, val, count);
        };
    }

    /// Overwrites a memory location with `src` without reading or dropping.
    ///
    /// # Safety
    /// See std's [`write_unaligned`].
    pub unsafe fn write_unaligned<T>(dst: *mut T, src: T) {
        // SAFETY: Caller must uphold the safety contract.
        unsafe {
            write_unaligned(dst, src);
        };
    }

    /// Performs a volatile write of a memory location with `src` without reading or dropping.
    ///
    /// # Safety
    /// See std's [`write_volatile`].
    pub unsafe fn write_volatile<T>(dst: *mut T, src: T) {
        // SAFETY: Caller must uphold the safety contract.
        unsafe {
            write_volatile(dst, src);
        };
    }
}
