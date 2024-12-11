// devela::text::fmt
//
//! Formatting strings.
//!
#![doc = crate::doc_!(extends: fmt)]
//

mod reexports;
mod buf;

#[cfg(feature = "fmt")]
mod num_to_str;

crate::items! { // structural access: doc_inline, all, always
    #[allow(unused)]
    pub use doc_inline::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use always::*;

    mod doc_inline {
        pub use super::{buf::*, reexports::*};

        #[cfg(feature = "fmt")]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "fmt")))]
        pub use super::num_to_str::*;
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused)]
        pub use super::reexports::*;
    }
}
