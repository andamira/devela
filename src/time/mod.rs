// devela::time
//
//! Time and calendar types and operations, extends
//! `std::`[`time`].
//!
//! [`time`]: std::time
//

// safety:
#![cfg_attr(feature = "safe_time", forbid(unsafe_code))]

/* always compiled, private modules */

mod calendar;
mod error;
mod reexports;
mod split;
mod unix;

pub use {calendar::*, error::*, reexports::*, split::*, unix::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{calendar::*, error::*, reexports::*, split::*, unix::*};
}
