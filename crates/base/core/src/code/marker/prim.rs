// devela_base_core::num::grain::prim
//
//! Marker traits for primitive scalars.
//
// TOC
// - (trait Sealed)
// - trait Prim
// - trait PrimFitPtr

#![expect(private_bounds, reason = "Sealed traits")]

macro_rules! impl_prim {
    ($trait:ident for $($P:ty),+ $(,)?) => { $( impl_prim![% $trait for $P]; )+ };
    (%$trait:ident for $P:ty) => { impl $trait for $P {} };
}

/// Marker trait to prevent downstream implementations of the `Prim*` traits.
trait Sealed {}
impl_prim![Sealed for
    (), bool, char,
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64,
];

/* primitives */

#[doc = crate::_tags!(code)]
/// Language primitive value types.
#[doc = crate::_doc_location!("code/marker")]
pub trait Prim: Sealed + Copy + 'static {}
impl_prim![Prim for
    (), bool, char,
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize,
    f32, f64,
];

/* pointer-width sized */

#[doc = crate::_tags!(code mem)]
/// Primitive value types that fit in pointer-width on supported Rust targets.
#[doc = crate::_doc_location!("code/marker")]
pub trait PrimFitPtr: Prim {}

impl_prim![PrimFitPtr for (), bool, u8, i8, usize, isize];
#[cfg(target_pointer_width = "16")]
impl_prim![PrimFitPtr for bool, u16, i16];
#[cfg(target_pointer_width = "32")]
impl_prim![PrimFitPtr for u16, u32, i16, i32, f32];
#[cfg(target_pointer_width = "64")]
impl_prim![PrimFitPtr for u16, u32, u64, i16, i32, i64, f32, f64];
