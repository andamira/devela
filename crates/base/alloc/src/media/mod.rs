// devela_base_alloc::media
//
#![doc = crate::_tags!(wip)]
#![doc = crate::_DOC_MEDIA!()] // public
#![doc = crate::_DOC_MEDIA_MODULES!()]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//
// safety
#![cfg_attr(feature = "safe_lang", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_MEDIA_MODULES =
    crate::_doc!(modules: crate; media); // audio, compo, doc, font, visual
}

// pub mod visual;

crate::structural_mods! { // _crate_internals
    // _pub_mods {
    //     pub use super::{
    //         visual::_all::*,
    //     };
    // }
    _crate_internals {
        pub(crate) use super::_DOC_MEDIA_MODULES;
    }
}
