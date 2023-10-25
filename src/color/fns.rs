// devela::color::fns
//

#[cfg(all(feature = "no_std", feature = "dep"))]
use crate::_dep::libm::Libm;

/// Applies the `gamma` to an `f32` channel.
#[inline]
#[cfg(any(feature = "std", all(feature = "no_std", feature = "dep")))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(feature = "std", all(feature = "no_std", feature = "dep"))))
)]
pub fn linearize32(nonlinear: f32, gamma: f32) -> f32 {
    if nonlinear >= 0.04045 {
        #[cfg(feature = "std")]
        return ((nonlinear + 0.055) / (1. + 0.055)).powf(gamma);

        #[cfg(not(feature = "std"))]
        return Libm::<f32>::pow((nonlinear + 0.055) / (1. + 0.055), gamma);
    } else {
        nonlinear / 12.92
    }
}

/// Removes the `gamma` from an `f32` channel.
#[inline]
#[cfg(any(feature = "std", all(feature = "no_std", feature = "dep")))]
#[cfg_attr(
    feature = "nightly",
    doc(cfg(any(feature = "std", all(feature = "no_std", feature = "dep"))))
)]
pub fn nonlinearize32(linear: f32, gamma: f32) -> f32 {
    if linear >= 0.0031308 {
        #[cfg(feature = "std")]
        return (1.055) * linear.powf(1.0 / gamma) - 0.055;

        #[cfg(not(feature = "std"))]
        return (1.055) * Libm::<f32>::pow(linear, 1.0 / gamma) - 0.055;
    } else {
        12.92 * linear
    }
}
