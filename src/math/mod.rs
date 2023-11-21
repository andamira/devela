// devela::math
//
//! Mathematics, numbers, operations, extends
//! `std::{`[`num`][std::num],
//! [`ops`][std::ops]`}`.
//

/* contains always compiled items */

pub mod num;

#[allow(unused)]
#[cfg(not(feature = "math"))]
pub(crate) use num::*;

/* feature-gated */

#[doc(no_inline)]
#[cfg(feature = "math")]
pub use num::all::*;

#[cfg(feature = "math")]
pub(crate) mod all {
    #[doc(inline)]
    pub use super::num::all::*;
}
