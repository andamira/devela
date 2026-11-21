// boot/rust/_.rs

#![allow(dead_code, reason = "Rust reexports may be unused by selectively compiled root modules")]

pub(crate) mod error;
pub(crate) mod text;

pub(crate) mod _fallback {
    #[cfg(not(root_error))]
    pub(crate) use super::error::*;

    #[cfg(not(root_text))]
    pub(crate) use super::text::*;
}
