// devela::text::fmt
//
//! Formatting, extends `std::`[`fmt`].
//!
//! [`fmt`]: std::fmt
//

mod misc;
mod num_to_str;
#[allow(unused_imports)]
pub use {misc::*, num_to_str::*};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::{misc::*, num_to_str::*};
}
