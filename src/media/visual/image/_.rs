// devela/src/media/visual/image/_.rs
//
#![doc = crate::_DOC_MEDIA_VISUAL_IMAGE!()] // public
#![doc = crate::_doc!(modules: crate::media::visual; image: format, raster, sixel)]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
        mod error;
        mod info; // ImageInfo, ImageFrame<Info|Span>
    pub mod_ format; // Pnm, WIP Jpeg, Png, Qoi…
    pub mod_ raster; // Raster[[Buf|View][Bytes]], Raster<Sample|View>Packed, raster!

    #[cfg(feature = "term")]
    pub mod_ sixel; // SixelChar, SixelColor, SixelEncoder, SixelPalette
}
crate::mods_out! { // _mods, _pub_mods, _crate_internals, _hidden
    _mods {
        pub use super::{
            error::*,
            info::*,
        };
    }
    _pub_mods {
        pub use super::{
            raster::_all::*,
            format::_all::*,
        };
        #[doc(inline)]
        #[cfg(feature = "term")]
        pub use super::sixel::_all::*;
    }
    _crate_internals {
        pub use super::{
            raster::_crate_internals::*,
        };
    }
}
