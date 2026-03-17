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
    crate::_doc!(modules: crate; run: cycle, time); // regime, state,
}

mod iface; // RunApp

pub mod cycle; // RunCycle, RunControl, RunPhase
// mod regime;
// mod state;
pub mod time; // RunPacer, RunStep, Runtime, RuntimeTick

crate::structural_mods! { // _pub_mods, _crate_internals
    _mods {
        pub use super::{
            iface::*,
        };
    }
    _pub_mods {
        pub use super::{
            cycle::_all::*,
            // regime::_all::*,
            // state::_all::*,
            time::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::_DOC_RUN_MODULES;
    }
}
