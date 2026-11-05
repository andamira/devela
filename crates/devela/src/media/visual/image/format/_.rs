//
#![doc = crate::_DOC_MEDIA_VISUAL_IMAGE_FORMAT!()] // public
#![doc = crate::_doc!(modules: crate::media::visual::image; format)]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    // mod_ jpeg; // FUTURE
    #[cfg(feature = "pnm")]
    mod_ netpbm; // Pnm
    // #[cfg(feature = "alloc"] // TEMP
    // mod_ png; // WIP
    // mod_ qoi; // WIP
}
crate::mods_out! { // _mods
    _mods {
        // pub use super::jpg::_all::*;
        #[cfg(feature = "pnm")]
        pub use super::netpbm::_all::*;
        // pub use super::png::_all::*;
        // pub use super::qoi::_all::*;
    }
}
