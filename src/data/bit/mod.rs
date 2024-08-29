// devela::data::bit
//
//! Bit-focused types and traits.
//

mod r#trait;
mod wrapper;
#[allow(unused_imports)]
pub use {r#trait::BitOps, wrapper::Bitwise};

#[cfg(_some_bit)]
crate::items! {
    mod field;
    pub use field::*;
}

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{r#trait::BitOps, wrapper::Bitwise};

    #[doc(inline)]
    #[cfg(_some_bit)]
    pub use super::field::*;
}
