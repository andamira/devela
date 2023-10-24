// devela::num
//
//! Numeric types, extends `std::`[`num`][std::num].
//

/* contains always compiled items */

mod alias;
mod always_fns;
mod non_specific;

#[allow(unused)]
#[cfg(not(feature = "num"))]
pub(crate) use {alias::*, always_fns::*, non_specific::*};

/* feature-gated */

#[cfg(feature = "num")]
mod error;
#[cfg(feature = "num")]
mod non_range;
#[cfg(feature = "num")]
mod range;
#[cfg(feature = "num")]
mod reexports;
#[cfg(all(feature = "num", test))]
mod tests;
#[cfg(feature = "num")]
mod traits;

// re-export private sub-modules
#[cfg(feature = "num")]
pub use {
    alias::*, always_fns::*, error::*, non_range::*, non_specific::*, range::*, reexports::*,
    traits::*,
};

#[cfg(feature = "num")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{
        alias::*, always_fns::*, error::*, non_range::*, non_specific::*, range::*, reexports::*,
        traits::*,
    };
}
