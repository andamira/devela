// devela/src/num/grain/prim/scalar.rs
//
//! Primitive scalar classification traits.
//
// TOC
// - trait PrimScalar
// - trait PrimInt
// - trait PrimSint
// - trait PrimUint
// - trait PrimFloat
// - trait PrimIndex

#![expect(private_bounds, reason = "Sealed traits")]

use crate::{Prim, PrimFitPtr};

macro_rules! _impl_prim {
    ($trait:ident for $($P:ty),+ $(,)?) => { $( _impl_prim![% $trait for $P]; )+ };
    (%$trait:ident for $P:ty) => { impl $trait for $P {} };
}

/// Marker trait to prevent downstream implementations of the `Prim*` traits.
trait Sealed {}

_impl_prim![Sealed for
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64,
];

/* scalars */

#[doc = crate::_tags!(num primitive)]
/// Primitive scalars, both integers and floating-point numbers.
#[doc = crate::_doc_meta!{
    location("num/grain/prim", trait PrimScalar),
}]
#[doc(alias = "PrimitiveScalar")]
pub trait PrimScalar: Sealed + Prim {}

_impl_prim![PrimScalar for
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64,
];

/* integers */

#[doc = crate::_tags!(num primitive)]
/// Primitive integer numbers.
#[doc = crate::_doc_meta!{
    location("num/grain/prim", trait PrimInt),
}]
#[doc(alias = "PrimInteger")]
#[doc(alias = "PrimitiveInteger")]
pub trait PrimInt: PrimScalar {}

_impl_prim![PrimInt for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];

#[doc = crate::_tags!(num primitive)]
/// Signed primitive integer numbers.
#[doc = crate::_doc_meta!{
    location("num/grain/prim", trait PrimSint),
}]
#[doc(alias = "PrimSignedInteger")]
#[doc(alias = "PrimitiveSignedInteger")]
pub trait PrimSint: PrimInt {}

_impl_prim![PrimSint for i8, i16, i32, i64, i128, isize];

#[doc = crate::_tags!(num primitive)]
/// Unsigned primitive integer numbers.
#[doc = crate::_doc_meta!{
    location("num/grain/prim", trait PrimUint),
}]
#[doc(alias = "PrimUnsignedInteger")]
#[doc(alias = "PrimitiveUnsignedInteger")]
pub trait PrimUint: PrimInt {}

_impl_prim![PrimUint for u8, u16, u32, u64, u128, usize];

/* floating-point */

#[doc = crate::_tags!(num primitive)]
/// Primitive floating-point numbers.
#[doc = crate::_doc_meta!{
    location("num/grain/prim", trait PrimFloat),
}]
#[doc(alias = "PrimitiveFloat")]
pub trait PrimFloat: PrimScalar {}

_impl_prim![PrimFloat for f32, f64];

/* index */

#[doc = crate::_tags!(mem num primitive)]
/// Unsigned primitive integers suitable for machine-addressable indexing.
#[doc = crate::_doc_meta!{
    location("num/grain/prim", trait PrimIndex),
}]
#[doc(alias = "PrimitiveIndex")]
pub trait PrimIndex: PrimUint + PrimFitPtr {}

_impl_prim![PrimIndex for u8, usize];
#[cfg(target_pointer_width = "16")]
_impl_prim![PrimIndex for u16];
#[cfg(target_pointer_width = "32")]
_impl_prim![PrimIndex for u16, u32];
#[cfg(target_pointer_width = "64")]
_impl_prim![PrimIndex for u16, u32, u64];
