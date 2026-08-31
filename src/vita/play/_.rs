// devela/src/vita/play/_.rs
//
#![doc = crate::_DOC_VITA_PLAY!()] // public
#![doc = crate::_doc!(modules: crate::vita; play: game)]
#![doc = crate::_doc!(flat:"vita")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    // pub mod_ comp; // computer and simulated games
    // pub mod_ dance; // expressive movement
    pub mod_ game; // games across cultures
    // pub mod_ music;// music, rhythm, embodied sound
    // pub mod_ sport; // play-oriented physical competition
}
crate::mods_out! { // _pub_mods
    _pub_mods {
        pub use super::{
            // comp::_all::*,
            // dance::_all::*,
            game::_all::*,
            // music::_all::*,
            // sport::_all::*,
        };
    }
}
