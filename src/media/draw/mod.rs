// devela::media::draw
//
//! Drawing functionality.
//
// safety
#![cfg_attr(feature = "safe_draw", forbid(unsafe_code))]

mod error;

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods { #![allow(unused)]
        pub use super::error::*;
        // WIPZONE
        // pub use super::buffer::*;
        // pub use super::canvas::*;
        // pub use super::grid::*;
        // pub use super::line::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
    }
}
// WIPZONE
// mod buffer;
// mod canvas;
// mod grid;
// mod line;
