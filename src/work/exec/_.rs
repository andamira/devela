// devela/src/work/exec/_.rs
//
#![doc = crate::_DOC_WORK_EXEC!()] // public
#![doc = crate::_doc!(modules: crate::work; exec: process, thread)] // …
#![doc = crate::_doc!(flat:"work")]
#![doc = crate::_doc!(extends: process, thread)]
//

crate::mods_in! {
    // pub mod_ container;
    // pub mod_ fiber;
    pub mod_ process;
    // pub mod_ remote;
    pub mod_ thread;
    // pub mod_ worker;
}
crate::mods_out! { // _pub_mods, _hidden
    _pub_mods {
        pub use super::{
            // container::_all::*,
            // fiber::_all::*,
            process::_all::*,
            // remote::_all::*,
            thread::_all::*,
            // worker::_all::*,
        };
    }
    _hidden {
        pub use super::{
            process::_hidden::*,
        };
    }
}
