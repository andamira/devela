// devela::data::conversion::primitive::bits
//
//! fns for extended bit manipulation
//

use crate::{
    data::{DataErrors as E, DataResult as Result},
    meta::{iif, paste},
};

// Implements bit operations for primitives
//
// `$t`: the type
macro_rules! impl_bits {
    () => {
        /* independent of pointer width */
        impl_bits![
            i8, i16, i32, i64, i128, isize,
            u8, u16, u32, u64, u128, usize
        ];
    };
    ( $( $t:ty ),+ ) => { $( impl_bits![@$t]; )+ };
    (@$t:ty) => { paste! {
        /// Returns a bitmask set from the `start` to the `end` bit, inclusive.
        ///
        /// # Errors
        #[doc = "Returns [`OutOfBounds`][E::OutOfBounds] if `start >= `[`"
            $t "::BITS`]` || end >= `[`" $t "::BITS`] and
            [`MismatchedIndices`][E::MismatchedIndices] if `start > end`."]
        ///
        #[doc = include_str!("./Benchmarks_bit_mask_checked.md")]
        #[inline]
        pub const fn [<bit_mask_range_ $t>](start: u32, end: u32) -> Result<$t> {
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

        /// Returns a bitmask set from the `start` to the `end` bit, inclusive. Unchecked version.
        ///
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[doc = include_str!("./Benchmarks_bit_mask_unchecked.md")]
        #[must_use] #[inline]
        pub const fn [<bit_mask_range_unchecked_ $t>](start: u32, end: u32) -> $t {
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

        /// Sets the bits in `bits` to 1, from `start` to `end`, inclusive.
        ///
        /// # Errors
        #[doc = "Returns [`OutOfBounds`][E::OutOfBounds] if `start >= `[`"
            $t "::BITS`]` || end >= `[`" $t "::BITS`] and
            [`MismatchedIndices`][E::MismatchedIndices] if `start > end`."]
        #[inline]
        pub const fn [<bit_set_range_ $t>](bits: $t, start: u32, end: u32) -> Result<$t> {
            match [< bit_mask_range_ $t>](start, end) {
                Ok(mask) => Ok(bits | mask),
                Err(e) => Err(e),
            }
        }
        /// Sets the bits in `bits` to 1, from `start` to `end`, inclusive, unchecked version.
        ///
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[must_use] #[inline]
        pub const fn [<bit_set_range_unchecked_ $t>](bits: $t, start: u32, end: u32) -> $t {
            bits | [< bit_mask_range_unchecked_ $t>](start, end)
        }

        /// Unsets the bits in `bits` to 0, from `start` to `end`, inclusive.
        ///
        /// # Errors
        #[doc = "Returns [`OutOfBounds`][E::OutOfBounds] if `start >= `[`"
            $t "::BITS`]` || end >= `[`" $t "::BITS`] and
            [`MismatchedIndices`][E::MismatchedIndices] if `start > end`."]
        #[inline]
        pub const fn [<bit_unset_range_ $t>](bits: $t, start: u32, end: u32) -> Result<$t> {
            match [< bit_mask_range_ $t>](start, end) {
                Ok(mask) => Ok(bits & !mask),
                Err(e) => Err(e),
            }
        }
        /// Unsets the bits in `bits` to 0, from `start` to `end`, inclusive, unchecked version.
        ///
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[must_use] #[inline]
        pub const fn [<bit_unset_range_unchecked_ $t>](bits: $t, start: u32, end: u32) -> $t {
            bits & ![< bit_mask_range_unchecked_ $t>](start, end)
        }

        /// Flips the bits in `bits` from `start` to `end`, inclusive.
        ///
        /// # Errors
        #[doc = "Returns [`OutOfBounds`][E::OutOfBounds] if `start >= `[`"
            $t "::BITS`]` || end >= `[`" $t "::BITS`] and
            [`MismatchedIndices`][E::MismatchedIndices] if `start > end`."]
        #[inline]
        pub const fn [<bit_flip_range_ $t>](bits: $t, start: u32, end: u32) -> Result<$t> {
            match [< bit_mask_range_ $t>](start, end) {
                Ok(mask) => Ok(bits ^ mask),
                Err(e) => Err(e),
            }
        }
        /// Flips the bits in `bits` from `start` to `end`, inclusive.
        ///
        /// # Panics
        #[doc = "Panics in debug if `start >= `[`" $t "::BITS`]` || end >= `[`"
            $t "::BITS`]` || start > end`."]
        #[must_use] #[inline]
        pub const fn [<bit_flip_range_unchecked_ $t>](bits: $t, start: u32, end: u32) -> $t {
            bits ^ [< bit_mask_range_unchecked_ $t>](start, end)
        }
    }};
}
impl_bits![];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn bit_mask_range() {
        assert_eq![0b_0000_0001, bit_mask_range_u8(0, 0).unwrap()];
        assert_eq![0b_0000_0001, bit_mask_range_unchecked_u8(0, 0)];
        assert_eq![0b_1000_0000, bit_mask_range_u8(7, 7).unwrap()];
        assert_eq![0b_1000_0000, bit_mask_range_unchecked_u8(7, 7)];
        assert_eq![0b_0111_1110, bit_mask_range_u8(1, 6).unwrap()];
        assert_eq![0b_0111_1110, bit_mask_range_unchecked_u8(1, 6)];

        debug_assert![bit_mask_range_u8(8, 8).is_err()];
        debug_assert![bit_mask_range_u8(0, 8).is_err()];
        debug_assert![bit_mask_range_u8(4, 1).is_err()];
        #[cfg(feature = "std")]
        {
            use std::panic::catch_unwind;
            debug_assert![catch_unwind(|| {
                let _ = bit_mask_range_unchecked_u8(8, 8); }).is_err()];
            debug_assert![catch_unwind(|| {
                let _ = bit_mask_range_unchecked_u8(0, 8); }).is_err()];
            debug_assert![catch_unwind(|| {
                let _ = bit_mask_range_unchecked_u8(4, 1); }).is_err()];
        }
    }
    #[test]
    #[rustfmt::skip]
    fn bit_ops() { // ALL
        let bits = 0b_1111_0000;
        assert_eq![0b_1111_1100, bit_set_range_u8(bits, 2, 5).unwrap()];
        assert_eq![0b_1100_0000, bit_unset_range_u8(bits, 2, 5).unwrap()];
        assert_eq![0b_1100_1100, bit_flip_range_u8(bits, 2, 5).unwrap()];
    }
}
