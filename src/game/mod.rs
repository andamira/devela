// devela::game
//
#![doc = crate::_DOC_GAME!()]
#![doc = crate::_doc!(modules: crate; game: loop, map, state)] // map
#![doc = crate::_doc!(newline)]
//
// safety
#![cfg_attr(feature = "safe_game", forbid(unsafe_code))]

pub mod r#loop;
pub mod map;
pub mod state;

// WIPZONE

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{r#loop::_all::*, map::_all::*, state::_all::*};

        // WIPZONE
    }
}
