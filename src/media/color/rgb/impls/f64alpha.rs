// devela::media::color::rgb::impls::f64alpha

#![expect(unused)]

use super::*;
use crate::{ColorBase, Rgb8, Rgba8};
#[cfg(feature = "_float_f32")]
use crate::{RgbF32, RgbaF32};
use crate::{RgbF64, RgbaF64};

/// # Constructors
impl RgbaF64 {
    /// New RgbaF64.
    pub const fn new(r: f64, g: f64, b: f64, a: f64) -> RgbaF64 {
        Self { r, g, b, a }
    }
}
