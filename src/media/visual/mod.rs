// devela::media::visual
//
#![doc = crate::_DOC_MEDIA_VISUAL!()] // public
#![doc = crate::_doc!(modules: crate::media; visual: color, image)] // draw, video
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//

#[cfg(feature = "color")]
pub mod color;
#[cfg(feature = "draw")]
pub mod draw;
pub mod image;
// pub mod video;

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        #[cfg(feature = "color")] pub use super::color::_all::*;
        #[cfg(feature = "draw")] pub use super::draw::_all::*;
        pub use super::image::_all::*;
        // pub use super::video::_all::*;
    }
    _crate_internals {
        #[cfg(feature = "color")] pub(crate) use super::color::_crate_internals::*;
    }
}
