// devela/src/code/hint/mod.rs
//
#![doc = crate::_DOC_CODE_HINT!()] // public
#![doc = crate::_doc!(modules: crate::code; hint)]
#![doc = crate::_doc!(flat: "code")]
#![doc = crate::_doc!(extends: hint)]
//

mod _reexport_core;

mod likely;

crate::structural_mods! { // _reexports
    _mods {
        pub use super::{
            likely::{likely, unlikely}
        };
    }
    _reexports {
        pub use super::_reexport_core::*;
    }
}
