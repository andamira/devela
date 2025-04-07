// devela::media::color::rgb::impls::u8

use super::*;
use crate::{ColorBase, Rgb8, Rgb16, Rgba8, Rgba16};

#[cfg(feature = "_float_f32")]
use crate::{RgbF32, RgbaF32};
#[cfg(feature = "_float_f64")]
use crate::{RgbF64, RgbaF64};

#[allow(missing_docs)]
#[rustfmt::skip]
impl Rgb8 {
    /// New `Rgb8`.
    pub const fn new(r: u8, g: u8, b: u8) -> Rgb8 { Self { c: [r, g, b] } }
    /// The red component.
    pub const fn red(self) -> u8 { self.c[0] }
    pub const fn r(self) -> u8 { self.c[0] }
    /// The green component.
    pub const fn green(self) -> u8 { self.c[1] }
    pub const fn g(self) -> u8 { self.c[1] }
    /// The blue component.
    pub const fn blue(self) -> u8 { self.c[2] }
    pub const fn b(self) -> u8 { self.c[2] }
}

/// # General conversions
impl Rgb8 {
    /// Create from an array.
    pub const fn from_array(c: [u8; 3]) -> Rgb8 {
        Rgb8 { c }
    }
    /// Convert to an array.
    pub const fn as_array(self) -> [u8; 3] {
        self.c
    }
    /// Create from a tuple.
    pub const fn from_tuple(c: (u8, u8, u8)) -> Rgb8 {
        Rgb8::new(c.0, c.1, c.2)
    }
    /// Convert to a tuple.
    pub const fn to_tuple(self) -> (u8, u8, u8) {
        (self.r(), self.g(), self.b())
    }

    /* Rgba8 */

    /// Create from [`Rgba8`].
    ///
    /// Loses the alpha channel.
    pub const fn from_rgba8(c: Rgba8) -> Rgb8 {
        Rgb8::new(c.r(), c.g(), c.b())
    }
    /// Convert to [`Rgba8`].
    ///
    /// Adds the given `alpha` channel.
    pub const fn to_rgba8(self, alpha: u8) -> Rgba8 {
        Rgba8::new(self.r(), self.g(), self.b(), alpha)
    }

    /* packed u32 */

    /// Create from a packed `u32` in `0xRRGGBB` format.
    ///
    /// Any bits above `0x00FF_FFFF` are ignored.
    pub const fn from_rgb8_packed(packed: u32) -> Rgb8 {
        Rgb8::from_array([
            ((packed >> 16) & 0xFF) as u8,
            ((packed >> 8) & 0xFF) as u8,
            (packed & 0xFF) as u8,
        ])
    }
    /// Convert to a packed `u32` in `0xRRGGBB` format.
    pub const fn to_rgb8_packed(self) -> u32 {
        ((self.r() as u32) << 16) | ((self.g() as u32) << 8) | (self.b() as u32)
    }

    /// Create from a packed `u32` in `0xRRGGBBAA` format, discarding alpha.
    pub const fn from_rgba8_packed(packed: u32) -> Rgb8 {
        Rgb8::from_array([
            ((packed >> 24) & 0xFF) as u8,
            ((packed >> 16) & 0xFF) as u8,
            ((packed >> 8) & 0xFF) as u8,
        ])
    }
    /// Convert to a packed `u32` in `0xRRGGBBAA` format, with the given `alpha`.
    pub const fn to_rgba8_packed(self, alpha: u8) -> u32 {
        ((self.r() as u32) << 24)
            | ((self.g() as u32) << 16)
            | ((self.b() as u32) << 8)
            | (alpha as u32)
    }

    /* u16 */

    /// Convert to `Rgba8` by scaling each component proportionally.
    pub fn from_rgb16(from: Rgb16) -> Rgb8 {
        Rgb8::new(
            ((from.c[0] + 128) / 257) as u8, // Rounding via +128
            ((from.c[1] + 128) / 257) as u8,
            ((from.c[2] + 128) / 257) as u8,
        )
    }
    /// Create from `Rgb16` by scaling each component proportionally.
    pub fn to_rgb16(self) -> Rgb16 {
        Rgb16::new(
            (self.c[0] as u16) * 257, // 255 * 257 = 65535
            (self.c[1] as u16) * 257,
            (self.c[2] as u16) * 257,
        )
    }
}
#[rustfmt::skip]
impl From<Rgba8> for Rgb8 { fn from(from: Rgba8) -> Rgb8 { Rgb8::from_rgba8(from) } }
#[rustfmt::skip]
impl From<Rgb16> for Rgb8 { fn from(from: Rgb16) -> Rgb8 { Rgb8::from_rgb16(from) } }

/// # `f32` conversions
#[cfg(feature = "_float_f32")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "_float_f32")))]
impl Rgb8 {
    /// Create from [`RgbF32`].
    pub const fn from_rgb_f32(c: RgbF32) -> Rgb8 {
        Rgb8::new(f32_to_u8(c.r()), f32_to_u8(c.g()), f32_to_u8(c.b()))
    }
    /// Convert to [`RgbF32`].
    pub const fn to_rgb_f32(self) -> RgbF32 {
        RgbF32::new(u8_to_f32(self.r()), u8_to_f32(self.g()), u8_to_f32(self.b()))
    }

    /// Create from [`RgbaF32`].
    ///
    /// Loses the alpha channel.
    pub const fn from_rgba_f32(c: RgbaF32) -> Rgb8 {
        Rgb8::new(f32_to_u8(c.r()), f32_to_u8(c.g()), f32_to_u8(c.b()))
    }
    /// Convert to [`RgbaF32`].
    ///
    /// Adds the given `alpha` channel.
    pub const fn to_rgba_f32(self, alpha: u8) -> RgbaF32 {
        RgbaF32::new(
            u8_to_f32(self.r()),
            u8_to_f32(self.g()),
            u8_to_f32(self.b()),
            u8_to_f32(alpha),
        )
    }
}

/// # `f64` conversions
#[cfg(feature = "_float_f64")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "_float_f64")))]
impl Rgb8 {
    /// Create from [`RgbF64`].
    pub const fn from_rgb_f64(c: RgbF64) -> Rgb8 {
        Rgb8::new(f64_to_u8(c.r()), f64_to_u8(c.g()), f64_to_u8(c.b()))
    }
    /// Convert to [`RgbF64`].
    pub const fn to_rgb_f64(self) -> RgbF64 {
        RgbF64::new(u8_to_f64(self.r()), u8_to_f64(self.g()), u8_to_f64(self.b()))
    }

    /// Create from [`RgbaF64`].
    ///
    /// Loses the alpha channel.
    pub const fn from_rgba_f64(c: RgbaF64) -> Rgb8 {
        Rgb8::new(f64_to_u8(c.r()), f64_to_u8(c.g()), f64_to_u8(c.b()))
    }
    /// Convert to [`RgbaF64`].
    ///
    /// Adds the given `alpha` channel.
    pub const fn to_rgba_f64(self, alpha: u8) -> RgbaF64 {
        RgbaF64::new(
            u8_to_f64(self.r()),
            u8_to_f64(self.g()),
            u8_to_f64(self.b()),
            u8_to_f64(alpha),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const C8: Rgb8 = Rgb8::new(10, 20, 30);
    const CA8: Rgba8 = Rgba8::new(10, 20, 30, 40);
    const C16: Rgb16 = Rgb16::new(2570, 5140, 7710);
    const H8: u32 = 0x_0A_14_1E;
    const HA8: u32 = 0x_0A_14_1E_28;

    #[test]
    fn general_conversions() {
        // array/tuple
        assert_eq![Rgb8::from_array([10, 20, 30]), C8];
        assert_eq![C8.as_array(), [10, 20, 30]];
        assert_eq![Rgb8::from_tuple((10, 20, 30)), C8];
        assert_eq![C8.to_tuple(), (10, 20, 30)];
        // rgba
        assert_eq![Rgb8::from_rgba8(CA8), C8];
        assert_eq![C8.to_rgba8(40), CA8];
        // packed rgb
        assert_eq![Rgb8::from_rgb8_packed(H8), C8];
        assert_eq![C8.to_rgb8_packed(), H8];
        // packed rgba
        assert_eq![Rgb8::from_rgba8_packed(HA8), C8];
        assert_eq![C8.to_rgba8_packed(40), HA8];
        // u16
        assert_eq![Rgb8::from_rgb16(C16), C8];
        assert_eq![C8.to_rgb16(), C16];
    }

    #[test]
    #[cfg(feature = "_float_f32")]
    fn f32_conversions() {
        let f = RgbF32::new(0.039215688, 0.078431375, 0.11764706);
        let fa = RgbaF32::new(0.039215688, 0.078431375, 0.11764706, 0.15686275);
        assert_eq![Rgb8::from_rgb_f32(f), C8];
        assert_eq![C8.to_rgb_f32(), f];
        assert_eq![Rgb8::from_rgba_f32(fa), C8];
        assert_eq![C8.to_rgba_f32(40), fa];
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
        assert_eq![Rgb8::from_rgb_f64(f), C8];
        assert_eq![C8.to_rgb_f64(), f];
        assert_eq![Rgb8::from_rgba_f64(fa), C8];
        assert_eq![C8.to_rgba_f64(40), fa];
    }
}
