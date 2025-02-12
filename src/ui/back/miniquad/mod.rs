// devela::ui::back::miniquad
//
//! [`miniquad`][crate::_dep::miniquad] UI backends.
//
// ISSUES
// - WAIT: [precise input](https://github.com/not-fl3/miniquad/issues/117)
// - WAIT: [linux resize](https://github.com/not-fl3/miniquad/issues/193)

mod service; // MiniquadEventHandlerExt, MiniquadService
mod window; // MiniquadWindow

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "alloc")))]
mod pixels; // MiniquadPixels

crate::items! { // structural access: _mods, _all
    #[allow(unused)]
    pub use _mods::*;

    mod _mods {
        pub use super::{service::*, window::*};
        #[cfg(feature = "alloc")]
        pub use super::pixels::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_mods::*;
    }
}
