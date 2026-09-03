// devela/src/media/visual/image/format/_.rs
//
#![doc = crate::_DOC_MEDIA_VISUAL_IMAGE_FORMAT!()] // public
#![doc = crate::_doc!(modules: crate::media::visual::image; format)]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    // mod_ jpeg; // FUTURE
    mod_ netpbm; // Pnm
    // #[cfg(feature = "alloc"] // TEMP
    // mod_ png; // WIP
    // mod_ qoi; // WIP
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            // jpg::_all::*,
            netpbm::_all::*,
            // png::_all::*,
            // qoi::_all::*,
        };
    }
}
