// devela_base_core::media::visual::draw
//
#![doc = crate::_DOC_MEDIA_VISUAL_DRAW!()] // public
#![doc = crate::_doc!(modules: crate::media::visual; draw: compose, line)]
#![doc = crate::_doc!(flat:"media")]
#![doc = crate::_doc!(hr)]
//
// safety
// #![cfg_attr(feature = "safe_draw", forbid(unsafe_code))] // no feature

mod traits; // WIP Canvas

// pub mod compose; // WIP
// pub mod line; // WIP

crate::structural_mods! { // _pub_mods
    _mods {
        pub use super::{
            traits::*,
        };
    }
    _pub_mods {
        // pub use super::{
        //     compose::*,
        //     line::*,
        // };
    }
}
