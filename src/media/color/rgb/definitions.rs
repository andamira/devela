// devela::media::color::rgb::definitions
//
//! Defines the [`Rgb`] and [`Rgba`] types.
//

/// RGB color with 3 components.
#[repr(C)]
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rgb<T, const LINEAR: bool = false> {
    /// Color channels in order: [red, green, blue].
    ///
    /// - **Red/Green/Blue**: Gamma-encoded luminosity (0..=255).
    pub c: [T; 3],
}
/// RGB+A color with 4 components.
#[repr(C)]
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rgba<T, const LINEAR: bool = false> {
    /// Color channels in order: [red, green, blue, alpha].
    ///
    /// - **Red/Green/Blue**: Gamma-encoded luminosity (0..=255).
    /// - **Alpha**: Linear opacity (0 = transparent, 255 = opaque).
    pub c: [T; 4],
}

/* aliases */

pub(crate) type Rgb8 = Rgb<u8>;
pub(crate) type Rgba8 = Rgba<u8>;
pub(crate) type Rgb16 = Rgb<u16>;
pub(crate) type Rgba16 = Rgba<u16>;

#[cfg(feature = "_float_f32")]
crate::items! {
    pub(crate) type RgbF32 = Rgb<f32>;
    pub(crate) type RgbaF32 = Rgba<f32>;
    pub(crate) type RgbLinF32 = Rgb<f32, true>;
    pub(crate) type RgbaLinF32 = Rgba<f32, true>;
}
#[cfg(feature = "_float_f64")]
crate::items! {
    pub(crate) type RgbF64 = Rgb<f64>;
    pub(crate) type RgbaF64 = Rgba<f64>;
    pub(crate) type RgbLinF64 = Rgb<f64, true>;
    pub(crate) type RgbaLinF64 = Rgba<f64, true>;
}
