// devela::media::color
//
//! Chromatic functionality.
//
// safety
#![cfg_attr(feature = "safe_color", forbid(unsafe_code))]

mod base; // trait ColorBase
mod namespace; // struct Color

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{base::*, namespace::*};
        // WIPZONE
        // pub use super::gray::*;
        // pub use super::rgb::_all::*;
        // pub use super::spectral::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
    }
}
// WIPZONE
// mod gray;
// mod rgb;
// mod spectral;
