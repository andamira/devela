// devela::data::conversion::primitive::bits
//
//! fns for extended bit manipulation
//
// TOC
// - trait definition
// - trait implementation
// - functions definitions

use crate::{
    data::{DataErrors as E, DataResult as Result},
    meta::{iif, paste},
};

/// Offers methods for bitwise operations on primitives.
pub trait BitsPrimitives
where
    Self: Sized,
{
    /* new mask */

    /// Returns a new bitmask of ones from the `start` to the `end` bit inclusive.
    #[doc = include_str!("./Benchmarks_bits_new_mask_checked.md")]
    fn bits_new_mask_range(start: u32, end: u32) -> Result<Self>;
    /// Returns a new bitmask of ones from the `start` to the `end` bit inclusive,
    /// unchecked version.
    #[doc = include_str!("./Benchmarks_bits_new_mask_unchecked.md")]
    #[must_use]
    fn bits_new_mask_range_unchecked(start: u32, end: u32) -> Self;

    /* get */

    /// Gets the bits in `self`, from `start` to `end` inclusive.
    fn bits_get_range(self, start: u32, end: u32) -> Result<Self>;
    /// Gets the bits in `self`, from `start` to `end` inclusive, unchecked version.
    #[must_use]
    fn bits_get_range_unchecked(self, start: u32, end: u32) -> Self;

    /// Gets the shifted bits in `self`, from `start` to `end` inclusive.
    fn bits_get_shift_range(self, start: u32, end: u32) -> Result<Self>;
    /// Sets the shifted bits in `self`, from `start` to `end` inclusive, unchecked version.
    #[must_use]
    fn bits_get_shift_range_unchecked(self, start: u32, end: u32) -> Self;

    /* set */

    /// Sets the bits in `self` to 1, from `start` to `end` inclusive.
    fn bits_set_range(self, start: u32, end: u32) -> Result<Self>;
    /// Sets the bits in `self` to 1, from `start` to `end` inclusive, unchecked version.
    #[must_use]
    fn bits_set_range_unchecked(self, start: u32, end: u32) -> Self;

    /* unset */

    /// Unsets the bits in `self` to 0, from `start` to `end` inclusive.
    fn bits_unset_range(self, start: u32, end: u32) -> Result<Self>;
    /// Unsets the bits in `self` to 0, from `start` to `end` inclusive, unchecked version.
    #[must_use]
    fn bits_unset_range_unchecked(self, start: u32, end: u32) -> Self;

    /* flip */

    /// Flips the bits in `self`, from `start` to `end` inclusive.
    fn bits_flip_range(self, start: u32, end: u32) -> Result<Self>;
    /// Flips the bits in `self` from `start` to `end` inclusive, unchecked version.
    #[must_use]
    fn bits_flip_range_unchecked(self, start: u32, end: u32) -> Self;
}

macro_rules! impl_bits_trait {
    ($($t:ty),+) => { $( impl_bits_trait![@$t]; )+ };
    (@$t:ty) => { paste! {
        impl BitsPrimitives for $t {
            fn bits_new_mask_range(start: u32, end: u32) -> Result<Self> {
                [<bits_new_mask_range_ $t>](start, end)
            }
            fn bits_new_mask_range_unchecked(start: u32, end: u32) -> Self {
                [<bits_new_mask_range_unchecked_ $t>](start, end)
            }
            fn bits_get_range(self, start: u32, end: u32) -> Result<Self> {
                [<bits_get_range_ $t>](self, start, end)
            }
            fn bits_get_range_unchecked(self, start: u32, end: u32) -> Self {
                [<bits_get_range_unchecked_ $t>](self, start, end)
            }
            fn bits_get_shift_range(self, start: u32, end: u32) -> Result<Self> {
                [<bits_get_shift_range_ $t>](self, start, end)
            }
            fn bits_get_shift_range_unchecked(self, start: u32, end: u32) -> Self {
                [<bits_get_shift_range_unchecked_ $t>](self, start, end)
            }
            fn bits_set_range(self, start: u32, end: u32) -> Result<Self> {
                [<bits_set_range_ $t>](self, start, end)
            }
            fn bits_set_range_unchecked(self, start: u32, end: u32) -> Self {
                [<bits_set_range_unchecked_ $t>](self, start, end)
            }
            fn bits_unset_range(self, start: u32, end: u32) -> Result<Self> {
                [<bits_unset_range_ $t>](self, start, end)
            }
            fn bits_unset_range_unchecked(self, start: u32, end: u32) -> Self {
                [<bits_unset_range_unchecked_ $t>](self, start, end)
            }
            fn bits_flip_range(self, start: u32, end: u32) -> Result<Self> {
                [<bits_flip_range_ $t>](self, start, end)
            }
            fn bits_flip_range_unchecked(self, start: u32, end: u32) -> Self {
                [<bits_flip_range_unchecked_ $t>](self, start, end)
            }
        }
    }};
}
impl_bits_trait![u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];

/* functions */

// Implements bit operations for primitives
//
// `$t`: the type
macro_rules! impl_bits_fns {
    () => {
        /* independent of pointer width */
        impl_bits_fns![
            i8, i16, i32, i64, i128, isize,
            u8, u16, u32, u64, u128, usize
        ];
    };
    ( $( $t:ty ),+ ) => { $( impl_bits_fns![@$t]; )+ };
    (@$t:ty) => { paste! {
        /// Returns a new bitmask of ones from the `start` to the `end` bit inclusive.
        ///
        /// # Errors
        #[doc = "Returns [`OutOfBounds`][E::OutOfBounds] if `start >= `[`"
            $t "::BITS`]` || end >= `[`" $t "::BITS`] and
            [`MismatchedIndices`][E::MismatchedIndices] if `start > end`."]
        ///
        #[doc = include_str!("./Benchmarks_bits_new_mask_checked.md")]
        #[inline]
        pub const fn [<bits_new_mask_range_ $t>](start: u32, end: u32) -> Result<$t> {
            if start >= <$t>::BITS {
                Err(E::OutOfBounds(Some(start as usize)))
            } else if end >= <$t>::BITS {
                Err(E::OutOfBounds(Some(end as usize)))
            } else if start > end {
                Err(E::MismatchedIndices)
            } else {
                // create a mask with all bits set, from 0 to end:
                let mask_end = iif![end == <$t>::BITS -1; !0; (1 << (end + 1)) - 1];
                // create a mask with all bits set from 0 to start - 1:
                let mask_start = iif![start == 0; 0; (1 << start) - 1];
                Ok(mask_end - mask_start)
            }
        }

        /// Returns a new bitmask of onew from the `start` to the `end` bit inclusive,
        /// unchecked version.
        ///
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[doc = include_str!("./Benchmarks_bits_new_mask_unchecked.md")]
        #[must_use] #[inline]
        pub const fn [<bits_new_mask_range_unchecked_ $t>](start: u32, end: u32) -> $t {
            debug_assert![start <= end];
            // a mask with all bits set, from 0 to end:
            let mask_end = iif![end == <$t>::BITS -1; !0; (1 << (end + 1)) - 1];
            // a mask with all bits set from 0 to start - 1:
            let mask_start = iif![start == 0; 0; (1 << start) - 1];
            mask_end - mask_start

            // naive loop implementation:
            // let mut mask = 0;
            // while start <= end {
            //     mask |= 1 << start;
            //     start += 1;
            // }
            // mask
        }

        /// Gets the bits in `bits` from `start` to `end` inclusive.
        ///
        /// # Errors
        #[doc = "Returns [`OutOfBounds`][E::OutOfBounds] if `start >= `[`"
            $t "::BITS`]` || end >= `[`" $t "::BITS`] and
            [`MismatchedIndices`][E::MismatchedIndices] if `start > end`."]
        #[inline]
        pub const fn [<bits_get_range_ $t>](bits: $t, start: u32, end: u32) -> Result<$t> {
            match [< bits_new_mask_range_ $t>](start, end) {
                Ok(mask) => Ok(bits & mask),
                Err(e) => Err(e),
            }
        }
        /// Gets the bits in `bits` from `start` to `end` inclusive, unchecked version.
        ///
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[must_use] #[inline]
        pub const fn [<bits_get_range_unchecked_ $t>](bits: $t, start: u32, end: u32) -> $t {
            (bits & [< bits_new_mask_range_unchecked_ $t>](start, end))
        }

        /// Gets the shifted bits in `bits` from `start` to `end` inclusive.
        ///
        /// # Errors
        #[doc = "Returns [`OutOfBounds`][E::OutOfBounds] if `start >= `[`"
            $t "::BITS`]` || end >= `[`" $t "::BITS`] and
            [`MismatchedIndices`][E::MismatchedIndices] if `start > end`."]
        #[inline]
        pub const fn [<bits_get_shift_range_ $t>](bits: $t, start: u32, end: u32) -> Result<$t> {
            match [< bits_new_mask_range_ $t>](start, end) {
                Ok(mask) => Ok((bits & mask) >> start),
                Err(e) => Err(e),
            }
        }
        /// Gets the shifted bits in `bits` from `start` to `end` inclusive, unchecked version.
        ///
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[must_use] #[inline]
        pub const fn [<bits_get_shift_range_unchecked_ $t>](bits: $t, start: u32, end: u32) -> $t {
            (bits & [< bits_new_mask_range_unchecked_ $t>](start, end)) >> start
        }

        /// Sets the bits in `bits` to 1, from `start` to `end` inclusive.
        ///
        /// # Errors
        #[doc = "Returns [`OutOfBounds`][E::OutOfBounds] if `start >= `[`"
            $t "::BITS`]` || end >= `[`" $t "::BITS`] and
            [`MismatchedIndices`][E::MismatchedIndices] if `start > end`."]
        #[inline]
        pub const fn [<bits_set_range_ $t>](bits: $t, start: u32, end: u32) -> Result<$t> {
            match [< bits_new_mask_range_ $t>](start, end) {
                Ok(mask) => Ok(bits | mask),
                Err(e) => Err(e),
            }
        }
        /// Sets the bits in `bits` to 1, from `start` to `end` inclusive, unchecked version.
        ///
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[must_use] #[inline]
        pub const fn [<bits_set_range_unchecked_ $t>](bits: $t, start: u32, end: u32) -> $t {
            bits | [< bits_new_mask_range_unchecked_ $t>](start, end)
        }

        /// Unsets the bits in `bits` to 0, from `start` to `end` inclusive.
        ///
        /// # Errors
        #[doc = "Returns [`OutOfBounds`][E::OutOfBounds] if `start >= `[`"
            $t "::BITS`]` || end >= `[`" $t "::BITS`] and
            [`MismatchedIndices`][E::MismatchedIndices] if `start > end`."]
        #[inline]
        pub const fn [<bits_unset_range_ $t>](bits: $t, start: u32, end: u32) -> Result<$t> {
            match [< bits_new_mask_range_ $t>](start, end) {
                Ok(mask) => Ok(bits & !mask),
                Err(e) => Err(e),
            }
        }
        /// Unsets the bits in `bits` to 0, from `start` to `end` inclusive, unchecked version.
        ///
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[must_use] #[inline]
        pub const fn [<bits_unset_range_unchecked_ $t>](bits: $t, start: u32, end: u32) -> $t {
            bits & ![< bits_new_mask_range_unchecked_ $t>](start, end)
        }

        /// Flips the bits in `bits` from `start` to `end` inclusive.
        ///
        /// # Errors
        #[doc = "Returns [`OutOfBounds`][E::OutOfBounds] if `start >= `[`"
            $t "::BITS`]` || end >= `[`" $t "::BITS`] and
            [`MismatchedIndices`][E::MismatchedIndices] if `start > end`."]
        #[inline]
        pub const fn [<bits_flip_range_ $t>](bits: $t, start: u32, end: u32) -> Result<$t> {
            match [< bits_new_mask_range_ $t>](start, end) {
                Ok(mask) => Ok(bits ^ mask),
                Err(e) => Err(e),
            }
        }
        /// Flips the bits in `bits` from `start` to `end` inclusive.
        ///
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[must_use] #[inline]
        pub const fn [<bits_flip_range_unchecked_ $t>](bits: $t, start: u32, end: u32) -> $t {
            bits ^ [< bits_new_mask_range_unchecked_ $t>](start, end)
        }
    }};
}
impl_bits_fns![];
