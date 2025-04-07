// devela::media::color::rgb::impls::u8alpha

use super::*;
use crate::{ColorBase, Rgb8, Rgba8};

#[cfg(feature = "_float_f32")]
use crate::{RgbF32, RgbaF32};
#[cfg(feature = "_float_f64")]
use crate::{RgbF64, RgbaF64};

impl Rgba8 {
    /// New `Rgba8`.
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Rgba8 {
        Self { r, g, b, a }
    }
}

/// # General conversions
impl Rgba8 {
    /// Create from an array.
    pub const fn from_array(c: [u8; 4]) -> Rgba8 {
        Rgba8 { r: c[0], g: c[1], b: c[2], a: c[3] }
    }
    /// Convert to an array.
    pub const fn to_array(self) -> [u8; 4] {
        [self.r, self.g, self.b, self.a]
    }

    /// Create from a tuple.
    pub const fn from_tuple(c: (u8, u8, u8, u8)) -> Rgba8 {
        Rgba8 { r: c.0, g: c.1, b: c.2, a: c.3 }
    }
    /// Convert to a tuple.
    pub const fn to_tuple(self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }

    /* Rgb8 */

    /// Create from [`Rgb8`].
    ///
    /// Adds the given `alpha` channel.
    pub const fn from_rgb8(c: Rgb8, alpha: u8) -> Rgba8 {
        Rgba8 { r: c.r, g: c.g, b: c.b, a: alpha }
    }
    /// Convert to [`Rgb8`].
    ///
    /// Loses the alpha channel.
    pub const fn to_rgb8(self) -> Rgb8 {
        Rgb8 { r: self.r, g: self.g, b: self.b }
    }

    /* packed u32 */

    /// Create from a packed `u32` in `0xRRGGBB` format, with the given `alpha`.
    ///
    /// Any bits above `0x00FF_FFFF` are ignored.
    pub const fn from_rgb8_packed(packed: u32, alpha: u8) -> Rgba8 {
        Rgba8 {
            r: ((packed >> 16) & 0xFF) as u8,
            g: ((packed >> 8) & 0xFF) as u8,
            b: (packed & 0xFF) as u8,
            a: alpha,
        }
    }
    /// Convert to a packed `u32` in `0xRRGGBB` format, discarding alpha.
    pub const fn to_rgb8_packed(self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }

    /// Create from a packed `u32` in `0xRRGGBBAA` format (big-endian).
    pub const fn from_rgba8_packed(packed: u32) -> Rgba8 {
        Rgba8 {
            r: ((packed >> 24) & 0xFF) as u8,
            g: ((packed >> 16) & 0xFF) as u8,
            b: ((packed >> 8) & 0xFF) as u8,
            a: (packed & 0xFF) as u8,
        }
    }
    /// Convert to a packed `u32` in `0xRRGGBBAA` format (big-endian).
    pub const fn to_rgba8_packed(self) -> u32 {
        ((self.r as u32) << 24) | ((self.g as u32) << 16) | ((self.b as u32) << 8) | (self.a as u32)
    }
}

/// # `f32` conversions
#[cfg(feature = "_float_f32")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "_float_f32")))]
impl Rgba8 {
    /// Create from [`RgbF32`].
    ///
    /// Adds the given `alpha` channel.
    pub const fn from_rgb_f32(c: RgbF32, alpha: f32) -> Rgba8 {
        Rgba8 {
            r: f32_to_u8(c.r),
            g: f32_to_u8(c.g),
            b: f32_to_u8(c.b),
            a: f32_to_u8(alpha),
        }
    }
    /// Convert to [`RgbF32`].
    ///
    /// Loses the alpha channel.
    pub const fn to_rgb_f32(self) -> RgbF32 {
        RgbF32 {
            r: u8_to_f32(self.r),
            g: u8_to_f32(self.g),
            b: u8_to_f32(self.b),
        }
    }

    /// Create from [`RgbaF32`].
    pub const fn from_rgba_f32(c: RgbaF32) -> Rgba8 {
        Rgba8 {
            r: f32_to_u8(c.r),
            g: f32_to_u8(c.g),
            b: f32_to_u8(c.b),
            a: f32_to_u8(c.a),
        }
    }
    /// Convert to [`RgbaF32`].
    pub const fn to_rgba_f32(self) -> RgbaF32 {
        RgbaF32 {
            r: u8_to_f32(self.r),
            g: u8_to_f32(self.g),
            b: u8_to_f32(self.b),
            a: u8_to_f32(self.a),
        }
    }
}

/// # `f64` conversions
#[cfg(feature = "_float_f64")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "_float_f64")))]
impl Rgba8 {
    /// Create from [`RgbF64`].
    ///
    /// Adds the given `alpha` channel.
    pub const fn from_rgb_f64(c: RgbF64, alpha: f64) -> Rgba8 {
        Rgba8 {
            r: f64_to_u8(c.r),
            g: f64_to_u8(c.g),
            b: f64_to_u8(c.b),
            a: f64_to_u8(alpha),
        }
    }
    /// Convert to [`RgbF64`].
    ///
    /// Loses the alpha channel.
    pub const fn to_rgb_f64(self) -> RgbF64 {
        RgbF64 {
            r: u8_to_f64(self.r),
            g: u8_to_f64(self.g),
            b: u8_to_f64(self.b),
        }
    }

    /// Create from [`RgbaF64`].
    pub const fn from_rgba_f64(c: RgbaF64) -> Rgba8 {
        Rgba8 {
            r: f64_to_u8(c.r),
            g: f64_to_u8(c.g),
            b: f64_to_u8(c.b),
            a: f64_to_u8(c.a),
        }
    }
    /// Convert to [`RgbaF64`].
    pub const fn to_rgba_f64(self) -> RgbaF64 {
        RgbaF64 {
            r: u8_to_f64(self.r),
            g: u8_to_f64(self.g),
            b: u8_to_f64(self.b),
            a: u8_to_f64(self.a),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const C: Rgb8 = Rgb8::new(10, 20, 30);
    const CA: Rgba8 = Rgba8::new(10, 20, 30, 40);
    const H: u32 = 0x_0A_14_1E;
    const HA: u32 = 0x_0A_14_1E_28;

    #[test]
    fn general_conversions() {
        // array/tuple
        assert_eq![Rgba8::from_array([10, 20, 30, 40]), CA];
        assert_eq![CA.to_array(), [10, 20, 30, 40]];
        assert_eq![Rgba8::from_tuple((10, 20, 30, 40)), CA];
        assert_eq![CA.to_tuple(), (10, 20, 30, 40)];
        // rgb
        assert_eq![Rgba8::from_rgb8(C, 40), CA];
        assert_eq![CA.to_rgb8(), C];
        // packed rgba
        assert_eq![Rgba8::from_rgba8_packed(HA), CA];
        assert_eq![CA.to_rgba8_packed(), HA];
        // packed rgb
        assert_eq![Rgba8::from_rgb8_packed(H, 40), CA];
        assert_eq![CA.to_rgb8_packed(), H];
    }

    #[test]
    #[cfg(feature = "_float_f32")]
    fn f32_conversions() {
        let f = RgbF32::new(0.039215688, 0.078431375, 0.11764706);
        let fa = RgbaF32::new(0.039215688, 0.078431375, 0.11764706, 0.15686275);
        assert_eq![Rgba8::from_rgb_f32(f, 0.156), CA];
        assert_eq![CA.to_rgb_f32(), f];
        assert_eq![Rgba8::from_rgba_f32(fa), CA];
        assert_eq![CA.to_rgba_f32(), fa];
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
        assert_eq![Rgba8::from_rgb_f64(f, 0.156), CA];
        assert_eq![CA.to_rgb_f64(), f];
        assert_eq![Rgba8::from_rgba_f64(fa), CA];
        assert_eq![CA.to_rgba_f64(), fa];
    }
}
