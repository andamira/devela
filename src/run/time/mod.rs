// devela::run::time
//
#![doc = crate::_DOC_RUN_TIME!()] // public
#![doc = crate::_doc!(modules: crate::run; time)]
#![doc = crate::_doc!(flat:"run")]
#![doc = crate::_doc!(hr)]

// mod clock; // RunClock RunDelta
mod frame; // RunFrame
// mod loop;
mod pacer; // RunPacer
mod run; // Runtime
mod step; // RunStep
mod tick; // RuntimeTick

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            // clock::*,
            frame::*,
            // loop::*,
            pacer::*,
            run::*,
            step::*,
            tick::*,
        };
    }
}
