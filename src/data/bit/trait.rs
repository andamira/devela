// devela::data::bit::ops::trait

use super::Biting;
#[cfg(doc)]
use crate::data::DataErrors::{MismatchedIndices, OutOfBounds, Overflow};
use crate::data::DataResult as Result;

/// Provides bitwise operations on `T`.
///
/// See also [`Biting`] for the equivalent const wrapper.
pub trait BitOps
where
    Self: Sized,
{
    /// The inner type for the bit representation.
    type Inner;

    /* new mask */

    /// Returns a bitmask of ones from the `[start..=end]` range.
    ///
    /// Sets the rest of the bits to 0.
    ///
    /// # Panics
    /// Panics in debug if `start >= Self::BITS` || `end >= Self::BITS` || `start > end`.
    #[doc = include_str!("./benches/mask_range.md")]
    #[must_use]
    fn bit_mask_range(start: u32, end: u32) -> Self;

    /// Returns a bitmask of ones from the `[start..=end]` checked range.
    ///
    /// Sets the rest of the bits to 0.
    ///
    /// # Errors
    /// Returns [`OutOfBounds`] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`] if `start > end`.
    #[doc = include_str!("./benches/mask_checked_range.md")]
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
    /// Returns [`OutOfBounds`] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`] if `start > end`.
    fn bit_get_checked_range(self, start: u32, end: u32) -> Result<Self>;

    /* get value */

    /// Gets the rightwards shifted bits in `self` from the `[start..=end]` range.
    ///
    /// Sets the rest of the bits to 0.
    ///
    /// The bits in the specified range are shifted rightwards so that the least
    /// significant bit (LSB) aligns with the units place, forming the integer value.
    /// # Panics
    /// Panics in debug if `start >= Self::BITS` || `end >= Self::BITS` || `start > end`.
    #[must_use]
    fn bit_get_value_range(self, start: u32, end: u32) -> Self;

    /// Gets the rightwards shifted bits in `self` from the `[start..=end]` checked range.
    ///
    /// Sets the rest of the bits to 0.
    ///
    /// The bits in the specified range are shifted rightwards so that the least
    /// significant bit (LSB) aligns with the units place, forming the integer value.
    /// # Errors
    /// Returns [`OutOfBounds`] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`] if `start > end`.
    fn bit_get_value_checked_range(self, start: u32, end: u32) -> Result<Self>;

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
    /// Returns [`OutOfBounds`] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`] if `start > end`.
    fn bit_set_checked_range(self, start: u32, end: u32) -> Result<Self>;

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
    fn bit_set_value_range(self, value: Self::Inner, start: u32, end: u32) -> Self;

    /// Sets the given `value` into the bits from the `[start..=end]` checked range.
    ///
    /// Leaves the rest of the bits unchanged.
    /// # Errors
    /// Returns [`OutOfBounds`] if `start >= BITS || end >= BITS`
    /// and [`MismatchedIndices`] if `start > end`.
    fn bit_set_value_checked_range(self, value: Self::Inner, start: u32, end: u32) -> Result<Self>;

    /// Sets the given checked `value` into the bits from the `[start..=end]` checked range.
    ///
    /// Leaves the rest of the bits unchanged.
    /// # Errors
    /// Returns [`OutOfBounds`] if `start >= BITS || end >= BITS`,
    /// [`MismatchedIndices`] if `start > end` and
    /// [`Overflow`] if `value` does not fit within the specified bit range.
    fn bit_set_checked_value_checked_range(
        self,
        value: Self::Inner,
        start: u32,
        end: u32,
    ) -> Result<Self>;

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
    /// Returns [`OutOfBounds`] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`] if `start > end`.
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
    /// Returns [`OutOfBounds`] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`] if `start > end`.
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
    /// Returns [`OutOfBounds`] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`] if `start > end`.
    fn bit_reverse_checked_range(self, start: u32, end: u32) -> Result<Self>;

    /* count */

    /// Counts the number of 1s in `bits` from the `[start..=end]` range.
    /// # Panics
    /// Panics in debug if `start >= Self::BITS` || `end >= Self::BITS` || `start > end`.
    #[must_use]
    fn bit_count_ones_range(self, start: u32, end: u32) -> u32;

    /// Counts the number of 1s in `bits` from the `[start..=end]` checked range.
    /// # Errors
    /// Returns [`OutOfBounds`] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`] if `start > end`.
    fn bit_count_ones_checked_range(self, start: u32, end: u32) -> Result<u32>;

    /// Counts the number of 0s in `bits` from the `[start..=end]` range.
    /// # Panics
    /// Panics in debug if `start >= Self::BITS` || `end >= Self::BITS` || `start > end`.
    #[must_use]
    fn bit_count_zeros_range(self, start: u32, end: u32) -> u32;

    /// Counts the number of 0s in `bits` from the `[start..=end]` checked range.
    /// # Errors
    /// Returns [`OutOfBounds`] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`] if `start > end`.
    fn bit_count_zeros_checked_range(self, start: u32, end: u32) -> Result<u32>;

    /* find first */

    /// Finds the index of the first 1 in `bits` from the `[start..=end]` range.
    ///
    /// Returns `None` if there are no bits set.
    ///
    /// The index is relative to the entire sequence of `bits`, not to the given `start`.
    /// # Panics
    /// Panics in debug if `start >= Self::BITS` || `end >= Self::BITS` || `start > end`.
    #[must_use]
    fn bit_find_first_one_range(self, start: u32, end: u32) -> Option<u32>;

    /// Finds the index of the first 1 in `bits` from the `[start..=end]` checked range.
    ///
    /// Returns `None` if there are no bits set.
    ///
    /// The index is relative to the entire sequence of `bits`, not to the given `start`.
    /// # Errors
    /// Returns [`OutOfBounds`] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`] if `start > end`.
    fn bit_find_first_one_checked_range(self, start: u32, end: u32) -> Result<Option<u32>>;

    /// Finds the index of the first 0 in `bits` from the `[start..=end]` range.
    ///
    /// Returns `None` if there are no bits unset.
    ///
    /// The index is relative to the entire sequence of `bits`, not to the given `start`.
    /// # Panics
    /// Panics in debug if `start >= Self::BITS` || `end >= Self::BITS` || `start > end`.
    #[must_use]
    fn bit_find_first_zero_range(self, start: u32, end: u32) -> Option<u32>;

    /// Finds the index of the first 0 in `bits` from the `[start..=end]` checked range.
    ///
    /// Returns `None` if there are no bits unset.
    ///
    /// The index is relative to the entire sequence of `bits`, not to the given `start`.
    /// # Errors
    /// Returns [`OutOfBounds`] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`] if `start > end`.
    fn bit_find_first_zero_checked_range(self, start: u32, end: u32) -> Result<Option<u32>>;

    /* find last */

    /// Finds the index of the last 1 in `bits` from the `[start..=end]` range.
    ///
    /// Returns `None` if there are no bits set.
    ///
    /// The index is relative to the entire sequence of `bits`, not to the given `start`.
    /// # Panics
    /// Panics in debug if `start >= Self::BITS` || `end >= Self::BITS` || `start > end`.
    #[must_use]
    fn bit_find_last_one_range(self, start: u32, end: u32) -> Option<u32>;

    /// Finds the index of the last 1 in `bits` from the `[start..=end]` checked range.
    ///
    /// Returns `None` if there are no bits set.
    ///
    /// The index is relative to the entire sequence of `bits`, not to the given `start`.
    /// # Errors
    /// Returns [`OutOfBounds`] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`] if `start > end`.
    fn bit_find_last_one_checked_range(self, start: u32, end: u32) -> Result<Option<u32>>;

    /// Finds the index of the last 0 in `bits` from the `[start..=end]` range.
    ///
    /// Returns `None` if there are no bits unset.
    ///
    /// The index is relative to the entire sequence of `bits`, not to the given `start`.
    /// # Panics
    /// Panics in debug if `start >= Self::BITS` || `end >= Self::BITS` || `start > end`.
    #[must_use]
    fn bit_find_last_zero_range(self, start: u32, end: u32) -> Option<u32>;

    /// Finds the index of the last 0 in `bits` from the `[start..=end]` checked range.
    ///
    /// Returns `None` if there are no bits unset.
    ///
    /// The index is relative to the entire sequence of `bits`, not to the given `start`.
    /// # Errors
    /// Returns [`OutOfBounds`] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`] if `start > end`.
    fn bit_find_last_zero_checked_range(self, start: u32, end: u32) -> Result<Option<u32>>;
}

macro_rules! impl_bit_ops {
    ($($t:ty),+) => { $( impl_bit_ops![@$t]; )+ };
    (@$t:ty) => {
        impl BitOps for $t {
            type Inner = $t;

            // new mask
            fn bit_mask_range(start: u32, end: u32) -> Self {
                Biting::<$t>::mask_range(start, end).0
            }
            fn bit_mask_checked_range(start: u32, end: u32) -> Result<Self> {
                Ok(Biting::<$t>::mask_checked_range(start, end)?.0)
            }
            // get
            fn bit_get_range(self, start: u32, end: u32) -> Self {
                Biting(self).get_range(start, end).0
            }
            fn bit_get_checked_range(self, start: u32, end: u32) -> Result<Self> {
                Ok(Biting(self).get_checked_range(start, end)?.0)
            }
            // get value
            fn bit_get_value_range(self, start: u32, end: u32) -> Self {
                Biting(self).get_value_range(start, end).0
            }
            fn bit_get_value_checked_range(self, start: u32, end: u32) -> Result<Self> {
                Ok(Biting(self).get_value_checked_range(start, end)?.0)
            }
            // set
            fn bit_set_range(self, start: u32, end: u32) -> Self {
                Biting(self).set_range(start, end).0
            }
            fn bit_set_checked_range(self, start: u32, end: u32) -> Result<Self> {
                Ok(Biting(self).set_checked_range(start, end)?.0)
            }
            // set value
            fn bit_set_value_range(self, value: Self::Inner, start: u32, end: u32) -> Self {
                Biting(self).set_value_range(value, start, end).0
            }
            fn bit_set_value_checked_range(self, value: Self::Inner, start: u32, end: u32)
                -> Result<Self> {
                Ok(Biting(self).set_value_checked_range(value, start, end)?.0)
            }
            fn bit_set_checked_value_checked_range(self, value: Self::Inner, start: u32, end: u32)
                -> Result<Self> {
                Ok(Biting(self).set_checked_value_checked_range(value, start, end)?.0)
            }
            // unset
            fn bit_unset_range(self, start: u32, end: u32) -> Self {
                Biting(self).unset_range(start, end).0
            }
            fn bit_unset_checked_range(self, start: u32, end: u32) -> Result<Self> {
                Ok(Biting(self).unset_checked_range(start, end)?.0)
            }
            // flip
            fn bit_flip_range(self, start: u32, end: u32) -> Self {
                Biting(self).flip_range(start, end).0
            }
            fn bit_flip_checked_range(self, start: u32, end: u32) -> Result<Self> {
                Ok(Biting(self).flip_checked_range(start, end)?.0)
            }
            // reverse
            fn bit_reverse_range(self, start: u32, end: u32) -> Self {
                Biting(self).reverse_range(start, end).0
            }
            fn bit_reverse_checked_range(self, start: u32, end: u32) -> Result<Self> {
                Ok(Biting(self).reverse_checked_range(start, end)?.0)
            }
            // count
            fn bit_count_ones_range(self, start: u32, end: u32) -> u32 {
                Biting(self).count_ones_range(start, end)
            }
            fn bit_count_ones_checked_range(self, start: u32, end: u32) -> Result<u32> {
                Biting(self).count_ones_checked_range(start, end)
            }
            fn bit_count_zeros_range(self, start: u32, end: u32) -> u32 {
                Biting(self).count_zeros_range(start, end)
            }
            fn bit_count_zeros_checked_range(self, start: u32, end: u32) -> Result<u32> {
                Biting(self).count_zeros_checked_range(start, end)
            }
            // find first
            fn bit_find_first_one_range(self, start: u32, end: u32) -> Option<u32> {
                Biting(self).find_first_one_range(start, end)
            }
            fn bit_find_first_one_checked_range(self, start: u32, end: u32) -> Result<Option<u32>> {
                Biting(self).find_first_one_checked_range(start, end)
            }
            fn bit_find_first_zero_range(self, start: u32, end: u32) -> Option<u32> {
                Biting(self).find_first_zero_range(start, end)
            }
            fn bit_find_first_zero_checked_range(self, start: u32, end: u32) -> Result<Option<u32>> {
                Biting(self).find_first_zero_checked_range(start, end)
            }
            // find last
            fn bit_find_last_one_range(self, start: u32, end: u32) -> Option<u32> {
                Biting(self).find_last_one_range(start, end)
            }
            fn bit_find_last_one_checked_range(self, start: u32, end: u32) -> Result<Option<u32>> {
                Biting(self).find_last_one_checked_range(start, end)
            }
            fn bit_find_last_zero_range(self, start: u32, end: u32) -> Option<u32> {
                Biting(self).find_last_zero_range(start, end)
            }
            fn bit_find_last_zero_checked_range(self, start: u32, end: u32) -> Result<Option<u32>> {
                Biting(self).find_last_zero_checked_range(start, end)
            }
        }
    };
}
impl_bit_ops![u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];
