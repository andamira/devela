// devela::num
//
//! Numeric types, operations, comparison, conversion, extends
//! `std::{`[`cmp`][std::cmp],
//! [`convert`][std::convert],
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
#[cfg(not(feature = "num"))]
pub(crate) use {alias::*, always_fns::*, cmp::*, non_specific::*, ops::*};

/* feature-gated */

#[cfg(feature = "num")]
pub mod convert;

#[cfg(feature = "num")]
mod error;
#[cfg(feature = "num")]
mod fns;
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

// re-export public sub-modules
#[doc(no_inline)]
#[cfg(feature = "num")]
pub use {cmp::all::*, convert::all::*, ops::all::*};

// re-export private sub-modules
#[cfg(feature = "num")]
pub use {
    alias::*, always_fns::*, error::*, fns::*, non_range::*, non_specific::*, range::*,
    reexports::*, traits::*,
};

#[cfg(feature = "num")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{
        alias::*, always_fns::*, cmp::all::*, convert::all::*, error::*, fns::*, non_range::*,
        non_specific::*, ops::*, range::*, reexports::*, traits::*,
    };
}
