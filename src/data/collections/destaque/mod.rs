// devela::data::collections::destaque
//
//! A type that can be used as a double-ended queue and a double-ended stack.
//

// no items defined
mod impl_traits;
mod methods;
#[cfg(all(test, feature = "_destaque_u8"))]
mod tests;

mod definitions;

// structural access
crate::items! {
    mod doc_inline {
        pub use super::definitions::*;
    }
    #[allow(unused_imports)] pub use doc_inline::*;
    pub(super) mod all { #[doc(inline)] pub use super::doc_inline::*; }
}
