// devela::text::fmt
//
//! Formatting strings.
//!
#![doc = crate::doc_!(extends: fmt)]
//

mod buf;
mod num_to_str;
mod reexports;

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::{buf::*, num_to_str::*, reexports::*};
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
