// devela::ui::back::miniquad
//
//! [`miniquad`][crate::_dep::miniquad] UI backends.
//
// ISSUES
// - WAIT: [precise input](https://github.com/not-fl3/miniquad/issues/117)
// - WAIT: [linux resize](https://github.com/not-fl3/miniquad/issues/193)
//
// TODO ::miniquad::data::now (secs since SystemTime::UNIX_EPOCH) or Date.now() / 1000.0 in wasm

mod namespace; // miniquad!
mod reexports;
mod service; // MiniquadEventHandlerExt, MiniquadService
mod window; // MiniquadWindow

#[cfg(feature = "alloc")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "alloc")))]
mod pixels; // MiniquadPixels

// WIPZONE
// mod events;
// mod text;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{namespace::*, reexports::*, service::*, window::*};
        #[cfg(feature = "alloc")]
        pub use super::pixels::*;

        // WIPZONE
        // pub use super::events::*;
        // pub use super::text::*;
    }
}
