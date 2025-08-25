// devela::data::conversion::cast::join
//
//! construct an unsigned primitive by joining an array of smaller unsigned primitives.
//
// TOC
// - trait PrimitiveJoin
// - macro impl_from_trait!

use crate::Cast;

/// Offers methods to construct a primitive from an array or slice of smaller primitives.
///
/// Methods expecting an array are more efficient than the ones expecting an
/// slice. On the other hand slices of any lenght are supported as follows:
/// - If the slice contains fewer elements than required, the method will
///   fill in the missing values with zeros.
/// - If the slice contains more elements than required, the method will
///   ignore the extra elements.
///
/// See also the [`Cast`] type for the equivalent *const* methods, and the
/// [`PrimitiveSplit`][super::PrimitiveSplit] trait for the opposite operations.
pub trait PrimitiveJoin<T, U, const LEN: usize> {
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

/// Implements the trait methods.
macro_rules! impl_from_trait {
    ( $( $T:ident, $U:ident, $LEN:literal );+ $(;)? ) => {
        $( impl_from_trait![@$T, $U, $LEN]; )+
    };
    (@$T:ident, $U:ident, $LEN:literal) => { crate::paste! {
        impl PrimitiveJoin<$T, $U, $LEN> for $T {
            fn from_array_be(values: [$U; $LEN]) -> $T { Cast::<$T>::[<from_ $U _be>](values) }
            fn from_array_le(values: [$U; $LEN]) -> $T { Cast::<$T>::[<from_ $U _le>](values) }
            fn from_array_ne(values: [$U; $LEN]) -> $T { Cast::<$T>::[<from_ $U _ne>](values) }
            fn from_slice_be(values: &[$U]) -> $T {
                let mut array = [0; $LEN];
                for (i, &v) in values.iter().enumerate() {
                    array[i] = v;
                }
                Cast::<$T>::[<from_ $U _be>](array)
            }
            fn from_slice_le(values: &[$U]) -> $T {
                let mut array = [0; $LEN];
                for (i, &v) in values.iter().enumerate() {
                    array[i] = v;
                }
                Cast::<$T>::[<from_ $U _le>](array)
            }
            fn from_slice_ne(values: &[$U]) -> $T {
                let mut array = [0; $LEN];
                for (i, &v) in values.iter().enumerate() {
                    array[i] = v;
                }
                Cast::<$T>::[<from_ $U _ne>](array)
            }
        }
    }};
}
impl_from_trait![
    u128, u64, 2; u128, u32, 4; u128, u16, 8; u128, u8, 16;
    u64, u32, 2; u64, u16, 4; u64, u8, 8;
    u32, u16, 2; u32, u8, 4;
    u16, u8, 2;
];
