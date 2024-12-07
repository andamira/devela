// devela::sys::arch
//
//! SIMD and vendor intrinsics.
#![doc = crate::doc_!(extends: arch)]
#![doc = crate::doc_!(modules: crate::sys; arch)]
#![doc = crate::doc_!(newline)]
//!
//

mod reexports;

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::reexports::*;
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
