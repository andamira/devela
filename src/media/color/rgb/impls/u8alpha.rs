// devela::media::color::rgb::impls::u8alpha

use super::*;
use crate::{Rgb8, Rgb16, Rgba8, Rgba16};

#[cfg(feature = "_float_f32")]
use crate::{RgbF32, RgbaF32};
#[cfg(feature = "_float_f64")]
use crate::{RgbF64, RgbaF64};

#[allow(missing_docs)]
#[rustfmt::skip]
impl Rgba8 {
    /// New `Rgba8`.
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Rgba8 { Self { c: [r, g, b, a] } }
    /// The red component.
    pub const fn red(self) -> u8 { self.c[0] }
    pub const fn r(self) -> u8 { self.c[0] }
    /// The green component.
    pub const fn green(self) -> u8 { self.c[1] }
    pub const fn g(self) -> u8 { self.c[1] }
    /// The blue component.
    pub const fn blue(self) -> u8 { self.c[2] }
    pub const fn b(self) -> u8 { self.c[2] }
    /// The alpha component.
    pub const fn alpha(self) -> u8 { self.c[3] }
    pub const fn a(self) -> u8 { self.c[3] }
}

/// # General conversions
impl Rgba8 {
    /// Create from an array.
    pub const fn from_array(c: [u8; 4]) -> Rgba8 {
        Rgba8 { c }
    }
    /// Convert to an array.
    pub const fn as_array(self) -> [u8; 4] {
        self.c
    }

    /// Create from a tuple.
    pub const fn from_tuple(c: (u8, u8, u8, u8)) -> Rgba8 {
        Rgba8::new(c.0, c.1, c.2, c.3)
    }
    /// Convert to a tuple.
    pub const fn to_tuple(self) -> (u8, u8, u8, u8) {
        (self.r(), self.g(), self.b(), self.a())
    }

    /* Rgb8 */

    /// Create from [`Rgb<u8>`].
    ///
    /// Adds the given `alpha` channel.
    pub const fn from_rgb8(c: Rgb8, alpha: u8) -> Rgba8 {
        Rgba8::new(c.r(), c.g(), c.b(), alpha)
    }
    /// Convert to [`Rgb<u8>`].
    ///
    /// Loses the alpha channel.
    pub const fn to_rgb8(self) -> Rgb8 {
        Rgb8::new(self.r(), self.g(), self.b())
    }

    /* packed u32 */

    /// Create from a packed `u32` in `0xRRGGBB` format, with the given `alpha`.
    ///
    /// Any bits above `0x00FF_FFFF` are ignored.
    pub const fn from_rgb8_packed(packed: u32, alpha: u8) -> Rgba8 {
        Rgba8::new(
            ((packed >> 16) & 0xFF) as u8,
            ((packed >> 8) & 0xFF) as u8,
            (packed & 0xFF) as u8,
            alpha,
        )
    }
    /// Convert to a packed `u32` in `0xRRGGBB` format, discarding alpha.
    pub const fn to_rgb8_packed(self) -> u32 {
        ((self.r() as u32) << 16) | ((self.g() as u32) << 8) | (self.b() as u32)
    }

    /// Create from a packed `u32` in `0xRRGGBBAA` format (big-endian).
    pub const fn from_rgba8_packed(packed: u32) -> Rgba8 {
        Rgba8::new(
            ((packed >> 24) & 0xFF) as u8,
            ((packed >> 16) & 0xFF) as u8,
            ((packed >> 8) & 0xFF) as u8,
            (packed & 0xFF) as u8,
        )
    }
    /// Convert to a packed `u32` in `0xRRGGBBAA` format (big-endian).
    pub const fn to_rgba8_packed(self) -> u32 {
        ((self.r() as u32) << 24)
            | ((self.g() as u32) << 16)
            | ((self.b() as u32) << 8)
            | (self.a() as u32)
    }

    /* u16 */

    /// Create from `Rgba<u16>` by scaling each component proportionally.
    pub fn from_rgba16(from: Rgba16) -> Rgba8 {
        Rgba8::new(
            ((from.c[0] + 128) / 257) as u8, // Rounding via +128
            ((from.c[1] + 128) / 257) as u8,
            ((from.c[2] + 128) / 257) as u8,
            ((from.c[3] + 128) / 257) as u8,
        )
    }
    /// Convert to Rgba16 by scaling each component proportionally.
    pub fn to_rgba16(self) -> Rgba16 {
        Rgba16::new(
            (self.c[0] as u16) * 257, // 255 * 257 = 65535
            (self.c[1] as u16) * 257,
            (self.c[2] as u16) * 257,
            (self.c[3] as u16) * 257,
        )
    }
}
#[rustfmt::skip]
impl From<Rgb8> for Rgba8 { fn from(from: Rgb8) -> Rgba8 { Rgba8::from_rgb8(from, u8::MAX) } }
#[rustfmt::skip]
impl From<Rgba16> for Rgba8 { fn from(from: Rgba16) -> Rgba8 { Rgba8::from_rgba16(from) } }

/// # `f32` conversions
#[cfg(feature = "_float_f32")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "_float_f32")))]
impl Rgba8 {
    /// Create from [`Rgb<f32>`].
    ///
    /// Adds the given `alpha` channel.
    pub const fn from_rgb_f32(c: RgbF32, alpha: f32) -> Rgba8 {
        Rgba8::new(f32_to_u8(c.r()), f32_to_u8(c.g()), f32_to_u8(c.b()), f32_to_u8(alpha))
    }
    /// Convert to [`Rgb<f32>`].
    ///
    /// Loses the alpha channel.
    pub const fn to_rgb_f32(self) -> RgbF32 {
        RgbF32::new(u8_to_f32(self.r()), u8_to_f32(self.g()), u8_to_f32(self.b()))
    }

    /// Create from [`Rgba<f32>`].
    pub const fn from_rgba_f32(c: RgbaF32) -> Rgba8 {
        Rgba8::new(f32_to_u8(c.r()), f32_to_u8(c.g()), f32_to_u8(c.b()), f32_to_u8(c.a()))
    }
    /// Convert to [`Rgba<f32>`].
    pub const fn to_rgba_f32(self) -> RgbaF32 {
        RgbaF32::new(
            u8_to_f32(self.r()),
            u8_to_f32(self.g()),
            u8_to_f32(self.b()),
            u8_to_f32(self.a()),
        )
    }
}

/// # `f64` conversions
#[cfg(feature = "_float_f64")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "_float_f64")))]
impl Rgba8 {
    /// Create from [`Rgb<f64>`].
    ///
    /// Adds the given `alpha` channel.
    pub const fn from_rgb_f64(c: RgbF64, alpha: f64) -> Rgba8 {
        Rgba8::new(f64_to_u8(c.r()), f64_to_u8(c.g()), f64_to_u8(c.b()), f64_to_u8(alpha))
    }
    /// Convert to [`Rgb<f64>`].
    ///
    /// Loses the alpha channel.
    pub const fn to_rgb_f64(self) -> RgbF64 {
        RgbF64::new(u8_to_f64(self.r()), u8_to_f64(self.g()), u8_to_f64(self.b()))
    }

    /// Create from [`Rgba<f64>`].
    pub const fn from_rgba_f64(c: RgbaF64) -> Rgba8 {
        Rgba8::new(f64_to_u8(c.r()), f64_to_u8(c.g()), f64_to_u8(c.b()), f64_to_u8(c.a()))
    }
    /// Convert to [`Rgba<f64>`].
    pub const fn to_rgba_f64(self) -> RgbaF64 {
        RgbaF64::new(
            u8_to_f64(self.r()),
            u8_to_f64(self.g()),
            u8_to_f64(self.b()),
            u8_to_f64(self.a()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const C8: Rgb8 = Rgb8::new(10, 20, 30);
    const CA8: Rgba8 = Rgba8::new(10, 20, 30, 40);
    const CA16: Rgba16 = Rgba16::new(2570, 5140, 7710, 10280);
    const H8: u32 = 0x_0A_14_1E;
    const HA8: u32 = 0x_0A_14_1E_28;

    #[test]
    fn general_conversions() {
        // array/tuple
        assert_eq![Rgba8::from_array([10, 20, 30, 40]), CA8];
        assert_eq![CA8.as_array(), [10, 20, 30, 40]];
        assert_eq![Rgba8::from_tuple((10, 20, 30, 40)), CA8];
        assert_eq![CA8.to_tuple(), (10, 20, 30, 40)];
        // rgb
        assert_eq![Rgba8::from_rgb8(C8, 40), CA8];
        assert_eq![CA8.to_rgb8(), C8];
        // packed rgba
        assert_eq![Rgba8::from_rgba8_packed(HA8), CA8];
        assert_eq![CA8.to_rgba8_packed(), HA8];
        // packed rgb
        assert_eq![Rgba8::from_rgb8_packed(H8, 40), CA8];
        assert_eq![CA8.to_rgb8_packed(), H8];
        // u16
        assert_eq![Rgba8::from_rgba16(CA16), CA8];
        assert_eq![CA8.to_rgba16(), CA16];
    }

    #[test]
    #[cfg(feature = "_float_f32")]
    fn f32_conversions() {
        let f = RgbF32::new(0.039215688, 0.078431375, 0.11764706);
        let fa = RgbaF32::new(0.039215688, 0.078431375, 0.11764706, 0.15686275);
        assert_eq![Rgba8::from_rgb_f32(f, 0.156), CA8];
        assert_eq![CA8.to_rgb_f32(), f];
        assert_eq![Rgba8::from_rgba_f32(fa), CA8];
        assert_eq![CA8.to_rgba_f32(), fa];
    }

    #[test]
    #[cfg(feature = "_float_f64")]
    fn f64_conversions() {
        let f = RgbF64::new(0.0392156862745098, 0.0784313725490196, 0.11764705882352941);
        let fa = RgbaF64::new(
            0.0392156862745098,
            0.0784313725490196,
            0.11764705882352941,
            0.1568627450980392,
        );
        assert_eq![Rgba8::from_rgb_f64(f, 0.156), CA8];
        assert_eq![CA8.to_rgb_f64(), f];
        assert_eq![Rgba8::from_rgba_f64(fa), CA8];
        assert_eq![CA8.to_rgba_f64(), fa];
    }
}
