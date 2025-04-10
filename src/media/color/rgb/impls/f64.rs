// devela::media::color::rgb::impls::f64

#![expect(unused)]

use super::*;
use crate::{Color, Rgb8, Rgba8};
#[cfg(feature = "_float_f32")]
use crate::{RgbF32, RgbaF32};
use crate::{RgbF64, RgbaF64};

#[allow(missing_docs)]
#[rustfmt::skip]
impl RgbF64 {
    /// New `RgbF64`.
    pub const fn new(r: f64, g: f64, b: f64) -> RgbF64 { Self { c: [r, g, b] } }
    /// The red component.
    pub const fn red(self) -> f64 { self.c[0] }
    pub const fn r(self) -> f64 { self.c[0] }
    /// The green component.
    pub const fn green(self) -> f64 { self.c[1] }
    pub const fn g(self) -> f64 { self.c[1] }
    /// The blue component.
    pub const fn blue(self) -> f64 { self.c[2] }
    pub const fn b(self) -> f64 { self.c[2] }
}
