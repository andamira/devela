// devela_base_core::sys::mem::slice::namespace
//
//! Defines the [`Slice`] namespace.
//
// TODO:
// - unchecked subslicing. (split)
// - saturating subslicing.
// - first_chunk_padded

#[cfg(any(doc, unsafe··))]
use crate::Ptr;
use crate::is;
use ::core::slice::{from_mut, from_ref};
#[allow(unused_imports, reason = "unsafe feature-gated")]
use core::slice::{from_raw_parts, from_raw_parts_mut};

mod range;
mod take;
mod split;
mod bytes;

#[cfg(test)]
mod tests;

#[doc = crate::_TAG_NAMESPACE!()]
/// Slice-related operations, all of them *const*.
///
/// It is designed as a utility namespace and does not hold or wrap data itself.
/// Instead, it operates on slices provided directly as arguments to its static methods.
///
/// # Methods
/// - [methods namespaced from `core::slice`](#coreslice-namespaced-methods)
///
/// - [`range*` API methods](#range-api-methods-for-subslicing):</br>
///   - [**range_to**](#method.range_to)
///    ([*checked*](#method.range_to_checked),
///     [*unchecked*](#method.range_to_unchecked),
///     [_**mut**_](#method.range_to_mut),
///     [*mut_checked*](#method.range_to_mut_checked),
///     [*mut_unchecked*](#method.range_to_mut_unchecked)),           ≈ `&slice[..end]`
///   - [**range_to_inclusive**](#method.range_to_inclusive)
///    ([*checked*](#method.range_to_inclusive_checked),
///     [*unchecked*](#method.range_to_inclusive_unchecked),
///     [_**mut**_](#method.range_to_inclusive_mut),
///     [*mut_checked*](#method.range_to_inclusive_mut_checked),
///     [*mut_unchecked*](#method.range_to_inclusive_mut_unchecked)), ≈ `&slice[..=end]`
///   - [**range_from**](#method.range_from),
///    ([*checked*](#method.range_from_checked),
///     [*unchecked*](#method.range_from_unchecked),
///     [_**mut**_](#method.range_from_mut),
///     [*mut_checked*](#method.range_from_mut_checked),
///     [*mut_unchecked*](#method.range_from_mut_unchecked)),         ≈ `&slice[start..]`
///   - [**range**](#method.range)
///    ([*checked*](#method.range_checked),
///     [*unchecked*](#method.range_unchecked),
///     [_**mut**_](#method.range_mut),
///     [*mut_checked*](#method.range_mut_checked),
///     [*mut_unchecked*](#method.range_mut_unchecked)),              ≈ `&slice[start..end]`
///   - [**range_inclusive**](#method.range_inclusive)
///    ([*checked*](#method.range_inclusive_checked),
///     [*unchecked*](#method.range_inclusive_unchecked),
///     [_**mut**_](#method.range_inclusive_mut),
///     [*mut_checked*](#method.range_inclusive_mut_checked),
///     [*mut_unchecked*](#method.range_inclusive_mut_unchecked)).    ≈ `&slice[start..=end]`
///
/// - [`take*` API methods](#take-api-methods-for-subslicing):</br>
///   - [**take_first**](#method.take_first)
///    ([*checked*](#method.take_first_checked),
///     [*unchecked*](#method.take_first_unchecked),
///     [_**mut**_](#method.take_first_mut),
///     [*mut_checked*](#method.take_first_mut_checked),
///     [*mut_unchecked*](#method.take_first_mut_unchecked)),         ≈ `&slice[..n]`
///   - [**take_last**](#method.take_last)
///    ([*checked*](#method.take_last_checked),
///     [*unchecked*](#method.take_last_unchecked),
///     [_**mut**_](#method.take_last_mut),
///     [*mut_checked*](#method.take_last_mut_checked),
///     [*mut_unchecked*](#method.take_last_mut_unchecked)),          ≈ `&slice[len - n..]`
///   - [**take_omit_last**](#method.take_omit_last)
///    ([*checked*](#method.take_omit_last_checked),
///     [*unchecked*](#method.take_omit_last_unchecked),
///     [_**mut**_](#method.take_omit_last_mut),
///     [*mut_checked*](#method.take_omit_last_mut_checked),
///     [*mut_unchecked*](#method.take_omit_last_mut_unchecked)).     ≈ `&slice[..len - n]`
///
/// - [`*split*` API methods](#split-api-methods-for-subslicing):</br>
///   - [**lsplit**](#method.lsplit)
///    ([*mut*](#method.lsplit_mut)),
///   - [**rsplit**](#method.rsplit)
///    ([*mut*](#method.rsplit_mut)),
///   - [**msplit_left**](#method.msplit_left)
///    ([*mut*](#method.msplit_left_mut)),
///   - [**msplit_right**](#method.msplit_right)
///    ([*mut*](#method.msplit_right_mut)).
///
/// - [specific methods for byte slices](#methods-for-byte-slices)
/// - [`eq` methods for slices of (slices of) primitives](#method.eq)
///
/// Additionally implements `eq()` methods for comparing primitives and slices of primitives.
///
/// See also: [`ExtSlice`], [`Mem`][crate::Mem], [`Ptr`][crate::Ptr].
#[doc = crate::doclink!(custom devela "[`ExtSlice`]" "sys/mem/trait.ExtSlice.html")]
#[derive(Debug)]
pub struct Slice<T>(crate::PhantomData<T>);

/// # `core::slice` namespaced methods.
impl<T> Slice<T> {
    /// Copies all elements from `src` into `dst`.
    ///
    /// # Features
    /// - Uses `Ptr::copy_nonoverlapping` when unsafe operations are allowed.
    /// - Falls back to safe element-wise copy otherwise.
    ///
    /// # Panics
    /// Panics if `src` and `dst` slices have different lengths.
    // WAIT:1.87 [const_copy_from_slice](https://github.com/rust-lang/rust/issues/131415)
    #[rustfmt::skip]
    pub const fn copy_from_slice(dst: &mut [T], src: &[T]) where T: Copy {
        if dst.len() != src.len() { panic!("`src` and `dst` slices have different lengths"); }

        #[cfg(all(not(base_safe_mem), unsafe··))]
        // SAFETY: Lengths checked equal, T is Copy, and pointers are valid
        unsafe { Ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), src.len()); }

        #[cfg(any(base_safe_mem, not(unsafe··)))]
        { let mut i = 0; while i < src.len() { dst[i] = src[i]; i += 1; } }
    }

    /// Converts a reference to `T` into a slice of length 1 (without copying).
    ///
    /// See `core::slice::`[`from_ref`].
    #[must_use]
    #[inline(always)]
    pub const fn from_ref(s: &T) -> &[T] {
        from_ref(s)
    }

    /// Converts a reference to `T` into a slice of length 1 (without copying).
    ///
    /// See `core::slice::`[`from_mut`].
    #[must_use]
    #[inline(always)]
    pub const fn from_mut(s: &mut T) -> &mut [T] {
        from_mut(s)
    }

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
