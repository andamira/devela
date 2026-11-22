// boot/rust/_.rs

#![allow(dead_code, reason = "Rust reexports may be unused by selectively compiled root modules")]

pub(crate) mod code;
pub(crate) mod error;
pub(crate) mod lang;
pub(crate) mod phys;
pub(crate) mod sys;
pub(crate) mod text;
pub(crate) mod work;

pub(crate) mod _fallback {
    #[cfg(not(root_code))]
    pub(crate) use super::code::*;

    #[cfg(not(root_error))]
    pub(crate) use super::error::*;

    #[cfg(not(root_lang))]
    pub(crate) use super::lang::*;

    #[cfg(not(root_phys))]
    pub(crate) use super::phys::*;

    #[cfg(not(root_sys))]
    pub(crate) use super::sys::*;

    #[cfg(not(root_text))]
    pub(crate) use super::text::*;

    #[cfg(not(root_work))]
    pub(crate) use super::work::*;
}
