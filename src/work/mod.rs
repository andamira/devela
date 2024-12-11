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

crate::items! { // structural access: doc_inline, all, always
    #[allow(unused)]
    pub use doc_inline::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use always::*;

    mod doc_inline {
        pub use super::r#async::all::*;

        #[allow(unused, reason = "feature-gated")]
        pub use super::{sync::all::*, thread::all::*};
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused)]
        pub use super::{r#async::always::*, sync::always::*, thread::always::*};
    }
}
