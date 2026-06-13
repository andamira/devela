// devela/src/data/layout/buffer/mod.rs
//
#![doc = crate::_DOC_DATA_LAYOUT_BUFFER!()] // public
#![doc = crate::_doc!(modules: crate::data::layout; buffer)]
#![doc = crate::_doc!(flat:"data")]
#![doc = crate::_doc!(hr)]
//

mod linear; // buffer_linear!
mod ring; // buffer_ring!

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            linear::_all::*,
            ring::_all::*,
        };
    }
}
