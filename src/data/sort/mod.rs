// devela::data::sort
//
//! Sorting functionality.
//

// no items defined
mod generic;
#[cfg(_sort_·)]
mod primitives;

mod definition;

crate::items! { // structural access: doc_inline, all
    #[allow(unused)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::definition::*;
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
