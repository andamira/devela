// devela::time
//
//! Time and calendar types and operations, <small>extends
//! `std::`[`time`].</small>
//!
//! [`time`]: std::time
//

// safety:
#![cfg_attr(feature = "safe_time", forbid(unsafe_code))]

/* always compiled */

mod calendar;
mod error;
mod fmt;
mod reexports;
mod split;
mod unix;
pub use {calendar::*, error::*, fmt::*, reexports::*, split::*, unix::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{calendar::*, error::*, fmt::*, reexports::*, split::*, unix::*};
}
