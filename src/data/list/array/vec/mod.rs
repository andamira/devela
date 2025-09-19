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

mod chunk; // VecChunk
mod ext; // ExtVec
mod reexports;

// WIPZONE
// mod d2; // Vec2d
// mod hvec; // HVec ?

crate::structural_mods! { // _mods
    _mods {
        pub use super::{chunk::*, ext::*, reexports::*};
        // WIPZONE
        // pub use super::d2::_all::*;
        // pub use super::hvec::_all::*;
    }
}
