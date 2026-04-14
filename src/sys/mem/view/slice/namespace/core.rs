// devela::sys::mem::view::slice::namespace::core

use crate::{Slice, is};
use ::core::slice::{from_mut, from_ref};
#[allow(unused_imports, reason = "unsafe feature-gated")]
use ::core::slice::{from_raw_parts, from_raw_parts_mut};

/// # `core::slice` namespaced methods.
#[rustfmt::skip]
impl<T> Slice<T> {
    /// Clones all elements from `src` into `dst`.
    ///
    /// # Panics
    /// Panics if `src` and `dst` slices have different lengths.
    ///
    /// See `core::slice::`[`clone_from_slice`][slice#method.clone_from_slice].
    #[inline(always)]
    pub fn clone(dst: &mut [T], src: &[T]) where T: Clone { dst.clone_from_slice(src); }

    /// Copies all elements from `src` into `dst` using a memcpy.
    ///
    /// # Panics
    /// Panics if `src` and `dst` slices have different lengths.
    ///
    /// See `core::slice::`[`copy_from_slice`][slice#method.copy_from_slice].
    #[inline(always)]
    pub const fn copy(dst: &mut [T], src: &[T]) where T: Copy { dst.copy_from_slice(src); }

    /// Returns a shared reference to the element at `index`, or `None` if out of bounds.
    ///
    /// This is a stable const analogue just for single-element checked access.
    ///
    /// See `core::slice::`[`get`][slice#method.get].
    #[must_use]
    #[inline(always)]
    pub const fn get(slice: &[T], index: usize) -> Option<&T> {
        is! { index < slice.len(), Some(&slice[index]), None }
    }
    /// Returns an exclusive reference to the element at `index`, or `None` if out of bounds.
    ///
    /// This is a stable const analogue just for single-element checked access.
    ///
    /// See `core::slice::`[`get_mut`][slice#method.get_mut].
    #[must_use]
    #[inline(always)]
    pub const fn get_mut(slice: &mut [T], index: usize) -> Option<&mut T> {
        is! { index < slice.len(), Some(&mut slice[index]), None }
    }

    /// Converts a reference to `T` into a slice of length 1 (without copying).
    ///
    /// See `core::slice::`[`from_ref`].
    #[must_use]
    #[inline(always)]
    pub const fn from_ref(s: &T) -> &[T] { from_ref(s) }

    /// Converts a reference to `T` into a slice of length 1 (without copying).
    ///
    /// See `core::slice::`[`from_mut`].
    #[must_use]
    #[inline(always)]
    pub const fn from_mut(s: &mut T) -> &mut [T] { from_mut(s) }

    /// Forms a shared slice from a pointer and a length.
    ///
    /// # Safety
    /// See `core::slice::`[`from_raw_parts`]
    ///
    /// See also `Ptr::`[`slice_from_raw_parts`][crate::Ptr::slice_from_raw_parts].
    #[inline(always)]
    #[cfg_attr(nightly_doc, doc(cfg(unsafe··)))]
    #[cfg(all(not(feature = "safe_mem"), unsafe··))]
    pub const unsafe fn from_raw_parts<'a>(data: *const T, len: usize) -> &'a [T] {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { from_raw_parts(data, len) }
    }

    /// Forms an exclusive slice from a pointer and a length.
    ///
    /// # Safety
    /// See `core::slice::`[`from_raw_parts_mut`].
    ///
    /// See also `Ptr::`[`slice_from_raw_parts_mut`][crate::Ptr::slice_from_raw_parts_mut].
    #[inline(always)]
    #[cfg_attr(nightly_doc, doc(cfg(unsafe··)))]
    #[cfg(all(not(feature = "safe_mem"), unsafe··))]
    pub const unsafe fn from_raw_parts_mut<'a>(data: *mut T, len: usize) -> &'a mut [T] {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { from_raw_parts_mut(data, len) }
    }
}
