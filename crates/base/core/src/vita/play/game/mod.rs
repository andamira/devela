// devela_base_core::vita::play::game
//
#![doc = crate::_DOC_VITA_PLAY_GAME!()] // public
#![doc = crate::_doc!(modules: crate::vita::play; game)]
#![doc = crate::_doc!(flat:"vita")]
#![doc = crate::_doc!(hr)]
//

mod action; // actions taken within formal play
mod cycle; // cycles grouping turns or repeated phases TEMP
mod legacy; // persistent continuity across sessions TEMP
mod outcome; // resolved results of actions, rounds, or sessions
mod phase; // named phases within turn or round structure TEMP
mod role; // rule-bearing roles assumed in play
mod session; // bounded instances of play activity
mod turn; // bounded opportunities for one side to act

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            action::*,
            cycle::*,
            legacy::*,
            outcome::*,
            phase::*,
            role::*,
            session::*,
            turn::*,
        };
    }
}
