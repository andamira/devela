// devela::num::fin::bit::ops::wise::impls
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

// helper for the methods examples
macro_rules! _example {
    (%open) => { concat!["\n# Examples\n", "```\n", "# use devela::Bitwise;\n"] };
    (%close) => { "\n```\n" };
    // raw body: useful for static methods.
    ($body:expr) => { concat![_example!(%open), $body, _example!(%close)] };
    // default self-value arm: declares `b`.
    ($t:ty, $value:expr; $body:expr) => { _example![b: $t = $value; $body] };
    // named self-value arm: useful when `b` is not clear enough.
    ($name:ident : $t:ty = $value:expr; $body:expr) => {
        concat![_example!(%open), "let ", stringify!($name), " = Bitwise::<",
        stringify!($t), ">(", stringify!($value), ");\n", $body, _example!(%close)]
    };
}

macro_rules! _num_fin_bit_ops_wise_impl_prims {
    () => { _num_fin_bit_ops_wise_impl_prims![
        u8, u16, u32, u64, u128, usize
    ]; };
    ( $( $t:ty),+ ) => { $( _num_fin_bit_ops_wise_impl_prims![@$t]; )+ };

    // `$t`: the primitive type
    (@$t:ty) => {
        /* impl traits */

        #[doc = concat!["# Implementations for `", stringify!($t), "`."]]
        #[doc = concat!["# Constants and mask constructors for `", stringify!($t), "`."]]
        impl Bitwise::<$t> {
            /* constants */

            /// The size in bits.
            pub const BITS: u32 = <$t>::BITS;

            /* mask constructors */

            #[doc = _DOC_BIT_MASK_RANGE!()]
            #[doc = _example!(concat!(
                "assert_eq![0b0001_1110, Bitwise::<", stringify!($t), ">::mask_range(1, 4).0]"))]
            #[doc = include_str!("../_benches/mask_range.md")]
            pub const fn mask_range(start: u32, end: u32) -> Self {
                debug_assert![start <= end];
                // a mask with all bits set, from 0 to end:
                let mask_end = is![end == <$t>::BITS -1, !0, (1 << (end + 1)) - 1];
                // a mask with all bits set from 0 to start - 1:
                let mask_start = is![start == 0, 0, (1 << start) - 1];
                Self(mask_end - mask_start)
            }
            #[doc = _DOC_BIT_MASK_RANGE_CHECKED!()]
            #[doc = _example!(concat!("assert_eq![0b0001_1110, Bitwise::<", stringify!($t),
                ">::mask_range_checked(1, 4).unwrap().0];"))]
            #[doc = include_str!("../_benches/mask_range_checked.md")]
            pub const fn mask_range_checked(start: u32, end: u32)
                -> Result<Self, MismatchedBounds> {
                if start >= <$t>::BITS {
                    Err(IndexOutOfBounds(Some(start as usize)))
                } else if end >= <$t>::BITS {
                    Err(IndexOutOfBounds(Some(end as usize)))
                } else if start > end {
                    Err(MismatchedIndices)
                } else {
                    // create a mask with all bits set, from 0 to end:
                    let mask_end = is![end == <$t>::BITS -1, !0, (1 << (end + 1)) - 1];
                    // create a mask with all bits set from 0 to start - 1:
                    let mask_start = is![start == 0, 0, (1 << start) - 1];
                    Ok(Self(mask_end - mask_start))
                }
            }

            #[must_use]
            #[doc = _DOC_BIT_IS_SET_MASK!()]
            #[doc = _example!($t, 0b1010_0000; "assert![b.is_set_mask(0b1000_0000)];")]
            pub const fn is_set_mask(self, mask: $t) -> bool { (self.0 & mask) != 0 }

            #[doc = _DOC_BIT_SET_MASK!()]
            #[doc = _example!($t, 0b1010_0000;
                "assert_eq![0b1010_1111, b.set_mask(0b0000_1111).0];")]
            pub const fn set_mask(self, mask: $t) -> Self { Self(self.0 | mask) }

            #[must_use]
            #[doc = _DOC_BIT_IS_UNSET_MASK!()]
            #[doc = _example!($t, 0b1010_0000; "assert![b.is_unset_mask(0b0100_0000)];")]
            pub const fn is_unset_mask(self, mask: $t) -> bool { (self.0 & mask) == 0 }

            #[doc = _DOC_BIT_UNSET_MASK!()]
            #[doc = _example!($t, 0b1010_0000;
                "assert_eq![0b0010_0000, b.unset_mask(0b1000_0000).0];")]
            pub const fn unset_mask(self, mask: $t) -> Self { Self(self.0 & !mask) }
        }
        #[doc = concat!["# Get methods for `", stringify!($t), "`."]]
        impl Bitwise::<$t> {
            /* get */

            #[doc = _DOC_BIT_GET_RANGE!()]
            #[doc = _example!($t, 0b1011_0110; "assert_eq![0b0001_0110, b.get_range(1, 4).0];")]
            pub const fn get_range(self, start: u32, end: u32) -> Self {
                Self(self.0 & Self::mask_range(start, end).0)
            }
            #[doc = _DOC_BIT_GET_RANGE_CHECKED!()]
            #[doc = _example!($t, 0b1011_0110;
                "assert_eq![0b0001_0110, b.get_range_checked(1, 4).unwrap().0];")]
            pub const fn get_range_checked(self, start: u32, end: u32)
                -> Result<Self, MismatchedBounds> {
                match Self::mask_range_checked(start, end) {
                    Ok(mask) => Ok(Self(self.0 & mask.0)),
                    Err(e) => Err(e),
                }
            }

            /* get value */

            #[doc = _DOC_BIT_GET_VALUE_RANGE!()]
            #[doc = _example!($t, 0b1011_0110; "assert_eq![0b1011, b.get_value_range(1, 4).0];")]
            pub const fn get_value_range(self, start: u32, end: u32) -> Self {
                Self((self.0 & Self::mask_range(start, end).0) >> start)
            }
            #[doc = _DOC_BIT_GET_VALUE_RANGE_CHECKED!()]
            #[doc = _example!($t, 0b1011_0110;
                "assert_eq![0b1011, b.get_value_range_checked(1, 4).unwrap().0];")]
            pub const fn get_value_range_checked(self, start: u32, end: u32)
                -> Result<Self, MismatchedBounds> {
                match Self::mask_range_checked(start, end) {
                    Ok(mask) => Ok(Self((self.0 & mask.0) >> start)),
                    Err(e) => Err(e),
                }
            }
        }
        #[doc = concat!["# Set ops for `", stringify!($t), "`."]]
        impl Bitwise::<$t> {
            /* set */

            #[doc = _DOC_BIT_IS_SET!()]
            #[doc = _example!($t, 0b1000_0000; "assert![b.is_set(7)];")]
            pub const fn is_set(self, nth: u32) -> bool {
                (self.0 & (1 << nth)) != 0
            }
            #[doc = _DOC_BIT_IS_SET_CHECKED!()]
            #[doc = _example!($t, 0b1000_0000; "assert![b.is_set_checked(7).unwrap()];")]
            pub const fn is_set_checked(self, nth: u32) -> Result<bool, MismatchedBounds> {
                if nth >= Self::BITS { Err(IndexOutOfBounds(Some(nth as usize))) }
                else { Ok((self.0 & (1 << nth)) != 0) }
            }

            #[doc = _DOC_BIT_SET!()]
            #[doc = _example!($t, 0; "assert_eq![0b0000_1000, b.set(3).0];")]
            pub const fn set(self, nth: u32) -> Self {
                Self(self.0 | 1 << nth)
            }
            #[doc = _DOC_BIT_SET_CHECKED!()]
            #[doc = _example!($t, 0; "assert_eq![0b0000_1000, b.set_checked(3).unwrap().0];")]
            pub const fn set_checked(self, nth: u32) -> Result<Self, MismatchedBounds> {
                if nth >= Self::BITS { Err(IndexOutOfBounds(Some(nth as usize))) }
                else { Ok(self.set(nth)) }
            }

            #[doc = _DOC_BIT_IS_SET_RANGE!()]
            #[doc = _example!($t, 0b0011_1000; "assert![b.is_set_range(3, 5)];")]
            pub const fn is_set_range(self, start: u32, end: u32) -> bool {
                self.get_range(start, end).0 == Self::mask_range(start, end).0
            }
            #[doc = _DOC_BIT_IS_SET_RANGE_CHECKED!()]
            #[doc = _example!($t, 0b0011_1000; "assert![b.is_set_range_checked(3, 5).unwrap()];")]
            pub const fn is_set_range_checked(self, start: u32, end: u32,)
                -> Result<bool, MismatchedBounds> {
                match Self::mask_range_checked(start, end) {
                    Ok(mask) => Ok((self.0 & mask.0) == mask.0),
                    Err(e) => Err(e),
                }
            }

            #[doc = _DOC_BIT_SET_RANGE!()]
            #[doc = _example!($t, 0b1000_0001; "assert_eq![0b1001_1111, b.set_range(1, 4).0];")]
            pub const fn set_range(self, start: u32, end: u32) -> Self {
                Self(self.0 | Self::mask_range(start, end).0)
            }
            #[doc = _DOC_BIT_SET_RANGE_CHECKED!()]
            #[doc = _example!($t, 0b1000_0001;
                "assert_eq![0b1001_1111, b.set_range_checked(1, 4).unwrap().0];")]
            pub const fn set_range_checked(self, start: u32, end: u32)
                -> Result<Self, MismatchedBounds> {
                match Self::mask_range_checked(start, end) {
                    Ok(mask) => Ok(Self(self.0 | mask.0)),
                    Err(e) => Err(e),
                }
            }

            #[doc = _DOC_BIT_SET_ALL!()]
            #[doc = _example!(concat!("let b = Bitwise::<", stringify!($t), ">(0);\n",
                "assert_eq![!0 as ", stringify!($t), ", b.set_all().0];"))]
            pub const fn set_all(self) -> Self { Self(!0) }

            /* set value */

            #[doc = _DOC_BIT_SET_VALUE_RANGE!()]
            #[doc = _example!($t, 0b1000_0001;
                "assert_eq![0b1000_1011, b.set_value_range(0b101, 1, 3).0];")]
            pub const fn set_value_range(self, value: $t, start: u32, end: u32) -> Self {
                let mask = Self::mask_range(start, end).0;
                let value_shifted = (value << start) & mask;
                Self((self.0 & !mask) | value_shifted)
            }
            #[doc = _DOC_BIT_SET_VALUE_RANGE_CHECKED!()]
            #[doc = _example!($t, 0b1000_0001;
                "assert_eq![0b1000_1011, b.set_value_range_checked(0b101, 1, 3).unwrap().0];")]
            pub const fn set_value_range_checked(self, value: $t, start: u32, end: u32)
                -> Result<Self, MismatchedBounds> {
                match Self::mask_range_checked(start, end) {
                    Ok(mask) => {
                        let value_shifted = (value << start) & mask.0;
                        Ok(Self((self.0 & !mask.0) | value_shifted))
                    },
                    Err(e) => Err(e),
                }
            }
            #[doc = _DOC_BIT_SET_VALUE_RANGE_CHECKED_STRICT!()]
            #[doc = _example!($t, 0;
                "assert![b.set_value_range_checked_strict(0b100, 0, 1).is_err()];")]
            pub const fn set_value_range_checked_strict(self, value: $t, start: u32, end: u32)
                -> Result<Self, MismatchedBounds> {
                match Self::mask_range_checked(start, end) {
                    Ok(mask) => {
                        let width = end - start + 1;
                        let limit = is! { width == <$t>::BITS, None, Some(1 as $t << width) };
                        if let Some(limit) = limit {
                            if value >= limit {
                                let err = crate::MismatchedCapacity
                                    ::too_large(value as usize, limit as usize);
                                return Err(MismatchedBounds::from_mismatched_capacity(err));
                            }
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
            #[doc = _example!($t, 0b1010_0000; "assert![b.is_unset(6)];")]
            pub const fn is_unset(self, nth: u32) -> bool {
                (self.0 & (1 << nth)) == 0
            }
            #[doc = _DOC_BIT_IS_UNSET_CHECKED!()]
            #[doc = _example!($t, 0b1010_0000; "assert![b.is_unset_checked(6).unwrap()];")]
            pub const fn is_unset_checked(self, nth: u32) -> Result<bool, MismatchedBounds> {
                if nth >= Self::BITS { Err(IndexOutOfBounds(Some(nth as usize))) }
                else { Ok((self.0 & (1 << nth)) == 0) }
            }

            #[doc = _DOC_BIT_UNSET!()]
            #[doc = _example!($t, 0b1010_0000; "assert_eq![0b0010_0000, b.unset(7).0];")]
            pub const fn unset(self, nth: u32) -> Self {
                Self(self.0 & !(1 << nth))
            }
            #[doc = _DOC_BIT_UNSET_CHECKED!()]
            #[doc = _example!($t, 0b1010_0000;
                "assert_eq![0b0010_0000, b.unset_checked(7).unwrap().0];")]
            pub const fn unset_checked(self, nth: u32) -> Result<Self, MismatchedBounds> {
                if nth >= Self::BITS { Err(IndexOutOfBounds(Some(nth as usize))) }
                else { Ok(self.unset(nth)) }
            }

            #[doc = _DOC_BIT_IS_UNSET_RANGE!()]
            #[doc = _example!($t, 0b1000_0001; "assert![b.is_unset_range(1, 6)];")]
            pub const fn is_unset_range(self, start: u32, end: u32) -> bool {
                self.get_range(start, end).0 == 0
            }
            #[doc = _DOC_BIT_IS_UNSET_RANGE_CHECKED!()]
            #[doc = _example!($t, 0b1000_0001; "assert![b.is_unset_range_checked(1, 6).unwrap()];")]
            pub const fn is_unset_range_checked(self, start: u32, end: u32)
                -> Result<bool, MismatchedBounds> {
                match Self::mask_range_checked(start, end) {
                    Ok(mask) => Ok((self.0 & mask.0) == 0),
                    Err(e) => Err(e),
                }
            }

            #[doc = _DOC_BIT_UNSET_RANGE!()]
            #[doc = _example!($t, 0b1111_1111; "assert_eq![0b1110_0001, b.unset_range(1, 4).0];")]
            pub const fn unset_range(self, start: u32, end: u32) -> Self {
                Self(self.0 & !Self::mask_range(start, end).0)
            }
            #[doc = _DOC_BIT_UNSET_RANGE_CHECKED!()]
            #[doc = _example!($t, 0b1111_1111;
                "assert_eq![0b1110_0001, b.unset_range_checked(1, 4).unwrap().0];")]
            pub const fn unset_range_checked(self, start: u32, end: u32)
                -> Result<Self, MismatchedBounds> {
                match Self::mask_range_checked(start, end) {
                    Ok(mask) => Ok(Self(self.0 & !mask.0)),
                    Err(e) => Err(e),
                }
            }

            #[doc = _DOC_BIT_UNSET_ALL!()]
            #[doc = _example!($t, 0b1111_1111; "assert_eq![0, b.unset_all().0];")]
            pub const fn unset_all(self) -> Self { Self(0) }
        }
        #[doc = concat!["# Flip ops for `", stringify!($t), "`."]]
        impl Bitwise::<$t> {
            /* flip */

            #[doc = _DOC_BIT_FLIP!()]
            #[doc = _example!($t, 0b0000_0001; "assert_eq![0, b.flip(0).0];")]
            pub const fn flip(self, nth: u32) -> Self {
                Self(self.0 ^ (1 << nth))
            }
            #[doc = _DOC_BIT_FLIP_CHECKED!()]
            #[doc = _example!($t, 0b0000_0001; "assert_eq![0, b.flip_checked(0).unwrap().0];")]
            pub const fn flip_checked(self, nth: u32) -> Result<Self, MismatchedBounds> {
                if nth >= Self::BITS { Err(IndexOutOfBounds(Some(nth as usize))) }
                else { Ok(self.flip(nth)) }
            }

            #[doc = _DOC_BIT_FLIP_RANGE!()]
            #[doc = _example!($t, 0b1010_0000; "assert_eq![0b0101_0000, b.flip_range(4, 7).0];")]
            pub const fn flip_range(self, start: u32, end: u32) -> Self {
                Self(self.0 ^ Self::mask_range(start, end).0)
            }

            #[doc = _DOC_BIT_FLIP_RANGE_CHECKED!()]
            #[doc = _example!($t, 0b1010_0000;
                "assert_eq![0b0101_0000, b.flip_range_checked(4, 7).unwrap().0];")]
            pub const fn flip_range_checked(self, start: u32, end: u32)
                -> Result<Self, MismatchedBounds> {
                match Self::mask_range_checked(start, end) {
                    Ok(mask) => Ok(Self(self.0 ^ mask.0)),
                    Err(e) => Err(e),
                }
            }

            #[doc = _DOC_BIT_FLIP_RANGE_IF!()]
            #[doc = _example!($t, 0b1010_0000;
                "assert_eq![0b1010_0000, b.flip_range_if(4, 7, false).0];")]
            pub const fn flip_range_if(self, start: u32, end: u32, cond: bool) -> Self {
                if cond { self.flip_range(start, end) } else { self }
            }
            #[doc = _DOC_BIT_FLIP_RANGE_CHECKED_IF!()]
            #[doc = _example!($t, 0b1010_0000;
                "assert_eq![0b1010_0000, b.flip_range_if_checked(4, 7, false).unwrap().0];")]
            pub const fn flip_range_if_checked(self, start: u32, end: u32, cond: bool)
                -> Result<Self, MismatchedBounds> {
                if cond { self.flip_range_checked(start, end) } else { Ok(self) }
            }
        }
        #[doc = concat!["# Reverse ops for `", stringify!($t), "`."]]
        impl Bitwise::<$t> {
            /* reverse */

            #[doc = _DOC_BIT_REVERSE_RANGE!()]
            #[doc = _example!($t, 0b0001_0110; "assert_eq![0b0001_1010, b.reverse_range(1, 4).0];")]
            pub const fn reverse_range(self, start: u32, end: u32) -> Self {
                debug_assert![start <= end];
                // If the entire range of bits is selected, simply reverse all bits
                let range_bits = end - start + 1;
                is![range_bits == Self::BITS, return Self(self.0.reverse_bits())];
                // Create the mask for the range and reverse its bits
                let mask = (((1 as $t) << range_bits) - 1) << start;
                let bits_to_rev = (self.0 & mask) >> start;
                let rev = bits_to_rev.reverse_bits();
                // Shift the reversed bits back to their original position
                let rev_shifted = (rev >> (Self::BITS - range_bits)) << start;
                // Combine with the original number, preserving bits outside the range
                Self((self.0 & !mask) | rev_shifted)
            }
            #[doc = _DOC_BIT_REVERSE_RANGE_CHECKED!()]
            #[doc = _example!($t, 0b0001_0110;
                "assert_eq![0b0001_1010, b.reverse_range_checked(1, 4).unwrap().0];")]
            pub const fn reverse_range_checked(self, start: u32, end: u32)
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
                    is![range_bits == Self::BITS, return Ok(Self(self.0.reverse_bits()))];
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
            #[doc = _example!($t, 0b1011_0110; "assert_eq![3, b.count_ones_range(1, 4)];")]
            pub const fn count_ones_range(self, start: u32, end: u32) -> u32 {
                let masked_bits = self.0 & Self::mask_range(start, end).0;
                masked_bits.count_ones()
            }
            #[doc = _DOC_BIT_COUNT_ONES_RANGE_CHECKED!()]
            #[doc = _example!($t, 0b1011_0110;
                "assert_eq![3, b.count_ones_range_checked(1, 4).unwrap()];")]
            pub const fn count_ones_range_checked(self, start: u32, end: u32)
                -> Result<u32, MismatchedBounds> {
                match Self::mask_range_checked(start, end) {
                    Ok(mask) => Ok((self.0 & mask.0).count_ones()),
                    Err(e) => Err(e),
                }
            }

            #[must_use]
            #[doc = _DOC_BIT_COUNT_ZEROS_RANGE!()]
            #[doc = _example!($t, 0b1011_0110; "assert_eq![1, b.count_zeros_range(1, 4)];")]
            pub const fn count_zeros_range(self, start: u32, end: u32) -> u32 {
                let mask = Self::mask_range(start, end).0;
                let masked_bits = self.0 & mask;
                (!masked_bits & mask).count_ones()
            }
            #[doc = _DOC_BIT_COUNT_ZEROS_RANGE_CHECKED!()]
            #[doc = _example!($t, 0b1011_0110;
                "assert_eq![1, b.count_zeros_range_checked(1, 4).unwrap()];")]
            pub const fn count_zeros_range_checked(self, start: u32, end: u32)
                -> Result<u32, MismatchedBounds> {
                match Self::mask_range_checked(start, end) {
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
            #[doc = _example!($t, 0b1011_0000;
                "assert_eq![Some(4), b.find_first_one_range(0, 7)];")]
            pub const fn find_first_one_range(self, start: u32, end: u32) -> Option<u32> {
                let masked_bits = self.0 & Self::mask_range(start, end).0;
                let mut idx = start;
                while idx <= end {
                    is![(masked_bits & (1 << idx)) != 0, return Some(idx)];
                    idx += 1;
                }
                None
            }
            #[doc = _DOC_BIT_FIND_FIRST_ONE_RANGE_CHECKED!()]
            #[doc = _example!($t, 0b1011_0000;
                "assert_eq![Some(4), b.find_first_one_range_checked(0, 7).unwrap()];")]
            pub const fn find_first_one_range_checked(self, start: u32, end: u32)
                -> Result<Option<u32>, MismatchedBounds> {
                match Self::mask_range_checked(start, end) {
                    Ok(mask) => {
                        let masked_bits = self.0 & mask.0;
                        let mut idx = start;
                        while idx <= end {
                            is![(masked_bits & (1 << idx)) != 0, return Ok(Some(idx))];
                            idx += 1;
                        }
                        Ok(None)
                    },
                    Err(e) => Err(e),
                }
            }

            #[must_use]
            #[doc = _DOC_BIT_FIND_FIRST_ZERO_RANGE!()]
            #[doc = _example!($t, 0b1111_1011;
                "assert_eq![Some(2), b.find_first_zero_range(0, 7)];")]
            pub const fn find_first_zero_range(self, start: u32, end: u32)
                -> Option<u32> {
                let masked_bits = !(self.0 & Self::mask_range(start, end).0);
                let mut idx = start;
                while idx <= end {
                    is![(masked_bits & (1 << idx)) != 0, return Some(idx)];
                    idx += 1;
                }
                None
            }
            #[doc = _DOC_BIT_FIND_FIRST_ZERO_RANGE_CHECKED!()]
            #[doc = _example!($t, 0b1111_1011;
                "assert_eq![Some(2), b.find_first_zero_range_checked(0, 7).unwrap()];")]
            pub const fn find_first_zero_range_checked(self, start: u32, end: u32)
                -> Result<Option<u32>, MismatchedBounds> {
                match Self::mask_range_checked(start, end) {
                    Ok(mask) => {
                        let masked_bits = !(self.0 & mask.0);
                        let mut idx = start;
                        while idx <= end {
                            is![(masked_bits & (1 << idx)) != 0, return Ok(Some(idx))];
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
            #[doc = _example!($t, 0b0001_0110; "assert_eq![Some(4), b.find_last_one_range(0, 7)];")]
            pub const fn find_last_one_range(self, start: u32, end: u32) -> Option<u32> {
                let masked_bits = self.0 & Self::mask_range(start, end).0;
                let mut idx = end;
                loop {
                    is![(masked_bits & (1 << idx)) != 0, return Some(idx)];
                    is![idx == start, break];
                    idx -= 1;
                }
                None
            }

            #[doc = _DOC_BIT_FIND_LAST_ONE_RANGE_CHECKED!()]
            #[doc = _example!($t, 0b0001_0110;
                "assert_eq![Some(4), b.find_last_one_range_checked(0, 7).unwrap()];")]
            pub const fn find_last_one_range_checked(self, start: u32, end: u32)
                -> Result<Option<u32>, MismatchedBounds> {
                match Self::mask_range_checked(start, end) {
                    Ok(mask) => {
                        let masked_bits = self.0 & mask.0;
                        let mut idx = end;
                        loop {
                            is![(masked_bits & (1 << idx)) != 0, return Ok(Some(idx))];
                            is![idx == start, break];
                            idx -= 1;
                        }
                        Ok(None)
                    },
                    Err(e) => Err(e),
                }
            }

            #[must_use]
            #[doc = _DOC_BIT_FIND_LAST_ZERO_RANGE!()]
            #[doc = _example!($t, 0b1110_1111;
                "assert_eq![Some(4), b.find_last_zero_range(0, 7)];")]
            pub const fn find_last_zero_range(self, start: u32, end: u32) -> Option<u32> {
                let masked_bits = !(self.0 & Self::mask_range(start, end).0);
                let mut idx = end;
                loop {
                    is![(masked_bits & (1 << idx)) != 0, return Some(idx)];
                    is![idx == start, break];
                    idx -= 1;
                }
                None
            }
            #[doc = _DOC_BIT_FIND_LAST_ZERO_RANGE_CHECKED!()]
            #[doc = _example!($t, 0b1110_1111;
                "assert_eq![Some(4), b.find_last_zero_range_checked(0, 7).unwrap()];")]
            pub const fn find_last_zero_range_checked(self, start: u32, end: u32)
                -> Result<Option<u32>, MismatchedBounds> {
                match Self::mask_range_checked(start, end) {
                    Ok(mask) => {
                        let masked_bits = !(self.0 & mask.0);
                        let mut idx = end;
                        loop {
                            is![(masked_bits & (1 << idx)) != 0, return Ok(Some(idx))];
                            is![idx == start, break];
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
_num_fin_bit_ops_wise_impl_prims![];
