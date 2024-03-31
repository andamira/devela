// devela::sys::ffi
//
//! Platform-specific types, as defined by C, extends
//! `std::`[`ffi`].
//!
//! [`ffi`]: std::ffi
//

mod reexports;

#[allow(unused)]
pub use reexports::*;

pub(crate) mod all {
    #[doc(inline)]
    pub use super::reexports::*;
}
