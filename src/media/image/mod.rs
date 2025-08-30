// devela::media::image
//
//! Image manipulation.
#![doc = crate::_doc!(modules: crate::media; image: sixel)]
// #![doc = crate::_doc!(newline)]
//
// safety
#![cfg_attr(feature = "safe_image", forbid(unsafe_code))]

mod error;
mod pnm;

#[cfg(all(feature = "alloc", feature = "term"))]
#[cfg(any(feature = "io", feature = "std"))]
#[cfg(any(feature = "dep_hashbrown", feature = "std"))]
#[cfg_attr(nightly_doc, doc(cfg(all(feature = "alloc", feature = "term"))))]
#[cfg_attr(nightly_doc, doc(cfg(any(feature = "io", feature = "std"))))]
#[cfg_attr(nightly_doc, doc(cfg(any(feature = "dep_hashbrown", feature = "std"))))]
pub mod sixel;

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::{error::*, pnm::*};
    }
    _pub_mods {
        #[cfg(all(feature = "alloc", feature = "term"))]
        #[cfg(any(feature = "io", feature = "std"))]
        #[cfg(any(feature = "dep_hashbrown", feature = "std"))]
        pub use super::sixel::_all::*;
    }
}
