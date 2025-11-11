// devela_base_core::sys::mem::arena
//
//!
//

mod define; // defina_arena!
mod internals;

// WIPZONE
// mod field; // ArenaField
// mod primitive; // ArenaPrimitive
// mod recipe; // ArenaRecipe

crate::structural_mods! { // _mods, hidden
    _mods {
        pub use super::{
            define::*,
            internals::*,
            // WIPZONE
            // field::*,
            // primitive::*,
            // recipe::*,
        };
    }
    _hidden {
        pub use super::internals::*;
    }
}
