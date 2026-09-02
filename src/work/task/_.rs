// devela/src/work/task/_.rs
//
#![doc = crate::_DOC_WORK_TASK!()] // public
#![doc = crate::_doc!(modules: crate::work; task: coro)]
#![doc = crate::_doc!(flat:"work")]
// #![doc = crate::_doc!(extends: ops)]
#![doc = crate::_doc!(hr)]
//

crate::mods_in! {
    // pub mod actor;
    pub mod_ coro;
    // mod kernel;
    // pub mod state;
}
crate::mods_out! { // _mods, _pub_mods
    _mods {
        // pub use super::kernel::*;
    }
    _pub_mods {
        pub use super::{
            // actor::_all::*,
            coro::_all::*,
            // state::_all::*,
        };
    }
}
