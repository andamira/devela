// devela::data::iter
//
//! Composable external iteration.
#![doc = crate::code::doc_!(extends: iter)]
#![doc = crate::code::doc_!(modules: crate::data; iter)]
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
