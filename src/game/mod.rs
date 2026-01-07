// devela::game
//
#![doc = crate::_DOC_GAME!()]
#![doc = crate::_DOC_GAME_MODULES!()]
#![doc = crate::_doc!(flat:"game")]
#![doc = crate::_doc!(newline)]
//
// safety
#![cfg_attr(feature = "safe_game", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_GAME_MODULES =
    crate::_doc!(modules: crate; game: loop, map, state); // map
}

pub mod r#loop;
pub mod map;
pub mod state;

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        pub use super::{
            r#loop::_all::*,
            map::_all::*,
            state::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_GAME_MODULES;
    }
}
