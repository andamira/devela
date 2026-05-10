// devela::sys::mem::arena
//
//!
//

mod define; // defina_arena!
mod internals; // __Arena
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
        pub use super::internals::*;
    }
}
