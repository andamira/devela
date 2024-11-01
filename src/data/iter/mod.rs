// devela::data::iter
//
//! Composable external iteration.
#![doc = crate::doc_!(extends: iter)]
// #![doc = crate::doc_!(modules: crate::data; iter)]
// #![doc = crate::doc_!(newline)]
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
