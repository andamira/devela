// devela_base_core::num::grain::prim
//
//! Marker traits for primitive scalars.
//
// TOC
// - trait PrimScalar
// - trait PrimInt
// - trait PrimSint
// - trait PrimUint
// - trait PrimFloat

macro_rules! impl_prim {
    ($trait:ident for $($P:ty),+ $(,)?) => { $( impl_prim![% $trait for $P]; )+ };
    (%$trait:ident for $P:ty) => { impl $trait for $P {} };
}

/* scalars */

#[doc = crate::_tags!(code num)]
/// Primitive scalars, both integers and floating-point numbers.
#[doc = crate::_doc_location!("num/grain")]
pub trait PrimScalar: crate::Prim {}
impl_prim![PrimScalar for
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64,
];

/* integers */

#[doc = crate::_tags!(code num)]
/// Primitive integer numbers.
#[doc = crate::_doc_location!("num/grain")]
#[doc(alias = "PrimInteger")]
pub trait PrimInt: PrimScalar {}
impl_prim![PrimInt for u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize];

#[doc = crate::_tags!(code num)]
/// Signed primitive integer numbers.
#[doc = crate::_doc_location!("num/grain")]
#[doc(alias = "PrimSignedInteger")]
pub trait PrimSint: PrimInt {}
impl_prim![PrimSint for i8, i16, i32, i64, i128, isize];

#[doc = crate::_tags!(code num)]
/// Unsigned primitive integer numbers.
#[doc = crate::_doc_location!("num/grain")]
#[doc(alias = "PrimUnsignedInteger")]
pub trait PrimUint: PrimInt {}
impl_prim![PrimUint for u8, u16, u32, u64, u128, usize];

/* floating-point */

#[doc = crate::_tags!(code num)]
/// Primitive floating-point numbers.
#[doc = crate::_doc_location!("num/grain")]
pub trait PrimFloat: PrimScalar {}
impl_prim![PrimFloat for f32, f64];
