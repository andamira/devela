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

// structural access
crate::items! { #[allow(unused_imports)]
    pub use {always::*, doc_inline::*};

    mod doc_inline {
        #[cfg(feature = "std")]
        pub use super::{reexports::*, sleep::*};
    }
    pub(super) mod all { #[doc(inline)]
        #[allow(unused_imports, reason = "feature-gated")]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #![allow(unused_imports)]
        #[cfg(feature = "std")] #[doc(hidden)] #[doc(no_inline)]
        pub use super::reexports::*;
    }
}
