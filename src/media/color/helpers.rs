// devela::media::color::rgb::impl::helpers
//
// TOC
// - impl_color!
// - Norm
//   - tests

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
pub(crate) use impl_color;

/// Temporary helper for color channel normalization.
pub(crate) struct Norm;
impl Norm {
    #[cfg(feature = "_float_f32")]
    crate::items! {
        /// Convert u8 to normalized f32 (0.0..=1.0)
        ///
        /// This performs perfect normalization where:
        /// - `0u8` becomes `0.0f32`
        /// - `255u8` becomes `1.0f32`
        /// - All intermediate values are evenly spaced
        pub(crate) const fn u8_to_f32(u: u8) -> f32 { u as f32 / u8::MAX as f32 }
        /// Convert f32 to u8 with clamping and rounding
        ///
        /// This conversion:
        /// 1. Clamps input to 0.0..=1.0 range
        /// 2. Scales to 0..255 range
        /// 3. Rounds to nearest integer
        ///
        /// Behavior at boundaries:
        /// - Values ≤ 0.0 become 0u8
        /// - Values ≥ 1.0 become 255u8
        /// - 0.5 becomes 128u8
        pub(crate) const fn f32_to_u8(f: f32) -> u8 {
            crate::Float(f.clamp(0.0, 1.0) * u8::MAX as f32).const_round().0 as u8
        }

        /// Convert u16 to normalized f32 (0.0..=1.0)
        pub(crate) const fn u16_to_f32(u: u16) -> f32 { u as f32 / u16::MAX as f32 }
        /// Convert f32 to u16 with clamping and rounding
        pub(crate) const fn f32_to_u16(f: f32) -> u16 {
            crate::Float(f.clamp(0.0, 1.0) * u16::MAX as f32).const_round().0 as u16
        }
    }

    #[cfg(feature = "_float_f64")]
    crate::items! {
        /// Convert u8 to normalized f64 (0.0..=1.0)
        pub(crate) const fn u8_to_f64(u: u8) -> f64 { u as f64 / u8::MAX as f64 }
        /// Convert f64 to u8 with clamping and rounding
        pub(crate) const fn f64_to_u8(f: f64) -> u8 {
            crate::Float(f.clamp(0.0, 1.0) * u8::MAX as f64).const_round().0 as u8
        }

        /// Convert u16 to normalized f64 (0.0..=1.0)
        pub(crate) const fn u16_to_f64(u: u16) -> f64 { u as f64 / u16::MAX as f64 }
        /// Convert f64 to u16 with clamping and rounding
        pub(crate) const fn f64_to_u16(f: f64) -> u16 {
            crate::Float(f.clamp(0.0, 1.0) * u16::MAX as f64).const_round().0 as u16
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Norm;
    use crate::assert_approx_eq_all;

    #[test]
    #[cfg(feature = "_float_f32")]
    fn test_u8_to_f32() {
        assert_eq![Norm::u8_to_f32(0), 0.0];
        assert_eq![Norm::u8_to_f32(255), 1.0];
        assert_approx_eq_all![tolerance: 0.01, Norm::u8_to_f32(u8::MAX/2+1), 0.5];
    }
    #[test]
    #[cfg(feature = "_float_f32")]
    fn test_f32_to_u8() {
        assert_eq![Norm::f32_to_u8(0.0), 0];
        assert_eq![Norm::f32_to_u8(-500.), 0];
        assert_eq![Norm::f32_to_u8(1.0), u8::MAX]; // == 255
        assert_eq![Norm::f32_to_u8(500.0), u8::MAX];
        assert_eq![Norm::f32_to_u8(0.5), u8::MAX / 2 + 1]; // == 128
    }

    #[test]
    #[cfg(feature = "_float_f32")]
    fn test_u16_to_f32() {
        assert_eq![Norm::u16_to_f32(0), 0.0];
        assert_eq![Norm::u16_to_f32(u16::MAX), 1.0];
        assert_approx_eq_all![tolerance: 0.000_01, Norm::u16_to_f32(u16::MAX/2+1), 0.5];
    }
    #[test]
    #[cfg(feature = "_float_f32")]
    fn test_f32_to_u16() {
        assert_eq![Norm::f32_to_u16(0.0), 0];
        assert_eq![Norm::f32_to_u16(-500.), 0];
        assert_eq![Norm::f32_to_u16(1.0), u16::MAX]; // 65_535
        assert_eq![Norm::f32_to_u16(500.0), u16::MAX];
        assert_eq![Norm::f32_to_u16(0.5), u16::MAX / 2 + 1]; // == 32_768
    }
}
