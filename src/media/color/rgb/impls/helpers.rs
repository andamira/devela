// devela::media::color::rgb::impls
//
//!
//
// TOC
// - helper fns
// - Rgb8
// - Rgba8

/* helper fns */

#[cfg(feature = "_float_f32")]
crate::items! {
    /// f32 ← u8: Correct normalization
    pub(crate) const fn u8_to_f32(u: u8) -> f32 { u as f32 / u8::MAX as f32 }
    /// u8 ← f32: Correct with rounding and clamping
    pub(crate) const fn f32_to_u8(f: f32) -> u8 {
        crate::Float(f.clamp(0.0, 1.0) * u8::MAX as f32).const_round().0 as u8
    }

    /// f32 ← u16: Correct normalization
    pub(crate) const fn u16_to_f32(u: u16) -> f32 { u as f32 / u16::MAX as f32 }
    /// u16 ← f32: Correct with rounding and clamping
    pub(crate) const fn f32_to_u16(f: f32) -> u16 {
        crate::Float(f.clamp(0.0, 1.0) * u16::MAX as f32).const_round().0 as u16
    }
}

#[cfg(feature = "_float_f64")]
crate::items! {
    /// f64 ← u8: Correct normalization
    pub(crate) const fn u8_to_f64(u: u8) -> f64 { u as f64 / u8::MAX as f64 }
    /// u8 ← f64: Correct with rounding and clamping
    pub(crate) const fn f64_to_u8(f: f64) -> u8 {
        crate::Float(f.clamp(0.0, 1.0) * u8::MAX as f64).const_round().0 as u8
    }

    /// f64 ← u16: Correct normalization
    pub(crate) const fn u16_to_f64(u: u16) -> f64 { u as f64 / u16::MAX as f64 }
    /// u16 ← f64: Correct with rounding and clamping
    pub(crate) const fn f64_to_u16(f: f64) -> u16 {
        crate::Float(f.clamp(0.0, 1.0) * u16::MAX as f64).const_round().0 as u16
    }
}
