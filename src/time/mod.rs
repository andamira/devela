// devela::time
//
//! Temporal quantification, extends
//! `std::`[`time`].
//!
//! [`time`]: std::time
//

// safety:
#![cfg_attr(feature = "safe_time", forbid(unsafe_code))]

/* always compiled, private modules */

mod reexports;

pub use reexports::*;

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::reexports::*;
}
