// devela::work::exec
//
#![doc = crate::_DOC_WORK_EXEC!()] // public
#![doc = crate::_doc!(modules: crate::work; exec: process, thread)] // …
#![doc = crate::_doc!(flat:"work")]
#![doc = crate::_doc!(extends: process, thread)]
//

// pub mod fiber;
pub mod process;
// pub mod remote;
pub mod thread;
// pub mod worker;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::{
            // fiber::_all::*,
            process::_all::*,
            // remote::_all::*,
            thread::_all::*,
            // worker::_all::*,
        };
    }
}
