// devela::data::bit
//
//! Bit-focused types and traits.
//

/* contains always compiled items */

mod field;
mod ops;

#[allow(unused)]
#[cfg(not(feature = "data"))]
pub use {field::*, ops::*};

/* feature-gated */

// re-export private sub-modules
#[cfg(feature = "data")]
pub use {field::*, ops::*};

#[cfg(feature = "data")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{field::*, ops::*};
}
