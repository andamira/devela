// devela::media::font
//
//! Font functionality.
//
// safety
#![cfg_attr(feature = "safe_font", forbid(unsafe_code))]

mod error;
pub use error::*;

pub(crate) mod all {
    pub use super::error::*;
}
