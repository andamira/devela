// devela_base_core::sys::mem::namespace
//
//! Defines the [`Mem`] namespace.
//

use crate::{Discriminant, is};
use ::core::mem::{
    align_of, align_of_val, discriminant, drop, forget, needs_drop, replace, size_of, size_of_val,
    swap, take,
};
#[allow(unused_imports, reason = "±unsafe")]
use ::core::{
    mem::{transmute_copy, zeroed},
    slice::{from_raw_parts, from_raw_parts_mut},
};

#[doc = crate::_tags!(mem namespace)]
/// Memory-related operations.
#[doc = crate::_doc_location!("sys/mem")]
///
/// See also: [`MemExt`], [`MemAligned`][crate::MemAligned]
/// [`Ptr`][crate::Ptr], [`Slice`][crate::Slice].
#[doc = crate::doclink!(custom devela "[`MemExt`]" "sys/mem/trait.MemExt.html")]
#[derive(Debug)]
pub struct Mem;

/// # Safe methods.
impl Mem {
    /// Aligns `value` downward to the nearest multiple of `align`.
    ///
    /// This is equivalent to `value & !(align - 1)` but uses `wrapping_neg()`
    /// which may generate better code on some architectures.
    ///
    /// # Requirements
    /// - `align` must be a power of two (guaranteed when using [`MemLayout`])
    #[doc = crate::doclink!(custom devela "[`MemLayout`]" "sys/mem/struct.MemLayout.html")]
    ///
    /// # Examples
    /// ```
    /// # use devela_base_core::Mem;
    /// assert_eq!(Mem::align_down(13, 8), 8);
    /// assert_eq!(Mem::align_down(16, 8), 16);
    /// ```
    #[doc = crate::_doc!(vendor: "mini-alloc")]
    #[must_use]
    #[inline(always)]
    pub const fn align_down(value: usize, align: usize) -> usize {
        value & align.wrapping_neg()
    }

    /// Aligns `value` upward to the nearest multiple of `align`.
    #[must_use]
    #[inline(always)]
    pub const fn align_up(value: usize, align: usize) -> usize {
        (value + align - 1) & !(align - 1)
    }

    /// Checks if `value` is aligned to `align`.
    #[must_use]
    #[inline(always)]
    pub const fn is_aligned(value: usize, align: usize) -> bool {
        value & (align - 1) == 0
    }

    /// Runtime version of
    /// [`MemAligned::is_compatible`][super::MemAligned::is_compatible] for raw pointers.
    #[inline(always)]
    pub fn is_aligned_to<T>(ptr: *const T, requirement: usize) -> bool {
        let align = Mem::align_of::<T>();
        requirement >= align && Mem::is_aligned(ptr as usize, align)
    }

    /// Returns the minimum alignment of the type in bytes.
    ///
    /// See `core::mem::`[`align_of`].
    #[must_use]
    pub const fn align_of<T>() -> usize {
        align_of::<T>()
    }

    /// Returns the alignment of the pointed-to value in bytes.
    ///
    /// See `core::mem::`[`align_of_val`].
    #[must_use]
    pub const fn align_of_val<T: ?Sized>(val: &T) -> usize {
        align_of_val(val)
    }

    /// Bitwise-copies a value.
    ///
    /// It is useful when you want to pass a function pointer to a combinator,
    /// rather than defining a new closure.
    ///
    /// # Example
    /// ```
    /// # use devela_base_core::Mem;
    /// let result_from_ffi_fn: Result<(), &i32> = Err(&1);
    /// let result_copied: Result<(), i32> = result_from_ffi_fn.map_err(Mem::copy);
    /// ```
    // WAIT: [core::mem::copy](https://github.com/rust-lang/rust/issues/98262)
    #[must_use]
    pub const fn copy<T: Copy>(x: &T) -> T {
        *x
    }

    /// Returns a value uniquely identifying the enum variant in v.
    #[must_use]
    pub const fn discriminant<T>(v: &T) -> Discriminant<T> {
        discriminant(v)
    }

    /// Disposes of a value.
    ///
    /// See `core::mem::`[`drop`].
    pub fn drop<T>(_x: T) {
        drop(_x);
    }

    /// Takes ownership and “forgets” about `t` *without running its destructor*.
    ///
    /// See `core::mem::`[`forget`].
    pub fn forget<T>(t: T) {
        forget(t);
    }

    /// Returns true if dropping values of type T matters.
    ///
    /// See `core::mem::`[`needs_drop`].
    #[must_use]
    pub const fn needs_drop<T: ?Sized>() -> bool {
        needs_drop::<T>()
    }

    /// Moves `src` into `dest`, returning the previous `dest` value.
    ///
    /// See `core::mem::`[`replace`].
    #[must_use]
    pub const fn replace<T>(dest: &mut T, src: T) -> T {
        replace::<T>(dest, src)
    }

    /// Returns the size of a type in bytes.
    ///
    /// See `core::mem::`[`size_of`].
    #[must_use]
    pub const fn size_of<T>() -> usize {
        size_of::<T>()
    }

    /// Returns the size of the pointed-to value in bytes.
    /// See `core::mem::`[`size_of_val`].
    #[must_use]
    pub const fn size_of_val<T: ?Sized>(val: &T) -> usize {
        size_of_val(val)
    }

    /// Swaps the values at two locations, without deinitializing either one.
    ///
    /// See `core::mem::`[`swap`].
    pub const fn swap<T>(x: &mut T, y: &mut T) {
        swap::<T>(x, y);
    }

    /// Replaces `dest` with `T::default()`, returning the previous `dest` value.
    ///
    /// See `core::mem::`[`take`].
    #[must_use]
    pub fn take<T: Default>(dest: &mut T) -> T {
        take::<T>(dest)
    }
}

/// # Extra methods
#[rustfmt::skip]
impl Mem {
    /// Returns the rounded-up byte count for a bit size.
    ///
    /// Fast path suitable for typical inputs.
    /// For extremely large values (`> usize::MAX - 7`)
    /// the internal wraparound can produce an incorrect result.
    ///
    /// Use [`Mem::bytes_from_bits_saturating`] when correctness
    /// must be guaranteed even for impractical edge cases.
    pub const fn bytes_from_bits(bit_size: usize) -> usize { (bit_size + 7) >> 3 }

    /// Returns the rounded-up byte count for a bit size, saturating on overflow.
    ///
    /// If the addition overflows, this returns the maximum representable
    /// rounded-up value: `usize::MAX >> 3`.
    ///
    /// Always produces the mathematically correct upper bound, even for inputs
    /// near `usize::MAX`, at the cost of a small performance overhead compared
    /// to [`Mem::bytes_from_bits`].
    #[must_use]
    pub const fn bytes_from_bits_saturating(bit_size: usize) -> usize {
        #[cold] const fn bytes_from_bits_cold() -> usize { usize::MAX >> 3 }
        is![let Some(t) = bit_size.checked_add(7); t >> 3; bytes_from_bits_cold()]
    }
}

/// # Unsafe methods
///
/// ## Features
/// They depend on enabling any `unsafe*` feature, and not enabling `safe_mem`.
#[cfg_attr(nightly_doc, doc(cfg(unsafe··)))]
#[cfg(all(not(base_safe_mem), unsafe··))]
impl Mem {
    // NOTE: can't compile, errors with: error[E0512]:
    // cannot transmute between types of different sizes, or dependently-sized types
    //
    // /// Reinterprets the bits of a value of one type as another type.
    // ///
    // /// See `core::mem::`[`transmute`].
    // pub const unsafe fn transmute<Src: Sized, Dst: Sized>(_src: Src) -> Dst {
    //     unsafe { transmute::<Src, Dst>(_src) }
    // }

    /// Reads `src` as having type `&Dst` without moving the contained value.
    ///
    /// # Safety
    /// See `core::mem::`[`transmute_copy`].
    #[must_use]
    pub const unsafe fn transmute_copy<Src, Dst>(src: &Src) -> Dst {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { transmute_copy::<Src, Dst>(src) }
    }

    /// Returns the value of type `T` represented by the all-zero byte-pattern.
    ///
    /// # Safety
    /// See `core::mem::`[`zeroed`].
    #[must_use]
    pub const unsafe fn zeroed<T>() -> T {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { zeroed::<T>() }
    }
}

/// # Unsafe methods gated by `unsafe_slice`
///
/// ## Features
/// They depend on enabling `unsafe_slice` feature, and not enabling `safe_mem`.
#[cfg(all(not(base_safe_mem), feature = "unsafe_slice"))]
#[cfg_attr(nightly_doc, doc(cfg(feature = "unsafe_slice")))]
impl Mem {
    /// View any `T: Sync + Unpin + ?Sized` as `&[u8]`.
    ///
    /// This is a safer interface to `core::slice::`[`from_raw_parts`].
    /// # Example
    /// ```
    /// # use devela_base_core::Mem;
    /// #[repr(C)]
    /// struct Data(u32);
    ///
    /// let data = Data(1234);
    /// let bytes = Mem::as_bytes(&data);
    ///
    /// if cfg!(target_endian = "little") {
    ///     assert!(bytes == &[210, 4, 0, 0]);
    /// } else {
    ///     assert!(bytes == &[0, 0, 4, 210]);
    /// }
    /// ```
    #[doc = crate::_doc!(vendor: "rawbytes")]
    #[must_use]
    pub fn as_bytes<'t, T: Sync + Unpin + ?Sized + 't>(v: &T) -> &'t [u8] {
        // SAFETY: `v` is valid; u8 has alignment 1, size_of_val(v) gives the exact byte length.
        unsafe { from_raw_parts(v as *const _ as *const u8, size_of_val(v)) }
    }

    /// View any `T: Sync + Unpin + ?Sized` as `&mut [u8]`.
    ///
    /// This is a safer interface to `core::slice::`[`from_raw_parts_mut`].
    /// # Examples
    /// ```
    /// # use devela_base_core::Mem;
    /// #[repr(C)]
    /// struct Data(u32);
    ///
    /// let mut data = Data(1234);
    /// let bytes = Mem::as_bytes_mut(&mut data);
    ///
    /// if cfg!(target_endian = "little") {
    ///     bytes[1] = 0;
    ///     assert!(bytes == &[210, 0, 0, 0] && data.0 == 210);
    /// } else {
    ///     bytes[1] = 0;
    ///     assert!(bytes == &[0, 0, 0, 210] && data.0 == 210);
    /// }
    /// ```
    #[doc = crate::_doc!(vendor: "rawbytes")]
    #[must_use]
    pub fn as_bytes_mut<'t, T: Sync + Unpin + ?Sized + 't>(v: &mut T) -> &'t mut [u8] {
        // SAFETY: `v` is a valid, exclusive reference;
        // u8’s alignment is 1, and size_of_val(v) bounds the mutable slice.
        unsafe { from_raw_parts_mut(v as *mut _ as *mut u8, size_of_val(v)) }
    }

    /// View any `T: Sync + Unpin + Sized` as `&[u8]` *compile-time* friendly.
    ///
    /// This is a safer interface to `core::slice::`[`from_raw_parts`], for `Sized` types.
    /// # Examples
    /// ```
    /// # use devela_base_core::Mem;
    /// const DATA: u32 = 1234;
    /// const BYTES: &[u8] = Mem::as_bytes_sized(&DATA);
    ///
    /// if cfg!(target_endian = "little") {
    ///     assert_eq!(BYTES, &[210, 4, 0, 0]);
    /// } else {
    ///     assert_eq!(BYTES, &[0, 0, 4, 210]);
    /// }
    /// ```
    #[must_use]
    pub const fn as_bytes_sized<T: Sync + Unpin>(v: &T) -> &[u8] {
        // SAFETY: `v` is valid; casting to *const u8 is safe (u8 has alignment 1)
        // and size_of::<T>() exactly covers the object.
        unsafe { from_raw_parts(v as *const T as *const u8, size_of::<T>()) }
    }
}
