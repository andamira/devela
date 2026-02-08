// devela_base_alloc::num::prob::rand
//
#![doc = crate::_DOC_NUM_PROB_RAND!()] // public
#![doc = crate::_doc!(modules: crate::num::prob; rand)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
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
