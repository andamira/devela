// devela::num::geom
//
//! Geometric types and operations, spatial constructs and analysis.
//

mod shape;

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::shape::all::*;
    }
    pub(crate) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
