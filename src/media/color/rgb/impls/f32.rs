// devela::media::color::rgb::impls::f32
//
//!
//

#![expect(unused)]

use super::*;
use crate::{Color, Rgb8, Rgba8};
use crate::{RgbF32, RgbaF32};
#[cfg(feature = "_float_f64")]
use crate::{RgbF64, RgbaF64};

#[rustfmt::skip]
impl Color for RgbF32 {
    type Component = f32;
    fn color_component_count(&self) -> usize { 3 }
    fn color_components_write(&self, b: &mut[f32]) { b.copy_from_slice(&self.c); }
}

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
