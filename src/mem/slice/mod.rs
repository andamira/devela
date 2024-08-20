// devela::mem::slice
//
//! Slices of memory.
#![doc = crate::code::doc_extends!(slice)]
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
