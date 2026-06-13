// devela/src/media/visual/color/rgb/impls/mod.rs
//
//!
//

use super::*;
use crate::_media_visual_color_impl;

// methods:
mod u8;
mod u8alpha;
mod u16;
mod u16alpha;
crate::items! { mod f32; mod f32alpha; }
crate::items! { mod f64; mod f64alpha; }

// impl Color trait
// type, name, primitive, bits, integer, linear, premul
_media_visual_color_impl![rgb: Rgb8, u8, 8, true, false];
_media_visual_color_impl![rgba: Rgba8, u8, 8, true, false, false];
_media_visual_color_impl![rgba: RgbaPre8, u8, 8, true, false, true];
_media_visual_color_impl![rgb: Rgb16, u16, 16, true, false];
_media_visual_color_impl![rgba: Rgba16, u16, 16, true, false, false];
_media_visual_color_impl![rgba: RgbaPre16, u16, 16, true, false, true];
crate::items! {
    _media_visual_color_impl![rgb: RgbF32, f32, 32, false, false];
    _media_visual_color_impl![rgba: RgbaF32, f32, 32, false, false, false];
    _media_visual_color_impl![rgba: RgbaPreF32, f32, 32, false, false, true];
    _media_visual_color_impl![rgb: RgbLinF32, f32, 32, false, true];
    _media_visual_color_impl![rgba: RgbaLinF32, f32, 32, false, true, false];
    _media_visual_color_impl![rgba: RgbaLinPreF32, f32, 32, false, true, true];
}
crate::items! {
    _media_visual_color_impl![rgb: RgbF64, f64, 64, false, false];
    _media_visual_color_impl![rgba: RgbaF64, f64, 64, false, false, false];
    _media_visual_color_impl![rgba: RgbaPreF64, f64, 64, false, false, true];
    _media_visual_color_impl![rgb: RgbLinF64, f64, 64, false, true];
    _media_visual_color_impl![rgba: RgbaLinF64, f64, 64, false, true, false];
    _media_visual_color_impl![rgba: RgbaLinPreF64, f64, 64, false, true, true];
}
