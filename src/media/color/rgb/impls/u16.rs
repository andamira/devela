// devela::media::color::rgb::impls::u16

use super::*;
use crate::{ColorBase, Rgb16, Rgba16};

#[cfg(feature = "_float_f32")]
use crate::{RgbF32, RgbaF32};
#[cfg(feature = "_float_f64")]
use crate::{RgbF64, RgbaF64};

#[allow(missing_docs)]
#[rustfmt::skip]
impl Rgb16 {
    /// New `Rgb16`.
    pub const fn new(r: u16, g: u16, b: u16) -> Rgb16 { Self { c: [r, g, b] } }
    /// The red component.
    pub const fn red(self) -> u16 { self.c[0] }
    pub const fn r(self) -> u16 { self.c[0] }
    /// The green component.
    pub const fn green(self) -> u16 { self.c[1] }
    pub const fn g(self) -> u16 { self.c[1] }
    /// The blue component.
    pub const fn blue(self) -> u16 { self.c[2] }
    pub const fn b(self) -> u16 { self.c[2] }
}

/// # General conversions
impl Rgb16 {
    /// Create from an array.
    pub const fn from_array(c: [u16; 3]) -> Rgb16 {
        Rgb16 { c }
    }
    /// Convert to an array.
    pub const fn as_array(self) -> [u16; 3] {
        self.c
    }
    /// Create from a tuple.
    pub const fn from_tuple(c: (u16, u16, u16)) -> Rgb16 {
        Rgb16::new(c.0, c.1, c.2)
    }
    /// Convert to a tuple.
    pub const fn to_tuple(self) -> (u16, u16, u16) {
        (self.r(), self.g(), self.b())
    }

    /* Rgba16 */

    /// Create from [`Rgba16`].
    ///
    /// Loses the alpha channel.
    pub const fn from_rgba16(c: Rgba16) -> Rgb16 {
        Rgb16::new(c.r(), c.g(), c.b())
    }
    /// Convert to [`Rgba16`].
    ///
    /// Adds the given `alpha` channel.
    pub const fn to_rgba16(self, alpha: u16) -> Rgba16 {
        Rgba16::new(self.r(), self.g(), self.b(), alpha)
    }

    /* packed u64 */

    /// Create from a packed `u64` in `0xRRRRGGGGBBBB` format.
    ///
    /// Any bits above `0x0000_FFFF_FFFF_FFFF` are ignored.
    pub const fn from_rgb16_packed(packed: u64) -> Rgb16 {
        Rgb16::from_array([
            ((packed >> 32) & 0xFFFF) as u16,
            ((packed >> 16) & 0xFFFF) as u16,
            (packed & 0xFFFF) as u16,
        ])
    }
    /// Convert to a packed `u64` in `0xRRRRGGGGBBBB` format.
    pub const fn to_rgb16_packed(self) -> u64 {
        ((self.r() as u64) << 32) | ((self.g() as u64) << 16) | (self.b() as u64)
    }

    /// Create from a packed `u64` in `0xRRRRGGGGBBBBAAAA` format, discarding alpha.
    pub const fn from_rgba16_packed(packed: u64) -> Rgb16 {
        Rgb16::from_array([
            ((packed >> 48) & 0xFFFF) as u16,
            ((packed >> 32) & 0xFFFF) as u16,
            ((packed >> 16) & 0xFFFF) as u16,
        ])
    }
    /// Convert to a packed `u64` in `0xRRRRGGGGBBBBAAAA` format, with the given `alpha`.
    pub const fn to_rgba16_packed(self, alpha: u16) -> u64 {
        ((self.r() as u64) << 48)
            | ((self.g() as u64) << 32)
            | ((self.b() as u64) << 16)
            | (alpha as u64)
    }
}
#[rustfmt::skip]
impl From<Rgba16> for Rgb16 { fn from(from: Rgba16) -> Rgb16 { Rgb16::from_rgba16(from) } }

/// # `f32` conversions
#[cfg(feature = "_float_f32")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "_float_f32")))]
impl Rgb16 {
    /// Create from [`RgbF32`].
    pub const fn from_rgb_f32(c: RgbF32) -> Rgb16 {
        Rgb16::new(f32_to_u16(c.r()), f32_to_u16(c.g()), f32_to_u16(c.b()))
    }
    /// Convert to [`RgbF32`].
    pub const fn to_rgb_f32(self) -> RgbF32 {
        RgbF32::new(u16_to_f32(self.r()), u16_to_f32(self.g()), u16_to_f32(self.b()))
    }

    /// Create from [`RgbaF32`].
    ///
    /// Loses the alpha channel.
    pub const fn from_rgba_f32(c: RgbaF32) -> Rgb16 {
        Rgb16::new(f32_to_u16(c.r()), f32_to_u16(c.g()), f32_to_u16(c.b()))
    }
    /// Convert to [`RgbaF32`].
    ///
    /// Adds the given `alpha` channel.
    pub const fn to_rgba_f32(self, alpha: u16) -> RgbaF32 {
        RgbaF32::new(
            u16_to_f32(self.r()),
            u16_to_f32(self.g()),
            u16_to_f32(self.b()),
            u16_to_f32(alpha),
        )
    }
}

/// # `f64` conversions
#[cfg(feature = "_float_f64")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "_float_f64")))]
impl Rgb16 {
    /// Create from [`RgbF64`].
    pub const fn from_rgb_f64(c: RgbF64) -> Rgb16 {
        Rgb16::new(f64_to_u16(c.r()), f64_to_u16(c.g()), f64_to_u16(c.b()))
    }
    /// Convert to [`RgbF64`].
    pub const fn to_rgb_f64(self) -> RgbF64 {
        RgbF64::new(u16_to_f64(self.r()), u16_to_f64(self.g()), u16_to_f64(self.b()))
    }

    /// Create from [`RgbaF64`].
    ///
    /// Loses the alpha channel.
    pub const fn from_rgba_f64(c: RgbaF64) -> Rgb16 {
        Rgb16::new(f64_to_u16(c.r()), f64_to_u16(c.g()), f64_to_u16(c.b()))
    }
    /// Convert to [`RgbaF64`].
    ///
    /// Adds the given `alpha` channel.
    pub const fn to_rgba_f64(self, alpha: u16) -> RgbaF64 {
        RgbaF64::new(
            u16_to_f64(self.r()),
            u16_to_f64(self.g()),
            u16_to_f64(self.b()),
            u16_to_f64(alpha),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const C: Rgb16 = Rgb16::new(1000, 2000, 3000);
    const CA: Rgba16 = Rgba16::new(1000, 2000, 3000, 4000);
    const H: u64 = 0x_03E8_07D0_0BB8;
    const HA: u64 = 0x_03E8_07D0_0BB8_0FA0;

    #[test]
    fn general_conversions() {
        // array/tuple
        assert_eq![Rgb16::from_array([1000, 2000, 3000]), C];
        assert_eq![C.as_array(), [1000, 2000, 3000]];
        assert_eq![Rgb16::from_tuple((1000, 2000, 3000)), C];
        assert_eq![C.to_tuple(), (1000, 2000, 3000)];
        // rgba
        assert_eq![Rgb16::from_rgba16(CA), C];
        assert_eq![C.to_rgba16(4000), CA];
        // packed rgb
        assert_eq![Rgb16::from_rgb16_packed(H), C];
        assert_eq![C.to_rgb16_packed(), H];
        // packed rgba
        assert_eq![Rgb16::from_rgba16_packed(HA), C];
        assert_eq![C.to_rgba16_packed(4000), HA];
    }

    #[test]
    #[cfg(feature = "_float_f32")]
    fn f32_conversions() {
        let f = RgbF32::new(0.015259022, 0.030518044, 0.045777068);
        let fa = RgbaF32::new(0.015259022, 0.030518044, 0.045777068, 0.061036088);
        assert_eq![Rgb16::from_rgb_f32(f), C];
        assert_eq![C.to_rgb_f32(), f];
        assert_eq![Rgb16::from_rgba_f32(fa), C];
        assert_eq![C.to_rgba_f32(4000), fa];
    }

    #[test]
    #[cfg(feature = "_float_f64")]
    fn f64_conversions() {
        let f = RgbF64::new(0.015259021896696421, 0.030518043793392843, 0.04577706569008926);
        let fa = RgbaF64::new(
            0.015259021896696421,
            0.030518043793392843,
            0.04577706569008926,
            0.061036087586785685,
        );
        assert_eq![Rgb16::from_rgb_f64(f), C];
        assert_eq![C.to_rgb_f64(), f];
        assert_eq![Rgb16::from_rgba_f64(fa), C];
        assert_eq![C.to_rgba_f64(4000), fa];
    }
}
