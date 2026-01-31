// devela::media::image
//
#![doc = crate::_DOC_MEDIA_IMAGE!()] // public
#![doc = crate::_doc!(modules: crate::media; image: sixel)]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//
// safety
#![cfg_attr(feature = "safe_image", forbid(unsafe_code))]

mod error;
mod pnm;

#[cfg(feature = "term")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "term")))]
pub mod sixel;

crate::structural_mods! { // _mods, _pub_mods, _hidden
    _mods {
        pub use super::{error::*, pnm::*};
    }
    _pub_mods {
        #[doc(inline)]
        #[cfg(feature = "term")]
        pub use super::sixel::_all::*;
    }
    _hidden {
        #[cfg(feature = "term")]
        pub use super::sixel::_hidden::*;
    }
}
