// devela::media::color::rgb::impls::u16alpha

use super::*;
use crate::{ColorBase, Rgb16, Rgba16};

#[cfg(feature = "_float_f32")]
use crate::{RgbF32, RgbaF32};
#[cfg(feature = "_float_f64")]
use crate::{RgbF64, RgbaF64};

#[allow(missing_docs)]
#[rustfmt::skip]
impl Rgba16 {
    /// New `Rgba16`.
    pub const fn new(r: u16, g: u16, b: u16, a: u16) -> Rgba16 { Self { c: [r, g, b, a] } }
    /// The red component.
    pub const fn red(self) -> u16 { self.c[0] }
    pub const fn r(self) -> u16 { self.c[0] }
    /// The green component.
    pub const fn green(self) -> u16 { self.c[1] }
    pub const fn g(self) -> u16 { self.c[1] }
    /// The blue component.
    pub const fn blue(self) -> u16 { self.c[2] }
    pub const fn b(self) -> u16 { self.c[2] }
    /// The alpha component.
    pub const fn alpha(self) -> u16 { self.c[3] }
    pub const fn a(self) -> u16 { self.c[3] }
}

/// # General conversions
impl Rgba16 {
    /// Create from an array.
    pub const fn from_array(c: [u16; 4]) -> Rgba16 {
        Rgba16 { c }
    }
    /// Convert to an array.
    pub const fn as_array(self) -> [u16; 4] {
        self.c
    }

    /// Create from a tuple.
    pub const fn from_tuple(c: (u16, u16, u16, u16)) -> Rgba16 {
        Rgba16::new(c.0, c.1, c.2, c.3)
    }
    /// Convert to a tuple.
    pub const fn to_tuple(self) -> (u16, u16, u16, u16) {
        (self.r(), self.g(), self.b(), self.a())
    }

    /* Rgb16 */

    /// Create from [`Rgb16`].
    ///
    /// Adds the given `alpha` channel.
    pub const fn from_rgb16(c: Rgb16, alpha: u16) -> Rgba16 {
        Rgba16::new(c.r(), c.g(), c.b(), alpha)
    }
    /// Convert to [`Rgb16`].
    ///
    /// Loses the alpha channel.
    pub const fn to_rgb16(self) -> Rgb16 {
        Rgb16::new(self.r(), self.g(), self.b())
    }

    /* packed u64 */

    /// Create from a packed `u64` in `0xRRRRGGGGBBBBAAAA` format, with the given `alpha`.
    ///
    /// Any bits above `0x0000_FFFF_FFFF` are ignored.
    pub const fn from_rgb16_packed(packed: u64, alpha: u16) -> Rgba16 {
        Rgba16::new(
            ((packed >> 32) & 0xFFFF) as u16,
            ((packed >> 16) & 0xFFFF) as u16,
            (packed & 0xFFFF) as u16,
            alpha,
        )
    }
    /// Convert to a packed `u64` in `0xRRRRGGGGBBBB` format, discarding alpha.
    pub const fn to_rgb16_packed(self) -> u64 {
        ((self.r() as u64) << 32) | ((self.g() as u64) << 16) | (self.b() as u64)
    }

    /// Create from a packed `u64` in `0xRRRRGGGGBBBBAAAA` format (big-endian).
    pub const fn from_rgba16_packed(packed: u64) -> Rgba16 {
        Rgba16::new(
            ((packed >> 48) & 0xFFFF) as u16,
            ((packed >> 32) & 0xFFFF) as u16,
            ((packed >> 16) & 0xFFFF) as u16,
            (packed & 0xFFFF) as u16,
        )
    }
    /// Convert to a packed `u64` in `0xRRRRGGGGBBBBAAAA` format (big-endian).
    pub const fn to_rgba16_packed(self) -> u64 {
        ((self.r() as u64) << 48)
            | ((self.g() as u64) << 32)
            | ((self.b() as u64) << 16)
            | (self.a() as u64)
    }
}
#[rustfmt::skip]
impl From<Rgb16> for Rgba16 {
    fn from(from: Rgb16) -> Rgba16 { Rgba16::from_rgb16(from, u16::MAX) } }

/// # `f32` conversions
#[cfg(feature = "_float_f32")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "_float_f32")))]
impl Rgba16 {
    /// Create from [`RgbF32`].
    ///
    /// Adds the given `alpha` channel.
    pub const fn from_rgb_f32(c: RgbF32, alpha: f32) -> Rgba16 {
        Rgba16::new(f32_to_u16(c.r()), f32_to_u16(c.g()), f32_to_u16(c.b()), f32_to_u16(alpha))
    }
    /// Convert to [`RgbF32`].
    ///
    /// Loses the alpha channel.
    pub const fn to_rgb_f32(self) -> RgbF32 {
        RgbF32::new(u16_to_f32(self.r()), u16_to_f32(self.g()), u16_to_f32(self.b()))
    }

    /// Create from [`RgbaF32`].
    pub const fn from_rgba_f32(c: RgbaF32) -> Rgba16 {
        Rgba16::new(f32_to_u16(c.r()), f32_to_u16(c.g()), f32_to_u16(c.b()), f32_to_u16(c.a()))
    }
    /// Convert to [`RgbaF32`].
    pub const fn to_rgba_f32(self) -> RgbaF32 {
        RgbaF32::new(
            u16_to_f32(self.r()),
            u16_to_f32(self.g()),
            u16_to_f32(self.b()),
            u16_to_f32(self.a()),
        )
    }
}

/// # `f64` conversions
#[cfg(feature = "_float_f64")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "_float_f64")))]
impl Rgba16 {
    /// Create from [`RgbF64`].
    ///
    /// Adds the given `alpha` channel.
    pub const fn from_rgb_f64(c: RgbF64, alpha: f64) -> Rgba16 {
        Rgba16::new(f64_to_u16(c.r()), f64_to_u16(c.g()), f64_to_u16(c.b()), f64_to_u16(alpha))
    }
    /// Convert to [`RgbF64`].
    ///
    /// Loses the alpha channel.
    pub const fn to_rgb_f64(self) -> RgbF64 {
        RgbF64::new(u16_to_f64(self.r()), u16_to_f64(self.g()), u16_to_f64(self.b()))
    }

    /// Create from [`RgbaF64`].
    pub const fn from_rgba_f64(c: RgbaF64) -> Rgba16 {
        Rgba16::new(f64_to_u16(c.r()), f64_to_u16(c.g()), f64_to_u16(c.b()), f64_to_u16(c.a()))
    }
    /// Convert to [`RgbaF64`].
    pub const fn to_rgba_f64(self) -> RgbaF64 {
        RgbaF64::new(
            u16_to_f64(self.r()),
            u16_to_f64(self.g()),
            u16_to_f64(self.b()),
            u16_to_f64(self.a()),
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
        assert_eq![Rgba16::from_array([1000, 2000, 3000, 4000]), CA];
        assert_eq![CA.as_array(), [1000, 2000, 3000, 4000]];
        assert_eq![Rgba16::from_tuple((1000, 2000, 3000, 4000)), CA];
        assert_eq![CA.to_tuple(), (1000, 2000, 3000, 4000)];
        // rgb
        assert_eq![Rgba16::from_rgb16(C, 4000), CA];
        assert_eq![CA.to_rgb16(), C];
        // packed rgba
        assert_eq![Rgba16::from_rgba16_packed(HA), CA];
        assert_eq![CA.to_rgba16_packed(), HA];
        // packed rgb
        assert_eq![Rgba16::from_rgb16_packed(H, 4000), CA];
        assert_eq![CA.to_rgb16_packed(), H];
    }

    #[test]
    #[cfg(feature = "_float_f32")]
    fn f32_conversions() {
        let f = RgbF32::new(0.015259022, 0.030518044, 0.045777068);
        let fa = RgbaF32::new(0.015259022, 0.030518044, 0.045777068, 0.061036088);
        assert_eq![Rgba16::from_rgb_f32(f, 0.06103), CA];
        assert_eq![CA.to_rgb_f32(), f];
        assert_eq![Rgba16::from_rgba_f32(fa), CA];
        assert_eq![CA.to_rgba_f32(), fa];
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
        assert_eq![Rgba16::from_rgb_f64(f, 0.06103), CA];
        assert_eq![CA.to_rgb_f64(), f];
        assert_eq![Rgba16::from_rgba_f64(fa), CA];
        assert_eq![CA.to_rgba_f64(), fa];
    }
}
