// devela/src/run/cycle/_.rs
//
#![doc = crate::_DOC_RUN_CYCLE!()] // public
#![doc = crate::_doc!(modules: crate::run; cycle)]
#![doc = crate::_doc!(flat:"run")]
#![doc = crate::_doc!(hr)]

crate::mods_in! {
    mod cycle; // RunCycle, RunControl, RunPhase
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            cycle::*,
        };
    }
}
