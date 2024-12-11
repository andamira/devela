// devela::ui::layout
//
//! Layout functionality.
//
// safety
#![cfg_attr(feature = "safe_layout", forbid(unsafe_code))]

mod error;

crate::items! { // structural access: doc_inline, all
    #[allow(unused)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::error::*;
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
