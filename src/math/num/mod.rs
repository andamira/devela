// devela::math::num
//
//! Numeric types, extends `std::`[`num`].
//!
//! [`num`]: std::num
//

/* contains always compiled items */

mod float;

pub mod niche;

#[allow(unused)]
#[cfg(not(feature = "math"))]
pub use {float::*, niche::*};

/* feature-gated */

#[cfg(all(feature = "math", test))]
mod tests;
#[cfg(feature = "math")]
mod traits;

// re-export private sub-modules
#[cfg(feature = "math")]
pub use {float::*, traits::*};

// re-export public sub-modules
#[doc(no_inline)]
#[cfg(feature = "math")]
pub use niche::all::*;

pub(crate) mod all {
    #[doc(inline)]
    #[cfg(feature = "math")]
    pub use super::traits::*;

    #[doc(inline)]
    pub use super::{float::*, niche::all::*};
}
