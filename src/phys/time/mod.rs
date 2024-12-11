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

crate::items! { // structural access: doc_inline, all, always
    #[allow(unused)]
    pub use doc_inline::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use always::*;

    mod doc_inline {
        pub use super::reexports::*;

        #[cfg(feature = "time")]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "time")))]
        pub use super::{calendar::*, error::*, fmt::*, no::*, split::*, unix::*};
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused)]
        pub use super::reexports::*;
    }
}
