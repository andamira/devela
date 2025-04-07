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
    /// Gamma encoded red luminosity.
    pub r: u8,
    /// Gamma encoded green luminosity.
    pub g: u8,
    /// Gamma encoded blue luminosity.
    pub b: u8,
}
/// sRGB color (non-linear, gamma-encoded) with 4 [`u8`] components (RGB + linear alpha).
/// - **Range**: `0` to `255` (integer).
/// - **Use**: Display, storage (e.g., textures with blending).
#[repr(C)]
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rgba8 {
    /// Gamma encoded red luminosity.
    pub r: u8,
    /// Gamma encoded green luminosity.
    pub g: u8,
    /// Gamma encoded blue luminosity.
    pub b: u8,
    /// Linear alpha channel.
    pub a: u8,
}

/* u16 */

/// sRGB color (non-linear, gamma-encoded) with 3 [`u16`] components.
/// - **Range**: `0` to `65535` (higher precision than [`Rgb8`]).
/// - **Use**: High-depth storage (e.g., 16-bit PNGs, HDR intermediates).
#[repr(C)]
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rgb16 {
    /// Gamma encoded red luminosity.
    pub r: u16,
    /// Gamma encoded green luminosity.
    pub g: u16,
    /// Gamma encoded blue luminosity.
    pub b: u16,
}
/// sRGB color (non-linear, gamma-encoded) with 4 [`u16`] components (RGB + linear alpha).
/// - **Range**: `0` to `65535` (higher precision than [`Rgba8`]).
/// - **Use**: High-depth storage (e.g., 16-bit PNGs, HDR intermediates).
#[repr(C)]
#[must_use]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Rgba16 {
    /// Gamma encoded red luminosity.
    pub r: u16,
    /// Gamma encoded green luminosity.
    pub g: u16,
    /// Gamma encoded blue luminosity.
    pub b: u16,
    /// Linear alpha channel.
    pub a: u16,
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
    /// Gamma encoded red luminosity.
    pub r: f32,
    /// Gamma encoded green luminosity.
    pub g: f32,
    /// Gamma encoded blue luminosity.
    pub b: f32,
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
    /// Gamma encoded red luminosity.
    pub r: f32,
    /// Gamma encoded green luminosity.
    pub g: f32,
    /// Gamma encoded blue luminosity.
    pub b: f32,
    /// Linear alpha channel.
    pub a: f32,
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
    /// Linear red luminosity.
    pub r: f32,
    /// Linear green luminosity.
    pub g: f32,
    /// Linear blue luminosity.
    pub b: f32,
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
    /// Linear red luminosity.
    pub r: f32,
    /// Linear green luminosity.
    pub g: f32,
    /// Linear blue luminosity.
    pub b: f32,
    /// Linear alpha channel.
    pub a: f32,
}

/* f64 */

/// sRGB color (non-linear, gamma-encoded) with 3 [`f32`] components.
/// - **Range**: 0.0 to 1.0 (clamped)
/// - **Use**: Gamma-space workflows (e.g., color grading, UI colors)
#[repr(C)]
#[must_use]
#[cfg(feature = "_float_f64")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "_float_f64")))]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct RgbF64 {
    /// Gamma encoded red luminosity.
    pub r: f64,
    /// Gamma encoded green luminosity.
    pub g: f64,
    /// Gamma encoded blue luminosity.
    pub b: f64,
}
/// sRGB color (non-linear, gamma-encoded) with 4 [`f32`] components (RGB + linear alpha).
/// - **Range**: 0.0 to 1.0 (clamped)
/// - **Use**: Gamma-space rendering with blending (e.g., transparent UI elements)
#[repr(C)]
#[must_use]
#[cfg(feature = "_float_f64")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "_float_f64")))]
#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd)]
pub struct RgbaF64 {
    /// Gamma encoded red luminosity.
    pub r: f64,
    /// Gamma encoded green luminosity.
    pub g: f64,
    /// Gamma encoded blue luminosity.
    pub b: f64,
    /// Linear alpha channel.
    pub a: f64,
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
    /// Linear red luminosity.
    pub r: f64,
    /// Linear green luminosity.
    pub g: f64,
    /// Linear blue luminosity.
    pub b: f64,
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
    /// Linear red luminosity.
    pub r: f64,
    /// Linear green luminosity.
    pub g: f64,
    /// Linear blue luminosity.
    pub b: f64,
    /// Linear alpha channel.
    pub a: f64,
}
