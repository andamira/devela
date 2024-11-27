// devela::rend::layout
//
//! Layout functionality.
//

// safety:
#![cfg_attr(feature = "safe_layout", forbid(unsafe_code))]

mod error;
pub use error::*;

pub(crate) mod all {
    pub use super::error::*;
}
