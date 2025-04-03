// devela::game
//
//! Game-development and interactive applications.
#![doc = crate::doc_!(modules: crate; game: loop, map, state)] // map
#![doc = crate::doc_!(newline)]
//
// safety
#![cfg_attr(feature = "safe_game", forbid(unsafe_code))]

crate::items! { // structural access: _pub_mods, _all
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _pub_mods::*;

    mod _pub_mods { #![allow(unused)]
        // WIPZONE
        // pub use super::loop::_all::*;
        // pub use super::map::_all::*;
        // pub use super::state::_all::*;
    }
    pub(super) mod _all { #![allow(unused)]
        #[doc(inline)]
        pub use super::_pub_mods::*;
    }
}
// WIPZONE
// pub mod loop;
// pub mod map;
// pub mod state;
