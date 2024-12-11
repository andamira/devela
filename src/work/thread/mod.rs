// devela::work::thread
//
//! Native threads.
//!
#![doc = crate::doc_!(extends: thread)]
//

#[cfg(feature = "std")]
mod reexports;
#[cfg(feature = "std")]
mod sleep; // sleep4!

crate::items! { // structural access: doc_inline, all, always
    #[allow(unused)]
    pub use doc_inline::*;
    #[allow(unused)] #[doc(hidden)] #[doc(no_inline)]
    pub use always::*;

    mod doc_inline {
        #[cfg(feature = "std")]
        pub use super::{reexports::*, sleep::*};
    }
    pub(super) mod all { #![allow(unused)]
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused)]
        #[cfg(feature = "std")]
        pub use super::reexports::*;
    }
}
