// devela::num
//
//! Numeric types, extends [`core::num`].
//!
//! It also reexports the `NonZero*` types from `libcore`.
//

#[cfg(test)]
mod tests;

mod non_range;
mod non_specific;
mod range;

pub use {non_range::*, non_specific::*, range::*};

#[doc = "A signed integer that is known not to equal zero.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`num`](https://doc.rust-lang.org/core/num)*.\n\n---"]
pub use core::num::{NonZeroI128, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize};
#[doc = "An unsigned integer that is known not to equal zero.\n\n"]
#[doc = "*Reexported from"]
#[doc = "`core::`[`num`](https://doc.rust-lang.org/core/num)*.\n\n---"]
pub use core::num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};
