// devela::text::fmt
//
//! Formatting, extends `std::`[`fmt`].
//!
//! [`fmt`]: std::fmt
//

/* modules */

// always compiled, private
mod misc;
mod num_to_str;

/* re-exports */

// always compiled, private
pub use {misc::*, num_to_str::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{misc::*, num_to_str::*};
}
