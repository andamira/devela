// devela::text::ascii
//
//! ASCII strings and characters.
#![doc = crate::doc_!(extends: ascii)]
#![doc = crate::doc_!(modules: crate::text; ascii)]
#![doc = crate::doc_!(newline)]
//!
//

mod char;
mod wrapper;

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::{char::AsciiChar, wrapper::Ascii};
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
