// devela::data::conversion::primitive
//
//! Helpers for converting between primitives.
//

use crate::{data::DataResult as Result, meta::paste};

#[cfg(test)]
mod tests;

mod bits;
mod cast;
mod join;
mod split;

pub use {bits::*, cast::*, join::*, split::*};

/* define traits */

/// Offers methods for bitwise operations on primitives.
pub trait BitwisePrimitives
where
    Self: Sized,
{
    /// Returns a bitmask set from the `start` to the `end` bit, inclusive.
    ///
    /// This function underlies all the bit range operations.
    #[doc = include_str!("./Benchmarks_bit_mask_checked.md")]
    fn bit_mask_range(start: u32, end: u32) -> Result<Self>;
    /// Returns a bitmask set from the `start` to the `end` bit, inclusive, unchecked version.
    ///
    /// This function underlies all the unchecked bit range operations.
    #[doc = include_str!("./Benchmarks_bit_mask_unchecked.md")]
    #[must_use]
    fn bit_mask_range_unchecked(start: u32, end: u32) -> Self;
    /// Sets the bits in `self` to 1, from `start` to `end`, inclusive.
    fn bit_set_range(self, start: u32, end: u32) -> Result<Self>;
    /// Sets the bits in `self` to 1, from `start` to `end`, inclusive, unchecked version.
    #[must_use]
    fn bit_set_range_unchecked(self, start: u32, end: u32) -> Self;
    /// Unsets the bits in `self` to 0, from `start` to `end`, inclusive.
    fn bit_unset_range(self, start: u32, end: u32) -> Result<Self>;
    /// Unsets the bits in `self` to 0, from `start` to `end`, inclusive, unchecked version.
    #[must_use]
    fn bit_unset_range_unchecked(self, start: u32, end: u32) -> Self;
    /// Flips the bits in `self`, from `start` to `end`, inclusive.
    fn bit_flip_range(self, start: u32, end: u32) -> Result<Self>;
    /// Flips the bits in `self` from `start` to `end`, inclusive, unchecked version.
    #[must_use]
    fn bit_flip_range_unchecked(self, start: u32, end: u32) -> Self;
}

/// Offers methods for casting between primitives.
pub trait CastPrimitives {
    /// Safely casts `self` to `u8` with range check.
    fn checked_cast_to_u8(self) -> Result<u8>;
    /// Safely casts `self` to `u16` with range check.
    fn checked_cast_to_u16(self) -> Result<u16>;
    /// Safely casts `self` to `u32` with range check.
    fn checked_cast_to_u32(self) -> Result<u32>;
    /// Safely casts `self` to `u64` with range check.
    fn checked_cast_to_u64(self) -> Result<u64>;
    /// Safely casts `self` to `u128` with range check.
    fn checked_cast_to_u128(self) -> Result<u128>;
    /// Safely casts `self` to `i8` with range check.
    fn checked_cast_to_i8(self) -> Result<i8>;
    /// Safely casts `self` to `i16` with range check.
    fn checked_cast_to_i16(self) -> Result<i16>;
    /// Safely casts `self` to `i32` with range check.
    fn checked_cast_to_i32(self) -> Result<i32>;
    /// Safely casts `self` to `i64` with range check.
    fn checked_cast_to_i64(self) -> Result<i64>;
    /// Safely casts `self` to `i128` with range check.
    fn checked_cast_to_i128(self) -> Result<i128>;
    /// Saturating casts `self` to `u8` clamping at the numeric bounds.
    fn saturating_cast_to_u8(self) -> u8;
    /// Saturating casts `self` to `u16` clamping at the numeric bounds.
    fn saturating_cast_to_u16(self) -> u16;
    /// Saturating casts `self` to `u32` clamping at the numeric bounds.
    fn saturating_cast_to_u32(self) -> u32;
    /// Saturating casts `self` to `u64` clamping at the numeric bounds.
    fn saturating_cast_to_u64(self) -> u64;
    /// Saturating casts `self` to `u128` clamping at the numeric bounds.
    fn saturating_cast_to_u128(self) -> u128;
    /// Saturating casts `self` to `i8` clamping at the numeric bounds.
    fn saturating_cast_to_i8(self) -> i8;
    /// Saturating casts `self` to `i16` clamping at the numeric bounds.
    fn saturating_cast_to_i16(self) -> i16;
    /// Saturating casts `self` to `i32` clamping at the numeric bounds.
    fn saturating_cast_to_i32(self) -> i32;
    /// Saturating casts `self` to `i64` clamping at the numeric bounds.
    fn saturating_cast_to_i64(self) -> i64;
    /// Saturating casts `self` to `i128` clamping at the numeric bounds.
    fn saturating_cast_to_i128(self) -> i128;
}

/// Offers methods to construct a primitive `T` from a slice of `U` primitives.
///
/// Methods expecting an array are more efficient than the ones expecting an
/// slice. On the other hand slices of any lenght are supported as follows:
/// - If the slice contains fewer elements than required, the method will
///   fill in the missing values with zeros.
/// - If the slice contains more elements than required, the method will
///   ignore the extra elements.
///
/// See also [`IntoPrimitives`].
pub trait FromPrimitives<T, U, const LEN: usize> {
    /// Constructs a primitive `T` from an array of `U` in big-endian order.
    #[must_use]
    fn from_array_be(values: [U; LEN]) -> T;
    /// Constructs a primitive `T` from an array of `U` in little-endian order.
    #[must_use]
    fn from_array_le(values: [U; LEN]) -> T;
    /// Constructs a primitive `T` from an array of `U` in native-endian order.
    #[must_use]
    fn from_array_ne(values: [U; LEN]) -> T;

    /// Constructs a primitive `T` from a slice of `U` in big-endian order.
    #[must_use]
    fn from_slice_be(values: &[U]) -> T;
    /// Constructs a primitive `T` from a slice of `U` in little-endian order.
    #[must_use]
    fn from_slice_le(values: &[U]) -> T;
    /// Constructs a primitive `T` from a slice of `U` in native-endian order.
    #[must_use]
    fn from_slice_ne(values: &[U]) -> T;
}

/// Offers methods to split a primitive `T` into a slice of `U` primitives.
///
/// Slices of different lengths will be handled as follows:
/// - If the slice contains fewer elements than required, the method will
///   fill in the missing values with zeros.
/// - If the slice contains more elements than required, the method will
///   ignore the extra elements.
///
/// See also [`FromPrimitives`].
pub trait IntoPrimitives<T, U, const LEN: usize> {
    /// Splits a primitive `T` from a slice of `U` in big-endian order.
    #[must_use]
    fn into_array_be(value: T) -> [U; LEN];
    /// Splits a primitive `T` into a slice of `U` in little-endian order.
    #[must_use]
    fn into_array_le(value: T) -> [U; LEN];
    /// Splits a primitive `T` into an array of `U` in native-endian order.
    #[must_use]
    fn into_array_ne(value: T) -> [U; LEN];
}

/* implement traits */

macro_rules! impl_bitwise_primitives {
    ($($t:ty),+) => { $( impl_bitwise_primitives![@$t]; )+ };
    (@$t:ty) => { paste! {
        impl BitwisePrimitives for $t {
            fn bit_mask_range(start: u32, end: u32) -> Result<Self> {
                [<bit_mask_range_ $t>](start, end)
            }
            fn bit_mask_range_unchecked(start: u32, end: u32) -> Self {
                [<bit_mask_range_unchecked_ $t>](start, end)
            }
            fn bit_set_range(self, start: u32, end: u32) -> Result<Self> {
                [<bit_set_range_ $t>](self, start, end)
            }
            fn bit_set_range_unchecked(self, start: u32, end: u32) -> Self {
                [<bit_set_range_unchecked_ $t>](self, start, end)
            }
            fn bit_unset_range(self, start: u32, end: u32) -> Result<Self> {
                [<bit_unset_range_ $t>](self, start, end)
            }
            fn bit_unset_range_unchecked(self, start: u32, end: u32) -> Self {
                [<bit_unset_range_unchecked_ $t>](self, start, end)
            }
            fn bit_flip_range(self, start: u32, end: u32) -> Result<Self> {
                [<bit_flip_range_ $t>](self, start, end)
            }
            fn bit_flip_range_unchecked(self, start: u32, end: u32) -> Self {
                [<bit_flip_range_unchecked_ $t>](self, start, end)
            }
        }
    }};
}
impl_bitwise_primitives![u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];

macro_rules! impl_cast_primitives {
    ($($t:ty),+) => { $( impl_cast_primitives![@$t]; )+ };
    (@$t:ty) => { paste! {
        impl CastPrimitives for $t {
            fn checked_cast_to_u8(self) -> Result<u8> {
                [<checked_cast_ $t _ to_ u8>](self)
            }
            fn checked_cast_to_u16(self) -> Result<u16> {
                [<checked_cast_ $t _ to_ u16>](self)
            }
            fn checked_cast_to_u32(self) -> Result<u32> {
                [<checked_cast_ $t _ to_ u32>](self)
            }
            fn checked_cast_to_u64(self) -> Result<u64> {
                [<checked_cast_ $t _ to_ u64>](self)
            }
            fn checked_cast_to_u128(self) -> Result<u128> {
                [<checked_cast_ $t _ to_ u128>](self)
            }
            fn checked_cast_to_i8(self) -> Result<i8> {
                [<checked_cast_ $t _ to_ i8>](self)
            }
            fn checked_cast_to_i16(self) -> Result<i16> {
                [<checked_cast_ $t _ to_ i16>](self)
            }
            fn checked_cast_to_i32(self) -> Result<i32> {
                [<checked_cast_ $t _ to_ i32>](self)
            }
            fn checked_cast_to_i64(self) -> Result<i64> {
                [<checked_cast_ $t _ to_ i64>](self)
            }
            fn checked_cast_to_i128(self) -> Result<i128> {
                [<checked_cast_ $t _ to_ i128>](self)
            }
            fn saturating_cast_to_u8(self) -> u8 {
                [<saturating_cast_ $t _ to_ u8>](self)
            }
            fn saturating_cast_to_u16(self) -> u16 {
                [<saturating_cast_ $t _ to_ u16>](self)
            }
            fn saturating_cast_to_u32(self) -> u32 {
                [<saturating_cast_ $t _ to_ u32>](self)
            }
            fn saturating_cast_to_u64(self) -> u64 {
                [<saturating_cast_ $t _ to_ u64>](self)
            }
            fn saturating_cast_to_u128(self) -> u128 {
                [<saturating_cast_ $t _ to_ u128>](self)
            }
            fn saturating_cast_to_i8(self) -> i8 {
                [<saturating_cast_ $t _ to_ i8>](self)
            }
            fn saturating_cast_to_i16(self) -> i16 {
                [<saturating_cast_ $t _ to_ i16>](self)
            }
            fn saturating_cast_to_i32(self) -> i32 {
                [<saturating_cast_ $t _ to_ i32>](self)
            }
            fn saturating_cast_to_i64(self) -> i64 {
                [<saturating_cast_ $t _ to_ i64>](self)
            }
            fn saturating_cast_to_i128(self) -> i128 {
                [<saturating_cast_ $t _ to_ i128>](self)
            }
        }
    }};
}
impl_cast_primitives![u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];

macro_rules! impl_from_primitives {
    ( $( $T:ident, $U:ident, $LEN:literal );+ $(;)? ) => {
        $( impl_from_primitives![@$T, $U, $LEN]; )+
    };
    (@$T:ident, $U:ident, $LEN:literal) => { paste! {
        impl FromPrimitives<$T, $U, $LEN> for $T {
            #[inline]
            fn from_array_be(values: [$U; $LEN]) -> $T {
                [<$T _from_ $U _be>](values)
            }
            #[inline]
            fn from_array_le(values: [$U; $LEN]) -> $T {
                [<$T _from_ $U _le>](values)
            }
            #[inline]
            fn from_array_ne(values: [$U; $LEN]) -> $T {
                [<$T _from_ $U _ne>](values)
            }

            #[inline]
            fn from_slice_be(values: &[$U]) -> $T {
                let mut array = [0; $LEN];

                for (i, &v) in values.iter().enumerate() {
                    array[i] = v;
                }
                [<$T _from_ $U _be>](array)
            }
            #[inline]
            fn from_slice_le(values: &[$U]) -> $T {
                let mut array = [0; $LEN];

                for (i, &v) in values.iter().enumerate() {
                    array[i] = v;
                }
                [<$T _from_ $U _le>](array)
            }
            #[inline]
            fn from_slice_ne(values: &[$U]) -> $T {
                let mut array = [0; $LEN];

                for (i, &v) in values.iter().enumerate() {
                    array[i] = v;
                }
                [<$T _from_ $U _ne>](array)
            }
        }
    }};
}
impl_from_primitives![
    u128, u64, 2; u128, u32, 4; u128, u16, 8; u128, u8, 16;
    u64, u32, 2; u64, u16, 4; u64, u8, 8;
    u32, u16, 2; u32, u8, 4;
    u16, u8, 2;
];

macro_rules! impl_into_primitives {
    ( $( $T:ident, $U:ident, $LEN:literal );+ $(;)? ) => {
        $( impl_into_primitives![@$T, $U, $LEN]; )+
    };
    (@$T:ident, $U:ident, $LEN:literal) => { paste! {
        impl IntoPrimitives<$T, $U, $LEN> for $T {
            #[inline]
            fn into_array_be(value: $T) -> [$U; $LEN] {
                [<$T _into_ $U _be>](value)
            }
            #[inline]
            fn into_array_le(value: $T) -> [$U; $LEN] {
                [<$T _into_ $U _le>](value)
            }
            #[inline]
            fn into_array_ne(value: $T) -> [$U; $LEN] {
                [<$T _into_ $U _ne>](value)
            }
        }
    }};
}
impl_into_primitives![
    u128, u64, 2; u128, u32, 4; u128, u16, 8; u128, u8, 16;
    u64, u32, 2; u64, u16, 4; u64, u8, 8;
    u32, u16, 2; u32, u8, 4;
    u16, u8, 2;
];
