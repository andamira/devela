// devela::media::image::sixel
//
//! [Sixel] encoding functionality.
//!
//! [Sixel]: https://en.wikipedia.org/wiki/Sixel
//

crate::structural_mods! { // _reexports, _hidden
    _reexports {
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
    _hidden {
        crate::__dbg! {
            crate::slog![@#[doc(hidden)] pub use fn: sixel_encoder:64+64 in devela_base_core];
        }
    }
}
