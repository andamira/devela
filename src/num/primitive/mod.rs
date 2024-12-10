// devela::num::primitive
//
//! Helpers for converting between primitives.
//

#[cfg(test)]
mod tests;

mod namespace; // Cast

#[cfg(feature = "prim")]
crate::items! {
    mod cast; // PrimitiveCast
    mod join; // PrimitiveJoin
    mod split; // PrimitiveSplit
}

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::namespace::*;
        #[cfg(feature = "prim")]
        #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "prim")))]
        pub use super::{cast::*, join::*, split::*};
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
