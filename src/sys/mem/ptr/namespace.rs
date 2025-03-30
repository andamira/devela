// devela::sys::mem::ptr::namespace
//
//! Defines the [`Ptr`] namespace.
//
// WAIT: [intra-doc links to pointers](https://github.com/rust-lang/rust/issues/80896)

#[allow(unused_imports, reason = "unsafe feature-gated")]
use crate::_core::ptr::{
    copy, copy_nonoverlapping, drop_in_place, read, read_unaligned, read_volatile, replace, swap,
    swap_nonoverlapping, write, write_bytes, write_unaligned, write_volatile,
};
use crate::{
    _core::ptr::{
        addr_eq, dangling, dangling_mut, eq, from_mut, from_ref, hash, null, null_mut,
        slice_from_raw_parts, slice_from_raw_parts_mut, with_exposed_provenance,
        with_exposed_provenance_mut, without_provenance, without_provenance_mut,
    },
    Hasher, iif,
};

#[doc = crate::TAG_NAMESPACE!()]
/// Pointer-related operations.
///
/// See also [`Mem`][crate::Mem], [`Slice`][crate::Slice].
pub struct Ptr;

/// # Safe methods
impl Ptr {
    /// The size of a pointer in bits, for the current platform.
    pub const BITS: usize = usize::BITS as usize;
    /// The size of a pointer in bytes, for the current platform.
    pub const BYTES: usize = size_of::<usize>();

    /// True if the system's architecture is little-endian.
    pub const LITTLE_ENDIAN: bool = cfg!(target_endian = "little");
    /// True if the system's architecture is big-endian.
    pub const BIG_ENDIAN: bool = cfg!(target_endian = "big");

    /// Compares raw pointer addresses for equality, ignoring any metadata in fat pointers.
    ///
    /// See `core::ptr::`[`addr_eq`].
    ///
    /// For functions you can use `core::ptr::`[`fn_addr_eq`][core::ptr::fn_addr_eq] directly.
    #[must_use]
    pub fn addr_eq<T: ?Sized, U: ?Sized>(p: *const T, q: *const U) -> bool {
        addr_eq(p, q)
    }

    // WAIT: `fn_ptr_trait`: FnPtr requires nightly. Until then can't name FnPtr.
    // https://github.com/rust-lang/rust/issues/129322#issuecomment-2418860738
    // /// Compares the *addresses* of the two function pointers for equality.
    // ///
    // /// See `core::ptr::`[`fn_addr_eq`].
    // #[must_use]
    // pub fn fn_addr_eq<T: ::core::marker::FnPtr, U: ::core::marker::FnPtr>(f: T, g: U) -> bool {
    //     fn_addr_eq(p, q)
    // }

    /// Compares raw pointers for equality.
    ///
    /// See `core::ptr::`[`eq`].
    #[must_use]
    pub fn eq<T: ?Sized>(a: *const T, b: *const T) -> bool {
        eq(a, b)
    }

    /// Creates a new pointer that is dangling, but non-null and well-aligned.
    ///
    /// See `core::ptr::`[`dangling`].
    #[must_use]
    pub const fn dangling<T>() -> *const T {
        dangling()
    }
    /// Creates a new pointer that is dangling, but non-null and well-aligned.
    ///
    /// See `core::ptr::`[`dangling_mut`].
    #[must_use]
    pub const fn dangling_mut<T>() -> *mut T {
        dangling_mut()
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
    /// See `core::ptr::`[`from_mut`].
    #[must_use]
    pub const fn from_mut<T: ?Sized>(r: &mut T) -> *mut T {
        from_mut(r)
    }

    /// Convert a shared reference to a raw pointer.
    ///
    /// See `core::ptr::`[`from_ref`].
    #[must_use]
    pub const fn from_ref<T: ?Sized>(r: &T) -> *const T {
        from_ref(r)
    }

    /// Hash a raw pointer.
    ///
    /// See `core::ptr::`[`hash`].
    pub fn hash<T: ?Sized, S: Hasher>(hashee: *const T, into: &mut S) {
        hash(hashee, into);
    }

    /// Creates a null raw pointer.
    ///
    /// See `core::ptr::`[`null`]
    /// and `<ptr>::`[`is_null`][pointer#method.is_null].
    // WAIT: [ptr_metadata](https://github.com/rust-lang/rust/issues/81513)
    // T: Thin + ?Sized https://doc.rust-lang.org/core/ptr/traitalias.Thin.html
    #[must_use]
    pub const fn null<T>() -> *const T {
        null()
    }

    /// Creates a null mutable raw pointer.
    ///
    /// See `core::ptr::`[`null_mut`].
    // WAIT: [ptr_metadata](https://github.com/rust-lang/rust/issues/81513)
    // T: Thin + ?Sized https://doc.rust-lang.org/core/ptr/traitalias.Thin.html
    #[must_use]
    pub const fn null_mut<T>() -> *mut T {
        null_mut()
    }

    /// Returns the ratio of a `usize` in respect to `other_size`.
    ///
    /// For example: the ratio will be `(1, 1)` if both sizes are equal, `(2, 1)`
    /// if the pointer size is double the other size, and `(1, 2)` if is is half
    /// the other byte size.
    ///
    /// # Examples
    /// ```
    /// use devela::Ptr;
    ///
    /// assert_eq![Ptr::size_ratio(0), [1, 0]];
    /// assert_eq![Ptr::size_ratio(size_of::<usize>()), [1, 1]];
    /// assert_eq![Ptr::size_ratio(size_of::<&str>()), [1, 2]];
    /// assert_eq![Ptr::size_ratio(size_of::<String>()), [1, 3]];
    ///
    /// #[cfg(target_pointer_width = "64")]
    /// assert_eq![Ptr::size_ratio(size_of::<char>()), [2,1]];
    /// ```
    ///
    /// Note that when `other_size == 0` it returns `(1, 0)` which is an invalid ratio.
    #[must_use]
    pub const fn size_ratio(other_size: usize) -> [usize; 2] {
        const fn gcd(m: usize, n: usize) -> usize {
            iif![n == 0; m; gcd(n, m % n)]
        }
        let g = gcd(size_of::<usize>(), other_size);
        [size_of::<usize>() / g, other_size / g]
    }

    /// Forms a raw slice from a pointer and a length.
    ///
    /// See `core::ptr::`[`slice_from_raw_parts`], and also
    /// `Slice::`[`from_raw_parts`][crate::Slice::from_raw_parts].
    #[must_use]
    pub const fn slice_from_raw_parts<T>(data: *const T, len: usize) -> *const [T] {
        slice_from_raw_parts(data, len)
    }

    /// Forms a mutable raw slice from a mutable pointer and a length.
    ///
    /// See `core::ptr::`[`slice_from_raw_parts_mut`], and also
    /// `Slice::`[`from_raw_parts_mut`][crate::Slice::from_raw_parts_mut].
    #[must_use]
    pub const fn slice_from_raw_parts_mut<T>(data: *mut T, len: usize) -> *mut [T] {
        slice_from_raw_parts_mut(data, len)
    }

    /// Converts an address back to a pointer,
    /// picking up some previously ‘exposed’ *provenance*.
    ///
    /// See `core::ptr::`[`with_exposed_provenance`]
    /// and `<ptr>::`[`expose_provenance`][pointer#method.expose_provenance].
    #[must_use]
    pub fn with_exposed_provenance<T>(addr: usize) -> *const T {
        with_exposed_provenance(addr)
    }
    /// Converts an address back to a mutable pointer,
    /// picking up some previously ‘exposed’ *provenance*.
    ///
    /// See `core::ptr::`[`with_exposed_provenance_mut`]
    /// and `<ptr>::`[`expose_provenance`][pointer#method.expose_provenance].
    #[must_use]
    pub fn with_exposed_provenance_mut<T>(addr: usize) -> *mut T {
        with_exposed_provenance_mut(addr)
    }

    /// Creates a pointer with the given address and no *provenance*.
    ///
    /// See `core::ptr::`[`without_provenance`]
    /// `<ptr>::`[`with_addr`][pointer#method.with_addr],
    /// and `<ptr>::`[`map_addr`][pointer#method.map_addr].
    #[must_use]
    pub fn without_provenance<T>(addr: usize) -> *const T {
        without_provenance(addr)
    }
    /// Creates a pointer with the given address and no *provenance*.
    ///
    /// See `core::ptr::`[`without_provenance_mut`]
    /// `<ptr>::`[`with_addr`][pointer#method.with_addr],
    /// and `<ptr>::`[`map_addr`][pointer#method.map_addr].
    #[must_use]
    pub fn without_provenance_mut<T>(addr: usize) -> *mut T {
        without_provenance_mut(addr)
    }
}

/// # Unsafe methods
///
/// ## Features
/// They depend on enabling any `unsafe*` feature, and not enabling `safe_mem`.
#[rustfmt::skip]
#[cfg_attr(nightly_doc, doc(cfg(unsafe··)))]
#[cfg(all(not(feature = "safe_mem"), unsafe··))]
impl Ptr {
    /// Copies `count * size_of::<T>()` bytes from `src` to `dst`. Can overlap.
    ///
    /// # Safety
    /// See `core::ptr::`[`copy`].
    pub const unsafe fn copy<T>(src: *const T, dst: *mut T, count: usize) {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { copy(src, dst, count); }
    }

    /// Copies `count * size_of::<T>()` bytes from `src` to `dst`. Must *not* overlap.
    ///
    /// # Safety
    /// See `core::ptr::`[`copy_nonoverlapping`].
    pub const unsafe fn copy_nonoverlapping<T>(src: *const T, dst: *mut T, count: usize) {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { copy_nonoverlapping(src, dst, count); }
    }

    /// Executes the destructor (if any) of the pointed-to value.
    ///
    /// # Safety
    /// See `core::ptr::`[`drop_in_place`].
    pub unsafe fn drop_in_place<T: ?Sized>(to_drop: *mut T) {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { drop_in_place(to_drop); }
    }

    /// Reads the value from src without moving it.
    ///
    /// # Safety
    /// See `core::ptr::`[`read`].
    #[must_use]
    pub const unsafe fn read<T>(src: *const T) -> T {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { read(src) }
    }

    /// Reads the value from src without moving it.
    ///
    /// # Safety
    /// See `core::ptr::`[`read_unaligned`].
    #[must_use]
    pub const unsafe fn read_unaligned<T>(src: *const T) -> T {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { read_unaligned(src) }
    }

    /// Performs a volatile read of the value from src without moving it.
    ///
    /// # Safety
    /// See `core::ptr::`[`read_volatile`].
    #[must_use]
    pub unsafe fn read_volatile<T>(src: *const T) -> T {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { read_volatile(src) }
    }

    /// Moves src into the pointed dst, returning the previous dst value.
    ///
    /// # Safety
    /// See `core::ptr::`[`replace`].
    #[must_use]
    pub const unsafe fn replace<T>(dst: *mut T, src: T) -> T {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { replace(dst, src) }
    }

    /// Swaps the values at two mutable locations of the same type, without deinitializing.
    ///
    /// # Safety
    /// See `core::ptr::`[`swap`].
    pub const unsafe fn swap<T>(x: *mut T, y: *mut T) {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { swap(x, y); }
    }

    /// Swaps the two regions of memory beginning at `x` and `y`. Must *not* overlap.
    ///
    /// # Safety
    /// See `core::ptr::`[`swap_nonoverlapping`].
    pub unsafe fn swap_nonoverlapping<T>(x: *mut T, y: *mut T, count: usize) {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { swap_nonoverlapping(x, y, count) };
    }

    /// Overwrites a memory location with `src` without reading or dropping.
    ///
    /// # Safety
    /// See `core::ptr::`[`write`][fn@write].
    pub const unsafe fn write<T>(dst: *mut T, src: T) {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { write(dst, src); };
    }

    /// Sets `count * size_of::<T>()` bytes of memory starting at `dst` to `val`.
    ///
    /// # Safety
    /// See `core::ptr::`[`write_bytes`].
    pub const unsafe fn write_bytes<T>(dst: *mut T, val: u8, count: usize) {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { write_bytes(dst, val, count); };
    }

    /// Overwrites a memory location with `src` without reading or dropping.
    ///
    /// # Safety
    /// See `core::ptr::`[`write_unaligned`].
    pub const unsafe fn write_unaligned<T>(dst: *mut T, src: T) {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { write_unaligned(dst, src); };
    }

    /// Performs a volatile write of a memory location with `src` without reading or dropping.
    ///
    /// # Safety
    /// See `core::ptr::`[`write_volatile`].
    pub unsafe fn write_volatile<T>(dst: *mut T, src: T) {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { write_volatile(dst, src); };
    }
}
