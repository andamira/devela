// devela_base_alloc::media
//
#![doc = crate::_tags!(wip)]
#![doc = crate::_DOC_MEDIA!()] // public
#![doc = crate::_DOC_MEDIA_MODULES!()]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//
// safety
#![cfg_attr(base_safe_lang, forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_MEDIA_MODULES =
    crate::_doc!(modules: crate; media); // audio, color, draw, font, image, video
}

// pub mod image;

crate::structural_mods! { // _crate_internals
    // _pub_mods {
    //     pub use super::{
    //         image::_all::*,
    //     };
    // }
    _crate_internals {
        pub(crate) use super::_DOC_MEDIA_MODULES;
    }
}
