// devela::string::char
//
//! Unicode scalars, extends `std::`[`char`][std::char].
//

/* always compiled for internal use */

mod always_fns;
#[allow(unused)]
#[cfg(not(feature = "string"))]
pub(crate) use always_fns::*;

/* only compiled with the `string` feature */

#[cfg(feature = "string")]
mod definitions;
#[cfg(feature = "string")]
mod fns;

#[cfg(feature = "string")]
mod core_impls;
#[cfg(feature = "string")]
mod impls;
#[cfg(all(feature = "string", test))]
mod tests;

/* re-exports */

#[cfg(feature = "string")]
pub use all::*;
#[cfg(feature = "string")]
pub(crate) mod all {
    pub use super::{always_fns::*, definitions::*, fns::*};
}
