// devela_base_std::media::visual
//
#![doc = crate::_DOC_MEDIA_VISUAL!()] // public
#![doc = crate::_doc!(modules: crate::media; visual: color)] // draw, image, video
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//

#[cfg(feature = "color")]
pub mod color;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        #[cfg(feature = "color")] pub use super::color::_all::*;
    }
}
