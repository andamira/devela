// devela::text::fmt
//
//! Formatting, extends `std::`[`fmt`].
//!
//! [`fmt`]: std::fmt
//

/* always compiled, private modules */

mod misc;
mod num_to_str;

pub use {misc::*, num_to_str::*};

pub(crate) mod all {
    // always compiled
    #[doc(inline)]
    pub use super::{misc::*, num_to_str::*};
}
