// devela::media::image::sixel
//
//! [Sixel] encoding functionality.
//!
//! [Sixel]: https://en.wikipedia.org/wiki/Sixel
//

#[cfg(all(feature = "alloc", feature = "term"))]
#[cfg(any(feature = "io", feature = "std"))]
#[cfg(any(feature = "dep_hashbrown", feature = "std"))]
#[cfg_attr(nightly_doc, doc(cfg(all(feature = "alloc", feature = "term"))))]
#[cfg_attr(nightly_doc, doc(cfg(any(feature = "io", feature = "std"))))]
#[cfg_attr(nightly_doc, doc(cfg(any(feature = "dep_hashbrown", feature = "std"))))]
pub mod legacy;

crate::structural_mods! { // _mods
    _mods {
        // re-exports
        pub use devela_base_core::media::image::{
            SixelChar, SixelColor, SixelEncoder, SixelPalette,
        };
    }
    _pub_mods {
        #[cfg(all(feature = "alloc", feature = "term"))]
        #[cfg(any(feature = "io", feature = "std"))]
        #[cfg(any(feature = "dep_hashbrown", feature = "std"))]
        pub use super::legacy::*;
    }
}
