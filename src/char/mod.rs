// devela::char
//
//! Unicode scalars, extends [`core::char`].
//

/* always compiled for internal use */

/* only compiled with the `char` feature */

#[cfg(feature = "char")]
mod definitions;
#[cfg(feature = "char")]
mod fns;

#[cfg(feature = "char")]
mod core_impls;
#[cfg(feature = "char")]
mod impls;
#[cfg(all(feature = "char", test))]
mod tests;

/* re-exports */

#[cfg(feature = "char")]
pub use all::*;
#[cfg(feature = "char")]
pub(crate) mod all {
    pub use super::{definitions::*, fns::*};
}
