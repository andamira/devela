// devela::text::char
//
//! Unicode scalars, extends `std::`[`char`].
//!
//! [`char`]: std::char
//

/* always compiled */

mod always_fns;

#[allow(unused)]
pub use always_fns::*;

/* feature-gated */

// without re-exports
#[cfg(feature = "text")]
mod core_impls;
#[cfg(feature = "text")]
mod impls;
#[cfg(all(feature = "text", test))]
mod tests;

#[cfg(feature = "text")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
mod definitions;
#[cfg(feature = "text")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
mod fns;

#[cfg(feature = "text")]
pub use {definitions::*, fns::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::always_fns::*;

    #[doc(inline)]
    #[cfg(feature = "text")]
    pub use super::{definitions::*, fns::*};
}
