// devela/src/data/layout/array/owned/vec/_.rs
//
//! Dynamic arrays.
//!
//! Vectors are random-access, sequentially allocated, *dynamically* sized,
//! homogeneous data structures.
//

crate::mods_in! {
    mod _reexport_alloc;

    mod ext; // VecExt
    // mod d2; // Vec2d WIP
    // mod hvec; // HVec ? WIP
}
crate::mods_out! { // _mods, _reexports
    _mods {
        pub use super::{
            ext::*,
        };
        // pub use super::d2::_all::*;
        // pub use super::hvec::_all::*;
    }
    _reexports {
        pub use super::_reexport_alloc::{Vec, vec_};
    }
}
