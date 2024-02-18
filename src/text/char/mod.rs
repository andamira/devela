// devela::text::char
//
//! Unicode scalars, extends `std::`[`char`].
//!
//! [`char`]: std::char
//

/* always compiled modules */

mod always_fns;

#[allow(unused)]
#[cfg(not(feature = "text"))]
pub use always_fns::*;

/* feature-gated modules */

// without re-exports
#[cfg(feature = "text")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
mod core_impls;
#[cfg(feature = "text")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
mod impls;
#[cfg(all(feature = "text", test))]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
mod tests;

#[cfg(feature = "text")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
mod definitions;
#[cfg(feature = "text")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "text")))]
mod fns;

#[cfg(feature = "text")]
pub use {always_fns::*, definitions::*, fns::*};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::always_fns::*;

    #[doc(inline)]
    #[cfg(feature = "text")]
    pub use super::{definitions::*, fns::*};
}
