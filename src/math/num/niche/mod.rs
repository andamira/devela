// devela::math::num::niche
//
//! Numeric types with niche memory layout optimization.
//

/* contains always compiled items */

mod non_specific;
mod reexports;

#[allow(unused)]
#[cfg(not(feature = "math"))]
pub use {non_specific::*, reexports::*};

/* feature-gated */

#[cfg(feature = "math")]
mod non_range;
#[cfg(feature = "math")]
mod range;

// re-export private sub-modules
#[cfg(feature = "math")]
pub use {non_range::*, non_specific::*, range::*, reexports::*};

pub(crate) mod all {
    #[doc(inline)]
    #[cfg(feature = "math")]
    pub use super::{non_range::*, range::*};

    #[doc(inline)]
    pub use super::{non_specific::*, reexports::*};
}
