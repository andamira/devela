// devela::media::color::rgb::impls::f32
//
//!
//

use super::*;
use crate::{ColorBase, Rgb8, Rgba8};
use crate::{RgbF32, RgbaF32};
#[cfg(feature = "_float_f64")]
use crate::{RgbF64, RgbaF64};

/// # Constructors
impl RgbF32 {
    /// New RgbF32.
    pub const fn new(r: f32, g: f32, b: f32) -> RgbF32 {
        Self { r, g, b }
    }
}
