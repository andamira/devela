// devela::media::color::rgb::impls::f64

#![expect(unused)]

use super::*;
use crate::{ColorBase, Rgb8, Rgba8};
#[cfg(feature = "_float_f32")]
use crate::{RgbF32, RgbaF32};
use crate::{RgbF64, RgbaF64};

/// # Constructors
impl RgbF64 {
    /// New RgbF64.
    pub const fn new(r: f64, g: f64, b: f64) -> RgbF64 {
        Self { r, g, b }
    }
}
