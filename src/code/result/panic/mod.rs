// devela::code::result::panic
//
//! Panic support,
#![doc = crate::doc_!(extends: panic)]
#![doc = crate::doc_!(modules: crate::code::result; panic)]
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
    pub(crate) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
