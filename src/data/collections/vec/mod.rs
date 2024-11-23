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

mod chunk;
mod ext;
#[allow(unused_imports)]
pub use {chunk::*, ext::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{chunk::*, ext::*};
}
