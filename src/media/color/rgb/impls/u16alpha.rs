// devela::media::color::rgb::impls::u16alpha

use super::*;
use crate::{ColorBase, Rgb16, Rgba16};

#[cfg(feature = "_float_f32")]
use crate::{RgbF32, RgbaF32};
#[cfg(feature = "_float_f64")]
use crate::{RgbF64, RgbaF64};

#[rustfmt::skip]
impl ColorBase for Rgba16 {
    type Component = u16;
    fn color_component_count(&self) -> usize { 4 }
    fn color_components_write(&self, b: &mut[u16]) { b.copy_from_slice(&self.c); }
}

#[allow(missing_docs)]
#[rustfmt::skip]
impl Rgba16 {
    /// New `Rgba<u16>`.
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

    /// Create from [`Rgb<u16>`].
    ///
    /// Adds the given `alpha` channel.
    pub const fn from_rgb16(c: Rgb16, alpha: u16) -> Rgba16 {
        Rgba16::new(c.r(), c.g(), c.b(), alpha)
    }
    /// Convert to [`Rgb<u16>`].
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
    /// Create from [`Rgb<f32>`].
    ///
    /// Adds the given `alpha` channel.
    pub const fn from_rgb_f32(c: RgbF32, alpha: f32) -> Rgba16 {
        Rgba16::new(f32_to_u16(c.r()), f32_to_u16(c.g()), f32_to_u16(c.b()), f32_to_u16(alpha))
    }
    /// Convert to [`Rgb<f32>`].
    ///
    /// Loses the alpha channel.
    pub const fn to_rgb_f32(self) -> RgbF32 {
        RgbF32::new(u16_to_f32(self.r()), u16_to_f32(self.g()), u16_to_f32(self.b()))
    }

    /// Create from [`Rgba<f32>`].
    pub const fn from_rgba_f32(c: RgbaF32) -> Rgba16 {
        Rgba16::new(f32_to_u16(c.r()), f32_to_u16(c.g()), f32_to_u16(c.b()), f32_to_u16(c.a()))
    }
    /// Convert to [`Rgba<f32>`].
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
    /// Create from [`Rgb<f64>`].
    ///
    /// Adds the given `alpha` channel.
    pub const fn from_rgb_f64(c: RgbF64, alpha: f64) -> Rgba16 {
        Rgba16::new(f64_to_u16(c.r()), f64_to_u16(c.g()), f64_to_u16(c.b()), f64_to_u16(alpha))
    }
    /// Convert to [`Rgb<f64>`].
    ///
    /// Loses the alpha channel.
    pub const fn to_rgb_f64(self) -> RgbF64 {
        RgbF64::new(u16_to_f64(self.r()), u16_to_f64(self.g()), u16_to_f64(self.b()))
    }

    /// Create from [`Rgba<f64>`].
    pub const fn from_rgba_f64(c: RgbaF64) -> Rgba16 {
        Rgba16::new(f64_to_u16(c.r()), f64_to_u16(c.g()), f64_to_u16(c.b()), f64_to_u16(c.a()))
    }
    /// Convert to [`Rgba<f64>`].
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

    const C16: Rgb16 = Rgb16::new(2570, 5140, 7710);
    const CA16: Rgba16 = Rgba16::new(2570, 5140, 7710, 10280);
    const H16: u64 = 0x_0A0A_1414_1E1E;
    const HA16: u64 = 0x_0A0A_1414_1E1E_2828;

    #[test]
    fn general_conversions() {
        // array/tuple
        assert_eq![Rgba16::from_array([2570, 5140, 7710, 10280]), CA16];
        assert_eq![CA16.as_array(), [2570, 5140, 7710, 10280]];
        assert_eq![Rgba16::from_tuple((2570, 5140, 7710, 10280)), CA16];
        assert_eq![CA16.to_tuple(), (2570, 5140, 7710, 10280)];
        // rgb
        assert_eq![Rgba16::from_rgb16(C16, 10280), CA16];
        assert_eq![CA16.to_rgb16(), C16];
        // packed rgba
        assert_eq![Rgba16::from_rgba16_packed(HA16), CA16];
        assert_eq![CA16.to_rgba16_packed(), HA16];
        // packed rgb
        assert_eq![Rgba16::from_rgb16_packed(H16, 10280), CA16];
        assert_eq![CA16.to_rgb16_packed(), H16];
    }

    #[test]
    #[cfg(feature = "_float_f32")]
    fn f32_conversions() {
        let f = RgbF32::new(0.039215688, 0.078431375, 0.11764706);
        let fa = RgbaF32::new(0.039215688, 0.078431375, 0.11764706, 0.15686275);
        assert_eq![Rgba16::from_rgb_f32(f, 0.15686), CA16];
        assert_eq![CA16.to_rgb_f32(), f];
        assert_eq![Rgba16::from_rgba_f32(fa), CA16];
        assert_eq![CA16.to_rgba_f32(), fa];
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
        assert_eq![Rgba16::from_rgb_f64(f, 0.15686), CA16];
        assert_eq![CA16.to_rgb_f64(), f];
        assert_eq![Rgba16::from_rgba_f64(fa), CA16];
        assert_eq![CA16.to_rgba_f64(), fa];
    }
}
