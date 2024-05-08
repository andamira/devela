// devela::data::bit
//
//! Bit-focused types and traits.
//

mod r#trait;
mod wrapper;
#[allow(unused_imports)]
pub use {r#trait::BitOps, wrapper::Bitwise};

#[cfg(_bit_some)]
mod field;
#[cfg(_bit_some)]
pub use field::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{r#trait::BitOps, wrapper::Bitwise};

    #[doc(inline)]
    #[cfg(_bit_some)]
    pub use super::field::*;
}
