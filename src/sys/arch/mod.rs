// devela::sys::arch
//
//! SIMD and vendor intrinsics, extends
//! `std::`[`arch`].
//!
//! [`arch`]: std::arch
//

mod reexports;
#[allow(unused_imports)]
pub use reexports::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::reexports::*;
}
