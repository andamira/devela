// devela::data::id
//
//! Data identifiers.
//

mod pin; // pinned memory-based ids
mod seq; // static sequential ids

// structural access
crate::items! {
    mod doc_inline {
        pub use super::{pin::*, seq::*};
    }
    #[allow(unused_imports)] pub use doc_inline::*;
    pub(super) mod all { #[doc(inline)] pub use super::doc_inline::*; }
}
