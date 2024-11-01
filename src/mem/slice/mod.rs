// devela::mem::slice
//
//! Slices of memory.
#![doc = crate::doc_!(extends: slice)]
#![doc = crate::doc_!(modules: crate::mem; slice)]
#![doc = crate::doc_!(newline)]
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
