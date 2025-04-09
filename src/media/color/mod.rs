// devela::media::color
//
//! Chromatic functionality.
//
// safety
#![cfg_attr(feature = "safe_color", forbid(unsafe_code))]

mod color; // Color
mod gamma; // Gamma
mod luminance; // Luma, Luminance
mod rgb; // Rgb[a][8|16|F32|F64], Rgb[a]Lin[F32|F64]

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{color::*, gamma::*, luminance::*, rgb::_all::*};
        // WIPZONE
        // pub use super::spectral::*;
        // #[cfg(feature = "linear")]
        // pub use super::xyz::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
    }
}
// WIPZONE
// mod spectral;
// #[cfg(feature = "linear")]
// mod xyz; // Xyz
