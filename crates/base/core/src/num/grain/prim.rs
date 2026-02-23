// devela_base_core::num::grain::prim
//
//! Marker traits for primitive scalars.
//
// TOC
// - (trait Sealed)
// - trait PrimScalar
// - trait PrimFitPtr
// - trait PrimInt
// - trait PrimSint
// - trait PrimUint
// - trait PrimFloat

#![expect(private_bounds, reason = "Sealed traits")]

macro_rules! impl_prim {
    ($trait:ident for $($P:ty),+ $(,)?) => { $( impl_prim![% $trait for $P]; )+ };
    (%$trait:ident for $P:ty) => { impl $trait for $P {} };
}

/// Marker trait to prevent downstream implementations of the `Prim*` traits.
trait Sealed {}
impl_prim![Sealed for
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64,
];

/* scalars */

#[doc = crate::_tags!(num)]
/// Primitive scalars, both integers and floating-point numbers.
#[doc = crate::_doc_location!("num/grain")]
pub trait PrimScalar: Sealed + Copy + 'static {}
impl_prim![PrimScalar for
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64,
];

/* pointer-width sized */

#[doc = crate::_tags!(num)]
/// Primitive scalars with a pointer-width size or less.
#[doc = crate::_doc_location!("num/grain")]
pub trait PrimFitPtr: PrimScalar {}

impl_prim![PrimFitPtr for usize, isize];
#[cfg(target_pointer_width = "8")]
impl_prim![PrimFitPtr for u8, i8];
#[cfg(target_pointer_width = "16")]
impl_prim![PrimFitPtr for u8, u16, i8, i16];
#[cfg(target_pointer_width = "32")]
impl_prim![PrimFitPtr for u8, u16, u32, i8, i16, i32, f32];
#[cfg(target_pointer_width = "64")]
impl_prim![PrimFitPtr for u8, u16, u32, u64, i8, i16, i32, i64, f32, f64];

/* integers */

#[doc = crate::_tags!(num)]
/// Primitive integer numbers.
#[doc = crate::_doc_location!("num/grain")]
#[doc(alias = "PrimInteger")]
pub trait PrimInt: PrimScalar {}
impl_prim![PrimInt for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];

#[doc = crate::_tags!(num)]
/// Signed primitive integer numbers.
#[doc = crate::_doc_location!("num/grain")]
#[doc(alias = "PrimSignedInteger")]
pub trait PrimSint: PrimInt {}
impl_prim![PrimSint for i8, i16, i32, i64, i128, isize];

#[doc = crate::_tags!(num)]
/// Unsigned primitive integer numbers.
#[doc = crate::_doc_location!("num/grain")]
#[doc(alias = "PrimUnsignedInteger")]
pub trait PrimUint: PrimInt {}
impl_prim![PrimUint for u8, u16, u32, u64, u128, usize];

/* floating-point */

#[doc = crate::_tags!(num)]
/// Primitive floating-point numbers.
#[doc = crate::_doc_location!("num/grain")]
pub trait PrimFloat: PrimScalar {}
impl_prim![PrimFloat for f32, f64];
