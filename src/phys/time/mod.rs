// devela::phys::time
//
//! Time and calendar types and operations.
//!
#![doc = crate::doc_!(extends: time)]
//
// safety
#![cfg_attr(feature = "safe_time", forbid(unsafe_code))]

mod reexports;

#[cfg(feature = "time")]
crate::items! {
    mod calendar;
    mod error;
    mod fmt;
    mod no;
    mod split;
    mod unix;
}

// structural access
crate::items! { #[allow(unused_imports)]
    pub use {always::*, doc_inline::*};

    mod doc_inline {
        pub use super::reexports::*;
        #[cfg(feature = "time")]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "time")))]
        pub use super::{calendar::*, error::*, fmt::*, no::*, split::*, unix::*};
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused_imports)]
        #[doc(hidden)] #[doc(no_inline)]
        pub use super::reexports::*;
    }
}
