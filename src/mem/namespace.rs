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
/// See also the [`std::mem`] module.
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
    //     // SAFETY: Caller must uphold the safety contract.
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
