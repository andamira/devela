// devela/src/media/visual/mod.rs
//
#![doc = crate::_DOC_MEDIA_VISUAL!()] // public
#![doc = crate::_doc!(modules: crate::media; visual: color, draw, image)] // pattern, video
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//

pub mod color;
#[cfg(feature = "draw")]
pub mod draw;
pub mod image;
// pub mod lattice; // WIP
// pub mod video; //

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        pub use super::{
            color::_all::*,
            image::_all::*,
            // lattice::_all::*,
            // video::_all::*,
        };
        #[cfg(feature = "draw")]
        pub use super::draw::_all::*;
    }
    _crate_internals {
        pub(crate) use super::color::_crate_internals::*;
    }
}
