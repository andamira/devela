// devela::media::color
//
//! Chromatic functionality.
//
// safety
#![cfg_attr(feature = "safe_color", forbid(unsafe_code))]

mod base;
mod error;
mod namespace;

crate::items! { // structural access: doc_inline, all, always
    #[allow(unused)]
    pub use doc_inline::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use always::*;

    mod doc_inline {
        pub use super::{base::*, error::*, namespace::*};
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused)]
    }
}
