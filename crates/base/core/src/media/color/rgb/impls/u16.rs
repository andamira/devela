// devela_base_core::media::color::rgb::impls::u16

use super::*;
use crate::{Norm, Rgb16, Rgba16};
// use crate::{Rgb8, Rgba8};

use crate::{RgbF32, RgbF64, RgbaF32, RgbaF64};

#[allow(missing_docs)]
#[rustfmt::skip]
impl Rgb16 {
    /// New `Rgb<u16>`.
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

    /// Create from [`Rgba<u16>`].
    ///
    /// Loses the alpha channel.
    pub const fn from_rgba16(c: Rgba16) -> Rgb16 {
        Rgb16::new(c.r(), c.g(), c.b())
    }
    /// Convert to [`Rgba<u16>`].
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
impl Rgb16 {
    /// Create from [`Rgb<f32>`].
    pub const fn from_rgb_f32(c: RgbF32) -> Rgb16 {
        Rgb16::new(Norm::f32_to_u16(c.r()), Norm::f32_to_u16(c.g()), Norm::f32_to_u16(c.b()))
    }
    /// Convert to [`Rgb<f32>`].
    pub const fn to_rgb_f32(self) -> RgbF32 {
        RgbF32::new(
            Norm::u16_to_f32(self.r()),
            Norm::u16_to_f32(self.g()),
            Norm::u16_to_f32(self.b()),
        )
    }

    /// Create from [`Rgba<f32>`].
    ///
    /// Loses the alpha channel.
    pub const fn from_rgba_f32(c: RgbaF32) -> Rgb16 {
        Rgb16::new(Norm::f32_to_u16(c.r()), Norm::f32_to_u16(c.g()), Norm::f32_to_u16(c.b()))
    }
    /// Convert to [`Rgba<f32>`].
    ///
    /// Adds the given `alpha` channel.
    pub const fn to_rgba_f32(self, alpha: u16) -> RgbaF32 {
        RgbaF32::new(
            Norm::u16_to_f32(self.r()),
            Norm::u16_to_f32(self.g()),
            Norm::u16_to_f32(self.b()),
            Norm::u16_to_f32(alpha),
        )
    }
}

/// # `f64` conversions
impl Rgb16 {
    /// Create from [`Rgb<f64>`].
    pub const fn from_rgb_f64(c: RgbF64) -> Rgb16 {
        Rgb16::new(Norm::f64_to_u16(c.r()), Norm::f64_to_u16(c.g()), Norm::f64_to_u16(c.b()))
    }
    /// Convert to [`Rgb<f64>`].
    pub const fn to_rgb_f64(self) -> RgbF64 {
        RgbF64::new(
            Norm::u16_to_f64(self.r()),
            Norm::u16_to_f64(self.g()),
            Norm::u16_to_f64(self.b()),
        )
    }

    /// Create from [`Rgba<f64>`].
    ///
    /// Loses the alpha channel.
    pub const fn from_rgba_f64(c: RgbaF64) -> Rgb16 {
        Rgb16::new(Norm::f64_to_u16(c.r()), Norm::f64_to_u16(c.g()), Norm::f64_to_u16(c.b()))
    }
    /// Convert to [`Rgba<f64>`].
    ///
    /// Adds the given `alpha` channel.
    pub const fn to_rgba_f64(self, alpha: u16) -> RgbaF64 {
        RgbaF64::new(
            Norm::u16_to_f64(self.r()),
            Norm::u16_to_f64(self.g()),
            Norm::u16_to_f64(self.b()),
            Norm::u16_to_f64(alpha),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const C16: Rgb16 = Rgb16::new(2570, 5140, 7710);
    const CA16: Rgba16 = Rgba16::new(2570, 5140, 7710, 10280);
    const H16: u64 = 0x_0A0A_1414_1E1E;
    const HA16: u64 = 0x_0A0A_1414_1E1E_2828;

    #[test]
    fn general_conversions() {
        // array/tuple
        assert_eq![Rgb16::from_array([2570, 5140, 7710]), C16];
        assert_eq![C16.as_array(), [2570, 5140, 7710]];
        assert_eq![Rgb16::from_tuple((2570, 5140, 7710)), C16];
        assert_eq![C16.to_tuple(), (2570, 5140, 7710)];
        // rgba
        assert_eq![Rgb16::from_rgba16(CA16), C16];
        assert_eq![C16.to_rgba16(10280), CA16];
        // packed rgb
        assert_eq![Rgb16::from_rgb16_packed(H16), C16];
        assert_eq![C16.to_rgb16_packed(), H16];
        // packed rgba
        assert_eq![Rgb16::from_rgba16_packed(HA16), C16];
        assert_eq![C16.to_rgba16_packed(10280), HA16];
    }

    #[test]
    fn f32_conversions() {
        let f = RgbF32::new(0.039215688, 0.078431375, 0.11764706);
        let fa = RgbaF32::new(0.039215688, 0.078431375, 0.11764706, 0.15686275);
        assert_eq![Rgb16::from_rgb_f32(f), C16];
        assert_eq![C16.to_rgb_f32(), f];
        assert_eq![Rgb16::from_rgba_f32(fa), C16];
        assert_eq![C16.to_rgba_f32(10280), fa];
    }

    #[test]
    fn f64_conversions() {
        let f = RgbF64::new(0.0392156862745098, 0.0784313725490196, 0.11764705882352941);
        let fa = RgbaF64::new(
            0.0392156862745098,
            0.0784313725490196,
            0.11764705882352941,
            0.1568627450980392,
        );
        assert_eq![Rgb16::from_rgb_f64(f), C16];
        assert_eq![C16.to_rgb_f64(), f];
        assert_eq![Rgb16::from_rgba_f64(fa), C16];
        assert_eq![C16.to_rgba_f64(10280), fa];
    }
}
