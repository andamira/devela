// devela::num::grain::cast::traits::split
//
//! fns to split a primitive into an array of smaller primitives.
//
// TOC
// - trait PrimSplit
// - macro _num_grain_cast_trait_split_impl_into!

use crate::Cast;

#[doc = crate::_tags!(num)]
/// Offers methods to split a primitive into an array of smaller primitives.
#[doc = crate::_doc_location!("num/grain")]
///
/// See also the [`Cast`] type for the equivalent *const* methods, and the
/// [`PrimJoin`][super::PrimJoin] trait for the opposite operations.
#[doc(alias = "PrimitiveSplit")]
pub trait PrimSplit<T, const LEN: usize> {
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
macro_rules! _num_grain_cast_trait_split_impl_into {
    ( $( $P:ident, $T:ident, $LEN:literal );+ $(;)? ) => {
        $( _num_grain_cast_trait_split_impl_into![@$P, $T, $LEN]; )+
    };
    (@$P:ident, $T:ident, $LEN:literal) => { crate::paste! {
        impl PrimSplit<$T, $LEN> for $P {
            fn into_array_be(self) -> [$T; $LEN] { Cast(self).[<into_ $T _be>]() }
            fn into_array_le(self) -> [$T; $LEN] { Cast(self).[<into_ $T _le>]() }
            fn into_array_ne(self) -> [$T; $LEN] { Cast(self).[<into_ $T _ne>]() }
        }
    }};
}
_num_grain_cast_trait_split_impl_into![
    u128, u64, 2; u128, u32, 4; u128, u16, 8; u128, u8, 16;
    u64, u32, 2; u64, u16, 4; u64, u8, 8;
    u32, u16, 2; u32, u8, 4;
    u16, u8, 2;
];
