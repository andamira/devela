// devela::math
//
//! Mathematics, numbers, operations, extends
//! `std::{`[`num`], [`ops`]`}`.
//!
//! [`num`]: core::num
//! [`ops`]: core::ops
//

/* contains always compiled items */

pub mod num;
pub mod ops;

#[allow(unused)]
#[cfg(not(feature = "math"))]
pub(crate) use {num::*, ops::*};

/* feature-gated */

#[cfg(feature = "math")]
mod error;

// re-export private sub-modules
#[cfg(feature = "math")]
pub use error::*;

// re-export public sub-modules
#[doc(no_inline)]
#[cfg(feature = "math")]
pub use {num::all::*, ops::all::*};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::{num::all::*, ops::all::*};

    #[doc(inline)]
    #[cfg(feature = "math")]
    pub use super::{error::*, num::all::*, ops::all::*};
}
