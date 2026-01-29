// devela_base_core::num::prob::rand
//
#![doc = crate::_DOC_NUM_PROB_RAND!()]
//!
//

// mod noise; // Structured deterministic randomness
mod prng; // concrete PRNGs
mod rand; // Rand

crate::structural_mods! { // _mods, _crate_internals
    _mods {
        pub use super::{
            prng::_all::*,
            rand::*,
            // noise::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::prng::_crate_internals::*;
    }
}
