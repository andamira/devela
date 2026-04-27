// devela::lang::prog::template
//
#![doc = crate::_DOC_LANG_PROG_TEMPLATE!()] // public
#![doc = crate::_doc!(modules: crate::lang::prog; template)]
#![doc = crate::_doc!(flat:"lang")]
#![doc = crate::_doc!(hr)]
//

// mod core;

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            core::_all::*,
        };
    }
}
