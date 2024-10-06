// devela::data::collections::vec
//
//! Vectors,
#![doc = crate::code::doc_!(extends: vec)]
#![doc = crate::code::doc_!(modules: crate::data::collections; vec)]
#![doc = crate::code::doc_!(newline)]
//!
//! Vectors are random-access, sequentially allocated, *dynamically* sized,
//! homogeneous data structures.
//

mod ext;
#[allow(unused_imports)]
pub use ext::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::ext::*;
}
