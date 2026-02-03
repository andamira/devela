// devela_base_core::sys::mem::arena::arena::internals
//
//! Defines [`__Arena`].
//!
//! This module groups all the arena-related safety feature-gates.
//

use crate::{MaybeByte, Slice};

#[doc(hidden)]
/// Internal arena methods.
///
/// Abstracts away over safe `u8` or unsafe `MaybeUninit<u8>` representations.
///
/// # Features
/// Uses the `unsafe_array` and `unsafe_slice` features.
#[derive(Debug)]
pub struct __Arena<const CAP: usize>;

#[rustfmt::skip]
impl<const CAP: usize> __Arena<CAP> {
    #[inline(always)]
    ///
    pub const fn new_array<const N: usize>() -> [MaybeByte; N] {
        #[cfg(any(base_safe_mem, not(feature = "unsafe_array")))] // safe
        { [0_u8; N] }
        #[cfg(all(not(base_safe_mem), feature = "unsafe_array"))] // unsafe
        { [MaybeByte::uninit(); N] }
    }

    #[inline(always)]
    ///
    pub const fn read_byte(data: &[MaybeByte], i: usize) -> u8 {
        #[cfg(any(base_safe_mem, not(feature = "unsafe_array")))] // safe
        return data[i];
        #[cfg(all(not(base_safe_mem), feature = "unsafe_array"))] // unsafe
        unsafe { data[i].assume_init_read() }
    }

    #[inline(always)]
    ///
    pub const fn read_byte_mut(data: &mut [MaybeByte], i: usize) -> &mut u8 {
        #[cfg(any(base_safe_mem, not(feature = "unsafe_array")))] // safe
        return &mut data[i];
        #[cfg(all(not(base_safe_mem), feature = "unsafe_array"))] // unsafe
        unsafe { data[i].assume_init_mut() }
    }

    #[inline(always)]
    ///
    pub const fn write_byte(data: &mut [MaybeByte], i: usize, b: u8) {
        #[cfg(any(base_safe_mem, not(feature = "unsafe_array")))] // safe
        { data[i] = b; }
        #[cfg(all(not(base_safe_mem), feature = "unsafe_array"))] // unsafe
        data[i].write(b);
    }

    #[inline(always)]
    ///
    pub const fn slice_bytes(data: &[MaybeByte], start: usize, end: usize) -> &[u8] {
        #[cfg(any(base_safe_mem, not(feature = "unsafe_array")))] // safe
        return Slice::range(data, start, end);
        // unsafe
        #[cfg(all(not(base_safe_mem), feature = "unsafe_array", not(feature = "unsafe_slice")))]
        unsafe { Slice::range(
            Slice::from_raw_parts(data.as_ptr().cast::<u8>(), CAP), start, end) }
        // unsafest
        #[cfg(all(not(base_safe_mem), feature = "unsafe_array", feature = "unsafe_slice"))]
        unsafe { Slice::range_unchecked(
            Slice::from_raw_parts(data.as_ptr().cast::<u8>(), CAP), start, end) }
    }

    #[inline(always)]
    ///
    pub const fn slice_bytes_mut(data: &mut [MaybeByte], start: usize, end: usize) -> &mut [u8] {
        #[cfg(any(base_safe_mem, not(feature = "unsafe_array")))] // safe
        return Slice::range_mut(data, start, end);
        // unsafe
        #[cfg(all(not(base_safe_mem), feature = "unsafe_array", not(feature = "unsafe_slice")))]
        unsafe { Slice::range_mut(
            Slice::from_raw_parts_mut(data.as_mut_ptr().cast::<u8>(), CAP), start, end) }
        // unsafest
        #[cfg(all(not(base_safe_mem), feature = "unsafe_array", feature = "unsafe_slice"))]
        unsafe { Slice::range_mut_unchecked(
            Slice::from_raw_parts_mut(data.as_mut_ptr().cast::<u8>(), CAP), start, end) }
    }
}
