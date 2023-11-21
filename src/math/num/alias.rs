// devela::math::num::alias

/// An alias for a pointer-sized floating-point primitive.
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(target_pointer_width = "32", target_pointer_width = "64")))
)]
#[cfg(target_pointer_width = "32")]
#[allow(non_camel_case_types, unused)]
pub type fsize = f32;

/// An alias for a pointer-sized floating-point primitive.
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(target_pointer_width = "32", target_pointer_width = "64")))
)]
#[cfg(target_pointer_width = "64")]
#[allow(non_camel_case_types, unused)]
pub type fsize = f64;
