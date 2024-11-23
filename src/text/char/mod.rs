// devela::text::char
//
//! Unicode scalars.
// #![doc = crate::doc_!(extends: char)]
// #![doc = crate::doc_!(modules: crate::text; char)]
// #![doc = crate::doc_!(newline)]
//

// without re-exports
mod core_impls;
mod impls;
#[cfg(test)]
mod tests;

// with re-exports
mod define_trait;
mod define_types;
mod namespace;
#[allow(unused_imports)]
pub use {define_trait::*, define_types::*, namespace::*};

pub(crate) mod all {
    #[doc(inline)]
    #[allow(unused_imports)]
    pub use super::{define_trait::*, define_types::*, namespace::*};
}
