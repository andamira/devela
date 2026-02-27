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
    crate::_doc!(modules: crate; media: audio, visual); // compo, doc, font, motion
}

#[cfg(feature = "audio")]
pub mod audio;
// pub mod compo; // WIP
// pub mod doc; // WIP
// #[cfg(feature = "font")]
// pub mod font;
// #[cfg(feature = "motion")]
// pub mod motion; // WIP
pub mod visual;

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        #[cfg(feature = "audio")] pub use super::audio::_all::*;
        // pub use super::compo::_all::*;
        // pub use super::doc::_all::*;
        // #[cfg(feature = "font")] pub use super::font::_all::*;
        // pub use super::motion::_all::*;
        // TODO: new feature visual
        pub use super::visual::_all::*;
    }
    _crate_internals {
        pub(crate) use super::_DOC_MEDIA_MODULES;
        pub use super::visual::_crate_internals::*;
    }
}
