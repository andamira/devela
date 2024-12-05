// devela::num::geom
//
//! Geometric types and operations, spatial constructs and analysis.
//

// mod algebra;
mod shape;

// structural access
crate::items! {
    mod doc_inline {
        pub use super::{ /* algebra::all::*, */ shape::all::*};
    }
    #[allow(unused_imports)] pub use doc_inline::*;
    pub(crate) mod all { #[doc(inline)] pub use super::doc_inline::*; }
}
