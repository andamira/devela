// devela_base_core::sys::mem::view::slice::namespace::core

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
    #[cfg(all(not(base_safe_mem), unsafe··))]
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
    #[cfg(all(not(base_safe_mem), unsafe··))]
    pub const unsafe fn from_raw_parts_mut<'a>(data: *mut T, len: usize) -> &'a mut [T] {
        // SAFETY: Caller must uphold the safety contract.
        unsafe { from_raw_parts_mut(data, len) }
    }
}

/// Helper for implementing slice operations for primitives.
macro_rules! impl_prim {
    () => {
        impl_prim![
            u8, u16, u32, u64, u128, usize,
            i8, i16, i32, i64, i128, isize,
            f32, f64,
            bool, char
        ];
        #[cfg(nightly_float)]
        impl_prim![f16, f128];
    };
    ($($t:ty),+) => { $( impl_prim![@$t]; )+ };
    (@$t:ty) => {
        impl Slice<$t> {
            /// Checks the equality of two slices of primitives in compile-time.
            #[must_use]
            pub const fn eq(a: &[$t], b: &[$t]) -> bool {
                is! { a.len() != b.len(); return false }
                let mut i = 0;
                while i < a.len() {
                    is! { a[i] != b[i]; return false }
                    i += 1;
                }
                true
            }
        }
        impl Slice<&[$t]> {
            /// Checks the equality of two slices of slices of primitives in compile-time.
            #[must_use]
            pub const fn eq(a: &[&[$t]], b: &[&[$t]]) -> bool {
                is! { a.len() != b.len(); return false }
                let mut i = 0;
                while i < a.len() {
                    is! { !Slice::<$t>::eq(a[i], b[i]); return false }
                    i += 1;
                }
                true
            }
        }
    };
}
impl_prim!();

/// # Methods for string slices.
impl Slice<&str> {
    /// Checks the equality of two string slices in compile-time.
    #[must_use]
    pub const fn eq(a: &str, b: &str) -> bool {
        Slice::<u8>::eq(a.as_bytes(), b.as_bytes())
    }
}
impl Slice<&[&str]> {
    /// Checks the equality of two slices of string slices in compile-time.
    #[must_use]
    pub const fn eq(a: &[&str], b: &[&str]) -> bool {
        is! { a.len() != b.len(); return false }
        let mut i = 0;
        while i < a.len() {
            is! { !Slice::<&str>::eq(a[i], b[i]); return false }
            i += 1;
        }
        true
    }
}
