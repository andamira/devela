// devela::data::collections::vec
//
//! Vectors,
// #![doc = crate::doc_!(extends: vec)]
// #![doc = crate::doc_!(modules: crate::data::collections; vec)]
// #![doc = crate::doc_!(newline)]
//!
//! Vectors are random-access, sequentially allocated, *dynamically* sized,
//! homogeneous data structures.
//

mod chunk; // VecChunk
mod ext; // ExtVec

// structural access
crate::items! { #[allow(unused_imports)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::{chunk::*, ext::*};
    }
    pub(super) mod all { #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
