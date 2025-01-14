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

crate::items! { // structural access: _mods, _all, _always
    #[allow(unused)]
    pub use _mods::*;
    #[allow(unused)] #[doc(hidden, no_inline)]
    pub use _always::*;

    mod _mods {
        pub use super::{chunk::*, ext::*};
    }
    pub(super) mod _all {
        #[doc(inline)]
        pub use super::_mods::*;
    }
    pub(super) mod _always { #![allow(unused)]
        pub use super::ext::*;
    }
}
