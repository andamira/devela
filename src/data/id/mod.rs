// devela::data::id
//
//! Data identifiers.
//

mod pin; // pinned memory-based ids
mod seq; // static sequential ids

crate::items! { // structural access: doc_inline, all
    #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::{pin::*, seq::*};
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
