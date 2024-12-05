// devela::data::sort
//
//! Sorting functionality.
//

// no items defined
mod generic;
#[cfg(_sort_·)]
mod primitives;

mod definition;

// structural access
crate::items! {
    mod doc_inline {
        pub use super::definition::*;
    }
    #[allow(unused_imports)] pub use doc_inline::*;
    pub(super) mod all { #[doc(inline)] pub use super::doc_inline::*; }
}
