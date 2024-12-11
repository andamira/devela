// devela::num::algebra
//
//! Linear algebra and symbolic computation.
//

pub mod linear;

crate::items! { // structural access: doc_inline, all
    #[allow(unused)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::linear::all::*;
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
