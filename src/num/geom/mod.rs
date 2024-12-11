// devela::num::geom
//
//! Geometric types and operations, spatial constructs and analysis.
//

mod shape;

crate::items! { // structural access: doc_inline, all
    #[allow(unused)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::shape::all::*;
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
