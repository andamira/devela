// devela::text::char
//
//! Unicode scalars, extends `std::`[`char`][std::char].
//

/* always compiled for internal use */

mod always_fns;
#[allow(unused)]
#[cfg(not(feature = "text"))]
pub(crate) use always_fns::*;

/* only compiled with the `text` feature */

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

/* re-exports */

#[cfg(feature = "text")]
pub use all::*;
#[cfg(feature = "text")]
pub(crate) mod all {
    pub use super::{always_fns::*, definitions::*, fns::*};
}
