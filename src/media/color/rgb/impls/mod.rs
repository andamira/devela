// devela::media::color::rgb::impls
//
//!
//

use super::definitions::*;
use crate::impl_color;

// methods:
mod u8;
mod u8alpha;
mod u16;
mod u16alpha;
crate::items! { mod f32; mod f32alpha; }
crate::items! { mod f64; mod f64alpha; }

// impl Color trait
// type, name, primitive, bits, integer, linear, premul
impl_color![rgb: Rgb8, u8, 8, true, false];
impl_color![rgba: Rgba8, u8, 8, true, false, false];
impl_color![rgba: RgbaPre8, u8, 8, true, false, true];
impl_color![rgb: Rgb16, u16, 16, true, false];
impl_color![rgba: Rgba16, u16, 16, true, false, false];
impl_color![rgba: RgbaPre16, u16, 16, true, false, true];
crate::items! {
    impl_color![rgb: RgbF32, f32, 32, false, false];
    impl_color![rgba: RgbaF32, f32, 32, false, false, false];
    impl_color![rgba: RgbaPreF32, f32, 32, false, false, true];
    impl_color![rgb: RgbLinF32, f32, 32, false, true];
    impl_color![rgba: RgbaLinF32, f32, 32, false, true, false];
    impl_color![rgba: RgbaLinPreF32, f32, 32, false, true, true];
}
crate::items! {
    impl_color![rgb: RgbF64, f64, 64, false, false];
    impl_color![rgba: RgbaF64, f64, 64, false, false, false];
    impl_color![rgba: RgbaPreF64, f64, 64, false, false, true];
    impl_color![rgb: RgbLinF64, f64, 64, false, true];
    impl_color![rgba: RgbaLinF64, f64, 64, false, true, false];
    impl_color![rgba: RgbaLinPreF64, f64, 64, false, true, true];
}
