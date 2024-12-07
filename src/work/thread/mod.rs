// devela::work::thread
//
//! Native threads.
//!
#![doc = crate::doc_!(extends: thread)]
//

#[cfg(feature = "std")]
mod reexports;
#[cfg(feature = "std")]
mod sleep;

// structural access
crate::items! {
    mod doc_inline {
        #[cfg(feature = "std")]
        pub use super::{reexports::*, sleep::*};
    }
    #[allow(unused_imports)] pub use doc_inline::*;
    pub(super) mod all { #[doc(inline)]
        #[allow(unused_imports, reason = "feature-gated")]
        pub use super::doc_inline::*;
    }
}
