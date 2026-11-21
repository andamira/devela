// boot/rust/_.rs

pub(crate) mod error;

pub(crate) mod _fallback {
    #[cfg(not(root_error))]
    pub(crate) use super::error::*;
}
