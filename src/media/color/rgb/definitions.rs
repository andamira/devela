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
pub struct Rgba<T, const LINEAR: bool = false, const PREMUL: bool = false> {
    /// Color channels in order: [red, green, blue, alpha].
    ///
    /// - **Red/Green/Blue**: Gamma-encoded luminosity (0..=255).
    /// - **Alpha**: Linear opacity (0 = transparent, 255 = opaque).
    pub c: [T; 4],
}

/* aliases */

/// RGB color with 8-bit integer components (sRGB gamma space).
pub type Rgb8 = Rgb<u8>;
/// RGB+A color with 8-bit integer components (sRGB gamma space, straight alpha).
pub type Rgba8 = Rgba<u8>;
/// RGB+A color with 8-bit integer components (sRGB gamma space, premultiplied alpha).
pub type RgbaPre8 = Rgba<u8, false, true>;

/// RGB color with 16-bit integer components (sRGB gamma space).
pub type Rgb16 = Rgb<u16>;
/// RGB+A color with 16-bit integer components (sRGB gamma space, straight alpha).
pub type Rgba16 = Rgba<u16>;
/// RGB+A color with 16-bit integer components (sRGB gamma space, premultiplied alpha).
pub type RgbaPre16 = Rgba<u16, false, true>;

crate::items! {
    /// RGB color with 32-bit float components (sRGB gamma space).
    pub type RgbF32 = Rgb<f32>;
    /// RGB+A color with 32-bit float components (sRGB gamma space, straight alpha).
    pub type RgbaF32 = Rgba<f32>;
    /// RGB+A color with 32-bit float components (sRGB gamma space, premultiplied alpha).
    pub type RgbaPreF32 = Rgba<f32, false, true>;

    /// RGB color with 32-bit float components (linear space).
    pub type RgbLinF32 = Rgb<f32, true>;
    /// RGB+A color with 32-bit float components (linear space, straight alpha).
    pub type RgbaLinF32 = Rgba<f32, true>;
    /// RGB+A color with 32-bit float components (linear space, premultiplied alpha).
    pub type RgbaLinPreF32 = Rgba<f32, true, true>;
}
crate::items! {
    /// RGB color with 64-bit float components (sRGB gamma space).
    pub type RgbF64 = Rgb<f64>;
    /// RGB+A color with 64-bit float components (sRGB gamma space, straight alpha).
    pub type RgbaF64 = Rgba<f64>;
    /// RGB+A color with 64-bit float components (sRGB gamma space, premultiplied alpha).
    pub type RgbaPreF64 = Rgba<f64, false, true>;

    /// RGB color with 64-bit float components (linear space).
    pub type RgbLinF64 = Rgb<f64, true>;
    /// RGB+A color with 64-bit float components (linear space, straight alpha).
    pub type RgbaLinF64 = Rgba<f64, true>;
    /// RGB+A color with 64-bit float components (linear space, premultiplied alpha).
    pub type RgbaLinPreF64 = Rgba<f64, true, true>;
}
