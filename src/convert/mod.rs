// devela::convert
//
//! Conversion, extends [`std::convert`].
//

/* always compiled for internal use */

/* only compiled with the `convert` feature */

#[cfg(feature = "convert")]
pub mod collection;
#[cfg(feature = "convert")]
pub mod primitive;

/* re-exports */

// Reexported [`az`](https://docs.rs/az) crate.
// Provides casts and checked casts.
#[doc(no_inline)]
#[cfg(any(feature = "az", all(feature = "convert", feature = "depend")))]
pub use ::depend::az;

#[cfg(feature = "convert")]
pub use all::*;
#[cfg(feature = "convert")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{collection::*, primitive::*};
}
