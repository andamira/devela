// devela::lang
//
//! Foreign function interfaces and languages bindings.
// #![doc = crate::doc_!(modules: crate; lang: c, py, script, wasm)]
#![doc = crate::doc_!(newline)]
//!
#![doc = crate::doc_!(extends: ffi)]
//
// safety
#![cfg_attr(feature = "safe_lang", forbid(unsafe_code))]

mod reexports;
#[allow(unused_imports)]
pub use reexports::*;

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::reexports::*;
}
