//
//! Curated Rust library re-exports and foundational aliases.
//!
//! Names follow devela's conventions, with availability
//! determined by the target, environment and enabled features.
//! Some types have alternative dependency-backed implementations.
//

#![allow(dead_code, reason = "Rust reexports may be unused by selectively compiled root modules")]

pub(crate) mod code;
pub(crate) mod data;
pub(crate) mod error;
pub(crate) mod lang;
pub(crate) mod num;
pub(crate) mod phys;
pub(crate) mod sys;
pub(crate) mod text;
pub(crate) mod work;

#[doc(inline)]
pub use {code::*, data::*, error::*, lang::*, num::*, phys::*, sys::*, text::*, work::*};

pub(crate) mod _fallback {
    #[cfg(not(root_code))]
    pub use super::code::*;

    #[cfg(not(root_data))]
    pub use super::data::*;

    #[cfg(not(root_error))]
    pub use super::error::*;

    #[cfg(not(root_lang))]
    pub use super::lang::*;

    #[cfg(not(root_num))]
    pub use super::num::*;

    #[cfg(not(root_phys))]
    pub use super::phys::*;

    #[cfg(not(root_sys))]
    pub use super::sys::*;

    #[cfg(not(root_text))]
    pub use super::text::*;

    #[cfg(not(root_work))]
    pub use super::work::*;
}
