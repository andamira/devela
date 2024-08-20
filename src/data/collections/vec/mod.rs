// devela::data::collections::vec
//
//! Vectors,
#![doc = crate::code::doc_extends!(vec)]
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
