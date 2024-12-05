// devela::data::iter
//
//! Composable external iteration.
//!
#![doc = crate::doc_!(extends: iter)]
//

mod reexports;
#[allow(unused_imports)]
pub use reexports::*;

// structural access
crate::items! {
    mod doc_inline {
        pub use super::reexports::*;
    }
    #[allow(unused_imports)] pub use doc_inline::*;
    pub(super) mod all { #[doc(inline)] pub use super::doc_inline::*; }
}
