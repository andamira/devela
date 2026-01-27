// devela::num::grain::cast::cast
//
//! fns to cast between primitives in a checked manner.
//
// TOC
// - trait PrimitiveCast
// - macro impl_cast_methods!

use crate::{Cast, Overflow, isize_down, isize_up, usize_down, usize_up};

type Result<T> = crate::Result<T, Overflow>;

#[doc = crate::_tags!(num)]
/// Offers methods for casting between primitives.
#[doc = crate::_doc_location!("num")]
///
/// See also the [`Cast`] type for the equivalent *const* methods.
/// # Errors
/// Checked methods will return [`Overflow`]
/// if the original value can't fit in the returned type.
pub trait PrimitiveCast {
    /* checked */

    /// Casts `self` to `u8` with range check.
    fn checked_cast_to_u8(self) -> Result<u8>;
    /// Casts `self` to `u16` with range check.
    fn checked_cast_to_u16(self) -> Result<u16>;
    /// Casts `self` to `u32` with range check.
    fn checked_cast_to_u32(self) -> Result<u32>;
    /// Casts `self` to `u64` with range check.
    fn checked_cast_to_u64(self) -> Result<u64>;
    /// Casts `self` to `u128` with range check.
    fn checked_cast_to_u128(self) -> Result<u128>;
    /// Casts `self` to `usize` with range check.
    fn checked_cast_to_usize(self) -> Result<usize>;
    /// Casts `self` to `usize_up` with range check.
    fn checked_cast_to_usize_up(self) -> Result<usize_up>;
    /// Casts `self` to `usize_down` with range check.
    fn checked_cast_to_usize_down(self) -> Result<usize_down>;

    /// Casts `self` to `i8` with range check.
    fn checked_cast_to_i8(self) -> Result<i8>;
    /// Casts `self` to `i16` with range check.
    fn checked_cast_to_i16(self) -> Result<i16>;
    /// Casts `self` to `i32` with range check.
    fn checked_cast_to_i32(self) -> Result<i32>;
    /// Casts `self` to `i64` with range check.
    fn checked_cast_to_i64(self) -> Result<i64>;
    /// Casts `self` to `i128` with range check.
    fn checked_cast_to_i128(self) -> Result<i128>;
    /// Casts `self` to `isize` with range check.
    fn checked_cast_to_isize(self) -> Result<isize>;
    /// Casts `self` to `isize_up` with range check.
    fn checked_cast_to_isize_up(self) -> Result<isize_up>;
    /// Casts `self` to `isize_down` with range check.
    fn checked_cast_to_isize_down(self) -> Result<isize_down>;

    /* saturating */

    #[must_use]
    /// Casts `self` to `u8` clamping at the numeric bounds.
    fn saturating_cast_to_u8(self) -> u8;
    #[must_use]
    /// Casts `self` to `u16` clamping at the numeric bounds.
    fn saturating_cast_to_u16(self) -> u16;
    #[must_use]
    /// Casts `self` to `u32` clamping at the numeric bounds.
    fn saturating_cast_to_u32(self) -> u32;
    #[must_use]
    /// Casts `self` to `u64` clamping at the numeric bounds.
    fn saturating_cast_to_u64(self) -> u64;
    #[must_use]
    /// Casts `self` to `u128` clamping at the numeric bounds.
    fn saturating_cast_to_u128(self) -> u128;
    #[must_use]
    /// Casts `self` to `usize` clamping at the numeric bounds.
    fn saturating_cast_to_usize(self) -> usize;
    #[must_use]
    /// Casts `self` to `usize_up` clamping at the numeric bounds.
    fn saturating_cast_to_usize_up(self) -> usize_up;
    #[must_use]
    /// Casts `self` to `usize_down` clamping at the numeric bounds.
    fn saturating_cast_to_usize_down(self) -> usize_down;

    #[must_use]
    /// Casts `self` to `i8` clamping at the numeric bounds.
    fn saturating_cast_to_i8(self) -> i8;
    #[must_use]
    /// Casts `self` to `i16` clamping at the numeric bounds.
    fn saturating_cast_to_i16(self) -> i16;
    #[must_use]
    /// Casts `self` to `i32` clamping at the numeric bounds.
    fn saturating_cast_to_i32(self) -> i32;
    #[must_use]
    /// Casts `self` to `i64` clamping at the numeric bounds.
    fn saturating_cast_to_i64(self) -> i64;
    #[must_use]
    /// Casts `self` to `i128` clamping at the numeric bounds.
    fn saturating_cast_to_i128(self) -> i128;
    #[must_use]
    /// Casts `self` to `isize` clamping at the numeric bounds.
    fn saturating_cast_to_isize(self) -> isize;
    #[must_use]
    /// Casts `self` to `isize_up` clamping at the numeric bounds.
    fn saturating_cast_to_isize_up(self) -> isize_up;
    #[must_use]
    /// Casts `self` to `isize_down` clamping at the numeric bounds.
    fn saturating_cast_to_isize_down(self) -> isize_down;

    /* wrapping */

    #[must_use]
    /// Casts `self` to `u8` wrapping at the numeric bounds.
    fn wrapping_cast_to_u8(self) -> u8;
    #[must_use]
    /// Casts `self` to `u16` wrapping at the numeric bounds.
    fn wrapping_cast_to_u16(self) -> u16;
    #[must_use]
    /// Casts `self` to `u32` wrapping at the numeric bounds.
    fn wrapping_cast_to_u32(self) -> u32;
    #[must_use]
    /// Casts `self` to `u64` wrapping at the numeric bounds.
    fn wrapping_cast_to_u64(self) -> u64;
    #[must_use]
    /// Casts `self` to `u128` wrapping at the numeric bounds.
    fn wrapping_cast_to_u128(self) -> u128;
    #[must_use]
    /// Casts `self` to `usize` wrapping at the numeric bounds.
    fn wrapping_cast_to_usize(self) -> usize;
    #[must_use]
    /// Casts `self` to `usize_up` wrapping at the numeric bounds.
    fn wrapping_cast_to_usize_up(self) -> usize_up;
    #[must_use]
    /// Casts `self` to `usize_down` wrapping at the numeric bounds.
    fn wrapping_cast_to_usize_down(self) -> usize_down;

    #[must_use]
    /// Casts `self` to `i8` wrapping at the numeric bounds.
    fn wrapping_cast_to_i8(self) -> i8;
    #[must_use]
    /// Casts `self` to `i16` wrapping at the numeric bounds.
    fn wrapping_cast_to_i16(self) -> i16;
    #[must_use]
    /// Casts `self` to `i32` wrapping at the numeric bounds.
    fn wrapping_cast_to_i32(self) -> i32;
    #[must_use]
    /// Casts `self` to `i64` wrapping at the numeric bounds.
    fn wrapping_cast_to_i64(self) -> i64;
    #[must_use]
    /// Casts `self` to `i128` wrapping at the numeric bounds.
    fn wrapping_cast_to_i128(self) -> i128;
    #[must_use]
    /// Casts `self` to `isize` wrapping at the numeric bounds.
    fn wrapping_cast_to_isize(self) -> isize;
    #[must_use]
    /// Casts `self` to `isize_up` wrapping at the numeric bounds.
    fn wrapping_cast_to_isize_up(self) -> isize_up;
    #[must_use]
    /// Casts `self` to `isize_down` wrapping at the numeric bounds.
    fn wrapping_cast_to_isize_down(self) -> isize_down;
}

/// Implements the public casting methods for the trait and [`Cast`] wrapper.
macro_rules! impl_cast_methods {
    () => {
        impl_cast_methods![u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];
    };
    ($($t:ty),+) => { $( impl_cast_methods![@$t]; )+ };
    (@$t:ty) => { crate::paste! {
        impl PrimitiveCast for $t {
            #[inline(always)]
            fn checked_cast_to_u8(self) -> Result<u8> { Cast(self).checked_cast_to_u8() }
            #[inline(always)]
            fn checked_cast_to_u16(self) -> Result<u16> { Cast(self).checked_cast_to_u16() }
            #[inline(always)]
            fn checked_cast_to_u32(self) -> Result<u32> { Cast(self).checked_cast_to_u32() }
            #[inline(always)]
            fn checked_cast_to_u64(self) -> Result<u64> { Cast(self).checked_cast_to_u64() }
            #[inline(always)]
            fn checked_cast_to_u128(self) -> Result<u128> { Cast(self).checked_cast_to_u128() }
            #[inline(always)]
            fn checked_cast_to_usize(self) -> Result<usize> { Cast(self).checked_cast_to_usize() }
            #[inline(always)]
            fn checked_cast_to_usize_up(self) -> Result<usize_up> {
                Cast(self).checked_cast_to_usize_up() }
            #[inline(always)]
            fn checked_cast_to_usize_down(self) -> Result<usize_down> {
                Cast(self).checked_cast_to_usize_down() }

            #[inline(always)]
            fn checked_cast_to_i8(self) -> Result<i8> { Cast(self).checked_cast_to_i8() }
            #[inline(always)]
            fn checked_cast_to_i16(self) -> Result<i16> { Cast(self).checked_cast_to_i16() }
            #[inline(always)]
            fn checked_cast_to_i32(self) -> Result<i32> { Cast(self).checked_cast_to_i32() }
            #[inline(always)]
            fn checked_cast_to_i64(self) -> Result<i64> { Cast(self).checked_cast_to_i64() }
            #[inline(always)]
            fn checked_cast_to_i128(self) -> Result<i128> { Cast(self).checked_cast_to_i128() }
            #[inline(always)]
            fn checked_cast_to_isize(self) -> Result<isize> { Cast(self).checked_cast_to_isize() }
            #[inline(always)]
            fn checked_cast_to_isize_up(self) -> Result<isize_up> {
                Cast(self).checked_cast_to_isize_up() }
            #[inline(always)]
            fn checked_cast_to_isize_down(self) -> Result<isize_down> {
                Cast(self).checked_cast_to_isize_down() }

            #[inline(always)]
            fn saturating_cast_to_u8(self) -> u8 { Cast(self).saturating_cast_to_u8() }
            #[inline(always)]
            fn saturating_cast_to_u16(self) -> u16 { Cast(self).saturating_cast_to_u16() }
            #[inline(always)]
            fn saturating_cast_to_u32(self) -> u32 { Cast(self).saturating_cast_to_u32() }
            #[inline(always)]
            fn saturating_cast_to_u64(self) -> u64 { Cast(self).saturating_cast_to_u64() }
            #[inline(always)]
            fn saturating_cast_to_u128(self) -> u128 { Cast(self).saturating_cast_to_u128() }
            #[inline(always)]
            fn saturating_cast_to_usize(self) -> usize { Cast(self).saturating_cast_to_usize() }
            #[inline(always)]
            fn saturating_cast_to_usize_up(self) -> usize_up {
                Cast(self).saturating_cast_to_usize_up() }
            #[inline(always)]
            fn saturating_cast_to_usize_down(self) -> usize_down {
                Cast(self).saturating_cast_to_usize_down() }

            #[inline(always)]
            fn saturating_cast_to_i8(self) -> i8 { Cast(self).saturating_cast_to_i8() }
            #[inline(always)]
            fn saturating_cast_to_i16(self) -> i16 { Cast(self).saturating_cast_to_i16() }
            #[inline(always)]
            fn saturating_cast_to_i32(self) -> i32 { Cast(self).saturating_cast_to_i32() }
            #[inline(always)]
            fn saturating_cast_to_i64(self) -> i64 { Cast(self).saturating_cast_to_i64() }
            #[inline(always)]
            fn saturating_cast_to_i128(self) -> i128 { Cast(self).saturating_cast_to_i128() }
            #[inline(always)]
            fn saturating_cast_to_isize(self) -> isize { Cast(self).saturating_cast_to_isize() }
            #[inline(always)]
            fn saturating_cast_to_isize_up(self) -> isize_up {
                Cast(self).saturating_cast_to_isize_up() }
            #[inline(always)]
            fn saturating_cast_to_isize_down(self) -> isize_down {
                Cast(self).saturating_cast_to_isize_down() }

            #[inline(always)]
            fn wrapping_cast_to_u8(self) -> u8 { Cast(self).wrapping_cast_to_u8() }
            #[inline(always)]
            fn wrapping_cast_to_u16(self) -> u16 { Cast(self).wrapping_cast_to_u16() }
            #[inline(always)]
            fn wrapping_cast_to_u32(self) -> u32 { Cast(self).wrapping_cast_to_u32() }
            #[inline(always)]
            fn wrapping_cast_to_u64(self) -> u64 { Cast(self).wrapping_cast_to_u64() }
            #[inline(always)]
            fn wrapping_cast_to_u128(self) -> u128 { Cast(self).wrapping_cast_to_u128() }
            #[inline(always)]
            fn wrapping_cast_to_usize(self) -> usize { Cast(self).wrapping_cast_to_usize() }
            #[inline(always)]
            fn wrapping_cast_to_usize_up(self) -> usize_up {
                Cast(self).wrapping_cast_to_usize_up() }
            #[inline(always)]
            fn wrapping_cast_to_usize_down(self) -> usize_down {
                Cast(self).wrapping_cast_to_usize_down() }

            #[inline(always)]
            fn wrapping_cast_to_i8(self) -> i8 { Cast(self).wrapping_cast_to_i8() }
            #[inline(always)]
            fn wrapping_cast_to_i16(self) -> i16 { Cast(self).wrapping_cast_to_i16() }
            #[inline(always)]
            fn wrapping_cast_to_i32(self) -> i32 { Cast(self).wrapping_cast_to_i32() }
            #[inline(always)]
            fn wrapping_cast_to_i64(self) -> i64 { Cast(self).wrapping_cast_to_i64() }
            #[inline(always)]
            fn wrapping_cast_to_i128(self) -> i128 { Cast(self).wrapping_cast_to_i128() }
            #[inline(always)]
            fn wrapping_cast_to_isize(self) -> isize { Cast(self).wrapping_cast_to_isize() }
            #[inline(always)]
            fn wrapping_cast_to_isize_up(self) -> isize_up {
                Cast(self).wrapping_cast_to_isize_up() }
            #[inline(always)]
            fn wrapping_cast_to_isize_down(self) -> isize_down {
                Cast(self).wrapping_cast_to_isize_down() }
        }
    }};
}
impl_cast_methods![];
