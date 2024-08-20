// devela::sys::time
//
//! Time and calendar types and operations.
#![doc = crate::code::doc_extends!(time)]
//!
//

// safety:
#![cfg_attr(feature = "safe_time", forbid(unsafe_code))]

mod calendar;
mod error;
mod fmt;
mod reexports;
mod split;
mod unix;
pub use {calendar::*, error::*, fmt::*, reexports::*, split::*, unix::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{calendar::*, error::*, fmt::*, reexports::*, split::*, unix::*};
}
