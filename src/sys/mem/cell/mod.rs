// devela::sys::mem::cell
//
//! Shareable mutable containers.
//!
#![doc = crate::doc_!(extends: cell)]
//

mod option;
mod reexports;

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::{option::*, reexports::*};
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
