// devela::mem::namespace
//
//! `Str` namespace.
//

use crate::_core::mem::{
    align_of, align_of_val, drop, forget, needs_drop, replace, size_of, size_of_val, swap, take,
    transmute_copy, zeroed,
};

/// A memory functionality namespace.
///
/// See also the [`std::mem`] module, and the [`ExtMem`][crate::ExtMem] trait.
pub struct Mem;

impl Mem {
    /// Returns the minimum alignment of the type in bytes.
    ///
    /// See `std::`[`align_of`].
    #[must_use]
    pub const fn align_of<T>() -> usize {
        align_of::<T>()
    }

    /// Returns the alignment of the pointed-to value in bytes.
    ///
    /// See `std::`[`align_of_val`].
    #[must_use]
    // WAIT: [const_align_of_val](https://github.com/rust-lang/rust/issues/46571)
    pub fn align_of_val<T: ?Sized>(val: &T) -> usize {
        align_of_val(val)
    }

    /// Bitwise-copies a value.
    ///
    /// It is useful when you want to pass a function pointer to a combinator,
    /// rather than defining a new closure.
    ///
    /// # Example
    /// ```
    /// # use devela::Mem;
    /// let result_from_ffi_fn: Result<(), &i32> = Err(&1);
    /// let result_copied: Result<(), i32> = result_from_ffi_fn.map_err(Mem::copy);
    /// ```
    // WAIT: [core::mem::copy](https://github.com/rust-lang/rust/issues/98262)
    #[must_use]
    pub const fn copy<T: Copy>(x: &T) -> T {
        *x
    }

    /// Disposes of a value.
    ///
    /// See `std::`[`drop`].
    pub fn drop<T>(_x: T) {
        drop(_x);
    }

    /// Takes ownership and “forgets” about `t` *without running its destructor*.
    ///
    /// See `std::`[`forget`].
    pub fn forget<T>(t: T) {
        forget(t);
    }

    /// Returns true if dropping values of type T matters.
    ///
    /// See `std::`[`needs_drop`].
    #[must_use]
    pub const fn needs_drop<T: ?Sized>() -> bool {
        needs_drop::<T>()
    }

    /// Moves `src` into `dest`, returning the previous `dest` value.
    ///
    /// See `std::`[`replace`].
    // WAIT:1.83 [const_mut_refs](https://github.com/rust-lang/rust/issues/129195)
    #[must_use]
    pub fn replace<T>(dest: &mut T, src: T) -> T {
        replace::<T>(dest, src)
    }

    /// Returns the size of a type in bytes.
    ///
    /// See `std::`[`size_of`].
    #[must_use]
    pub const fn size_of<T>() -> usize {
        size_of::<T>()
    }

    /// Returns the size of the pointed-to value in bytes.
    /// See `std::`[`size_of_val`].
    #[must_use]
    // WAIT: [const_size_of_val](https://github.com/rust-lang/rust/issues/46571)
    pub fn size_of_val<T: ?Sized>(val: &T) -> usize {
        size_of_val(val)
    }

    /// Swaps the values at two locations, without deinitializing either one.
    ///
    /// See `std::`[`swap`].
    pub fn swap<T>(x: &mut T, y: &mut T) {
        swap::<T>(x, y);
    }

    /// Replaces `dest` with `T::default()`, returning the previous `dest` value.
    ///
    /// See `std::`[`take`].
    #[must_use]
    pub fn take<T: Default>(dest: &mut T) -> T {
        take::<T>(dest)
    }

    // NOTE: can't compile, errors with: error[E0512]:
    // cannot transmute between types of different sizes, or dependently-sized types
    //
    // /// Reinterprets the bits of a value of one type as another type.
    // ///
    // /// See `std::`[`transmute`].
    // #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_layout")))]
    // #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
    // pub const unsafe fn transmute<Src: Sized, Dst: Sized>(_src: Src) -> Dst {
    //     unsafe { transmute::<Src, Dst>(_src) }
    // }

    /// Reads `src` as having type `&Dst` without moving the contained value.
    ///
    /// # Safety
    /// See `std::`[`transmute_copy`].
    #[must_use]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_layout")))]
    #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
    pub const unsafe fn transmute_copy<Src, Dst>(src: &Src) -> Dst {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { transmute_copy::<Src, Dst>(src) }
    }

    /// Returns the value of type `T` represented by the all-zero byte-pattern.
    ///
    /// # Safety
    /// See `std::`[`zeroed`].
    #[must_use]
    #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_layout")))]
    #[cfg(all(not(feature = "safe_mem"), feature = "unsafe_layout"))]
    pub const unsafe fn zeroed<T>() -> T {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { zeroed::<T>() }
    }
}

#[cfg(all(not(feature = "safe_data"), feature = "unsafe_slice"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "unsafe_slice")))]
impl Mem {
    /// View any `T: Sync + Unpin + ?Sized` as `&[u8]`.
    ///
    /// This is a safer interface to [`slice::from_raw_parts`].
    /// # Example
    /// ```
    /// # use devela::Mem;
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
    #[inline]
    #[must_use]
    pub fn as_bytes<'t, T: Sync + Unpin + ?Sized + 't>(v: &T) -> &'t [u8] {
        // SAFETY:
        unsafe { core::slice::from_raw_parts(v as *const _ as *const u8, size_of_val(v)) }
    }

    /// View any `T: Sync + Unpin + ?Sized` as `&mut [u8]`.
    ///
    /// This is a safer interface to [`slice::from_raw_parts_mut`].
    /// # Examples
    /// ```
    /// # use devela::Mem;
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
    #[inline]
    #[must_use]
    pub fn as_bytes_mut<'t, T: Sync + Unpin + ?Sized + 't>(v: &mut T) -> &'t mut [u8] {
        // SAFETY:
        unsafe { core::slice::from_raw_parts_mut(v as *mut _ as *mut u8, size_of_val(v)) }
    }

    /// View any `T: Sync + Unpin + Sized` as `&[u8]` *compile-time* friendly.
    ///
    /// This is a safer interface to [`slice::from_raw_parts`], for `Sized` types.
    /// # Examples
    /// ```
    /// # use devela::Mem;
    /// const DATA: u32 = 1234;
    /// const BYTES: &[u8] = Mem::as_bytes_sized(&DATA);
    ///
    /// if cfg!(target_endian = "little") {
    ///     assert_eq!(BYTES, &[210, 4, 0, 0]);
    /// } else {
    ///     assert_eq!(BYTES, &[0, 0, 4, 210]);
    /// }
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_bytes_sized<T: Sync + Unpin>(v: &T) -> &[u8] {
        // SAFETY:
        unsafe { core::slice::from_raw_parts(v as *const T as *const u8, size_of::<T>()) }
    }
}
