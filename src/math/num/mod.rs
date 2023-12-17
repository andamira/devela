// devela::math::num
//
//! Numeric types, extends `std::`[`num`].
//!
//! [`num`]: std::num
//

/* contains always compiled items */

mod float;
mod non_specific;

#[allow(unused)]
#[cfg(not(feature = "math"))]
pub use {float::*, non_specific::*};

/* feature-gated */

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

// re-export private sub-modules
#[cfg(feature = "math")]
pub use {float::*, non_range::*, non_specific::*, range::*, reexports::*, traits::*};

pub(crate) mod all {
    #[doc(inline)]
    #[cfg(feature = "math")]
    pub use super::{non_range::*, range::*, reexports::*, traits::*};

    #[doc(inline)]
    pub use super::{float::*, non_specific::*};
}
