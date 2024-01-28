// devela::num::float::alias

#![allow(non_camel_case_types, unused)]

/// An alias for a pointer-sized floating-point primitive.
///
/// On a 32 bit target, this is 4 bytes and on a 64 bit target, 8 bytes.
#[cfg_attr(
    feature = "nightly_doc",
    doc(cfg(any(target_pointer_width = "32", target_pointer_width = "64")))
)]
#[cfg(target_pointer_width = "32")]
pub type fsize = f32;

/// An alias for a pointer-sized floating-point primitive.
///
/// On a 32 bit target, this is 4 bytes and on a 64 bit target, 8 bytes.
#[cfg_attr(
    feature = "nightly_doc",
    doc(cfg(any(target_pointer_width = "32", target_pointer_width = "64")))
)]
#[cfg(target_pointer_width = "64")]
pub type fsize = f64;

/* upcasted isize|usize aliases */

/// An alias for an upcasted pointer-sized signed integer primitive.
#[cfg(target_pointer_width = "8")]
pub type isize_up = i16;
/// An alias for an upcasted pointer-sized signed integer primitive.
#[cfg(target_pointer_width = "16")]
pub type isize_up = i32;
/// An alias for an upcasted pointer-sized signed integer primitive.
#[cfg(target_pointer_width = "32")]
pub type isize_up = i64;
/// An alias for an upcasted pointer-sized signed integer primitive.
#[cfg(target_pointer_width = "64")]
pub type isize_up = i128;
// /// An alias for an upcasted pointer-sized signed integer primitive.
// #[cfg(target_pointer_width = "128")]
// pub type isize_up = isize;

/// An alias for an upcasted pointer-sized unsigned integer primitive.
#[cfg(target_pointer_width = "8")]
pub type usize_up = u16;
/// An alias for an upcasted pointer-sized unsigned integer primitive.
#[cfg(target_pointer_width = "16")]
pub type usize_up = u32;
/// An alias for an upcasted pointer-sized unsigned integer primitive.
#[cfg(target_pointer_width = "32")]
pub type usize_up = u64;
/// An alias for an upcasted pointer-sized unsigned integer primitive.
#[cfg(target_pointer_width = "64")]
pub type usize_up = u128;
// /// An alias for an upcasted pointer-sized unsigned integer primitive.
// #[cfg(target_pointer_width = "128")]
// pub type usize_up = usize;
