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

    /// Returns a bitmask of ones from the `start` to the `end` bit inclusive.
    ///
    /// This is the base of the rest of the mask related functionality.
    /// # Panics
    /// Panics in debug if `start >= Self::BITS` || `end >= Self::BITS` || `start > end`.
    #[doc = include_str!("./Benchmarks_bits_mask_range.md")]
    #[must_use]
    fn bits_mask_range(start: u32, end: u32) -> Self;

    /// Returns a bitmask of ones from the `start` to the `end` bit inclusive, checked.
    ///
    /// This is the base of the rest of the mask related functionality.
    /// # Errors
    /// Returns [`OutOfBounds`][E::OutOfBounds] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`][E::MismatchedIndices] if `start > end`.
    #[doc = include_str!("./Benchmarks_bits_mask_checked_range.md")]
    fn bits_mask_checked_range(start: u32, end: u32) -> Result<Self>;

    /* get */

    /// Gets the bits in `self`, from `start` to `end` inclusive.
    /// # Panics
    /// Panics in debug if `start >= Self::BITS` || `end >= Self::BITS` || `start > end`.
    #[must_use]
    fn bits_get_range(self, start: u32, end: u32) -> Self;

    /// Gets the bits in `self`, from `start` to `end` inclusive, checked.
    /// # Errors
    /// Returns [`OutOfBounds`][E::OutOfBounds] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`][E::MismatchedIndices] if `start > end`.
    fn bits_get_checked_range(self, start: u32, end: u32) -> Result<Self>;

    /* get shift */

    /// Gets the rightwards shifted bits in `self`, from `start` to `end` inclusive.
    ///
    /// Extracts the bits from the range specified by `start` and `end` (inclusive),
    /// and shifts them rightwards so that the least significant bit (LSB) of the extracted
    /// segment aligns with the units place. The rest of the bits are filled with zeros.
    /// # Panics
    /// Panics in debug if `start >= Self::BITS` || `end >= Self::BITS` || `start > end`.
    #[must_use]
    fn bits_get_shift_range(self, start: u32, end: u32) -> Self;

    /// Gets the rightwards shifted bits in `self`, from `start` to `end` inclusive, checked.
    ///
    /// Extracts the bits from the range specified by `start` and `end` (inclusive),
    /// and shifts them rightwards so that the least significant bit (LSB) of the extracted
    /// segment aligns with the units place. The rest of the bits are filled with zeros.
    ///
    /// # Errors
    /// Returns [`OutOfBounds`][E::OutOfBounds] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`][E::MismatchedIndices] if `start > end`.
    fn bits_get_shift_checked_range(self, start: u32, end: u32) -> Result<Self>;

    /* set */

    /// Sets the bits in `self` to 1, from `start` to `end` inclusive.
    /// # Panics
    /// Panics in debug if `start >= Self::BITS` || `end >= Self::BITS` || `start > end`.
    #[must_use]
    fn bits_set_range(self, start: u32, end: u32) -> Self;

    /// Sets the bits in `self` to 1, from `start` to `end` inclusive, checked.
    /// # Errors
    /// Returns [`OutOfBounds`][E::OutOfBounds] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`][E::MismatchedIndices] if `start > end`.
    fn bits_set_checked_range(self, start: u32, end: u32) -> Result<Self>;

    /* unset */

    /// Unsets the bits in `self` to 0, from `start` to `end` inclusive.
    /// # Panics
    /// Panics in debug if `start >= Self::BITS` || `end >= Self::BITS` || `start > end`.
    #[must_use]
    fn bits_unset_range(self, start: u32, end: u32) -> Self;

    /// Unsets the bits in `self` to 0, from `start` to `end` inclusive, checked.
    /// # Errors
    /// Returns [`OutOfBounds`][E::OutOfBounds] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`][E::MismatchedIndices] if `start > end`.
    fn bits_unset_checked_range(self, start: u32, end: u32) -> Result<Self>;

    /* flip */

    /// Flips the bits in `self` from `start` to `end` inclusive.
    /// # Panics
    /// Panics in debug if `start >= Self::BITS` || `end >= Self::BITS` || `start > end`.
    #[must_use]
    fn bits_flip_range(self, start: u32, end: u32) -> Self;

    /// Flips the bits in `self`, from `start` to `end` inclusive, checked.
    /// # Errors
    /// Returns [`OutOfBounds`][E::OutOfBounds] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`][E::MismatchedIndices] if `start > end`.
    fn bits_flip_checked_range(self, start: u32, end: u32) -> Result<Self>;

    /* count */

    /// Counts the number of set bits (ones) in `bits` from `start` to `end` inclusive.
    /// # Panics
    /// Panics in debug if `start >= Self::BITS` || `end >= Self::BITS` || `start > end`.
    #[must_use]
    fn bits_count_ones_range(self, start: u32, end: u32) -> u32;

    /// Counts the number of set bits (ones) in `bits` from `start` to `end` inclusive, checked.
    /// # Errors
    /// Returns [`OutOfBounds`][E::OutOfBounds] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`][E::MismatchedIndices] if `start > end`.
    fn bits_count_ones_checked_range(self, start: u32, end: u32) -> Result<u32>;

    /// Counts the number of unset bits (zeros) in `bits` from `start` to `end` inclusive.
    /// # Panics
    /// Panics in debug if `start >= Self::BITS` || `end >= Self::BITS` || `start > end`.
    #[must_use]
    fn bits_count_zeros_range(self, start: u32, end: u32) -> u32;

    /// Counts the number of unset bits (zeros) in `bits` from `start` to `end` inclusive, checked.
    /// # Errors
    /// Returns [`OutOfBounds`][E::OutOfBounds] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`][E::MismatchedIndices] if `start > end`.
    fn bits_count_zeros_checked_range(self, start: u32, end: u32) -> Result<u32>;

    /* find first */

    /// Finds the index of the first set bit (one) in `bits` from `start` to `end` inclusive.
    ///
    /// Returns `None` if there are no bits set.
    ///
    /// The index is relative to the entire sequence of `bits`, not to the given `start`.
    /// You can always substract `start` from the result afterwards for that.
    /// # Panics
    /// Panics in debug if `start >= Self::BITS` || `end >= Self::BITS` || `start > end`.
    #[must_use]
    fn bits_find_first_one_range(self, start: u32, end: u32) -> Option<u32>;

    /// Finds the index of the first set bit (one) in `bits` from `start` to `end` inclusive,
    /// checked.
    ///
    /// Returns `None` if there are no bits set.
    ///
    /// The index is relative to the entire sequence of `bits`, not to the given `start`.
    /// You can always substract `start` from the result afterwards for that.
    /// # Errors
    /// Returns [`OutOfBounds`][E::OutOfBounds] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`][E::MismatchedIndices] if `start > end`.
    fn bits_find_first_one_checked_range(self, start: u32, end: u32) -> Result<Option<u32>>;

    /// Finds the index of the first unset bit (zero) in `bits` from `start` to `end` inclusive.
    ///
    /// Returns `None` if there are no bits unset.
    ///
    /// The index is relative to the entire sequence of `bits`, not to the given `start`.
    /// You can always substract `start` from the result afterwards for that.
    /// # Panics
    /// Panics in debug if `start >= Self::BITS` || `end >= Self::BITS` || `start > end`.
    #[must_use]
    fn bits_find_first_zero_range(self, start: u32, end: u32) -> Option<u32>;

    /// Finds the index of the first unset bit (zero) in `bits` from `start` to `end` inclusive,
    /// checked.
    ///
    /// Returns `None` if there are no bits unset.
    ///
    /// The index is relative to the entire sequence of `bits`, not to the given `start`.
    /// You can always substract `start` from the result afterwards for that.
    /// # Errors
    /// Returns [`OutOfBounds`][E::OutOfBounds] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`][E::MismatchedIndices] if `start > end`.
    fn bits_find_first_zero_checked_range(self, start: u32, end: u32) -> Result<Option<u32>>;
}

macro_rules! impl_bits_trait {
    ($($t:ty),+) => { $( impl_bits_trait![@$t]; )+ };
    (@$t:ty) => { paste! {
        impl BitsPrimitives for $t {
            // new mask
            fn bits_mask_range(start: u32, end: u32) -> Self {
                [<bits_mask_range_ $t>](start, end)
            }
            fn bits_mask_checked_range(start: u32, end: u32) -> Result<Self> {
                [<bits_mask_checked_range_ $t>](start, end)
            }
            // get
            fn bits_get_range(self, start: u32, end: u32) -> Self {
                [<bits_get_range_ $t>](self, start, end)
            }
            fn bits_get_checked_range(self, start: u32, end: u32) -> Result<Self> {
                [<bits_get_checked_range_ $t>](self, start, end)
            }
            // get shift
            fn bits_get_shift_range(self, start: u32, end: u32) -> Self {
                [<bits_get_shift_range_ $t>](self, start, end)
            }
            fn bits_get_shift_checked_range(self, start: u32, end: u32) -> Result<Self> {
                [<bits_get_shift_checked_range_ $t>](self, start, end)
            }
            // set
            fn bits_set_range(self, start: u32, end: u32) -> Self {
                [<bits_set_range_ $t>](self, start, end)
            }
            fn bits_set_checked_range(self, start: u32, end: u32) -> Result<Self> {
                [<bits_set_checked_range_ $t>](self, start, end)
            }
            // unset
            fn bits_unset_range(self, start: u32, end: u32) -> Self {
                [<bits_unset_range_ $t>](self, start, end)
            }
            fn bits_unset_checked_range(self, start: u32, end: u32) -> Result<Self> {
                [<bits_unset_checked_range_ $t>](self, start, end)
            }
            // flip
            fn bits_flip_range(self, start: u32, end: u32) -> Self {
                [<bits_flip_range_ $t>](self, start, end)
            }
            fn bits_flip_checked_range(self, start: u32, end: u32) -> Result<Self> {
                [<bits_flip_checked_range_ $t>](self, start, end)
            }
            // count
            fn bits_count_ones_range(self, start: u32, end: u32) -> u32 {
                [<bits_count_ones_range_ $t>](self, start, end)
            }
            fn bits_count_ones_checked_range(self, start: u32, end: u32) -> Result<u32> {
                [<bits_count_ones_checked_range_ $t>](self, start, end)
            }
            fn bits_count_zeros_range(self, start: u32, end: u32) -> u32 {
                [<bits_count_zeros_range_ $t>](self, start, end)
            }
            fn bits_count_zeros_checked_range(self, start: u32, end: u32) -> Result<u32> {
                [<bits_count_zeros_checked_range_ $t>](self, start, end)
            }
            // find first
            fn bits_find_first_one_range(self, start: u32, end: u32) -> Option<u32> {
                [<bits_find_first_one_range_ $t>](self, start, end)
            }
            fn bits_find_first_one_checked_range(self, start: u32, end: u32) -> Result<Option<u32>> {
                [<bits_find_first_one_checked_range_ $t>](self, start, end)
            }
            fn bits_find_first_zero_range(self, start: u32, end: u32) -> Option<u32> {
                [<bits_find_first_zero_range_ $t>](self, start, end)
            }
            fn bits_find_first_zero_checked_range(self, start: u32, end: u32) -> Result<Option<u32>> {
                [<bits_find_first_zero_checked_range_ $t>](self, start, end)
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
        /* new mask */

        /// Returns a new bitmask of ones from the `start` to the `end` bit inclusive.
        ///
        /// This is the base of the rest of the mask related functionality.
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[doc = include_str!("./Benchmarks_bits_mask_range.md")]
        #[must_use] #[inline]
        pub const fn [<bits_mask_range_ $t>](start: u32, end: u32) -> $t {
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
        /// Returns a new bitmask of ones from the `start` to the `end` bit inclusive, checked.
        ///
        /// This is the base of the rest of the mask related functionality.
        /// # Errors
        #[doc = "Returns [`OutOfBounds`][E::OutOfBounds] if `start >= `[`"
            $t "::BITS`]` || end >= `[`" $t "::BITS`] and
            [`MismatchedIndices`][E::MismatchedIndices] if `start > end`."]
        #[doc = include_str!("./Benchmarks_bits_mask_checked_range.md")]
        #[inline]
        pub const fn [<bits_mask_checked_range_ $t>](start: u32, end: u32) -> Result<$t> {
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

        /* get */

        /// Gets the bits in `bits` from `start` to `end` inclusive.
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[must_use] #[inline]
        pub const fn [<bits_get_range_ $t>](bits: $t, start: u32, end: u32) -> $t {
            (bits & [< bits_mask_range_ $t>](start, end))
        }

        /// Gets the bits in `bits` from `start` to `end` inclusive, checked.
        /// # Errors
        #[doc = "Returns [`OutOfBounds`][E::OutOfBounds] if `start >= `[`"
            $t "::BITS`]` || end >= `[`" $t "::BITS`] and
            [`MismatchedIndices`][E::MismatchedIndices] if `start > end`."]
        #[inline]
        pub const fn [<bits_get_checked_range_ $t>](bits: $t, start: u32, end: u32) -> Result<$t> {
            match [< bits_mask_checked_range_ $t>](start, end) {
                Ok(mask) => Ok(bits & mask),
                Err(e) => Err(e),
            }
        }

        /* get shift */

        /// Gets the shifted bits in `bits` from `start` to `end` inclusive.
        ///
        /// Extracts the bits from the range specified by `start` and `end` (inclusive),
        /// and shifts them rightwards so that the least significant bit (LSB) of the extracted
        /// segment aligns with the units place. The rest of the bits are filled with zeros.
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[must_use] #[inline]
        pub const fn [<bits_get_shift_range_ $t>](bits: $t, start: u32, end: u32) -> $t {
            (bits & [< bits_mask_range_ $t>](start, end)) >> start
        }

        /// Gets the shifted bits in `bits` from `start` to `end` inclusive, checked.
        ///
        /// Extracts the bits from the range specified by `start` and `end` (inclusive),
        /// and shifts them rightwards so that the least significant bit (LSB) of the extracted
        /// segment aligns with the units place. The rest of the bits are filled with zeros.
        /// # Errors
        #[doc = "Returns [`OutOfBounds`][E::OutOfBounds] if `start >= `[`"
            $t "::BITS`]` || end >= `[`" $t "::BITS`] and
            [`MismatchedIndices`][E::MismatchedIndices] if `start > end`."]
        #[inline]
        pub const fn [<bits_get_shift_checked_range_ $t>](bits: $t, start: u32, end: u32) -> Result<$t> {
            match [< bits_mask_checked_range_ $t>](start, end) {
                Ok(mask) => Ok((bits & mask) >> start),
                Err(e) => Err(e),
            }
        }

        /* set */

        /// Sets the bits in `bits` to 1, from `start` to `end` inclusive.
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[must_use] #[inline]
        pub const fn [<bits_set_range_ $t>](bits: $t, start: u32, end: u32) -> $t {
            bits | [< bits_mask_range_ $t>](start, end)
        }

        /// Sets the bits in `bits` to 1, from `start` to `end` inclusive, checked.
        /// # Errors
        #[doc = "Returns [`OutOfBounds`][E::OutOfBounds] if `start >= `[`"
            $t "::BITS`]` || end >= `[`" $t "::BITS`] and
            [`MismatchedIndices`][E::MismatchedIndices] if `start > end`."]
        #[inline]
        pub const fn [<bits_set_checked_range_ $t>](bits: $t, start: u32, end: u32) -> Result<$t> {
            match [< bits_mask_checked_range_ $t>](start, end) {
                Ok(mask) => Ok(bits | mask),
                Err(e) => Err(e),
            }
        }

        /* unset */

        /// Unsets the bits in `bits` to 0, from `start` to `end` inclusive.
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[must_use] #[inline]
        pub const fn [<bits_unset_range_ $t>](bits: $t, start: u32, end: u32) -> $t {
            bits & ![< bits_mask_range_ $t>](start, end)
        }

        /// Unsets the bits in `bits` to 0, from `start` to `end` inclusive, checked.
        /// # Errors
        #[doc = "Returns [`OutOfBounds`][E::OutOfBounds] if `start >= `[`"
            $t "::BITS`]` || end >= `[`" $t "::BITS`] and
            [`MismatchedIndices`][E::MismatchedIndices] if `start > end`."]
        #[inline]
        pub const fn [<bits_unset_checked_range_ $t>](bits: $t, start: u32, end: u32) -> Result<$t> {
            match [< bits_mask_checked_range_ $t>](start, end) {
                Ok(mask) => Ok(bits & !mask),
                Err(e) => Err(e),
            }
        }

        /* flip */

        /// Flips the bits in `bits` from `start` to `end` inclusive.
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[must_use] #[inline]
        pub const fn [<bits_flip_range_ $t>](bits: $t, start: u32, end: u32) -> $t {
            bits ^ [< bits_mask_range_ $t>](start, end)
        }

        /// Flips the bits in `bits` from `start` to `end` inclusive, checked.
        /// # Errors
        #[doc = "Returns [`OutOfBounds`][E::OutOfBounds] if `start >= `[`"
            $t "::BITS`]` || end >= `[`" $t "::BITS`] and
            [`MismatchedIndices`][E::MismatchedIndices] if `start > end`."]
        #[inline]
        pub const fn [<bits_flip_checked_range_ $t>](bits: $t, start: u32, end: u32) -> Result<$t> {
            match [< bits_mask_checked_range_ $t>](start, end) {
                Ok(mask) => Ok(bits ^ mask),
                Err(e) => Err(e),
            }
        }

        /* count */

        /// Counts the number of set bits (ones) in `bits` from `start` to `end` inclusive.
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[must_use] #[inline]
        pub const fn [<bits_count_ones_range_ $t>](bits: $t, start: u32, end: u32) -> u32 {
            let masked_bits = bits & [<bits_mask_range_ $t>](start, end);
            masked_bits.count_ones()
        }
        /// Counts the number of set bits (ones) in `bits` from `start` to `end` inclusive, checked.
        /// # Errors
        #[doc = "Returns [`OutOfBounds`][E::OutOfBounds] if `start >= `[`"
            $t "::BITS`]` || end >= `[`" $t "::BITS`] and
            [`MismatchedIndices`][E::MismatchedIndices] if `start > end`."]
        #[inline]
        pub const fn [<bits_count_ones_checked_range_ $t>](bits: $t, start: u32, end: u32) -> Result<u32> {
            match [<bits_mask_checked_range_ $t>](start, end) {
                Ok(mask) => Ok((bits & mask).count_ones()),
                Err(e) => Err(e),
            }
        }

        /// Counts the number of unset bits (zeros) in `bits` from `start` to `end` inclusive.
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[must_use] #[inline]
        pub const fn [<bits_count_zeros_range_ $t>](bits: $t, start: u32, end: u32) -> u32 {
            let masked_bits = bits & [<bits_mask_range_ $t>](start, end);
            masked_bits.count_zeros()
        }

        /// Counts the number of unset bits (zeros) in `bits` from `start` to `end` inclusive, checked.
        /// # Errors
        #[doc = "Returns [`OutOfBounds`][E::OutOfBounds] if `start >= `[`"
            $t "::BITS`]` || end >= `[`" $t "::BITS`] and
            [`MismatchedIndices`][E::MismatchedIndices] if `start > end`."]
        #[inline]
        pub const fn [<bits_count_zeros_checked_range_ $t>](bits: $t, start: u32, end: u32) -> Result<u32> {
            match [<bits_mask_checked_range_ $t>](start, end) {
                Ok(mask) => Ok((bits & mask).count_zeros()),
                Err(e) => Err(e),
            }
        }

        /* find first */

        /// Finds the index of the first set bit (one) in `bits` from `start` to `end` inclusive.
        ///
        /// Returns `None` if there are no bits set.
        ///
        /// The index is relative to the entire sequence of `bits`, not to the given `start`.
        /// You can always substract `start` from the result afterwards for that.
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[must_use] #[inline]
        pub const fn [<bits_find_first_one_range_ $t>](bits: $t, start: u32, end: u32)
            -> Option<u32> {
            let masked_bits = bits & [<bits_mask_range_ $t>](start, end);
            let mut idx = start;
            while idx <= end {
                iif![(masked_bits & (1 << idx)) != 0; return Some(idx)];
                idx += 1;
            }
            None
        }

        /// Finds the index of the first set bit (one) in `bits` from `start` to `end` inclusive,
        /// checked.
        ///
        /// Returns `None` if there are no bits set.
        ///
        /// The index is relative to the entire sequence of `bits`, not to the given `start`.
        /// You can always substract `start` from the result afterwards for that.
        /// # Errors
        #[doc = "Returns [`OutOfBounds`][E::OutOfBounds] if `start >= `[`"
            $t "::BITS`]` || end >= `[`" $t "::BITS`] and
            [`MismatchedIndices`][E::MismatchedIndices] if `start > end`."]
        #[inline]
        pub const fn [<bits_find_first_one_checked_range_ $t>](bits: $t, start: u32, end: u32)
            -> Result<Option<u32>> {
            match [<bits_mask_checked_range_ $t>](start, end) {
                Ok(mask) => {
                    let masked_bits = bits & mask;
                    let mut idx = start;
                    while idx <= end {
                        iif![(masked_bits & (1 << idx)) != 0; return Ok(Some(idx))];
                        idx += 1;
                    }
                    Ok(None)
                },
                Err(e) => Err(e),
            }
        }

        /// Finds the index of the first unset bit (zero) in `bits` from `start` to `end` inclusive.
        ///
        /// Returns `None` if there are no bits unset.
        ///
        /// The index is relative to the entire sequence of `bits`, not to the given `start`.
        /// You can always substract `start` from the result afterwards for that.
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[must_use] #[inline]
        pub const fn [<bits_find_first_zero_range_ $t>](bits: $t, start: u32, end: u32)
            -> Option<u32> {
            let masked_bits = !(bits & [<bits_mask_range_ $t>](start, end));
            let mut idx = start;
            while idx <= end {
                iif![(masked_bits & (1 << idx)) != 0; return Some(idx)];
                idx += 1;
            }
            None
        }

        /// Finds the index of the first unset bit (zero) in `bits` from `start` to `end` inclusive,
        /// checked.
        ///
        /// Returns `None` if there are no bits unset.
        ///
        /// The index is relative to the entire sequence of `bits`, not to the given `start`.
        /// You can always substract `start` from the result afterwards for that.
        /// # Errors
        #[doc = "Returns [`OutOfBounds`][E::OutOfBounds] if `start >= `[`"
            $t "::BITS`]` || end >= `[`" $t "::BITS`] and
            [`MismatchedIndices`][E::MismatchedIndices] if `start > end`."]
        #[inline]
        pub const fn [<bits_find_first_zero_checked_range_ $t>](bits: $t, start: u32, end: u32)
            -> Result<Option<u32>> {
            match [<bits_mask_checked_range_ $t>](start, end) {
                Ok(mask) => {
                    let masked_bits = !(bits & mask);
                    let mut idx = start;
                    while idx <= end {
                        iif![(masked_bits & (1 << idx)) != 0; return Ok(Some(idx))];
                        idx += 1;
                    }
                    Ok(None)
                },
                Err(e) => Err(e),
            }
        }
    }};
}
impl_bits_fns![];
