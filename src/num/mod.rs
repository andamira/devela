// devela::num
//
//! Numeric types, extends [`core::num`].
//!
//! It also reexports the `NonZero*` types from `libcore`.
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

#[cfg(all(feature = "num", test))]
mod tests;

/* re-exports */

#[cfg(feature = "num")]
pub use all::*;
#[cfg(feature = "num")]
pub(crate) mod all {
    pub use super::{always_fns::*, non_range::*, non_specific::*, range::*};
}

#[doc = "A signed integer that is known not to equal zero.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`num`](https://doc.rust-lang.org/core/num)*.\n\n---"]
pub use core::num::{NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize};
#[doc = "An unsigned integer that is known not to equal zero.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`num`](https://doc.rust-lang.org/core/num)*.\n\n---"]
pub use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};
