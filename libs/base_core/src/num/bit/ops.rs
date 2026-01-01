// devela_base_core::num::bit::ops
//
//! Defines the [`BitOps`] trait.
//
// TOC
// - definition
// - impls

use super::_docs::*;
#[cfg(doc)]
use crate::MismatchedBounds::{DataOverflow, IndexOutOfBounds, MismatchedIndices};
use crate::{Bitwise, MismatchedBounds};

#[doc = crate::_TAG_NUM!()]
#[doc = crate::_TAG_NAMESPACE!()]
/// Provides bitwise operations on `T`.
#[doc = crate::_doc!(location: "num")]
///
/// See also [`Bitwise`] for the related const wrapper.
#[rustfmt::skip]
pub trait BitOps where Self: Sized {
    /// The inner type for the bit representation.
    type Inner;

    // const BITS: u32; // no need because the primitives already have this.

    /* mask */

    #[must_use]
    #[doc = _DOC_BIT_MASK_RANGE!()]
    #[doc = include_str!("_benches/mask_range.md")]
    fn bit_mask_range(start: u32, end: u32) -> Self;

    #[doc = _DOC_BIT_MASK_RANGE_CHECKED!()]
    #[doc = include_str!("_benches/mask_checked_range.md")]
    fn bit_mask_checked_range(start: u32, end: u32) -> Result<Self, MismatchedBounds>;

    #[must_use]
    #[doc = _DOC_BIT_IS_SET_MASK!()]
    fn bit_is_set_mask(self, mask: Self::Inner) -> bool;

    #[doc = _DOC_BIT_SET_MASK!()]
    fn bit_set_mask(self, mask: Self::Inner) -> Self;

    #[must_use]
    #[doc = _DOC_BIT_IS_UNSET_MASK!()]
    fn bit_is_unset_mask(self, mask: Self::Inner) -> bool;

    #[doc = _DOC_BIT_UNSET_MASK!()]
    fn bit_unset_mask(self, mask: Self::Inner) -> Self;

    /* get */

    #[must_use]
    #[doc = _DOC_BIT_GET_RANGE!()]
    fn bit_get_range(self, start: u32, end: u32) -> Self;
    #[doc = _DOC_BIT_GET_CHECKED_RANGE!()]
    fn bit_get_checked_range(self, start: u32, end: u32) -> Result<Self, MismatchedBounds>;

    /* get value */

    #[must_use]
    #[doc = _DOC_BIT_GET_VALUE_RANGE!()]
    fn bit_get_value_range(self, start: u32, end: u32) -> Self;
    #[doc = _DOC_BIT_GET_VALUE_CHECKED_RANGE!()]
    fn bit_get_value_checked_range(self, start: u32, end: u32) -> Result<Self, MismatchedBounds>;

    /* set */

    #[must_use]
    #[doc = _DOC_BIT_IS_SET!()]
    fn bit_is_set(self, nth: u32) -> bool;
    #[doc = _DOC_BIT_IS_SET_CHECKED!()]
    fn bit_is_set_checked(self, nth: u32) -> Result<bool, MismatchedBounds>;

    #[must_use]
    #[doc = _DOC_BIT_SET!()]
    fn bit_set(self, nth: u32) -> Self;
    #[doc = _DOC_BIT_SET_CHECKED!()]
    fn bit_set_checked(self, nth: u32) -> Result<Self, MismatchedBounds>;

    #[must_use]
    #[doc = _DOC_BIT_SET_RANGE!()]
    fn bit_is_set_range(self, start: u32, end: u32) -> bool;
    #[doc = _DOC_BIT_SET_CHECKED_RANGE!()]
    fn bit_is_set_checked_range(self, start: u32, end: u32) -> Result<bool, MismatchedBounds>;

    #[must_use]
    #[doc = _DOC_BIT_SET_RANGE!()]
    fn bit_set_range(self, start: u32, end: u32) -> Self;
    #[doc = _DOC_BIT_SET_CHECKED_RANGE!()]
    fn bit_set_checked_range(self, start: u32, end: u32) -> Result<Self, MismatchedBounds>;

    #[doc = _DOC_BIT_SET_ALL!()]
    fn bit_set_all(self) -> Self;

    /* set value */

    #[must_use]
    #[doc = _DOC_BIT_SET_VALUE_RANGE!()]
    fn bit_set_value_range(self, value: Self::Inner, start: u32, end: u32) -> Self;
    #[doc = _DOC_BIT_SET_VALUE_CHECKED_RANGE!()]
    fn bit_set_value_checked_range(self, value: Self::Inner, start: u32, end: u32)
        -> Result<Self, MismatchedBounds>;
    #[doc = _DOC_BIT_SET_CHECKED_VALUE_CHECKED_RANGE!()]
    fn bit_set_checked_value_checked_range(self, value: Self::Inner, start: u32, end: u32)
        -> Result<Self, MismatchedBounds>;

    /* unset */

    #[must_use]
    #[doc = _DOC_BIT_IS_UNSET!()]
    fn bit_is_unset(self, nth: u32) -> bool;
    #[doc = _DOC_BIT_IS_UNSET_CHECKED!()]
    fn bit_is_unset_checked(self, nth: u32) -> Result<bool, MismatchedBounds>;

    #[must_use]
    #[doc = _DOC_BIT_UNSET!()]
    fn bit_unset(self, nth: u32) -> Self;
    #[doc = _DOC_BIT_UNSET_CHECKED!()]
    fn bit_unset_checked(self, nth: u32) -> Result<Self, MismatchedBounds>;

    #[must_use]
    #[doc = _DOC_BIT_UNSET_RANGE!()]
    fn bit_is_unset_range(self, start: u32, end: u32) -> bool;
    #[doc = _DOC_BIT_UNSET_CHECKED_RANGE!()]
    fn bit_is_unset_checked_range(self, start: u32, end: u32) -> Result<bool, MismatchedBounds>;


    #[must_use]
    #[doc = _DOC_BIT_UNSET_RANGE!()]
    fn bit_unset_range(self, start: u32, end: u32) -> Self;
    #[doc = _DOC_BIT_UNSET_CHECKED_RANGE!()]
    fn bit_unset_checked_range(self, start: u32, end: u32) -> Result<Self, MismatchedBounds>;

    #[doc = _DOC_BIT_UNSET_ALL!()]
    fn bit_unset_all(self) -> Self;

    /* flip */

    #[must_use]
    #[doc = _DOC_BIT_FLIP!()]
    fn bit_flip(self, nth: u32) -> Self;
    #[doc = _DOC_BIT_FLIP_CHECKED!()]
    fn bit_flip_checked(self, nth: u32) -> Result<Self, MismatchedBounds>;

    #[must_use]
    #[doc = _DOC_BIT_FLIP_RANGE!()]
    fn bit_flip_range(self, start: u32, end: u32) -> Self;
    #[doc = _DOC_BIT_FLIP_CHECKED_RANGE!()]
    fn bit_flip_checked_range(self, start: u32, end: u32) -> Result<Self, MismatchedBounds>;

    #[must_use]
    #[doc = _DOC_BIT_FLIP_RANGE_IF!()]
    fn bit_flip_range_if(self, start: u32, end: u32, cond: bool) -> Self;
    #[doc = _DOC_BIT_FLIP_CHECKED_RANGE_IF!()]
    fn bit_flip_checked_range_if(self, start: u32, end: u32, cond: bool)
        -> Result<Self, MismatchedBounds>;

    /* reverse */

    #[must_use]
    #[doc = _DOC_BIT_REVERSE_RANGE!()]
    fn bit_reverse_range(self, start: u32, end: u32) -> Self;
    #[doc = _DOC_BIT_REVERSE_CHECKED_RANGE!()]
    fn bit_reverse_checked_range(self, start: u32, end: u32) -> Result<Self, MismatchedBounds>;

    /* count */

    #[must_use]
    #[doc = _DOC_BIT_COUNT_ONES_RANGE!()]
    fn bit_count_ones_range(self, start: u32, end: u32) -> u32;
    #[doc = _DOC_BIT_COUNT_ONES_CHECKED_RANGE!()]
    fn bit_count_ones_checked_range(self, start: u32, end: u32) -> Result<u32, MismatchedBounds>;

    #[must_use]
    #[doc = _DOC_BIT_COUNT_ZEROS_RANGE!()]
    fn bit_count_zeros_range(self, start: u32, end: u32) -> u32;
    #[doc = _DOC_BIT_COUNT_ZEROS_CHECKED_RANGE!()]
    fn bit_count_zeros_checked_range(self, start: u32, end: u32) -> Result<u32, MismatchedBounds>;

    /* find first */

    #[must_use]
    #[doc = _DOC_BIT_FIND_FIRST_ONE_RANGE!()]
    fn bit_find_first_one_range(self, start: u32, end: u32) -> Option<u32>;
    #[doc = _DOC_BIT_FIND_FIRST_ONE_CHECKED_RANGE!()]
    fn bit_find_first_one_checked_range(self, start: u32, end: u32)
        -> Result<Option<u32>, MismatchedBounds>;

    #[must_use]
    #[doc = _DOC_BIT_FIND_FIRST_ZERO_RANGE!()]
    fn bit_find_first_zero_range(self, start: u32, end: u32) -> Option<u32>;
    #[doc = _DOC_BIT_FIND_FIRST_ZERO_CHECKED_RANGE!()]
    fn bit_find_first_zero_checked_range(self, start: u32, end: u32)
        -> Result<Option<u32>, MismatchedBounds>;

    /* find last */

    #[must_use]
    #[doc = _DOC_BIT_FIND_LAST_ONE_RANGE!()]
    fn bit_find_last_one_range(self, start: u32, end: u32) -> Option<u32>;
    #[doc = _DOC_BIT_FIND_LAST_ONE_CHECKED_RANGE!()]
    fn bit_find_last_one_checked_range(self, start: u32, end: u32)
        -> Result<Option<u32>, MismatchedBounds>;

    #[must_use]
    #[doc = _DOC_BIT_FIND_LAST_ZERO_RANGE!()]
    fn bit_find_last_zero_range(self, start: u32, end: u32) -> Option<u32>;
    #[doc = _DOC_BIT_FIND_LAST_ZERO_CHECKED_RANGE!()]
    fn bit_find_last_zero_checked_range(self, start: u32, end: u32)
        -> Result<Option<u32>, MismatchedBounds>;
}

macro_rules! impl_bit_ops {
    () => {
        impl_bit_ops![
            u8, u16, u32, u64, u128, usize
        ];
    };

    // `$t`: the type to implement the trait for.
    ($($t:ty),+) => { $( impl_bit_ops![@$t]; )+ };
    (@$t:ty) => {
        impl BitOps for $t {
            type Inner = $t;

            /* mask */

            fn bit_mask_range(start: u32, end: u32) -> Self {
                Bitwise::<$t>::mask_range(start, end).0
            }
            fn bit_mask_checked_range(start: u32, end: u32) -> Result<Self, MismatchedBounds> {
                Ok(Bitwise::<$t>::mask_checked_range(start, end)?.0)
            }

            #[doc = _DOC_BIT_IS_SET_MASK!()]
            fn bit_is_set_mask(self, mask: $t) -> bool { Bitwise(self).is_set_mask(mask) }

            #[doc = _DOC_BIT_SET_MASK!()]
            fn bit_set_mask(self, mask: $t) -> Self { Bitwise(self).set_mask(mask).0 }

            #[doc = _DOC_BIT_IS_UNSET_MASK!()]
            fn bit_is_unset_mask(self, mask: $t) -> bool { Bitwise(self).is_unset_mask(mask) }

            #[doc = _DOC_BIT_UNSET_MASK!()]
            fn bit_unset_mask(self, mask: $t) -> Self { Bitwise(self).unset_mask(mask).0 }

            /* get */

            fn bit_get_range(self, start: u32, end: u32) -> Self {
                Bitwise(self).get_range(start, end).0
            }
            fn bit_get_checked_range(self, start: u32, end: u32) -> Result<Self, MismatchedBounds> {
                Ok(Bitwise(self).get_checked_range(start, end)?.0)
            }

            /* get value */

            fn bit_get_value_range(self, start: u32, end: u32) -> Self {
                Bitwise(self).get_value_range(start, end).0
            }
            fn bit_get_value_checked_range(self, start: u32, end: u32)
                -> Result<Self, MismatchedBounds> {
                Ok(Bitwise(self).get_value_checked_range(start, end)?.0)
            }

            /* set */

            fn bit_is_set(self, nth: u32) -> bool { Bitwise(self).is_set(nth) }
            fn bit_is_set_checked(self, nth: u32) -> Result<bool, MismatchedBounds> {
                Bitwise(self).is_set_checked(nth)
            }
            fn bit_set(self, nth: u32) -> Self { Bitwise(self).set(nth).0 }
            fn bit_set_checked(self, nth: u32) -> Result<Self, MismatchedBounds> {
                Ok(Bitwise(self).set_checked(nth)?.0)
            }

            fn bit_is_set_range(self, start: u32, end: u32) -> bool {
                Bitwise(self).is_set_range(start, end)
            }
            fn bit_is_set_checked_range(self, start: u32, end: u32)
                -> Result<bool, MismatchedBounds> {
                Bitwise(self).is_set_checked_range(start, end)
            }

            fn bit_set_range(self, start: u32, end: u32) -> Self {
                Bitwise(self).set_range(start, end).0
            }
            fn bit_set_checked_range(self, start: u32, end: u32) -> Result<Self, MismatchedBounds> {
                Ok(Bitwise(self).set_checked_range(start, end)?.0)
            }

            fn bit_set_all(self) -> Self { Bitwise(self).set_all().0 }

            /* set value */

            fn bit_set_value_range(self, value: Self::Inner, start: u32, end: u32) -> Self {
                Bitwise(self).set_value_range(value, start, end).0
            }
            fn bit_set_value_checked_range(self, value: Self::Inner, start: u32, end: u32)
                -> Result<Self, MismatchedBounds> {
                Ok(Bitwise(self).set_value_checked_range(value, start, end)?.0)
            }
            fn bit_set_checked_value_checked_range(self, value: Self::Inner, start: u32, end: u32)
                -> Result<Self, MismatchedBounds> {
                Ok(Bitwise(self).set_checked_value_checked_range(value, start, end)?.0)
            }

            /* unset */

            fn bit_is_unset(self, nth: u32) -> bool { Bitwise(self).is_unset(nth) }
            fn bit_is_unset_checked(self, nth: u32) -> Result<bool, MismatchedBounds> {
                Bitwise(self).is_unset_checked(nth)
            }
            fn bit_unset(self, nth: u32) -> Self { Bitwise(self).unset(nth).0 }
            fn bit_unset_checked(self, nth: u32) -> Result<Self, MismatchedBounds> {
                Ok(Bitwise(self).unset_checked(nth)?.0)
            }

            fn bit_is_unset_range(self, start: u32, end: u32) -> bool {
                Bitwise(self).is_unset_range(start, end)
            }
            fn bit_is_unset_checked_range(self, start: u32, end: u32)
                -> Result<bool, MismatchedBounds> {
                Bitwise(self).is_unset_checked_range(start, end)
            }

            fn bit_unset_range(self, start: u32, end: u32) -> Self {
                Bitwise(self).unset_range(start, end).0
            }
            fn bit_unset_checked_range(self, start: u32, end: u32)
                -> Result<Self, MismatchedBounds> {
                Ok(Bitwise(self).unset_checked_range(start, end)?.0)
            }

            fn bit_unset_all(self) -> Self { Bitwise(self).unset_all().0 }

            /* flip */

            fn bit_flip(self, nth: u32) -> Self { Bitwise(self).flip(nth).0 }
            fn bit_flip_checked(self, nth: u32) -> Result<Self, MismatchedBounds> {
                Ok(Bitwise(self).flip_checked(nth)?.0)
            }

            fn bit_flip_range(self, start: u32, end: u32) -> Self {
                Bitwise(self).flip_range(start, end).0
            }
            fn bit_flip_checked_range(self, start: u32, end: u32)
                -> Result<Self, MismatchedBounds> {
                Ok(Bitwise(self).flip_checked_range(start, end)?.0)
            }

            fn bit_flip_range_if(self, start: u32, end: u32, cond: bool) -> Self {
                Bitwise(self).flip_range_if(start, end, cond).0
            }
            fn bit_flip_checked_range_if(self, start: u32, end: u32, cond: bool)
                -> Result<Self, MismatchedBounds> {
                Ok(Bitwise(self).flip_checked_range_if(start, end, cond)?.0)
            }

            /* reverse */

            fn bit_reverse_range(self, start: u32, end: u32) -> Self {
                Bitwise(self).reverse_range(start, end).0
            }
            fn bit_reverse_checked_range(self, start: u32, end: u32)
                -> Result<Self, MismatchedBounds> {
                Ok(Bitwise(self).reverse_checked_range(start, end)?.0)
            }

            /* count */

            fn bit_count_ones_range(self, start: u32, end: u32) -> u32 {
                Bitwise(self).count_ones_range(start, end)
            }
            fn bit_count_ones_checked_range(self, start: u32, end: u32)
                -> Result<u32, MismatchedBounds> {
                Bitwise(self).count_ones_checked_range(start, end)
            }
            fn bit_count_zeros_range(self, start: u32, end: u32) -> u32 {
                Bitwise(self).count_zeros_range(start, end)
            }
            fn bit_count_zeros_checked_range(self, start: u32, end: u32)
                -> Result<u32, MismatchedBounds> {
                Bitwise(self).count_zeros_checked_range(start, end)
            }

            /* find first */

            fn bit_find_first_one_range(self, start: u32, end: u32) -> Option<u32> {
                Bitwise(self).find_first_one_range(start, end)
            }
            fn bit_find_first_one_checked_range(self, start: u32, end: u32)
                -> Result<Option<u32>, MismatchedBounds> {
                Bitwise(self).find_first_one_checked_range(start, end)
            }
            fn bit_find_first_zero_range(self, start: u32, end: u32) -> Option<u32> {
                Bitwise(self).find_first_zero_range(start, end)
            }
            fn bit_find_first_zero_checked_range(self, start: u32, end: u32)
                -> Result<Option<u32>, MismatchedBounds> {
                Bitwise(self).find_first_zero_checked_range(start, end)
            }

            /* find last */

            fn bit_find_last_one_range(self, start: u32, end: u32) -> Option<u32> {
                Bitwise(self).find_last_one_range(start, end)
            }
            fn bit_find_last_one_checked_range(self, start: u32, end: u32)
                -> Result<Option<u32>, MismatchedBounds> {
                Bitwise(self).find_last_one_checked_range(start, end)
            }
            fn bit_find_last_zero_range(self, start: u32, end: u32) -> Option<u32> {
                Bitwise(self).find_last_zero_range(start, end)
            }
            fn bit_find_last_zero_checked_range(self, start: u32, end: u32)
                -> Result<Option<u32>, MismatchedBounds> {
                Bitwise(self).find_last_zero_checked_range(start, end)
            }
        }
    };
}
impl_bit_ops![];
