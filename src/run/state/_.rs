// devela/src/run/state/_.rs
//
#![doc = crate::_tags!(wip)]
#![doc = crate::_DOC_RUN_STATE!()] // public
#![doc = crate::_doc!(modules: crate::run; state)]
#![doc = crate::_doc!(flat:"run")]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    // mod context;
    // mod_ log;
    mod_ machine; // WIP
    // mod_ scene;
}
crate::mods_out! { // _mods, _reexports
    _mods {
        pub use super::{
            // context::*,
            // log::*,
            machine::_all::*,
            // scene::*,
        };
    }
    _reexports {
        // pub use devela::run::state::{
        // }
    }
}
