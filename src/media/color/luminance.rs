// devela::media::color::luminance
//
//! Defines the [`Lum`] type and aliases:
//! [`Luminance`], [`Lightness`], [`Luma`], `LinearLightness`.
//

use super::*;
use crate::NumConst;
#[cfg(any(feature = "std", _float··))]
use crate::is;
#[cfg(_float··)]
use crate::{ExtFloat, Float};

/// A generic luminance-like component.
///
/// Represents either physical luminance, gamma-encoded luma, or perceptual lightness,
/// depending on the `LINEAR` and `LIGHTNESS` flags.
///
/// Variants (in order of typical usage):
/// - [`Luminance<T>`]:        linear, physical
/// - [`Lightness<T>`]:        non-linear, perceptual
/// - [`Luma<T>`]:             non-linear, technical
/// - [`LinearLightness<T>`]:  linear, perceptual (experimental hybrid)
#[must_use]
#[repr(C)]
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Lum<T, const LINEAR: bool = true, const LIGHTNESS: bool = false> {
    /// The luminance-like channel value.
    pub c: [T; 1],
}

/* aliases*/

/// Physical [luminance].
///
/// Linear light intensity, measured in cd/m² or normalized to [0.0, 1.0].
///
/// [luminance]: https://en.wikipedia.org/wiki/Luminance
pub type Luminance<T> = Lum<T, true, false>;

/// Perceptual [lightness] (L*).
///
/// Non-linear encoding of luminance,
/// normalized to [0.0, 1.0] for floats or 0..=MAX for integers.
///
/// [lightness]: https://en.wikipedia.org/wiki/Lightness
pub type Lightness<T> = Lum<T, false, true>;

/// Gamma-encoded [luma] (Y′).
///
/// A non-linear approximation of luminance, typically used in video systems.
///
/// [luma]: https://en.wikipedia.org/wiki/Luma_(video)
pub type Luma<T> = Lum<T, false, false>;

/// Linearized perceptual lightness (L* in linear space).
///
/// Use cases include:
/// - Combining linear luminance (for precise computations)
///   and perceptual lightness (for display scaling).
/// - Tone mapping in HDR imaging, where linear data is scaled to a perceptual range.
/// - Representing raw radiometric data (e.g., watts/sr/m²) prior to photometric weighting (CIE Y).
pub type LinearLightness<T> = Lum<T, true, true>;

/// ## Args
/// `$T`   : the type used to represent the main value. (u8, u16, f32, f64)
/// `$f`   : associated floating-point type for operations. (f32|f64)
/// `$B`   : float-related feature bound. ("_float_f32"|"_float_f64")
/// `$BITS`: the number of bits of each inner component.
/// `$INT` : a boolean indicating whether the components are integers.
macro_rules! impl_lum {
    () => {
        impl_lum![common u8|f32, u16|f32];
        #[cfg(feature = "_float_f32")] impl_lum![common f32|f32];
        #[cfg(feature = "_float_f64")] impl_lum![common f64|f64];

        impl_lum![lumina u8|f32:"_float_f32":8+true, u16|f32:"_float_f32":16+true];
        #[cfg(feature = "_float_f32")]impl_lum![lumina f32|f32:"_float_f32":32+false];
        #[cfg(feature = "_float_f64")]impl_lum![lumina f64|f64:"_float_f64":64+false];

        impl_lum![light u8|f32:"_float_f32":8+true, u16|f32:"_float_f32":16+true];
        #[cfg(feature = "_float_f32")]impl_lum![light f32|f32:"_float_f32":32+false];
        #[cfg(feature = "_float_f64")]impl_lum![light f64|f64:"_float_f64":64+false];

        impl_lum![luma u8|f32:"_float_f32":8+true, u16|f32:"_float_f32":16+true];
        #[cfg(feature = "_float_f32")]impl_lum![luma f32|f32:"_float_f32":32+false];
        #[cfg(feature = "_float_f64")]impl_lum![luma f64|f64:"_float_f64":64+false];

        impl_lum![lumi_light u8|f32:"_float_f32":8+true, u16|f32:"_float_f32":16+true];
        #[cfg(feature = "_float_f32")]impl_lum![lumi_light f32|f32:"_float_f32":32+false];
        #[cfg(feature = "_float_f64")]impl_lum![lumi_light f64|f64:"_float_f64":64+false];
    };
    ( // Methods common to all types.
      common  $( $T:ty | $f:ty ),+) => { $( impl_lum![@common $T|$f]; )+ };
    (@common     $T:ty | $f:ty    ) => {
        impl<const LINEAR: bool, const LIGHTNESS: bool> Lum<$T, LINEAR, LIGHTNESS> {
            /// New `Luminance` with the given channel.
            pub const fn new(c: $T) -> Self { Self { c: [c] } }

            /// Returns the raw channel value, regardless of interpretation.
            ///
            /// Prefer type-specific methods like [`luminance()`](Luminance::luminance) or
            /// [`lightness()`](Lightness::lightness) where possible.
            pub const fn l(self) -> $T { self.c[0] }

            /// Returns a mutable reference to the raw channel value.
            pub const fn l_mut(&mut self) -> &mut $T { &mut self.c[0] }

            /// Converts an `Rgb` into unweighted brightness by averaging the R'G'B' components.
            ///
            /// May be useful for quick approximations.
            /// Not correct for perceptual brightness (luma) or physical light (luminance).
            pub const fn brightness_from_rgb(rgb: Rgb<$T>) -> Self {
                Self::new((rgb.r() + rgb.g() + rgb.b()) / <$T as NumConst>::NUM_THREE.unwrap())
            }
        }
    };
    ( // Methods for Luminance: (linear non-lightness)
      lumina $( $T:ty | $f:ty : $B:literal : $BITS:literal + $INT:literal),+) => {
        $( impl_lum![@lumina $T|$f:$B : $BITS+$INT]; )+
    };
    (@lumina    $T:ty | $f:ty : $B:literal : $BITS:literal + $INT:literal   ) => {
        impl_color![lum: $T, $BITS, $INT, true, false];

        impl Luminance<$T> {
            /// Returns the **linear luminance** (physical light intensity, Y).
            ///
            /// Measured in cd/m² (floats) or normalized (integers).
            pub const fn luminance(self) -> $T { self.c[0] }
            /// Returns a mutable reference to the **linear luminance**.
            pub const fn luminance_mut(&mut self) -> &mut $T { &mut self.c[0] }

            /* gamma conversion */
        }
    };
    ( // Methods for Lightness: (non-linear, lightness)
      light $( $T:ty | $f:ty : $B:literal : $BITS:literal + $INT:literal),+) => {
        $( impl_lum![@light $T|$f:$B : $BITS+$INT]; )+
    };
    (@light    $T:ty | $f:ty : $B:literal : $BITS:literal + $INT:literal   ) => {
        impl_color![lum: $T, $BITS, $INT, false, true];

        impl Lightness<$T> {
            /// Returns the **perceptual lightness** (CIE L\*).
            ///
            /// Normalized to `0.0..=1.0` (floats) or `0..=MAX` (integers).
            pub const fn lightness(self) -> $T { self.c[0] }
            /// Returns a mutable reference to the **perceptual lightness**.
            pub const fn lightness_mut(&mut self) -> &mut $T { &mut self.c[0] }

            /* gamma conversion */
        }
    };
    ( // Methods for Luma: (non-linear, non-lightness)
      luma $( $T:ty | $f:ty : $B:literal : $BITS:literal + $INT:literal),+) => {
        $( impl_lum![@luma $T|$f:$B : $BITS+$INT]; )+
    };
    (@luma    $T:ty | $f:ty : $B:literal : $BITS:literal + $INT:literal    ) => {
        impl_color![lum: $T, $BITS, $INT, false, false];

        impl Luma<$T> {
            /// Returns the **gamma-encoded luma** (non-linear Y′).
            ///
            /// Compatible with sRGB/Rec. 709 for display.
            pub const fn lightness(self) -> $T { self.c[0] }
            /// Returns a mutable reference to the **gamma-encoded luma**.
            pub const fn lightness_mut(&mut self) -> &mut $T { &mut self.c[0] }

            /* gamma conversion */

        }
    };
    ( // Methods for LinearLuminance: (linear, lightness)
      lumi_light $( $T:ty | $f:ty : $B:literal : $BITS:literal + $INT:literal),+) => {
          $( impl_lum![@lumi_light $T|$f:$B : $BITS+$INT]; )+
    };
    (@lumi_light    $T:ty | $f:ty : $B:literal : $BITS:literal + $INT:literal   ) => {
        impl_color![lum: $T, $BITS, $INT, true, true];

        impl LinearLightness<$T> {
            /// Returns the **linear-light perceptual** value (experimental).
            ///
            /// Used for hybrid workflows like HDR tonemapping.
            pub const fn linear_lightness(self) -> $T { self.c[0] }
            /// Returns a mutable reference to the **linear-light perceptual** value.
            pub const fn linear_lightness_mut(&mut self) -> &mut $T { &mut self.c[0] }

            /* gamma conversion */
        }
    };
}
use impl_lum;
impl_lum!();
