// devela::media::image
//
//! Image manipulation.
// #![doc = crate::doc_!(modules: crate::media; image: pnm)]
// #![doc = crate::doc_!(newline)]
//
// safety
#![cfg_attr(feature = "safe_image", forbid(unsafe_code))]

mod error;
mod pnm;

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{error::*, pnm::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
    }
}
