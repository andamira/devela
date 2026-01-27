// devela_base_core::num::fin::bit::ops::wise::impls
//
//! Implements `Bitwise` for the integer primitives.
//

use super::super::_docs::*;
#[allow(unused_imports, reason = "MismatchedCapacity only used for docs")]
use crate::{
    Bitwise,
    MismatchedBounds::{self, IndexOutOfBounds, MismatchedCapacity, MismatchedIndices},
    is,
};

macro_rules! impl_bits_wrapper {
    () => { impl_bits_wrapper![
        u8, u16, u32, u64, u128, usize
    ]; };
    ( $( $t:ty),+ ) => { $( impl_bits_wrapper![@$t]; )+ };

    // `$t`: the primitive type
    (@$t:ty) => {
        /* impl traits */

        #[doc = concat!["# Implementations for `", stringify!($t), "`."]]
        /// ---
        /// TOC
        /// - constants + mask constructors
        /// - single-bit \[get|set\], \[get|set\]_range, \[get|set\]_value_range…
        /// - unset\[_range\]…
        /// - flip, flip_range…
        /// - reverse_range…
        /// - count_ones_range, count_zeros_range…
        /// - find_first_one, find_last_one…
        impl Bitwise::<$t> {
            /* constants */

            /// The size in bits.
            pub const BITS: u32 = <$t>::BITS;

            /* mask constructors */

            #[doc = _DOC_BIT_MASK_RANGE!()]
            #[doc = include_str!("../_benches/mask_range.md")]
            pub const fn mask_range(start: u32, end: u32) -> Self {
                debug_assert![start <= end];
                // a mask with all bits set, from 0 to end:
                let mask_end = is![end == <$t>::BITS -1; !0; (1 << (end + 1)) - 1];
                // a mask with all bits set from 0 to start - 1:
                let mask_start = is![start == 0; 0; (1 << start) - 1];
                Self(mask_end - mask_start)
            }
            #[doc = _DOC_BIT_MASK_RANGE_CHECKED!()]
            #[doc = include_str!("../_benches/mask_checked_range.md")]
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
                    let mask_end = is![end == <$t>::BITS -1; !0; (1 << (end + 1)) - 1];
                    // create a mask with all bits set from 0 to start - 1:
                    let mask_start = is![start == 0; 0; (1 << start) - 1];
                    Ok(Self(mask_end - mask_start))
                }
            }

            #[must_use]
            #[doc = _DOC_BIT_IS_SET_MASK!()]
            pub const fn is_set_mask(self, mask: $t) -> bool { (self.0 & mask) != 0 }

            #[doc = _DOC_BIT_SET_MASK!()]
            pub const fn set_mask(self, mask: $t) -> Self { Self(self.0 | mask) }

            #[must_use]
            #[doc = _DOC_BIT_IS_UNSET_MASK!()]
            pub const fn is_unset_mask(self, mask: $t) -> bool { (self.0 & mask) == 0 }

            #[doc = _DOC_BIT_UNSET_MASK!()]
            pub const fn unset_mask(self, mask: $t) -> Self { Self(self.0 & !mask) }
        }
        #[doc = concat!["# Get methods for `", stringify!($t), "`."]]
        impl Bitwise::<$t> {
            /* get */

            #[doc = _DOC_BIT_GET_RANGE!()]
            pub const fn get_range(self, start: u32, end: u32) -> Self {
                Self(self.0 & Self::mask_range(start, end).0)
            }
            #[doc = _DOC_BIT_GET_CHECKED_RANGE!()]
            pub const fn get_checked_range(self, start: u32, end: u32)
                -> Result<Self, MismatchedBounds> {
                match Self::mask_checked_range(start, end) {
                    Ok(mask) => Ok(Self(self.0 & mask.0)),
                    Err(e) => Err(e),
                }
            }

            /* get value */

            #[doc = _DOC_BIT_GET_VALUE_RANGE!()]
            pub const fn get_value_range(self, start: u32, end: u32) -> Self {
                Self((self.0 & Self::mask_range(start, end).0) >> start)
            }
            #[doc = _DOC_BIT_GET_VALUE_CHECKED_RANGE!()]
            pub const fn get_value_checked_range(self, start: u32, end: u32)
                -> Result<Self, MismatchedBounds> {
                match Self::mask_checked_range(start, end) {
                    Ok(mask) => Ok(Self((self.0 & mask.0) >> start)),
                    Err(e) => Err(e),
                }
            }
        }
        #[doc = concat!["# Set ops for `", stringify!($t), "`."]]
        impl Bitwise::<$t> {
            /* set */

            #[doc = _DOC_BIT_IS_SET!()]
            pub const fn is_set(self, nth: u32) -> bool {
                (self.0 & (1 << nth)) != 0
            }
            #[doc = _DOC_BIT_IS_SET_CHECKED!()]
            pub const fn is_set_checked(self, nth: u32) -> Result<bool, MismatchedBounds> {
                if nth >= Self::BITS { Err(IndexOutOfBounds(Some(nth as usize))) }
                else { Ok((self.0 & (1 << nth)) != 0) }
            }

            #[doc = _DOC_BIT_SET!()]
            pub const fn set(self, nth: u32) -> Self {
                Self(self.0 | 1 << nth)
            }
            #[doc = _DOC_BIT_SET_CHECKED!()]
            pub const fn set_checked(self, nth: u32) -> Result<Self, MismatchedBounds> {
                if nth >= Self::BITS { Err(IndexOutOfBounds(Some(nth as usize))) }
                else { Ok(self.set(nth)) }
            }

            #[doc = _DOC_BIT_IS_SET_RANGE!()]
            pub const fn is_set_range(self, start: u32, end: u32) -> bool {
                self.get_range(start, end).0 == Self::mask_range(start, end).0
            }
            #[doc = _DOC_BIT_IS_SET_CHECKED_RANGE!()]
            pub const fn is_set_checked_range(self, start: u32, end: u32,)
                -> Result<bool, MismatchedBounds> {
                match Self::mask_checked_range(start, end) {
                    Ok(mask) => Ok((self.0 & mask.0) == mask.0),
                    Err(e) => Err(e),
                }
            }

            #[doc = _DOC_BIT_SET_RANGE!()]
            pub const fn set_range(self, start: u32, end: u32) -> Self {
                Self(self.0 | Self::mask_range(start, end).0)
            }
            #[doc = _DOC_BIT_SET_CHECKED_RANGE!()]
            pub const fn set_checked_range(self, start: u32, end: u32)
                -> Result<Self, MismatchedBounds> {
                match Self::mask_checked_range(start, end) {
                    Ok(mask) => Ok(Self(self.0 | mask.0)),
                    Err(e) => Err(e),
                }
            }

            #[doc = _DOC_BIT_SET_ALL!()]
            pub const fn set_all(self) -> Self { Self(!0) }

            /* set value */

            #[doc = _DOC_BIT_SET_VALUE_RANGE!()]
            pub const fn set_value_range(self, value: $t, start: u32, end: u32) -> Self {
                let mask = Self::mask_range(start, end).0;
                let value_shifted = (value << start) & mask;
                Self((self.0 & !mask) | value_shifted)
            }
            #[doc = _DOC_BIT_SET_VALUE_CHECKED_RANGE!()]
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
            #[doc = _DOC_BIT_SET_CHECKED_VALUE_CHECKED_RANGE!()]
            pub const fn set_checked_value_checked_range(self, value: $t, start: u32, end: u32)
                -> Result<Self, MismatchedBounds> {
                match Self::mask_checked_range(start, end) {
                    Ok(mask) => {
                        if value >= (1 << (end - start)) {
                            let err = crate::MismatchedCapacity
                                ::too_large(value as usize, ( 1 << (end - start)));
                            return Err(MismatchedBounds::from_mismatched_capacity(err));
                        }
                        let value_shifted = (value << start) & mask.0;
                        Ok(Self((self.0 & !mask.0) | value_shifted))
                    },
                    Err(e) => Err(e),
                }
            }
        }
        #[doc = concat!["# Unset ops for `", stringify!($t), "`."]]
        impl Bitwise::<$t> {
            /* unset */

            #[doc = _DOC_BIT_IS_UNSET!()]
            pub const fn is_unset(self, nth: u32) -> bool {
                (self.0 & (1 << nth)) == 0
            }
            #[doc = _DOC_BIT_IS_UNSET_CHECKED!()]
            pub const fn is_unset_checked(self, nth: u32) -> Result<bool, MismatchedBounds> {
                if nth >= Self::BITS { Err(IndexOutOfBounds(Some(nth as usize))) }
                else { Ok((self.0 & (1 << nth)) == 0) }
            }

            #[doc = _DOC_BIT_UNSET!()]
            pub const fn unset(self, nth: u32) -> Self {
                Self(self.0 & !(1 << nth))
            }
            #[doc = _DOC_BIT_UNSET_CHECKED!()]
            pub const fn unset_checked(self, nth: u32) -> Result<Self, MismatchedBounds> {
                if nth >= Self::BITS { Err(IndexOutOfBounds(Some(nth as usize))) }
                else { Ok(self.unset(nth)) }
            }

            #[doc = _DOC_BIT_IS_UNSET_RANGE!()]
            pub const fn is_unset_range(self, start: u32, end: u32) -> bool {
                self.get_range(start, end).0 == 0
            }
            #[doc = _DOC_BIT_IS_UNSET_CHECKED_RANGE!()]
            pub const fn is_unset_checked_range(self, start: u32, end: u32)
                -> Result<bool, MismatchedBounds> {
                match Self::mask_checked_range(start, end) {
                    Ok(mask) => Ok((self.0 & mask.0) == 0),
                    Err(e) => Err(e),
                }
            }

            #[doc = _DOC_BIT_UNSET_RANGE!()]
            pub const fn unset_range(self, start: u32, end: u32) -> Self {
                Self(self.0 & !Self::mask_range(start, end).0)
            }
            #[doc = _DOC_BIT_UNSET_CHECKED_RANGE!()]
            pub const fn unset_checked_range(self, start: u32, end: u32)
                -> Result<Self, MismatchedBounds> {
                match Self::mask_checked_range(start, end) {
                    Ok(mask) => Ok(Self(self.0 & !mask.0)),
                    Err(e) => Err(e),
                }
            }

            #[doc = _DOC_BIT_UNSET_ALL!()]
            pub const fn unset_all(self) -> Self { Self(0) }
        }
        #[doc = concat!["# Flip ops for `", stringify!($t), "`."]]
        impl Bitwise::<$t> {
            /* flip */

            #[doc = _DOC_BIT_FLIP!()]
            pub const fn flip(self, nth: u32) -> Self {
                Self(self.0 ^ (1 << nth))
            }
            #[doc = _DOC_BIT_FLIP_CHECKED!()]
            pub const fn flip_checked(self, nth: u32) -> Result<Self, MismatchedBounds> {
                if nth >= Self::BITS { Err(IndexOutOfBounds(Some(nth as usize))) }
                else { Ok(self.flip(nth)) }
            }

            #[doc = _DOC_BIT_FLIP_RANGE!()]
            pub const fn flip_range(self, start: u32, end: u32) -> Self {
                Self(self.0 ^ Self::mask_range(start, end).0)
            }

            #[doc = _DOC_BIT_FLIP_CHECKED_RANGE!()]
            pub const fn flip_checked_range(self, start: u32, end: u32)
                -> Result<Self, MismatchedBounds> {
                match Self::mask_checked_range(start, end) {
                    Ok(mask) => Ok(Self(self.0 ^ mask.0)),
                    Err(e) => Err(e),
                }
            }

            #[doc = _DOC_BIT_FLIP_RANGE_IF!()]
            pub const fn flip_range_if(self, start: u32, end: u32, cond: bool) -> Self {
                if cond { self.flip_range(start, end) } else { self }
            }
            #[doc = _DOC_BIT_FLIP_CHECKED_RANGE_IF!()]
            pub const fn flip_checked_range_if(self, start: u32, end: u32, cond: bool)
                -> Result<Self, MismatchedBounds> {
                if cond { self.flip_checked_range(start, end) } else { Ok(self) }
            }
        }
        #[doc = concat!["# Reverse ops for `", stringify!($t), "`."]]
        impl Bitwise::<$t> {
            /* reverse */

            #[doc = _DOC_BIT_REVERSE_RANGE!()]
            pub const fn reverse_range(self, start: u32, end: u32) -> Self {
                debug_assert![start <= end];
                // If the entire range of bits is selected, simply reverse all bits
                let range_bits = end - start + 1;
                is![range_bits == Self::BITS; return Self(self.0.reverse_bits())];
                // Create the mask for the range and reverse its bits
                let mask = (((1 as $t) << range_bits) - 1) << start;
                let bits_to_rev = (self.0 & mask) >> start;
                let rev = bits_to_rev.reverse_bits();
                // Shift the reversed bits back to their original position
                let rev_shifted = (rev >> (Self::BITS - range_bits)) << start;
                // Combine with the original number, preserving bits outside the range
                Self((self.0 & !mask) | rev_shifted)
            }

            #[doc = _DOC_BIT_REVERSE_CHECKED_RANGE!()]
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
                    is![range_bits == Self::BITS; return Ok(Self(self.0.reverse_bits()))];
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
        }
        #[doc = concat!["# Count ops for `", stringify!($t), "`."]]
        impl Bitwise::<$t> {
            /* count */

            #[must_use]
            #[doc = _DOC_BIT_COUNT_ONES_RANGE!()]
            pub const fn count_ones_range(self, start: u32, end: u32) -> u32 {
                let masked_bits = self.0 & Self::mask_range(start, end).0;
                masked_bits.count_ones()
            }
            #[doc = _DOC_BIT_COUNT_ONES_RANGE!()]
            pub const fn count_ones_checked_range(self, start: u32, end: u32)
                -> Result<u32, MismatchedBounds> {
                match Self::mask_checked_range(start, end) {
                    Ok(mask) => Ok((self.0 & mask.0).count_ones()),
                    Err(e) => Err(e),
                }
            }

            #[must_use]
            #[doc = _DOC_BIT_COUNT_ZEROS_RANGE!()]
            pub const fn count_zeros_range(self, start: u32, end: u32) -> u32 {
                let mask = Self::mask_range(start, end).0;
                let masked_bits = self.0 & mask;
                (!masked_bits & mask).count_ones()
            }
            #[doc = _DOC_BIT_COUNT_ZEROS_CHECKED_RANGE!()]
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
        }
        #[doc = concat!["# Find ops for `", stringify!($t), "`."]]
        impl Bitwise::<$t> {
            /* find first */

            #[must_use]
            #[doc = _DOC_BIT_FIND_FIRST_ONE_RANGE!()]
            pub const fn find_first_one_range(self, start: u32, end: u32) -> Option<u32> {
                let masked_bits = self.0 & Self::mask_range(start, end).0;
                let mut idx = start;
                while idx <= end {
                    is![(masked_bits & (1 << idx)) != 0; return Some(idx)];
                    idx += 1;
                }
                None
            }
            #[doc = _DOC_BIT_FIND_FIRST_ONE_CHECKED_RANGE!()]
            pub const fn find_first_one_checked_range(self, start: u32, end: u32)
                -> Result<Option<u32>, MismatchedBounds> {
                match Self::mask_checked_range(start, end) {
                    Ok(mask) => {
                        let masked_bits = self.0 & mask.0;
                        let mut idx = start;
                        while idx <= end {
                            is![(masked_bits & (1 << idx)) != 0; return Ok(Some(idx))];
                            idx += 1;
                        }
                        Ok(None)
                    },
                    Err(e) => Err(e),
                }
            }

            #[must_use]
            #[doc = _DOC_BIT_FIND_FIRST_ZERO_RANGE!()]
            pub const fn find_first_zero_range(self, start: u32, end: u32)
                -> Option<u32> {
                let masked_bits = !(self.0 & Self::mask_range(start, end).0);
                let mut idx = start;
                while idx <= end {
                    is![(masked_bits & (1 << idx)) != 0; return Some(idx)];
                    idx += 1;
                }
                None
            }
            #[doc = _DOC_BIT_FIND_FIRST_ZERO_CHECKED_RANGE!()]
            pub const fn find_first_zero_checked_range(self, start: u32, end: u32)
                -> Result<Option<u32>, MismatchedBounds> {
                match Self::mask_checked_range(start, end) {
                    Ok(mask) => {
                        let masked_bits = !(self.0 & mask.0);
                        let mut idx = start;
                        while idx <= end {
                            is![(masked_bits & (1 << idx)) != 0; return Ok(Some(idx))];
                            idx += 1;
                        }
                        Ok(None)
                    },
                    Err(e) => Err(e),
                }
            }

            /* find last */

            #[must_use]
            #[doc = _DOC_BIT_FIND_LAST_ONE_RANGE!()]
            pub const fn find_last_one_range(self, start: u32, end: u32) -> Option<u32> {
                let masked_bits = self.0 & Self::mask_range(start, end).0;
                let mut idx = end;
                loop {
                    is![(masked_bits & (1 << idx)) != 0; return Some(idx)];
                    is![idx == start; break];
                    idx -= 1;
                }
                None
            }

            #[doc = _DOC_BIT_FIND_LAST_ONE_CHECKED_RANGE!()]
            pub const fn find_last_one_checked_range(self, start: u32, end: u32)
                -> Result<Option<u32>, MismatchedBounds> {
                match Self::mask_checked_range(start, end) {
                    Ok(mask) => {
                        let masked_bits = self.0 & mask.0;
                        let mut idx = end;
                        loop {
                            is![(masked_bits & (1 << idx)) != 0; return Ok(Some(idx))];
                            is![idx == start; break];
                            idx -= 1;
                        }
                        Ok(None)
                    },
                    Err(e) => Err(e),
                }
            }

            #[must_use]
            #[doc = _DOC_BIT_FIND_LAST_ZERO_RANGE!()]
            pub const fn find_last_zero_range(self, start: u32, end: u32) -> Option<u32> {
                let masked_bits = !(self.0 & Self::mask_range(start, end).0);
                let mut idx = end;
                loop {
                    is![(masked_bits & (1 << idx)) != 0; return Some(idx)];
                    is![idx == start; break];
                    idx -= 1;
                }
                None
            }
            #[doc = _DOC_BIT_FIND_LAST_ZERO_CHECKED_RANGE!()]
            pub const fn find_last_zero_checked_range(self, start: u32, end: u32)
                -> Result<Option<u32>, MismatchedBounds> {
                match Self::mask_checked_range(start, end) {
                    Ok(mask) => {
                        let masked_bits = !(self.0 & mask.0);
                        let mut idx = end;
                        loop {
                            is![(masked_bits & (1 << idx)) != 0; return Ok(Some(idx))];
                            is![idx == start; break];
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
