// devela::text::char
//
//! Unicode scalars, extends
//! `std::`[`char`][std::char].
//

/* contains always compiled items */

mod always_fns;

#[allow(unused)]
#[cfg(not(feature = "text"))]
pub(crate) use always_fns::*;

/* feature-gated */

#[cfg(feature = "text")]
mod definitions;
#[cfg(feature = "text")]
mod fns;

#[cfg(feature = "text")]
mod core_impls;
#[cfg(feature = "text")]
mod impls;
#[cfg(all(feature = "text", test))]
mod tests;

// re-exports private sub-modules
#[cfg(feature = "text")]
pub use {always_fns::*, definitions::*, fns::*};

#[cfg(feature = "text")]
pub(crate) mod all {
    pub use super::{always_fns::*, definitions::*, fns::*};
}
