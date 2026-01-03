// devela::data::list::array::vec
//
//! Vectors,
// #![doc = crate::_doc!(extends: vec)]
// #![doc = crate::_doc!(modules: crate::data::collections; vec)]
// #![doc = crate::_doc!(newline)]
//!
//! Vectors are random-access, sequentially allocated, *dynamically* sized,
//! homogeneous data structures.
//

mod _reexport_alloc; // SYMLINK to /libs/base_alloc/src/data/list/array/vec/_reexport.rs

mod chunk; // VecChunk
mod ext; // VecExt
// mod d2; // Vec2d WIP
// mod hvec; // HVec ? WIP

crate::structural_mods! { // _mods, _reexports
    _mods {
        pub use super::{
            chunk::*,
            ext::*,
        };
        // pub use super::d2::_all::*;
        // pub use super::hvec::_all::*;
    }
    _reexports {
        pub use super::_reexport_alloc::*;
    }
}
