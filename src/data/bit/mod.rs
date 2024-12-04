// devela::data::bit
//
//! Bit-focused types and traits.
//

mod r#trait;

#[cfg(_bit_·)]
crate::items! {
    mod field;
    mod wrapper;
}

/* structural access */

#[allow(unused_imports)]
pub use doc_inline::*;

mod doc_inline {
    pub use super::r#trait::BitOps;

    #[cfg(_bit_·)]
    pub use super::{field::bitfield, wrapper::Bitwise};
}
pub(super) mod all {
    #[doc(inline)]
    pub use super::doc_inline::*;
}
