// devela_base_core::media::visual::color::rgb::impls::f64alpha

#![expect(unused)]

use super::*;
use crate::{Rgb8, RgbF32, RgbF64, Rgba8, RgbaF32, RgbaF64};

#[allow(missing_docs)]
#[rustfmt::skip]
impl RgbaF64 {
    /// New `RgbF64`.
    pub const fn new(r: f64, g: f64, b: f64, a: f64) -> RgbaF64 { Self { c: [r, g, b, a] } }
    /// The red component.
    pub const fn red(self) -> f64 { self.c[0] }
    pub const fn r(self) -> f64 { self.c[0] }
    /// The green component.
    pub const fn green(self) -> f64 { self.c[1] }
    pub const fn g(self) -> f64 { self.c[1] }
    /// The blue component.
    pub const fn blue(self) -> f64 { self.c[2] }
    pub const fn b(self) -> f64 { self.c[2] }
    /// The alpha component.
    pub const fn alpha(self) -> f64 { self.c[3] }
    pub const fn a(self) -> f64 { self.c[3] }
}
