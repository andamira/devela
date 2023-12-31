// devela::num::niche
//
//! Numeric types with niche memory layout optimization.
//

/* contains always compiled items */

mod non_specific;
mod reexports;

#[allow(unused)]
#[cfg(not(feature = "num"))]
pub use {non_specific::*, reexports::*};

/* feature-gated */

#[cfg(all(feature = "num", test))]
mod tests;

#[cfg(feature = "num")]
mod non_range;
#[cfg(feature = "num")]
mod range;

// re-export private sub-modules
#[cfg(feature = "num")]
pub use {non_range::*, non_specific::*, range::*, reexports::*};

pub(crate) mod all {
    #[doc(inline)]
    #[cfg(feature = "num")]
    pub use super::{non_range::*, range::*};

    #[doc(inline)]
    pub use super::{non_specific::*, reexports::*};
}
