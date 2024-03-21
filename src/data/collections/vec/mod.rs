// devela::data::collections::vec
//
//! Vectors, extends `std::`[`vec`].
//!
//! [`vec`]: mod@std::vec
//!
//! Vectors are random-access, sequentially allocated, *dynamically* sized,
//! homogeneous data structures.
//

/* always compiled */

mod ext;

pub use ext::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::ext::*;
}
