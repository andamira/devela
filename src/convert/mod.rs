// devela::convert
//
//! Conversion, extends [`std::convert`].
//

pub mod collection;
pub mod primitive;

#[doc(inline)]
pub use {collection::*, primitive::*};
