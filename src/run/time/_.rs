// devela/src/run/time/_.rs
//
#![doc = crate::_DOC_RUN_TIME!()] // public
#![doc = crate::_doc!(modules: crate::run; time)]
#![doc = crate::_doc!(flat:"run")]
#![doc = crate::_doc!(hr)]

crate::mods_in! {
    // mod clock; // RunClock RunDelta
    mod frame; // RunFrame
    // mod_ loop;
    #[cfg(feature = "time")]
    mod pacer; // RunPacer
    mod run; // Runtime
    mod step; // RunStep
    mod tick; // RuntimeTick
}
crate::mods_out! { // _mods
    _mods {
        pub use super::{
            // clock::*,
            frame::*,
            // loop::_all::*,
            run::*,
            step::*,
            tick::*,
        };
        #[cfg(feature = "time")]
        pub use super::{
            pacer::*,
        };
    }
}
