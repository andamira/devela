// devela::data::collections::vec
//
//! Vectors, extends `std::`[`vec`].
//!
//! [`vec`]: mod@std::vec
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
