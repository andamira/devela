// devela::sys::mem::arena::arena::internals
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
        cfg_select! { all(feature = "unsafe_array", not(feature = "safe_mem")) => {
            [MaybeByte::uninit(); N]
        } _ => { [0_u8; N] }}
    }

    #[inline(always)]
    ///
    pub const fn read_byte(data: &[MaybeByte], i: usize) -> u8 {
        cfg_select! { all(feature = "unsafe_array", not(feature = "safe_mem")) => {
            unsafe { data[i].assume_init_read() }
        } _ => { data[i] }}
    }

    #[inline(always)]
    ///
    pub const fn read_byte_mut(data: &mut [MaybeByte], i: usize) -> &mut u8 {
        cfg_select! { all(feature = "unsafe_array", not(feature = "safe_mem")) => {
            unsafe { data[i].assume_init_mut() }
        } _ => { &mut data[i] }}
    }

    #[inline(always)]
    ///
    pub const fn write_byte(data: &mut [MaybeByte], i: usize, b: u8) {
        cfg_select! { all(feature = "unsafe_array", not(feature = "safe_mem")) => {
            data[i].write(b);
        } _ => { data[i] = b; }}
    }

    #[inline(always)]
    ///
    pub const fn slice_bytes(data: &[MaybeByte], start: usize, end: usize) -> &[u8] {
        cfg_select! { // unsafest, unsafe, safe:
            all(feature = "unsafe_array", feature = "unsafe_slice", not(feature = "safe_mem")) => {
                unsafe { Slice::range_unchecked(
                    Slice::from_raw_parts(data.as_ptr().cast::<u8>(), CAP), start, end)
            }} all(feature = "unsafe_array", not(feature = "safe_mem")) => {
                unsafe { Slice::range(Slice::from_raw_parts(
                    data.as_ptr().cast::<u8>(), CAP), start, end)
            }}
            _ => { Slice::range(data, start, end) }}
    }

    #[inline(always)]
    ///
    pub const fn slice_bytes_mut(data: &mut [MaybeByte], start: usize, end: usize) -> &mut [u8] {
        cfg_select! { // unsafest, unsafe, safe:
            all(feature = "unsafe_array", feature = "unsafe_slice", not(feature = "safe_mem")) => {
                unsafe { Slice::range_mut_unchecked(
                    Slice::from_raw_parts_mut(data.as_mut_ptr().cast::<u8>(), CAP), start, end)
            }} all(feature = "unsafe_array", not(feature = "safe_mem")) => {
                unsafe { Slice::range_mut(Slice::from_raw_parts_mut(
                    data.as_mut_ptr().cast::<u8>(), CAP), start, end)
            }}
            _ => { Slice::range_mut(data, start, end) }}
    }
}
