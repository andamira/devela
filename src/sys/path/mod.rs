// devela::sys::path
//
//! Paths.
#![doc = crate::doc_!(extends: path)]
#![doc = crate::doc_!(modules: crate::sys; path)]
#![doc = crate::doc_!(newline)]
//!
//

#[cfg(feature = "sys")]
mod project;

// structural access
crate::items! {
    mod doc_inline {
        #[cfg(feature = "sys")]
        pub use super::project::*;
    }
    #[allow(unused_imports)] pub use doc_inline::*;
    pub(super) mod all { #[doc(inline)] pub use super::doc_inline::*; }
}
