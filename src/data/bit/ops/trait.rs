// devela::data::bit::ops::trait

use super::Bits;
#[cfg(doc)]
use crate::data::DataErrors as E;
use crate::data::DataResult as Result;

/// Provides bitwise operations on primitives.
pub trait BitOps
where
    Self: Sized,
{
    /* new mask */

    /// Returns a bitmask of ones from the `[start..=end]` range.
    ///
    /// Sets the rest of the bits to 0.
    ///
    /// # Panics
    /// Panics in debug if `start >= Self::BITS` || `end >= Self::BITS` || `start > end`.
    #[doc = include_str!("./Benchmarks_bit_mask_range.md")]
    #[must_use]
    fn bit_mask_range(start: u32, end: u32) -> Self;

    /// Returns a bitmask of ones from the `[start..=end]` checked range.
    ///
    /// Sets the rest of the bits to 0.
    ///
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
    /// Returns [`OutOfBounds`][E::OutOfBounds] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`][E::MismatchedIndices] if `start > end`.
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
    /// Returns [`OutOfBounds`][E::OutOfBounds] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`][E::MismatchedIndices] if `start > end`.
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
    /// Returns [`OutOfBounds`][E::OutOfBounds] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`][E::MismatchedIndices] if `start > end`.
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
    /// Returns [`OutOfBounds`][E::OutOfBounds] if `start >= Self::BITS` || `end >= Self::BITS`
    /// and [`MismatchedIndices`][E::MismatchedIndices] if `start > end`.
    fn bit_find_last_zero_checked_range(self, start: u32, end: u32) -> Result<Option<u32>>;
}

macro_rules! impl_bit_ops {
    ($($t:ty),+) => { $( impl_bit_ops![@$t]; )+ };
    (@$t:ty) => {
        impl BitOps for $t {
            // new mask
            fn bit_mask_range(start: u32, end: u32) -> Self {
                Bits::<$t>::mask_range(start, end).0
            }
            fn bit_mask_checked_range(start: u32, end: u32) -> Result<Self> {
                Ok(Bits::<$t>::mask_checked_range(start, end)?.0)
            }
            // get
            fn bit_get_range(self, start: u32, end: u32) -> Self {
                Bits(self).get_range(start, end).0
            }
            fn bit_get_checked_range(self, start: u32, end: u32) -> Result<Self> {
                Ok(Bits(self).get_checked_range(start, end)?.0)
            }
            // get shifted
            fn bit_get_shifted_range(self, start: u32, end: u32) -> Self {
                Bits(self).get_shifted_range(start, end).0
            }
            fn bit_get_shifted_checked_range(self, start: u32, end: u32) -> Result<Self> {
                Ok(Bits(self).get_shifted_checked_range(start, end)?.0)
            }
            // set
            fn bit_set_range(self, start: u32, end: u32) -> Self {
                Bits(self).set_range(start, end).0
            }
            fn bit_set_checked_range(self, start: u32, end: u32) -> Result<Self> {
                Ok(Bits(self).set_checked_range(start, end)?.0)
            }
            // unset
            fn bit_unset_range(self, start: u32, end: u32) -> Self {
                Bits(self).unset_range(start, end).0
            }
            fn bit_unset_checked_range(self, start: u32, end: u32) -> Result<Self> {
                Ok(Bits(self).unset_checked_range(start, end)?.0)
            }
            // flip
            fn bit_flip_range(self, start: u32, end: u32) -> Self {
                Bits(self).flip_range(start, end).0
            }
            fn bit_flip_checked_range(self, start: u32, end: u32) -> Result<Self> {
                Ok(Bits(self).flip_checked_range(start, end)?.0)
            }
            // reverse
            fn bit_reverse_range(self, start: u32, end: u32) -> Self {
                Bits(self).reverse_range(start, end).0
            }
            fn bit_reverse_checked_range(self, start: u32, end: u32) -> Result<Self> {
                Ok(Bits(self).reverse_checked_range(start, end)?.0)
            }
            // count
            fn bit_count_ones_range(self, start: u32, end: u32) -> u32 {
                Bits(self).count_ones_range(start, end)
            }
            fn bit_count_ones_checked_range(self, start: u32, end: u32) -> Result<u32> {
                Bits(self).count_ones_checked_range(start, end)
            }
            fn bit_count_zeros_range(self, start: u32, end: u32) -> u32 {
                Bits(self).count_zeros_range(start, end)
            }
            fn bit_count_zeros_checked_range(self, start: u32, end: u32) -> Result<u32> {
                Bits(self).count_zeros_checked_range(start, end)
            }
            // find first
            fn bit_find_first_one_range(self, start: u32, end: u32) -> Option<u32> {
                Bits(self).find_first_one_range(start, end)
            }
            fn bit_find_first_one_checked_range(self, start: u32, end: u32) -> Result<Option<u32>> {
                Bits(self).find_first_one_checked_range(start, end)
            }
            fn bit_find_first_zero_range(self, start: u32, end: u32) -> Option<u32> {
                Bits(self).find_first_zero_range(start, end)
            }
            fn bit_find_first_zero_checked_range(self, start: u32, end: u32) -> Result<Option<u32>> {
                Bits(self).find_first_zero_checked_range(start, end)
            }
            // find last
            fn bit_find_last_one_range(self, start: u32, end: u32) -> Option<u32> {
                Bits(self).find_last_one_range(start, end)
            }
            fn bit_find_last_one_checked_range(self, start: u32, end: u32) -> Result<Option<u32>> {
                Bits(self).find_last_one_checked_range(start, end)
            }
            fn bit_find_last_zero_range(self, start: u32, end: u32) -> Option<u32> {
                Bits(self).find_last_zero_range(start, end)
            }
            fn bit_find_last_zero_checked_range(self, start: u32, end: u32) -> Result<Option<u32>> {
                Bits(self).find_last_zero_checked_range(start, end)
            }
        }
    };
}
impl_bit_ops![u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];
