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

// structural access
crate::items! { #[allow(unused_imports)]
    pub use {always::*, doc_inline::*};

    mod doc_inline {
        pub use super::r#async::all::*;
        #[allow(unused_imports, reason = "feature-gated")]
        pub use super::{sync::all::*, thread::all::*};
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused_imports)]
        #[doc(hidden)] #[doc(no_inline)]
        pub use super::{r#async::always::*, sync::always::*, thread::always::*};
    }
}
