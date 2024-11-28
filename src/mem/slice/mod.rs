// devela::mem::slice
//
//! Slices of memory.
// #![doc = crate::doc_!(extends: slice)]
// #![doc = crate::doc_!(modules: crate::mem; slice)]
// #![doc = crate::doc_!(newline)]

#[cfg(test)]
mod tests;

mod ext;
mod namespace;
pub use {ext::*, namespace::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{ext::*, namespace::*};
}
