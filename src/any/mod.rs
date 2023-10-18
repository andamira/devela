// devela::any
//
//! Dynamic typing and reflection, extends [`core::any`].
//!
//! It also reexports items from `core::any`.
//

/* always compiled for internal use */

/* only compiled with the `any` feature */

#[cfg(feature = "any")]
mod ext;

/* re-exports */
#[cfg(feature = "any")]
mod reexports;

#[cfg(feature = "any")]
pub use all::*;
#[cfg(feature = "any")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{ext::AnyExt, reexports::*};
}