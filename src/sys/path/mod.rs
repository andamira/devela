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

// structural access
crate::items! {
    mod doc_inline {
        #[cfg(all(feature = "sys", feature = "std"))]
        pub use super::project::*;
    }
    #[allow(unused_imports)] pub use doc_inline::*;
    pub(super) mod all { #[doc(inline)]
        #[allow(unused_imports, reason = "feature-gated")]
        pub use super::doc_inline::*;
    }
}
