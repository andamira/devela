// devela::mem::slice
//
//! Slices of memory.
#![doc = crate::code::doc_!(extends: slice)]
#![doc = crate::code::doc_!(modules: crate::mem; slice)]
#![doc = crate::code::doc_!(newline)]
//!
//

mod ext;
mod wrapper;
pub use {ext::*, wrapper::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{ext::*, wrapper::*};
}
