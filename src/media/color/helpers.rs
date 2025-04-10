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
    // - $C      : the type of the component
    // - $LINEAR : a boolean indicating whether it's linear
    rgb: $Name:ty, $C:ty, $LINEAR:literal) => {
        impl $crate::Color for $Name {
            type Component = $C;

            const COLOR_COUNT: usize = 3;
            const COLOR_IS_LINEAR: bool = $LINEAR;

            fn color_components_write(&self, buf: &mut [$C]) -> Result<(), $crate::NotEnoughSpace> {
                let needed = Self::COLOR_COUNT;
                if buf.len() < needed {
                    Err($crate::NotEnoughSpace(Some(Self::COLOR_COUNT)))
                } else {
                    // Create temporary exact-size slice:
                    buf[..Self::COLOR_COUNT].copy_from_slice(&self.c[..needed]);
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
        assert_approx_eq_all![tolerance: 0.01, u8_to_f32(u8::MAX/2+1), 0.5];
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
        assert_approx_eq_all![tolerance: 0.000_01, u16_to_f32(u16::MAX/2+1), 0.5];
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
