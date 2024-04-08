// devela::data::bit
//
//! Bit-focused types and traits.
//

mod r#trait;
mod wrapper;
#[allow(unused_imports)]
pub use {r#trait::BitOps, wrapper::Bitwise};

#[cfg(feature = "_-bit_any-_")]
mod field;
#[cfg(feature = "_-bit_any-_")]
pub use field::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{r#trait::BitOps, wrapper::Bitwise};

    #[doc(inline)]
    #[cfg(feature = "_-bit_any-_")]
    pub use super::field::*;
}
