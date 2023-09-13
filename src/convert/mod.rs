// devela::convert
//
//! Conversion, extends [`core::convert`].
//!
//! It also reexports the [`az`](https://docs.rs/az) crate.
//

pub mod collection;
pub mod primitive;

#[doc(inline)]
pub use {collection::*, primitive::*};

// Reexported [`az`](https://docs.rs/az) crate.
// Provides casts and checked casts.
#[doc(no_inline)]
pub use ::az;
