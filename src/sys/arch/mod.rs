// devela::sys::arch
//
//! SIMD and vendor intrinsics, extends
//! `std::`[`arch`].
//!
//! [`arch`]: std::arch
//

mod reexports;

#[allow(unused)]
pub use reexports::*;

pub(crate) mod all {
    #[doc(inline)]
    pub use super::reexports::*;
}
