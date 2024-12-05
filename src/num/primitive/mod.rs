// devela::num::primitive
//
//! Helpers for converting between primitives.
//

#[cfg(test)]
mod tests;

mod namespace; // Cast

mod cast; // PrimitiveCast
mod join; // PrimitiveJoin
mod split; // PrimitiveSplit

// structural access
crate::items! {
    mod doc_inline {
        pub use super::{cast::*, join::*, namespace::*, split::*};
    }
    #[allow(unused_imports)] pub use doc_inline::*;
    pub(crate) mod all { #[doc(inline)] pub use super::doc_inline::*; }
}
