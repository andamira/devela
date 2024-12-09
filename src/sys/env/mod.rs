// devela::sys::env
//
//! Inspection and manipulation of the process’s environment.
//!
#![doc = crate::doc_!(extends: env)]
//

mod reexports;

#[cfg(feature = "std")]
mod env;

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::reexports::*;
        #[cfg(feature = "std")]
        pub use super::env::*;
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
    pub(super) mod always { #[allow(unused_imports)]
        pub use super::reexports::*;
    }
}
