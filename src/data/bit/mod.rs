// devela::data::bit
//
//! Bit-focused types and traits.
//

mod r#trait;
pub use r#trait::BitOps;

#[cfg(_some_bit)]
crate::items! {
    mod field;
    mod wrapper;
    #[doc(inline)]
    pub use {field::bitfield, wrapper::Bitwise};
}

pub(crate) mod all {
    #[doc(inline)]
    pub use super::r#trait::BitOps;

    #[doc(inline)]
    #[cfg(_some_bit)]
    pub use super::{field::bitfield, wrapper::Bitwise};
}
