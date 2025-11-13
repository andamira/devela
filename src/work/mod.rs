// devela::work
//
#![doc = crate::_DOC_WORK!()]
#![doc = crate::_doc!(modules: crate; work: future, process, sync, thread)]
#![doc = "<br/><hr>"] // gives way to zall
//!
#![doc = crate::_doc!(extends: future, process, sync, task, thread)]
//
// safety
#![cfg_attr(feature = "safe_work", forbid(unsafe_code))]

pub mod future;
pub mod process;
pub mod sync;
pub mod thread;

// WIPZONE
// pub mod actor;
// pub mod fiber;

crate::structural_mods! { // _pub_mods
    _pub_mods {
        pub use super::future::_all::*;
        #[allow(unused, reason = "feature-gated")]
        pub use super::{process::_all::*, sync::_all::*, thread::_all::*};
        // WIPZONE:
        // pub use super::actor::*;
        // pub use super::fiber::*;
    }
}
