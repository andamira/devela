// devela::num
//
//! Numeric types and operations, extends
//! `std::{`[`num`], [`ops`]`}`.
//!
//! [`num`]: core::num
//! [`ops`]: core::ops
//

/* contains always compiled items */

mod _private;
mod alias;
mod error;
mod float;

pub mod niche;
pub mod ops;

#[allow(unused)]
pub(crate) use _private::*;
#[allow(unused)]
#[cfg(not(feature = "num"))]
pub use {alias::*, error::*, float::*, niche::*, ops::*};

/* feature-gated */

#[cfg(feature = "num")]
mod frac;
#[cfg(feature = "num")]
mod int;
#[cfg(feature = "num")]
mod no;
#[cfg(feature = "num")]
mod r#trait;

// re-export private sub-modules
#[cfg(feature = "num")]
pub use {alias::*, error::*, float::*, frac::*, int::*, no::*, r#trait::*};

// re-export public sub-modules
#[doc(no_inline)]
#[cfg(feature = "num")]
pub use {niche::all::*, ops::all::*};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::{alias::*, error::*, float::*, niche::all::*, ops::all::*};

    #[doc(inline)]
    #[cfg(feature = "num")]
    pub use super::{frac::*, int::*, no::*, r#trait::*};
}
