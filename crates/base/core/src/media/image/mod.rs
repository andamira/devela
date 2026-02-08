// devela_base_core::media::image
//
#![doc = crate::_DOC_MEDIA_IMAGE!()] // public
#![doc = crate::_doc!(modules: crate::media; image: sixel)]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//
// safety
// #![cfg_attr(feature = "safe_image", forbid(unsafe_code))] // no feature

/* image formats */

// #[cfg(feature = "alloc"] // TEMP
// mod png;
//
// mod pnm;

#[cfg(feature = "term")]
#[cfg_attr(nightly_doc, doc(cfg(feature = "term")))]
pub mod sixel; // SixelChar, SixelColor, SixelEncoder, SixelPalette

crate::structural_mods! { // _pub_mods
    _pub_mods {
        // #[cfg(feature = "alloc"]
        // pub use super::png::*;
        //
        // pub use super::pnm::*;

        #[cfg(feature = "term")]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "term")))]
        pub use super::sixel::_all::*;
    }
}
