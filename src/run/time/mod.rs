// devela::run::time
//
#![doc = crate::_DOC_RUN_TIME!()] // public
#![doc = crate::_doc!(modules: crate::run; time)]
#![doc = crate::_doc!(flat:"run")]
#![doc = crate::_doc!(hr)]

crate::structural_mods! { // _mods, _reexports
    _reexports {
        pub use devela_base_core::run::time::{
            RunPacer, RunStep, Runtime, RuntimeTick,
        };
    }
}
