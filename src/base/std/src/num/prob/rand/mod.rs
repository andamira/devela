// devela_base_std::num::prob::rand
//
#![doc = crate::_DOC_NUM_PROB_RAND!()]
//

// mod noise; // Structured deterministic randomness
mod rand; // RandStd

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            rand::*,
        };
    }
}
