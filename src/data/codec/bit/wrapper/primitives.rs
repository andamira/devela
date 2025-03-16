// devela::data::codec::bit::ops::wrapper::primitives
//
//! Implements `Bitwise` for the integer primitives.
//

#[cfg(_bit··)]
use crate::{
    iif, Bitwise,
    MismatchedBounds::{self, DataOverflow, IndexOutOfBounds, MismatchedIndices},
};

macro_rules! impl_bits_wrapper {
    () => {
        impl_bits_wrapper![
            i8:"_bit_i8", i16:"_bit_i16", i32:"_bit_i32",
            i64:"_bit_i64", i128:"_bit_i128", isize:"_bit_isize",
            u8:"_bit_u8", u16:"_bit_u16", u32:"_bit_u32",
            u64:"_bit_u64", u128:"_bit_u128", usize:"_bit_usize"
        ];
    };
    ( $( $t:ty : $cap:literal ),+ ) => { $( impl_bits_wrapper![@$t : $cap]; )+ };

    // `$t`: the primitive type
    // $cap: the capability feature that enables the given implementation. E.g "_bit_u8".
    (@$t:ty : $cap:literal) => {
        /* impl traits */

        #[doc = concat!["# Implementation for `", stringify!($t), "`."]]
        #[cfg(feature = $cap )]
        #[cfg_attr(nightly_doc, doc(cfg(feature = $cap)))]
        impl Bitwise::<$t> {
            /* constants */

            /// The size in bits.
            pub const BITS: u32 = <$t>::BITS;

            /* new mask */

            /// Returns a new bitmask of 1s from the `[start..=end]` range.
            ///
            /// Sets the rest of the bits to 0.
            ///
            /// # Panics
            /// Panics if `start >= BITS || end >= BITS || start > end`.
            #[doc = include_str!("../benches/mask_range.md")]
            pub const fn mask_range(start: u32, end: u32) -> Self {
                debug_assert![start <= end];
                // a mask with all bits set, from 0 to end:
                let mask_end = iif![end == <$t>::BITS -1; !0; (1 << (end + 1)) - 1];
                // a mask with all bits set from 0 to start - 1:
                let mask_start = iif![start == 0; 0; (1 << start) - 1];
                Self(mask_end - mask_start)
            }
            /// Returns a new bitmask of ones from the `[start..=end]` checked range.
            ///
            /// Sets the rest of the bits to 0.
            ///
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS` and
            /// [`MismatchedIndices`] if `start > end`.
            #[doc = include_str!("../benches/mask_checked_range.md")]
            pub const fn mask_checked_range(start: u32, end: u32)
                -> Result<Self, MismatchedBounds> {
                if start >= <$t>::BITS {
                    Err(IndexOutOfBounds(Some(start as usize)))
                } else if end >= <$t>::BITS {
                    Err(IndexOutOfBounds(Some(end as usize)))
                } else if start > end {
                    Err(MismatchedIndices)
                } else {
                    // create a mask with all bits set, from 0 to end:
                    let mask_end = iif![end == <$t>::BITS -1; !0; (1 << (end + 1)) - 1];
                    // create a mask with all bits set from 0 to start - 1:
                    let mask_start = iif![start == 0; 0; (1 << start) - 1];
                    Ok(Self(mask_end - mask_start))
                }
            }

            /* get */

            /// Gets the bits in `self` from the `[start..=end]` range.
            ///
            /// Sets the rest of the bits to 0.
            /// # Panics
            /// Panics if `start >= BITS || end >= BITS || start > end`.
            pub const fn get_range(self, start: u32, end: u32) -> Self {
                Self(self.0 & Self::mask_range(start, end).0)
            }

            /// Gets the bits in `self` from the `[start..=end]` checked range.
            ///
            /// Sets the rest of the bits to 0.
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS` and
            /// [`MismatchedIndices`] if `start > end`.
            pub const fn get_checked_range(self, start: u32, end: u32)
                -> Result<Self, MismatchedBounds> {
                match Self::mask_checked_range(start, end) {
                    Ok(mask) => Ok(Self(self.0 & mask.0)),
                    Err(e) => Err(e),
                }
            }

            /* get value */

            /// Gets the value of the bits in `self` from the `[start..=end]` range.
            ///
            /// Sets the rest of the bits to 0.
            ///
            /// The bits in the specified range are shifted rightwards so that the least
            /// significant bit (LSB) aligns with the units place, forming the integer value.
            /// # Panics
            /// Panics if `start >= BITS || end >= BITS || start > end`.
            pub const fn get_value_range(self, start: u32, end: u32) -> Self {
                Self((self.0 & Self::mask_range(start, end).0) >> start)
            }

            /// Gets the value of the bits in `self` from the `[start..=end]` checked range.
            ///
            /// Sets the rest of the bits to 0.
            ///
            /// The bits in the specified range are shifted rightwards so that the least
            /// significant bit (LSB) aligns with the units place, forming the integer value.
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS` and
            /// [`MismatchedIndices`] if `start > end`.
            pub const fn get_value_checked_range(self, start: u32, end: u32)
                -> Result<Self, MismatchedBounds> {
                match Self::mask_checked_range(start, end) {
                    Ok(mask) => Ok(Self((self.0 & mask.0) >> start)),
                    Err(e) => Err(e),
                }
            }

            /* set */

            /// Sets the bits in `self` to 1, from the `[start..=end]` range.
            ///
            /// Leaves the rest of the bits unchanged.
            /// # Panics
            /// Panics if `start >= BITS || end >= BITS || start > end`.
            pub const fn set_range(self, start: u32, end: u32) -> Self {
                Self(self.0 | Self::mask_range(start, end).0)
            }

            /// Sets the bits in `self` to 1, from the `[start..=end]` checked range.
            ///
            /// Leaves the rest of the bits unchanged.
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS`
            /// and [`MismatchedIndices`] if `start > end`.
            pub const fn set_checked_range(self, start: u32, end: u32)
                -> Result<Self, MismatchedBounds> {
                match Self::mask_checked_range(start, end) {
                    Ok(mask) => Ok(Self(self.0 | mask.0)),
                    Err(e) => Err(e),
                }
            }

            /* set value */

            /// Sets the given `value` into the bits from the `[start..=end]` range.
            ///
            /// Leaves the rest of the bits unchanged.
            ///
            /// The value is first masked to fit the size of the range, and then
            /// it is inserted into the specified bit range of `self`, replacing
            /// the existing bits in that range. The rest of the bits in `self` remain unchanged.
            /// # Panics
            /// Panics if `start >= BITS || end >= BITS || start > end`.
            pub const fn set_value_range(self, value: $t, start: u32, end: u32) -> Self {
                let mask = Self::mask_range(start, end).0;
                let value_shifted = (value << start) & mask;
                Self((self.0 & !mask) | value_shifted)
            }

            /// Sets the given `value` into the bits from the `[start..=end]` checked range.
            ///
            /// Leaves the rest of the bits unchanged.
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS`
            /// and [`MismatchedIndices`] if `start > end`.
            pub const fn set_value_checked_range(self, value: $t, start: u32, end: u32)
                -> Result<Self, MismatchedBounds> {
                match Self::mask_checked_range(start, end) {
                    Ok(mask) => {
                        let value_shifted = (value << start) & mask.0;
                        Ok(Self((self.0 & !mask.0) | value_shifted))
                    },
                    Err(e) => Err(e),
                }
            }

            /// Sets the given checked `value` into the bits from the `[start..=end]` checked range.
            ///
            /// Leaves the rest of the bits unchanged.
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS`,
            /// [`MismatchedIndices`] if `start > end`, and
            /// [`DataOverflow`] if `value` does not fit within the specified bit range.
            pub const fn set_checked_value_checked_range(self, value: $t, start: u32, end: u32)
                -> Result<Self, MismatchedBounds> {
                match Self::mask_checked_range(start, end) {
                    Ok(mask) => {
                        iif![value >= (1 << (end - start));
                            return Err(DataOverflow(Some(value as usize)))];
                        let value_shifted = (value << start) & mask.0;
                        Ok(Self((self.0 & !mask.0) | value_shifted))
                    },
                    Err(e) => Err(e),
                }
            }

            /* unset */

            /// Unsets the bits in `self` to 0, from the `[start..=end]` range.
            ///
            /// Leaves the rest of the bits unchanged.
            /// # Panics
            /// Panics if `start >= BITS || end >= BITS || start > end`.
            pub const fn unset_range(self, start: u32, end: u32) -> Self {
                Self(self.0 & !Self::mask_range(start, end).0)
            }

            /// Unsets the bits in `self` to 0, from the `[start..=end]` checked range.
            ///
            /// Leaves the rest of the bits unchanged.
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS` and
            /// [`MismatchedIndices`] if `start > end`.
            pub const fn unset_checked_range(self, start: u32, end: u32)
                -> Result<Self, MismatchedBounds> {
                match Self::mask_checked_range(start, end) {
                    Ok(mask) => Ok(Self(self.0 & !mask.0)),
                    Err(e) => Err(e),
                }
            }

            /* flip */

            /// Flips the bits in `self` from the `[start..=end]` range.
            ///
            /// Leaves the rest of the bits unchanged.
            /// # Panics
            /// Panics if `start >= BITS || end >= BITS || start > end`.
            pub const fn flip_range(self, start: u32, end: u32) -> Self {
                Self(self.0 ^ Self::mask_range(start, end).0)
            }

            /// Flips the bits in `self` from the `[start..=end]` checked range.
            ///
            /// Leaves the rest of the bits unchanged.
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS` and
            /// [`MismatchedIndices`] if `start > end`.
            pub const fn flip_checked_range(self, start: u32, end: u32)
                -> Result<Self, MismatchedBounds> {
                match Self::mask_checked_range(start, end) {
                    Ok(mask) => Ok(Self(self.0 ^ mask.0)),
                    Err(e) => Err(e),
                }
            }

            /* reverse */

            /// Reverses the order of the bits in `self` from the `[start..=end]` range.
            ///
            /// Leaves the rest of the bits unchanged.
            /// # Panics
            /// Panics if `start >= BITS || end >= BITS || start > end`.
            pub const fn reverse_range(self, start: u32, end: u32) -> Self {
                debug_assert![start <= end];
                // If the entire range of bits is selected, simply reverse all bits
                let range_bits = end - start + 1;
                iif![range_bits == Self::BITS; return Self(self.0.reverse_bits())];
                // Create the mask for the range and reverse its bits
                let mask = (((1 as $t) << range_bits) - 1) << start;
                let bits_to_rev = (self.0 & mask) >> start;
                let rev = bits_to_rev.reverse_bits();
                // Shift the reversed bits back to their original position
                let rev_shifted = (rev >> (Self::BITS - range_bits)) << start;
                // Combine with the original number, preserving bits outside the range
                Self((self.0 & !mask) | rev_shifted)
            }

            /// Reverses the order of the bits in `self` from the `[start..=end]` checked range.
            ///
            /// Leaves the rest of the bits unchanged.
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS` and
            /// [`MismatchedIndices`] if `start > end`.
            pub const fn reverse_checked_range(self, start: u32, end: u32)
                -> Result<Self, MismatchedBounds> {
                if start >= Self::BITS {
                    Err(IndexOutOfBounds(Some(start as usize)))
                } else if end >= <$t>::BITS {
                    Err(IndexOutOfBounds(Some(end as usize)))
                } else if start > end {
                    Err(MismatchedIndices)
                } else {
                    // If the entire range of bits is selected, simply reverse all bits
                    let range_bits = end - start + 1;
                    iif![range_bits == Self::BITS; return Ok(Self(self.0.reverse_bits()))];
                    // Create the mask for the range and reverse its bits
                    let mask = (((1 as $t) << range_bits) - 1) << start;
                    let bits_to_rev = (self.0 & mask) >> start;
                    let rev = bits_to_rev.reverse_bits();
                    // Shift the reversed bits back to their original position
                    let rev_shifted = (rev >> (Self::BITS - range_bits)) << start;
                    // Combine with the original number, preserving bits outside the range
                    Ok(Self((self.0 & !mask) | rev_shifted))
                }
            }

            /* count */

            /// Counts the number of 1s in `self` from the `[start..=end]` range.
            /// # Panics
            /// Panics if `start >= BITS || end >= BITS || start > end`.
            #[must_use]
            pub const fn count_ones_range(self, start: u32, end: u32) -> u32 {
                let masked_bits = self.0 & Self::mask_range(start, end).0;
                masked_bits.count_ones()
            }
            /// Counts the number of 1s in `self` from the `[start..=end]` checked range.
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS` and
            /// [`MismatchedIndices`] if `start > end`.
            pub const fn count_ones_checked_range(self, start: u32, end: u32)
                -> Result<u32, MismatchedBounds> {
                match Self::mask_checked_range(start, end) {
                    Ok(mask) => Ok((self.0 & mask.0).count_ones()),
                    Err(e) => Err(e),
                }
            }

            /// Counts the number of 0s in `self` from the `[start..=end]` range.
            /// # Panics
            /// Panics if `start >= BITS || end >= BITS || start > end`.
            #[must_use]
            pub const fn count_zeros_range(self, start: u32, end: u32) -> u32 {
                let mask = Self::mask_range(start, end).0;
                let masked_bits = self.0 & mask;
                (!masked_bits & mask).count_ones()
            }

            /// Counts the number of 0s in `self` from the `[start..=end]` checked range.
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS` and
            /// [`MismatchedIndices`] if `start > end`.
            pub const fn count_zeros_checked_range(self, start: u32, end: u32)
                -> Result<u32, MismatchedBounds> {
                match Self::mask_checked_range(start, end) {
                    Ok(mask) => {
                        let masked_bits = self.0 & mask.0;
                        Ok((!masked_bits & mask.0).count_ones())
                    },
                    Err(e) => Err(e),
                }
            }

            /* find first */

            /// Finds the index of the first 1 in `self` from the `[start..=end]` range.
            ///
            /// Returns `None` if there are no bits set.
            ///
            /// The index is relative to the entire sequence of `self`, not to the given `start`.
            /// # Panics
            /// Panics if `start >= BITS || end >= BITS || start > end`.
            #[must_use]
            pub const fn find_first_one_range(self, start: u32, end: u32) -> Option<u32> {
                let masked_bits = self.0 & Self::mask_range(start, end).0;
                let mut idx = start;
                while idx <= end {
                    iif![(masked_bits & (1 << idx)) != 0; return Some(idx)];
                    idx += 1;
                }
                None
            }

            /// Finds the index of the first 1 in `self` from the `[start..=end]` checked range.
            ///
            /// Returns `None` if there are no bits set.
            ///
            /// The index is relative to the entire sequence of `self`, not to the given `start`.
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS` and
            /// [`MismatchedIndices`] if `start > end`.
            pub const fn find_first_one_checked_range(self, start: u32, end: u32)
                -> Result<Option<u32>, MismatchedBounds> {
                match Self::mask_checked_range(start, end) {
                    Ok(mask) => {
                        let masked_bits = self.0 & mask.0;
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

            /// Finds the index of the first 0 in `self` from the `[start..=end]` range.
            ///
            /// Returns `None` if there are no bits unset.
            ///
            /// The index is relative to the entire sequence of `self`, not to the given `start`.
            /// # Panics
            /// Panics if `start >= BITS || end >= BITS || start > end`.
            #[must_use]
            pub const fn find_first_zero_range(self, start: u32, end: u32)
                -> Option<u32> {
                let masked_bits = !(self.0 & Self::mask_range(start, end).0);
                let mut idx = start;
                while idx <= end {
                    iif![(masked_bits & (1 << idx)) != 0; return Some(idx)];
                    idx += 1;
                }
                None
            }

            /// Finds the index of the first 0 in `self` from the `[start..=end]` checked range.
            ///
            /// Returns `None` if there are no bits unset.
            ///
            /// The index is relative to the entire sequence of `self`, not to the given `start`.
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS` and
            /// [`MismatchedIndices`] if `start > end`.
            pub const fn find_first_zero_checked_range(self, start: u32, end: u32)
                -> Result<Option<u32>, MismatchedBounds> {
                match Self::mask_checked_range(start, end) {
                    Ok(mask) => {
                        let masked_bits = !(self.0 & mask.0);
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

            /// Finds the index of the last 1 in `self` from the `[start..=end]` range.
            ///
            /// Returns `None` if there are no bits set.
            ///
            /// The index is relative to the entire sequence of `self`, not to the given `start`.
            /// # Panics
            /// Panics if `start >= BITS || end >= BITS || start > end`.
            #[must_use]
            pub const fn find_last_one_range(self, start: u32, end: u32) -> Option<u32> {
                let masked_bits = self.0 & Self::mask_range(start, end).0;
                let mut idx = end;
                loop {
                    iif![(masked_bits & (1 << idx)) != 0; return Some(idx)];
                    iif![idx == start; break];
                    idx -= 1;
                }
                None
            }

            /// Finds the index of the last 1 in `self` from the `[start..=end]` checked range.
            ///
            /// Returns `None` if there are no bits set.
            ///
            /// The index is relative to the entire sequence of `self`, not to the given `start`.
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS` and
            /// [`MismatchedIndices`] if `start > end`.
            pub const fn find_last_one_checked_range(self, start: u32, end: u32)
                -> Result<Option<u32>, MismatchedBounds> {
                match Self::mask_checked_range(start, end) {
                    Ok(mask) => {
                        let masked_bits = self.0 & mask.0;
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

            /// Finds the index of the last 0 in `self` from the `[start..=end]` range.
            ///
            /// Returns `None` if there are no bits set.
            ///
            /// The index is relative to the entire sequence of `self`, not to the given `start`.
            /// # Panics
            /// Panics if `start >= BITS || end >= BITS || start > end`.
            #[must_use]
            pub const fn find_last_zero_range(self, start: u32, end: u32) -> Option<u32> {
                let masked_bits = !(self.0 & Self::mask_range(start, end).0);
                let mut idx = end;
                loop {
                    iif![(masked_bits & (1 << idx)) != 0; return Some(idx)];
                    iif![idx == start; break];
                    idx -= 1;
                }
                None
            }

            /// Finds the index of the last 0 in `self` from the `[start..=end]` checked range.
            ///
            /// Returns `None` if there are no bits set.
            ///
            /// The index is relative to the entire sequence of `self`, not to the given `start`.
            /// # Errors
            /// Returns [`IndexOutOfBounds`] if `start >= BITS || end >= BITS` and
            /// [`MismatchedIndices`] if `start > end`.
            pub const fn find_last_zero_checked_range(self, start: u32, end: u32)
                -> Result<Option<u32>, MismatchedBounds> {
                match Self::mask_checked_range(start, end) {
                    Ok(mask) => {
                        let masked_bits = !(self.0 & mask.0);
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
        }
    };
}
impl_bits_wrapper![];
