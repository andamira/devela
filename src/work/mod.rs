// devela::work
//
//! Work management, concurrency handling.
#![doc = crate::doc_!(modules: crate; work)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: future, process, sync, task, thread)]
//
// safety
#![cfg_attr(feature = "safe_work", forbid(unsafe_code))]

mod r#async;
mod sync;
mod thread;

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::r#async::_all::*;

        #[allow(unused, reason = "feature-gated")]
        pub use super::{sync::_all::*, thread::_all::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::{r#async::_always::*, sync::_always::*, thread::_always::*};
    }
}
