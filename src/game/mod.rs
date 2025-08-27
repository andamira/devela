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

crate::items! { // structural access: _pub_mods, _all
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _pub_mods { #![allow(unused)]
        pub use super::{r#loop::_all::*, map::_all::*, state::_all::*};
        // WIPZONE
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_pub_mods::*;
    }
}
// WIPZONE
