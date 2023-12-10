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
pub trait BitwisePrimitives
where
    Self: Sized,
{
    /* new mask */

    /// Returns a bitmask of ones from the `[start..=end]` range.
    ///
    /// Sets the rest of the bits to 0.
    ///
    /// This is the base of the rest of the mask related functionality.
    /// # Panics
    /// Panics in debug if `start >= Self::BITS` || `end >= Self::BITS` || `start > end`.
    #[doc = include_str!("./Benchmarks_bit_mask_range.md")]
    #[must_use]
    fn bit_mask_range(start: u32, end: u32) -> Self;

    /// Returns a bitmask of ones from the `[start..=end]` checked range.
    ///
    /// Sets the rest of the bits to 0.
    ///
    /// This is the base of the rest of the mask related functionality.
    /// # Errors
    /// Returns [`OutOfBounds`][E::OutOfBounds] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`][E::MismatchedIndices] if `start > end`.
    #[doc = include_str!("./Benchmarks_bit_mask_checked_range.md")]
    fn bit_mask_checked_range(start: u32, end: u32) -> Result<Self>;

    /* get */

    /// Gets the bits in `self` from the `[start..=end]` range.
    ///
    /// Sets the rest of the bits to 0.
    /// # Panics
    /// Panics in debug if `start >= Self::BITS` || `end >= Self::BITS` || `start > end`.
    #[must_use]
    fn bit_get_range(self, start: u32, end: u32) -> Self;

    /// Gets the bits in `self` from the `[start..=end]` checked range.
    ///
    /// Sets the rest of the bits to 0.
    /// # Errors
    /// Returns [`OutOfBounds`][E::OutOfBounds] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`][E::MismatchedIndices] if `start > end`.
    fn bit_get_checked_range(self, start: u32, end: u32) -> Result<Self>;

    /* get shift */

    /// Gets the rightwards shifted bits in `self` from the `[start..=end]` range.
    ///
    /// Like [`bit_get_range`][Self:bit_get_range] and then shifting rightwards
    /// so that the least significant bit (LSB) of the range aligns with the units place.
    /// # Panics
    /// Panics in debug if `start >= Self::BITS` || `end >= Self::BITS` || `start > end`.
    #[must_use]
    fn bit_get_shifted_range(self, start: u32, end: u32) -> Self;

    /// Gets the rightwards shifted bits in `self` from the `[start..=end]` checked range.
    ///
    /// Like [`bit_get_checked_range`][Self:bit_get_cheked_range] and then shifting rightwards
    /// so that the least significant bit (LSB) of the range aligns with the units place.
    /// # Errors
    /// Returns [`OutOfBounds`][E::OutOfBounds] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`][E::MismatchedIndices] if `start > end`.
    fn bit_get_shifted_checked_range(self, start: u32, end: u32) -> Result<Self>;

    /* set */

    /// Sets the bits in `self` to 1 from the `[start..=end]` range.
    ///
    /// Leaves the rest of the bits untouched.
    /// # Panics
    /// Panics in debug if `start >= Self::BITS` || `end >= Self::BITS` || `start > end`.
    #[must_use]
    fn bit_set_range(self, start: u32, end: u32) -> Self;

    /// Sets the bits in `self` to 1 from the `[start..=end]` checked range.
    ///
    /// Leaves the rest of the bits untouched.
    /// # Errors
    /// Returns [`OutOfBounds`][E::OutOfBounds] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`][E::MismatchedIndices] if `start > end`.
    fn bit_set_checked_range(self, start: u32, end: u32) -> Result<Self>;

    /* unset */

    /// Unsets the bits in `self` to 0 from the `[start..=end]` range.
    ///
    /// Leaves the rest of the bits untouched.
    /// # Panics
    /// Panics in debug if `start >= Self::BITS` || `end >= Self::BITS` || `start > end`.
    #[must_use]
    fn bit_unset_range(self, start: u32, end: u32) -> Self;

    /// Unsets the bits in `self` to 0 from the `[start..=end]` checked range.
    ///
    /// Leaves the rest of the bits untouched.
    /// # Errors
    /// Returns [`OutOfBounds`][E::OutOfBounds] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`][E::MismatchedIndices] if `start > end`.
    fn bit_unset_checked_range(self, start: u32, end: u32) -> Result<Self>;

    /* flip */

    /// Flips the bits in `self` from the `[start..=end]` range.
    ///
    /// Leaves the rest of the bits untouched.
    /// # Panics
    /// Panics in debug if `start >= Self::BITS` || `end >= Self::BITS` || `start > end`.
    #[must_use]
    fn bit_flip_range(self, start: u32, end: u32) -> Self;

    /// Flips the bits in `self` from the `[start..=end]` checked range.
    ///
    /// Leaves the rest of the bits untouched.
    /// # Errors
    /// Returns [`OutOfBounds`][E::OutOfBounds] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`][E::MismatchedIndices] if `start > end`.
    fn bit_flip_checked_range(self, start: u32, end: u32) -> Result<Self>;

    /* reverse */

    /// Reverses the order of the bits in `self` from the `[start..=end]` range.
    ///
    /// Leaves the rest of the bits untouched.
    /// # Panics
    /// Panics in debug if `start >= Self::BITS` || `end >= Self::BITS` || `start > end`.
    #[must_use]
    fn bit_reverse_range(self, start: u32, end: u32) -> Self;

    /// Reverses the order of the bits in `self` from the `[start..=end]` checked range.
    ///
    /// Leaves the rest of the bits untouched.
    /// # Errors
    /// Returns [`OutOfBounds`][E::OutOfBounds] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`][E::MismatchedIndices] if `start > end`.
    fn bit_reverse_checked_range(self, start: u32, end: u32) -> Result<Self>;

    /* count */

    /// Counts the number of 1s in `bits` from the `[start..=end]` range.
    /// # Panics
    /// Panics in debug if `start >= Self::BITS` || `end >= Self::BITS` || `start > end`.
    #[must_use]
    fn bit_count_ones_range(self, start: u32, end: u32) -> u32;

    /// Counts the number of 1s in `bits` from the `[start..=end]` checked range.
    /// # Errors
    /// Returns [`OutOfBounds`][E::OutOfBounds] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`][E::MismatchedIndices] if `start > end`.
    fn bit_count_ones_checked_range(self, start: u32, end: u32) -> Result<u32>;

    /// Counts the number of 0s in `bits` from the `[start..=end]` range.
    /// # Panics
    /// Panics in debug if `start >= Self::BITS` || `end >= Self::BITS` || `start > end`.
    #[must_use]
    fn bit_count_zeros_range(self, start: u32, end: u32) -> u32;

    /// Counts the number of 0s in `bits` from the `[start..=end]` checked range.
    /// # Errors
    /// Returns [`OutOfBounds`][E::OutOfBounds] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`][E::MismatchedIndices] if `start > end`.
    fn bit_count_zeros_checked_range(self, start: u32, end: u32) -> Result<u32>;

    /* find first */

    /// Finds the index of the first 1 in `bits` from the `[start..=end]` range.
    ///
    /// Returns `None` if there are no bits set.
    ///
    /// The index is relative to the entire sequence of `bits`, not to the given `start`.
    /// You can always substract `start` from the result afterwards for that.
    /// # Panics
    /// Panics in debug if `start >= Self::BITS` || `end >= Self::BITS` || `start > end`.
    #[must_use]
    fn bit_find_first_one_range(self, start: u32, end: u32) -> Option<u32>;

    /// Finds the index of the first 1 in `bits` from the `[start..=end]` checked range.
    ///
    /// Returns `None` if there are no bits set.
    ///
    /// The index is relative to the entire sequence of `bits`, not to the given `start`.
    /// You can always substract `start` from the result afterwards for that.
    /// # Errors
    /// Returns [`OutOfBounds`][E::OutOfBounds] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`][E::MismatchedIndices] if `start > end`.
    fn bit_find_first_one_checked_range(self, start: u32, end: u32) -> Result<Option<u32>>;

    /// Finds the index of the first 0 in `bits` from the `[start..=end]` range.
    ///
    /// Returns `None` if there are no bits unset.
    ///
    /// The index is relative to the entire sequence of `bits`, not to the given `start`.
    /// You can always substract `start` from the result afterwards for that.
    /// # Panics
    /// Panics in debug if `start >= Self::BITS` || `end >= Self::BITS` || `start > end`.
    #[must_use]
    fn bit_find_first_zero_range(self, start: u32, end: u32) -> Option<u32>;

    /// Finds the index of the first 0 in `bits` from the `[start..=end]` checked range.
    ///
    /// Returns `None` if there are no bits unset.
    ///
    /// The index is relative to the entire sequence of `bits`, not to the given `start`.
    /// You can always substract `start` from the result afterwards for that.
    /// # Errors
    /// Returns [`OutOfBounds`][E::OutOfBounds] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`][E::MismatchedIndices] if `start > end`.
    fn bit_find_first_zero_checked_range(self, start: u32, end: u32) -> Result<Option<u32>>;

    /* find last */

    /// Finds the index of the last 1 in `bits` from the `[start..=end]` range.
    ///
    /// Returns `None` if there are no bits set.
    ///
    /// The index is relative to the entire sequence of `bits`, not to the given `start`.
    /// You can always substract `start` from the result afterwards for that.
    /// # Panics
    /// Panics in debug if `start >= Self::BITS` || `end >= Self::BITS` || `start > end`.
    #[must_use]
    fn bit_find_last_one_range(self, start: u32, end: u32) -> Option<u32>;

    /// Finds the index of the last 1 in `bits` from the `[start..=end]` checked range.
    ///
    /// Returns `None` if there are no bits set.
    ///
    /// The index is relative to the entire sequence of `bits`, not to the given `start`.
    /// You can always substract `start` from the result afterwards for that.
    /// # Errors
    /// Returns [`OutOfBounds`][E::OutOfBounds] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`][E::MismatchedIndices] if `start > end`.
    fn bit_find_last_one_checked_range(self, start: u32, end: u32) -> Result<Option<u32>>;

    /// Finds the index of the last 0 in `bits` from the `[start..=end]` range.
    ///
    /// Returns `None` if there are no bits unset.
    ///
    /// The index is relative to the entire sequence of `bits`, not to the given `start`.
    /// You can always substract `start` from the result afterwards for that.
    /// # Panics
    /// Panics in debug if `start >= Self::BITS` || `end >= Self::BITS` || `start > end`.
    #[must_use]
    fn bit_find_last_zero_range(self, start: u32, end: u32) -> Option<u32>;

    /// Finds the index of the last 0 in `bits` from the `[start..=end]` checked range.
    ///
    /// Returns `None` if there are no bits unset.
    ///
    /// The index is relative to the entire sequence of `bits`, not to the given `start`.
    /// You can always substract `start` from the result afterwards for that.
    /// # Errors
    /// Returns [`OutOfBounds`][E::OutOfBounds] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`][E::MismatchedIndices] if `start > end`.
    fn bit_find_last_zero_checked_range(self, start: u32, end: u32) -> Result<Option<u32>>;
}

macro_rules! impl_bitwise_trait {
    ($($t:ty),+) => { $( impl_bitwise_trait![@$t]; )+ };
    (@$t:ty) => { paste! {
        impl BitwisePrimitives for $t {
            // new mask
            fn bit_mask_range(start: u32, end: u32) -> Self {
                [<bit_mask_range_ $t>](start, end)
            }
            fn bit_mask_checked_range(start: u32, end: u32) -> Result<Self> {
                [<bit_mask_checked_range_ $t>](start, end)
            }
            // get
            fn bit_get_range(self, start: u32, end: u32) -> Self {
                [<bit_get_range_ $t>](self, start, end)
            }
            fn bit_get_checked_range(self, start: u32, end: u32) -> Result<Self> {
                [<bit_get_checked_range_ $t>](self, start, end)
            }
            // get shifted
            fn bit_get_shifted_range(self, start: u32, end: u32) -> Self {
                [<bit_get_shifted_range_ $t>](self, start, end)
            }
            fn bit_get_shifted_checked_range(self, start: u32, end: u32) -> Result<Self> {
                [<bit_get_shifted_checked_range_ $t>](self, start, end)
            }
            // set
            fn bit_set_range(self, start: u32, end: u32) -> Self {
                [<bit_set_range_ $t>](self, start, end)
            }
            fn bit_set_checked_range(self, start: u32, end: u32) -> Result<Self> {
                [<bit_set_checked_range_ $t>](self, start, end)
            }
            // unset
            fn bit_unset_range(self, start: u32, end: u32) -> Self {
                [<bit_unset_range_ $t>](self, start, end)
            }
            fn bit_unset_checked_range(self, start: u32, end: u32) -> Result<Self> {
                [<bit_unset_checked_range_ $t>](self, start, end)
            }
            // flip
            fn bit_flip_range(self, start: u32, end: u32) -> Self {
                [<bit_flip_range_ $t>](self, start, end)
            }
            fn bit_flip_checked_range(self, start: u32, end: u32) -> Result<Self> {
                [<bit_flip_checked_range_ $t>](self, start, end)
            }
            // reverse
            fn bit_reverse_range(self, start: u32, end: u32) -> Self {
                [<bit_reverse_range_ $t>](self, start, end)
            }
            fn bit_reverse_checked_range(self, start: u32, end: u32) -> Result<Self> {
                [<bit_reverse_checked_range_ $t>](self, start, end)
            }
            // count
            fn bit_count_ones_range(self, start: u32, end: u32) -> u32 {
                [<bit_count_ones_range_ $t>](self, start, end)
            }
            fn bit_count_ones_checked_range(self, start: u32, end: u32) -> Result<u32> {
                [<bit_count_ones_checked_range_ $t>](self, start, end)
            }
            fn bit_count_zeros_range(self, start: u32, end: u32) -> u32 {
                [<bit_count_zeros_range_ $t>](self, start, end)
            }
            fn bit_count_zeros_checked_range(self, start: u32, end: u32) -> Result<u32> {
                [<bit_count_zeros_checked_range_ $t>](self, start, end)
            }
            // find first
            fn bit_find_first_one_range(self, start: u32, end: u32) -> Option<u32> {
                [<bit_find_first_one_range_ $t>](self, start, end)
            }
            fn bit_find_first_one_checked_range(self, start: u32, end: u32) -> Result<Option<u32>> {
                [<bit_find_first_one_checked_range_ $t>](self, start, end)
            }
            fn bit_find_first_zero_range(self, start: u32, end: u32) -> Option<u32> {
                [<bit_find_first_zero_range_ $t>](self, start, end)
            }
            fn bit_find_first_zero_checked_range(self, start: u32, end: u32) -> Result<Option<u32>> {
                [<bit_find_first_zero_checked_range_ $t>](self, start, end)
            }
            // find last
            fn bit_find_last_one_range(self, start: u32, end: u32) -> Option<u32> {
                [<bit_find_last_one_range_ $t>](self, start, end)
            }
            fn bit_find_last_one_checked_range(self, start: u32, end: u32) -> Result<Option<u32>> {
                [<bit_find_last_one_checked_range_ $t>](self, start, end)
            }
            fn bit_find_last_zero_range(self, start: u32, end: u32) -> Option<u32> {
                [<bit_find_last_zero_range_ $t>](self, start, end)
            }
            fn bit_find_last_zero_checked_range(self, start: u32, end: u32) -> Result<Option<u32>> {
                [<bit_find_last_zero_checked_range_ $t>](self, start, end)
            }
        }
    }};
}
impl_bitwise_trait![u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];

/* functions */

// Implements bit operations for primitives
//
// `$t`: the type
macro_rules! impl_bitwise_fns {
    () => {
        /* independent of pointer width */
        impl_bitwise_fns![
            i8, i16, i32, i64, i128, isize,
            u8, u16, u32, u64, u128, usize
        ];
    };
    ( $( $t:ty ),+ ) => { $( impl_bitwise_fns![@$t]; )+ };
    (@$t:ty) => { paste! {
        /* new mask */

        /// Returns a new bitmask of 1s from the `[start..=end]` range.
        ///
        /// Sets the rest of the bits to 0.
        ///
        /// This is the base of most of the related bitwise functionality.
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[doc = include_str!("./Benchmarks_bit_mask_range.md")]
        #[must_use] #[inline]
        pub const fn [<bit_mask_range_ $t>](start: u32, end: u32) -> $t {
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
        /// Returns a new bitmask of ones from the `[start..=end]` checked range.
        ///
        /// Sets the rest of the bits to 0.
        ///
        /// This is the base of most of the related bitwise functionality.
        /// # Errors
        #[doc = "Returns [`OutOfBounds`][E::OutOfBounds] if `start >= `[`"
            $t "::BITS`]` || end >= `[`" $t "::BITS`] and
            [`MismatchedIndices`][E::MismatchedIndices] if `start > end`."]
        #[doc = include_str!("./Benchmarks_bit_mask_checked_range.md")]
        #[inline]
        pub const fn [<bit_mask_checked_range_ $t>](start: u32, end: u32) -> Result<$t> {
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

        /// Gets the bits in `bits` from the `[start..=end]` range.
        ///
        /// Sets the rest of the bits to 0.
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[must_use] #[inline]
        pub const fn [<bit_get_range_ $t>](bits: $t, start: u32, end: u32) -> $t {
            (bits & [< bit_mask_range_ $t>](start, end))
        }

        /// Gets the bits in `bits` from the `[start..=end]` checked range.
        ///
        /// Sets the rest of the bits to 0.
        /// # Errors
        #[doc = "Returns [`OutOfBounds`][E::OutOfBounds] if `start >= `[`"
            $t "::BITS`]` || end >= `[`" $t "::BITS`] and
            [`MismatchedIndices`][E::MismatchedIndices] if `start > end`."]
        #[inline]
        pub const fn [<bit_get_checked_range_ $t>](bits: $t, start: u32, end: u32) -> Result<$t> {
            match [< bit_mask_checked_range_ $t>](start, end) {
                Ok(mask) => Ok(bits & mask),
                Err(e) => Err(e),
            }
        }

        /* get shifted */

        /// Gets the shifted bits in `bits` from the `[start..=end]` range.
        ///
        #[doc = "Like [`bit_get_range_ " $t "`] and then shifting rightwards"]
        /// so that the least significant bit (LSB) of the range aligns with the units place.
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[must_use] #[inline]
        pub const fn [<bit_get_shifted_range_ $t>](bits: $t, start: u32, end: u32) -> $t {
            (bits & [< bit_mask_range_ $t>](start, end)) >> start
        }

        /// Gets the shifted bits in `bits` from the `[start..=end]` checked range.
        ///
        #[doc = "Like [`bit_get_checked_range_ " $t "`] and then shifting rightwards"]
        /// so that the least significant bit (LSB) of the range aligns with the units place.
        /// # Errors
        #[doc = "Returns [`OutOfBounds`][E::OutOfBounds] if `start >= `[`"
            $t "::BITS`]` || end >= `[`" $t "::BITS`] and
            [`MismatchedIndices`][E::MismatchedIndices] if `start > end`."]
        #[inline]
        pub const fn [<bit_get_shifted_checked_range_ $t>](bits: $t, start: u32, end: u32) -> Result<$t> {
            match [< bit_mask_checked_range_ $t>](start, end) {
                Ok(mask) => Ok((bits & mask) >> start),
                Err(e) => Err(e),
            }
        }

        /* set */

        /// Sets the bits in `bits` to 1, from the `[start..=end]` range.
        ///
        /// Leaves the rest of the bits untouched.
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[must_use] #[inline]
        pub const fn [<bit_set_range_ $t>](bits: $t, start: u32, end: u32) -> $t {
            bits | [< bit_mask_range_ $t>](start, end)
        }

        /// Sets the bits in `bits` to 1, from the `[start..=end]` checked range.
        ///
        /// Leaves the rest of the bits untouched.
        /// # Errors
        #[doc = "Returns [`OutOfBounds`][E::OutOfBounds] if `start >= `[`"
            $t "::BITS`]` || end >= `[`" $t "::BITS`] and
            [`MismatchedIndices`][E::MismatchedIndices] if `start > end`."]
        #[inline]
        pub const fn [<bit_set_checked_range_ $t>](bits: $t, start: u32, end: u32) -> Result<$t> {
            match [< bit_mask_checked_range_ $t>](start, end) {
                Ok(mask) => Ok(bits | mask),
                Err(e) => Err(e),
            }
        }

        /* unset */

        /// Unsets the bits in `bits` to 0, from the `[start..=end]` range.
        ///
        /// Leaves the rest of the bits untouched.
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[must_use] #[inline]
        pub const fn [<bit_unset_range_ $t>](bits: $t, start: u32, end: u32) -> $t {
            bits & ![< bit_mask_range_ $t>](start, end)
        }

        /// Unsets the bits in `bits` to 0, from the `[start..=end]` checked range.
        ///
        /// Leaves the rest of the bits untouched.
        /// # Errors
        #[doc = "Returns [`OutOfBounds`][E::OutOfBounds] if `start >= `[`"
            $t "::BITS`]` || end >= `[`" $t "::BITS`] and
            [`MismatchedIndices`][E::MismatchedIndices] if `start > end`."]
        #[inline]
        pub const fn [<bit_unset_checked_range_ $t>](bits: $t, start: u32, end: u32) -> Result<$t> {
            match [< bit_mask_checked_range_ $t>](start, end) {
                Ok(mask) => Ok(bits & !mask),
                Err(e) => Err(e),
            }
        }

        /* flip */

        /// Flips the bits in `bits` from the `[start..=end]` range.
        ///
        /// Leaves the rest of the bits untouched.
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[must_use] #[inline]
        pub const fn [<bit_flip_range_ $t>](bits: $t, start: u32, end: u32) -> $t {
            bits ^ [< bit_mask_range_ $t>](start, end)
        }

        /// Flips the bits in `bits` from the `[start..=end]` checked range.
        ///
        /// Leaves the rest of the bits untouched.
        /// # Errors
        #[doc = "Returns [`OutOfBounds`][E::OutOfBounds] if `start >= `[`"
            $t "::BITS`]` || end >= `[`" $t "::BITS`] and
            [`MismatchedIndices`][E::MismatchedIndices] if `start > end`."]
        #[inline]
        pub const fn [<bit_flip_checked_range_ $t>](bits: $t, start: u32, end: u32) -> Result<$t> {
            match [< bit_mask_checked_range_ $t>](start, end) {
                Ok(mask) => Ok(bits ^ mask),
                Err(e) => Err(e),
            }
        }

        /* reverse */

        /// Reverses the order of the bits in `bits` from the `[start..=end]` range.
        ///
        /// Leaves the rest of the bits untouched.
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[must_use ] #[inline]
        pub const fn [<bit_reverse_range_ $t>](bits: $t, start: u32, end: u32) -> $t {
            debug_assert![start <= end];
            // If the entire range of bits is selected, simply reverse all bits
            let range_bits = end - start + 1;
            iif![range_bits == <$t>::BITS; return bits.reverse_bits()];
            // Create the mask for the range and reverse its bits
            let mask = (((1 as $t) << range_bits) - 1) << start;
            let bits_to_rev = (bits & mask) >> start;
            let rev = bits_to_rev.reverse_bits();
            // Shift the reversed bits back to their original position
            let rev_shifted = (rev >> (<$t>::BITS - range_bits)) << start;
            // Combine with the original number, preserving bits outside the range
            (bits & !mask) | rev_shifted
        }

        /// Reverses the order of the bits in `bits` from the `[start..=end]` checked range.
        ///
        /// Leaves the rest of the bits untouched.
        /// # Errors
        #[doc = "Returns [`OutOfBounds`][E::OutOfBounds] if `start >= `[`"
            $t "::BITS`]` || end >= `[`" $t "::BITS`] and
            [`MismatchedIndices`][E::MismatchedIndices] if `start > end`."]
        #[inline]
        pub const fn [<bit_reverse_checked_range_ $t>](bits: $t, start: u32, end: u32) -> Result<$t> {
            if start >= <$t>::BITS {
                Err(E::OutOfBounds(Some(start as usize)))
            } else if end >= <$t>::BITS {
                Err(E::OutOfBounds(Some(end as usize)))
            } else if start > end {
                Err(E::MismatchedIndices)
            } else {
                // If the entire range of bits is selected, simply reverse all bits
                let range_bits = end - start + 1;
                iif![range_bits == <$t>::BITS; return Ok(bits.reverse_bits())];
                // Create the mask for the range and reverse its bits
                let mask = (((1 as $t) << range_bits) - 1) << start;
                let bits_to_rev = (bits & mask) >> start;
                let rev = bits_to_rev.reverse_bits();
                // Shift the reversed bits back to their original position
                let rev_shifted = (rev >> (<$t>::BITS - range_bits)) << start;
                // Combine with the original number, preserving bits outside the range
                Ok((bits & !mask) | rev_shifted)
            }
        }

        /* count */

        /// Counts the number of 1s in `bits` from the `[start..=end]` range.
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[must_use] #[inline]
        pub const fn [<bit_count_ones_range_ $t>](bits: $t, start: u32, end: u32) -> u32 {
            let masked_bits = bits & [<bit_mask_range_ $t>](start, end);
            masked_bits.count_ones()
        }
        /// Counts the number of 1s in `bits` from the `[start..=end]` checked range.
        /// # Errors
        #[doc = "Returns [`OutOfBounds`][E::OutOfBounds] if `start >= `[`"
            $t "::BITS`]` || end >= `[`" $t "::BITS`] and
            [`MismatchedIndices`][E::MismatchedIndices] if `start > end`."]
        #[inline]
        pub const fn [<bit_count_ones_checked_range_ $t>](bits: $t, start: u32, end: u32) -> Result<u32> {
            match [<bit_mask_checked_range_ $t>](start, end) {
                Ok(mask) => Ok((bits & mask).count_ones()),
                Err(e) => Err(e),
            }
        }

        /// Counts the number of 0s in `bits` from the `[start..=end]` range.
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[must_use] #[inline]
        pub const fn [<bit_count_zeros_range_ $t>](bits: $t, start: u32, end: u32) -> u32 {
            let mask = [<bit_mask_range_ $t>](start, end);
            let masked_bits = bits & mask;
            (!masked_bits & mask).count_ones()
        }

        /// Counts the number of 0s in `bits` from the `[start..=end]` checked range.
        /// # Errors
        #[doc = "Returns [`OutOfBounds`][E::OutOfBounds] if `start >= `[`"
            $t "::BITS`]` || end >= `[`" $t "::BITS`] and
            [`MismatchedIndices`][E::MismatchedIndices] if `start > end`."]
        #[inline]
        pub const fn [<bit_count_zeros_checked_range_ $t>](bits: $t, start: u32, end: u32) -> Result<u32> {
            match [<bit_mask_checked_range_ $t>](start, end) {
                Ok(mask) => {
                    let masked_bits = bits & mask;
                    Ok((!masked_bits & mask).count_ones())
                },
                Err(e) => Err(e),
            }
        }

        /* find first */

        /// Finds the index of the first 1 in `bits` from the `[start..=end]` range.
        ///
        /// Returns `None` if there are no bits set.
        ///
        /// The index is relative to the entire sequence of `bits`, not to the given `start`.
        /// You can always substract `start` from the result afterwards for that.
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[must_use] #[inline]
        pub const fn [<bit_find_first_one_range_ $t>](bits: $t, start: u32, end: u32)
            -> Option<u32> {
            let masked_bits = bits & [<bit_mask_range_ $t>](start, end);
            let mut idx = start;
            while idx <= end {
                iif![(masked_bits & (1 << idx)) != 0; return Some(idx)];
                idx += 1;
            }
            None
        }

        /// Finds the index of the first 1 in `bits` from the `[start..=end]` checked range.
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
        pub const fn [<bit_find_first_one_checked_range_ $t>](bits: $t, start: u32, end: u32)
            -> Result<Option<u32>> {
            match [<bit_mask_checked_range_ $t>](start, end) {
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

        /// Finds the index of the first 0 in `bits` from the `[start..=end]` range.
        ///
        /// Returns `None` if there are no bits unset.
        ///
        /// The index is relative to the entire sequence of `bits`, not to the given `start`.
        /// You can always substract `start` from the result afterwards for that.
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[must_use] #[inline]
        pub const fn [<bit_find_first_zero_range_ $t>](bits: $t, start: u32, end: u32)
            -> Option<u32> {
            let masked_bits = !(bits & [<bit_mask_range_ $t>](start, end));
            let mut idx = start;
            while idx <= end {
                iif![(masked_bits & (1 << idx)) != 0; return Some(idx)];
                idx += 1;
            }
            None
        }

        /// Finds the index of the first 0 in `bits` from the `[start..=end]` checked range.
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
        pub const fn [<bit_find_first_zero_checked_range_ $t>](bits: $t, start: u32, end: u32)
            -> Result<Option<u32>> {
            match [<bit_mask_checked_range_ $t>](start, end) {
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

        /* find last */

        /// Finds the index of the last 1 in `bits` from the `[start..=end]` range.
        ///
        /// Returns `None` if there are no bits set.
        ///
        /// The index is relative to the entire sequence of `bits`, not to the given `start`.
        /// You can always substract `start` from the result afterwards for that.
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[must_use] #[inline]
        pub const fn [<bit_find_last_one_range_ $t>](bits: $t, start: u32, end: u32) -> Option<u32> {
            let masked_bits = bits & [<bit_mask_range_ $t>](start, end);
            let mut idx = end;
            loop {
                iif![(masked_bits & (1 << idx)) != 0; return Some(idx)];
                iif![idx == start; break];
                idx -= 1;
            }
            None
        }

        /// Finds the index of the last 1 in `bits` from the `[start..=end]` checked range.
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
        pub const fn [<bit_find_last_one_checked_range_ $t>](bits: $t, start: u32, end: u32) -> Result<Option<u32>> {
            match [<bit_mask_checked_range_ $t>](start, end) {
                Ok(mask) => {
                    let masked_bits = bits & mask;
                    let mut idx = end;
                    loop {
                        iif![(masked_bits & (1 << idx)) != 0; return Ok(Some(idx))];
                        iif![idx == start; break];
                        idx -= 1;
                    }
                    Ok(None)
                },
                Err(e) => Err(e),
            }
        }

        /// Finds the index of the last 0 in `bits` from the `[start..=end]` range.
        ///
        /// Returns `None` if there are no bits set.
        ///
        /// The index is relative to the entire sequence of `bits`, not to the given `start`.
        /// You can always substract `start` from the result afterwards for that.
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[must_use] #[inline]
        pub const fn [<bit_find_last_zero_range_ $t>](bits: $t, start: u32, end: u32) -> Option<u32> {
            let masked_bits = !(bits & [<bit_mask_range_ $t>](start, end));
            let mut idx = end;
            loop {
                iif![(masked_bits & (1 << idx)) != 0; return Some(idx)];
                iif![idx == start; break];
                idx -= 1;
            }
            None
        }

        /// Finds the index of the last 0 in `bits` from the `[start..=end]` checked range.
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
        pub const fn [<bit_find_last_zero_checked_range_ $t>](bits: $t, start: u32, end: u32) -> Result<Option<u32>> {
            match [<bit_mask_checked_range_ $t>](start, end) {
                Ok(mask) => {
                    let masked_bits = !(bits & mask);
                    let mut idx = end;
                    loop {
                        iif![(masked_bits & (1 << idx)) != 0; return Ok(Some(idx))];
                        iif![idx == start; break];
                        idx -= 1;
                    }
                    Ok(None)
                },
                Err(e) => Err(e),
            }
        }
    }};
}
impl_bitwise_fns![];
