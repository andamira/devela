// devela::text::char
//
//! Unicode scalars.
#![doc = crate::doc_!(extends: char)]
#![doc = crate::doc_!(modules: crate::text; char)]
#![doc = crate::doc_!(newline)]
//!
//

mod always_fns;
#[allow(unused_imports)]
pub use always_fns::*;

// without re-exports
mod core_impls;
mod impls;
#[cfg(test)]
mod tests;

// with re-exports
mod define_trait;
mod define_types;
mod fns;
#[allow(unused_imports)]
pub use {define_trait::*, define_types::*, fns::*};

pub(crate) mod all {
    #[doc(inline)]
    pub use super::always_fns::*;

    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{define_trait::*, define_types::*, fns::*};
}
