// devela::data::bit
//
//! Bit-focused types and traits.
//

/* modules */

// always compiled, non-public
mod field;
mod r#trait;
mod wrapper;

/* re-exports */

// always compiled, non-public
pub use {field::*, r#trait::BitOps, wrapper::Biting};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{field::*, r#trait::BitOps, wrapper::Biting};
}
