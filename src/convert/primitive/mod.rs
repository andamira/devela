// devela::conversion::primitive
//
//! Helpers for converting between primitives.
//

use crate::meta::paste;

#[cfg(test)]
mod tests;

mod join;
mod split;

pub use {join::*, split::*};

/* define traits */

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
