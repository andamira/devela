// devela::media::color
//
//! Chromatic functionality.
//
// safety
#![cfg_attr(feature = "safe_color", forbid(unsafe_code))]

mod base;
mod error;
mod namespace;

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{base::*, error::*, namespace::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
    }
}
