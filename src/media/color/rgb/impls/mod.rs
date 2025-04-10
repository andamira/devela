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
impl_color![rgb: Rgb8, u8, false];
impl_color![rgb: Rgba8, u8, false];
impl_color![rgb: Rgb16, u16, false];
impl_color![rgb: Rgba16, u16, false];
#[cfg(feature = "_float_f32")]
crate::items! {
    impl_color![rgb: RgbF32, f32, false];
    impl_color![rgb: RgbaF32, f32, false];
    impl_color![rgb: RgbLinF32, f32, true];
    impl_color![rgb: RgbaLinF32, f32, true];
}
#[cfg(feature = "_float_f64")]
crate::items! {
    impl_color![rgb: RgbF64, f64, false];
    impl_color![rgb: RgbaF64, f64, false];
    impl_color![rgb: RgbLinF64, f64, true];
    impl_color![rgb: RgbaLinF64, f64, true];
}
