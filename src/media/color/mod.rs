// devela::media::color
//
//! Chromatic functionality.
//
// safety
#![cfg_attr(feature = "safe_color", forbid(unsafe_code))]

mod base; // trait ColorBase
mod namespace; // struct Color
mod rgb; // Rgb[a][8|16|F32|F64], Rgb[a]Lin[F32|F64]

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{base::*, namespace::*, rgb::_all::*};
        // WIPZONE
        // pub use super::gray::*;
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
// mod spectral;
