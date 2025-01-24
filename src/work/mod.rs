// devela::work
//
//! Work management, concurrency handling.
#![doc = crate::doc_!(modules: crate; work: future, process, sync)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: future, process, sync, task, thread)]
//
// safety
#![cfg_attr(feature = "safe_work", forbid(unsafe_code))]

pub mod future;
pub mod process;
pub mod sync;

crate::items! { // structural access: _pub_mods, _all, _always
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use {_always::*, _pub_mods::*};

    mod _pub_mods { #![allow(unused)]
        pub use super::future::_all::*;
        #[allow(unused, reason = "feature-gated")]
        pub use super::{process::_all::*, sync::_all::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_pub_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::{future::_always::*, sync::_always::*, process::_always::*};
    }
}
