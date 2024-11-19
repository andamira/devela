// devela::sys::time
//
//! Time and calendar types and operations.
//!
#![doc = crate::doc_!(extends: time)]
//

// safety:
#![cfg_attr(feature = "safe_time", forbid(unsafe_code))]

mod calendar;
mod error;
mod fmt;
mod no;
mod reexports;
mod split;
mod unix;
#[allow(unused_imports)]
pub use {calendar::*, error::*, fmt::*, no::*, reexports::*, split::*, unix::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{calendar::*, error::*, fmt::*, no::*, reexports::*, split::*, unix::*};
}
