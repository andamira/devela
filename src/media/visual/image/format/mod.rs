// devela/src/media/visual/image/format/mod.rs
//
#![doc = crate::_DOC_MEDIA_VISUAL_IMAGE_FORMAT!()] // public
#![doc = crate::_doc!(modules: crate::media::visual::image; format)]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//

// mod jpg; // FUTURE
mod netpbm; // Pnm
// #[cfg(feature = "alloc"] // TEMP
// mod png; // WIP
// mod qoi; // WIP

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            // jpg::*,
            netpbm::_all::*,
            // png::_all::*,
            // qoi::*,
        };
    }
}
