// devela::sys::mem::cell
//
//! Shareable mutable containers.
//!
#![doc = crate::doc_!(extends: cell)]
//

mod option;
mod reexports;

crate::items! { // structural access: doc_inline, all, always
    #[allow(unused)]
    pub use doc_inline::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use always::*;

    mod doc_inline {
        pub use super::{option::*, reexports::*};
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused)]
        pub use super::reexports::*;
    }
}
