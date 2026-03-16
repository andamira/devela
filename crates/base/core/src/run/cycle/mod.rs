// devela_base_core::run::cycle
//
#![doc = crate::_DOC_RUN_CYCLE!()] // public
#![doc = crate::_doc!(modules: crate::run; cycle)]
#![doc = crate::_doc!(flat:"run")]
#![doc = crate::_doc!(hr)]

mod cycle; // RunCycle, RunControl, RunPhase

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            cycle::*,
        };
    }
}
