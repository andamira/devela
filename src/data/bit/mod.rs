// devela::data::bit
//
//! Bit-focused types and traits.
//

/* always compiled */

mod field;
mod r#trait;
mod wrapper;
#[allow(unused_imports)]
pub use {field::*, r#trait::BitOps, wrapper::Biting};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{field::*, r#trait::BitOps, wrapper::Biting};
}
