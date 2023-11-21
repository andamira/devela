// devela::math::num
//
//! Numeric types, operations, comparison, extends
//! `std::{`[`cmp`][std::cmp],
//! [`num`][std::num],
//! [`ops`][std::ops]`}`.
//

/* contains always compiled items */

pub mod cmp;
pub mod ops;

mod alias;
mod always_fns;
mod non_specific;

#[allow(unused)]
#[cfg(not(feature = "math"))]
pub(crate) use {alias::*, always_fns::*, cmp::*, non_specific::*, ops::*};

/* feature-gated */

#[cfg(feature = "math")]
mod error;
#[cfg(feature = "math")]
mod fns;
#[cfg(feature = "math")]
mod non_range;
#[cfg(feature = "math")]
mod range;
#[cfg(feature = "math")]
mod reexports;
#[cfg(all(feature = "math", test))]
mod tests;
#[cfg(feature = "math")]
mod traits;

// re-export public sub-modules
#[doc(no_inline)]
#[cfg(feature = "math")]
pub use {cmp::all::*, ops::all::*};

// re-export private sub-modules
#[cfg(feature = "math")]
pub use {
    alias::*, always_fns::*, error::*, fns::*, non_range::*, non_specific::*, range::*,
    reexports::*, traits::*,
};

#[cfg(feature = "math")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{
        alias::*, always_fns::*, cmp::all::*, error::*, fns::*, non_range::*, non_specific::*,
        ops::*, range::*, reexports::*, traits::*,
    };
}
