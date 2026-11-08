//
//! Primitive-backed integer representation traits.
//
// TOC
// - trait ReprInt
// - trait ReprSint
// - trait ReprUint
// - trait ReprIndex

#![expect(
    private_bounds,
    reason = "private traits seal Repr* implementations and constrain supported BittenU* capacities"
)]

use crate::{_BittenSpec, _BittenU8Select, BittenU8};
#[cfg(feature = "bitten")]
use crate::{_BittenU16Select, _BittenU32Select, BittenU16, BittenU32};
use crate::{NonMaxU8, NonMaxU16, NonMaxU32, NonMaxU64, NonMaxUsize};
use crate::{NonValueI8, NonValueI16, NonValueI32, NonValueI64, NonValueI128, NonValueIsize};
use crate::{NonValueU8, NonValueU16, NonValueU32, NonValueU64, NonValueU128, NonValueUsize};
use crate::{NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize};
use crate::{NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize};
use crate::{PrimIndex, PrimInt, PrimSint, PrimUint};

macro_rules! _impl_prim {
    ($trait:ident for $($P:ty),+ $(,)?) => { $( _impl_prim![% $trait for $P]; )+ };
    (%$trait:ident for $P:ty) => { impl $trait for $P {} };
}
macro_rules! _impl_nv {
    ($trait:ident for $($P:ident<$V:ident>),+ $(,)?) => { $( _impl_nv![% $trait for $P<$V>]; )+ };
    (%$trait:ident for $P:ident<$V:ident>) => { impl<const V: $V> $trait for $P<V> {} };
}

/// Marker trait to prevent downstream implementations of the `Repr*` traits.
trait Sealed {}

_impl_prim![Sealed for
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize,
    NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize,
];
_impl_nv![Sealed for
    NonValueI8<i8>, NonValueI16<i16>, NonValueI32<i32>, NonValueI64<i64>,
    NonValueI128<i128>, NonValueIsize<isize>,
    NonValueU8<u8>, NonValueU16<u16>, NonValueU32<u32>, NonValueU64<u64>,
    NonValueU128<u128>, NonValueUsize<usize>,
];

#[doc = crate::_tags!(num primitive niche)]
/// Integer representations associated with a primitive integer carrier.
#[doc = crate::_doc_meta!{
    location("num/grain/prim", trait ReprInt),
}]
/// This sealed trait identifies integer representations recognized by devela
/// and exposes their primitive carrier through [`Prim`][Self::Prim].
///
/// Implementations include primitive integers and selected niche-backed types.
///
/// `VALUE_BITS` describes numeric value width,
/// not storage size or the number of valid representations.
///
/// Excluding isolated values does not reduce this width: for example,
/// `NonZeroU32` and `NonMaxU32` retain 32 value bits. Representations
/// such as `BittenU32<4>` instead reduce the numeric capacity itself
/// and therefore report 28 value bits.
#[doc(alias = "IntegerRepresentation")]
pub trait ReprInt: Sealed + Copy + 'static {
    /// The primitive integer carrier of this representation.
    type Prim: PrimInt;

    /// Bits of numeric value capacity represented by this type.
    const VALUE_BITS: u32;
}

macro_rules! impl_repr_int_prim {
    ($($for:ty, $prim:ty, $bits:expr);+ $(;)?) => {
        $( impl_repr_int_prim! { % $for, $prim, $bits } )+
    };
    (% $for:ty, $prim:ty, $bits:expr) => {
        impl ReprInt for $for {
            type Prim = $prim;
            const VALUE_BITS: u32 = $bits;
        }
    };
}
impl_repr_int_prim! {
    i8, i8, 8;
    i16, i16, 16;
    i32, i32, 32;
    i64, i64, 64;
    i128, i128, 128;
    isize, isize, isize::BITS;
    u8, u8, 8;
    u16, u16, 16;
    u32, u32, 32;
    u64, u64, 64;
    u128, u128, 128;
    usize, usize, usize::BITS;
    NonZeroI8, i8, 8;
    NonZeroI16, i16, 16;
    NonZeroI32, i32, 32;
    NonZeroI64, i64, 64;
    NonZeroI128, i128, 128;
    NonZeroIsize, isize, isize::BITS;
    NonZeroU8, u8, 8;
    NonZeroU16, u16, 16;
    NonZeroU32, u32, 32;
    NonZeroU64, u64, 64;
    NonZeroU128, u128, 128;
    NonZeroUsize, usize, usize::BITS;
}

macro_rules! impl_repr_int_nv {
    ($($for:ident, $prim:ty, $bits:expr);+ $(;)?) => {
        $( impl_repr_int_nv! { % $for, $prim, $bits } )+
    };
    (% $for:ident, $prim:ty, $bits:expr) => {
        impl<const V: $prim> ReprInt for $for<V> {
            type Prim = $prim;
            const VALUE_BITS: u32 = $bits;
        }
    };
}
impl_repr_int_nv! {
    NonValueI8, i8, 8;
    NonValueI16, i16, 16;
    NonValueI32, i32, 32;
    NonValueI64, i64, 64;
    NonValueI128, i128, 128;
    NonValueIsize, isize, isize::BITS;
    NonValueU8, u8, 8;
    NonValueU16, u16, 16;
    NonValueU32, u32, 32;
    NonValueU64, u64, 64;
    NonValueU128, u128, 128;
    NonValueUsize, usize, usize::BITS;
}

macro_rules! _impl_bitten_repr {
    ($T:ident, $Select:ident, $Prim:ty) => {
        impl<const B: u8> Sealed for $T<B> where $Select<B>: _BittenSpec {}
        impl<const B: u8> ReprInt for $T<B>
        where
            $Select<B>: _BittenSpec,
        {
            type Prim = $Prim;
            const VALUE_BITS: u32 = <$Prim>::BITS - B as u32;
        }
    };
}
macro_rules! _impl_bitten_index {
    ($T:ident, $Select:ident) => {
        impl<const B: u8> ReprIndex for $T<B> where $Select<B>: _BittenSpec {}
    };
}
_impl_bitten_repr!(BittenU8, _BittenU8Select, u8);
#[cfg(feature = "bitten")]
_impl_bitten_repr!(BittenU16, _BittenU16Select, u16);
#[cfg(feature = "bitten")]
_impl_bitten_repr!(BittenU32, _BittenU32Select, u32);

_impl_bitten_index!(BittenU8, _BittenU8Select);
#[cfg(feature = "bitten")]
#[cfg(any(target_pointer_width = "16", target_pointer_width = "32", target_pointer_width = "64"))]
_impl_bitten_index!(BittenU16, _BittenU16Select);
#[cfg(feature = "bitten")]
#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
_impl_bitten_index!(BittenU32, _BittenU32Select);

#[doc = crate::_tags!(num primitive niche)]
/// Integer representations with a signed primitive integer carrier.
#[doc = crate::_doc_meta!{
    location("num/grain/prim", trait ReprSint),
}]
#[doc(alias = "SignedIntegerRepresentation")]
pub trait ReprSint: ReprInt<Prim: PrimSint> {}

impl<R> ReprSint for R
where
    R: ReprInt,
    R::Prim: PrimSint,
{
}

#[doc = crate::_tags!(num primitive niche)]
/// Integer representations with an unsigned primitive integer carrier.
#[doc = crate::_doc_meta!{
    location("num/grain/prim", trait ReprUint),
}]
#[doc(alias = "UnsignedIntegerRepresentation")]
pub trait ReprUint: ReprInt<Prim: PrimUint> {}

impl<R> ReprUint for R
where
    R: ReprInt,
    R::Prim: PrimUint,
{
}

#[doc = crate::_tags!(mem num primitive niche)]
/// Unsigned integer representations suitable for contiguous indexing.
#[doc = crate::_doc_meta!{
    location("num/grain/prim", trait ReprIndex),
}]
/// The represented domain is non-negative, starts at zero, and is contiguous,
/// while its primitive carrier is suitable for machine-addressable indexing.
///
/// Implementations include primitive index integers and selected
/// niche-backed representations such as `NonMaxU*` and the
/// [`BittenU8`], [`BittenU16`], [`BittenU32`] family.
///
/// See also [`PrimIndex`], which classifies the primitive carriers themselves.
#[doc(alias = "IndexRepresentation")]
#[doc(alias = "PrimitiveIndexRepresentation")]
pub trait ReprIndex: ReprUint + ReprInt<Prim: PrimIndex> {}

_impl_prim![ReprIndex for u8, usize, NonMaxU8, NonMaxU16, NonMaxUsize];

#[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
_impl_prim![ReprIndex for u32, NonMaxU32];

#[cfg(target_pointer_width = "64")]
_impl_prim![ReprIndex for u64, NonMaxU64];
