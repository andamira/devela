// devela/src/data/store/arena/byte/mod.rs
//
//!
//

mod define; // arena_bytes!
mod _internal; // __ArenaBytes
// mod field; // WIP ArenaField
// mod primitive; // WIP ArenaPrimitive
// mod recipe; // WIP ArenaRecipe

crate::structural_mods! { // _mods, _hidden
    _mods {
        pub use super::{
            define::*,
            // field::*,
            // primitive::*,
            // recipe::*,
        };
    }
    _hidden {
        pub use super::_internal::*;
    }
}
