// devela::num::alg::linear
//
//! Linear algebra.
//

// mod matrix; // TODO
mod vector;

// structural access
crate::items! {
    #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::vector::*;
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
