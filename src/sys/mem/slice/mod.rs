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

// structural access
crate::items! {
    mod doc_inline {
        pub use super::{ext::*, namespace::*};
    }
    #[allow(unused_imports)] pub use doc_inline::*;
    pub(super) mod all { #[doc(inline)] pub use super::doc_inline::*; }
}
