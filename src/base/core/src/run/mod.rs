// devela_base_core::run
//
#![doc = crate::_DOC_RUN!()] // public, root
#![doc = crate::_DOC_RUN_MODULES!()]
#![doc = crate::_doc!(flat:"run")]
#![doc = crate::_doc!(hr)]
//
// safety
#![cfg_attr(base_safe_run, forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_RUN_MODULES =
    crate::_doc!(modules: crate; run: frame, setup);
}

pub mod frame; // WIP
pub mod setup; // WIP

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        pub use super::{
            frame::_all::*,
            setup::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_RUN_MODULES;
    }
}
