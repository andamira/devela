// devela::sys::mem::slice
//
//! Slices of memory.
// #![doc = crate::doc_!(extends: slice)]
// #![doc = crate::doc_!(modules: crate::sys::mem; slice)]
// #![doc = crate::doc_!(newline)]

#[cfg(test)]
mod tests;

mod ext;
mod namespace;

crate::items! { // structural access: doc_inline, all
    #[allow(unused)]
    pub use doc_inline::*;

    mod doc_inline {
        pub use super::{ext::*, namespace::*};
    }
    pub(super) mod all {
        #[doc(inline)]
        pub use super::doc_inline::*;
    }
}
