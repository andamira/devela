// devela::media::color::rgb::definitions
//
//!
//

/* u8 */

/// sRGB color (non-linear, gamma-encoded) with 3 [`u8`] components.
/// - **Range**: `0` to `255` (integer).
/// - **Use**: Display, storage (e.g., textures, framebuffers).
#[repr(C)]
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rgb8 {
    /// Color channels in order: [red, green, blue].
    ///
    /// - **Red/Green/Blue**: Gamma-encoded luminosity (0..=255).
    pub c: [u8; 3],
}
/// sRGB color (non-linear, gamma-encoded) with 4 [`u8`] components (RGB + linear alpha).
/// - **Range**: `0` to `255` (integer).
/// - **Use**: Display, storage (e.g., textures with blending).
#[repr(C)]
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rgba8 {
    /// Color channels in order: [red, green, blue, alpha].
    ///
    /// - **Red/Green/Blue**: Gamma-encoded luminosity (0..=255).
    /// - **Alpha**: Linear opacity (0 = transparent, 255 = opaque).
    pub c: [u8; 4],
}

/* u16 */

/// sRGB color (non-linear, gamma-encoded) with 3 [`u16`] components.
/// - **Range**: `0` to `65535` (higher precision than [`Rgb8`]).
/// - **Use**: High-depth storage (e.g., 16-bit PNGs, HDR intermediates).
#[repr(C)]
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rgb16 {
    /// Color channels in order: [red, green, blue].
    ///
    /// - **Red/Green/Blue**: Gamma-encoded luminosity (0..=65_535).
    pub c: [u16; 3],
}
/// sRGB color (non-linear, gamma-encoded) with 4 [`u16`] components (RGB + linear alpha).
/// - **Range**: `0` to `65535` (higher precision than [`Rgba8`]).
/// - **Use**: High-depth storage (e.g., 16-bit PNGs, HDR intermediates).
#[repr(C)]
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rgba16 {
    /// Color channels in order: [red, green, blue, alpha].
    ///
    /// - **Red/Green/Blue**: Gamma-encoded luminosity (0..=65_535).
    /// - **Alpha**: Linear opacity (0 = transparent, 65_535 = opaque).
    pub c: [u16; 4],
}

/* f32 */

/// sRGB color (non-linear, gamma-encoded) with 3 [`f32`] components.
/// - **Range**: 0.0 to 1.0 (clamped)
/// - **Use**: Gamma-space workflows (e.g., color grading, UI colors)
#[repr(C)]
#[must_use]
#[cfg(feature = "_float_f32")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "_float_f32")))]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct RgbF32 {
    /// Color channels in order: [red, green, blue].
    ///
    /// - **Red/Green/Blue**: Gamma-encoded luminosity (0..=65_535).
    pub c: [f32; 3],
}
/// sRGB color (non-linear, gamma-encoded) with 4 [`f32`] components (RGB + linear alpha).
/// - **Range**: 0.0 to 1.0 (clamped)
/// - **Use**: Gamma-space rendering with blending (e.g., transparent UI elements)
#[repr(C)]
#[must_use]
#[cfg(feature = "_float_f32")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "_float_f32")))]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct RgbaF32 {
    /// Color channels in order: [red, green, blue, alpha].
    ///
    /// - **Red/Green/Blue**: Gamma-encoded luminosity (0..=1.0).
    /// - **Alpha**: Linear opacity (0 = transparent, 1.0 = opaque).
    pub c: [f32; 4],
}

/// Linear RGB color with 3 [`f32`] components (no gamma encoding).
/// - **Range**: 0.0 to ∞ (HDR-capable)
/// - **Use**: Physical light calculations (e.g., shaders, ray tracing)
#[repr(C)]
#[must_use]
#[cfg(feature = "_float_f32")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "_float_f32")))]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct RgbLinF32 {
    /// Color channels in order: [red, green, blue].
    ///
    /// - **Red/Green/Blue**: Physical light intensity (unbounded, HDR).
    pub c: [f32; 3],
}
/// Linear RGB color with 4 [`f32`] components (RGB + linear alpha).
/// - **Range**: 0.0 to ∞ (HDR-capable)
/// - **Use**: Physical rendering (e.g., transparent materials, volumetrics)
#[repr(C)]
#[must_use]
#[cfg(feature = "_float_f32")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "_float_f32")))]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct RgbaLinF32 {
    /// Color channels in order: [red, green, blue, alpha].
    ///
    /// - **Red/Green/Blue**: Physical light intensity (unbounded, HDR).
    /// - **Alpha**: Linear opacity (0.0 = transparent, 1.0 = opaque).
    pub c: [f32; 4],
}

/* f64 */

/// sRGB color (non-linear, gamma-encoded) with 3 [`f64`] components.
/// - **Range**: 0.0 to 1.0 (clamped)
/// - **Use**: Gamma-space workflows (e.g., color grading, UI colors)
#[repr(C)]
#[must_use]
#[cfg(feature = "_float_f64")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "_float_f64")))]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct RgbF64 {
    /// Color channels in order: [red, green, blue].
    ///
    /// - **Red/Green/Blue**: Gamma-encoded luminosity (0..=65_535).
    pub c: [f64; 3],
}
/// sRGB color (non-linear, gamma-encoded) with 4 [`f64`] components (RGB + linear alpha).
/// - **Range**: 0.0 to 1.0 (clamped)
/// - **Use**: Gamma-space rendering with blending (e.g., transparent UI elements)
#[repr(C)]
#[must_use]
#[cfg(feature = "_float_f64")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "_float_f64")))]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct RgbaF64 {
    /// Color channels in order: [red, green, blue, alpha].
    ///
    /// - **Red/Green/Blue**: Gamma-encoded luminosity (0..=1.0).
    /// - **Alpha**: Linear opacity (0 = transparent, 1.0 = opaque).
    pub c: [f64; 4],
}

/// Linear RGB color with 3 [`f64`] components (no gamma encoding).
/// - **Range**: 0.0 to ∞ (HDR-capable)
/// - **Use**: Physical light calculations (e.g., shaders, ray tracing)
#[repr(C)]
#[must_use]
#[cfg(feature = "_float_f64")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "_float_f64")))]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct RgbLinF64 {
    /// Color channels in order: [red, green, blue].
    ///
    /// - **Red/Green/Blue**: Physical light intensity (unbounded, HDR).
    pub c: [f64; 3],
}
/// Linear RGB color with 4 [`f64`] components (RGB + linear alpha).
/// - **Range**: 0.0 to ∞ (HDR-capable)
/// - **Use**: Physical rendering (e.g., transparent materials, volumetrics)
#[repr(C)]
#[must_use]
#[cfg(feature = "_float_f64")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "_float_f64")))]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct RgbaLinF64 {
    /// Color channels in order: [red, green, blue, alpha].
    ///
    /// - **Red/Green/Blue**: Physical light intensity (unbounded, HDR).
    /// - **Alpha**: Linear opacity (0.0 = transparent, 1.0 = opaque).
    pub c: [f64; 4],
}
