// devela::media::color::rgb::impls::f32
//
//!
//

use super::*;
use crate::{ColorBase, Rgb8, Rgba8};
use crate::{RgbF32, RgbaF32};
#[cfg(feature = "_float_f64")]
use crate::{RgbF64, RgbaF64};

#[allow(missing_docs)]
#[rustfmt::skip]
impl RgbF32 {
    /// New `RgbF32`.
    pub const fn new(r: f32, g: f32, b: f32) -> RgbF32 { Self { c: [r, g, b] } }
    /// The red component.
    pub const fn red(self) -> f32 { self.c[0] }
    pub const fn r(self) -> f32 { self.c[0] }
    /// The green component.
    pub const fn green(self) -> f32 { self.c[1] }
    pub const fn g(self) -> f32 { self.c[1] }
    /// The blue component.
    pub const fn blue(self) -> f32 { self.c[2] }
    pub const fn b(self) -> f32 { self.c[2] }
}
