// devela::num::ops
//
//! Operations, extends
//! `std::`[`ops`].
//!
//! [`ops`]: std::ops
//

/* modules */

// always compiled, non-public
mod always_fns;

/* re-exports */

// always compiled, non-public
#[allow(unused)]
pub use always_fns::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::always_fns::*;
}
