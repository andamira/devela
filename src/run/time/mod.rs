// devela::run::time
//
#![doc = crate::_tags!(wip)]
#![doc = crate::_DOC_RUN_TIME!()] // public
#![doc = crate::_doc!(modules: crate::run; time)]
#![doc = crate::_doc!(flat:"run")]
#![doc = crate::_doc!(hr)]

mod frame; // FramePacer // WIP
// mod run; // Runtime // WIP
mod tick; // RuntimeTick

crate::structural_mods! { // _mods
    _mods {
        pub use super::{
            frame::*,
            // run::*,
            tick::*,
        };
    }
}
