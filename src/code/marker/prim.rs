// devela/src/code/marker/prim.rs
//
//! Marker traits for primitives.
//
// TOC
// - (trait Sealed)
// - trait Prim
// - trait PrimFitPtr

#![expect(private_bounds, reason = "Sealed traits")]

#[cfg(doc)]
use crate::{
    PrimCast, PrimFloat, PrimIndex, PrimInt, PrimJoin, PrimScalar, PrimSint, PrimSplit, PrimUint,
};

macro_rules! _impl_prim {
    ($trait:ident for $($P:ty),+ $(,)?) => { $( _impl_prim![% $trait for $P]; )+ };
    (%$trait:ident for $P:ty) => { impl $trait for $P {} };
}

/// Marker trait to prevent downstream implementations of the `Prim*` traits.
trait Sealed {}
_impl_prim![Sealed for
    (), bool, char,
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64,
];

/* primitives */

#[doc = crate::_tags!(code primitive)]
/// Primitive value types recognized by devela.
#[doc = crate::_doc_meta!{
    location("code/marker", trait Prim),
}]
/// This sealed trait covers `()`, `bool`, `char`, primitive integers,
/// and primitive floating-point types.
///
/// See also the related primitive traits:
/// - target width: [`PrimFitPtr`],
/// - numeric classification: [`PrimScalar`], [`PrimInt`], [`PrimSint`],
///   [`PrimUint`], [`PrimFloat`], [`PrimIndex`],
/// - casting and composition: [`PrimCast`], [`PrimJoin`], [`PrimSplit`].
#[doc(alias = "Primitive")]
pub trait Prim: Sealed + Copy + 'static {}
_impl_prim![Prim for
    (), bool, char,
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64,
];

/* pointer-width related */

#[doc = crate::_tags!(code mem primitive)]
/// Primitive value types whose size does not exceed the target pointer width.
#[doc = crate::_doc_meta!{
    location("code/marker", trait PrimFitPtr),
}]
#[doc(alias = "PrimitiveFitPtr")]
pub trait PrimFitPtr: Prim {}

_impl_prim![PrimFitPtr for (), bool, u8, i8, usize, isize];
#[cfg(target_pointer_width = "16")]
_impl_prim![PrimFitPtr for u16, i16];
#[cfg(target_pointer_width = "32")]
_impl_prim![PrimFitPtr for u16, u32, i16, i32, f32, char];
#[cfg(target_pointer_width = "64")]
_impl_prim![PrimFitPtr for u16, u32, u64, i16, i32, i64, f32, f64, char];
