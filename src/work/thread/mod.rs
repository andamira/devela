// devela::work::thread
//
//! Native threads.
//!
#![doc = crate::doc_!(extends: thread)]
//

mod reexports;
mod sleep;

// structural access
crate::items! {
    mod doc_inline {
        pub use super::{reexports::*, sleep::*};
    }
    #[allow(unused_imports)] pub use doc_inline::*;
    pub(super) mod all { #[doc(inline)] pub use super::doc_inline::*; }
}
