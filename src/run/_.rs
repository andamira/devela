// devela/src/run/_.rs
//
#![doc = crate::_DOC_RUN!()] // public, root
#![doc = crate::_DOC_RUN_MODULES!()]
#![doc = crate::_doc!(flat:"run")]
#![doc = crate::_QUO_RUN!()]
//!
//
// safety
#![cfg_attr(feature = "safe_run", forbid(unsafe_code))]
// docs
crate::CONST! { pub(crate) _DOC_RUN_MODULES =
    crate::_doc!(modules: crate; run: app, cycle, regime, time); // state
}

crate::mods_in! {
    pub mod_ app; // AppControl
    pub mod_ cycle; // RunCycle, RunControl, RunPhase
    mod_ driver; // RunDriver
    mod iface; // RunApp
    mod permission; // Permission<Error|Query|State>
    pub mod_ regime; // RunCap*, RunService
    // pub mod_ state; // WIP
    pub mod_ time; // RunPacer, RunStep, Runtime, RuntimeTick
}
crate::mods_out! { // _mods, _pub_mods, _crate_internals
    _mods {
        pub use super::{
            iface::*,
            driver::_all::*,
            permission::*,
        };
    }
    _pub_mods {
        pub use super::{
            app::_all::*,
            cycle::_all::*,
            regime::_all::*,
            // state::_all::*,
            time::_all::*,
        };
    }
    _crate_internals {
        pub(crate) use super::{
            _DOC_RUN_MODULES,
            driver::_crate_internals::*,
        };
    }
}
