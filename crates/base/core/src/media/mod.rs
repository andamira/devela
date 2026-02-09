// devela_base_core::media
//!
#![doc = crate::_DOC_MEDIA!()] // public, root
#![doc = crate::_DOC_MEDIA_MODULES!()]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
// lints
#![cfg_attr(doc, allow(rustdoc::broken_intra_doc_links))]
// safety
#![cfg_attr(feature = "safe_media", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_MEDIA_MODULES =
    crate::_doc!(modules: crate; media: audio, color, image); // draw, glyph, motion, plate, video
}

crate::items! {
    #[cfg(feature = "audio")] #[cfg_attr(nightly_doc, doc(cfg(feature = "audio")))]
    pub mod audio;
    #[cfg(feature = "color")] #[cfg_attr(nightly_doc, doc(cfg(feature = "color")))]
    pub mod color;
    // #[cfg(feature = "draw")] #[cfg_attr(nightly_doc, doc(cfg(feature = "draw")))]
    // pub mod draw;
    // #[cfg(feature = "glyph")] #[cfg_attr(nightly_doc, doc(cfg(feature = "glyph")))]
    // pub mod glyph;
    #[cfg(feature = "image")] #[cfg_attr(nightly_doc, doc(cfg(feature = "image")))]
    pub mod image;
    // #[cfg(feature = "motion")] #[cfg_attr(nightly_doc, doc(cfg(feature = "motion")))]
    // pub mod motion; // WIP
    //
    // pub mod plate; // WIP
    //
    // pub mod video; // WIP
}

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        #[cfg(feature = "audio")] pub use super::audio::_all::*;
        #[cfg(feature = "color")] pub use super::color::_all::*;
        // #[cfg(feature = "draw")] pub use super::draw::_all::*;
        // #[cfg(feature = "glyph")] pub use super::glyph::_all::*;
        #[cfg(feature = "image")] pub use super::image::_all::*;
        // pub use super::motion::_all::*;
        // pub use super::plate::_all::*;
        // pub use super::video::_all::*;
    }
    _crate_internals {
        pub(crate) use super::_DOC_MEDIA_MODULES;
        #[cfg(feature = "color")]
        pub use super::color::_crate_internals::*;
    }
}
