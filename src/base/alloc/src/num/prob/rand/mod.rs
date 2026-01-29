// devela_base_alloc::num::rand
//
#![doc = crate::_DOC_NUM_PROB_RAND!()]
//!
//

// mod noise; // Structured deterministic randomness
mod rand; // RandAlloc

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            rand::*,
        };
    }
}
