// devela::color::fns
//

#[cfg(feature = "libm")]
use crate::num::FloatExt;

/// Applies the `gamma` to an `f32` channel.
#[inline]
#[cfg(any(feature = "std", feature = "libm"))]
#[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
pub fn linearize32(nonlinear: f32, gamma: f32) -> f32 {
    if nonlinear >= 0.04045 {
        return ((nonlinear + 0.055) / (1. + 0.055)).powf(gamma);
    } else {
        nonlinear / 12.92
    }
}

/// Removes the `gamma` from an `f32` channel.
#[inline]
#[cfg(any(feature = "std", feature = "libm"))]
#[cfg_attr(feature = "nightly", doc(cfg(any(feature = "std", feature = "libm"))))]
pub fn nonlinearize32(linear: f32, gamma: f32) -> f32 {
    if linear >= 0.0031308 {
        return (1.055) * linear.powf(1.0 / gamma) - 0.055;
    } else {
        12.92 * linear
    }
}
