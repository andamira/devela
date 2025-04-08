// devela::media::color::luminance

use super::*;
use crate::NumConst;

/// Luminance representation.
///
/// By default represents linear light.
#[must_use]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Luminance<T, const LINEAR: bool = true> {
    /// Luminosity channel.
    pub c: [T; 1],
}
/// The weighted sum of gamma-compressed R'G'B' components.
pub type Luma<T> = Luminance<T, false>;

crate::CONST! {
    DOC_709 = "Returns the [Rec. 709]  R'G'B' coefficients.\n
[Rec. 709]; https://en.wikipedia.org/wiki/Rec._709";
    DOC_601 = "Returns the [Rec. 601]  R'G'B' coefficients (SDTV).\n
[Rec. 601]; https://en.wikipedia.org/wiki/Rec._601";
}

impl<const LINEAR: bool> Luminance<f32, LINEAR> {
    #[doc = DOC_709!()]
    pub const fn rec_709() -> [f32; 3] { [0.212_639_f32, 0.715_169, 0.072_192] }
    #[doc = DOC_601!()]
    pub const fn rec_601() -> [f32; 3] { [0.299, 0.587, 0.114] }

}
impl<const LINEAR: bool> Luminance<f64, LINEAR> {
    #[doc = DOC_709!()]
    pub const fn rec_709() -> [f64; 3] { [0.212_639_f64, 0.715_169, 0.072_192] }
    #[doc = DOC_601!()]
    pub const fn rec_601() -> [f64; 3] { [0.299, 0.587, 0.114] }
}

macro_rules! impl_luminance {
    () => {
        impl_luminance![common u8, u16];
        // TODO
        // #[cfg(feature = "_float_f32")]
        // impl_luminance![common f32];
        // #[cfg(feature = "_float_f64")]
        // impl_luminance![common f64];

        // impl_luminance![non-linear u8, u16];
        // #[cfg(feature = "_float_f32")]
        // impl_luminance![non-linear f32];
        // #[cfg(feature = "_float_f64")]
        // impl_luminance![non-linear f64];

        // $( impl_luminance![linear u8, u16, f32, f64]; )+ // TODO
    };
    (common $( $T:ty ),+) => { $( impl_luminance![@common $T]; )+ };
    (@common $T:ty) => {
        impl<const LINEAR: bool> ColorBase for Luminance<$T, LINEAR> {
            type Component = $T;
            fn color_component_count(&self) -> usize { 1 }
            fn color_components_write(&self, b: &mut[$T]) { b[0] = self.c[0]; }
        }
        impl<const LINEAR: bool> Luminance<$T, LINEAR> {
            /// New `Luminance` with the given channel.
            pub const fn new(c: $T) -> Self { Self { c: [c] } }

            /// Converts an `Rgb` into unweighted brightness by averaging the R'G'B' components.
            ///
            /// May be useful for quick approximations.
            /// Not correct for perceptual brightness (luma) or physical light (luminance).
            pub const fn brightness_from_rgb(rgb: Rgb<$T>) -> Self {
                todo![]
                // Self::new((rgb.r() + rgb.g() + rgb.b()) / <$T as NumConst>::NUM_THREE)
            }
        }
    };
    (non-linear $( $T:ty ),+) => { $( impl_luminance![@non-linear $T]; )+ };
    (@non-linear $T:ty) => {
        impl Luminance<$T, true> {
            /// The luminosity component.
            pub const fn luminosity(self) -> $T { self.c[0] }
            #[allow(missing_docs)]
            pub const fn l(self) -> $T { self.c[0] }

            /// Convert this `Luminance` into an `Rgb` representation.
            pub const fn to_rgb(self) -> Rgb<$T> { Rgb::<$T>::new(self.l(), self.l(), self.l()) }

            /// Converts a linear `Rgb` into linear `Luminance`.
            pub const fn from_rgb(rgb: Rgb<$T, true>) -> Self {
                let [kr, kg, kb] = Self::rgb_luminance_coeffs();
                Self::new(kr * rgb.r() + kg * rgb.g() + kb * rgb.b())
            }
        }
    };
    (@linear $T:ty) => {
        impl Luma<$T> {
            /// Converts a gamma-encoded `Rgb` into perceptual `Luma`.
            pub const fn from_rgb(rgb: Rgb<$T, false>) -> Self {
                let [kr, kg, kb] = Self::rec_709();
                Self::new(kr * rgb.r() + kg * rgb.g() + kb * rgb.b())
            }
        }
    };
}
use impl_luminance;
impl_luminance!();
