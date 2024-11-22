// devela::num::float::alias

#![allow(non_camel_case_types, unused)]

use crate::paste;

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

/* [up|down]casted [i|u]size aliases */

macro_rules! iusize_alias {
    ($casted:literal, $sign:literal, $ty:ident, $($width:literal : $cast:ty),+) => { paste! {
        $(
            #[doc = "An alias for " $casted " pointer-sized " $sign " integer primitive."]
            #[cfg(target_pointer_width = $width)]
            pub type $ty = $cast;
        )+
    }}
}
iusize_alias!["an upcasted", "signed", isize_up, "8":i16, "16":i32, "32":i64, "64":i128];
iusize_alias!["an upcasted", "unsigned", usize_up, "8":u16, "16":u32, "32":u64, "64":u128];
iusize_alias!["a downcasted", "signed", isize_down, "16":i8, "32":i16, "64":i32, "128":i64];
iusize_alias!["a downcasted", "unsigned", usize_down, "16":u8, "32":u16, "64":u32, "128":u64];
