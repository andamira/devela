// devela::media::audio
//
//! Audio functionality.
//
// safety
#![cfg_attr(feature = "safe_audio", forbid(unsafe_code))]

mod error;
pub use error::*;

pub(crate) mod all {
    pub use super::error::*;
}
