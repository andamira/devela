// devela::media::color::color
//
//! Defines the [`Color`] trait.
//
// TOC
// - trait Color
// - macro impl_color!
// - impl Color

use crate::{
    Rgb8, Rgb16, RgbF32, RgbF64, RgbLinF32, RgbLinF64, Rgba8, Rgba16, RgbaF32, RgbaF64, RgbaLinF32,
    RgbaLinF64, RgbaLinPreF32, RgbaLinPreF64, RgbaPre8, RgbaPre16, RgbaPreF32, RgbaPreF64,
};

use crate::{NotEnoughSpace, NumConst};
#[cfg(feature = "alloc")]
use crate::{Vec, vec_ as vec};

#[doc = crate::_TAG_COLOR!()]
/// Base trait for general color data representation.
#[doc = crate::_doc_location!("media/color")]
///
/// Provides a core interface for working with color data across different
/// formats and models, supporting both practical and scientific applications.
#[rustfmt::skip]
pub trait Color {
    /// The type of a single color component (e.g., `u8`, `f32`).
    type Component: NumConst;

    /// The bit depth of each color component (e.g., `8` for `u8`, `32` for `f32`).
    const COLOR_BITS: usize;

    /// The number of color components (channels) in the representation.
    ///
    /// Examples:
    /// - RGB: `3`
    /// - RGBA: `4`
    /// - Spectral data: Arbitrary `n` (e.g., `31` for 400..=700nm in 10nm steps)
    const COLOR_COUNT: usize;

    /// Whether the color has an alpha component are integer types (e.g., `u8`, `u16`).
    const COLOR_HAS_ALPHA: bool;

    /// Whether the color components are integer types (e.g., `u8`, `u16`).
    ///
    /// If `false`, the components are floating-point (e.g., `f32`, `f64`).
    const COLOR_IS_INT: bool;

    /// Whether the color space is linear (as opposed to non-linear, e.g., sRGB).
    ///
    /// - Linear colors are physically accurate for lighting/blending math.
    /// - Non-linear colors (e.g., sRGB) are gamma-encoded for storage/display.
    const COLOR_IS_LINEAR: bool;

    /// Whether the color uses premultiplied alpha (vs. straight/unassociated alpha).
    ///
    /// - Premultiplied alpha (`true`) means each RGB component is scaled by the alpha value.
    /// - Straight alpha (`false`) means RGB components are independent of alpha.
    ///
    /// Note: Only relevant for alpha-enabled formats (e.g., `COLOR_COUNT > 3`).
    const COLOR_IS_PREMUL: bool;

    /// Writes the color components to a pre-allocated `buffer`.
    ///
    /// # Errors
    /// Returns [`NotEnoughSpace`] if the buffer size is less than `COLOR_COUNT`.
    fn color_components_write(&self, buffer: &mut [Self::Component]) -> Result<(), NotEnoughSpace>;

    /// Get the red component.
    fn color_red(&self) -> Self::Component;

    /// Get the green component.
    fn color_green(&self) -> Self::Component;

    /// Get the blue component.
    fn color_blue(&self) -> Self::Component;

    /// Get the alpha component.
    ///
    /// When the color has no alpha component, it should return the maximum normalized value.
    /// (e.g. 1.0 for floats, and `MAX` value for integers).
    fn color_alpha(&self) -> Self::Component;

    /* non-required */

    #[inline(always)]
    /// Returns the bit depth of each color component (e.g., 8 for `u8`, 32 for `f32`).
    fn color_bits(&self) -> usize { Self::COLOR_BITS }

    #[inline(always)]
    /// Returns the number of color components (channels).
    fn color_count(&self) -> usize { Self::COLOR_COUNT }

    #[inline(always)]
    /// Returns `true` if the color has an alpha component.
    fn color_has_alpha(&self) -> bool { Self::COLOR_HAS_ALPHA }

    #[inline(always)]
    /// Returns `true` if the color uses integer components (e.g., `u8`, `u16`).
    fn color_is_int(&self) -> bool { Self::COLOR_IS_INT }

    #[inline(always)]
    /// Returns `true` if the color is in a linear space (not gamma-encoded like sRGB).
    fn color_is_linear(&self) -> bool { Self::COLOR_IS_LINEAR }

    #[inline(always)]
    /// Returns `true` if the color uses premultiplied alpha.
    fn color_is_premul(&self) -> bool { Self::COLOR_IS_PREMUL }

    #[cfg(feature = "alloc")]
    #[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
    /// Returns a vector containing the color components.
    fn color_components_vec(&self) -> Vec<Self::Component> where Self::Component: Default + Clone {
        let mut buffer = vec![Self::Component::default(); self.color_count()];
        let _ = self.color_components_write(&mut buffer); // CHECK
        buffer
    }
}

/// Args
/// - `$C` the number of components in the inner c array.
/// - `$C` the numberof
macro_rules! impl_color {
    (
    // rgb colors (3 components)
    // - $Name   : the name of the rgb color type
    // - $C      : the type of the inner component
    // - $BITS   : the number of bits of each inner component
    // - $INT    : a boolean indicating whether the components are integers
    // - $LINEAR : a boolean indicating whether it's linear
    rgb: $Name:ty, $C:ty, $BITS:literal, $INT:literal, $LINEAR:literal) => {
        #[rustfmt::skip]
        impl $crate::Color for $Name {
            type Component = $C;
            const COLOR_BITS: usize = $BITS;
            const COLOR_COUNT: usize = 3;
            const COLOR_HAS_ALPHA: bool = false;
            const COLOR_IS_LINEAR: bool = $LINEAR;
            const COLOR_IS_INT: bool = $INT;
            const COLOR_IS_PREMUL: bool = false;

            fn color_red(&self) -> Self::Component { self.c[0] }
            fn color_green(&self) -> Self::Component { self.c[1] }
            fn color_blue(&self) -> Self::Component { self.c[2] }
            /// Since the color has no alpha, the maximum normalized value is returned.
            fn color_alpha(&self) -> Self::Component {
                // NOTE: there's a NUM_MAX_NORM for all supported primitive types
                <Self::Component as $crate::NumConst>::NUM_MAX_NORM.unwrap()
            }

            fn color_components_write(&self, b: &mut [$C]) -> Result<(), $crate::NotEnoughSpace> {
                let c = self.c;
                let needed = Self::COLOR_COUNT;
                if b.len() < needed {
                    Err($crate::NotEnoughSpace(Some(Self::COLOR_COUNT)))
                } else {
                    b[0] = c[0];
                    b[1] = c[1];
                    b[2] = c[2];
                    Ok(())
                }
            }
        }
    };
    (
    // rgba colors (4 components)
    // - $Name   : the name of the rgba color type
    // - $C      : the type of the inner component
    // - $BITS   : the number of bits of each inner component
    // - $INT    : a boolean indicating whether the components are integers
    // - $LINEAR : a boolean indicating whether it's linear
    // - $PREMUL : a boolean indicating whether the alpha is premultiplied
    rgba: $Name:ty, $C:ty, $BITS:literal, $INT:literal, $LINEAR:literal, $PREMUL:literal) => {
        #[rustfmt::skip]
        impl $crate::Color for $Name {
            type Component = $C;
            const COLOR_BITS: usize = $BITS;
            const COLOR_COUNT: usize = 4;
            const COLOR_HAS_ALPHA: bool = true;
            const COLOR_IS_LINEAR: bool = $LINEAR;
            const COLOR_IS_INT: bool = $INT;
            const COLOR_IS_PREMUL: bool = $PREMUL;

            fn color_red(&self) -> Self::Component { self.c[0] }
            fn color_green(&self) -> Self::Component { self.c[1] }
            fn color_blue(&self) -> Self::Component { self.c[2] }
            fn color_alpha(&self) -> Self::Component { self.c[3] }

            fn color_components_write(&self, b: &mut [$C]) -> Result<(), $crate::NotEnoughSpace> {
                let c = self.c;
                let needed = Self::COLOR_COUNT;
                if b.len() < needed {
                    Err($crate::NotEnoughSpace(Some(Self::COLOR_COUNT)))
                } else {
                    b[0] = c[0];
                    b[1] = c[1];
                    b[2] = c[2];
                    b[3] = c[3];
                    Ok(())
                }
            }
        }
    };
    (
    // lum colors (1 component)
    // - $C      : the type of the inner component
    // - $BITS   : the number of bits of each inner component
    // - $INT    : a boolean indicating whether the components are integers
    // - $LINEAR : a boolean indicating whether it's linear
    // - $LIGHT  : a boolean indicating whether it's lightness
    lum: $C:ty, $BITS:literal, $INT:expr, $LINEAR:literal, $LIGHT:literal) => {
        #[rustfmt::skip]
        impl $crate::Color for Lum<$C, $LINEAR, $LIGHT> {
            type Component = $C;
            const COLOR_BITS: usize = $BITS;
            const COLOR_COUNT: usize = 1;
            const COLOR_HAS_ALPHA: bool = false;
            const COLOR_IS_LINEAR: bool = $LINEAR;
            const COLOR_IS_INT: bool = $INT;
            const COLOR_IS_PREMUL: bool = false;

            /// Since the color has no red, the luminosity is returned.
            fn color_red(&self) -> Self::Component { self.c[0] }
            /// Since the color has no red, the luminosity is returned.
            fn color_green(&self) -> Self::Component { self.c[0] }
            /// Since the color has no red, the luminosity is returned.
            fn color_blue(&self) -> Self::Component { self.c[0] }
            /// Since the color has no alpha, the maximum normalized value is returned.
            fn color_alpha(&self) -> Self::Component { self.c[0] }

            fn color_components_write(&self, b: &mut [$C]) -> Result<(), $crate::NotEnoughSpace> {
                let needed = Self::COLOR_COUNT;
                if b.len() < needed {
                    Err($crate::NotEnoughSpace(Some(Self::COLOR_COUNT)))
                } else {
                    b[0] = self.l();
                    Ok(())
                }
            }
        }
    };
}

// impl Color trait
// type, name, primitive, bits, integer, linear, premul
impl_color![rgb: Rgb8, u8, 8, true, false];
impl_color![rgba: Rgba8, u8, 8, true, false, false];
impl_color![rgba: RgbaPre8, u8, 8, true, false, true];
impl_color![rgb: Rgb16, u16, 16, true, false];
impl_color![rgba: Rgba16, u16, 16, true, false, false];
impl_color![rgba: RgbaPre16, u16, 16, true, false, true];
crate::items! {
    impl_color![rgb: RgbF32, f32, 32, false, false];
    impl_color![rgba: RgbaF32, f32, 32, false, false, false];
    impl_color![rgba: RgbaPreF32, f32, 32, false, false, true];
    impl_color![rgb: RgbLinF32, f32, 32, false, true];
    impl_color![rgba: RgbaLinF32, f32, 32, false, true, false];
    impl_color![rgba: RgbaLinPreF32, f32, 32, false, true, true];
}
crate::items! {
    impl_color![rgb: RgbF64, f64, 64, false, false];
    impl_color![rgba: RgbaF64, f64, 64, false, false, false];
    impl_color![rgba: RgbaPreF64, f64, 64, false, false, true];
    impl_color![rgb: RgbLinF64, f64, 64, false, true];
    impl_color![rgba: RgbaLinF64, f64, 64, false, true, false];
    impl_color![rgba: RgbaLinPreF64, f64, 64, false, true, true];
}
