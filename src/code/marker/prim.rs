// devela::num::grain::prim
//
//! Marker traits for primitives.
//
// TOC
// - (trait Sealed)
// - trait Prim
// - trait PrimFitPtr
// - trait PrimIndex

#![expect(private_bounds, reason = "Sealed traits")]

use crate::{NonMaxU8, NonMaxU16, NonMaxU32, NonMaxU64, NonMaxUsize, items};
#[cfg(doc)]
use crate::{PrimCast, PrimFloat, PrimInt, PrimJoin, PrimScalar, PrimSint, PrimSplit, PrimUint};

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
    NonMaxU8, NonMaxU16, NonMaxU32, NonMaxU64, NonMaxUsize,
];

/* primitives */

#[doc = crate::_tags!(code primitive)]
/// Language primitive value types.
#[doc = crate::_doc_meta!{location("code/marker")}]
///
/// See also the related traits:
/// - markers: [`PrimFitPtr`], [`PrimIndex`],
/// - casting: [`PrimCast`], [`PrimJoin`], [`PrimSplit`],
/// - numeric: [`PrimScalar`], [`PrimFloat`], [`PrimInt`], [`PrimSint`], [`PrimUint`].
///
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
/// Primitive value types that fit in pointer-width on supported Rust targets.
#[doc = crate::_doc_meta!{location("code/marker")}]
#[doc(alias = "PrimitiveFitPtr")]
pub trait PrimFitPtr: Prim {}

_impl_prim![PrimFitPtr for (), bool, u8, i8, usize, isize];
#[cfg(target_pointer_width = "16")]
_impl_prim![PrimFitPtr for bool, u16, i16];
#[cfg(target_pointer_width = "32")]
_impl_prim![PrimFitPtr for u16, u32, i16, i32, f32];
#[cfg(target_pointer_width = "64")]
_impl_prim![PrimFitPtr for u16, u32, u64, i16, i32, i64, f32, f64];

#[doc = crate::_tags!(code mem num primitive)]
/// Primitive types that can be used for indexing.
#[doc = crate::_doc_meta!{location("code/marker")}]
#[doc(alias = "PrimitiveIndex")]
pub trait PrimIndex: crate::PrimUint + PrimFitPtr {}

_impl_prim![PrimIndex for u8, usize];
#[cfg(target_pointer_width = "16")]
_impl_prim![PrimIndex for u16];
#[cfg(target_pointer_width = "32")]
_impl_prim![PrimIndex for u16, u32];
#[cfg(target_pointer_width = "64")]
_impl_prim![PrimIndex for u16, u32, u64];

#[doc = crate::_tags!(code mem num primitive niche)]
/// A representation accepted for compact contiguous indexing.
#[doc = crate::_doc_meta!{location("code/marker")}]
///
/// This trait marks types whose value domain is suitable for representing
/// non-negative indices, including primitive index integers and selected
/// niche-backed wrappers such as `NonMaxU*`.
///
/// Unlike [`PrimIndex`], this trait may be implemented for wrapper types.
/// Its [`Prim`][Self::Prim] associated type names the primitive integer carrier.
#[doc(alias = "IndexRepresentation")]
#[doc(alias = "PrimitiveIndexRepresentation")]
pub trait IndexRepr: Copy {
    /// The primitive integer backing this index representation.
    type Prim: PrimIndex;
}
items! {
    impl IndexRepr for u8 { type Prim = u8; }
    impl IndexRepr for usize { type Prim = u8; }
    impl IndexRepr for NonMaxU8 { type Prim = u8; }
    impl IndexRepr for NonMaxUsize { type Prim = usize; }
    #[cfg(target_pointer_width = "16")]
    items! {
        impl IndexRepr for u16 { type Prim = u16; }
        impl IndexRepr for NonMaxU16 { type Prim = u16; }
    }
    #[cfg(target_pointer_width = "32")]
    items! {
        impl IndexRepr for u16 { type Prim = u16; }
        impl IndexRepr for u32 { type Prim = u32; }
        impl IndexRepr for NonMaxU16 { type Prim = u16; }
        impl IndexRepr for NonMaxU32 { type Prim = u32; }
    }
    #[cfg(target_pointer_width = "64")]
    items! {
        impl IndexRepr for u16 { type Prim = u16; }
        impl IndexRepr for u32 { type Prim = u32; }
        impl IndexRepr for u64 { type Prim = u64; }
        impl IndexRepr for NonMaxU16 { type Prim = u16; }
        impl IndexRepr for NonMaxU32 { type Prim = u32; }
        impl IndexRepr for NonMaxU64 { type Prim = u64; }
    }
}
