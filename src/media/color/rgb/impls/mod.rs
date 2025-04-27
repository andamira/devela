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
#[cfg(feature = "_float_f32")]
crate::items! { mod f32; mod f32alpha; }
#[cfg(feature = "_float_f64")]
crate::items! { mod f64; mod f64alpha; }

// impl Color trait
// type, name, primitive, bits, integer, linear
impl_color![rgb: Rgb8, u8, 8, true, false];
impl_color![rgb: Rgb16, u16, 16, true, false];
impl_color![rgba: Rgba8, u8, 8, true, false];
impl_color![rgba: Rgba16, u16, 16, true, false];
#[cfg(feature = "_float_f32")]
crate::items! {
    impl_color![rgb: RgbF32, f32, 32, false, false];
    impl_color![rgb: RgbLinF32, f32, 32, false, true];
    impl_color![rgba: RgbaF32, f32, 32, false, false];
    impl_color![rgba: RgbaLinF32, f32, 32, false, true];
}
#[cfg(feature = "_float_f64")]
crate::items! {
    impl_color![rgb: RgbF64, f64, 64, false, false];
    impl_color![rgb: RgbLinF64, f64, 64, false, true];
    impl_color![rgba: RgbaF64, f64, 64, false, false];
    impl_color![rgba: RgbaLinF64, f64, 64, false, true];
}
