// devela::media::visual
//
#![doc = crate::_DOC_MEDIA_VISUAL!()] // public
#![doc = crate::_doc!(modules: crate::media; visual: color, image)] // draw, video
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//

#[cfg(feature = "draw")]
mod draw;

#[cfg(feature = "color")]
pub mod color;
#[cfg(feature = "image")]
pub mod image;
// pub mod video;

crate::structural_mods! { // _mods, _pub_mods, _hidden
    _mods {
        #[cfg(feature = "draw")] pub use super::draw::_all::*;
    }
    _pub_mods {
        #[cfg(feature = "color")] pub use super::color::_all::*;
        #[cfg(feature = "image")] pub use super::image::_all::*;
        // pub use super::video::_all::*;
    }
    _hidden {
        #[cfg(feature = "image")] pub use super::image::_hidden::*;
    }
}
