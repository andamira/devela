// devela::data::list
//
//! List data types.
//

/* contains always compiled items */

mod array;

#[allow(unused)]
#[cfg(not(feature = "data"))]
pub use array::*;

/* feature-gated */

// private sub-modules

// re-export private sub-modules
#[cfg(feature = "data")]
pub use array::all::*;

pub(crate) mod all {
    #[doc(inline)]
    pub use super::array::all::*;
}
