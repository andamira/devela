// devela::os::arch
//
//! SIMD and vendor intrinsics, extends
//! `std::`[`arch`].
//!
//! [`arch`]: std::arch
//

mod reexports;

#[allow(unused)]
pub use reexports::*;

// // re-export private sub-modules
// pub use {non_range::*, non_specific::*, range::*, reexports::*};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::reexports::*;
}
