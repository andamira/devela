// devela::sys::ffi
//
//! Foreign function interfaces and languages bindings.
#![doc = crate::code::doc_extends!(ffi)]
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
