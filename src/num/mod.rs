// devela::num
//
//! Numeric types, extends [`core::num`].
//!
//! It also reexports the `NonZero*` types from `core`.
//

/* always compiled for internal use */

mod always_fns;
mod non_specific;
#[allow(unused)]
#[cfg(not(feature = "num"))]
pub use {always_fns::*, non_specific::*};

/* only compiled with the `mem` feature */

#[cfg(feature = "num")]
mod non_range;
#[cfg(feature = "num")]
mod range;
#[cfg(feature = "num")]
mod r#trait;

#[cfg(all(feature = "num", test))]
mod tests;

/* re-exports */

#[cfg(feature = "num")]
mod reexports;

#[cfg(feature = "num")]
pub use all::*;
#[cfg(feature = "num")]
pub(crate) mod all {
    pub use super::{
        always_fns::*, non_range::*, non_specific::*, r#trait::*, range::*, reexports::*,
    };
}
