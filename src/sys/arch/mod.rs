// devela::sys::arch
//
//! SIMD and vendor intrinsics.
#![doc = crate::doc_!(extends: arch)]
#![doc = crate::doc_!(modules: crate::sys; arch)]
#![doc = crate::doc_!(newline)]
//!
//

mod reexports;
#[allow(unused_imports)]
pub use reexports::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::reexports::*;
}
