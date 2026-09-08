// devela/src/num/grain/prim/repr.rs
//
//! Primitive-backed integer representation traits.
//
// TOC
// - trait ReprInt
// - trait ReprSint
// - trait ReprUint
// - trait ReprIndex

#![expect(private_bounds, reason = "Sealed traits")]

use crate::{NonMaxU8, NonMaxU16, NonMaxU32, NonMaxU64, NonMaxU128, NonMaxUsize};
use crate::{NonMinI8, NonMinI16, NonMinI32, NonMinI64, NonMinI128, NonMinIsize};
use crate::{PrimIndex, PrimInt, PrimSint, PrimUint, items};

macro_rules! _impl_prim {
    ($trait:ident for $($P:ty),+ $(,)?) => { $( _impl_prim![% $trait for $P]; )+ };
    (%$trait:ident for $P:ty) => { impl $trait for $P {} };
}

/// Marker trait to prevent downstream implementations of the `Repr*` traits.
trait Sealed {}

_impl_prim![Sealed for
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize,
    NonMinI8, NonMinI16, NonMinI32, NonMinI64, NonMinI128, NonMinIsize,
    NonMaxU8, NonMaxU16, NonMaxU32, NonMaxU64, NonMaxU128, NonMaxUsize,
];

#[doc = crate::_tags!(num primitive niche)]
/// Integer representations backed by a primitive integer.
#[doc = crate::_doc_meta!{
    location("num/grain/prim", trait ReprInt),
}]
/// This sealed trait identifies integer representations recognized by devela
/// and exposes their primitive carrier through [`Prim`][Self::Prim].
///
/// Implementations include primitive integers and selected niche-backed types.
#[doc(alias = "IntegerRepresentation")]
pub trait ReprInt: Sealed + Copy + 'static {
    /// The primitive integer backing this representation.
    type Prim: PrimInt;
}
items! {
    impl ReprInt for i8 { type Prim = i8; }
    impl ReprInt for i16 { type Prim = i16; }
    impl ReprInt for i32 { type Prim = i32; }
    impl ReprInt for i64 { type Prim = i64; }
    impl ReprInt for i128 { type Prim = i128; }
    impl ReprInt for isize { type Prim = isize; }
    impl ReprInt for u8 { type Prim = u8; }
    impl ReprInt for u16 { type Prim = u16; }
    impl ReprInt for u32 { type Prim = u32; }
    impl ReprInt for u64 { type Prim = u64; }
    impl ReprInt for u128 { type Prim = u128; }
    impl ReprInt for usize { type Prim = usize; }
    impl ReprInt for NonMaxU8 { type Prim = u8; }
    impl ReprInt for NonMaxU16 { type Prim = u16; }
    impl ReprInt for NonMaxU32 { type Prim = u32; }
    impl ReprInt for NonMaxU64 { type Prim = u64; }
    impl ReprInt for NonMaxU128 { type Prim = u128; }
    impl ReprInt for NonMaxUsize { type Prim = usize; }
    impl ReprInt for NonMinI8 { type Prim = i8; }
    impl ReprInt for NonMinI16 { type Prim = i16; }
    impl ReprInt for NonMinI32 { type Prim = i32; }
    impl ReprInt for NonMinI64 { type Prim = i64; }
    impl ReprInt for NonMinI128 { type Prim = i128; }
    impl ReprInt for NonMinIsize { type Prim = isize; }
}

#[doc = crate::_tags!(num primitive niche)]
/// Integer representations backed by a signed primitive integer.
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
/// Integer representations backed by an unsigned primitive integer.
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
/// Implementations include primitive index integers and selected niche-backed
/// representations such as `NonMaxU*`.
///
/// See also [`PrimIndex`], which classifies the primitive carriers themselves.
#[doc(alias = "IndexRepresentation")]
#[doc(alias = "PrimitiveIndexRepresentation")]
pub trait ReprIndex: ReprUint + ReprInt<Prim: PrimIndex> {}

_impl_prim![ReprIndex for u8, usize, NonMaxU8, NonMaxUsize];
#[cfg(target_pointer_width = "16")]
_impl_prim![ReprIndex for u16, NonMaxU16];
#[cfg(target_pointer_width = "32")]
_impl_prim![ReprIndex for u16, u32, NonMaxU16, NonMaxU32];
#[cfg(target_pointer_width = "64")]
_impl_prim![ReprIndex for u16, u32, u64, NonMaxU16, NonMaxU32, NonMaxU64];
