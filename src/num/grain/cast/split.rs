// devela::num::grain::cast::split
//
//! fns to split a primitive into an array of smaller primitives.
//
// TOC
// - trait PrimitiveSplit
// - macro impl_into_trait!

use crate::Cast;

#[doc = crate::_tags!(num)]
/// Offers methods to split a primitive into an array of smaller primitives.
#[doc = crate::_doc_location!("num")]
///
/// See also the [`Cast`] type for the equivalent *const* methods, and the
/// [`PrimitiveJoin`][super::PrimitiveJoin] trait for the opposite operations.
pub trait PrimitiveSplit<T, const LEN: usize> {
    /// Splits `self` into an array of `T` in big-endian order.
    #[must_use]
    fn into_array_be(self) -> [T; LEN];
    /// Splits `self` into an array of `T` in little-endian order.
    #[must_use]
    fn into_array_le(self) -> [T; LEN];
    /// Splits `self` into an array of `T` in native-endian order.
    #[must_use]
    fn into_array_ne(self) -> [T; LEN];
}

// Implements the trait methods
macro_rules! impl_into_trait {
    ( $( $P:ident, $T:ident, $LEN:literal );+ $(;)? ) => {
        $( impl_into_trait![@$P, $T, $LEN]; )+
    };
    (@$P:ident, $T:ident, $LEN:literal) => { crate::paste! {
        impl PrimitiveSplit<$T, $LEN> for $P {
            fn into_array_be(self) -> [$T; $LEN] { Cast(self).[<into_ $T _be>]() }
            fn into_array_le(self) -> [$T; $LEN] { Cast(self).[<into_ $T _le>]() }
            fn into_array_ne(self) -> [$T; $LEN] { Cast(self).[<into_ $T _ne>]() }
        }
    }};
}
impl_into_trait![
    u128, u64, 2; u128, u32, 4; u128, u16, 8; u128, u8, 16;
    u64, u32, 2; u64, u16, 4; u64, u8, 8;
    u32, u16, 2; u32, u8, 4;
    u16, u8, 2;
];
