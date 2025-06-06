// devela::data::conversion::cast::casts
//
//! fns to cast between primitives in a checked manner.
//
// TOC
// - trait definition
// - trait and wrapper implementation
// - functions definitions

use crate::{
    Cast,
    NumError::Overflow,
    NumResult as Result,
    Sign::{Negative, Positive},
    is, isize_down, isize_up, paste, usize_down, usize_up,
};

/// Offers methods for casting between primitives.
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

    /// Casts `self` to `u8` clamping at the numeric bounds.
    #[must_use]
    fn saturating_cast_to_u8(self) -> u8;
    /// Casts `self` to `u16` clamping at the numeric bounds.
    #[must_use]
    fn saturating_cast_to_u16(self) -> u16;
    /// Casts `self` to `u32` clamping at the numeric bounds.
    #[must_use]
    fn saturating_cast_to_u32(self) -> u32;
    /// Casts `self` to `u64` clamping at the numeric bounds.
    #[must_use]
    fn saturating_cast_to_u64(self) -> u64;
    /// Casts `self` to `u128` clamping at the numeric bounds.
    #[must_use]
    fn saturating_cast_to_u128(self) -> u128;
    /// Casts `self` to `usize` clamping at the numeric bounds.
    #[must_use]
    fn saturating_cast_to_usize(self) -> usize;
    /// Casts `self` to `usize_up` clamping at the numeric bounds.
    #[must_use]
    fn saturating_cast_to_usize_up(self) -> usize_up;
    /// Casts `self` to `usize_down` clamping at the numeric bounds.
    #[must_use]
    fn saturating_cast_to_usize_down(self) -> usize_down;

    /// Casts `self` to `i8` clamping at the numeric bounds.
    #[must_use]
    fn saturating_cast_to_i8(self) -> i8;
    /// Casts `self` to `i16` clamping at the numeric bounds.
    #[must_use]
    fn saturating_cast_to_i16(self) -> i16;
    /// Casts `self` to `i32` clamping at the numeric bounds.
    #[must_use]
    fn saturating_cast_to_i32(self) -> i32;
    /// Casts `self` to `i64` clamping at the numeric bounds.
    #[must_use]
    fn saturating_cast_to_i64(self) -> i64;
    /// Casts `self` to `i128` clamping at the numeric bounds.
    #[must_use]
    fn saturating_cast_to_i128(self) -> i128;
    /// Casts `self` to `isize` clamping at the numeric bounds.
    #[must_use]
    fn saturating_cast_to_isize(self) -> isize;
    /// Casts `self` to `isize_up` clamping at the numeric bounds.
    #[must_use]
    fn saturating_cast_to_isize_up(self) -> isize_up;
    /// Casts `self` to `isize_down` clamping at the numeric bounds.
    #[must_use]
    fn saturating_cast_to_isize_down(self) -> isize_down;

    /* wrapping */

    /// Casts `self` to `u8` wrapping at the numeric bounds.
    #[must_use]
    fn wrapping_cast_to_u8(self) -> u8;
    /// Casts `self` to `u16` wrapping at the numeric bounds.
    #[must_use]
    fn wrapping_cast_to_u16(self) -> u16;
    /// Casts `self` to `u32` wrapping at the numeric bounds.
    #[must_use]
    fn wrapping_cast_to_u32(self) -> u32;
    /// Casts `self` to `u64` wrapping at the numeric bounds.
    #[must_use]
    fn wrapping_cast_to_u64(self) -> u64;
    /// Casts `self` to `u128` wrapping at the numeric bounds.
    #[must_use]
    fn wrapping_cast_to_u128(self) -> u128;
    /// Casts `self` to `usize` wrapping at the numeric bounds.
    #[must_use]
    fn wrapping_cast_to_usize(self) -> usize;
    /// Casts `self` to `usize_up` wrapping at the numeric bounds.
    #[must_use]
    fn wrapping_cast_to_usize_up(self) -> usize_up;
    /// Casts `self` to `usize_down` wrapping at the numeric bounds.
    #[must_use]
    fn wrapping_cast_to_usize_down(self) -> usize_down;

    /// Casts `self` to `i8` wrapping at the numeric bounds.
    #[must_use]
    fn wrapping_cast_to_i8(self) -> i8;
    /// Casts `self` to `i16` wrapping at the numeric bounds.
    #[must_use]
    fn wrapping_cast_to_i16(self) -> i16;
    /// Casts `self` to `i32` wrapping at the numeric bounds.
    #[must_use]
    fn wrapping_cast_to_i32(self) -> i32;
    /// Casts `self` to `i64` wrapping at the numeric bounds.
    #[must_use]
    fn wrapping_cast_to_i64(self) -> i64;
    /// Casts `self` to `i128` wrapping at the numeric bounds.
    #[must_use]
    fn wrapping_cast_to_i128(self) -> i128;
    /// Casts `self` to `isize` wrapping at the numeric bounds.
    #[must_use]
    fn wrapping_cast_to_isize(self) -> isize;
    /// Casts `self` to `isize_up` wrapping at the numeric bounds.
    #[must_use]
    fn wrapping_cast_to_isize_up(self) -> isize_up;
    /// Casts `self` to `isize_down` wrapping at the numeric bounds.
    #[must_use]
    fn wrapping_cast_to_isize_down(self) -> isize_down;
}

/// Implements the public casting methods for the trait and [`Cast`] wrapper.
macro_rules! impl_cast_methods {
    ($($t:ty),+) => { $( impl_cast_methods![@$t]; )+ };
    (@$t:ty) => { paste! {
        impl PrimitiveCast for $t {
            fn checked_cast_to_u8(self) -> Result<u8> { [<checked_cast_ $t _to_u8>](self) }
            fn checked_cast_to_u16(self) -> Result<u16> { [<checked_cast_ $t _to_u16>](self) }
            fn checked_cast_to_u32(self) -> Result<u32> { [<checked_cast_ $t _to_u32>](self) }
            fn checked_cast_to_u64(self) -> Result<u64> { [<checked_cast_ $t _to_u64>](self) }
            fn checked_cast_to_u128(self) -> Result<u128> { [<checked_cast_ $t _to_u128>](self) }
            fn checked_cast_to_usize(self) -> Result<usize> { [<checked_cast_ $t _to_usize>](self) }
            fn checked_cast_to_usize_up(self) -> Result<usize_up> {
                [<checked_cast_ $t _to_usize_up>](self) }
            fn checked_cast_to_usize_down(self) -> Result<usize_down> {
                [<checked_cast_ $t _to_usize_down>](self) }

            fn checked_cast_to_i8(self) -> Result<i8> { [<checked_cast_ $t _to_i8>](self) }
            fn checked_cast_to_i16(self) -> Result<i16> { [<checked_cast_ $t _to_i16>](self) }
            fn checked_cast_to_i32(self) -> Result<i32> { [<checked_cast_ $t _to_i32>](self) }
            fn checked_cast_to_i64(self) -> Result<i64> { [<checked_cast_ $t _to_i64>](self) }
            fn checked_cast_to_i128(self) -> Result<i128> { [<checked_cast_ $t _to_i128>](self) }
            fn checked_cast_to_isize(self) -> Result<isize> { [<checked_cast_ $t _to_isize>](self) }
            fn checked_cast_to_isize_up(self) -> Result<isize_up> {
                [<checked_cast_ $t _to_isize_up>](self) }
            fn checked_cast_to_isize_down(self) -> Result<isize_down> {
                [<checked_cast_ $t _to_isize_down>](self) }

            fn saturating_cast_to_u8(self) -> u8 { [<saturating_cast_ $t _to_u8>](self) }
            fn saturating_cast_to_u16(self) -> u16 { [<saturating_cast_ $t _to_u16>](self) }
            fn saturating_cast_to_u32(self) -> u32 { [<saturating_cast_ $t _to_u32>](self) }
            fn saturating_cast_to_u64(self) -> u64 { [<saturating_cast_ $t _to_u64>](self) }
            fn saturating_cast_to_u128(self) -> u128 { [<saturating_cast_ $t _to_u128>](self) }
            fn saturating_cast_to_usize(self) -> usize { [<saturating_cast_ $t _to_usize>](self) }
            fn saturating_cast_to_usize_up(self) -> usize_up {
                [<saturating_cast_ $t _to_usize_up>](self) }
            fn saturating_cast_to_usize_down(self) -> usize_down {
                [<saturating_cast_ $t _to_usize_down>](self) }

            fn saturating_cast_to_i8(self) -> i8 { [<saturating_cast_ $t _to_i8>](self) }
            fn saturating_cast_to_i16(self) -> i16 { [<saturating_cast_ $t _to_i16>](self) }
            fn saturating_cast_to_i32(self) -> i32 { [<saturating_cast_ $t _to_i32>](self) }
            fn saturating_cast_to_i64(self) -> i64 { [<saturating_cast_ $t _to_i64>](self) }
            fn saturating_cast_to_i128(self) -> i128 { [<saturating_cast_ $t _to_i128>](self) }
            fn saturating_cast_to_isize(self) -> isize { [<saturating_cast_ $t _to_isize>](self) }
            fn saturating_cast_to_isize_up(self) -> isize_up {
                [<saturating_cast_ $t _to_isize_up>](self) }
            fn saturating_cast_to_isize_down(self) -> isize_down {
                [<saturating_cast_ $t _to_isize_down>](self) }

            fn wrapping_cast_to_u8(self) -> u8 { [<wrapping_cast_ $t _to_u8>](self) }
            fn wrapping_cast_to_u16(self) -> u16 { [<wrapping_cast_ $t _to_u16>](self) }
            fn wrapping_cast_to_u32(self) -> u32 { [<wrapping_cast_ $t _to_u32>](self) }
            fn wrapping_cast_to_u64(self) -> u64 { [<wrapping_cast_ $t _to_u64>](self) }
            fn wrapping_cast_to_u128(self) -> u128 { [<wrapping_cast_ $t _to_u128>](self) }
            fn wrapping_cast_to_usize(self) -> usize { [<wrapping_cast_ $t _to_usize>](self) }
            fn wrapping_cast_to_usize_up(self) -> usize_up {
                [<wrapping_cast_ $t _to_usize_up>](self) }
            fn wrapping_cast_to_usize_down(self) -> usize_down {
                [<wrapping_cast_ $t _to_usize_down>](self) }

            fn wrapping_cast_to_i8(self) -> i8 { [<wrapping_cast_ $t _to_i8>](self) }
            fn wrapping_cast_to_i16(self) -> i16 { [<wrapping_cast_ $t _to_i16>](self) }
            fn wrapping_cast_to_i32(self) -> i32 { [<wrapping_cast_ $t _to_i32>](self) }
            fn wrapping_cast_to_i64(self) -> i64 { [<wrapping_cast_ $t _to_i64>](self) }
            fn wrapping_cast_to_i128(self) -> i128 { [<wrapping_cast_ $t _to_i128>](self) }
            fn wrapping_cast_to_isize(self) -> isize { [<wrapping_cast_ $t _to_isize>](self) }
            fn wrapping_cast_to_isize_up(self) -> isize_up {
                [<wrapping_cast_ $t _to_isize_up>](self) }
            fn wrapping_cast_to_isize_down(self) -> isize_down {
                [<wrapping_cast_ $t _to_isize_down>](self) }
        }

        /// Checked casts and saturating casts.
        ///
        /// # Errors
        /// Checked methods will return [`Overflow`] if `self` can't fit in the returned type.
        #[cfg_attr(nightly_doc, doc(cfg(feature = "cast")))]
        impl Cast<$t> {
            /* checked */

            #[doc = "Casts from `" $t "` to `u8` with range check."]
            pub const fn checked_cast_to_u8(self) -> Result<u8> {
                [<checked_cast_ $t _to_u8>](self.0)
            }
            #[doc = "Casts from `" $t "` to `u16` with range check."]
            pub const fn checked_cast_to_u16(self) -> Result<u16> {
                [<checked_cast_ $t _to_u16>](self.0)
            }
            #[doc = "Casts from `" $t "` to `u32` with range check."]
            pub const fn checked_cast_to_u32(self) -> Result<u32> {
                [<checked_cast_ $t _to_u32>](self.0)
            }
            #[doc = "Casts from `" $t "` to `u64` with range check."]
            pub const fn checked_cast_to_u64(self) -> Result<u64> {
                [<checked_cast_ $t _to_u64>](self.0)
            }
            #[doc = "Casts from `" $t "` to `u128` with range check."]
            pub const fn checked_cast_to_u128(self) -> Result<u128> {
                [<checked_cast_ $t _to_u128>](self.0)
            }
            #[doc = "Casts from `" $t "` to `usize` with range check."]
            pub const fn checked_cast_to_usize(self) -> Result<usize> {
                [<checked_cast_ $t _to_usize>](self.0)
            }
            #[doc = "Casts from `" $t "` to `usize_up` with range check."]
            pub const fn checked_cast_to_usize_up(self) -> Result<usize_up> {
                [<checked_cast_ $t _to_usize_up>](self.0)
            }
            #[doc = "Casts from `" $t "` to `usize_down` with range check."]
            pub const fn checked_cast_to_usize_down(self) -> Result<usize_down> {
                [<checked_cast_ $t _to_usize_down>](self.0)
            }

            #[doc = "Casts from `" $t "` to `i8` with range check."]
            pub const fn checked_cast_to_i8(self) -> Result<i8> {
                [<checked_cast_ $t _to_i8>](self.0)
            }
            #[doc = "Casts from `" $t "` to `i16` with range check."]
            pub const fn checked_cast_to_i16(self) -> Result<i16> {
                [<checked_cast_ $t _to_i16>](self.0)
            }
            #[doc = "Casts from `" $t "` to `i32` with range check."]
            pub const fn checked_cast_to_i32(self) -> Result<i32> {
                [<checked_cast_ $t _to_i32>](self.0)
            }
            #[doc = "Casts from `" $t "` to `i64` with range check."]
            pub const fn checked_cast_to_i64(self) -> Result<i64> {
                [<checked_cast_ $t _to_i64>](self.0)
            }
            #[doc = "Casts from `" $t "` to `i128` with range check."]
            pub const fn checked_cast_to_i128(self) -> Result<i128> {
                [<checked_cast_ $t _to_i128>](self.0)
            }
            #[doc = "Casts from `" $t "` to `isize` with range check."]
            pub const fn checked_cast_to_isize(self) -> Result<isize> {
                [<checked_cast_ $t _to_isize>](self.0)
            }
            #[doc = "Casts from `" $t "` to `isize_up` with range check."]
            pub const fn checked_cast_to_isize_up(self) -> Result<isize_up> {
                [<checked_cast_ $t _to_isize_up>](self.0)
            }
            #[doc = "Casts from `" $t "` to `isize_down` with range check."]
            pub const fn checked_cast_to_isize_down(self) -> Result<isize_down> {
                [<checked_cast_ $t _to_isize_down>](self.0)
            }

            /* saturating */

            #[doc = "Casts from `" $t "` to `u8` clamping at the numeric bounds."]
            #[must_use]
            pub const fn saturating_cast_to_u8(self) -> u8 {
                [<saturating_cast_ $t _to_u8>](self.0)
            }
            #[doc = "Casts from `" $t "` to `u16` clamping at the numeric bounds."]
            #[must_use]
            pub const fn saturating_cast_to_u16(self) -> u16 {
                [<saturating_cast_ $t _to_u16>](self.0)
            }
            #[doc = "Casts from `" $t "` to `u32` clamping at the numeric bounds."]
            #[must_use]
            pub const fn saturating_cast_to_u32(self) -> u32 {
                [<saturating_cast_ $t _to_u32>](self.0)
            }
            #[doc = "Casts from `" $t "` to `u64` clamping at the numeric bounds."]
            #[must_use]
            pub const fn saturating_cast_to_u64(self) -> u64 {
                [<saturating_cast_ $t _to_u64>](self.0)
            }
            #[doc = "Casts from `" $t "` to `u128` clamping at the numeric bounds."]
            #[must_use]
            pub const fn saturating_cast_to_u128(self) -> u128 {
                [<saturating_cast_ $t _to_u128>](self.0)
            }
            #[doc = "Casts from `" $t "` to `usize` clamping at the numeric bounds."]
            #[must_use]
            pub const fn saturating_cast_to_usize(self) -> usize {
                [<saturating_cast_ $t _to_usize>](self.0)
            }
            #[doc = "Casts from `" $t "` to `usize_up` clamping at the numeric bounds."]
            #[must_use]
            pub const fn saturating_cast_to_usize_up(self) -> usize_up {
                [<saturating_cast_ $t _to_usize_up>](self.0)
            }
            #[doc = "Casts from `" $t "` to `usize_down` clamping at the numeric bounds."]
            #[must_use]
            pub const fn saturating_cast_to_usize_down(self) -> usize_down {
                [<saturating_cast_ $t _to_usize_down>](self.0)
            }

            #[doc = "Casts from `" $t "` to `i8` clamping at the numeric bounds."]
            #[must_use]
            pub const fn saturating_cast_to_i8(self) -> i8 {
                [<saturating_cast_ $t _to_i8>](self.0)
            }
            #[doc = "Casts from `" $t "` to `i16` clamping at the numeric bounds."]
            #[must_use]
            pub const fn saturating_cast_to_i16(self) -> i16 {
                [<saturating_cast_ $t _to_i16>](self.0)
            }
            #[doc = "Casts from `" $t "` to `i32` clamping at the numeric bounds."]
            #[must_use]
            pub const fn saturating_cast_to_i32(self) -> i32 {
                [<saturating_cast_ $t _to_i32>](self.0)
            }
            #[doc = "Casts from `" $t "` to `i64` clamping at the numeric bounds."]
            #[must_use]
            pub const fn saturating_cast_to_i64(self) -> i64 {
                [<saturating_cast_ $t _to_i64>](self.0)
            }
            #[doc = "Casts from `" $t "` to `i128` clamping at the numeric bounds."]
            #[must_use]
            pub const fn saturating_cast_to_i128(self) -> i128 {
                [<saturating_cast_ $t _to_i128>](self.0)
            }
            #[doc = "Casts from `" $t "` to `isize` clamping at the numeric bounds."]
            #[must_use]
            pub const fn saturating_cast_to_isize(self) -> isize {
                [<saturating_cast_ $t _to_isize>](self.0)
            }
            #[doc = "Casts from `" $t "` to `isize_up` clamping at the numeric bounds."]
            #[must_use]
            pub const fn saturating_cast_to_isize_up(self) -> isize_up {
                [<saturating_cast_ $t _to_isize_up>](self.0)
            }
            #[doc = "Casts from `" $t "` to `isize_down` clamping at the numeric bounds."]
            #[must_use]
            pub const fn saturating_cast_to_isize_down(self) -> isize_down {
                [<saturating_cast_ $t _to_isize_down>](self.0)
            }

            /* wrapping */

            #[doc = "Casts from `" $t "` to `u8` wrapping at the numeric bounds."]
            #[must_use]
            pub const fn wrapping_cast_to_u8(self) -> u8 {
                [<wrapping_cast_ $t _to_u8>](self.0)
            }
            #[doc = "Casts from `" $t "` to `u16` wrapping at the numeric bounds."]
            #[must_use]
            pub const fn wrapping_cast_to_u16(self) -> u16 {
                [<wrapping_cast_ $t _to_u16>](self.0)
            }
            #[doc = "Casts from `" $t "` to `u32` wrapping at the numeric bounds."]
            #[must_use]
            pub const fn wrapping_cast_to_u32(self) -> u32 {
                [<wrapping_cast_ $t _to_u32>](self.0)
            }
            #[doc = "Casts from `" $t "` to `u64` wrapping at the numeric bounds."]
            #[must_use]
            pub const fn wrapping_cast_to_u64(self) -> u64 {
                [<wrapping_cast_ $t _to_u64>](self.0)
            }
            #[doc = "Casts from `" $t "` to `u128` wrapping at the numeric bounds."]
            #[must_use]
            pub const fn wrapping_cast_to_u128(self) -> u128 {
                [<wrapping_cast_ $t _to_u128>](self.0)
            }
            #[doc = "Casts from `" $t "` to `usize` wrapping at the numeric bounds."]
            #[must_use]
            pub const fn wrapping_cast_to_usize(self) -> usize {
                [<wrapping_cast_ $t _to_usize>](self.0)
            }
            #[doc = "Casts from `" $t "` to `usize_up` wrapping at the numeric bounds."]
            #[must_use]
            pub const fn wrapping_cast_to_usize_up(self) -> usize_up {
                [<wrapping_cast_ $t _to_usize_up>](self.0)
            }
            #[doc = "Casts from `" $t "` to `usize_down` wrapping at the numeric bounds."]
            #[must_use]
            pub const fn wrapping_cast_to_usize_down(self) -> usize_down {
                [<wrapping_cast_ $t _to_usize_down>](self.0)
            }

            #[doc = "Casts from `" $t "` to `i8` wrapping at the numeric bounds."]
            #[must_use]
            pub const fn wrapping_cast_to_i8(self) -> i8 {
                [<wrapping_cast_ $t _to_i8>](self.0)
            }
            #[doc = "Casts from `" $t "` to `i16` wrapping at the numeric bounds."]
            #[must_use]
            pub const fn wrapping_cast_to_i16(self) -> i16 {
                [<wrapping_cast_ $t _to_i16>](self.0)
            }
            #[doc = "Casts from `" $t "` to `i32` wrapping at the numeric bounds."]
            #[must_use]
            pub const fn wrapping_cast_to_i32(self) -> i32 {
                [<wrapping_cast_ $t _to_i32>](self.0)
            }
            #[doc = "Casts from `" $t "` to `i64` wrapping at the numeric bounds."]
            #[must_use]
            pub const fn wrapping_cast_to_i64(self) -> i64 {
                [<wrapping_cast_ $t _to_i64>](self.0)
            }
            #[doc = "Casts from `" $t "` to `i128` wrapping at the numeric bounds."]
            #[must_use]
            pub const fn wrapping_cast_to_i128(self) -> i128 {
                [<wrapping_cast_ $t _to_i128>](self.0)
            }
            #[doc = "Casts from `" $t "` to `isize` wrapping at the numeric bounds."]
            #[must_use]
            pub const fn wrapping_cast_to_isize(self) -> isize {
                [<wrapping_cast_ $t _to_isize>](self.0)
            }
            #[doc = "Casts from `" $t "` to `isize_up` wrapping at the numeric bounds."]
            #[must_use]
            pub const fn wrapping_cast_to_isize_up(self) -> isize_up {
                [<wrapping_cast_ $t _to_isize_up>](self.0)
            }
            #[doc = "Casts from `" $t "` to `isize_down` wrapping at the numeric bounds."]
            #[must_use]
            pub const fn wrapping_cast_to_isize_down(self) -> isize_down {
                [<wrapping_cast_ $t _to_isize_down>](self.0)
            }
        }
    }};
}
impl_cast_methods![u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];

/// Implements private, standalone casting functions between integer primitives
///
/// `$f`:   the type to cast from
/// `$fun`: the type to cast from (unsigned version, used for can_overunderflow)
/// `$t`:   the type to cast to
macro_rules! impl_cast_fns {
    () => {
        /* independent of pointer width */

        impl_cast_fns![can_overunderflow
            // from bigger signed to unsigned
            i16|u16:u8, i32|u32:u8, i64|u64:u8, i128|u128:u8,
            i32|u32:u16, i64|u64:u16, i128|u128:u16,
            i64|u64:u32, i128|u128:u32,
            i128|u128:u64,
            // from bigger signed to signed
            i16|u16:i8, i32|u32:i8, i64|u64:i8, i128|u128:i8,
            i32|u32:i16, i64|u64:i16, i128|u128:i16,
            i64|u64:i32, i128|u128:i32,
            i128|u128:i64
        ];
        impl_cast_fns![can_overflow
            // from bigger unsigned to unsigned
            u16:u8, u32:u8, u64:u8, u128:u8,
            u32:u16, u64:u16, u128:u16,
            u64:u32, u128:u32,
            u128:u64,
            // from bigger unsigned to signed
            u16:i8, u32:i8, u64:i8, u128:i8,
            u32:i16, u64:i16, u128:i16,
            u64:i32, u128:i32,
            u128:i64,
            // from equalsized unsigned to signed
            u8:i8, u16:i16, u32:i32, u64:i64, u128:i128, usize:isize
        ];
        impl_cast_fns![can_underflow
            // from smaller signed to unsigned
            i8:u16, i8:u32, i8:u64, i8:u128,
            i16:u32, i16:u64, i16:u128,
            i32:u64, i32:u128,
            i64:u128,
            // from equalsized signed to unsigned
            i8:u8, i16:u16, i32:u32, i64:u64, i128:u128, isize:usize
        ];
        impl_cast_fns![cant_fail
            // from smaller unsigned to unsigned
            u8:u16, u8:u32, u8:u64, u8:u128,
            u16:u32, u16:u64, u16:u128,
            u32:u64, u32:u128,
            u64:u128,
            // from smaller unsigned to signed
            u8:i16, u8:i32, u8:i64, u8:i128,
            u16:i32, u16:i64, u16:i128,
            u32:i64, u32:i128,
            u64:i128,
            // from smaller signed to signed
            i8:i16, i8:i32, i8:i64, i8:i128,
            i16:i32, i16:i64, i16:i128,
            i32:i64, i32:i128,
            i64:i128,
            // from equalsized unsigned to unsigned
            u8:u8, u16:u16, u32:u32, u64:u64, u128:u128, usize:usize,
            // from equalsized signed to signed
            i8:i8, i16:i16, i32:i32, i64:i64, i128:i128, isize:isize
        ];

        /* dependent on pointer width */

        #[cfg(target_pointer_width = "16")]
        impl_cast_fns![can_overunderflow
            // from bigger signed to unsigned
            isize|usize:u8,
            i32|u32:usize, i64|u64:usize, i128|u128:usize,
            // from bigger signed to signed
            isize|usize:i8,
            i32|u32:isize, i64|u64:isize, i128|u128:isize
        ];
        #[cfg(target_pointer_width = "32")]
        impl_cast_fns![can_overunderflow
            // from bigger signed to unsigned
            isize|usize:u8, isize|usize:u16,
            i64|u64:usize, i128|u128:usize,
            // from bigger signed to signed
            isize|usize:i8, isize|usize:i16,
            i64|u64:isize, i128|u128:isize
        ];
        #[cfg(target_pointer_width = "64")]
        impl_cast_fns![can_overunderflow
            // from bigger signed to unsigned
            isize|usize:u8, isize|usize:u16, isize|usize:u32,
            i128|u128:usize,
            // from bigger signed to signed
            isize|usize:i8, isize|usize:i16, isize|usize:i32,
            i128|u128:isize
        ];

        #[cfg(target_pointer_width = "16")]
        impl_cast_fns![can_overflow
            // from bigger unsigned to unsigned
            usize:u8,
            u32:usize, u64:usize, u128:usize,
            // from bigger unsigned to signed
            usize:i8,
            u32:isize, u64:isize, u128:isize,
            // from equalsized unsigned to signed
            u16:isize, usize:i16
        ];
        #[cfg(target_pointer_width = "32")]
        impl_cast_fns![can_overflow
            // from bigger unsigned to unsigned
            usize:u8, usize:u16,
            u64:usize, u128:usize,
            // from bigger unsigned to signed
            usize:i8, usize:i16,
            u64:isize, u128:isize,
            // from equalsized unsigned to signed
            u32:isize, usize:i32
        ];
        #[cfg(target_pointer_width = "64")]
        impl_cast_fns![can_overflow
            // from bigger unsigned to unsigned
            usize:u8, usize:u16, usize:u32,
            u128:usize,
            // from bigger unsigned to signed
            usize:i8, usize:i16, usize:i32,
            u128:isize,
            // from equalsized unsigned to signed
            u64:isize, usize:i64
        ];

        #[cfg(target_pointer_width = "16")]
        impl_cast_fns![can_underflow
            // from smaller signed to unsigned
            isize:u32, isize:u64, isize:u128,
            i8:usize,
            // from equalsized signed to unsigned
            i16:usize, isize:u16
        ];
        #[cfg(target_pointer_width = "32")]
        impl_cast_fns![can_underflow
            // from smaller signed to unsigned
            isize:u64, isize:u128,
            i8:usize, i16:usize,
            // from equalsized signed to unsigned
            i32:usize, isize:u32
        ];
        #[cfg(target_pointer_width = "64")]
        impl_cast_fns![can_underflow
            // from smaller signed to unsigned
            isize:u128,
            i8:usize, i16:usize, i32:usize,
            // from equalsized signed to unsigned
            i64:usize, isize:u64
        ];

        #[cfg(target_pointer_width = "16")]
        impl_cast_fns![cant_fail ptr:16
            // from smaller unsigned to unsigned
            usize:u32, usize:u64, usize:u128,
            u8:usize,
            // from smaller unsigned to signed
            usize:i32, usize:i64, usize:i128,
            u8:isize,
            // from smaller signed to signed
            isize:i32, isize:i64, isize:i128,
            i8:isize,
            // from equalsized unsigned to unsigned
            usize:u16, u16:usize,
            // from equalsized signed to signed
            isize:i16, i16:isize
        ];
        #[cfg(target_pointer_width = "32")]
        impl_cast_fns![cant_fail ptr:32
            // from smaller unsigned to unsigned
            usize:u64, usize:u128,
            u8:usize, u16:usize,
            // from smaller unsigned to signed
            usize:i64, usize:i128,
            u8:isize, u16:isize,
            // from smaller signed to signed
            isize:i64, isize:i128,
            i8:isize, i16:isize,
            // from equalsized unsigned to unsigned
            usize:u32, u32:usize,
            // from equalsized signed to signed
            isize:i32, i32:isize
        ];
        #[cfg(target_pointer_width = "64")]
        impl_cast_fns![cant_fail ptr:64
            // from smaller unsigned to unsigned
            usize:u128,
            u8:usize, u16:usize, u32:usize,
            // from smaller unsigned to signed
            usize:i128,
            u8:isize, u16:isize, u32:isize,
            // from smaller signed to signed
            isize:i128,
            i8:isize, i16:isize, i32:isize,
            // from equalsized unsigned to unsigned
            usize:u64, u64:usize,
            // from equalsized signed to signed
            isize:i64, i64:isize
        ];
    };
    (can_overunderflow $( $f:ty | $fun:ty : $t:ty ),+) => {
        $( impl_cast_fns![@can_overunderflow $f|$fun:$t]; )+
    };
    (@can_overunderflow $f:ty | $fun:ty : $t:ty) => { paste! {
        const fn [<checked_cast_ $f _to_ $t>](p: $f) -> Result<$t> {
            if p < <$t>::MIN as $f {
                Err(Overflow(Some(Negative)))
            } else if p > $t::MAX as $f {
                Err(Overflow(Some(Positive)))
            } else {
                Ok(p as $t)
            }
        }
        const fn [<saturating_cast_ $f _to_ $t>](p: $f) -> $t {
            if p < <$t>::MIN as $f {
                <$t>::MIN
            } else if p > $t::MAX as $f {
                <$t>::MAX
            } else {
                p as $t
            }
        }
        const fn [<wrapping_cast_ $f _to_ $t>](p: $f) -> $t {
            (p as $fun % (<$t>::MAX as $fun + 1)) as $t
        }
    }};
    (can_overflow $( $f:ty:$t:ty ),+) => { $( impl_cast_fns![@can_overflow $f:$t]; )+ };
    (@can_overflow $f:ty:$t:ty) => { paste! {
        const fn [<checked_cast_ $f _to_ $t>](p: $f) -> Result<$t> {
            is![p > <$t>::MAX as $f; Err(Overflow(Some(Positive))); Ok(p as $t)]
        }
        const fn [<saturating_cast_ $f _to_ $t>](p: $f) -> $t {
            is![p > <$t>::MAX as $f; <$t>::MAX; p as $t]
        }
        const fn [<wrapping_cast_ $f _to_ $t>](p: $f) -> $t {
            (p % (<$t>::MAX as $f + 1)) as $t
        }
    }};
    (can_underflow $( $f:ty:$t:ty ),+) => { $( impl_cast_fns![@can_underflow $f:$t]; )+ };
    (@can_underflow $f:ty:$t:ty) => { paste! {
        const fn [<checked_cast_ $f _to_ $t>](p: $f) -> Result<$t> {
            is![p < 0; Err(Overflow(Some(Negative))); Ok(p as $t)]
        }
        const fn [<saturating_cast_ $f _to_ $t>](p: $f) -> $t {
            is![p < 0; 0; p as $t]
        }
        const fn [<wrapping_cast_ $f _to_ $t>](p: $f) -> $t {
            p as $t
        }
    }};
    (cant_fail $( $f:ty:$t:ty ),+) => { $( impl_cast_fns![@cant_fail $f:$t]; )+ };
    (@cant_fail $f:ty:$t:ty) => { paste! {
        const fn [<checked_cast_ $f _to_ $t>](p: $f) -> Result<$t> {
            Ok(p as $t)
        }
        const fn [<saturating_cast_ $f _to_ $t>](p: $f) -> $t {
            p as $t
        }
        const fn [<wrapping_cast_ $f _to_ $t>](p: $f) -> $t {
            p as $t
        }
    }};
    (cant_fail ptr:$ptr:literal $( $f:ty:$t:ty ),+) => {
        $( impl_cast_fns![@cant_fail ptr:$ptr $f:$t]; )+
    };
    (@cant_fail ptr:$ptr:literal $f:ty:$t:ty) => { paste! {
        const fn [<checked_cast_ $f _to_ $t>](p: $f) -> Result<$t> {
            Ok(p as $t)
        }
        const fn [<saturating_cast_ $f _to_ $t>](p: $f) -> $t {
            p as $t
        }
        const fn [<wrapping_cast_ $f _to_ $t>](p: $f) -> $t {
            p as $t
        }
    }};
}
impl_cast_fns![];

/// implement the casting functions for the upcasted aliases
///
/// `$f`: the type to cast from
/// `$a`: the type to cast to (name alias)
/// `$t`: the type to cast to (real type)
macro_rules! impl_cast_fns_alias {
    () => {
        #[cfg(target_pointer_width = "16")] impl_cast_fns_alias![@to isize_up|i32];
        #[cfg(target_pointer_width = "16")] impl_cast_fns_alias![@to usize_up|u32];
        #[cfg(target_pointer_width = "16")] impl_cast_fns_alias![@to isize_down|i8];
        #[cfg(target_pointer_width = "16")] impl_cast_fns_alias![@to usize_down|u8];
        //
        #[cfg(target_pointer_width = "32")] impl_cast_fns_alias![@to isize_up|i64];
        #[cfg(target_pointer_width = "32")] impl_cast_fns_alias![@to usize_up|u64];
        #[cfg(target_pointer_width = "32")] impl_cast_fns_alias![@to isize_down|i16];
        #[cfg(target_pointer_width = "32")] impl_cast_fns_alias![@to usize_down|u16];
        //
        #[cfg(target_pointer_width = "64")] impl_cast_fns_alias![@to isize_up|i128];
        #[cfg(target_pointer_width = "64")] impl_cast_fns_alias![@to usize_up|u128];
        #[cfg(target_pointer_width = "64")] impl_cast_fns_alias![@to isize_down|i32];
        #[cfg(target_pointer_width = "64")] impl_cast_fns_alias![@to usize_down|u32];
    };
    (@to $a:ident|$t:ty) => {
        impl_cast_fns_alias![@to $a|$t:
            i8, u8, i16, u16, i32, u32, i64, u64, i128, u128, isize, usize];
    };
    (@to $a:ident|$t:ty : $($f:ty),+) => { $( impl_cast_fns_alias!(@impl $f, $a, $t);)+ };
    (@impl $f:ty, $a:ty, $t:ty) => { paste! {
        const fn [<checked_cast_ $f _to_ $a>](p: $f) -> Result<$a> {
            [<checked_cast_ $f _to_ $t>](p)
        }
        const fn [<saturating_cast_ $f _to_ $a>](p: $f) -> $a {
            [<saturating_cast_ $f _to_ $t>](p)
        }
        const fn [<wrapping_cast_ $f _to_ $a>](p: $f) -> $a {
            [<wrapping_cast_ $f _to_ $t>](p)
        }
    }};
}
impl_cast_fns_alias![];
