// devela::num::algebra
//
//! Linear algebra and symbolic computation.
//

pub mod linear;

// structural access
crate::items! {
    mod doc_inline {
        pub use super::linear::*;
    }
    #[allow(unused_imports)] pub use doc_inline::*;
    pub(crate) mod all { #[doc(inline)] pub use super::doc_inline::*; }
}
