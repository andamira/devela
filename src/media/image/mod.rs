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

#[cfg(all(feature = "alloc", feature = "term"))]
#[cfg(any(feature = "io", feature = "std"))]
#[cfg(any(feature = "dep_hashbrown", feature = "std"))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(all(feature = "alloc", feature = "term"))))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(any(feature = "io", feature = "std"))))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(any(feature = "dep_hashbrown", feature = "std"))))]
pub mod sixel;

crate::items! { // structural access: _mods, _pub_mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use {_always::*, _pub_mods::*};

    mod _mods {
        pub use super::{error::*, pnm::*};
    }
    mod _pub_mods { #![allow(unused)]
        #[cfg(all(feature = "alloc", feature = "term"))]
        #[cfg(any(feature = "io", feature = "std"))]
        #[cfg(any(feature = "dep_hashbrown", feature = "std"))]
        pub use super::sixel::_all::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::{_mods::*, _pub_mods::*};
    }
    pub(super) mod _always { #![allow(unused)]
    }
}
