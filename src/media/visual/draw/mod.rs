// devela::media::visual::draw
//
#![doc = crate::_DOC_MEDIA_VISUAL_DRAW!()] // public
#![doc = crate::_doc!(modules: crate::media::visual; draw)]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//
// safety
#![cfg_attr(feature = "safe_draw", forbid(unsafe_code))]

mod traits; // Canvas, CanvasRead, CanvasTextel

// mod buffer;
// pub mod compose; // WIP
// mod dpi; // WIP
// mod grid;
// #[cfg(feature = "shape")]
// #[cfg_attr(nightly_doc, doc(cfg(feature = "shape")))]
// pub mod line; // WIP

crate::structural_mods! { // _mods, _pub_mods, _reexports
    _mods {
        pub use super::{
            // buffer::*,
            // dpi::*,
            // grid::*,
            traits::*,
        };
        // pub use super::buffer::*;
        // pub use super::grid::*;
        // #[cfg(feature = "shape")]
        // pub use super::line::*;
    }
    _pub_mods {
        // pub use super::{
        //     compose::*,
        //     line::*,
        // };
    }
}
