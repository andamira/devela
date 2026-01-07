// devela_base_core::media
//
#![doc = crate::_DOC_MEDIA!()]
#![doc = crate::_DOC_MEDIA_MODULES!()]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(newline)]
//
// safety
#![cfg_attr(base_safe_media, forbid(unsafe_code))]
//
#![cfg_attr(doc, allow(rustdoc::broken_intra_doc_links))]
// docs
crate::CONST! { pub(crate) _DOC_MEDIA_MODULES =
    crate::_doc!(modules: crate; media: audio, color, image); // draw, font, video
}

#[cfg(feature = "audio")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "audio")))]
pub mod audio;

#[cfg(feature = "color")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "color")))]
pub mod color;

// #[cfg(feature = "draw")]
// #[cfg_attr(nightly_doc, doc(cfg(feature = "draw")))]
// pub mod draw;

#[cfg(feature = "image")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "image")))]
pub mod image;

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        #[cfg(feature = "audio")]
        pub use super::audio::_all::*;

        #[cfg(feature = "color")]
        pub use super::color::_all::*;

        // #[cfg(feature = "draw")]
        // pub use super::draw::_all::*;

        #[cfg(feature = "image")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "image")))]
        pub use super::image::_all::*;
    }
    _crate_internals {
        pub(crate) use super::_DOC_MEDIA_MODULES;
        #[cfg(feature = "color")]
        pub use super::color::_crate_internals::*;
    }
}
