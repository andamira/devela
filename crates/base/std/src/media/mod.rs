// devela_base_std::media
//
#![doc = crate::_DOC_MEDIA!()] // public, root
#![doc = crate::_DOC_MEDIA_MODULES!()]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//
// safety
#![cfg_attr(feature = "safe_media", forbid(unsafe_code))]
// lints
#![cfg_attr(doc, allow(rustdoc::broken_intra_doc_links))]
// docs
crate::CONST! { pub(crate) _DOC_MEDIA_MODULES =
    crate::_doc!(modules: crate; media: visual); // audio, compo, doc, font, video
}

// #[cfg(feature = "audio")]
// pub mod audio;
pub mod visual;

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        // #[cfg(feature = "audio")]
        // pub use super::audio::_all::*;
        pub use super::visual::_all::*;
    }
    _crate_internals {
        pub(crate) use super::_DOC_MEDIA_MODULES;
    }
}
