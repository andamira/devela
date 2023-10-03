// devela::convert
//
//! Conversion, extends [`core::convert`].
//!
//! It also reexports the [`az`](https://docs.rs/az) crate.
//

/* always compiled for internal use */

/* only compiled with the `convert` feature */

#[cfg(feature = "convert")]
pub mod collection;
#[cfg(feature = "convert")]
pub mod primitive;

/* re-exports */

#[cfg(feature = "convert")]
pub use all::*;
#[cfg(feature = "convert")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{collection::*, primitive::*};
}

// Reexported [`az`](https://docs.rs/az) crate.
// Provides casts and checked casts.
#[doc(no_inline)]
#[cfg(feature = "az")]
pub use ::az;
