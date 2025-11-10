// devela_base_core::sys::mem::arena
//
//!
//

mod define; // defina_arena!
mod handle; // ArenaHandle
mod internals;

// WIPZONE
// mod field; // ArenaField
// mod handle_macro;
// mod primitive; // ArenaPrimitive
// mod recipe; // ArenaRecipe

crate::structural_mods! { // _mods, hidden
    _mods {
        pub use super::{
            define::*,
            handle::*,
            internals::*,
            // WIPZONE
            // handle_macro::*, // TODO
            // field::*,
            // primitive::*,
            // recipe::*,
        };
    }
    _hidden {
        pub use super::internals::*;
    }
}
