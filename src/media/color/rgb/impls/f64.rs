// devela::media::color::rgb::impls::f64

#![expect(unused)]

use super::*;
use crate::{ColorBase, Rgb8, Rgba8};
#[cfg(feature = "_float_f32")]
use crate::{RgbF32, RgbaF32};
use crate::{RgbF64, RgbaF64};

#[rustfmt::skip]
impl ColorBase for RgbF64 {
    type Component = f64;
    fn color_component_count(&self) -> usize { 3 }
    fn color_components_write(&self, b: &mut[f64]) { b.copy_from_slice(&self.c); }
}

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
