// devela::data::bit
//
//! Bit-focused types and traits.
//

/* contains always compiled items */

mod field;
mod r#trait;
mod wrapper;

#[allow(unused)]
#[cfg(not(feature = "data"))]
pub use {field::*, r#trait::BitOps, wrapper::Bits};

/* feature-gated */

// re-export private sub-modules
#[cfg(feature = "data")]
pub use {field::*, r#trait::BitOps, wrapper::Bits};

#[cfg(feature = "data")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{field::*, r#trait::BitOps, wrapper::Bits};
}
