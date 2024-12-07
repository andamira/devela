// devela::media::font
//
//! Font functionality.
//
// safety
#![cfg_attr(feature = "safe_font", forbid(unsafe_code))]

mod error;

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::error::*;
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
