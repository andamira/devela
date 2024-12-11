// devela::sys::path
//
//! Paths.
#![doc = crate::doc_!(extends: path)]
#![doc = crate::doc_!(modules: crate::sys; path)]
#![doc = crate::doc_!(newline)]
//!
//

#[cfg(all(feature = "sys", feature = "std"))]
mod project;

crate::items! { // structural access: doc_inline, all
    #[allow(unused)]
    pub use doc_inline::*;

    mod doc_inline {
        #[cfg(all(feature = "sys", feature = "std"))]
        pub use super::project::*;
    }
    pub(super) mod all {
        #[allow(unused_imports, reason = "feature-gated")]
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
