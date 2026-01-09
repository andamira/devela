// devela::run::setup
//
#![doc = crate::_DOC_RUN_SETUP!()]
//

mod cap; // Cap*

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            cap::*,
        };
    }
}
