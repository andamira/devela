// devela::media
//
#![doc = crate::_DOC_MEDIA!()] // public, root
#![doc = crate::_DOC_MEDIA_MODULES!()]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//
// safety
#![cfg_attr(feature = "safe_media", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_MEDIA_MODULES =
    crate::_doc!(modules: crate; media: audio, color, font, image); // draw, plate, video
}

#[cfg(feature = "draw")]
mod draw;

#[cfg(feature = "audio")]
pub mod audio;
#[cfg(feature = "color")]
pub mod color;
#[cfg(feature = "font")]
pub mod font;
#[cfg(feature = "image")]
pub mod image;
// #[cfg(feature = "video")]
// #[cfg_attr(nightly_doc, doc(cfg(feature = "video")))]
// pub mod video;

crate::structural_mods! { // _pub_mods, _crate_internals, _hidden
    _pub_mods {
        #[cfg(feature = "audio")] pub use super::audio::_all::*;
        #[cfg(feature = "color")] pub use super::color::_all::*;
        #[cfg(feature = "draw")]  pub use super::draw::_all::*;
        #[cfg(feature = "font")]  pub use super::font::_all::*;
        #[cfg(feature = "image")] pub use super::image::_all::*;
        // #[cfg(feature = "video")] pub use super::video::_all::*;
    }
    _crate_internals {
        pub(crate) use super::_DOC_MEDIA_MODULES;
    }
    _hidden {
        #[cfg(feature = "image")] pub use super::image::_hidden::*;
    }
}
