// devela::media::draw
//
//! Drawing functionality.
//
// safety
#![cfg_attr(feature = "safe_draw", forbid(unsafe_code))]

mod error;

// structural access
crate::items! {
    mod doc_inline {
        pub use super::error::*;
    }
    #[allow(unused_imports)] pub use doc_inline::*;
    pub(super) mod all { #[doc(inline)] pub use super::doc_inline::*; }
}
