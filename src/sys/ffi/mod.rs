// devela::sys::ffi
//
//! Platform-specific types, as defined by C, extends
//! `std::`[`ffi`].
//!
//! [`ffi`]: std::ffi
//

mod reexports;
#[allow(unused_imports)]
pub use reexports::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::reexports::*;
}
