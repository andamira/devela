// devela::text::ascii
//
//! ASCII strings and characters.
#![doc = crate::doc_!(extends: ascii)]
#![doc = crate::doc_!(modules: crate::text; ascii)]
#![doc = crate::doc_!(newline)]
//!
//

mod wrapper;

#[cfg(feature = "ascii")]
mod char;

crate::items! { // structural access: doc_inline, all
    #[allow(unused)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::wrapper::Ascii;

        #[cfg(feature = "ascii")]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "ascii")))]
        pub use super::char::AsciiChar;
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
