// devela::rend::draw
//
//! Drawing functionality.
//
// safety
#![cfg_attr(feature = "safe_draw", forbid(unsafe_code))]

mod error;
pub use error::*;

pub(crate) mod all {
    pub use super::error::*;
}
