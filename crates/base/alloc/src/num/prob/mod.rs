// devela_base_alloc::num::prob
//
#![doc = crate::_DOC_NUM_PROB!()] // public
#![doc = crate::_doc!(modules: crate::num; prob: rand)]
#![doc = crate::_doc!(flat:"num")]
#![doc = crate::_doc!(hr)]
//!
//! > What is the structure of uncertainty?
//

pub mod rand; // Random number generation

crate::structural_mods! { // _mods, _pub_mods
    _pub_mods {
        pub use super::{
            rand::_all::*,
        };
    }
}
