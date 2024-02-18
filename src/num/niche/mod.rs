// devela::num::niche
//
//! Numeric types with niche memory layout optimization.
//

/* modules */

// always compiled, non-public
mod non_specific;
mod reexports;

// feature-gated, non-public
#[cfg(all(feature = "num", test))]
mod tests;
//
#[cfg(feature = "num")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "num")))]
mod non_range;
#[cfg(feature = "num")]
#[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "num")))]
mod range;

/* re-exports */

// always compiled, non-public
pub use {non_specific::*, reexports::*};

// feature-gated, non-public
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
