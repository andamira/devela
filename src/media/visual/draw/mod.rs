// devela::media::visual::draw
//
#![doc = crate::_DOC_MEDIA_VISUAL_DRAW!()] // public
#![doc = crate::_doc!(modules: crate::media::visual; draw)]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//
// safety
#![cfg_attr(feature = "safe_draw", forbid(unsafe_code))]

// mod buffer;
// mod grid;
// #[cfg(feature = "shape")]
// #[cfg_attr(nightly_doc, doc(cfg(feature = "shape")))]
// mod line;

crate::structural_mods! { // _mods, _reexports
    _mods {
        // pub use super::buffer::*;
        // pub use super::grid::*;
        // #[cfg(feature = "shape")]
        // pub use super::line::*;
    }
    _reexports {
        pub use devela_base_core::media::visual::draw::{
            Canvas, CanvasRead, CanvasTextel,
        };
    }
}
