// devela::convert
//
//! Conversion, extends [`core::convert`].
//

pub mod collection;
pub mod primitive;

#[doc(inline)]
pub use {collection::*, primitive::*};

/* reexports */

/// Reexported [`az`](https://docs.rs/az) crate. Provides casts and checked casts.
#[doc(inline)]
pub use az;
