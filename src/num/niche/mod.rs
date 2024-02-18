// devela::num::niche
//
//! Numeric types with niche memory layout optimization.
//

/* always compiled, non-public modules */

mod non_specific;
mod reexports;

pub use {non_specific::*, reexports::*};

/* feature-gated, non-public modules */

#[cfg(all(feature = "num", test))]
mod tests;
//
#[cfg(feature = "num")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "num")))]
mod non_range;
#[cfg(feature = "num")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "num")))]
mod range;

#[cfg(feature = "num")]
pub use {non_range::*, range::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{non_specific::*, reexports::*};

    // feature-gated
    #[doc(inline)]
    #[cfg(feature = "num")]
    pub use super::{non_range::*, range::*};
}
