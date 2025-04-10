// devela::media::color::rgb::impls
//
//!
//

use super::definitions::*;

mod helpers; // helper fns

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
impl_color![Rgb8, u8, 3];
impl_color![Rgba8, u8, 3];
impl_color![Rgb16, u16, 3];
impl_color![Rgba16, u16, 3];
#[cfg(feature = "_float_f32")]
crate::items! {
    impl_color![RgbF32, f32, 3];
    impl_color![RgbaF32, f32, 3];
    impl_color![RgbLinF32, f32, 3];
    impl_color![RgbaLinF32, f32, 3];
}
#[cfg(feature = "_float_f64")]
crate::items! {
    impl_color![RgbF64, f64, 3];
    impl_color![RgbaF64, f64, 3];
    impl_color![RgbLinF64, f64, 3];
    impl_color![RgbaLinF64, f64, 3];
}

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods { #![allow(unused)]
        pub(crate) use super::helpers::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
