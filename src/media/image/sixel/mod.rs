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

crate::structural_mods! { // _mods, _pub_mods, _hidden
    _mods {
        // re-exports
        #[cfg_attr(nightly_doc, doc(cfg(feature = "image")))]
        #[cfg_attr(nightly_doc, doc(cfg(feature = "term")))]
        pub use devela_base_core::media::image::sixel:: {
            SixelChar, SixelColor, SixelEncoder, SixelPalette,
        };
        crate::__dbg! { crate::slog!{ @
            #[cfg_attr(nightly_doc, doc(cfg(feature = "image")))]
            #[cfg_attr(nightly_doc, doc(cfg(feature = "term")))]
            #[cfg_attr(nightly_doc, doc(cfg(feature = "__dbg")))]
            pub use static: sixel_encoder:64+64 in devela_base_core
        } }
    }
    _pub_mods {
        #[doc(inline)]
        #[cfg(all(feature = "alloc", feature = "term"))]
        #[cfg(any(feature = "io", feature = "std"))]
        #[cfg(any(feature = "dep_hashbrown", feature = "std"))]
        // FIX
        #[cfg_attr(nightly_doc, doc(cfg(all(feature = "alloc", feature = "term"))))]
        #[cfg_attr(nightly_doc, doc(cfg(any(feature = "io", feature = "std"))))]
        #[cfg_attr(nightly_doc, doc(cfg(any(feature = "dep_hashbrown", feature = "std"))))]
        pub use super::legacy::*;
    }
    _hidden {
        crate::__dbg! {
            crate::slog![@#[doc(hidden)] pub use fn: sixel_encoder:64+64 in devela_base_core];
        }
    }
}
