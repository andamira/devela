// devela_base_core::run
//
#![doc = crate::_DOC_RUN!()] // public, root
#![doc = crate::_DOC_RUN_MODULES!()]
#![doc = crate::_doc!(flat:"run")]
#![doc = crate::_doc!(hr)]
//
// safety
#![cfg_attr(feature = "safe_run", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_RUN_MODULES =
    crate::_doc!(modules: crate; run: regime, state, time);
}

// mod cycle;
pub mod regime;
pub mod state;
pub mod time;

crate::structural_mods! { // _pub_mods, _crate_internals
    _pub_mods {
        pub use super::{
            regime::_all::*,
            state::_all::*,
            time::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_RUN_MODULES;
    }
}
