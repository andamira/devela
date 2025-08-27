// devela::work
//
#![doc = crate::_DOC_WORK!()]
#![doc = crate::_doc!(modules: crate; work: future, process, sync, thread)]
#![doc = crate::_doc!(newline)]
//!
#![doc = crate::_doc!(extends: future, process, sync, task, thread)]
//
// safety
#![cfg_attr(feature = "safe_work", forbid(unsafe_code))]

pub mod future;
pub mod process;
pub mod sync;
pub mod thread;

crate::items! { // structural access: _pub_mods, _all, _always
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use {_always::*, _pub_mods::*};

    mod _pub_mods { #![allow(unused)]
        pub use super::future::_all::*;
        #[allow(unused, reason = "feature-gated")]
        pub use super::{process::_all::*, sync::_all::*, thread::_all::*};
        // WIPZONE:
        // pub use super::actor::*;
        // pub use super::fiber::*;
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_pub_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::{
            future::_always::*, process::_always::*, sync::_always::*, thread::_always::*,
        };
    }
}
// WIPZONE
// pub mod actor;
// pub mod fiber;
