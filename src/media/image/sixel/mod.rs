// devela::media::image::sixel
//
//! [Sixel] encoding functionality.
//!
//! [Sixel]: https://en.wikipedia.org/wiki/Sixel
//
// NOTE: doc features don't show from /all/

#[cfg(all(feature = "alloc", feature = "term"))]
#[cfg(any(feature = "io", feature = "std"))]
#[cfg(any(feature = "dep_hashbrown", feature = "std"))]
pub mod legacy;

crate::structural_mods! { // _mods
    _mods {
        // re-exports
        #[cfg_attr(nightly_doc, doc(cfg(feature = "image")))]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "term")))]
        pub use devela_base_core::media::image::{
            SixelChar, SixelColor, SixelEncoder, SixelPalette,
        };
    }
    _pub_mods {
        #[doc(inline)]
        #[cfg(all(feature = "alloc", feature = "term"))]
        #[cfg(any(feature = "io", feature = "std"))]
        #[cfg(any(feature = "dep_hashbrown", feature = "std"))]
        // #[cfg_attr(nightly_doc, doc(cfg(all(feature = "alloc", feature = "term"))))]
        // #[cfg_attr(nightly_doc, doc(cfg(any(feature = "io", feature = "std"))))]
        // #[cfg_attr(nightly_doc, doc(cfg(any(feature = "dep_hashbrown", feature = "std"))))]
        pub use super::legacy::*;
    }
}
