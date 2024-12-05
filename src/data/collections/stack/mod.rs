// devela::data::collections::stack
//
//! A type that can be used as a single-ended stack.
//

// no items defined
mod impl_traits;
mod methods;

mod definitions;

// structural access
crate::items! {
    mod doc_inline {
        pub use super::definitions::*;
    }
    #[allow(unused_imports)] pub use doc_inline::*;
    pub(super) mod all { #[doc(inline)] pub use super::doc_inline::*; }
}
