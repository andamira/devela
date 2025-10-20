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

#[cfg(feature = "term")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "term")))]
pub mod sixel;

crate::structural_mods! { // _mods, _pub_mods
    _mods {
        pub use super::{error::*, pnm::*};
    }
    _pub_mods {
        #[doc(inline)]
        pub use super::sixel::_all::*;
    }
}
