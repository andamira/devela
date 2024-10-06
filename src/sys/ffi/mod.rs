// devela::sys::ffi
//
//! Foreign function interfaces and languages bindings.
#![doc = crate::code::doc_!(extends: ffi)]
#![doc = crate::code::doc_!(modules: crate::sys; ffi)]
#![doc = crate::code::doc_!(newline)]
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
